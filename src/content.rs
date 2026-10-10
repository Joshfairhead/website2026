//! Loading and validating everything under `content/`.
//!
//! Layout (see docs/build-spec.md §3):
//!   content/for/<org>.md                    recipient cards
//!   content/<node path>/<slug>/index.md     entries (leaf nodes only), assets alongside
//!   content/<node path>/drafts/<slug>/      drafts of that node
//!   content/drafts/<slug>/                  drafts with no node yet
//!   content/<music node>/albums.toml, tracks.toml
//!   content/<art node>/items.toml, images/

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::config::{Collection, Tree};
use crate::error::{Error, Result};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    pub title: String,
    pub date: toml::value::Datetime,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub card: Option<String>,
    pub banner: Option<String>,
    #[serde(default)]
    pub toc: bool,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// Node id, or None for a site-level draft.
    pub node: Option<String>,
    pub slug: String,
    pub dir: PathBuf,
    pub draft: bool,
    pub fm: FrontMatter,
    /// `YYYY-MM-DD`.
    pub date: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Album {
    pub title: String,
    pub artist: String,
    pub link: Option<String>,
    /// Path relative to the music node folder.
    pub cover: Option<String>,
    /// Slug of an artist entry in the music node.
    pub artist_page: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtItem {
    pub id: String,
    pub group: String,
    pub artist: String,
    pub title: String,
    pub source: Option<String>,
    pub width: u32,
    pub height: u32,
    /// Path relative to the art node folder.
    pub file: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipientMatter {
    /// Who the card is for; a label for the author, never shown on the page.
    #[allow(dead_code)]
    pub title: String,
    pub tagline: Option<String>,
    #[serde(default)]
    pub links: Vec<RecipientLink>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipientLink {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Recipient {
    pub slug: String,
    pub card: RecipientMatter,
}

#[derive(Debug, Default)]
pub struct Content {
    pub root: PathBuf,
    pub entries: Vec<Entry>,
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>,
    pub art: Vec<ArtItem>,
    pub recipients: Vec<Recipient>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AlbumFile {
    #[serde(default)]
    album: Vec<Album>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrackFile {
    #[serde(default)]
    track: Vec<Track>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtFile {
    #[serde(default)]
    item: Vec<ArtItem>,
}

impl Content {
    pub fn load(root: &Path, tree: &Tree) -> Result<Self> {
        let mut c = Content {
            root: root.to_path_buf(),
            ..Default::default()
        };
        let mut problems = Vec::new();

        // Top level: branches, `for`, `drafts`; nothing else.
        for name in dir_names(root)? {
            let known = tree.branches().any(|b| b.id == name) || name == "for" || name == "drafts";
            if !known {
                problems.push(format!("content/{name}: not a branch, `for` or `drafts`"));
            }
        }
        for file in file_names(root)? {
            problems.push(format!("content/{file}: stray file at the top level"));
        }

        for node in tree.iter() {
            let dir = root.join(node.path());
            if !dir.is_dir() {
                continue;
            }
            let rel = format!("content/{}", node.path());
            if node.is_leaf() {
                for name in dir_names(&dir)? {
                    match name.as_str() {
                        "drafts" => {
                            for d in dir_names(&dir.join("drafts"))? {
                                c.load_entry(
                                    Some(&node.id),
                                    &dir.join("drafts").join(&d),
                                    true,
                                    &mut problems,
                                )?;
                            }
                        }
                        "images" if node.collection == Some(Collection::Art) => {}
                        _ => {
                            c.load_entry(Some(&node.id), &dir.join(&name), false, &mut problems)?
                        }
                    }
                }
                let allowed: &[&str] = match node.collection {
                    Some(Collection::Music) => &["albums.toml", "tracks.toml"],
                    Some(Collection::Art) => &["items.toml"],
                    None => &[],
                };
                for file in file_names(&dir)? {
                    if !allowed.contains(&file.as_str()) {
                        problems.push(format!("{rel}/{file}: unexpected file in a node folder"));
                    }
                }
                match node.collection {
                    Some(Collection::Music) => {
                        if let Some(f) = read_toml::<AlbumFile>(&dir.join("albums.toml"))? {
                            c.albums = f.album;
                        }
                        if let Some(f) = read_toml::<TrackFile>(&dir.join("tracks.toml"))? {
                            c.tracks = f.track;
                        }
                    }
                    Some(Collection::Art) => {
                        if let Some(f) = read_toml::<ArtFile>(&dir.join("items.toml"))? {
                            c.art = f.item;
                        }
                    }
                    None => {}
                }
            } else {
                for name in dir_names(&dir)? {
                    if !node.children.contains(&name) {
                        problems.push(format!(
                            "{rel}/{name}: `{}` has child nodes, so entries go in one of them ({})",
                            node.label,
                            node.children.join(", ")
                        ));
                    }
                }
                for file in file_names(&dir)? {
                    problems.push(format!("{rel}/{file}: unexpected file in a node folder"));
                }
            }
        }

        let drafts = root.join("drafts");
        if drafts.is_dir() {
            for d in dir_names(&drafts)? {
                c.load_entry(None, &drafts.join(&d), true, &mut problems)?;
            }
        }

        let recipients = root.join("for");
        if recipients.is_dir() {
            for file in file_names(&recipients)? {
                let path = recipients.join(&file);
                let Some(slug) = file.strip_suffix(".md") else {
                    problems.push(format!("content/for/{file}: recipient cards are .md files"));
                    continue;
                };
                let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
                match split_front_matter(&text) {
                    Some((fm, rest)) if rest.trim().is_empty() => match toml::from_str(fm) {
                        Ok(card) => c.recipients.push(Recipient {
                            slug: slug.to_string(),
                            card,
                        }),
                        Err(e) => problems.push(format!("content/for/{file}: {e}")),
                    },
                    Some(_) => {
                        problems.push(format!("content/for/{file}: recipient cards have no body"))
                    }
                    None => problems.push(format!("content/for/{file}: missing +++ front matter")),
                }
            }
        }

        c.validate(tree, &mut problems);
        if problems.is_empty() {
            c.entries.sort_by(|a, b| {
                b.date
                    .cmp(&a.date)
                    .then_with(|| a.fm.title.cmp(&b.fm.title))
            });
            Ok(c)
        } else {
            Err(Error::Content(problems))
        }
    }

    fn load_entry(
        &mut self,
        node: Option<&str>,
        dir: &Path,
        draft: bool,
        problems: &mut Vec<String>,
    ) -> Result<()> {
        let rel = dir
            .strip_prefix(&self.root)
            .unwrap_or(dir)
            .display()
            .to_string();
        let index = dir.join("index.md");
        let slug = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if !index.is_file() {
            problems.push(format!("content/{rel}: folder has no index.md"));
            return Ok(());
        }
        if !is_slug(&slug) {
            problems.push(format!(
                "content/{rel}: folder name must be letters, digits, `-` or `_`"
            ));
        }
        let text = fs::read_to_string(&index).map_err(|e| Error::io(&index, e))?;
        let Some((fm_text, body)) = split_front_matter(&text) else {
            problems.push(format!("content/{rel}/index.md: missing +++ front matter"));
            return Ok(());
        };
        let fm: FrontMatter = match toml::from_str(fm_text) {
            Ok(fm) => fm,
            Err(e) => {
                problems.push(format!("content/{rel}/index.md: {}", e.to_string().trim()));
                return Ok(());
            }
        };
        let date = match (fm.date.date, fm.date.time) {
            (Some(d), None) => format!("{:04}-{:02}-{:02}", d.year, d.month, d.day),
            _ => {
                problems.push(format!(
                    "content/{rel}/index.md: date must be a plain date (YYYY-MM-DD)"
                ));
                String::new()
            }
        };
        for (field, value) in [("card", &fm.card), ("banner", &fm.banner)] {
            if let Some(f) = value
                && !dir.join(f).is_file()
            {
                problems.push(format!(
                    "content/{rel}/index.md: {field} `{f}` does not exist"
                ));
            }
        }
        if fm.title.trim().is_empty() || fm.description.trim().is_empty() {
            problems.push(format!(
                "content/{rel}/index.md: title and description must not be empty"
            ));
        }
        self.entries.push(Entry {
            node: node.map(str::to_string),
            slug,
            dir: dir.to_path_buf(),
            draft,
            fm,
            date,
            body: body.to_string(),
        });
        Ok(())
    }

    fn validate(&self, tree: &Tree, problems: &mut Vec<String>) {
        let mut seen = HashSet::new();
        for e in &self.entries {
            if !seen.insert((e.node.clone(), e.slug.clone())) {
                problems.push(format!(
                    "entry `{}` exists twice in {} (once as a draft?)",
                    e.slug,
                    e.node.as_deref().unwrap_or("drafts")
                ));
            }
        }
        let music = tree
            .iter()
            .find(|n| n.collection == Some(Collection::Music));
        let art = tree.iter().find(|n| n.collection == Some(Collection::Art));
        if let Some(music) = music {
            let dir = self.root.join(music.path());
            let artists: HashSet<_> = self
                .entries
                .iter()
                .filter(|e| e.node.as_deref() == Some(&music.id) && !e.draft)
                .map(|e| e.slug.as_str())
                .collect();
            for a in &self.albums {
                if let Some(p) = &a.artist_page
                    && !artists.contains(p.as_str())
                {
                    problems.push(format!(
                        "albums.toml: `{}` has unknown artist_page `{p}`",
                        a.title
                    ));
                }
                if let Some(cover) = &a.cover
                    && !dir.join(cover).is_file()
                {
                    problems.push(format!(
                        "albums.toml: `{}` cover `{cover}` does not exist",
                        a.title
                    ));
                }
                check_url(
                    &format!("albums.toml: `{}`", a.title),
                    a.link.as_deref(),
                    problems,
                );
            }
            for t in &self.tracks {
                check_url(
                    &format!("tracks.toml: `{}`", t.title),
                    t.link.as_deref(),
                    problems,
                );
            }
        }
        if let Some(art) = art {
            let dir = self.root.join(art.path());
            let mut ids = HashSet::new();
            for i in &self.art {
                if !ids.insert(i.id.as_str()) {
                    problems.push(format!("items.toml: id `{}` is used twice", i.id));
                }
                if !dir.join(&i.file).is_file() {
                    problems.push(format!(
                        "items.toml: `{}` file `{}` does not exist",
                        i.id, i.file
                    ));
                }
                check_url(
                    &format!("items.toml: `{}`", i.id),
                    i.source.as_deref(),
                    problems,
                );
            }
        }
        for r in &self.recipients {
            for l in &r.card.links {
                check_url(
                    &format!("content/for/{}.md", r.slug),
                    Some(&l.url),
                    problems,
                );
            }
        }
    }

    /// Published entries of a node (drafts included when `drafts`).
    pub fn entries_of(&self, node: &str, drafts: bool) -> impl Iterator<Item = &Entry> {
        let node = node.to_string();
        self.entries
            .iter()
            .filter(move |e| e.node.as_deref() == Some(node.as_str()) && (drafts || !e.draft))
    }
}

/// Rejects obviously malformed external URLs (no network access).
pub fn check_url(context: &str, url: Option<&str>, problems: &mut Vec<String>) {
    let Some(url) = url else { return };
    let ok = url.starts_with("mailto:")
        || ["https://", "http://"].iter().any(|p| {
            url.strip_prefix(p).is_some_and(|rest| {
                !rest.is_empty() && !rest.contains("://") && !rest.starts_with('/')
            })
        });
    if !ok {
        problems.push(format!("{context}: malformed URL `{url}`"));
    }
}

pub fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("+++\n")?;
    let end = rest.find("\n+++")?;
    let body = &rest[end + 4..];
    Some((&rest[..end], body.strip_prefix('\n').unwrap_or(body)))
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Option<T>> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    toml::from_str(&text).map(Some).map_err(|e| Error::Toml {
        path: path.to_path_buf(),
        source: Box::new(e),
    })
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn dir_names(dir: &Path) -> Result<Vec<String>> {
    names(dir, true)
}

fn file_names(dir: &Path) -> Result<Vec<String>> {
    names(dir, false)
}

fn names(dir: &Path, dirs: bool) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| Error::io(dir, e))? {
        let entry = entry.map_err(|e| Error::io(dir, e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        if entry.path().is_dir() == dirs {
            out.push(name);
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn tree() -> Tree {
        Tree::load(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/site.toml"))).unwrap()
    }

    pub fn write(root: &Path, rel: &str, text: &str) {
        let p = root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }

    pub fn entry(title: &str, date: &str) -> String {
        format!(
            "+++\ntitle = \"{title}\"\ndate = {date}\ndescription = \"About {title}.\"\n+++\n\nHello **{title}**.\n"
        )
    }

    fn problems(root: &Path) -> Vec<String> {
        match Content::load(root, &tree()) {
            Err(Error::Content(p)) => p,
            other => panic!("expected content problems, got {other:?}"),
        }
    }

    #[test]
    fn loads_entries_drafts_and_collections() {
        let dir = tempfile::tempdir().unwrap();
        let r = dir.path();
        write(
            r,
            "curations/tastes/tea/assam/index.md",
            &entry("Assam", "2010-01-01"),
        );
        write(
            r,
            "curations/tastes/tea/drafts/oolong/index.md",
            &entry("Oolong", "2011-01-01"),
        );
        write(
            r,
            "curations/sounds/music/gogo/index.md",
            &entry("GoGo Penguin", "2021-11-20"),
        );
        write(r, "curations/sounds/music/gogo/cover.jpg", "jpg");
        write(
            r,
            "curations/sounds/music/albums.toml",
            "[[album]]\ntitle = \"Man Made Object\"\nartist = \"GoGo Penguin\"\ncover = \"gogo/cover.jpg\"\nartist_page = \"gogo\"\n",
        );
        write(
            r,
            "curations/sounds/music/tracks.toml",
            "[[track]]\ntitle = \"Chicago\"\nartist = \"Sufjan Stevens\"\nlink = \"https://youtu.be/x\"\n",
        );
        write(r, "drafts/career/index.md", &entry("Career", "2020-01-01"));
        write(
            r,
            "for/acme.md",
            "+++\ntitle = \"Acme\"\n[[links]]\nlabel = \"CV\"\nurl = \"https://cv.example.com\"\n+++\n",
        );
        let c = Content::load(r, &tree()).unwrap();
        assert_eq!(c.entries.len(), 4);
        assert_eq!(c.entries_of("tea", false).count(), 1);
        assert_eq!(c.entries_of("tea", true).count(), 2);
        assert_eq!(c.albums.len(), 1);
        assert_eq!(c.tracks.len(), 1);
        assert_eq!(c.recipients[0].card.links[0].label, "CV");
        assert_eq!(c.entries[0].date, "2021-11-20", "newest first");
    }

    #[test]
    fn rejects_entries_in_non_leaf_nodes_and_stray_folders() {
        let dir = tempfile::tempdir().unwrap();
        let r = dir.path();
        write(
            r,
            "curations/sounds/oops/index.md",
            &entry("Oops", "2020-01-01"),
        );
        write(r, "misc/index.md", &entry("Misc", "2020-01-01"));
        let p = problems(r);
        assert!(
            p.iter().any(|m| m.contains("curations/sounds/oops")),
            "{p:?}"
        );
        assert!(p.iter().any(|m| m.contains("content/misc")), "{p:?}");
    }

    #[test]
    fn rejects_bad_front_matter_and_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        let r = dir.path();
        write(
            r,
            "creations/media/a/index.md",
            "+++\ntitle = \"A\"\ndate = 2020-01-01\ndescription = \"x\"\nauthors = [\"me\"]\n+++\n",
        );
        write(
            r,
            "creations/media/b/index.md",
            "+++\ntitle = \"B\"\ndate = 2020-01-01\ndescription = \"x\"\ncard = \"nope.jpg\"\n+++\n",
        );
        write(r, "creations/media/c/index.md", "no front matter");
        let p = problems(r);
        assert!(
            p.iter()
                .any(|m| m.contains("media/a") && m.contains("authors")),
            "{p:?}"
        );
        assert!(p.iter().any(|m| m.contains("card `nope.jpg`")), "{p:?}");
        assert!(
            p.iter()
                .any(|m| m.contains("media/c") && m.contains("front matter")),
            "{p:?}"
        );
    }

    #[test]
    fn rejects_broken_collection_references() {
        let dir = tempfile::tempdir().unwrap();
        let r = dir.path();
        write(
            r,
            "curations/sounds/music/albums.toml",
            "[[album]]\ntitle = \"X\"\nartist = \"Y\"\nlink = \"https:/https://bad\"\nartist_page = \"ghost\"\n",
        );
        write(
            r,
            "curations/sights/art/items.toml",
            "[[item]]\nid = \"a\"\ngroup = \"G\"\nartist = \"A\"\ntitle = \"T\"\nwidth = 1\nheight = 1\nfile = \"images/missing.jpg\"\n",
        );
        let p = problems(r);
        assert!(p.iter().any(|m| m.contains("malformed URL")), "{p:?}");
        assert!(
            p.iter().any(|m| m.contains("unknown artist_page `ghost`")),
            "{p:?}"
        );
        assert!(p.iter().any(|m| m.contains("images/missing.jpg")), "{p:?}");
    }

    #[test]
    fn url_check() {
        let mut p = Vec::new();
        for ok in ["https://a.b/c", "http://x.y", "mailto:a@b.c"] {
            check_url("t", Some(ok), &mut p);
        }
        assert!(p.is_empty(), "{p:?}");
        for bad in ["https:/https://a", "ftp://x", "https://", "https:///x"] {
            check_url("t", Some(bad), &mut p);
        }
        assert_eq!(p.len(), 4, "{p:?}");
    }
}
