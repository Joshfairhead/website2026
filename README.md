# website2026

The static site generator and content for **joshafairhead.com**: Curations · Considerations · Creations.

One content pool is built as a **monolith** (the whole tree under `joshafairhead.com`) and as **breakouts** (a node's subtree deployed standalone on its own subdomain, e.g. `sounds.joshafairhead.com`). The tree lives in [`site.toml`](site.toml).

## Use

```sh
cargo run -- build                    # monolith -> dist/monolith
cargo run -- build --breakout sounds  # one breakout -> dist/sounds
cargo run -- build --all              # monolith + every breakout
cargo run -- check                    # validate, build everything to a temp dir, check links
```

Every build validates `site.toml` and fails on unknown keys, bad ids, unknown parents, a tree deeper than branch › category › instance, duplicate breakout hosts, or any internal link that does not resolve.

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

Phase P0 (scaffold) is done: the tree, validation, CLI, page shells for every node, the business card root, link checking and CI. Content loading and migration (P1) come next; see the build spec.
