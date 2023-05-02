# ----------------------------------------------------------------------
# Documentation macroses
# ----------------------------------------------------------------------
# Copyright (C) 2023 Gufo Labs
# See LICENSE for details
# ----------------------------------------------------------------------

# Python modules
import glob
import os
from dataclasses import dataclass
from typing import List, Iterable
import re
import operator


@dataclass
class Metric(object):
    name: str
    collector: str
    type: str
    help: str
    labels: List[str]


rx_counter = re.compile(
    r"^\s*(counter|gauge)!\(\s*(\S+)\s*,\s*\"([^\"]+)\"(\s*,.+?)?\);",
    re.DOTALL | re.MULTILINE,
)


def define_env(env):
    def iter_metrics_from_file(path: str) -> Iterable[Metric]:
        with open(path) as f:
            data = f.read()
        collector = path.split(os.sep)[1]
        if collector == "_template":
            return
        for match in rx_counter.finditer(data):
            if match.group(4):
                labels = [x.strip() for x in match.group(4).split(",") if x.strip()]
            else:
                labels = []
            yield Metric(
                name=match.group(2),
                collector=collector,
                type=match.group(1),
                help=match.group(3),
                labels=labels,
            )

    def iter_metrics() -> Iterable[Metric]:
        for fn in glob.glob(os.path.join("collectors", "*", "src", "lib.rs")):
            yield from iter_metrics_from_file(fn)

    @env.macro
    def metrics_table() -> str:
        """
        Generate and fill metrics table.
        """
        r = [
            "| Metric | Type | Collector | Labels | Help |",
            "| --- | --- | --- | --- | --- |",
        ]
        for m in sorted(iter_metrics(), key=operator.attrgetter("name")):
            r.append(
                f"| {m.name} | {m.type.capitalize()} | [{m.collector}](collectors/{m.collector}.md) | {', '.join(m.labels)} | {m.help} |"
            )
        return "\n".join(r)
