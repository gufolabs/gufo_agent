# memory collector

`memory` collects the host's memory usage statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
|------------|---------|---------|----------------------------------------------------|
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

  | Metric            | Metric Type | Platform | Description                                                                                                                |
  |-------------------|-------------|----------|----------------------------------------------------------------------------------------------------------------------------|
  | total             | Gauge       |          | Total memory in bytes                                                                                                      |
  | free              | Gauge       |          | Free memory in bytes                                                                                                       |
  | active            | Gauge       | Linux    | active,"Active memory, bytes                                                                                               |
  | active_anon       | Gauge       | Linux    | active_anon,"Active anonymous memory, bytes                                                                                |
  | active_file       | Gauge       | Linux    | active_file,"Active pagecache memory, bytes                                                                                |
  | anon_huge_pages   | Gauge       | Linux    | anon_huge_pages,"Non-file backed huge pages mapped into userspace page tables                                              |
  | anon_pages        | Gauge       | Linux    | anon_pages,"Non-file backed pages mapped into userspace page tables                                                        |
  | bounce            | Gauge       | Linux    | bounce,"Memory used for block device 'bounce buffers'                                                                      |
  | buffers           | Gauge       | Linux    | buffers,"Memory in buffer cache                                                                                            |
  | cached            | Gauge       | Linux    | cached,"Memory in the pagecache                                                                                            |
  | commit_limit      | Gauge       | Linux    | commit_limit,"Total amount of memory currently available to be allocated on the system                                     |
  | committed_as      | Gauge       | Linux    | committed_as," The amount of memory presently allocated on the system                                                      |
  | direct_map_1g     | Gauge       | Linux    | direct_map_1g,"The amount of memory being mapped into the kernel space with 1GB size pages                                 |
  | direct_map_2m     | Gauge       | Linux    | direct_map_2m,"The amount of memory being mapped into the kernel space with 2MB size pages                                 |
  | direct_map_4k     | Gauge       | Linux    | direct_map_4k,"The amount of memory being mapped into the kernel space with 4k size pages                                  |
  | dirty             | Gauge       | Linux    | dirty,"Memory waiting to be written back to disk                                                                           |
  | file_huge_pages   | Gauge       | Linux    | file_huge_pages,"Memory used for filesystem data (page cache) allocated with huge pages                                    |
  | file_pmd_mapped   | Gauge       | Linux    | file_pmd_mapped,"Page cache mapped into userspace with huge pages                                                          |
  | huge_page_size    | Gauge       | Linux    | huge_page_size,"default hugepage size (in kB)                                                                              |
  | huge_tlb          | Gauge       | Linux    | huge_tlb,"total amount of memory (in kB), consumed by huge pages of all sizes                                              |
  | inactive          | Gauge       | Linux    | inactive,"Memory which has been less recently used                                                                         |
  | inactive_anon     | Gauge       | Linux    | inactive_anon,"Anonymous memory that has not been used recently and can be swapped out                                     |
  | inactive_file     | Gauge       | Linux    | inactive_file,"Pagecache memory that can be reclaimed without huge performance impact                                      |
  | k_reclaimable     | Gauge       | Linux    | k_reclaimable,"Kernel allocations that the kernel will attempt to reclaim under memory pressure                            |
  | kernel_stack      | Gauge       | Linux    | kernel_stack,"Memory consumed by the kernel stacks of all tasks                                                            |
  | mapped            | Gauge       | Linux    | mapped,"files which have been mmaped, such as libraries                                                                    |
  | mem_available     | Gauge       | Linux    | mem_available,"estimate of how much memory is available for starting new applications                                      |
  | mem_free          | Gauge       | Linux    | mem_free,"Total free RAM                                                                                                   |
  | mem_total         | Gauge       | Linux    | mem_total,"Total usable RAM                                                                                                |
  | m_locked          | Gauge       | Linux    | m_locked,"Memory locked with mlock()                                                                                       |
  | nfs_unstable      | Gauge       | Linux    | nfs_unstable,"Always zero. Previous counted pages which had been written to the server, but has not been commit to storage |
  | page_tables       | Gauge       | Linux    | page_tables,"Memory consumed by userspace page tables                                                                      |
  | per_cpu           | Gauge       | Linux    | per_cpu,"Memory allocated to the percpu allocator used to back percpu allocations. This stat excludes the cost a           |
  | s_reclaimable     | Gauge       | Linux    | s_reclaimable,"Part of Slab, that might be reclaimed, such as caches                                                       |
  | s_unreclaim       | Gauge       | Linux    | s_unreclaim,"Part of Slab, that cannot be reclaimed on memory pressure                                                     |
  | sh_mem            | Gauge       | Linux    | sh_mem,"Total memory used by shared memory (shmem) and tmpfs                                                               |
  | sh_mem_huge_pages | Gauge       | Linux    | sh_mem_huge_pages,"Memory used by shared memory (shmem) and tmpfs allocated with huge pages                                |
  | sh_mem_pmd_mapped | Gauge       | Linux    | sh_mem_pmd_mapped,"Shared memory mapped into userspace with huge pages                                                     |
  | slab              | Gauge       | Linux    | slab,"in-kernel data structures cache                                                                                      |
  | swap_cached       | Gauge       | Linux    | swap_cached,"Memory that once was swapped out, is swapped back in but still also is in the swapfile                        |
  | swap_free         | Gauge       | Linux    | swap_free,"Memory which has been evicted from RAM, and is temporarily on the disk                                          |
  | swap_total        | Gauge       | Linux    | swap_total,"total amount of swap space available                                                                           |
  | unevictable       | Gauge       | Linux    | unevictable,"Memory allocated for userspace which cannot be reclaimed                                                      |
  | vmalloc_chunk     | Gauge       | Linux    | vmalloc_chunk,"largest contiguous block of vmalloc area which is free                                                      |
  | vmalloc_total     | Gauge       | Linux    | vmalloc_total,"total size of vmalloc virtual address space                                                                 |
  | vmalloc_used      | Gauge       | Linux    | vmalloc_used,"amount of vmalloc area which is used                                                                         |
  | writeback         | Gauge       | Linux    | writeback,"Memory which is actively being written back to the disk                                                         |
  | writeback_tmp     | Gauge       | Linux    | writeback_tmp,"Memory used by FUSE for temporary writeback buffers                                                         |
  | load              | Gauge       | Windows  | ???                                                                                                                        |
  | total_phys        | Gauge       | Windows  | ???                                                                                                                        |
  | avail_phys        | Gauge       | Windows  | ???                                                                                                                        |
  | total_pagefile    | Gauge       | Windows  | ???                                                                                                                        |
  | avail_pagefile    | Gauge       | Windows  | ???                                                                                                                        |
  | total_virt        | Gauge       | Windows  | ???                                                                                                                        |
  | avail_virt        | Gauge       | Windows  | ???                                                                                                                        |
  | avail_ext         | Gauge       | Windows  | ???                                                                                                                        |
  | active            | Gauge       | FreeBSD  | ???                                                                                                                        |
  | inactive          | Gauge       | FreeBSD  | ???                                                                                                                        |
  | wired             | Gauge       | FreeBSD  | ???                                                                                                                        |
  | cache             | Gauge       | FreeBSD  | ???                                                                                                                        |
  | zfs_arc           | Gauge       | FreeBSD  | ???                                                                                                                        |
  | active            | Gauge       | OpenBSD  | ???                                                                                                                        |
  | inactive          | Gauge       | OpenBSD  | ???                                                                                                                        |
  | wired             | Gauge       | OpenBSD  | ???                                                                                                                        |
  | cache             | Gauge       | OpenBSD  | ???                                                                                                                        |
  | paging            | Gauge       | OpenBSD  | ???                                                                                                                        |
  | active            | Gauge       | OS X     | ???                                                                                                                        |
  | inactive          | Gauge       | OS X     | ???                                                                                                                        |
  | wired             | Gauge       | OS X     | ???                                                                                                                        |
  | cache             | Gauge       | OS X     | ???                                                                                                                        |

## Labels

`memory` collector doesn't append its own labels.

## Sample Output

=== "OpenMetrics"

    ```
    # HELP memory_active Active memory, bytes
    # TYPE memory_active gauge
    memory_active{agent="gufo",host="ek-light",zone="DC1"} 3230384128 1682414881
    # HELP memory_active_anon Active anonymous memory, bytes
    # TYPE memory_active_anon gauge
    memory_active_anon{agent="gufo",host="ek-light",zone="DC1"} 931033088 1682414881
    # HELP memory_active_file Active pagecache memory, bytes
    # TYPE memory_active_file gauge
    memory_active_file{agent="gufo",host="ek-light",zone="DC1"} 2299351040 1682414881
    # HELP memory_anon_huge_pages Non-file backed huge pages mapped into userspace page tables
    # TYPE memory_anon_huge_pages gauge
    memory_anon_huge_pages{agent="gufo",host="ek-light",zone="DC1"} 0 1682414881
    # HELP memory_anon_pages Non-file backed pages mapped into userspace page tables
    # TYPE memory_anon_pages gauge
    memory_anon_pages{agent="gufo",host="ek-light",zone="DC1"} 9075548160 1682414881
    # HELP memory_bounce Memory used for block device 'bounce buffers'
    # TYPE memory_bounce gauge
    memory_bounce{agent="gufo",host="ek-light",zone="DC1"} 0 1682414881
    ```