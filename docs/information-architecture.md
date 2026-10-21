# Information Architecture — Curations · Considerations · Creations

Status: **ratified**.
Companions: [`content-inventory.md`](content-inventory.md) (what exists and where it goes) and [`build-spec.md`](build-spec.md) (how it is built).
Design: system https://claude.ai/artifact/6nFYsUWpdXBHbqCdVQjCAa · page mockups https://claude.ai/artifact/APpjD6pBB3iPqxNotukugo

## 1. Purpose and semantics

The site is **singular and collective**:

- **Singular:** one codebase, one content pool, one monolith served from `joshafairhead.com`, viewed through a three-level filter, **triad → category → instance**.
- **Collective:** the monolith is sectioned carefully, so any node (e.g. Sights, Sounds, Tastes) can be **broken out** and deployed on its own subdomain as a standalone site, without forking content or code. The triad follows a receive → reflect → make progression.

| Triad | Meaning | Placement test |
|---|---|---|
| **Curations** | the aesthetic world taken in | Did I *receive* it? |
| **Considerations** | what is thought through | Did I *think it through*? |
| **Creations** | what is made | Did I *make* it? |

## 2. Tree

```
joshafairhead.com             business card: name, contact, 3 triad links
│   └─ /for/<org>             recipient cards (tagline + tailored links)
├─ Curations
│   ├─ Sights       › Art
│   ├─ Sounds       › Music          facets: Artists · Albums · Tracks
│   ├─ Tastes       › Tea
│   ├─ Thoughts     › Books          (more non-book thoughts to come)
│   └─ Experiences  › Travel
├─ Considerations
│   ├─ Expositions                   own explanatory writing and research
│   ├─ Retrospectives                write-ups of courses taken
│   └─ Practices                     methods developed
└─ Creations
    ├─ Software
    ├─ Recordings
    ├─ Media                         scores, sound design, film, craft
    └─ Ventures                      venues, events, organising
```

The instance level is optional per category, and it is how a category grows. For example, Thoughts › Books will gain siblings that are not books. Categories without instances are terminal.

**Facets** are filters *within* a node. They never add a navigation level. Examples: Music's Artists / Albums / Tracks, Art's groups and artists, and tags within any node.

## 3. Filter / navigation model

1. **Root (business card):** name, contact email, and links to the three triads. No content listing.
2. **Selecting a triad** shows every entry in that triad as one mixed grid, newest first. The menu offers that triad's categories as pills (led by `All`) that filter the grid in place, updating the address to the category.
3. **Selecting a category** narrows to that category's entries. The menu offers its instances, if any.
4. **Selecting an instance** narrows further. This is the terminal level; facets may filter within it.
5. Every selection does three things together: (a) filters the content, (b) changes the address, (c) swaps the menu to the next level's options.
6. A breadcrumb (e.g. Curations › Sights › Art) shows the path, and each crumb steps back up. It is the site header: there is no separate header bar.
7. There is no sideways movement between triads except through the breadcrumb or root.
8. Nodes with no published entries are hidden from menus.

## 4. Drafts

- Any node may contain a `drafts/` folder. Content that does not belong to any node yet goes in a site-level `drafts/` folder.
- Drafts are never published. They can be previewed locally only.
- Draft status comes from location alone. There is no draft flag and no "hidden but published" state.

## 5. Addresses: monolith paths and breakouts

**In the monolith,** every node has a path under the root:

| Level | Monolith path |
|---|---|
| Root | `joshafairhead.com/` (+ `/for/<org>`) |
| Triad | `/curations/` · `/considerations/` · `/creations/` |
| Category | e.g. `/curations/sights/` |
| Instance | e.g. `/curations/sights/art/` |

**Breakouts.** Any triad or category can be flagged for breakout. A breakout is the same subtree, deployed as its own site on its own subdomain:

- The node becomes that site's root: `sights.joshafairhead.com/art/`.
- Its breadcrumb starts at the node, with one link back up to the monolith.
- Which nodes are broken out is configuration, not structure. Adding or removing a breakout changes nothing in the content.

| Candidate breakout hosts | |
|---|---|
| Triads | `curations.` · `considerations.` · `creations.` |
| Categories | `sights.` `sounds.` `tastes.` `thoughts.` `experiences.` · `expositions.` `retrospectives.` `practices.` · `software.` `recordings.` `media.` `ventures.` |

The first breakouts are Sights, Sounds and Tastes; the rest are enabled as wanted.

**Existing hosts** (`creative.`, `blog.`, `portfolio.`, `tea.`) are **left untouched**. They keep serving their current builds until a separate decision is made. `cv.` and `facilitation.` are also outside this site for now.

## 6. Constraints on the build

- Each breakout must render correctly as a standalone site, with no links into unpublished parts of the tree.
- Moving between subdomains is a full page load. The "filtering one monolith" feel (shared shell, consistent menu, fast transition) is a build requirement.
- Each entry has exactly one home node.
