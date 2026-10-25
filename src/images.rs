//! Responsive images: smaller copies of every raster image, made once per build.
//!
//! Originals stay as they are and are what the lightbox opens. Pages show a
//! scaled copy chosen by the browser from `srcset`. Copies live in a cache keyed
//! by the original's content, so unchanged images are not processed again, and
//! are hard-linked next to the original in each target's output.

use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageDecoder, ImageReader};
use rayon::prelude::*;

use crate::error::{Error, Result};

/// Widths of the scaled copies, smallest first.
pub const WIDTHS: [u32; 2] = [480, 1200];
const JPEG_QUALITY: u8 = 78;

/// `sizes` for images in card grids, album grids, the art grid and galleries.
pub const SIZES_GRID: &str = "(max-width: 640px) 50vw, 300px";
/// `sizes` for images that span the reading column.
pub const SIZES_COLUMN: &str = "(max-width: 760px) 100vw, 728px";

#[derive(Debug, Clone)]
pub struct Variant {
    pub width: u32,
    pub height: u32,
    /// File in the cache.
    cached: PathBuf,
    /// Suffix added to the original's file name, e.g. `.w480.jpg`.
    pub suffix: String,
}

#[derive(Debug, Clone)]
pub struct Info {
    /// Display size of the original, after EXIF orientation.
    pub width: u32,
    pub height: u32,
    pub variants: Vec<Variant>,
}

/// Every processed image, by source path.
#[derive(Debug, Default)]
pub struct Images {
    map: HashMap<PathBuf, Info>,
}

pub fn is_raster(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
        matches!(
            e.to_ascii_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "webp"
        )
    })
}

impl Images {
    /// Processes every raster image under `dirs`, in parallel, using `cache`.
    pub fn prepare(dirs: &[PathBuf], cache: &Path) -> Result<Self> {
        fs::create_dir_all(cache).map_err(|e| Error::io(cache, e))?;
        let mut files = Vec::new();
        for d in dirs {
            collect(d, &mut files)?;
        }
        files.sort();
        files.dedup();
        let results: Vec<std::result::Result<(PathBuf, Info), String>> = files
            .into_par_iter()
            .map(|f| {
                process(&f, cache)
                    .map(|i| (f.clone(), i))
                    .map_err(|e| format!("{}: {e}", f.display()))
            })
            .collect();
        let mut map = HashMap::new();
        let mut problems = Vec::new();
        for r in results {
            match r {
                Ok((f, i)) => {
                    map.insert(f, i);
                }
                Err(e) => problems.push(e),
            }
        }
        if problems.is_empty() {
            Ok(Images { map })
        } else {
            Err(Error::Content(problems))
        }
    }

    pub fn get(&self, path: &Path) -> Option<&Info> {
        self.map.get(path)
    }

    /// Hard-links (or copies) the scaled copies of `src` next to `dst`.
    pub fn publish(&self, src: &Path, dst: &Path) -> Result<()> {
        let Some(info) = self.map.get(src) else {
            return Ok(());
        };
        for v in &info.variants {
            let to = with_suffix(dst, &v.suffix);
            if fs::hard_link(&v.cached, &to).is_err() {
                fs::copy(&v.cached, &to).map_err(|e| Error::io(&v.cached, e))?;
            }
        }
        Ok(())
    }

    /// `<img>` attributes for the image at `src` published at `url`.
    /// Falls back to a plain `src` for images it has not processed.
    pub fn attrs(&self, src: &Path, url: &str, sizes: &str) -> ImgAttrs {
        let full = encode_url(url);
        let Some(info) = self.map.get(src) else {
            return ImgAttrs {
                src: full.clone(),
                srcset: String::new(),
                sizes: String::new(),
                width: 0,
                height: 0,
                full,
            };
        };
        let mut set: Vec<String> = info
            .variants
            .iter()
            .map(|v| {
                format!(
                    "{} {}w",
                    encode_url(&with_suffix_str(url, &v.suffix)),
                    v.width
                )
            })
            .collect();
        set.push(format!("{full} {}w", info.width));
        let smallest = info.variants.first();
        ImgAttrs {
            src: smallest.map_or(full.clone(), |v| {
                encode_url(&with_suffix_str(url, &v.suffix))
            }),
            srcset: if info.variants.is_empty() {
                String::new()
            } else {
                set.join(", ")
            },
            sizes: if info.variants.is_empty() {
                String::new()
            } else {
                sizes.to_string()
            },
            width: info.width,
            height: info.height,
            full,
        }
    }
}

/// Attributes for one `<img>`; empty strings and zero sizes are omitted.
#[derive(Debug, Clone)]
pub struct ImgAttrs {
    pub src: String,
    pub srcset: String,
    pub sizes: String,
    pub width: u32,
    pub height: u32,
    /// The original, for the lightbox.
    pub full: String,
}

impl ImgAttrs {
    /// Renders as HTML attributes (leading space included), for hand-built markup.
    pub fn html(&self) -> String {
        let mut s = format!(r#" src="{}""#, attr(&self.src));
        if !self.srcset.is_empty() {
            s += &format!(
                r#" srcset="{}" sizes="{}""#,
                attr(&self.srcset),
                attr(&self.sizes)
            );
        }
        if self.width > 0 {
            s += &format!(r#" width="{}" height="{}""#, self.width, self.height);
        }
        if self.full != self.src {
            s += &format!(r#" data-full="{}""#, attr(&self.full));
        }
        s
    }
}

fn process(path: &Path, cache: &Path) -> std::result::Result<Info, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let key = {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut h);
        WIDTHS.hash(&mut h);
        JPEG_QUALITY.hash(&mut h);
        format!("{:016x}", h.finish())
    };
    let meta = cache.join(format!("{key}.toml"));
    if let Some(info) = read_meta(&meta, cache) {
        return Ok(info);
    }

    let reader = ImageReader::new(std::io::Cursor::new(&bytes))
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    let mut decoder = reader
        .into_decoder()
        .map_err(|e| format!("cannot read image: {e}"))?;
    let orientation = decoder.orientation().map_err(|e| e.to_string())?;
    let mut img =
        DynamicImage::from_decoder(decoder).map_err(|e| format!("cannot decode image: {e}"))?;
    img.apply_orientation(orientation);
    let (w, h) = (img.width(), img.height());
    // Only keep PNG for images that are actually transparent somewhere.
    let alpha = img.color().has_alpha() && img.to_rgba8().pixels().any(|p| p.0[3] < 255);

    let mut variants = Vec::new();
    let mut source = img;
    for &target in WIDTHS.iter().rev() {
        if target >= w {
            continue;
        }
        let th = ((h as u64 * target as u64) / w as u64).max(1) as u32;
        let scaled = source.resize_exact(target, th, FilterType::Triangle);
        let ext = if alpha { "png" } else { "jpg" };
        let cached = cache.join(format!("{key}.w{target}.{ext}"));
        let out = fs::File::create(&cached).map_err(|e| e.to_string())?;
        let mut out = BufWriter::new(out);
        if alpha {
            scaled
                .write_to(&mut out, image::ImageFormat::Png)
                .map_err(|e| e.to_string())?;
        } else {
            JpegEncoder::new_with_quality(&mut out, JPEG_QUALITY)
                .encode_image(&scaled.to_rgb8())
                .map_err(|e| e.to_string())?;
        }
        variants.push(Variant {
            width: target,
            height: th,
            cached,
            suffix: format!(".w{target}.{ext}"),
        });
        source = scaled;
    }
    variants.reverse();
    let info = Info {
        width: w,
        height: h,
        variants,
    };
    write_meta(&meta, &info).map_err(|e| e.to_string())?;
    Ok(info)
}

fn read_meta(meta: &Path, cache: &Path) -> Option<Info> {
    let text = fs::read_to_string(meta).ok()?;
    let mut lines = text.lines();
    let (w, h) = lines.next()?.split_once('x')?;
    let mut info = Info {
        width: w.parse().ok()?,
        height: h.parse().ok()?,
        variants: Vec::new(),
    };
    for line in lines {
        let mut parts = line.split(' ');
        let (vw, vh, file) = (parts.next()?, parts.next()?, parts.next()?);
        let cached = cache.join(file);
        if !cached.is_file() {
            return None;
        }
        let suffix = file.split_once('.').map(|(_, s)| format!(".{s}"))?;
        info.variants.push(Variant {
            width: vw.parse().ok()?,
            height: vh.parse().ok()?,
            cached,
            suffix,
        });
    }
    Some(info)
}

fn write_meta(meta: &Path, info: &Info) -> std::io::Result<()> {
    let mut s = format!("{}x{}\n", info.width, info.height);
    for v in &info.variants {
        let file = v.cached.file_name().unwrap().to_string_lossy();
        s += &format!("{} {} {file}\n", v.width, v.height);
    }
    fs::write(meta, s)
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| Error::io(dir, e))? {
        let p = entry.map_err(|e| Error::io(dir, e))?.path();
        if p.file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            continue;
        }
        if p.is_dir() {
            collect(&p, out)?;
        } else if is_raster(&p) {
            out.push(p);
        }
    }
    Ok(())
}

/// `photo.jpg` + `.w480.jpg` → `photo.jpg.w480.jpg`: keeps every name unique,
/// even when two originals differ only in extension.
fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut s = path.as_os_str().to_os_string();
    s.push(suffix);
    PathBuf::from(s)
}

fn with_suffix_str(url: &str, suffix: &str) -> String {
    format!("{url}{suffix}")
}

/// Percent-encodes characters that would break `src`/`srcset` (spaces and commas).
pub fn encode_url(url: &str) -> String {
    url.replace('%', "%25")
        .replace(' ', "%20")
        .replace(',', "%2C")
}

fn attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage, Rgba, RgbaImage};

    #[test]
    fn makes_smaller_copies_only_when_smaller_and_caches_them() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();
        RgbImage::from_pixel(1600, 800, Rgb([200, 10, 10]))
            .save(src.join("big photo.jpg"))
            .unwrap();
        RgbImage::from_pixel(300, 300, Rgb([10, 10, 200]))
            .save(src.join("small.jpg"))
            .unwrap();
        RgbaImage::from_pixel(1000, 500, Rgba([0, 0, 0, 0]))
            .save(src.join("clear.png"))
            .unwrap();
        let cache = dir.path().join("cache");

        let images = Images::prepare(std::slice::from_ref(&src), &cache).unwrap();
        let big = images.get(&src.join("big photo.jpg")).unwrap();
        assert_eq!((big.width, big.height), (1600, 800));
        let widths: Vec<_> = big.variants.iter().map(|v| (v.width, v.height)).collect();
        assert_eq!(widths, [(480, 240), (1200, 600)]);
        assert!(
            images
                .get(&src.join("small.jpg"))
                .unwrap()
                .variants
                .is_empty()
        );
        let clear = images.get(&src.join("clear.png")).unwrap();
        assert!(
            clear.variants.iter().all(|v| v.suffix.ends_with(".png")),
            "alpha keeps PNG"
        );

        let a = images.attrs(&src.join("big photo.jpg"), "/x/big photo.jpg", SIZES_GRID);
        assert_eq!(a.src, "/x/big%20photo.jpg.w480.jpg");
        assert_eq!(a.full, "/x/big%20photo.jpg");
        assert!(
            a.srcset.ends_with("/x/big%20photo.jpg 1600w"),
            "{}",
            a.srcset
        );
        assert!(a.html().contains(r#"width="1600" height="800""#));

        // Second run reads the cache instead of decoding again.
        let files_before = fs::read_dir(&cache).unwrap().count();
        let again = Images::prepare(std::slice::from_ref(&src), &cache).unwrap();
        assert_eq!(fs::read_dir(&cache).unwrap().count(), files_before);
        assert_eq!(
            again
                .get(&src.join("big photo.jpg"))
                .unwrap()
                .variants
                .len(),
            2
        );

        let out = dir.path().join("out");
        fs::create_dir_all(&out).unwrap();
        images
            .publish(&src.join("big photo.jpg"), &out.join("big photo.jpg"))
            .unwrap();
        assert!(out.join("big photo.jpg.w480.jpg").is_file());
    }

    #[test]
    fn unreadable_images_fail_with_the_file_named() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("broken.jpg"), b"not an image").unwrap();
        let err =
            Images::prepare(&[dir.path().to_path_buf()], &dir.path().join("cache")).unwrap_err();
        let Error::Content(p) = err else {
            panic!("{err:?}")
        };
        assert!(p[0].contains("broken.jpg"), "{p:?}");
    }
}
