use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    Toml {
        path: PathBuf,
        source: Box<toml::de::Error>,
    },
    #[error("site.toml is invalid:\n  - {}", .0.join("\n  - "))]
    Invalid(Vec<String>),
    #[error("unknown node `{0}` (see site.toml)")]
    UnknownNode(String),
    #[error("node `{0}` has no breakout host in site.toml")]
    NotABreakout(String),
    #[error("template: {0}")]
    Render(#[from] askama::Error),
    #[error("broken links in {target}:\n  - {}", .problems.join("\n  - "))]
    Links {
        target: String,
        problems: Vec<String>,
    },
}

impl Error {
    pub fn io(path: &Path, source: std::io::Error) -> Self {
        Error::Io {
            path: path.to_path_buf(),
            source,
        }
    }
}
