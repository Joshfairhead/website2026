#!/usr/bin/env python3
"""One-off migration from the Zola prototype into this repo's content tree.

Follows docs/content-inventory.md section 6. Run once from the repo root:

    python3 tools/migrate.py --creative ../creative --hub ../hub \
        --art <folder holding items.json and web/> --out content

It moves entries into their new nodes, sets drafts by location, converts front
matter to the strict schema, turns music album galleries and "Special
Mentions" into albums.toml / tracks.toml, imports the Aesthetics Board as
art/items.toml, imports the hub's recipient cards, and downscales images over
1 MB to 2400 px. Kept for the record; not part of the build.
"""

import argparse
import json
import re
import shutil
import subprocess
import sys
import tomllib
from datetime import date, datetime
from pathlib import Path

SECTION_TO_NODE = {
    "music": "curations/sounds/music",
    "tea": "curations/tastes/tea",
    "books": "curations/thoughts/books",
    "travel": "curations/experiences/travel",
    "software": "creations/software",
    "recordings": "creations/recordings",
    "multimedia": "creations/media",
    "tech-management": "creations/ventures",
    "timeline": "drafts",
}
STUDY = {
    "process_philosophy": "considerations/retrospectives",
    "interactive_vsm": "considerations/practices",
    "monastic_design": "considerations/practices",
}
FORCE_DRAFT = {
    "travel/cornwall-reflections",
    "travel/newquay-par",
    "study/explorations_in_cosmic_ecology",
    "study/reflections_on_cosmic_ecology",
}
# Tags that only repeated the old section; the node now says it.
SECTION_TAGS = {"Music", "Artist", "Travel", "Tea", "Recordings", "Study",
                "Software", "Multimedia", "CV", "Books"}
IMAGE_EXT = {".jpg", ".jpeg", ".png", ".webp"}
# Broken references in the old site, fixed on the way in.
TEXT_FIXES = {
    "travel/austria": [('url="Culto.jpeg"', 'url="culto.jpeg"')],  # file is lower-case
    "travel/indiapt4": [('{{image(url="12perseverence.jpeg", alt="Perseverence", no_hover=true) }}\n', "")],  # never existed
    "software/hermitage": [("/images/hermitage/", "")],  # screenshots move into the entry
}
SHARED_ASSETS = {"software/hermitage": "static/images/hermitage"}
MAX_BYTES = 1_000_000


def node_for(section, slug):
    if section == "study":
        for prefix, node in STUDY.items():
            if slug.startswith(prefix):
                return node
        return "considerations/expositions"
    return SECTION_TO_NODE[section]


def split_front_matter(text, path):
    m = re.match(r"\+\+\+\n(.*?)\n\+\+\+\n?(.*)", text, re.S)
    if not m:
        sys.exit(f"{path}: no +++ front matter")
    return tomllib.loads(m.group(1)), m.group(2)


def toml_str(s):
    return json.dumps(s, ensure_ascii=False)


def toml_date(v):
    if isinstance(v, datetime):
        return v.date().isoformat()
    if isinstance(v, date):
        return v.isoformat()
    return str(v)[:10]


def front_matter(fm):
    extra = fm.get("extra", {})
    lines = [f"title = {toml_str(fm['title'])}",
             f"date = {toml_date(fm['date'])}",
             f"description = {toml_str(fm['description'].strip())}"]
    tags = [t for t in fm.get("taxonomies", {}).get("tags", []) if t not in SECTION_TAGS]
    if tags:
        lines.append("tags = [" + ", ".join(toml_str(t) for t in tags) + "]")
    card = extra.get("card") or extra.get("banner")
    if card:
        lines.append(f"card = {toml_str(card)}")
    if extra.get("banner") and extra.get("hero", True) is not False:
        lines.append(f"banner = {toml_str(extra['banner'])}")
    if extra.get("toc"):
        lines.append("toc = true")
    return "+++\n" + "\n".join(lines) + "\n+++\n"


def remove_block(html, start):
    """Remove the <div> opening at `start` with its balanced content."""
    depth, i = 0, start
    for m in re.finditer(r"<div\b|</div>", html[start:]):
        depth += 1 if m.group(0) == "<div" else -1
        if depth == 0:
            i = start + m.end()
            break
    return html[:start] + html[i:].lstrip("\n")


def extract_albums(body, slug, artist):
    albums = []
    for item in re.findall(r'<div class="album-item">(.*?)</div>\s*(?:</a>\s*)?</div>', body, re.S):
        link = re.search(r'<a href="([^"]+)"', item)
        img = re.search(r'<img src="([^"]+)"', item)
        title = re.search(r'album-title">([^<]*)<', item)
        albums.append({
            "title": title.group(1).strip() if title else (img and re.search(r'alt="([^"]*)"', item).group(1)),
            "artist": artist,
            "link": link.group(1).replace("&amp;", "&") if link else None,
            "cover": f"{slug}/{img.group(1)}" if img else None,
            "artist_page": slug,
        })
    while (i := body.find('<div class="album-gallery">')) != -1:
        body = remove_block(body, i)
    return albums, body


def special_mentions(body):
    albums, tracks, section = [], [], None
    for line in body.splitlines():
        if line.startswith("## "):
            section = line[3:].strip().lower()
        m = re.match(r"- \*\*(.+?)\*\*: \[(.+?)\]\((.+?)\)", line)
        if not m:
            continue
        link = m.group(3).replace("https:/https://", "https://")
        rec = {"title": m.group(2), "artist": m.group(1), "link": link}
        (albums if section == "albums" else tracks).append(rec)
    return albums, tracks


def write_records(path, table, records):
    out = []
    for r in records:
        out.append(f"[[{table}]]")
        for k, v in r.items():
            if v is None:
                continue
            out.append(f"{k} = {v if isinstance(v, int) else toml_str(v)}")
        out.append("")
    path.write_text("\n".join(out))


def copy_assets(src, dest):
    for f in src.rglob("*"):
        if f.is_dir() or f.name == "index.md" or f.name == ".DS_Store":
            continue
        target = dest / f.relative_to(src)
        target.parent.mkdir(parents=True, exist_ok=True)
        if f.suffix.lower() in IMAGE_EXT and f.stat().st_size > MAX_BYTES:
            subprocess.run(["convert", str(f), "-auto-orient", "-resize", "2400x2400>",
                            "-quality", "85", str(target)], check=True)
        else:
            shutil.copy2(f, target)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--creative", type=Path, required=True)
    ap.add_argument("--hub", type=Path, required=True)
    ap.add_argument("--art", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    args = ap.parse_args()
    src, out = args.creative / "content", args.out
    if out.exists():
        sys.exit(f"{out} already exists; refusing to overwrite")

    # First pass: where every entry goes, so internal links can be rewritten.
    plan = {}
    for index in sorted(src.glob("*/*/index.md")):
        section, slug = index.parent.parent.name, index.parent.name
        fm, _ = split_front_matter(index.read_text(), index)
        if section == "music" and slug == "all":
            plan[(section, slug)] = None
            continue
        node = node_for(section, slug)
        draft = node == "drafts" or fm.get("draft") is True or f"{section}/{slug}" in FORCE_DRAFT
        rel = Path(node) / ("drafts" if draft and node != "drafts" else "") / slug
        plan[(section, slug)] = (rel, draft)

    def rewrite_links(body, own_draft):
        def at_link(m):
            target = plan.get(tuple(m.group(1).split("/")))
            if target is None:
                sys.exit(f"unknown internal link {m.group(0)}")
            rel, draft = target
            if draft and not own_draft:
                return None
            return f"@/{rel.as_posix()}/"
        # Markdown links: drop the link (keep the text) when it points at a draft.
        def md(m):
            dest = at_link(re.match(r"@/([^/]+/[^/]+)/index\.md", m.group(2)))
            return m.group(1) if dest is None else f"[{m.group(1)}]({dest})"
        body = re.sub(r"\[([^\]]+)\]\((@/[^)]+)\)", md, body)
        body = body.replace("](/semioticsinbrief)", "](@/considerations/expositions/semiotics/)")
        return body

    albums, tracks = [], []
    for (section, slug), target in plan.items():
        index = src / section / slug / "index.md"
        fm, body = split_front_matter(index.read_text(), index)
        if target is None:
            a, t = special_mentions(body)
            albums += a
            tracks += t
            continue
        rel, draft = target
        if section == "music":
            a, body = extract_albums(body, slug, fm["title"])
            if not draft:
                albums = a + albums
        body = rewrite_links(body, draft)
        for old, new in TEXT_FIXES.get(f"{section}/{slug}", []):
            if old not in body:
                sys.exit(f"{section}/{slug}: fix target {old!r} not found")
            body = body.replace(old, new)
        if fm.get("extra", {}).get("card") == "woman.jpg":  # ironbutterfly: only Ball.jpg exists
            fm["extra"]["card"] = "Ball.jpg"
        dest = out / rel
        dest.mkdir(parents=True)
        (dest / "index.md").write_text(front_matter(fm) + "\n" + body.strip() + "\n")
        copy_assets(index.parent, dest)
        if f"{section}/{slug}" in SHARED_ASSETS:
            copy_assets(args.creative / SHARED_ASSETS[f"{section}/{slug}"], dest)

    music = out / "curations/sounds/music"
    write_records(music / "albums.toml", "album", albums)
    write_records(music / "tracks.toml", "track", tracks)

    # Aesthetics Board -> Art.
    art = out / "curations/sights/art"
    (art / "images").mkdir(parents=True)
    items = json.loads((args.art / "items.json").read_text())
    for it in items:
        name = Path(it["file"]).name
        shutil.copy2(args.art / it["file"], art / "images" / name)
    write_records(art / "items.toml", "item", [
        {"id": it["id"], "group": it["section"], "artist": it["artist"], "title": it["title"],
         "source": it.get("source") or None, "width": it["w"], "height": it["h"],
         "file": f"images/{Path(it['file']).name}"} for it in items])

    # Hub recipient cards.
    (out / "for").mkdir(parents=True)
    for card in sorted((args.hub / "content/for").glob("*.md")):
        if card.name == "_index.md":
            continue
        fm, _ = split_front_matter(card.read_text(), card)
        extra = fm.get("extra", {})
        lines = [f"title = {toml_str(fm['title'])}"]
        if extra.get("tagline"):
            lines.append(f"tagline = {toml_str(extra['tagline'])}")
        for link in extra.get("links", []):
            lines += ["", "[[links]]", f"label = {toml_str(link['label'])}", f"url = {toml_str(link['url'])}"]
        (out / "for" / card.name).write_text("+++\n" + "\n".join(lines) + "\n+++\n")

    print(f"migrated {sum(1 for t in plan.values() if t)} entries, {len(albums)} albums, "
          f"{len(tracks)} tracks, {len(items)} art items")


if __name__ == "__main__":
    main()
