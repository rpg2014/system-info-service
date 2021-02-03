use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Serialize, Deserialize)]
pub struct LoadAverageCopy {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

#[derive(Serialize)]
pub struct NetworkResult {
    pub networks: Vec<NetworkDetails>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum IpAddr {
    Empty,
    Unsupported,
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
#[derive(Debug, Clone, Serialize)]
pub struct NetworkAddrsDetails {
    pub addr: IpAddr,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkDetails {
    pub name: String,
    pub addrs: Vec<NetworkAddrsDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatsResults {
    One(NetworkStats),
    List(Vec<NetworkStats>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filesystem {
    pub files: usize,
    pub files_total: usize,
    pub files_avail: usize,
    pub free: u64,
    pub avail: u64,
    pub total: u64,
    pub name_max: usize,
    pub fs_type: String,
    pub fs_mounted_from: String,
    pub fs_mounted_on: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPULoad {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}
