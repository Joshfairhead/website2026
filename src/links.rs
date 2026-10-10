//! Post-build link check: every internal `href`/`src` must resolve to a file.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Checks every HTML file under `dir`. Root-relative links resolve inside `dir`;
/// absolute links to `base_url` resolve inside `monolith`, when it has been built.
pub fn check(dir: &Path, base_url: &str, monolith: Option<&Path>) -> Result<()> {
    let mut problems = Vec::new();
    for page in html_files(dir)? {
        let html = fs::read_to_string(&page).map_err(|e| Error::io(&page, e))?;
        let rel_page = page
            .strip_prefix(dir)
            .unwrap_or(&page)
            .display()
            .to_string();
        for link in attr_values(&html) {
            let resolved = if let Some(path) = link.strip_prefix(base_url) {
                match monolith {
                    Some(m) => resolve(m, path),
                    None => continue,
                }
            } else if link.starts_with('/') && !link.starts_with("//") {
                resolve(dir, &link)
            } else {
                continue; // external, mailto:, #anchor and relative links
            };
            if !resolved.is_file() {
                problems.push(format!("{rel_page}: `{link}` does not exist"));
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(Error::Links {
            target: dir.display().to_string(),
            problems,
        })
    }
}

fn resolve(root: &Path, url_path: &str) -> PathBuf {
    let path = url_path.split(['#', '?']).next().unwrap_or("");
    let mut p = root.join(path.trim_start_matches('/'));
    if path.is_empty() || path.ends_with('/') {
        p.push("index.html");
    }
    p
}

/// Values of every `href="…"` and `src="…"` attribute.
fn attr_values(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    for attr in ["href=\"", "src=\""] {
        let mut rest = html;
        while let Some(i) = rest.find(attr) {
            let after = &rest[i + attr.len()..];
            let Some(end) = after.find('"') else { break };
            out.push(after[..end].replace("&amp;", "&"));
            rest = &after[end..];
        }
    }
    out
}

fn html_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d).map_err(|e| Error::io(&d, e))? {
            let path = entry.map_err(|e| Error::io(&d, e))?.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "html") {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_broken_internal_links_and_ignores_external_ones() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("a")).unwrap();
        fs::write(dir.path().join("a/index.html"), "").unwrap();
        fs::write(
            dir.path().join("index.html"),
            r##"<a href="/a/">ok</a><a href="/missing/">bad</a>
               <a href="https://elsewhere.org/x">ext</a><a href="#top">anchor</a>
               <a href="mailto:a@b.c">mail</a>"##,
        )
        .unwrap();
        let err = check(dir.path(), "https://example.com", None).unwrap_err();
        let Error::Links { problems, .. } = err else {
            panic!("{err:?}")
        };
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("/missing/"));
    }

    #[test]
    fn absolute_monolith_links_are_checked_against_the_monolith() {
        let mono = tempfile::tempdir().unwrap();
        let brk = tempfile::tempdir().unwrap();
        fs::create_dir_all(mono.path().join("x")).unwrap();
        fs::write(mono.path().join("x/index.html"), "").unwrap();
        fs::write(
            brk.path().join("index.html"),
            r#"<a href="https://example.com/x/">ok</a><a href="https://example.com/y/">bad</a>"#,
        )
        .unwrap();
        let err = check(brk.path(), "https://example.com", Some(mono.path())).unwrap_err();
        let Error::Links { problems, .. } = err else {
            panic!("{err:?}")
        };
        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("/y/"));
        // Without a monolith build, absolute links can't be checked and are skipped.
        assert!(check(brk.path(), "https://example.com", None).is_ok());
    }
}
