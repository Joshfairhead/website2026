//! Loading and validating `site.toml`, the site tree.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use serde::Deserialize;

use crate::error::{Error, Result};

/// The three branches. Every node descends from exactly one of these.
pub const BRANCHES: [&str; 3] = ["curations", "considerations", "creations"];

/// Deepest level below the root: branch (1) › category (2) › instance (3).
const MAX_DEPTH: usize = 3;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteConfig {
    pub site: Site,
    #[serde(rename = "node")]
    pub nodes: Vec<NodeConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    pub title: String,
    pub base_url: String,
    pub contact: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeConfig {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
    pub collection: Option<Collection>,
    pub breakout: Option<String>,
}

/// A node whose content is a data collection rather than (only) entry pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Collection {
    /// Artist entries plus albums.toml and tracks.toml.
    Music,
    /// items.toml plus images/.
    Art,
}

/// A validated node with its resolved place in the tree.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub collection: Option<Collection>,
    pub breakout: Option<String>,
    /// Ids from the branch down to this node, inclusive.
    pub lineage: Vec<String>,
}

impl Node {
    pub fn branch(&self) -> &str {
        &self.lineage[0]
    }

    /// Path segments below the monolith root.
    pub fn segments(&self) -> &[String] {
        &self.lineage
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Path below the monolith root, e.g. `curations/sounds/music`.
    pub fn path(&self) -> String {
        self.lineage.join("/")
    }
}

/// The validated tree, nodes kept in `site.toml` order.
#[derive(Debug)]
pub struct Tree {
    pub site: Site,
    order: Vec<String>,
    nodes: HashMap<String, Node>,
}

impl Tree {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        Self::parse(&text, path)
    }

    pub fn parse(text: &str, path: &Path) -> Result<Self> {
        let config: SiteConfig = toml::from_str(text).map_err(|e| Error::Toml {
            path: path.to_path_buf(),
            source: Box::new(e),
        })?;
        Self::from_config(config)
    }

    pub fn from_config(config: SiteConfig) -> Result<Self> {
        let mut problems = Vec::new();
        let mut seen = HashSet::new();
        let mut hosts = HashSet::new();

        for n in &config.nodes {
            if !is_slug(&n.id) {
                problems.push(format!(
                    "node id `{}` must be lowercase letters, digits and hyphens",
                    n.id
                ));
            }
            if !seen.insert(n.id.as_str()) {
                problems.push(format!("node id `{}` is declared twice", n.id));
            }
            if n.label.trim().is_empty() {
                problems.push(format!("node `{}` has an empty label", n.id));
            }
            match &n.parent {
                None if !BRANCHES.contains(&n.id.as_str()) => problems.push(format!(
                    "node `{}` has no parent, but only {} may sit under the root",
                    n.id,
                    BRANCHES.join(", ")
                )),
                Some(_) if BRANCHES.contains(&n.id.as_str()) => {
                    problems.push(format!("branch `{}` must not have a parent", n.id))
                }
                _ => {}
            }
            if let Some(host) = &n.breakout {
                if !is_host(host) {
                    problems.push(format!(
                        "node `{}` breakout `{host}` is not a valid host name",
                        n.id
                    ));
                }
                if !hosts.insert(host.as_str()) {
                    problems.push(format!("breakout host `{host}` is used twice"));
                }
            }
        }
        for b in BRANCHES {
            if !seen.contains(b) {
                problems.push(format!("branch `{b}` is missing"));
            }
        }

        let by_id: HashMap<&str, &NodeConfig> =
            config.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        for n in &config.nodes {
            if let Some(p) = &n.parent
                && !by_id.contains_key(p.as_str())
            {
                problems.push(format!("node `{}` has unknown parent `{p}`", n.id));
            }
        }
        if !problems.is_empty() {
            return Err(Error::Invalid(problems));
        }

        // Resolve lineage, catching cycles and over-deep trees.
        let mut nodes = HashMap::new();
        for n in &config.nodes {
            let mut lineage = vec![n.id.clone()];
            let mut cur = n;
            let mut cycle = false;
            while let Some(p) = &cur.parent {
                if lineage.contains(p) {
                    cycle = true;
                    break;
                }
                lineage.push(p.clone());
                cur = by_id[p.as_str()];
            }
            lineage.reverse();
            if cycle {
                problems.push(format!("node `{}` is part of a parent cycle", n.id));
            } else if lineage.len() > MAX_DEPTH {
                problems.push(format!(
                    "node `{}` is {} levels deep; the tree allows {MAX_DEPTH} (branch › category › instance)",
                    n.id,
                    lineage.len()
                ));
            }
            let children = config
                .nodes
                .iter()
                .filter(|c| c.parent.as_deref() == Some(n.id.as_str()))
                .map(|c| c.id.clone())
                .collect();
            nodes.insert(
                n.id.clone(),
                Node {
                    id: n.id.clone(),
                    label: n.label.clone(),
                    parent: n.parent.clone(),
                    children,
                    collection: n.collection,
                    breakout: n.breakout.clone(),
                    lineage,
                },
            );
        }
        if !problems.is_empty() {
            return Err(Error::Invalid(problems));
        }

        Ok(Tree {
            site: config.site,
            order: config.nodes.iter().map(|n| n.id.clone()).collect(),
            nodes,
        })
    }

    pub fn get(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node(&self, id: &str) -> &Node {
        &self.nodes[id]
    }

    /// All nodes in declaration order.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.order.iter().map(|id| &self.nodes[id])
    }

    pub fn branches(&self) -> impl Iterator<Item = &Node> {
        BRANCHES.iter().map(|b| &self.nodes[*b])
    }

    pub fn breakouts(&self) -> impl Iterator<Item = &Node> {
        self.iter().filter(|n| n.breakout.is_some())
    }

    /// Whether `id` is `ancestor` or sits below it.
    pub fn is_within(&self, id: &str, ancestor: &str) -> bool {
        self.nodes[id].lineage.iter().any(|a| a == ancestor)
    }
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-')
        && !s.ends_with('-')
}

fn is_host(s: &str) -> bool {
    s.contains('.') && s.split('.').all(is_slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = r#"
[site]
title = "T"
base_url = "https://example.com"
contact = "a@example.com"
[[node]]
id = "curations"
label = "Curations"
[[node]]
id = "considerations"
label = "Considerations"
[[node]]
id = "creations"
label = "Creations"
"#;

    fn parse(extra: &str) -> Result<Tree> {
        Tree::parse(&format!("{HEAD}{extra}"), Path::new("site.toml"))
    }

    fn problems(extra: &str) -> Vec<String> {
        match parse(extra) {
            Err(Error::Invalid(p)) => p,
            other => panic!("expected validation problems, got {other:?}"),
        }
    }

    #[test]
    fn real_site_toml_is_valid() {
        let tree = Tree::load(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/site.toml")))
            .expect("site.toml should validate");
        assert_eq!(tree.branches().count(), 3);
        let music = tree.node("music");
        assert_eq!(music.lineage, ["curations", "sounds", "music"]);
        assert_eq!(music.branch(), "curations");
        assert_eq!(music.collection, Some(Collection::Music));
        let breakouts: Vec<_> = tree.breakouts().map(|n| n.id.as_str()).collect();
        assert_eq!(breakouts, ["sights", "sounds", "tastes"]);
    }

    #[test]
    fn children_follow_declaration_order() {
        let tree = parse(
            "[[node]]\nid = \"b\"\nlabel = \"B\"\nparent = \"creations\"\n\
             [[node]]\nid = \"a\"\nlabel = \"A\"\nparent = \"creations\"\n",
        )
        .unwrap();
        assert_eq!(tree.node("creations").children, ["b", "a"]);
    }

    #[test]
    fn rejects_duplicate_ids() {
        let p = problems("[[node]]\nid = \"creations\"\nlabel = \"Again\"\n");
        assert!(p.iter().any(|m| m.contains("declared twice")), "{p:?}");
    }

    #[test]
    fn rejects_unknown_parent() {
        let p = problems("[[node]]\nid = \"x\"\nlabel = \"X\"\nparent = \"nope\"\n");
        assert!(
            p.iter().any(|m| m.contains("unknown parent `nope`")),
            "{p:?}"
        );
    }

    #[test]
    fn rejects_orphan_that_is_not_a_branch() {
        let p = problems("[[node]]\nid = \"stray\"\nlabel = \"Stray\"\n");
        assert!(p.iter().any(|m| m.contains("no parent")), "{p:?}");
    }

    #[test]
    fn rejects_too_deep() {
        let p = problems(
            "[[node]]\nid = \"a\"\nlabel = \"A\"\nparent = \"creations\"\n\
             [[node]]\nid = \"b\"\nlabel = \"B\"\nparent = \"a\"\n\
             [[node]]\nid = \"c\"\nlabel = \"C\"\nparent = \"b\"\n",
        );
        assert!(p.iter().any(|m| m.contains("levels deep")), "{p:?}");
    }

    #[test]
    fn rejects_bad_slug_and_duplicate_host() {
        let p = problems(
            "[[node]]\nid = \"Bad_Id\"\nlabel = \"X\"\nparent = \"creations\"\n\
             [[node]]\nid = \"a\"\nlabel = \"A\"\nparent = \"creations\"\nbreakout = \"a.example.com\"\n\
             [[node]]\nid = \"b\"\nlabel = \"B\"\nparent = \"creations\"\nbreakout = \"a.example.com\"\n",
        );
        assert!(p.iter().any(|m| m.contains("lowercase")), "{p:?}");
        assert!(p.iter().any(|m| m.contains("used twice")), "{p:?}");
    }

    #[test]
    fn rejects_unknown_keys() {
        let err = parse(
            "[[node]]\nid = \"a\"\nlabel = \"A\"\nparent = \"creations\"\ncolour = \"red\"\n",
        )
        .unwrap_err();
        assert!(matches!(err, Error::Toml { .. }), "{err:?}");
    }
}
