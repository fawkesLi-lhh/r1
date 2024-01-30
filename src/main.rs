use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::thread;
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("used swap   : {} bytes", sys.global_cpu_info().name());

}
