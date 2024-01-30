use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};
use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");
    }

    let has_sse = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_sse());
    if has_sse {
        println!("CPU supports SSE!");
    }

    if let Some(cparams) = cpuid.get_cache_parameters() {
        for cache in cparams {
            let size = cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets();
            println!("L{}-Cache size is {}", cache.level(), size);
        }
    } else {
        println!("No cache parameter information available")
    }
}
