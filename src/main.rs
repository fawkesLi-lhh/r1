use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::thread;
use std::time::Duration;


fn main() {
    let linux = sys_info::linux_os_release().unwrap();
    println!("{:?}",linux);
}
