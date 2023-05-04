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
gauge!(total, "Total memory in bytes");
gauge!(free, "Free memory in bytes");
cfg_if! {
    if #[cfg(target_os = "linux")] {
        gauge!(active,"Active memory, bytes");
        gauge!(active_anon,"Active anonymous memory, bytes");
        gauge!(active_file,"Active pagecache memory, bytes");
        gauge!(anon_huge_pages,"Non-file backed huge pages mapped into userspace page tables");
        gauge!(anon_pages,"Non-file backed pages mapped into userspace page tables");
        gauge!(bounce,"Memory used for block device 'bounce buffers'");
        gauge!(buffers,"Memory in buffer cache");
        gauge!(cached,"Memory in the pagecache");
        gauge!(commit_limit,"Total amount of memory currently available to be allocated on the system");
        gauge!(committed_as," The amount of memory presently allocated on the system");
        gauge!(direct_map_1g,"The amount of memory being mapped into the kernel space with 1GB size pages");
        gauge!(direct_map_2m,"The amount of memory being mapped into the kernel space with 2MB size pages");
        gauge!(direct_map_4k,"The amount of memory being mapped into the kernel space with 4k size pages");
        gauge!(dirty,"Memory waiting to be written back to disk");
        gauge!(file_huge_pages,"Memory used for filesystem data (page cache) allocated with huge pages");
        gauge!(file_pmd_mapped,"Page cache mapped into userspace with huge pages");
        gauge!(huge_page_size,"default hugepage size (in kB)");
        gauge!(huge_tlb,"total amount of memory (in kB), consumed by huge pages of all sizes");
        gauge!(inactive,"Memory which has been less recently used");
        gauge!(inactive_anon,"Anonymous memory that has not been used recently and can be swapped out");
        gauge!(inactive_file,"Pagecache memory that can be reclaimed without huge performance impact");
        gauge!(k_reclaimable,"Kernel allocations that the kernel will attempt to reclaim under memory pressure");
        gauge!(kernel_stack,"Memory consumed by the kernel stacks of all tasks");
        gauge!(mapped,"files which have been mmaped, such as libraries");
        gauge!(mem_available,"estimate of how much memory is available for starting new applications");
        gauge!(mem_free,"Total free RAM");
        gauge!(mem_total,"Total usable RAM");
        gauge!(m_locked,"Memory locked with mlock()");
        gauge!(nfs_unstable,"Always zero. Previous counted pages which had been written to the server, but has not been committed to stable storage");
        gauge!(page_tables,"Memory consumed by userspace page tables");
        gauge!(per_cpu,"Memory allocated to the percpu allocator used to back percpu allocations. This stat excludes the cost of metadata");
        gauge!(s_reclaimable,"Part of Slab, that might be reclaimed, such as caches");
        gauge!(s_unreclaim,"Part of Slab, that cannot be reclaimed on memory pressure");
        gauge!(sh_mem,"Total memory used by shared memory (shmem) and tmpfs");
        gauge!(sh_mem_huge_pages,"Memory used by shared memory (shmem) and tmpfs allocated with huge pages");
        gauge!(sh_mem_pmd_mapped,"Shared memory mapped into userspace with huge pages");
        gauge!(slab,"in-kernel data structures cache");
        gauge!(swap_cached,"Memory that once was swapped out, is swapped back in but still also is in the swapfile");
        gauge!(swap_free,"Memory which has been evicted from RAM, and is temporarily on the disk");
        gauge!(swap_total,"total amount of swap space available");
        gauge!(unevictable,"Memory allocated for userspace which cannot be reclaimed");
        gauge!(vmalloc_chunk,"largest contiguous block of vmalloc area which is free");
        gauge!(vmalloc_total,"total size of vmalloc virtual address space");
        gauge!(vmalloc_used,"amount of vmalloc area which is used");
        gauge!(writeback,"Memory which is actively being written back to the disk");
        gauge!(writeback_tmp,"Memory used by FUSE for temporary writeback buffers");
    } else if #[cfg(target_os = "windows")] {
        gauge!(load,"???");
        gauge!(total_phys,"???");
        gauge!(avail_phys,"???");
        gauge!(total_pagefile,"???");
        gauge!(avail_pagefile,"???");
        gauge!(total_virt,"???");
        gauge!(avail_virt,"???");
        gauge!(avail_ext,"???");
    } else if #[cfg(target_os = "freebsd")] {
        gauge!(active,"???");
        gauge!(inactive,"???");
        gauge!(wired,"???");
        gauge!(cache,"???");
        gauge!(zfs_arc,"???");
    } else if #[cfg(target_os = "openbsd")] {
        gauge!(active,"???");
        gauge!(inactive,"???");
        gauge!(wired,"???");
        gauge!(cache,"???");
        gauge!(paging,"???");
    } else if #[cfg(target_os = "macos")] {
        gauge!(active,"???");
        gauge!(inactive,"???");
        gauge!(wired,"???");
        gauge!(cache,"???");
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
        let mut r = vec![total(memory.total.as_u64()), total(memory.free.as_u64())];
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
                        "active" => r.push(active(mv)),
                        "active(anon)" => r.push(active_anon(mv)),
                        "active(file)" => r.push(active_file(mv)),
                        "anonhugepages" => r.push(anon_huge_pages(mv)),
                        "anonpages" => r.push(anon_pages(mv)),
                        "bounce" => r.push(bounce(mv)),
                        "buffers" => r.push(buffers(mv)),
                        "cached" => r.push(cached(mv)),
                        "commitlimit" => r.push(commit_limit(mv)),
                        "committed_as" => r.push(committed_as(mv)),
                        "directmap1g" => r.push(direct_map_1g(mv)),
                        "directmap2m" => r.push(direct_map_2m(mv)),
                        "directmap4k" => r.push(direct_map_4k(mv)),
                        "dirty" => r.push(dirty(mv)),
                        "filehugepages" => r.push(file_huge_pages(mv)),
                        "filepmdmapped" => r.push(file_pmd_mapped(mv)),
                        "hugepagesize" => r.push(huge_page_size(mv)),
                        "hugetlb" => r.push(huge_tlb(mv)),
                        "inactive" => r.push(inactive(mv)),
                        "inactive(anon)" => r.push(inactive_anon(mv)),
                        "inactive(file)" => r.push(inactive_file(mv)),
                        "kreclaimable" => r.push(k_reclaimable(mv)),
                        "kernelstack" => r.push(kernel_stack(mv)),
                        "mapped" => r.push(mapped(mv)),
                        "memavailable" => r.push(mem_available(mv)),
                        "memfree" => r.push(mem_free(mv)),
                        "memtotal" => r.push(mem_total(mv)),
                        "mlocked" => r.push(m_locked(mv)),
                        "nfs_unstable" => r.push(nfs_unstable(mv)),
                        "pagetables" => r.push(page_tables(mv)),
                        "percpu" => r.push(per_cpu(mv)),
                        "sreclaimable" => r.push(s_reclaimable(mv)),
                        "sunreclaim" => r.push(s_unreclaim(mv)),
                        "shmem" => r.push(sh_mem(mv)),
                        "shmemhugepages" => r.push(sh_mem_huge_pages(mv)),
                        "shmempmdmapped" => r.push(sh_mem_pmd_mapped(mv)),
                        "slab" => r.push(slab(mv)),
                        "swapcached" => r.push(swap_cached(mv)),
                        "swapfree" => r.push(swap_free(mv)),
                        "swaptotal" => r.push(swap_total(mv)),
                        "unevictable" => r.push(unevictable(mv)),
                        "vmallocchunk" => r.push(vmalloc_chunk(mv)),
                        "vmalloctotal" => r.push(vmalloc_total(mv)),
                        "vmallocused" => r.push(vmalloc_used(mv)),
                        "writeback" => r.push(writeback(mv)),
                        "writebacktmp" => r.push(writeback_tmp(mv)),
                        _ => {}
                    }
                }
            }
        } else if #[cfg(target_os = "windows")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(load(value.load));
                r.push(total_phys(value.total_phys.as_u64()));
                r.push(avail_phys(value.avail_phys.as_u64()));
                r.push(total_pagefile(value.total_pagefile.as_u64()));
                r.push(avail_pagefile(value.avail_pagefile.as_u64()));
                r.push(total_virt(value.total_virt.as_u64()));
                r.push(avail_virt(value.avail_virt.as_u64()));
                r.push(avail_ext(value.avail_ext.as_u64()));
            }
        } else if #[cfg(target_os = "freebsd")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(active(mem.active.as_u64()));
                r.push(inactive(mem.inactive.as_u64()));
                r.push(wired(mem.wired.as_u64()));
                r.push(cache(mem.cache.as_u64()));
                r.push(zfs_arc(mem.zfs_arc.as_u64()));
            }
        } else if #[cfg(target_os = "openbsd")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(active(mem.active.as_u64()));
                r.push(inactive(mem.inactive.as_u64()));
                r.push(wired(mem.wired.as_u64()));
                r.push(cache(mem.cache.as_u64()));
                r.push(paging(mem.paging.as_u64()));
            }
        } else if #[cfg(target_os = "macos")] {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {
                r.push(active(mem.active.as_u64()));
                r.push(inactive(mem.inactive.as_u64()));
                r.push(wired(mem.wired.as_u64()));
                r.push(cache(mem.cache.as_u64()));
            }
        } else {
            fn apply_platform(r: &mut Vec<Measure>, mem: &PlatformMemory) {}
        }
    }
}
