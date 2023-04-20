#!/usr/bin/env python3

# Python modules
import sys
import os
import subprocess

COLLECTORS_ROOT = "collectors"
TEMPLATE = "_template"


def main(name: str) -> None:
    root = os.path.join(COLLECTORS_ROOT, name)
    if os.path.exists(root):
        print(f"{name} is already exists")
        sys.exit(2)
    for prefix, _, files in os.walk(os.path.join(COLLECTORS_ROOT, TEMPLATE)):
        parts = prefix.split(os.sep)[2:]
        rel = os.path.join(*parts) if parts else ""
        for fn in files:
            src = os.path.join(prefix, fn)
            dn = os.path.join(COLLECTORS_ROOT, name, rel)
            dst = os.path.join(dn, fn)
            print(f"{src} -> {dst}")
            with open(src) as f:
                data = f.read()
            os.makedirs(dn, exist_ok=True)
            with open(dst, "w") as f:
                f.write(data.replace(TEMPLATE, name))
    configure()


def configure():
    dn = os.path.dirname(sys.argv[0])
    cfg_path = os.path.join(dn, "configure.py")
    subprocess.check_call(cfg_path)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage:")
        print(f"  {sys.argv[0]} <name>")
        sys.exit(1)
    main(sys.argv[1])
