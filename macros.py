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
    r"^\s*(counter|gauge|gauge_i|gauge_f)!\(\s*(\S+)\s*,\s*\"([^\"]+)\"(\s*,.+?)?\);",
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
                type=match.group(1).split("_")[0].capitalize(),
                help=match.group(3),
                labels=labels,
            )

    def iter_metrics() -> Iterable[Metric]:
        for fn in glob.glob(os.path.join("collectors", "*", "src", "lib.rs")):
            yield from iter_metrics_from_file(fn)

    @env.macro
    def collector_config(name: str) -> str:
        r = [
            "The common collector's configuration is:",
            "",
            "| Parameter  | Type    | Default                   | Description                                        |",
            "| ---------- | ------- | ------------------------- | -------------------------------------------------- |",
            "| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |",
            f"| `type`     | String  |                           | Must be `{name}`                                 |",
            "| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |",
            "| `labels`   | Object  |                           | Additional collector-level labels                  |",
            "| `relabel`  | Array   |                           | Optional relabeling rules. See [Relabeling Rules](../relabel.md) for details |"
            "",
        ]
        return "\n".join(r)

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
                f"| {m.name} | {m.type} | [{m.collector}](collectors/{m.collector}.md) | {', '.join(m.labels)} | {m.help} |"
            )
        return "\n".join(r)
