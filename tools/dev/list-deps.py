#!/usr/bin/env python3
import os
from typing import Iterable, Optional, Dict, Union, Any
import tomllib
from dataclasses import dataclass
from collections import defaultdict
import sys
import re
import subprocess


@dataclass
class Crate(object):
    crate: str
    name: str
    ext_deps: Dict[str, str]


def iter_crates() -> Iterable[str]:
    def inner(root: Optional[str] = None) -> Iterable[str]:
        prefix = root or "."
        for f in os.listdir(prefix):
            if not os.path.isdir(os.path.join(prefix, f)):
                continue
            if os.path.exists(os.path.join(prefix, f, "Cargo.toml")):
                yield f"{root}/{f}" if root else f

    yield from inner()
    yield from inner("collectors")
    yield from inner("proto")


def read_toml(crate: str) -> Crate:
    def get_dep_version(data: Union[str, Dict[str, Any]]) -> Optional[str]:
        if isinstance(data, str):
            return data
        if isinstance(data, dict):
            return data.get("version")
        return None

    with open(os.path.join(crate, "Cargo.toml"), "rb") as f:
        data = tomllib.load(f)
    return Crate(
        crate=crate,
        name=data["package"]["name"],
        ext_deps={
            k: get_dep_version(data["dependencies"][k]) for k in data.get("dependencies",[])
        },
    )


def check_deps(crates: Iterable[Crate]) -> bool:
    print("# Checking dependencies")
    versions = defaultdict(list)
    for crate in crates:
        for dep, version in crate.ext_deps.items():
            if version is not None:
                versions[dep].append((crate.crate, version))
    status = True
    for dep, rel in versions.items():
        dv = defaultdict(list)
        for c, v in rel:
            dv[v].append(c)
        if len(dv) == 1:
            continue
        if status:
            print("!!! Mismatched versions for dependencies:")
            status = False
        print(f"{dep}:")
        for v in sorted(dv):
            print(f"{v:10s}: {', '.join(sorted(dv[v]))}")
    return status


def main() -> int:
    # Read dependencies
    versions = defaultdict(list)
    for crate_name in iter_crates():
        crate = read_toml(crate_name)
        for dep, version in crate.ext_deps.items():
            if version is not None:
                versions[dep].append((crate.crate, version))
    # Print dependencies
    status = True
    for name in sorted(versions):
        vlist = defaultdict(list)
        for crate, version in versions[name]:
            vlist[version].append(crate)
        if len(vlist) == 1:
            v = list(vlist)[0]
            items = "\n    ".join(sorted(vlist[v]))
            print(f"* {name} v{v}:\n    {items}")
        else:
            print(f"* !!! {name}")
            for v in sorted(vlist):
                items = ", ".join(sorted(vlist[v]))
                print(f"  {v}: {items}")
                status = False
    if not status:
        print("!!! Versions mismatch")
    return 0 if status else 1


if __name__ == "__main__":
    sys.exit(main())
