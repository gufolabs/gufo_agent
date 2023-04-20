#!/usr/bin/env python3
import os
from typing import Iterable, Optional, Dict, Union, Any
import tomllib
from dataclasses import dataclass
from collections import defaultdict
import sys
import re
import subprocess

COLLECTORS_ROOT = "collectors"
TEMPLATE_COLLECTOR = "collectors/_template"

rx_tpl = re.compile(
    r"^(\s*)// @@@\{\{\{(.+?)^\s*// @@@\}\}\}", re.DOTALL | re.MULTILINE
)
rx_tpl_line = re.compile(r"^\s*// \| (.+)$")
rx_members = re.compile(r"members\s*=\s*\[([^\]]+)\]", re.DOTALL | re.MULTILINE)
rx_dep_name = re.compile(r"^(\S+)\s*=\s*(.+)$")
rx_dep_path = re.compile(r"\{.*path\s*=\s*\"([^\"]+)\"")


def is_collector(name: str) -> bool:
    return name.startswith("collectors/") and name != TEMPLATE_COLLECTOR


@dataclass
class Crate(object):
    crate: str
    name: str
    ext_deps: Dict[str, str]

    @property
    def is_collector(self) -> bool:
        return is_collector(self.crate)


def iter_crates() -> Iterable[str]:
    def inner(root: Optional[str] = None) -> Iterable[str]:
        prefix = root or "."
        for f in os.listdir(prefix):
            if not os.path.isdir(os.path.join(prefix, f)):
                continue
            if os.path.exists(os.path.join(prefix, f, "Cargo.toml")):
                yield f"{root}/{f}" if root else f

    yield from inner()
    yield from inner(COLLECTORS_ROOT)


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
            k: get_dep_version(data["dependencies"][k]) for k in data["dependencies"]
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


def expand_registry(crates: Iterable[Crate]) -> None:
    def apply_template(s: str, indent: int) -> str:
        start = " " * indent
        tpl = []
        out = [start + "// @@@{{{"]
        for line in s.splitlines():
            match = rx_tpl_line.search(line)
            if match:
                tpl.append(match.group(1))
                out.append(line)
        # Expand
        for c in collectors:
            for line in tpl:
                out.append(
                    start
                    + line.replace("{name}", c["name"]).replace("{ename}", c["ename"])
                )
        out += [start + "// @@@}}}"]
        return "\n".join(out)

    def expand(m) -> str:
        return apply_template(m.group(2), len(m.group(1)))

    def to_ename(s: str) -> str:
        return "".join(c.capitalize() for c in s.split("_"))

    print("# Expanding registry")
    path = os.path.join("agent", "src", "registry.rs")
    with open(path) as f:
        data = f.read()
    collectors = list(
        sorted(
            (
                {"name": c.name, "ename": to_ename(c.name)}
                for c in crates
                if c.is_collector
            ),
            key=lambda x: x["name"],
        )
    )
    n = rx_tpl.sub(expand, data)
    with open(path, "w") as f:
        f.write(n)
    # Format
    subprocess.call(["cargo", "fmt", "-p", "agent"])


def expand_workspace(crates: Iterable[Crate]) -> None:
    print("# Checking workspace")
    with open("Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    members = data["workspace"]["members"]
    collector_members = set(c for c in members if is_collector(c))
    collector_crates = set(c.crate for c in crates if c.is_collector)
    other_crates = set(c for c in members if c not in collector_members)
    # if collector_members == collector_crates:
    #    return
    all_members = list(sorted(other_crates)) + list(sorted(collector_crates))
    mlst = ", ".join(f'"{m}"' for m in all_members)
    repl = f"members = [{mlst}]"
    with open("Cargo.toml") as f:
        raw = f.read()
    r = rx_members.sub(repl, raw)
    with open("Cargo.toml", "w") as f:
        f.write(r)


def expand_agent_cargo(crates: Iterable[Crate]) -> None:
    HEAD = 0
    DEPS = 1
    TAIL = 2
    state = HEAD
    cargo_toml = os.path.join("agent", "Cargo.toml")
    deps = {}
    head = []
    tail = []
    with open(cargo_toml) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            if state == HEAD:
                if line == "[dependencies]":
                    state = DEPS
                    continue
                head.append(line)
            elif state == DEPS:
                if line.startswith("["):
                    state = TAIL
                    tail.append(line)
                    continue
                match = rx_dep_name.match(line)
                if match:
                    name = match.group(1)
                    value = match.group(2)
                    match = rx_dep_path.search(value)
                    if match:
                        path = match.group(1)
                        if path.startswith("../collectors/"):
                            continue
                    deps[name] = value
            elif state == TAIL:
                tail.append(line)
            else:
                raise RuntimeError("Invalid state")
    # Append all collectors to deps
    for crate in crates:
        if crate.is_collector:
            deps[crate.name] = f'{{path = "../{crate.crate}"}}'
    r = head.copy()
    r.append("")
    r.append("[dependencies]")
    for name in sorted(deps):
        r.append(f"{name} = {deps[name]}")
    r += tail
    r.append("")
    data = "\n".join(r)
    with open(cargo_toml, "w") as f:
        f.write(data)


def main() -> int:
    crates = [read_toml(crate) for crate in iter_crates()]
    #
    status = check_deps(crates)
    expand_registry(crates)
    expand_agent_cargo(crates)
    expand_workspace(crates)
    return 0 if status else 1


if __name__ == "__main__":
    sys.exit(main())
