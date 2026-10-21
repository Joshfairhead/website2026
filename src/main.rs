//! website2026: the static site generator for joshafairhead.com.
//!
//! One content pool, built as a monolith and as any number of breakouts
//! (a node's subtree deployed as its own site). See docs/build-spec.md.

mod config;
mod content;
mod error;
mod links;
mod markdown;
mod render;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use config::Tree;
use content::Content;
use error::Result;
use render::{Site, Target};

#[derive(Parser)]
#[command(version, about = "Static site generator for joshafairhead.com")]
struct Cli {
    /// Path to the site tree.
    #[arg(long, default_value = "site.toml", global = true)]
    config: PathBuf,
    /// Path to the content folder.
    #[arg(long, default_value = "content", global = true)]
    content: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build the monolith, one breakout, or everything.
    Build {
        /// Build only this breakout node's subtree (an id from site.toml).
        #[arg(long, conflicts_with = "all")]
        breakout: Option<String>,
        /// Build the monolith and every breakout.
        #[arg(long)]
        all: bool,
        /// Output directory.
        #[arg(long, default_value = "dist")]
        out: PathBuf,
        /// Include drafts (for local preview only; never deploy this output).
        #[arg(long)]
        drafts: bool,
    },
    /// Validate site.toml, build every target into a temporary folder and check links.
    Check,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let tree = Tree::load(&cli.config)?;
    let content = Content::load(&cli.content, &tree)?;
    match cli.command {
        Command::Build {
            breakout,
            all,
            out,
            drafts,
        } => {
            let targets = match (breakout, all) {
                (Some(id), _) => vec![Target::Breakout(id)],
                (None, true) => all_targets(&tree),
                (None, false) => vec![Target::Monolith],
            };
            let site = Site::new(&tree, &content, drafts)?;
            build_and_check(&site, &targets, &out)
        }
        Command::Check => {
            let tmp = tempfile::tempdir().map_err(|e| error::Error::io(Path::new("<temp>"), e))?;
            let site = Site::new(&tree, &content, false)?;
            build_and_check(&site, &all_targets(&tree), tmp.path())?;
            println!("check passed");
            Ok(())
        }
    }
}

fn all_targets(tree: &Tree) -> Vec<Target> {
    std::iter::once(Target::Monolith)
        .chain(tree.breakouts().map(|n| Target::Breakout(n.id.clone())))
        .collect()
}

fn build_and_check(site: &Site, targets: &[Target], out: &Path) -> Result<()> {
    let mut built = Vec::new();
    for t in targets {
        let dir = render::build(site, t, out)?;
        println!("built {} -> {}", t.dir_name(), dir.display());
        built.push((t, dir));
    }
    let monolith = built
        .iter()
        .find(|(t, _)| **t == Target::Monolith)
        .map(|(_, d)| d.as_path());
    for (_, dir) in &built {
        links::check(dir, &site.tree.site.base_url, monolith)?;
    }
    Ok(())
}
