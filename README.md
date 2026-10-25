# website2026

The static site generator and content for **joshafairhead.com**: Curations · Considerations · Creations.

One content pool is built as a **monolith** (the whole tree under `joshafairhead.com`) and as **breakouts** (a node's subtree deployed standalone on its own subdomain, e.g. `sounds.joshafairhead.com`). The tree lives in [`site.toml`](site.toml).

## Use

```sh
cargo run --release -- build                    # monolith -> dist/monolith
cargo run --release -- build --drafts           # include drafts (local preview only)
python3 -m http.server -d dist/monolith         # then open http://localhost:8000
```

Other targets:

```sh
cargo run -- build                    # monolith -> dist/monolith
cargo run -- build --breakout sounds  # one breakout -> dist/sounds
cargo run -- build --all              # monolith + every breakout
cargo run -- check                    # validate, build everything to a temp dir, check links
```

Images: every photo gets 480 px and 1200 px copies used by `srcset`; originals open in the lightbox. Copies are cached in `.cache/images/` (safe to delete).

Every build validates `site.toml` and `content/` and fails, naming the file, on: unknown or missing front-matter fields, bad dates, missing card/banner/cover/art files, entries outside a leaf node, unknown shortcodes, malformed URLs, links from published pages to drafts, and any internal link or image that does not resolve in the built output.

## Content

```
content/for/<org>.md                    recipient cards (/for/<org>/)
content/<node path>/<slug>/index.md     an entry; its images sit beside it
content/<node path>/drafts/<slug>/      drafts of that node (not published)
content/drafts/<slug>/                  drafts with no node yet
content/curations/sounds/music/albums.toml, tracks.toml
content/curations/sights/art/items.toml, images/
```

Front matter (`+++` TOML): `title`, `date` (YYYY-MM-DD) and `description` are required; `tags`, `card`, `banner` and `toc` are optional. Link to another page with `@/<node path>/<slug>/`; it resolves correctly in the monolith and in every breakout. Shortcodes: `{{ image(url="x.jpg", alt="…") }}`, `{{ youtube(id="…") }}`, `{{ vimeo(id="…") }}`.

## Checks

CI (`.github/workflows/ci.yml`) runs on every push and pull request. Run the same checks locally before pushing:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -- build --all
```

## Docs

- [Information architecture](docs/information-architecture.md): the tree, navigation and breakout rules
- [Content inventory](docs/content-inventory.md): everything that exists today and where it goes
- [Build spec](docs/build-spec.md): how this generator works, phase by phase
- Design system: https://claude.ai/artifact/6nFYsUWpdXBHbqCdVQjCAa · page mockups: https://claude.ai/artifact/APpjD6pBB3iPqxNotukugo

## Status

Phases P0–P3 are done: the tree, all content migrated (111 pages, 12 drafts, 64 albums, 14 tracks, 130 artworks, 2 recipient cards), every page type rendered, filters and lightbox, link checking and CI. Deployment: `netlify.toml` builds the monolith on Netlify; breakout sites use the same repo with `--breakout <id>` (see the build spec).
