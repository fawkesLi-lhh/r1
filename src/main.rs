use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::thread;
use std::time::Duration;
pub fn init_ip_info()->anyhow::Result<()>{
    let my_local_ip = local_ip_address::local_ip()?;
    println!("{:?}",my_local_ip);
    Ok(())
}

fn main() {
    if let Err(e) = init_ip_info() {
        println!("Error: {}", e);
    }

    
}
