# Content Inventory — current estate → new schema

Companion to [`information-architecture.md`](information-architecture.md) and [`build-spec.md`](build-spec.md). This lists everything that exists today and where it lands. Snapshot taken at commit `c3541f8`.

## 1. Current estate

All content is assimilated into **one codebase and one monolith**, from which nodes can be broken out onto subdomains. Existing hosts are left untouched.

| # | Property | Source | What it is today | Fate |
|---|---|---|---|---|
| 1 | **joshafairhead.com** | `Joshfairhead/hub` (Zola) | Business card: name + `hello@joshafairhead.com`. Also `/for/<org>` recipient cards with a tagline and tailored links (`acme`, `ngo-org`) | Becomes the root of the new site. Card gains the three triad links; `/for/<org>` pages kept |
| 2 | **creative.joshafairhead.com** | this repo, `config.toml` | All 10 sections as one flat grid with tag filter and sort | **Left untouched.** Its content is copied into the new tree |
| 3 | **blog.joshafairhead.com** | this repo, `config.blog.toml` | books, music, study, travel | **Left untouched** |
| 4 | **portfolio.joshafairhead.com** | this repo, `config.portfolio.toml` | multimedia, recordings, software, tech-management, timeline | **Left untouched** |
| 5 | **tea.joshafairhead.com** | this repo, `config.tea.toml` | tea | **Left untouched.** The new `tastes.` breakout covers the same ground |
| 6 | **Aesthetics Board** | Claude artifact https://claude.ai/artifact/U2uwxKnSU2PrevNQqKd8mK | 130 images, Collection/Screening views | Its images become Curations › Sights › Art |
| 7 | **cv.joshafairhead.com**, **facilitation.joshafairhead.com** | Unknown (linked from the hub's `/for/` pages; candidate repos `Joshfairhead/CV`, `Joshfairhead/visualfacilitation`) | Not inventoried | **Open:** see build spec §11 |

## 2. Infrastructure being replaced

| Item | Today | Replacement |
|---|---|---|
| Zola (`config*.toml`, `themes/duckquill`, `templates/`, `sass/`) | Prototype site generator and theme | Rust build tool in the new `website-2026` repo (see build spec). This repo stays as is |
| `build-views.sh`, `build.sh`, `serve-view.sh` | Overlapping subset-copy scripts | A single `cargo run -- build` / `serve` |
| `netlify.toml` (here and in `hub`) | Builds only the creative view; other hosts are configured outside the repo | New repo: one monolith deploy, plus one deploy per breakout. This repo's deploys are left as they are |
| `static/tag-filter.js`, `travel-lightbox.js`, `gallery.js` and per-page CSS | Per-page scripts and styles | Shared components: facet filter, gallery/lightbox, album grid |
| `static/gallery-editor*.html` | Local gallery tooling | Not carried over; can be re-added as a dev tool |
| Front-matter flags `draft`, `hide_from_home` | 10 drafts, 2 hidden entries | Replaced by `drafts/` folders. There is no "hidden" state |
| Shortcodes `image()` (~1,870 uses), `youtube()` (19), `vimeo()` (5) | Zola shortcodes | Supported by the new build with the same names and arguments, so content moves without rewriting |
| Inline HTML in 101 pages (album galleries, etc.) | Raw HTML in Markdown | Passed through as-is. Album galleries are converted to music data (§4) |

## 3. Totals by new node

| New node | Source | Published | `drafts/` |
|---|---|---|---|
| Curations › Sights › Art | Aesthetics Board | 130 images (6 groups) | — |
| Curations › Sounds › Music | `music/` | 21 artists · 64 albums · 14 tracks | 1 (Iron Butterfly) |
| Curations › Tastes › Tea | `tea/` | 18 | — |
| Curations › Thoughts › Books | `books/` | 8 | — |
| Curations › Experiences › Travel | `travel/` | 17 | 2 (Cornwall Reflections, Newquay PAR) |
| Considerations › Expositions | `study/` (part) | 4 | 2 (both cosmic-ecology pieces) |
| Considerations › Retrospectives | `study/` (part) | 5 | — |
| Considerations › Practices | `study/` (part) | 2 | — |
| Creations › Software | `software/` | 8 | 1 (Hackalong) |
| Creations › Recordings | `recordings/` | 14 | 1 (Urban Big Band) |
| Creations › Media | `multimedia/` | 6 | — |
| Creations › Ventures | `tech-management/` | 8 | — |
| Site-level `drafts/` | `timeline/` | — | 5 (all of Career) |
| **Total** | | **111 pages + 130 images + 78 music records** | **12** |

There are no orphans: every current entry has exactly one new home.

## 4. Music → Artists · Albums · Tracks

The Music instance is filterable by **Artists / Albums / Tracks**. Songs and tracks are treated as the same thing.

| Facet | Source | Count |
|---|---|---|
| Artists | Artist pages in `music/` (22 pages, 1 draft) | 21 |
| Albums | `album-item` blocks in artist pages (58) + "Special Mentions" § Albums (6) | 64 |
| Tracks | "Special Mentions" § Tracks | 14 |

Migration:
- Each album or track becomes a structured record with title, artist, link, optional cover, and an optional link to the artist's page.
- Artist pages keep their prose, and their album grids render from the records.
- The "Special Mentions" page (`music/all`) is dissolved into records and no longer exists as a page.
- One link is broken today: the Elton John album has a malformed URL (`https:/https://…`). The new build's link validation will catch errors like this.

## 5. Aesthetics Board → Curations › Sights › Art

All 130 images are live: the board's `removed` and `screening` collections are empty. Each item has the fields `id, section, artist, title, source, w, h, file`.

| Board group | Images | Artists |
|---|---|---|
| Neo-Tantric | 53 | Mahirwan Mamtani (30), G.R. Santosh (14), Biren De (9) |
| Japanese | 23 | Kawase Hasui (18), Hiroshi Yoshida (3), Hiroo Isono (1), unidentified shin-hanga (1) |
| Nat Girsberger | 17 | Nat Girsberger |
| All India Radio | 16 | All India Radio |
| Leo Kenney | 13 | Leo Kenney |
| Generative | 8 | Hamonshū (@MoriYuzan) |

Migration:
- `items.json` becomes the Art data file, and the image files are exported into the repo.
- Groups and artists are facets within Art.
- The Keep/Pass screening workflow stays in the artifact as private curation tooling; it is not part of the public site.

## 6. Per-entry mapping

"Today" shows the current flags. "→ New home" gives the node, with `drafts/` marking entries that move into that node's drafts folder.

### `music/` — "Music" · 23 entries · today on: creative, blog

| Entry | Title | Today | → New home |
|---|---|---|---|
| `all` | Special Mentions |  | Dissolved into Music › Albums (6) + Tracks (14) records |
| `avishicohen` | Avishi Cohen |  | Curations › Sounds › Music › Artists |
| `brianwilson` | Brian Wilson |  | Curations › Sounds › Music › Artists |
| `catstevens` | Cat Stevens |  | Curations › Sounds › Music › Artists |
| `fatfreddiesdrop` | Fat Freddies Drop |  | Curations › Sounds › Music › Artists |
| `floatingpoint` | Floating Points |  | Curations › Sounds › Music › Artists |
| `ghost` | Ghost |  | Curations › Sounds › Music › Artists |
| `gogopenguin` | GoGo Penguin |  | Curations › Sounds › Music › Artists |
| `hilltophoods` | Hilltop Hoods |  | Curations › Sounds › Music › Artists |
| `ironbutterfly` | Iron Butterfly | draft | Curations › Sounds › Music › Artists › `drafts/` |
| `justice` | Justice |  | Curations › Sounds › Music › Artists |
| `killitkid` | Kill it Kid |  | Curations › Sounds › Music › Artists |
| `maynardjameskenan` | Maynard James Keenan |  | Curations › Sounds › Music › Artists |
| `nilerodgers` | Nile Rodgers |  | Curations › Sounds › Music › Artists |
| `robertfripp` | Robert Fripp |  | Curations › Sounds › Music › Artists |
| `robertplant` | Robert Plant |  | Curations › Sounds › Music › Artists |
| `snarkypuppy` | Snarky Puppy |  | Curations › Sounds › Music › Artists |
| `steelydan` | Steely Dan |  | Curations › Sounds › Music › Artists |
| `stevewilson` | Steve Wilson |  | Curations › Sounds › Music › Artists |
| `sturgilsimpson` | Sturgill Simpson |  | Curations › Sounds › Music › Artists |
| `sylvainrichards20syl` | Sylvain Richards (20SYL) |  | Curations › Sounds › Music › Artists |
| `theband` | The Band |  | Curations › Sounds › Music › Artists |
| `wolfmother` | Wolfmother |  | Curations › Sounds › Music › Artists |

### `tea/` — "Tea Journal" · 18 entries · today on: creative, tea

| Entry | Title | Today | → New home |
|---|---|---|---|
| `2009-12-05-assam-tgfop` | Assam T.G.F.O.P. |  | Curations › Tastes › Tea |
| `2009-12-05-ceylon-op` | Ceylon O.P. |  | Curations › Tastes › Tea |
| `2009-12-05-ceylon-silver-tips` | Ceylon Silver Tips |  | Curations › Tastes › Tea |
| `2009-12-05-english-breakfast-blend` | English Breakfast Blend |  | Curations › Tastes › Tea |
| `2010-02-20-arabian-tea` | Arabian Tea |  | Curations › Tastes › Tea |
| `2010-02-25-china-black-tea` | China Black Tea |  | Curations › Tastes › Tea |
| `2010-03-15-pettiagalla-op-ceylon` | Pettiagalla O.P. Ceylon |  | Curations › Tastes › Tea |
| `2010-06-06-blue-lady` | Blue Lady |  | Curations › Tastes › Tea |
| `2010-06-06-harmutty-tippy-golden-assam` | Harmutty Tippy Golden Assam |  | Curations › Tastes › Tea |
| `2010-06-06-passion-fruit-tea` | Passion Fruit Tea |  | Curations › Tastes › Tea |
| `2010-08-12-lingia-ftgfop1` | Lingia FTGFOP1 |  | Curations › Tastes › Tea |
| `2010-08-22-margrets-hope-ftgfop` | Margrets Hope FTGFOP |  | Curations › Tastes › Tea |
| `2010-09-14-whittards-125th-anniversary-blend` | Whittards 125th Anniversary Blend |  | Curations › Tastes › Tea |
| `2010-11-24-bannockburn-ftgfop1-darjeeling` | Bannockburn FTGFOP1 Darjeeling |  | Curations › Tastes › Tea |
| `2010-11-29-fortnum-mason-royal-blend` | Fortnum & Mason Royal Blend |  | Curations › Tastes › Tea |
| `2010-11-29-mohokutie-second-flush-assam` | Mohokutie Second Flush Assam |  | Curations › Tastes › Tea |
| `2010-11-29-whittards-darjeeling` | Whittard's Darjeeling |  | Curations › Tastes › Tea |
| `2010-11-29-whittards-mango-tea` | Whittard's Mango Tea |  | Curations › Tastes › Tea |

### `books/` — "Books" · 8 entries · today on: creative, blog

| Entry | Title | Today | → New home |
|---|---|---|---|
| `caravanofdreams` | Caravan of Dreams |  | Curations › Thoughts › Books |
| `dramaticuniversevol3` | The Dramatic Universe Vol. 3: Man and His Nature |  | Curations › Thoughts › Books |
| `dramaticuniversevol4` | The Dramatic Universe Vol. 4: History |  | Curations › Thoughts › Books |
| `dramaticuniversevolume1` | The Dramatic Universe Vol. 1: The Foundations of Natural Philosophy |  | Curations › Thoughts › Books |
| `dramaticuniversevolume2` | The Dramatic Universe Vol. 2: The Foundations of Moral Philosophy |  | Curations › Thoughts › Books |
| `energiesmaterialvitalcosmic` | Energies; material, vital, cosmic |  | Curations › Thoughts › Books |
| `musicthebrainandecstacy` | Music The Brain and Ecstacy |  | Curations › Thoughts › Books |
| `trueperception` | True Perception |  | Curations › Thoughts › Books |

### `travel/` — "Travel" · 19 entries · today on: creative, blog

| Entry | Title | Today | → New home |
|---|---|---|---|
| `Indiapt1` | India 1: Gorkana |  | Curations › Experiences › Travel |
| `austria` | Austria: Crypto Commons Gathering |  | Curations › Experiences › Travel |
| `cornwall-reflections` | Cornwall Reflections | hidden from grids | Curations › Experiences › Travel › `drafts/` |
| `croatia` | Croatia: Metafest |  | Curations › Experiences › Travel |
| `eden-project` | The Eden Project |  | Curations › Experiences › Travel |
| `heligan-gardens` | Heligan Gardens |  | Curations › Experiences › Travel |
| `indiapt2` | India 2: Bangalore |  | Curations › Experiences › Travel |
| `indiapt3` | India 3: Auroville Involoution |  | Curations › Experiences › Travel |
| `indiapt4` | India 4: Auroville Evolution |  | Curations › Experiences › Travel |
| `japan1` | Japan 1: Tokyo Energy |  | Curations › Experiences › Travel |
| `japan2` | Japan 2: Kyoto Tranquility |  | Curations › Experiences › Travel |
| `japan3` | Japan 3: Tokyo Return |  | Curations › Experiences › Travel |
| `kew-gardens` | Kew Gardens |  | Curations › Experiences › Travel |
| `metaphorum` | UK: Metaphorum Manchester |  | Curations › Experiences › Travel |
| `newquay-par` | Newquay Participatory Action Research | hidden from grids | Curations › Experiences › Travel › `drafts/` |
| `olbia` | Olbia: Metalinguistics |  | Curations › Experiences › Travel |
| `poland` | Poland: Chinwags Poznan |  | Curations › Experiences › Travel |
| `portugal` | Portugal: Boom Festival |  | Curations › Experiences › Travel |
| `switzerland` | Switzerland: Solstice Festival Geneva |  | Curations › Experiences › Travel |

### `study/` — "Writing" · 13 entries · today on: creative, blog

| Entry | Title | Today | → New home |
|---|---|---|---|
| `cats_theory_1` | Cats Theory Primer: Ground Zero |  | Considerations › Expositions |
| `cats_theory_2` | Cats Theory Primer: Reasoning and Revisions |  | Considerations › Expositions |
| `emergentcyclicaltheory` | Emergent Cyclical Theory |  | Considerations › Expositions |
| `explorations_in_cosmic_ecology` | Explorations in Cosmic Ecology | draft | Considerations › Expositions › `drafts/` |
| `interactive_vsm` | Methodology Development: Interactive VSM |  | Considerations › Practices |
| `monastic_design` | Methodology Development: Monastic Design |  | Considerations › Practices |
| `process_philosophy_1` | Process Thought and Science 2025 (Session 1) |  | Considerations › Retrospectives |
| `process_philosophy_2` | Process Thought and Science 2025 (Session 2) |  | Considerations › Retrospectives |
| `process_philosophy_3` | Process Thought and Science 2025 (Session 3) |  | Considerations › Retrospectives |
| `process_philosophy_4` | Process Thought and Science 2025 (Session 4) |  | Considerations › Retrospectives |
| `process_philosophy_5` | Process Thought and Science 2025 (Session 5) |  | Considerations › Retrospectives |
| `reflections_on_cosmic_ecology` | Reflections on Cosmic Ecology | draft | Considerations › Expositions › `drafts/` |
| `semiotics` | Semiotics in Brief |  | Considerations › Expositions |

### `software/` — "Technical" · 9 entries · today on: creative, portfolio

| Entry | Title | Today | → New home |
|---|---|---|---|
| `adaptivecapital` | Adaptive Capital |  | Creations › Software |
| `blockchainonboarding` | Blockchain Onboarding |  | Creations › Software |
| `hackalong` | Hackalong | draft | Creations › Software › `drafts/` |
| `hermitage` | Hermitage |  | Creations › Software |
| `holons` | Holons |  | Creations › Software |
| `interspace` | Interspace |  | Creations › Software |
| `notabot` | Not-a-Bot |  | Creations › Software |
| `saturator` | Saturator |  | Creations › Software |
| `systematics-interface` | Systematics Interface |  | Creations › Software |

### `recordings/` — "Recordings" · 15 entries · today on: creative, portfolio

| Entry | Title | Today | → New home |
|---|---|---|---|
| `beatbehind` | Beat Behind |  | Creations › Recordings |
| `brentwoodroyalyouthorchestra` | Brentwood Royal Youth Legion Orchestra |  | Creations › Recordings |
| `greysky` | Grey Sky |  | Creations › Recordings |
| `harmonicsplinters` | Harmonic Splinters |  | Creations › Recordings |
| `irishchamberorchestra` | Irish Chamber Orchestra |  | Creations › Recordings |
| `mountains` | Mountains |  | Creations › Recordings |
| `neonfleacircus` | Fistfull of I.O.U's |  | Creations › Recordings |
| `petemolinari` | Pete Moulinari |  | Creations › Recordings |
| `primates` | Primates |  | Creations › Recordings |
| `projectplay` | Project Play |  | Creations › Recordings |
| `robots` | Robots |  | Creations › Recordings |
| `soascubanbigbands` | SOAS Cuban Big Band |  | Creations › Recordings |
| `sonsofgingerbread` | Sons of Gingerbread |  | Creations › Recordings |
| `thamesvallyphilharmonia` | Isobella Peck Cubann II Thames Valley Philharmonic |  | Creations › Recordings |
| `urbanbigbandandgamelanorchestra` | Urban Big Band and Gamelan Orchestra Recording | draft | Creations › Recordings › `drafts/` |

### `multimedia/` — "Multimedia" · 6 entries · today on: creative, portfolio

| Entry | Title | Today | → New home |
|---|---|---|---|
| `calma` | Calma Video |  | Creations › Media |
| `cursed` | Cursed Animation |  | Creations › Media |
| `elephantsdream` | Elephants Dream Animation |  | Creations › Media |
| `encaged` | Encaged |  | Creations › Media |
| `lespaul` | Les Paul |  | Creations › Media |
| `wasser` | Wasser |  | Creations › Media |

### `tech-management/` — "Tech Management" · 8 entries · today on: creative, portfolio

| Entry | Title | Today | → New home |
|---|---|---|---|
| `borrisbrejeca` | Boris Brejcha (lights) |  | Creations › Ventures |
| `eggldn` | Egg London - Technical Management |  | Creations › Ventures |
| `equinoxunconf` | Equinox Unconf |  | Creations › Ventures |
| `ericmorillo` | Eric Morillo (lights) |  | Creations › Ventures |
| `mber` | Mber |  | Creations › Ventures |
| `oriole` | Oriole |  | Creations › Ventures |
| `sbg` | SBG - Technical Management |  | Creations › Ventures |
| `swift` | Swift |  | Creations › Ventures |

### `timeline/` — "Career" · 5 entries · today on: creative, portfolio

| Entry | Title | Today | → New home |
|---|---|---|---|
| `commonsstack` | Commons Stack | draft | drafts/ (site-level) |
| `giveth` | Giveth | draft | drafts/ (site-level) |
| `liminalvillage` | Liminal Village | draft | drafts/ (site-level) |
| `pillar` | Pillar Project | draft | drafts/ (site-level) |
| `regenfoundation` | Regen Foundation | draft | drafts/ (site-level) |

### Aesthetics Board — 130 images · today: private artifact only

All 130 → Curations › Sights › Art (see §5).
