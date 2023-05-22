# memory collector

`memory` collects the host's memory usage statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `memory`                                   |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |

Config example:

``` yaml
- id: CPU
  type: memory
```

## Collected Metrics

=== "OpenMetrics"

  | Metric                | Metric Type | Platform | Description                                                                                                  |
  | --------------------- | ----------- | -------- | ------------------------------------------------------------------------------------------------------------ |
  | mem_total             | Gauge       |          | Total mem ory in bytes                                                                                       |
  | mem_free              | Gauge       |          | Free memory in bytes                                                                                         |
  | mem_active            | Gauge       | Linux    | Active memory, bytes                                                                                         |
  | mem_active_anon       | Gauge       | Linux    | Active anonymous memory, bytes                                                                               |
  | mem_active_file       | Gauge       | Linux    | Active pagecache memory, bytes                                                                               |
  | mem_anon_huge_pages   | Gauge       | Linux    | Non-file backed huge pages mapped into userspace page tables                                                 |
  | mem_anon_pages        | Gauge       | Linux    | Non-file backed pages mapped into userspace page tables                                                      |
  | mem_bounce            | Gauge       | Linux    | Memory used for block device 'bounce buffers'                                                                |
  | mem_buffers           | Gauge       | Linux    | Memory in buffer cache                                                                                       |
  | mem_cached            | Gauge       | Linux    | Memory in the pagecache                                                                                      |
  | mem_commit_limit      | Gauge       | Linux    | Total amount of memory currently available to be allocated on the system                                     |
  | mem_committed_as      | Gauge       | Linux    | The amount of memory presently allocated on the system                                                       |
  | mem_direct_map_1g     | Gauge       | Linux    | The amount of memory being mapped into the kernel space with 1GB size pages                                  |
  | mem_direct_map_2m     | Gauge       | Linux    | The amount of memory being mapped into the kernel space with 2MB size pages                                  |
  | mem_direct_map_4k     | Gauge       | Linux    | The amount of memory being mapped into the kernel space with 4k size pages                                   |
  | mem_dirty             | Gauge       | Linux    | Memory waiting to be written back to disk                                                                    |
  | mem_file_huge_pages   | Gauge       | Linux    | Memory used for filesystem data (page cache) allocated with huge pages                                       |
  | mem_file_pmd_mapped   | Gauge       | Linux    | Page cache mapped into userspace with huge pages                                                             |
  | mem_huge_page_size    | Gauge       | Linux    | default hugepage size (in kB)                                                                                |
  | mem_huge_tlb          | Gauge       | Linux    | total amount of memory (in kB), consumed by huge pages of all sizes                                          |
  | mem_inactive          | Gauge       | Linux    | Memory which has been less recently used                                                                     |
  | mem_inactive_anon     | Gauge       | Linux    | Anonymous memory that has not been used recently and can be swapped out                                      |
  | mem_inactive_file     | Gauge       | Linux    | Pagecache memory that can be reclaimed without huge performance impact                                       |
  | mem_k_reclaimable     | Gauge       | Linux    | Kernel allocations that the kernel will attempt to reclaim under memory pressure                             |
  | mem_kernel_stack      | Gauge       | Linux    | Memory consumed by the kernel stacks of all tasks                                                            |
  | mem_mapped            | Gauge       | Linux    | files which have been mmaped, such as libraries                                                              |
  | mem_mem_available     | Gauge       | Linux    | estimate of how much memory is available for starting new applications                                       |
  | mem_mem_free          | Gauge       | Linux    | Total free RAM                                                                                               |
  | mem_mem_total         | Gauge       | Linux    | Total usable RAM                                                                                             |
  | mem_m_locked          | Gauge       | Linux    | Memory locked with mlock()                                                                                   |
  | mem_nfs_unstable      | Gauge       | Linux    | Always zero. Previous counted pages which had been written to the server, but has not been commit to storage |
  | mem_page_tables       | Gauge       | Linux    | Memory consumed by userspace page tables                                                                     |
  | mem_per_cpu           | Gauge       | Linux    | Memory allocated to the percpu allocator used to back percpu allocations. This stat excludes the cost a      |
  | mem_s_reclaimable     | Gauge       | Linux    | Part of Slab, that might be reclaimed, such as caches                                                        |
  | mem_s_unreclaim       | Gauge       | Linux    | Part of Slab, that cannot be reclaimed on memory pressure                                                    |
  | mem_sh_mem            | Gauge       | Linux    | Total memory used by shared memory (shmem) and tmpfs                                                         |
  | mem_sh_mem_huge_pages | Gauge       | Linux    | Memory used by shared memory (shmem) and tmpfs allocated with huge pages                                     |
  | mem_sh_mem_pmd_mapped | Gauge       | Linux    | Shared memory mapped into userspace with huge pages                                                          |
  | mem_slab              | Gauge       | Linux    | in-kernel data structures cache                                                                              |
  | swap_cached           | Gauge       | Linux    | Memory that once was swapped out, is swapped back in but still also is in the swapfile                       |
  | swap_free             | Gauge       | Linux    | Memory which has been evicted from RAM, and is temporarily on the disk                                       |
  | swap_total            | Gauge       | Linux    | total amount of swap space available                                                                         |
  | mem_unevictable       | Gauge       | Linux    | Memory allocated for userspace which cannot be reclaimed                                                     |
  | mem_vmalloc_chunk     | Gauge       | Linux    | largest contiguous block of vmalloc area which is free                                                       |
  | mem_vmalloc_total     | Gauge       | Linux    | total size of vmalloc virtual address space                                                                  |
  | mem_vmalloc_used      | Gauge       | Linux    | amount of vmalloc area which is used                                                                         |
  | mem_writeback         | Gauge       | Linux    | Memory which is actively being written back to the disk                                                      |
  | mem_writeback_tmp     | Gauge       | Linux    | Memory used by FUSE for temporary writeback buffers                                                          |
  | mem_load              | Gauge       | Windows  | ???                                                                                                          |
  | mem_total_phys        | Gauge       | Windows  | ???                                                                                                          |
  | mem_avail_phys        | Gauge       | Windows  | ???                                                                                                          |
  | mem_total_pagefile    | Gauge       | Windows  | ???                                                                                                          |
  | mem_avail_pagefile    | Gauge       | Windows  | ???                                                                                                          |
  | mem_total_virt        | Gauge       | Windows  | ???                                                                                                          |
  | mem_avail_virt        | Gauge       | Windows  | ???                                                                                                          |
  | mem_avail_ext         | Gauge       | Windows  | ???                                                                                                          |
  | mem_active            | Gauge       | FreeBSD  | ???                                                                                                          |
  | mem_inactive          | Gauge       | FreeBSD  | ???                                                                                                          |
  | mem_wired             | Gauge       | FreeBSD  | ???                                                                                                          |
  | mem_cache             | Gauge       | FreeBSD  | ???                                                                                                          |
  | mem_zfs_arc           | Gauge       | FreeBSD  | ???                                                                                                          |
  | mem_active            | Gauge       | OpenBSD  | ???                                                                                                          |
  | mem_inactive          | Gauge       | OpenBSD  | ???                                                                                                          |
  | mem_wired             | Gauge       | OpenBSD  | ???                                                                                                          |
  | mem_cache             | Gauge       | OpenBSD  | ???                                                                                                          |
  | mem_paging            | Gauge       | OpenBSD  | ???                                                                                                          |
  | mem_active            | Gauge       | OS X     | ???                                                                                                          |
  | mem_inactive          | Gauge       | OS X     | ???                                                                                                          |
  | mem_wired             | Gauge       | OS X     | ???                                                                                                          |
  | mem_cache             | Gauge       | OS X     | ???                                                                                                          |

## Labels

`memory` collector doesn't append its own labels.

## Config Discovery

`memory` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-memory
```

## Sample Output

=== "OpenMetrics"

    ```
    # HELP memory_active Active memory, bytes
    # TYPE memory_active gauge
    memory_active 3230384128 1682414881
    # HELP memory_active_anon Active anonymous memory, bytes
    # TYPE memory_active_anon gauge
    memory_active_anon 931033088 1682414881
    # HELP memory_active_file Active pagecache memory, bytes
    # TYPE memory_active_file gauge
    memory_active_file 2299351040 1682414881
    # HELP memory_anon_huge_pages Non-file backed huge pages mapped into userspace page tables
    # TYPE memory_anon_huge_pages gauge
    memory_anon_huge_pages 0 1682414881
    # HELP memory_anon_pages Non-file backed pages mapped into userspace page tables
    # TYPE memory_anon_pages gauge
    memory_anon_pages 9075548160 1682414881
    # HELP memory_bounce Memory used for block device 'bounce buffers'
    # TYPE memory_bounce gauge
    memory_bounce 0 1682414881
    ```