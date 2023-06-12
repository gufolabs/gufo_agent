#!/usr/bin/env python3

# Python modules
import sys
import os
import subprocess
from enum import Enum

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
    update_docs(name)


def configure():
    dn = os.path.dirname(sys.argv[0])
    cfg_path = os.path.join(dn, "configure.py")
    subprocess.check_call(cfg_path)


def update_docs(name: str) -> None:
    update_md("README.md", f"`{name}`")
    update_md("docs/index.md", f"[{name}](collectors/{name}.md)")
    update_md("docs/collectors/index.md", f"[{name}]({name}.md)")
    update_mkdocs_yml(name)
    write_doc_md(name)


class MDState(Enum):
    BEGIN = 1
    COLLECTORS = 2
    TABLE = 3
    REST = 4


def update_md(path: str, name: str) -> None:
    with open(path) as f:
        lines = f.readlines()
    state = MDState.BEGIN
    r = []
    for line in lines:
        if state == MDState.BEGIN and line.startswith("## Available Collectors"):
            state = MDState.COLLECTORS
        elif state == MDState.COLLECTORS and line.startswith("| ---"):
            state = MDState.TABLE
        elif state == MDState.TABLE and line[2:] > name:
            r.append(f"| {name} | ??? |\n")
            state = MDState.REST
        r.append(line)
    with open(path, "w") as f:
        f.write("".join(r))


def update_mkdocs_yml(name: str) -> None:
    with open("mkdocs.yml") as f:
        lines = f.readlines()
    r = []
    state = MDState.BEGIN
    cfg = None
    for line in lines:
        if state == MDState.BEGIN and "- Collectors Reference:" in line:
            state = MDState.COLLECTORS
        elif state == MDState.COLLECTORS and "- Overview: collectors/index.md" in line:
            indent = " " * (len(line) - len(line.lstrip()))
            cfg = f"{indent}- {name}: collectors/{name}.md\n"
            state = MDState.TABLE
        elif state == MDState.TABLE and line > cfg:
            r.append(cfg)
            state = MDState.REST
        r.append(line)
    with open("mkdocs.yml", "w") as f:
        f.write("".join(r))


DOC = """# {name} collector

`{name}` collects ...

## Configuration

{{ collector_config("{name}") }}

Config example:

``` yaml
- id: {name}
  disabled: false
  type: {name}
```

## Collected Metrics

=== "OpenMetrics"

    | Metric | Metric Type | Description  |
    | -------| ----------- | ------------ |


## Labels

`{name}` collector appends the following labels

| Label | Description   |
| ----- | ------------- |

## Sample Output

=== "OpenMetrics"

    ```
    ```
"""


def write_doc_md(name: str) -> None:
    d = DOC.replace("{name}", name)
    with open(f"docs/collectors/{name}.md", "w") as f:
        f.write(d)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage:")
        print(f"  {sys.argv[0]} <name>")
        sys.exit(1)
    main(sys.argv[1])
