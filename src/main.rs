use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};
use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();
    if let Some(sif) = cpuid.get_processor_serial() {
        println!("Serial number: {:?}", sif.serial_all());
    } else {
        println!("No serial number information available");
    }
}
