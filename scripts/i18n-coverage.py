#!/usr/bin/env python3

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LANGUAGES = ROOT / "crates/i18n/src/language.rs"
LOCALES = ROOT / "assets/i18n"
README = ROOT / "README.md"

START = "<!-- i18n:start -->"
END = "<!-- i18n:end -->"

SOURCE = "en-US"
LOCALE = re.compile(r'^\s*\(\w+, "([^"]+)", "([^"]+)"\),$', re.M)


def shipped():
    source = LANGUAGES.read_text(encoding="utf-8")
    locales = LOCALE.findall(source)
    if not locales:
        sys.exit(f"cannot find the shipped locales in {LANGUAGES}")
    return locales


def keys(locale):
    paths = sorted((LOCALES / locale).glob("*.ftl"))
    if not paths:
        sys.exit(f"cannot find Fluent resources for {locale}")

    found = set()
    for path in paths:
        for line in path.read_text(encoding="utf-8").splitlines():
            if not line or line[0] in " #.*[" or " =" not in line:
                continue
            key = line.split(" =", 1)[0]
            if key and not key.startswith("-"):
                found.add(key)
    return found


def table():
    source = keys(SOURCE)
    total = len(source)
    rows = ["| Language | Translated | Coverage |", "| --- | --- | --- |"]

    for locale, label in shipped():
        done = len(source & keys(locale))
        share = round(done * 100 / total) if total else 0
        rows.append(f"| {label} (`{locale}`) | {done}/{total} | {share}% |")

    return "\n".join(rows)


def main():
    readme = README.read_text(encoding="utf-8")
    if START not in readme or END not in readme:
        sys.exit(f"{README} has no {START} … {END} block")

    head, rest = readme.split(START, 1)
    _, tail = rest.split(END, 1)
    README.write_text(f"{head}{START}\n\n{table()}\n\n{END}{tail}", encoding="utf-8")
    print(table())


if __name__ == "__main__":
    main()
