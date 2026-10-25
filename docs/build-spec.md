# Build Spec — Rust static site generator

Status: **confirmed in principle**. Implementation waits on the design phase. Design system: https://claude.ai/artifact/6nFYsUWpdXBHbqCdVQjCAa (tokens, brand book, components; the templates implement it).
Inputs: [`information-architecture.md`](information-architecture.md) and [`content-inventory.md`](content-inventory.md).

## 1. Goals and non-goals

**Goals**
- A from-scratch static site generator, written in Rust, in the repo **`website2026`**. From one content pool it builds:
  - the **monolith** (the whole tree under `joshafairhead.com`);
  - any number of **breakouts** (a single node's subtree as a standalone site on its own subdomain).
- Content is **typed and validated**: bad content fails the build with a clear error and never ships broken pages.
- The structure mirrors the current static-site layout loosely (Markdown + TOML front matter, a content folder per node, co-located images), so content moves with minimal rewriting.
- Pages are fully usable without JavaScript. JS only enhances filtering, sorting and lightboxes.

**Non-goals (v1)**
- Visual design (a separate pass; v1 ships a clean, neutral theme).
- CMS or editing UI, comments, search, RSS.
- Image formats beyond JPEG/PNG for scaled copies (WebP/AVIF output could come later).

## 2. Stack

| Concern | Choice | Why |
|---|---|---|
| Language | Rust stable, pinned via `rust-toolchain.toml` | Your requirement |
| Markdown | `pulldown-cmark` (footnotes, tables, smart punctuation, heading anchors) | Matches current Markdown features |
| Front matter + data | `serde` + `toml`, with `deny_unknown_fields` on every struct | The current `+++` TOML front matter carries over; typos fail the build |
| Templates | `askama` | Templates are checked at compile time: a missing field or typo is a compile error, not a broken page |
| Code blocks | Plain, styled `<pre>` (no highlighter) | Only 2 pages contain code; add `syntect` if that grows |
| CLI | `clap`: `build`, `serve`, `check`, `new` | |
| Dev server | `axum` + `notify` (rebuild on change, live reload) | |
| Errors | `thiserror` / `miette` (file + line in messages) | Readable failures |
| Styles / JS | Plain CSS with custom properties; one small vanilla JS module; no bundler, no Sass | Fewer moving parts |

## 3. Repository layout

```
Cargo.toml / rust-toolchain.toml
src/            main.rs · config.rs · content/ (load, schema, validate) · render/ · shortcodes.rs · links.rs · serve.rs
templates/      askama templates (base, card, recipient, listing, entry, gallery, music)
static/         css/ · js/ · fonts/ · favicon
site.toml       the tree: triads, categories, instances, collections, breakouts (single source of truth)
content/
  root/         _index.md (business card) · for/<org>.md
  curations/sights/art/          items.toml + images/
  curations/sounds/music/        artists/<slug>/index.md · albums.toml · tracks.toml · drafts/
  curations/tastes/tea/          <slug>/index.md
  curations/thoughts/books/      …
  curations/experiences/travel/  … · drafts/
  considerations/{expositions,retrospectives,practices}/
  creations/{software,recordings,media,ventures}/
  drafts/       site-level drafts (e.g. former timeline/)
tests/          fixtures + snapshot tests
.github/workflows/ci.yml
```

A content folder's path *is* its node. An entry outside a node declared in `site.toml` is a build error.

## 4. Content model

**`site.toml`** declares each node's id, label, parent, an optional `collection` (`music` or `art`), and an optional `breakout = "sights.joshafairhead.com"`. Menus, breadcrumbs, listings and breakout targets all derive from it.

**Entry front matter** (common to all entries):
- Required: `title`, `date`, `description`.
- Optional: `tags`, `card` (listing image), `banner` (shown at the top of the entry), `toc`.

`hero`, `weight` and `featured` were dropped in migration: nothing ordered by weight (every section sorted by date) and `hero = false` is expressed by leaving `banner` out.

`authors` is dropped, since the site has a single author. Per-page `styles`/`scripts` are dropped: components come from the entry's node. Fields specific to a node type (e.g. tea) are declared on that type and validated the same way.

**Data collections** (records, not pages):
- `art/items.toml`: `id, group, artist, title, source, width, height, file`.
- `music/albums.toml` / `tracks.toml`: `title, artist, link, cover?, artist_page?`. Here `artist_page` must resolve to an artist entry.

**Recipient cards** (`root/for/<org>.md`): `title`, `tagline`, `links[] {label, url}`, the same shape as in the hub.

**Shortcodes**: `image(...)`, `youtube(...)` and `vimeo(...)` keep their current names and arguments. An unknown shortcode is a build error. Inline HTML in Markdown passes through.

## 5. Validation: the build fails on

1. TOML/Markdown parse errors, unknown or missing fields, invalid dates.
2. Any referenced local file that does not exist: card, hero, banner, shortcode image, data-record file.
3. Duplicate slugs within a node, or an entry outside a declared node.
4. Broken internal links or anchors in the rendered output. Every `href`/`src` is checked after rendering.
5. Malformed external URLs (syntax only, no network), e.g. the existing `https:/https://…`.
6. Unresolved references (`artist_page`, facet values not declared in `site.toml`).
7. Anything under `drafts/` linked from published content.

Empty nodes produce a warning, not an error, and are hidden from menus.

## 6a. Images

Every JPEG, PNG and WebP in published content gets scaled copies at 480 px and 1200 px wide (`src/images.rs`), never larger than the original. Pages use them through `srcset`/`sizes` with `width`/`height` set, so the browser fetches the smallest copy that fits and the layout doesn't shift; the lightbox opens the original (`data-full`). Copies are JPEG at quality 78, or PNG when the image is actually transparent, with EXIF orientation applied.

Copies are cached in `.cache/images/`, keyed by the original's content, and hard-linked into each target. A cold build makes about 2,500 images' copies in roughly a minute on four cores; a warm build skips them. An image that can't be decoded fails the build, and the link check covers every `srcset` entry.

`netlify.toml` caches images in browsers for a week and revalidates the stylesheet and script on every visit.

## 6. Pages rendered

| Page | Content |
|---|---|
| Root | Business card: name, contact email, three triad links |
| `/for/<org>` | Recipient card (tagline + links) |
| Triad / category / instance | Breadcrumb, level menu (children of the current node), and a card grid of all published descendants |
| Entry | Hero, title, date, reading time, optional ToC, body, breadcrumb back up |
| Art | Masonry gallery with lightbox; filter chips by group; artist and source on each caption |
| Music | Facet switch Artists / Albums / Tracks; artist cards link to artist pages; album and track records link out |
| 404 | Per site, with a link to root |

Listings sort by date, newest first, by default. Client-side sort (date / title) and tag or facet filtering are progressive enhancements over a server-rendered list.

## 7. URLs and hosting

- **Build targets:**
  - `build` writes the monolith to `dist/monolith/`, laid out by path (`/curations/sights/art/…`).
  - `build --breakout sights` writes that node's subtree to `dist/sights/`, re-rooted (`/art/…`). `build --all` builds the monolith plus every flagged breakout.
- **Re-rooting:** in a breakout, the node is the root. The breadcrumb starts there, with one "part of joshafairhead.com" link up to the monolith. Links to entries outside the subtree point at their monolith URL. Link validation runs per target.
- **Canonical URLs:** a page's canonical is its monolith URL, so search engines don't treat breakouts as duplicates. *(Can flip to the breakout URL per node if you prefer.)*
- **Local preview:** `serve` previews the monolith, and `serve --breakout sights` previews one breakout, both on `localhost`.
- **Hosting:** Netlify, one Netlify site per target. Each site builds this repo itself (`netlify.toml` for the monolith; breakout sites set `--breakout <id>` and `dist/<id>` in their own settings). Netlify installs the toolchain from `rust-toolchain.toml`, and the build runs the full validation and link check, so a broken build fails and never deploys. CI on GitHub runs the same checks on every push. This replaced the earlier plan of deploying from GitHub Actions with a `NETLIFY_AUTH_TOKEN`: no secret is needed.
- **Existing hosts** (`creative.`, `blog.`, `portfolio.`, `tea.`) and their deploys are not touched.

## 8. Quality gates (CI on every push / PR)

1. `cargo fmt --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`: unit tests for the parsers and validators, plus snapshot tests (`insta`) of rendered fixture pages.
4. `cargo run -- build --all`: the full validation in §5 over the real content, for the monolith and every breakout. Every problem is an error; there are no warnings to promote.
5. The built-in link check over `dist/`.

A red CI blocks merge and deploy. Locally, `cargo run -- check` runs 3–5.

## 9. Migration (one-off) ✅

Done with `tools/migrate.py` (kept for the record). What it changed beyond moving files:

- Photos over 1 MB were downscaled to 2400 px, then JPEG/WebP files over 400 KB to 2000 px at quality 82. The originals stay in the `creative` repo.
- Broken references in the old site were fixed: `Culto.jpeg` → `culto.jpeg` (case only worked on macOS), a `12perseverence.jpeg` that never existed was dropped from India 4, Hermitage's screenshots moved from a shared folder into the entry, the Iron Butterfly draft's missing card now uses `Ball.jpg`, and the Elton John album link `https:/https://…` was repaired.
- Links to pages that became drafts (Newquay PAR, Cornwall Reflections) keep their text and lose the link; the old `/semioticsinbrief` link points at Semiotics in Brief.
- Site-level drafts (`content/drafts/`, the former timeline) are loaded and validated but not rendered, even with `--drafts`, because they have no node yet.

Original plan, for reference:

A `tools/migrate` script (run once, then deleted) moves content from the Zola layout into the new tree, following `content-inventory.md` §6:
- Moves folders, and sets drafts by location instead of flags.
- Converts the album-gallery HTML and "Special Mentions" into `albums.toml` / `tracks.toml`.
- Imports `items.json` and its images from the Aesthetics Board.
- Imports the business card and `/for/` pages from `Joshfairhead/hub`.

The migrated content must pass §5 before Zola files are removed.

## 10. Phases and acceptance

| Phase | Delivers | Done when |
|---|---|---|
| P0 Scaffold ✅ | Cargo project, CLI skeleton, CI pipeline | CI green on an empty site |
| P1 Model ✅ | `site.toml`, schemas, loader, validation, migration | All 111 pages + records load; validation passes; deliberate bad fixtures fail with clear errors |
| P2 Render ✅ | Templates, menus, breadcrumbs, listings, entry pages, shortcodes | Every node and entry renders; link check passes; snapshots reviewed |
| P3 Collections ✅ | Art gallery, music facets, recipient cards | 130 images, 64 albums and 14 tracks render with filters; works without JS |
| P4 Deploy | `netlify.toml` ✅; link the joshafairhead.com Netlify site to this repo; breakout sites + DNS | `joshafairhead.com` serves the monolith, and each breakout host serves its subtree standalone |
| P5 Cleanup | Point `joshafairhead.com` at the new monolith (replacing the `hub` deploy) | The hub is superseded; the old Zola hosts are left untouched |

## 11. Decided / open

- **Decided:** Netlify + GitHub Actions; new repo `website-2026`; old hosts left untouched; Sights, Sounds and Tastes are the first breakouts.
- **Open:** `cv.` and `facilitation.` stay outside for now. Canonical URL policy for breakouts (default: the monolith URL).
