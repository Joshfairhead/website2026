//! Markdown to HTML: shortcodes, heading anchors, table of contents, word count.

use std::collections::HashMap;
use std::sync::LazyLock;

use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use regex::Regex;

static SHORTCODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{\{\s*(\w+)\s*\((.*?)\)\s*\}\}").unwrap());
static ARG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w+)\s*=\s*(?:"([^"]*)"|(true|false|\d+))"#).unwrap());
static TAGS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());

#[derive(Debug)]
pub struct Rendered {
    pub html: String,
    pub toc: Vec<Heading>,
    pub words: usize,
}

#[derive(Debug)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

impl Rendered {
    /// Whole minutes at 200 words a minute, at least one.
    pub fn reading_minutes(&self) -> usize {
        self.words.div_ceil(200).max(1)
    }
}

/// Renders an entry body. Errors name the offending shortcode.
pub fn render(body: &str) -> Result<Rendered, String> {
    let expanded = expand_shortcodes(body)?;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut events: Vec<Event> = Parser::new_ext(&expanded, opts).collect();
    let mut toc = Vec::new();
    let mut used: HashMap<String, usize> = HashMap::new();
    let mut words = 0;
    let mut i = 0;
    while i < events.len() {
        match &events[i] {
            Event::Text(t) | Event::Code(t) => words += t.split_whitespace().count(),
            Event::Html(h) | Event::InlineHtml(h) => {
                words += TAGS.replace_all(h, " ").split_whitespace().count()
            }
            _ => {}
        }
        if let Event::Start(Tag::Heading {
            level, id: None, ..
        }) = &events[i]
        {
            let level = *level;
            let mut text = String::new();
            let mut j = i + 1;
            while j < events.len() && !matches!(events[j], Event::End(TagEnd::Heading(_))) {
                if let Event::Text(t) | Event::Code(t) = &events[j] {
                    text.push_str(t);
                }
                j += 1;
            }
            let base = slugify(&text);
            let n = used.entry(base.clone()).or_insert(0);
            let id = if *n == 0 {
                base.clone()
            } else {
                format!("{base}-{n}")
            };
            *n += 1;
            if matches!(
                level,
                HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3
            ) {
                toc.push(Heading {
                    level: level as u8,
                    id: id.clone(),
                    text: text.clone(),
                });
            }
            if let Event::Start(Tag::Heading { id: slot, .. }) = &mut events[i] {
                *slot = Some(CowStr::from(id));
            }
        }
        i += 1;
    }
    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    Ok(Rendered {
        html: out,
        toc,
        words,
    })
}

fn expand_shortcodes(body: &str) -> Result<String, String> {
    let mut err = None;
    let out = SHORTCODE.replace_all(body, |c: &regex::Captures| {
        let name = &c[1];
        let mut args: HashMap<&str, String> = HashMap::new();
        for a in ARG.captures_iter(c.get(2).map_or("", |m| m.as_str())) {
            let v = a.get(2).or(a.get(3)).map_or("", |m| m.as_str());
            args.insert(a.get(1).unwrap().as_str(), v.to_string());
        }
        match shortcode(name, &args) {
            Ok(html) => html,
            Err(e) => {
                err.get_or_insert(format!("`{}`: {e}", &c[0]));
                String::new()
            }
        }
    });
    match err {
        Some(e) => Err(e),
        None => Ok(out.into_owned()),
    }
}

fn shortcode(name: &str, args: &HashMap<&str, String>) -> Result<String, String> {
    let need = |k: &str| args.get(k).cloned().ok_or(format!("missing `{k}`"));
    match name {
        "image" => {
            let url = need("url")?;
            let alt = args.get("alt").cloned().unwrap_or_default();
            Ok(format!(
                r#"<img src="{}" alt="{}" loading="lazy" decoding="async">"#,
                attr(&url),
                attr(&alt)
            ))
        }
        "youtube" => {
            let id = need("id")?;
            Ok(embed(
                &format!("https://www.youtube-nocookie.com/embed/{}", attr(&id)),
                "YouTube video",
            ))
        }
        "vimeo" => {
            let id = need("id")?;
            let src = match args.get("h") {
                Some(h) => format!("https://player.vimeo.com/video/{}?h={}", attr(&id), attr(h)),
                None => format!("https://player.vimeo.com/video/{}", attr(&id)),
            };
            Ok(embed(&src, "Vimeo video"))
        }
        other => Err(format!(
            "unknown shortcode `{other}` (image, youtube and vimeo are supported)"
        )),
    }
}

fn embed(src: &str, title: &str) -> String {
    format!(
        r#"<div class="embed"><iframe src="{src}" title="{title}" loading="lazy" allow="fullscreen; picture-in-picture" allowfullscreen></iframe></div>"#
    )
}

fn attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

pub fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in text.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() {
            out.push(ch);
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    let trimmed = out.trim_end_matches('-');
    if trimmed.is_empty() {
        "section".into()
    } else {
        trimmed.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_shortcodes() {
        let r = render(
            "{{ image(url=\"a b.jpg\", alt=\"Kyoto\", no_hover=true) }}\n\n{{ youtube(id=\"abc\") }}\n\n{{ vimeo(id=\"123?h=9f\") }}\n",
        )
        .unwrap();
        assert!(r.html.contains(r#"<img src="a b.jpg""#), "{}", r.html);
        assert!(r.html.contains("youtube-nocookie.com/embed/abc"));
        assert!(r.html.contains("player.vimeo.com/video/123?h=9f"));
    }

    #[test]
    fn unknown_or_incomplete_shortcodes_fail() {
        assert!(
            render("{{ gallery(x=1) }}")
                .unwrap_err()
                .contains("unknown shortcode")
        );
        assert!(
            render("{{ image(alt=\"x\") }}")
                .unwrap_err()
                .contains("missing `url`")
        );
    }

    #[test]
    fn headings_get_unique_ids_and_toc() {
        let r = render("# Intro\n\ntext\n\n## Intro\n\n### Café & Tea\n").unwrap();
        assert!(r.html.contains(r#"<h1 id="intro">"#), "{}", r.html);
        assert!(r.html.contains(r#"<h2 id="intro-1">"#));
        let ids: Vec<_> = r.toc.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, ["intro", "intro-1", "café-tea"]);
    }

    #[test]
    fn counts_words_in_text_and_html() {
        let r = render("one two three\n\n<div>four five</div>\n").unwrap();
        assert_eq!(r.words, 5);
        assert_eq!(r.reading_minutes(), 1);
    }
}
