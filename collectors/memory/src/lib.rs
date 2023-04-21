// --------------------------------------------------------------------
// Gufo Agent: memory collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use cfg_if::cfg_if;
use common::{gauge, AgentError, Collectable, Measure};
use serde::Deserialize;
use systemstat::{Platform, PlatformMemory, System};

// Collector config
#[derive(Deserialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
gauge!(total, "Total memory in bytes");
gauge!(free, "Free memory in bytes");
cfg_if! {
    if #[cfg(target_os = "linux")] {
        gauge!(active,"???");
        gauge!(active_anon,"???");
        gauge!(active_file,"???");
        gauge!(anon_huge_pages,"???");
        gauge!(anon_pages,"???");
        gauge!(bounce,"???");
        gauge!(buffers,"???");
        gauge!(cached,"???");
        gauge!(commit_limit,"???");
        gauge!(committed_as,"???");
        gauge!(direct_map_1g,"???");
        gauge!(direct_map_2m,"???");
        gauge!(direct_map_4k,"???");
        gauge!(dirty,"???");
        gauge!(file_huge_pages,"???");
        gauge!(file_pmd_mapped,"???");
        gauge!(huge_page_size,"???");
        gauge!(huge_tlb,"???");
        gauge!(inactive,"???");
        gauge!(inactive_anon,"???");
        gauge!(inactive_file,"???");
        gauge!(k_reclaimable,"???");
        gauge!(kernel_stack,"???");
        gauge!(mapped,"???");
        gauge!(mem_available,"???");
        gauge!(mem_free,"???");
        gauge!(mem_total,"???");
        gauge!(m_locked,"???");
        gauge!(nfs_unstable,"???");
        gauge!(page_tables,"???");
        gauge!(per_cpu,"???");
        gauge!(s_reclaimable,"???");
        gauge!(s_unreclaim,"???");
        gauge!(sh_mem,"???");
        gauge!(sh_mem_huge_pages,"???");
        gauge!(sh_mem_pmd_mapped,"???");
        gauge!(slab,"???");
        gauge!(swap_cached,"???");
        gauge!(swap_free,"???");
        gauge!(swap_total,"???");
        gauge!(unevictable,"???");
        gauge!(vmalloc_chunk,"???");
        gauge!(vmalloc_total,"???");
        gauge!(vmalloc_used,"???");
        gauge!(writeback,"???");
        gauge!(writeback_tmp,"???");
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
