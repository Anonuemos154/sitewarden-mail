#!/usr/bin/env python3
"""Fail CI when personal mailbox addresses are committed.

This deliberately blocks consumer Gmail/Googlemail addresses without storing any private address
in the repository. Role-based project addresses on project-owned domains remain allowed.
"""
from __future__ import annotations

import pathlib
import re
import subprocess
import sys

PERSONAL_PROVIDER_EMAIL = re.compile(
    rb"(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@(gmail|googlemail)\.com\b"
)

TEXT_EXTENSIONS = {
    ".md", ".txt", ".toml", ".json", ".yaml", ".yml", ".js", ".jsx", ".ts", ".tsx",
    ".rs", ".py", ".sh", ".ps1", ".html", ".css", ".xml", ".svg", ".env", ".example"
}

def tracked_files() -> list[pathlib.Path]:
    raw = subprocess.check_output(["git", "ls-files", "-z"])
    return [pathlib.Path(p.decode("utf-8")) for p in raw.split(b"\0") if p]

def is_candidate(path: pathlib.Path) -> bool:
    if path.name.startswith(".env"):
        return True
    return path.suffix.lower() in TEXT_EXTENSIONS or path.name in {
        "LICENSE", "README", "CODEOWNERS"
    }

def main() -> int:
    violations: list[str] = []
    for path in tracked_files():
        if not path.is_file() or not is_candidate(path):
            continue
        try:
            data = path.read_bytes()
        except OSError:
            continue
        if PERSONAL_PROVIDER_EMAIL.search(data):
            violations.append(str(path))

    if violations:
        print("Privacy guard failed: personal consumer-mail address pattern found in:")
        for path in violations:
            print(f" - {path}")
        print("Use a role-based project address or remove the address entirely.")
        return 1

    print("Privacy guard passed: no personal Gmail/Googlemail addresses found in tracked text files.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
