use dashmap::DashMap;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use systemstat::{Platform, System};

pub static SysInfo: Lazy<DashMap<SysInfoType, String>> = Lazy::new(|| Default::default());
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
pub enum SysInfoType {
    // 操作系统
    Windows,
    // 内网IP（LIP）
    LIP,
    // 硬盘序列号
    HD,
    // PC终端设备名
    PCN,
    //CPU序列号：！！！！！！！
    CPU,
    // 硬盘分区信息：（例如: C^NTFS^100G)
    PI,
    // 系统盘卷标号：（选填）
    VOL,
    //公网
    IIP,
    IPORT,
}

pub fn init_ip_info() -> anyhow::Result<()> {
    let my_local_ip = local_ip_address::local_ip()?;
    SysInfo.insert(SysInfoType::LIP, my_local_ip.to_string());
    Ok(())
}

pub fn init_linux_disk_info() -> anyhow::Result<()> {
    use std::process::Command;
    let cmd = Command::new("lsblk")
        .arg("--nodeps")
        .arg("-o")
        .arg("NAME,SERIAL")
        .output()?;
    let output = String::from_utf8(cmd.stdout)?;
    let ans = output.split('\n').map(|v|v.to_string()).collect::<Vec<_>>();

    println!("sn: {:?}", ans);
    Ok(())
}

pub fn init_sysinfo() -> anyhow::Result<()> {
    init_ip_info()?;
    #[cfg(target_os = "windows")]
    {
        SysInfo.insert(SysInfoType::Windows, "PC".to_string());
    }
    #[cfg(target_os = "linux")]
    {
        init_linux_disk_info()?;
        SysInfo.insert(SysInfoType::Windows, "LINUX_PC".to_string());
    }
    #[cfg(target_os = "macos")]
    {
        SysInfo.insert(SysInfoType::Windows, "MAC_PC".to_string());
    }
    Ok(())
}

fn main() {
    init_sysinfo().unwrap();
    println!("{:?}", SysInfo);
}
