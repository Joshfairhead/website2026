//! Rendering targets (the monolith and each breakout) to static HTML.

use std::fs;
use std::path::{Path, PathBuf};

use askama::Template;

use crate::config::{Node, Tree};
use crate::error::{Error, Result};

const STYLESHEET: &str = include_str!("../static/site.css");

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

pub struct Link {
    pub label: String,
    pub href: String,
}

pub struct MenuItem {
    pub label: String,
    pub href: String,
    pub current: bool,
}

#[derive(Template)]
#[template(path = "card.html")]
struct CardPage<'a> {
    title: &'a str,
    contact: &'a str,
    branches: Vec<BranchLink>,
}

struct BranchLink {
    id: String,
    label: String,
    href: String,
}

#[derive(Template)]
#[template(path = "node.html")]
struct NodePage<'a> {
    title: &'a str,
    site_title: &'a str,
    branch: &'a str,
    crumbs: Vec<Link>,
    home: Option<Link>,
    menu: Vec<MenuItem>,
    facets: &'a [String],
}

/// Builds one target into `out/<target dir>`. Returns the directory written.
pub fn build(tree: &Tree, target: &Target, out: &Path) -> Result<PathBuf> {
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
    let urls = Urls { tree, target };

    if *target == Target::Monolith {
        let page = CardPage {
            title: &tree.site.title,
            contact: &tree.site.contact,
            branches: tree
                .branches()
                .map(|b| BranchLink {
                    id: b.id.clone(),
                    label: b.label.clone(),
                    href: urls.node(b),
                })
                .collect(),
        };
        write(&dir.join("index.html"), &page.render()?)?;
    }

    for node in tree.iter() {
        if let Some(root) = target.root()
            && !tree.is_within(&node.id, root)
        {
            continue;
        }
        let page = NodePage {
            title: &node.label,
            site_title: &tree.site.title,
            branch: node.branch(),
            crumbs: urls.crumbs(node),
            home: target.root().map(|_| Link {
                label: host_of(&tree.site.base_url).to_string(),
                href: format!("{}/", tree.site.base_url),
            }),
            menu: urls.menu(node),
            facets: &node.facets,
        };
        let rel = urls.local_path(node).expect("node is within the target");
        write(&dir.join(rel).join("index.html"), &page.render()?)?;
    }

    write(&dir.join("site.css"), STYLESHEET)?;
    Ok(dir)
}

struct Urls<'a> {
    tree: &'a Tree,
    target: &'a Target,
}

impl Urls<'_> {
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
            None => format!("{}/{}/", self.tree.site.base_url, node.segments().join("/")),
        }
    }

    /// Ancestors shown in the breadcrumb, excluding the current node.
    fn crumbs(&self, node: &Node) -> Vec<Link> {
        let mut out = Vec::new();
        if self.target.root().is_none() {
            out.push(Link {
                label: host_of(&self.tree.site.base_url).to_string(),
                href: "/".to_string(),
            });
        }
        let ancestors = &node.lineage[..node.lineage.len() - 1];
        for id in ancestors {
            let a = self.tree.node(id);
            if self.local_path(a).is_some() {
                out.push(Link {
                    label: a.label.clone(),
                    href: self.node(a),
                });
            }
        }
        out
    }

    /// The level menu: a node's children, or for a leaf without facets,
    /// its siblings with itself marked current.
    fn menu(&self, node: &Node) -> Vec<MenuItem> {
        let (ids, current) = if !node.children.is_empty() {
            (&node.children, None)
        } else if !node.facets.is_empty() {
            return Vec::new();
        } else {
            match &node.parent {
                Some(p) if self.local_path(self.tree.node(p)).is_some() => {
                    (&self.tree.node(p).children, Some(node.id.as_str()))
                }
                _ => return Vec::new(),
            }
        };
        ids.iter()
            .map(|id| {
                let n = self.tree.node(id);
                MenuItem {
                    label: n.label.clone(),
                    href: self.node(n),
                    current: Some(id.as_str()) == current,
                }
            })
            .collect()
    }
}

fn host_of(url: &str) -> &str {
    url.split("://").nth(1).unwrap_or(url).trim_end_matches('/')
}

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
    }
    fs::write(path, contents).map_err(|e| Error::io(path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> Tree {
        Tree::load(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/site.toml"))).unwrap()
    }

    #[test]
    fn monolith_urls_are_paths() {
        let t = tree();
        let u = Urls {
            tree: &t,
            target: &Target::Monolith,
        };
        assert_eq!(u.node(t.node("curations")), "/curations/");
        assert_eq!(u.node(t.node("music")), "/curations/sounds/music/");
    }

    #[test]
    fn breakout_urls_reroot_and_leave_the_subtree_absolute() {
        let t = tree();
        let target = Target::Breakout("sounds".into());
        let u = Urls {
            tree: &t,
            target: &target,
        };
        assert_eq!(u.node(t.node("sounds")), "/");
        assert_eq!(u.node(t.node("music")), "/music/");
        assert_eq!(
            u.node(t.node("tastes")),
            "https://joshafairhead.com/curations/tastes/"
        );
    }

    #[test]
    fn breakout_crumbs_start_at_the_broken_out_node() {
        let t = tree();
        let target = Target::Breakout("sounds".into());
        let u = Urls {
            tree: &t,
            target: &target,
        };
        let crumbs: Vec<_> = u
            .crumbs(t.node("music"))
            .into_iter()
            .map(|c| c.label)
            .collect();
        assert_eq!(crumbs, ["Sounds"]);
        let mono = Urls {
            tree: &t,
            target: &Target::Monolith,
        };
        let crumbs: Vec<_> = mono
            .crumbs(t.node("music"))
            .into_iter()
            .map(|c| c.label)
            .collect();
        assert_eq!(crumbs, ["joshafairhead.com", "Curations", "Sounds"]);
    }

    #[test]
    fn leaf_menu_shows_siblings_with_current_marked() {
        let t = tree();
        let u = Urls {
            tree: &t,
            target: &Target::Monolith,
        };
        let menu = u.menu(t.node("media"));
        let current: Vec<_> = menu
            .iter()
            .filter(|m| m.current)
            .map(|m| m.label.as_str())
            .collect();
        assert_eq!(menu.len(), 4);
        assert_eq!(current, ["Media"]);
        assert!(
            u.menu(t.node("music")).is_empty(),
            "faceted leaves show facets instead"
        );
    }

    #[test]
    fn breakout_of_a_non_breakout_node_is_refused() {
        let t = tree();
        let out = tempfile::tempdir().unwrap();
        let err = build(&t, &Target::Breakout("media".into()), out.path()).unwrap_err();
        assert!(matches!(err, Error::NotABreakout(_)));
    }
}
