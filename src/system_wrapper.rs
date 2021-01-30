extern crate rocket;
extern crate systemstat;

// pub mod system_wrapper {
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
pub use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;
use systemstat::{NetworkAddrs, Platform, System};

pub fn get_uptime() -> Result<Duration, NotFound<Json<String>>> {
    let sys = System::new();
    sys.uptime()
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))
}

#[derive(Serialize, Deserialize)]
pub struct LoadAverageCopy {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

pub fn get_load_average() -> Result<LoadAverageCopy, NotFound<Json<String>>> {
    let sys = System::new();
    let load = sys
        .load_average()
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))?;
    Ok(LoadAverageCopy {
        one: load.one,
        five: load.five,
        fifteen: load.fifteen,
    })
}
#[derive(Serialize)]
pub struct NetworkResult {
    networks: Vec<NetworkDetails>,
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

fn get_network_addrs(addrs: &Vec<NetworkAddrs>) -> Vec<NetworkAddrsDetails> {
    let mut transformed_addrs = Vec::new();
    for addr in addrs {
        transformed_addrs.push(NetworkAddrsDetails {
            addr: match addr.addr {
                systemstat::IpAddr::V4(a) => IpAddr::V4(a),
                systemstat::IpAddr::V6(a) => IpAddr::V6(a),
                systemstat::IpAddr::Empty => IpAddr::Empty,
                systemstat::IpAddr::Unsupported => IpAddr::Unsupported,
            },
        })
    }
    transformed_addrs
}

pub fn get_networks() -> Result<NetworkResult, NotFound<Json<String>>> {
    let sys = System::new();
    let networks = sys
        .networks()
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))?;
    let mut result = Vec::new();
    for net_info in networks.values() {
        let network_info = NetworkDetails {
            name: net_info.name.clone(),
            addrs: get_network_addrs(&net_info.addrs),
        };
        result.push(network_info)
    }

    Ok(NetworkResult { networks: result })
    // for netif in networks.values()
    //         Ok(netifs) => {
    //             println!("\nNetworks:");
    //             for netif in netifs.values() {
    //                 println!("{} ({:?})", netif.name, netif.addrs);
    //             }
    //         }
    //         Err(x) => println!("\nNetworks: error: {}", x)
}

//     match sys.mounts() {
//     Ok(mounts) => {
//         println!("\nMounts:");
//         for mount in mounts.iter() {
//             println!("{} ---{}---> {} (available {} of {})",
//                      mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
//         }
//     }
//     Err(x) => println!("\nMounts: error: {}", x)
// }

// match sys.networks() {
//     Ok(netifs) => {
//         println!("\nNetworks:");
//         for netif in netifs.values() {
//             println!("{} ({:?})", netif.name, netif.addrs);
//         }
//     }
//     Err(x) => println!("\nNetworks: error: {}", x)
// }

// match sys.networks() {
//     Ok(netifs) => {
//         println!("\nNetwork interface statistics:");
//         for netif in netifs.values() {
//             println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
//         }
//     }
//     Err(x) => println!("\nNetworks: error: {}", x)
// }

// match sys.battery_life() {
//     Ok(battery) =>
//         print!("\nBattery: {}%, {}h{}m remaining",
//                battery.remaining_capacity*100.0,
//                battery.remaining_time.as_secs() / 3600,
//                battery.remaining_time.as_secs() % 60),
//     Err(x) => print!("\nBattery: error: {}", x)
// }

// match sys.on_ac_power() {
//     Ok(power) => println!(", AC power: {}", power),
//     Err(x) => println!(", AC power: error: {}", x)
// }

// match sys.memory() {
//     Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
//     Err(x) => println!("\nMemory: error: {}", x)
// }

// match sys.load_average() {
//     Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
//     Err(x) => println!("\nLoad average: error: {}", x)
// }

// match sys.boot_time() {
//     Ok(boot_time) => println!("\nBoot time: {}", boot_time),
//     Err(x) => println!("\nBoot time: error: {}", x)
// }

// match sys.cpu_load_aggregate() {
//     Ok(cpu)=> {
//         println!("\nMeasuring CPU load...");
//         thread::sleep(Duration::from_secs(1));
//         let cpu = cpu.done().unwrap();
//         println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
//             cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
//     },
//     Err(x) => println!("\nCPU load: error: {}", x)
// }

// match sys.cpu_temp() {
//     Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
//     Err(x) => println!("\nCPU temp: {}", x)
// }

// match sys.socket_stats() {
//     Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
//     Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
// }

// }
