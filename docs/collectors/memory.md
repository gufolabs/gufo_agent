# cpu collector

`memory` collects host's memory usage statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `memory`                                   |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: CPU
  type: memory
  interval: 10
```

## Collected Metrics

=== "OpenMetrics"
  | Metric            | Metric Type | Platform | Description           |
  | ----------------- | ----------- | -------- | --------------------- |
  | total             | Gauge       |          | Total memory in bytes |
  | free              | Gauge       |          | Free memory in bytes  |
  | active            | Gauge       | Linux    | ???                   |
  | active_anon       | Gauge       | Linux    | ???                   |
  | active_file       | Gauge       | Linux    | ???                   |
  | anon_huge_pages   | Gauge       | Linux    | ???                   |
  | anon_pages        | Gauge       | Linux    | ???                   |
  | bounce            | Gauge       | Linux    | ???                   |
  | buffers           | Gauge       | Linux    | ???                   |
  | cached            | Gauge       | Linux    | ???                   |
  | commit_limit      | Gauge       | Linux    | ???                   |
  | committed_as      | Gauge       | Linux    | ???                   |
  | direct_map_1g     | Gauge       | Linux    | ???                   |
  | direct_map_2m     | Gauge       | Linux    | ???                   |
  | direct_map_4k     | Gauge       | Linux    | ???                   |
  | dirty             | Gauge       | Linux    | ???                   |
  | file_huge_pages   | Gauge       | Linux    | ???                   |
  | file_pmd_mapped   | Gauge       | Linux    | ???                   |
  | huge_page_size    | Gauge       | Linux    | ???                   |
  | huge_tlb          | Gauge       | Linux    | ???                   |
  | inactive          | Gauge       | Linux    | ???                   |
  | inactive_anon     | Gauge       | Linux    | ???                   |
  | inactive_file     | Gauge       | Linux    | ???                   |
  | k_reclaimable     | Gauge       | Linux    | ???                   |
  | kernel_stack      | Gauge       | Linux    | ???                   |
  | mapped            | Gauge       | Linux    | ???                   |
  | mem_available     | Gauge       | Linux    | ???                   |
  | mem_free          | Gauge       | Linux    | ???                   |
  | mem_total         | Gauge       | Linux    | ???                   |
  | m_locked          | Gauge       | Linux    | ???                   |
  | nfs_unstable      | Gauge       | Linux    | ???                   |
  | page_tables       | Gauge       | Linux    | ???                   |
  | per_cpu           | Gauge       | Linux    | ???                   |
  | s_reclaimable     | Gauge       | Linux    | ???                   |
  | s_unreclaim       | Gauge       | Linux    | ???                   |
  | sh_mem            | Gauge       | Linux    | ???                   |
  | sh_mem_huge_pages | Gauge       | Linux    | ???                   |
  | sh_mem_pmd_mapped | Gauge       | Linux    | ???                   |
  | slab              | Gauge       | Linux    | ???                   |
  | swap_cached       | Gauge       | Linux    | ???                   |
  | swap_free         | Gauge       | Linux    | ???                   |
  | swap_total        | Gauge       | Linux    | ???                   |
  | unevictable       | Gauge       | Linux    | ???                   |
  | vmalloc_chunk     | Gauge       | Linux    | ???                   |
  | vmalloc_total     | Gauge       | Linux    | ???                   |
  | vmalloc_used      | Gauge       | Linux    | ???                   |
  | writeback         | Gauge       | Linux    | ???                   |
  | writeback_tmp     | Gauge       | Linux    | ???                   |
  | load              | Gauge       | Windows  | ???                   |
  | total_phys        | Gauge       | Windows  | ???                   |
  | avail_phys        | Gauge       | Windows  | ???                   |
  | total_pagefile    | Gauge       | Windows  | ???                   |
  | avail_pagefile    | Gauge       | Windows  | ???                   |
  | total_virt        | Gauge       | Windows  | ???                   |
  | avail_virt        | Gauge       | Windows  | ???                   |
  | avail_ext         | Gauge       | Windows  | ???                   |
  | active            | Gauge       | FreeBSD  | ???                   |
  | inactive          | Gauge       | FreeBSD  | ???                   |
  | wired             | Gauge       | FreeBSD  | ???                   |
  | cache             | Gauge       | FreeBSD  | ???                   |
  | zfs_arc           | Gauge       | FreeBSD  | ???                   |
  | active            | Gauge       | OpenBSD  | ???                   |
  | inactive          | Gauge       | OpenBSD  | ???                   |
  | wired             | Gauge       | OpenBSD  | ???                   |
  | cache             | Gauge       | OpenBSD  | ???                   |
  | paging            | Gauge       | OpenBSD  | ???                   |
  | active            | Gauge       | OS X     | ???                   |
  | inactive          | Gauge       | OS X     | ???                   |
  | wired             | Gauge       | OS X     | ???                   |
  | cache             | Gauge       | OS X     | ???                   |

## Labels

`memory` collector doesn't append own labels.

## Sample Output