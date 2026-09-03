#!/usr/bin/env python3
"""Regenerate README.md's progress section from the NeetCode 250 catalog
and whichever problem folders currently exist under "Data Structures & Algorithms".

A problem counts as solved if its folder contains at least one submission-*.<ext> file.
"""
import json
import os
import re
from collections import OrderedDict, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CATALOG_PATH = os.path.join(ROOT, "scripts", "neetcode250.json")
SUBMISSIONS_DIR = os.path.join(ROOT, "Data Structures & Algorithms")
README_PATH = os.path.join(ROOT, "README.md")

START_MARKER = "<!-- PROGRESS:START -->"
END_MARKER = "<!-- PROGRESS:END -->"

PATTERN_ORDER = [
    "Arrays & Hashing", "Two Pointers", "Sliding Window", "Stack",
    "Binary Search", "Linked List", "Trees", "Tries",
    "Heap / Priority Queue", "Backtracking", "Graphs", "Advanced Graphs",
    "1-D DP", "2-D DP", "Greedy", "Intervals", "Math & Geometry",
    "Bit Manipulation",
]


def load_catalog():
    with open(CATALOG_PATH) as f:
        return json.load(f)


DIFFICULTY_COLORS = OrderedDict(
    [
        ("Easy", "22c55e"),
        ("Medium", "eab308"),
        ("Hard", "ef4444"),
    ]
)

LANGUAGE_NAMES = {
    "py": "Python",
    "js": "JavaScript",
    "ts": "TypeScript",
    "java": "Java",
    "cpp": "C++",
    "cs": "C#",
    "go": "Go",
    "rs": "Rust",
    "kt": "Kotlin",
    "swift": "Swift",
    "sql": "SQL",
}


def language_counts():
    counts = defaultdict(int)
    if not os.path.isdir(SUBMISSIONS_DIR):
        return counts
    for entry in os.scandir(SUBMISSIONS_DIR):
        if not entry.is_dir():
            continue
        for fn in os.listdir(entry.path):
            if not fn.startswith("submission-"):
                continue
            ext = fn.rsplit(".", 1)[-1].lower() if "." in fn else ""
            counts[ext] += 1
    return counts


def solved_slugs():
    solved = set()
    if not os.path.isdir(SUBMISSIONS_DIR):
        return solved
    for entry in os.scandir(SUBMISSIONS_DIR):
        if not entry.is_dir():
            continue
        has_submission = any(
            fn.startswith("submission-") for fn in os.listdir(entry.path)
        )
        if has_submission:
            solved.add(entry.name)
    return solved


def build_section():
    catalog = load_catalog()
    solved = solved_slugs()
    catalog_slugs = {item["slug"] for item in catalog}

    by_pattern = OrderedDict((p, []) for p in PATTERN_ORDER)
    for item in catalog:
        by_pattern.setdefault(item["pattern"], []).append(item)

    total = len(catalog)
    total_solved = sum(1 for item in catalog if item["slug"] in solved)
    pct = (total_solved / total * 100) if total else 0

    lines = []
    lines.append(START_MARKER)
    lines.append("")

    lang_counts = language_counts()
    total_files = sum(lang_counts.values())
    if total_files:
        lines.append("### Language used")
        lines.append("")
        lines.append("| Language | Files | Percentage |")
        lines.append("|---|---|---|")
        for ext, count in sorted(lang_counts.items(), key=lambda kv: -kv[1]):
            name = LANGUAGE_NAMES.get(ext, ext)
            pct_lang = count / total_files * 100
            lines.append(f"| {name} | {count} | {pct_lang:.1f}% |")
        lines.append("")

    lines.append("## Progress tracker")
    lines.append("")
    lines.append(f"**{total_solved} / {total} solved ({pct:.1f}%)**")
    lines.append("")
    lines.append(
        f"![Progress](https://progress-bar.xyz/{total_solved}/?scale={total}&suffix=%20/%20{total}&width=300&progress_color=ffffff)"
    )
    lines.append("")
    for difficulty, color in DIFFICULTY_COLORS.items():
        items = [item for item in catalog if item["difficulty"] == difficulty]
        d_total = len(items)
        d_solved = sum(1 for item in items if item["slug"] in solved)
        d_pct = round(d_solved / d_total * 100) if d_total else 0
        suffix = f"%25%20({d_solved}/{d_total})"
        lines.append(
            f"**{difficulty}**  \n"
            f"![{difficulty}](https://progress-bar.xyz/{d_pct}/?scale=100&suffix={suffix}&width=300&progress_color={color})"
        )
        lines.append("")
    lines.append("| Pattern | Solved | Total | Percentage |")
    lines.append("|---|---|---|---|")
    for pattern in by_pattern:
        items = by_pattern[pattern]
        if not items:
            continue
        s = sum(1 for it in items if it["slug"] in solved)
        t = len(items)
        p = (s / t * 100) if t else 0
        lines.append(f"| {pattern} | {s} | {t} | {p:.1f}% |")
    lines.append("")

    lines.append("---")
    lines.append("")
    lines.append("## Solutions by pattern")
    lines.append("")
    for pattern in by_pattern:
        items = by_pattern[pattern]
        if not items:
            continue
        s = sum(1 for it in items if it["slug"] in solved)
        t = len(items)
        lines.append(f"<details>")
        lines.append(f"<summary><strong>{pattern}</strong> ({s}/{t})</summary>")
        lines.append("")
        for it in items:
            box = "x" if it["slug"] in solved else " "
            if it["slug"] in solved:
                link = f'[{it["name"]}](<Data Structures & Algorithms/{it["slug"]}>)'
            else:
                link = it["name"]
            lines.append(f"- [{box}] {link}")
        lines.append("")
        lines.append("</details>")
        lines.append("")

    unmapped = sorted(solved - catalog_slugs)
    if unmapped:
        lines.append("<details>")
        lines.append("<summary><strong>Other / unmapped</strong> (not in the NeetCode 250 catalog)</summary>")
        lines.append("")
        for slug in unmapped:
            lines.append(f'- [{slug}](<Data Structures & Algorithms/{slug}>)')
        lines.append("")
        lines.append("</details>")
        lines.append("")

    lines.append(END_MARKER)
    return "\n".join(lines)


def update_readme():
    with open(README_PATH) as f:
        content = f.read()

    section = build_section()

    if START_MARKER in content and END_MARKER in content:
        pattern = re.compile(
            re.escape(START_MARKER) + r".*?" + re.escape(END_MARKER), re.DOTALL
        )
        new_content = pattern.sub(section, content)
    else:
        new_content = content.rstrip() + "\n\n---\n\n" + section + "\n"

    if new_content != content:
        with open(README_PATH, "w") as f:
            f.write(new_content)
        print("README.md updated.")
    else:
        print("README.md already up to date.")


if __name__ == "__main__":
    update_readme()
