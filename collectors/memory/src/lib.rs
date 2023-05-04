// --------------------------------------------------------------------
// Gufo Agent: memory collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use cfg_if::cfg_if;
use common::{gauge, AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::{Deserialize, Serialize};
use systemstat::{Platform, PlatformMemory, System};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
gauge!(mem_total, "Total memory in bytes");
gauge!(mem_free, "Free memory in bytes");
cfg_if! {
    if #[cfg(target_os = "linux")] {
        gauge!(mem_active,"Active memory, bytes");
        gauge!(mem_active_anon,"Active anonymous memory, bytes");
        gauge!(mem_active_file,"Active pagecache memory, bytes");
        gauge!(mem_anon_huge_pages,"Non-file backed huge pages mapped into userspace page tables");
        gauge!(mem_anon_pages,"Non-file backed pages mapped into userspace page tables");
        gauge!(mem_bounce,"Memory used for block device 'bounce buffers'");
        gauge!(mem_buffers,"Memory in buffer cache");
        gauge!(mem_cached,"Memory in the pagecache");
        gauge!(mem_commit_limit,"Total amount of memory currently available to be allocated on the system");
        gauge!(mem_committed_as," The amount of memory presently allocated on the system");
        gauge!(mem_direct_map_1g,"The amount of memory being mapped into the kernel space with 1GB size pages");
        gauge!(mem_direct_map_2m,"The amount of memory being mapped into the kernel space with 2MB size pages");
        gauge!(mem_direct_map_4k,"The amount of memory being mapped into the kernel space with 4k size pages");
        gauge!(mem_dirty,"Memory waiting to be written back to disk");
        gauge!(mem_file_huge_pages,"Memory used for filesystem data (page cache) allocated with huge pages");
        gauge!(mem_file_pmd_mapped,"Page cache mapped into userspace with huge pages");
        gauge!(mem_huge_page_size,"default hugepage size (in kB)");
        gauge!(mem_huge_tlb,"total amount of memory (in kB), consumed by huge pages of all sizes");
        gauge!(mem_inactive,"Memory which has been less recently used");
        gauge!(mem_inactive_anon,"Anonymous memory that has not been used recently and can be swapped out");
        gauge!(mem_inactive_file,"Pagecache memory that can be reclaimed without huge performance impact");
        gauge!(mem_k_reclaimable,"Kernel allocations that the kernel will attempt to reclaim under memory pressure");
        gauge!(mem_kernel_stack,"Memory consumed by the kernel stacks of all tasks");
        gauge!(mem_mapped,"files which have been mmaped, such as libraries");
        gauge!(mem_available,"estimate of how much memory is available for starting new applications");
        gauge!(mem_m_locked,"Memory locked with mlock()");
        gauge!(mem_nfs_unstable,"Always zero. Previous counted pages which had been written to the server, but has not been committed to stable storage");
        gauge!(mem_page_tables,"Memory consumed by userspace page tables");
        gauge!(mem_per_cpu,"Memory allocated to the percpu allocator used to back percpu allocations. This stat excludes the cost of metadata");
        gauge!(mem_s_reclaimable,"Part of Slab, that might be reclaimed, such as caches");
        gauge!(mem_s_unreclaim,"Part of Slab, that cannot be reclaimed on memory pressure");
        gauge!(mem_sh_mem,"Total memory used by shared memory (shmem) and tmpfs");
        gauge!(mem_sh_mem_huge_pages,"Memory used by shared memory (shmem) and tmpfs allocated with huge pages");
        gauge!(mem_sh_mem_pmd_mapped,"Shared memory mapped into userspace with huge pages");
        gauge!(mem_slab,"in-kernel data structures cache");
        gauge!(swap_cached,"Memory that once was swapped out, is swapped back in but still also is in the swapfile");
        gauge!(swap_free,"Memory which has been evicted from RAM, and is temporarily on the disk");
        gauge!(swap_total,"total amount of swap space available");
        gauge!(mem_unevictable,"Memory allocated for userspace which cannot be reclaimed");
        gauge!(mem_vmalloc_chunk,"largest contiguous block of vmalloc area which is free");
        gauge!(mem_vmalloc_total,"total size of vmalloc virtual address space");
        gauge!(mem_vmalloc_used,"amount of vmalloc area which is used");
        gauge!(mem_writeback,"Memory which is actively being written back to the disk");
        gauge!(mem_writeback_tmp,"Memory used by FUSE for temporary writeback buffers");
    } else if #[cfg(target_os = "windows")] {
        gauge!(mem_load,"???");
        gauge!(mem_total_phys,"???");
        gauge!(mem_avail_phys,"???");
        gauge!(mem_total_pagefile,"???");
        gauge!(mem_avail_pagefile,"???");
        gauge!(mem_total_virt,"???");
        gauge!(mem_avail_virt,"???");
        gauge!(mem_avail_ext,"???");
    } else if #[cfg(target_os = "freebsd")] {
        gauge!(mem_active,"???");
        gauge!(mem_inactive,"???");
        gauge!(mem_wired,"???");
        gauge!(mem_cache,"???");
        gauge!(mem_zfs_arc,"???");
    } else if #[cfg(target_os = "openbsd")] {
        gauge!(mem_active,"???");
        gauge!(mem_inactive,"???");
        gauge!(mem_wired,"???");
        gauge!(mem_cache,"???");
        gauge!(mem_paging,"???");
    } else if #[cfg(target_os = "macos")] {
        gauge!(mem_active,"???");
        gauge!(mem_inactive,"???");
        gauge!(mem_wired,"???");
        gauge!(mem_cache,"???");
    }
}

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(_: Config) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    // !!! Set proper name
    const NAME: &'static str = "memory";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let memory = System::new()
            .memory()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut r = vec![
            mem_total(memory.total.as_u64()),
            mem_total(memory.free.as_u64()),
        ];
        // Add platform-specific data
        Self::apply_platform(&mut r, &memory.platform_memory);
        // Push result
        Ok(r)
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}

impl Collector {
    cfg_if! {
        if #[cfg(target_os = "linux")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                for (key, measure) in mem.meminfo.iter() {
                    let mv = measure.as_u64();
                    match &key.to_lowercase()[..] {
                        "active" => r.push(mem_active(mv)),
                        "active(anon)" => r.push(mem_active_anon(mv)),
                        "active(file)" => r.push(mem_active_file(mv)),
                        "anonhugepages" => r.push(mem_anon_huge_pages(mv)),
                        "anonpages" => r.push(mem_anon_pages(mv)),
                        "bounce" => r.push(mem_bounce(mv)),
                        "buffers" => r.push(mem_buffers(mv)),
                        "cached" => r.push(mem_cached(mv)),
                        "commitlimit" => r.push(mem_commit_limit(mv)),
                        "committed_as" => r.push(mem_committed_as(mv)),
                        "directmap1g" => r.push(mem_direct_map_1g(mv)),
                        "directmap2m" => r.push(mem_direct_map_2m(mv)),
                        "directmap4k" => r.push(mem_direct_map_4k(mv)),
                        "dirty" => r.push(mem_dirty(mv)),
                        "filehugepages" => r.push(mem_file_huge_pages(mv)),
                        "filepmdmapped" => r.push(mem_file_pmd_mapped(mv)),
                        "hugepagesize" => r.push(mem_huge_page_size(mv)),
                        "hugetlb" => r.push(mem_huge_tlb(mv)),
                        "inactive" => r.push(mem_inactive(mv)),
                        "inactive(anon)" => r.push(mem_inactive_anon(mv)),
                        "inactive(file)" => r.push(mem_inactive_file(mv)),
                        "kreclaimable" => r.push(mem_k_reclaimable(mv)),
                        "kernelstack" => r.push(mem_kernel_stack(mv)),
                        "mapped" => r.push(mem_mapped(mv)),
                        "memavailable" => r.push(mem_available(mv)),
                        "memfree" => r.push(mem_free(mv)),
                        "memtotal" => r.push(mem_total(mv)),
                        "mlocked" => r.push(mem_m_locked(mv)),
                        "nfs_unstable" => r.push(mem_nfs_unstable(mv)),
                        "pagetables" => r.push(mem_page_tables(mv)),
                        "percpu" => r.push(mem_per_cpu(mv)),
                        "sreclaimable" => r.push(mem_s_reclaimable(mv)),
                        "sunreclaim" => r.push(mem_s_unreclaim(mv)),
                        "shmem" => r.push(mem_sh_mem(mv)),
                        "shmemhugepages" => r.push(mem_sh_mem_huge_pages(mv)),
                        "shmempmdmapped" => r.push(mem_sh_mem_pmd_mapped(mv)),
                        "slab" => r.push(mem_slab(mv)),
                        "swapcached" => r.push(swap_cached(mv)),
                        "swapfree" => r.push(swap_free(mv)),
                        "swaptotal" => r.push(swap_total(mv)),
                        "unevictable" => r.push(mem_unevictable(mv)),
                        "vmallocchunk" => r.push(mem_vmalloc_chunk(mv)),
                        "vmalloctotal" => r.push(mem_vmalloc_total(mv)),
                        "vmallocused" => r.push(mem_vmalloc_used(mv)),
                        "writeback" => r.push(mem_writeback(mv)),
                        "writebacktmp" => r.push(mem_writeback_tmp(mv)),
                        _ => {}
                    }
                }
            }
        } else if #[cfg(target_os = "windows")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(mem_load(value.load));
                r.push(mem_total_phys(value.total_phys.as_u64()));
                r.push(mem_avail_phys(value.avail_phys.as_u64()));
                r.push(mem_total_pagefile(value.total_pagefile.as_u64()));
                r.push(mem_avail_pagefile(value.avail_pagefile.as_u64()));
                r.push(mem_total_virt(value.total_virt.as_u64()));
                r.push(mem_avail_virt(value.avail_virt.as_u64()));
                r.push(mem_avail_ext(value.avail_ext.as_u64()));
            }
        } else if #[cfg(target_os = "freebsd")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(mem_active(mem.active.as_u64()));
                r.push(mem_inactive(mem.inactive.as_u64()));
                r.push(mem_wired(mem.wired.as_u64()));
                r.push(mem_cache(mem.cache.as_u64()));
                r.push(mem_zfs_arc(mem.zfs_arc.as_u64()));
            }
        } else if #[cfg(target_os = "openbsd")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(mem_active(mem.active.as_u64()));
                r.push(mem_inactive(mem.inactive.as_u64()));
                r.push(mem_wired(mem.wired.as_u64()));
                r.push(mem_cache(mem.cache.as_u64()));
                r.push(mem_paging(mem.paging.as_u64()));
            }
        } else if #[cfg(target_os = "macos")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(mem_active(mem.active.as_u64()));
                r.push(mem_inactive(mem.inactive.as_u64()));
                r.push(mem_wired(mem.wired.as_u64()));
                r.push(mem_cache(mem.cache.as_u64()));
            }
        } else {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {}
        }
    }
}
