//! Rendering targets (the monolith and each breakout) to static HTML.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use askama::Template;
use regex::Regex;

use crate::config::{Collection, Node, Tree};
use crate::content::{Content, Entry};
use crate::error::{Error, Result};
use crate::images::{self, Images};
use crate::markdown::{self, Heading, Rendered};

const STYLESHEET: &str = include_str!("../static/site.css");
const SCRIPT: &str = include_str!("../static/site.js");

static IMG_OR_DIV: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<img\b[^>]*>|<div\b[^>]*>|</div>").unwrap());
static SRC_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\s(src|width|height)="[^"]*""#).unwrap());
static AT_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"(href|src)="@/([^"#]*?)/?(#[^"]*)?""##).unwrap());

/// What to build: the whole tree, or one node's subtree as a standalone site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Monolith,
    Breakout(String),
}

impl Target {
    /// Folder name under the output directory.
    pub fn dir_name(&self) -> &str {
        match self {
            Target::Monolith => "monolith",
            Target::Breakout(id) => id,
        }
    }

    fn root(&self) -> Option<&str> {
        match self {
            Target::Monolith => None,
            Target::Breakout(id) => Some(id),
        }
    }
}

/// Everything shared by all targets: the tree, the content and rendered bodies.
pub struct Site<'a> {
    pub tree: &'a Tree,
    pub content: &'a Content,
    pub drafts: bool,
    bodies: HashMap<(String, String), Rendered>,
    images: Images,
}

impl<'a> Site<'a> {
    /// Renders every entry body and scales every image once, collecting all
    /// problems before failing. Scaled images are cached in `cache`.
    pub fn new(tree: &'a Tree, content: &'a Content, drafts: bool, cache: &Path) -> Result<Self> {
        let mut bodies = HashMap::new();
        let mut problems = Vec::new();
        for e in &content.entries {
            let Some(node) = &e.node else { continue };
            if e.draft && !drafts {
                continue;
            }
            match markdown::render(&e.body) {
                Ok(r) => {
                    bodies.insert((node.clone(), e.slug.clone()), r);
                }
                Err(msg) => {
                    problems.push(format!("{}: {msg}", rel(content, &e.dir.join("index.md"))))
                }
            }
        }
        if !problems.is_empty() {
            return Err(Error::Content(problems));
        }
        let mut dirs: Vec<PathBuf> = content
            .entries
            .iter()
            .filter(|e| e.node.is_some() && (drafts || !e.draft))
            .map(|e| e.dir.clone())
            .collect();
        for n in tree
            .iter()
            .filter(|n| n.collection == Some(Collection::Art))
        {
            dirs.push(content.root.join(n.path()).join("images"));
        }
        let images = Images::prepare(&dirs, cache)?;
        Ok(Site {
            tree,
            content,
            drafts,
            bodies,
            images,
        })
    }

    fn entries(&self, node: &str) -> Vec<&'a Entry> {
        self.content.entries_of(node, self.drafts).collect()
    }

    /// Entries in a node's whole subtree, newest first.
    fn subtree_entries(&self, node: &Node) -> Vec<&'a Entry> {
        self.content
            .entries
            .iter()
            .filter(|e| {
                (self.drafts || !e.draft)
                    && e.node
                        .as_deref()
                        .is_some_and(|n| self.tree.is_within(n, &node.id))
            })
            .collect()
    }

    /// What a menu shows next to a node: entries plus collection items in its subtree.
    fn count(&self, node: &Node) -> usize {
        let art =
            if self.tree.iter().any(|n| {
                n.collection == Some(Collection::Art) && self.tree.is_within(&n.id, &node.id)
            }) {
                self.content.art.len()
            } else {
                0
            };
        self.subtree_entries(node).len() + art
    }
}

pub struct Link {
    pub label: String,
    pub href: String,
}

pub struct Pill {
    pub label: String,
    pub href: String,
    pub count: usize,
    pub current: bool,
    /// Child id to filter the grid by in place; empty for a plain link.
    pub filter: String,
}

pub struct Card {
    pub href: String,
    pub title: String,
    pub description: String,
    pub date: String,
    /// `<img>` attributes for the card image.
    pub image: Option<String>,
    pub label: String,
    pub filter: String,
    pub draft: bool,
}

pub struct AlbumCard {
    pub title: String,
    pub artist: String,
    pub link: Option<String>,
    /// `<img>` attributes for the cover.
    pub cover: Option<String>,
    pub artist_href: Option<String>,
}

pub struct TrackRow {
    pub title: String,
    pub artist: String,
    pub link: Option<String>,
}

pub struct Chip {
    pub label: String,
    pub value: String,
    pub count: usize,
}

pub struct ArtFigure {
    /// `<img>` attributes (src, srcset, sizes, width, height, data-full).
    pub img: String,
    pub title: String,
    pub artist: String,
    pub group: String,
    pub source: Option<String>,
}

#[derive(Template)]
#[template(path = "card.html")]
struct CardPage<'a> {
    title: &'a str,
    tagline: Option<&'a str>,
    contact: &'a str,
    links: Vec<BranchLink>,
    still: bool,
}

struct BranchLink {
    branch: String,
    label: String,
    href: String,
}

/// Breadcrumb trail and site-wide chrome shared by every page below the root.
struct Chrome {
    site_title: String,
    branch: String,
    crumbs: Vec<Link>,
    home: Option<Link>,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundPage<'a> {
    title: &'a str,
    home: String,
    host: &'a str,
}

#[derive(Template)]
#[template(path = "listing.html")]
struct ListingPage {
    chrome: Chrome,
    title: String,
    pills: Vec<Pill>,
    cards: Vec<Card>,
}

#[derive(Template)]
#[template(path = "music.html")]
struct MusicPage {
    chrome: Chrome,
    title: String,
    pills: Vec<Pill>,
    artists: Vec<Card>,
    albums: Vec<AlbumCard>,
    tracks: Vec<TrackRow>,
}

#[derive(Template)]
#[template(path = "art.html")]
struct ArtPage {
    chrome: Chrome,
    title: String,
    pills: Vec<Pill>,
    chips: Vec<Chip>,
    total: usize,
    figures: Vec<ArtFigure>,
}

#[derive(Template)]
#[template(path = "entry.html")]
struct EntryPage<'a> {
    chrome: Chrome,
    title: &'a str,
    description: &'a str,
    date: &'a str,
    minutes: usize,
    draft: bool,
    tags: &'a [String],
    /// `<img>` attributes for the banner.
    banner: Option<String>,
    toc: &'a [Heading],
    body: String,
    albums: Vec<AlbumCard>,
    newer: Option<Link>,
    older: Option<Link>,
}

/// Builds one target into `out/<target dir>`. Returns the directory written.
pub fn build(site: &Site, target: &Target, out: &Path) -> Result<PathBuf> {
    let tree = site.tree;
    if let Target::Breakout(id) = target {
        let node = tree.get(id).ok_or_else(|| Error::UnknownNode(id.clone()))?;
        if node.breakout.is_none() {
            return Err(Error::NotABreakout(id.clone()));
        }
    }
    let dir = out.join(target.dir_name());
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| Error::io(&dir, e))?;
    }
    let urls = Urls::new(site, target);

    if *target == Target::Monolith {
        let page = CardPage {
            title: &tree.site.title,
            tagline: None,
            contact: &tree.site.contact,
            links: tree
                .branches()
                .map(|b| BranchLink {
                    branch: b.id.clone(),
                    label: b.label.clone(),
                    href: urls.node(b),
                })
                .collect(),
            still: false,
        };
        write(&dir.join("index.html"), &page.render()?)?;
        for r in &site.content.recipients {
            let page = CardPage {
                title: &tree.site.title,
                tagline: r.card.tagline.as_deref(),
                contact: &tree.site.contact,
                links: r
                    .card
                    .links
                    .iter()
                    .map(|l| BranchLink {
                        branch: String::new(),
                        label: l.label.clone(),
                        href: l.url.clone(),
                    })
                    .collect(),
                still: true,
            };
            write(
                &dir.join("for").join(&r.slug).join("index.html"),
                &page.render()?,
            )?;
        }
    }

    for node in tree.iter() {
        let Some(rel) = urls.local_path(node) else {
            continue;
        };
        let node_dir = dir.join(&rel);
        let chrome = urls.chrome(node, &node.lineage[..node.lineage.len() - 1]);
        let html = match node.collection {
            Some(Collection::Music) => music_page(site, &urls, node, chrome)?,
            Some(Collection::Art) => {
                let src = site.content.root.join(node.path()).join("images");
                if src.is_dir() {
                    copy_dir(&site.images, &src, &node_dir.join("images"))?;
                }
                art_page(site, &urls, node, chrome)?
            }
            None => listing_page(site, &urls, node, chrome)?,
        };
        let html = urls
            .resolve(&html)
            .map_err(|err| Error::Content(vec![format!("content/{}: {err}", node.path())]))?;
        write(&node_dir.join("index.html"), &html)?;

        let entries = site.entries(&node.id);
        for (i, e) in entries.iter().enumerate() {
            let out_dir = node_dir.join(&e.slug);
            copy_dir(&site.images, &e.dir, &out_dir)?;
            let html = entry_page(
                site,
                &urls,
                node,
                e,
                entries.get(i.wrapping_sub(1)).copied(),
                entries.get(i + 1).copied(),
            )?;
            let html = urls
                .resolve(&html)
                .map_err(|err| Error::Content(vec![format!("{}: {err}", rel_entry(site, e))]))?;
            write(&out_dir.join("index.html"), &html)?;
        }
    }

    let not_found = NotFoundPage {
        title: &tree.site.title,
        home: format!("{}/", tree.site.base_url),
        host: host_of(&tree.site.base_url),
    };
    write(&dir.join("404.html"), &not_found.render()?)?;
    write(&dir.join("site.css"), STYLESHEET)?;
    write(&dir.join("site.js"), SCRIPT)?;
    Ok(dir)
}

fn listing_page(site: &Site, urls: &Urls, node: &Node, chrome: Chrome) -> Result<String> {
    let (pills, cards) = if node.is_leaf() {
        let cards = site
            .entries(&node.id)
            .into_iter()
            .map(|e| urls.card(node, e, None))
            .collect();
        (urls.sibling_pills(node), cards)
    } else {
        let depth = node.lineage.len();
        let cards = site
            .subtree_entries(node)
            .into_iter()
            .map(|e| {
                let n = site.tree.node(e.node.as_deref().unwrap());
                let child = site.tree.node(&n.lineage[depth]);
                urls.card(n, e, Some(child))
            })
            .collect();
        (urls.child_pills(node), cards)
    };
    Ok(ListingPage {
        chrome,
        title: node.label.clone(),
        pills,
        cards,
    }
    .render()?)
}

fn music_page(site: &Site, urls: &Urls, node: &Node, chrome: Chrome) -> Result<String> {
    let artists = site
        .entries(&node.id)
        .into_iter()
        .map(|e| urls.card(node, e, None))
        .collect();
    let albums = site
        .content
        .albums
        .iter()
        .map(|a| urls.album(node, a))
        .collect();
    let tracks = site
        .content
        .tracks
        .iter()
        .map(|t| TrackRow {
            title: t.title.clone(),
            artist: t.artist.clone(),
            link: t.link.clone(),
        })
        .collect();
    Ok(MusicPage {
        chrome,
        title: node.label.clone(),
        pills: urls.sibling_pills(node),
        artists,
        albums,
        tracks,
    }
    .render()?)
}

fn art_page(site: &Site, urls: &Urls, node: &Node, chrome: Chrome) -> Result<String> {
    let mut groups: BTreeMap<&str, usize> = BTreeMap::new();
    let mut order = Vec::new();
    for i in &site.content.art {
        let n = groups.entry(&i.group).or_insert(0);
        if *n == 0 {
            order.push(i.group.as_str());
        }
        *n += 1;
    }
    let base = urls.node(node);
    let art_dir = site.content.root.join(node.path());
    Ok(ArtPage {
        chrome,
        title: node.label.clone(),
        pills: urls.sibling_pills(node),
        chips: order
            .into_iter()
            .map(|g| Chip {
                label: g.to_string(),
                value: markdown::slugify(g),
                count: groups[g],
            })
            .collect(),
        total: site.content.art.len(),
        figures: site
            .content
            .art
            .iter()
            .map(|i| ArtFigure {
                img: site
                    .images
                    .attrs(
                        &art_dir.join(&i.file),
                        &format!("{base}{}", i.file),
                        images::SIZES_GRID,
                    )
                    .html(),
                title: i.title.clone(),
                artist: i.artist.clone(),
                group: markdown::slugify(&i.group),
                source: i.source.clone(),
            })
            .collect(),
    }
    .render()?)
}

fn entry_page(
    site: &Site,
    urls: &Urls,
    node: &Node,
    e: &Entry,
    newer: Option<&Entry>,
    older: Option<&Entry>,
) -> Result<String> {
    let body = &site.bodies[&(node.id.clone(), e.slug.clone())];
    let link = |x: &Entry| Link {
        label: x.fm.title.clone(),
        href: format!("{}{}/", urls.node(node), x.slug),
    };
    let albums = if node.collection == Some(Collection::Music) {
        site.content
            .albums
            .iter()
            .filter(|a| a.artist_page.as_deref() == Some(e.slug.as_str()))
            .map(|a| urls.album(node, a))
            .collect()
    } else {
        Vec::new()
    };
    Ok(EntryPage {
        chrome: urls.chrome(node, &node.lineage),
        title: &e.fm.title,
        description: &e.fm.description,
        date: &e.date,
        minutes: body.reading_minutes(),
        draft: e.draft,
        tags: &e.fm.tags,
        banner: e.fm.banner.as_ref().map(|b| {
            site.images
                .attrs(
                    &e.dir.join(b),
                    &format!("{}{}/{b}", urls.node(node), e.slug),
                    images::SIZES_COLUMN,
                )
                .html()
        }),
        toc: if e.fm.toc { &body.toc } else { &[] },
        body: responsive_body(&body.html, &e.dir, &site.images),
        albums,
        newer: newer.map(link),
        older: older.map(link),
    }
    .render()?)
}

struct Urls<'a> {
    site: &'a Site<'a>,
    target: &'a Target,
    /// Monolith path (`curations/sounds/music/gogo`) → URL for this target.
    paths: HashMap<String, String>,
}

impl<'a> Urls<'a> {
    fn new(site: &'a Site<'a>, target: &'a Target) -> Self {
        let mut u = Urls {
            site,
            target,
            paths: HashMap::new(),
        };
        for node in site.tree.iter() {
            let url = u.node(node);
            for e in site.entries(&node.id) {
                u.paths.insert(
                    format!("{}/{}", node.path(), e.slug),
                    format!("{url}{}/", e.slug),
                );
            }
            u.paths.insert(node.path(), url);
        }
        u
    }

    /// Path of a node relative to this target's root, if it is inside the target.
    fn local_path(&self, node: &Node) -> Option<PathBuf> {
        let segs = node.segments();
        let start = match self.target.root() {
            None => 0,
            Some(root) => segs.iter().position(|s| s == root)? + 1,
        };
        Some(segs[start..].iter().collect())
    }

    /// The URL to link to a node from a page in this target.
    fn node(&self, node: &Node) -> String {
        match self.local_path(node) {
            Some(p) if p.as_os_str().is_empty() => "/".to_string(),
            Some(p) => format!("/{}/", p.to_string_lossy()),
            None => format!("{}/{}/", self.site.tree.site.base_url, node.path()),
        }
    }

    /// Rewrites `@/<monolith path>/` links for this target.
    fn resolve(&self, html: &str) -> std::result::Result<String, String> {
        let mut missing = Vec::new();
        let out = AT_LINK.replace_all(html, |c: &regex::Captures| match self.paths.get(&c[2]) {
            Some(url) => format!(
                r#"{}="{url}{}""#,
                &c[1],
                c.get(3).map_or("", |m| m.as_str())
            ),
            None => {
                missing.push(format!("@/{}/", &c[2]));
                c[0].to_string()
            }
        });
        if missing.is_empty() {
            Ok(out.into_owned())
        } else {
            Err(format!(
                "internal link to an unpublished or unknown page: {}",
                missing.join(", ")
            ))
        }
    }

    fn chrome(&self, node: &Node, trail: &[String]) -> Chrome {
        let tree = self.site.tree;
        let mut crumbs = Vec::new();
        if self.target.root().is_none() {
            crumbs.push(Link {
                label: host_of(&tree.site.base_url).to_string(),
                href: "/".to_string(),
            });
        }
        for id in trail {
            let a = tree.node(id);
            if self.local_path(a).is_some() {
                crumbs.push(Link {
                    label: a.label.clone(),
                    href: self.node(a),
                });
            }
        }
        Chrome {
            site_title: tree.site.title.clone(),
            branch: node.branch().to_string(),
            crumbs,
            home: self.target.root().map(|_| Link {
                label: host_of(&tree.site.base_url).to_string(),
                href: format!("{}/", tree.site.base_url),
            }),
        }
    }

    /// A non-leaf node's children as filter pills, led by `All`.
    fn child_pills(&self, node: &Node) -> Vec<Pill> {
        let mut pills = vec![Pill {
            label: "All".into(),
            href: self.node(node),
            count: self.site.subtree_entries(node).len(),
            current: true,
            filter: "all".into(),
        }];
        for id in &node.children {
            let child = self.site.tree.node(id);
            let count = self.site.count(child);
            if count == 0 {
                continue;
            }
            let has_entries = !self.site.subtree_entries(child).is_empty();
            pills.push(Pill {
                label: child.label.clone(),
                href: self.node(child),
                count,
                current: false,
                filter: if has_entries {
                    child.id.clone()
                } else {
                    String::new()
                },
            });
        }
        pills
    }

    /// A leaf's siblings, itself marked current; nothing if the parent is outside the target.
    fn sibling_pills(&self, node: &Node) -> Vec<Pill> {
        let Some(parent) = node.parent.as_deref().map(|p| self.site.tree.node(p)) else {
            return Vec::new();
        };
        if self.local_path(parent).is_none() || parent.children.len() < 2 {
            return Vec::new();
        }
        parent
            .children
            .iter()
            .map(|id| self.site.tree.node(id))
            .filter(|n| n.id == node.id || self.site.count(n) > 0)
            .map(|n| Pill {
                label: n.label.clone(),
                href: self.node(n),
                count: self.site.count(n),
                current: n.id == node.id,
                filter: String::new(),
            })
            .collect()
    }

    fn card(&self, node: &Node, e: &Entry, group: Option<&Node>) -> Card {
        let href = format!("{}{}/", self.node(node), e.slug);
        Card {
            image: e.fm.card.as_ref().or(e.fm.banner.as_ref()).map(|c| {
                self.site
                    .images
                    .attrs(&e.dir.join(c), &format!("{href}{c}"), images::SIZES_GRID)
                    .html()
            }),
            href,
            title: e.fm.title.clone(),
            description: e.fm.description.clone(),
            date: e.date.clone(),
            label: group.map(|g| g.label.clone()).unwrap_or_default(),
            filter: group.map(|g| g.id.clone()).unwrap_or_default(),
            draft: e.draft,
        }
    }

    fn album(&self, music: &Node, a: &crate::content::Album) -> AlbumCard {
        let base = self.node(music);
        AlbumCard {
            title: a.title.clone(),
            artist: a.artist.clone(),
            link: a.link.clone(),
            cover: a.cover.as_ref().map(|c| {
                let src = self.site.content.root.join(music.path()).join(c);
                self.site
                    .images
                    .attrs(&src, &format!("{base}{c}"), images::SIZES_GRID)
                    .html()
            }),
            artist_href: a.artist_page.as_ref().map(|p| format!("{base}{p}/")),
        }
    }
}

/// Points every local `<img>` in an entry body at its scaled copies. Images
/// inside gallery `<div>`s are sized for the grid, the rest for the column.
fn responsive_body(html: &str, dir: &Path, images: &Images) -> String {
    let mut gallery_depth: Vec<bool> = Vec::new();
    IMG_OR_DIV
        .replace_all(html, |c: &regex::Captures| {
            let tag = &c[0];
            if tag == "</div>" {
                gallery_depth.pop();
                return tag.to_string();
            }
            if tag.starts_with("<div") {
                let is_gallery = tag
                    .split("class=\"")
                    .nth(1)
                    .and_then(|r| r.split('"').next())
                    .is_some_and(|cls| cls.split_whitespace().any(|c| c.contains("gallery")));
                gallery_depth.push(is_gallery);
                return tag.to_string();
            }
            let Some(src) = tag
                .split(" src=\"")
                .nth(1)
                .and_then(|r| r.split('"').next())
            else {
                return tag.to_string();
            };
            if src.contains(':') || src.starts_with('/') || src.starts_with('#') {
                return tag.to_string();
            }
            let file = dir.join(decode(&src.replace("&amp;", "&")));
            if images.get(&file).is_none() {
                return tag.to_string();
            }
            let sizes = if gallery_depth.iter().any(|g| *g) {
                images::SIZES_GRID
            } else {
                images::SIZES_COLUMN
            };
            let attrs = images
                .attrs(&file, &src.replace("&amp;", "&"), sizes)
                .html();
            let rest = SRC_ATTR.replace_all(&tag["<img".len()..], "");
            format!("<img{attrs}{rest}")
        })
        .into_owned()
}

fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && i + 2 < b.len()
            && let Ok(v) =
                u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap_or(""), 16)
        {
            out.push(v);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn host_of(url: &str) -> &str {
    url.split("://").nth(1).unwrap_or(url).trim_end_matches('/')
}

fn rel(content: &Content, path: &Path) -> String {
    format!(
        "content/{}",
        path.strip_prefix(&content.root).unwrap_or(path).display()
    )
}

fn rel_entry(site: &Site, e: &Entry) -> String {
    rel(site.content, &e.dir.join("index.md"))
}

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
    }
    fs::write(path, contents).map_err(|e| Error::io(path, e))
}

/// Copies an entry's assets (everything but index.md and `drafts/`), hard-linking where possible.
fn copy_dir(images: &Images, src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(|e| Error::io(dst, e))?;
    for entry in fs::read_dir(src).map_err(|e| Error::io(src, e))? {
        let entry = entry.map_err(|e| Error::io(src, e))?;
        let name = entry.file_name();
        let from = entry.path();
        if name == "index.md" || name.to_string_lossy().starts_with('.') {
            continue;
        }
        let to = dst.join(&name);
        if from.is_dir() {
            copy_dir(images, &from, &to)?;
        } else {
            if fs::hard_link(&from, &to).is_err() {
                fs::copy(&from, &to).map_err(|e| Error::io(&from, e))?;
            }
            images.publish(&from, &to)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::tests::{entry, tree, write as put};

    fn fixture() -> (tempfile::TempDir, Tree, Content) {
        let dir = tempfile::tempdir().unwrap();
        let r = dir.path();
        put(
            r,
            "curations/tastes/tea/assam/index.md",
            &entry("Assam", "2010-01-01"),
        );
        put(
            r,
            "curations/tastes/tea/ceylon/index.md",
            &entry("Ceylon", "2012-01-01"),
        );
        put(
            r,
            "curations/tastes/tea/drafts/oolong/index.md",
            &entry("Oolong", "2013-01-01"),
        );
        put(
            r,
            "curations/sounds/music/gogo/index.md",
            &entry("GoGo Penguin", "2021-11-20"),
        );
        image::RgbImage::from_pixel(900, 900, image::Rgb([20, 120, 60]))
            .save(r.join("curations/sounds/music/gogo/cover.jpg"))
            .unwrap();
        put(
            r,
            "curations/sounds/music/albums.toml",
            "[[album]]\ntitle = \"Man Made Object\"\nartist = \"GoGo Penguin\"\ncover = \"gogo/cover.jpg\"\nartist_page = \"gogo\"\n",
        );
        put(
            r,
            "considerations/expositions/semiotics/index.md",
            &entry("Semiotics", "2024-06-15"),
        );
        put(
            r,
            "considerations/expositions/cats/index.md",
            "+++\ntitle = \"Cats\"\ndate = 2024-01-01\ndescription = \"x\"\n+++\nSee [semiotics](@/considerations/expositions/semiotics/) and [tea](@/curations/tastes/tea/assam/#notes).\n",
        );
        let t = tree();
        let c = Content::load(r, &t).unwrap();
        (dir, t, c)
    }

    #[test]
    fn monolith_urls_are_paths() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let u = Urls::new(&site, &Target::Monolith);
        assert_eq!(u.node(t.node("curations")), "/curations/");
        assert_eq!(u.node(t.node("music")), "/curations/sounds/music/");
    }

    #[test]
    fn breakout_urls_reroot_and_leave_the_subtree_absolute() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let target = Target::Breakout("tastes".into());
        let u = Urls::new(&site, &target);
        assert_eq!(u.node(t.node("tastes")), "/");
        assert_eq!(u.node(t.node("tea")), "/tea/");
        assert_eq!(
            u.node(t.node("sounds")),
            "https://joshafairhead.com/curations/sounds/"
        );
        let html = u
            .resolve(r#"<a href="@/curations/tastes/tea/assam/#notes">"#)
            .unwrap();
        assert_eq!(html, r#"<a href="/tea/assam/#notes">"#);
        let html = u
            .resolve(r#"<a href="@/considerations/expositions/semiotics/">"#)
            .unwrap();
        assert_eq!(
            html,
            r#"<a href="https://joshafairhead.com/considerations/expositions/semiotics/">"#
        );
    }

    #[test]
    fn links_to_drafts_do_not_resolve() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let u = Urls::new(&site, &Target::Monolith);
        assert!(
            u.resolve(r#"<a href="@/curations/tastes/tea/oolong/">"#)
                .is_err()
        );
        let with_drafts = Site::new(&t, &c, true, &_d.path().join(".cache")).unwrap();
        let u = Urls::new(&with_drafts, &Target::Monolith);
        assert!(
            u.resolve(r#"<a href="@/curations/tastes/tea/oolong/">"#)
                .is_ok()
        );
    }

    #[test]
    fn breakout_crumbs_start_at_the_broken_out_node() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let target = Target::Breakout("sounds".into());
        let u = Urls::new(&site, &target);
        let music = t.node("music");
        let crumbs: Vec<_> = u
            .chrome(music, &music.lineage[..2])
            .crumbs
            .into_iter()
            .map(|c| c.label)
            .collect();
        assert_eq!(crumbs, ["Sounds"]);
        let mono = Urls::new(&site, &Target::Monolith);
        let crumbs: Vec<_> = mono
            .chrome(music, &music.lineage[..2])
            .crumbs
            .into_iter()
            .map(|c| c.label)
            .collect();
        assert_eq!(crumbs, ["joshafairhead.com", "Curations", "Sounds"]);
    }

    #[test]
    fn pills_hide_empty_nodes_and_count_subtrees() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let u = Urls::new(&site, &Target::Monolith);
        let pills: Vec<_> = u
            .child_pills(t.node("curations"))
            .into_iter()
            .map(|p| (p.label, p.count))
            .collect();
        assert_eq!(
            pills,
            [
                ("All".into(), 3),
                ("Sounds".into(), 1),
                ("Tastes".into(), 2)
            ]
        );
        let current: Vec<_> = u
            .sibling_pills(t.node("expositions"))
            .into_iter()
            .filter(|p| p.current)
            .map(|p| p.label)
            .collect();
        assert_eq!(current, ["Expositions"]);
    }

    #[test]
    fn builds_monolith_and_breakout_with_assets_and_passing_links() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let out = tempfile::tempdir().unwrap();
        let mono = build(&site, &Target::Monolith, out.path()).unwrap();
        assert!(mono.join("curations/sounds/music/gogo/cover.jpg").is_file());
        assert!(
            mono.join("curations/sounds/music/gogo/cover.jpg.w480.jpg")
                .is_file()
        );
        let music = fs::read_to_string(mono.join("curations/sounds/music/index.html")).unwrap();
        assert!(
            music.contains("cover.jpg.w480.jpg 480w"),
            "album covers use scaled copies"
        );
        assert!(music.contains(r#"data-full="/curations/sounds/music/gogo/cover.jpg""#));
        assert!(
            !mono.join("curations/tastes/tea/oolong").exists(),
            "drafts are not built"
        );
        let page =
            fs::read_to_string(mono.join("considerations/expositions/cats/index.html")).unwrap();
        assert!(
            page.contains(r#"href="/curations/tastes/tea/assam/#notes""#),
            "{page}"
        );
        let tastes = build(&site, &Target::Breakout("tastes".into()), out.path()).unwrap();
        assert!(tastes.join("tea/assam/index.html").is_file());
        crate::links::check(&mono, &t.site.base_url, None).unwrap();
        crate::links::check(&tastes, &t.site.base_url, Some(&mono)).unwrap();
    }

    #[test]
    fn breakout_of_a_non_breakout_node_is_refused() {
        let (_d, t, c) = fixture();
        let site = Site::new(&t, &c, false, &_d.path().join(".cache")).unwrap();
        let out = tempfile::tempdir().unwrap();
        let err = build(&site, &Target::Breakout("media".into()), out.path()).unwrap_err();
        assert!(matches!(err, Error::NotABreakout(_)));
    }
}
