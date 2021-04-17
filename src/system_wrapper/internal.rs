use crate::system_wrapper::types::*;
use rocket::response::status;
use rocket::response::status::{BadRequest, NotFound};
use rocket_contrib::json::Json;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, NetworkAddrs, Platform, System};

pub fn get_uptime() -> Result<Duration, NotFound<Json<String>>> {
    let sys = System::new();
    sys.uptime()
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))
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
}

pub fn get_networks_stats() -> Result<Vec<NetworkStats>, NotFound<Json<String>>> {
    let sys = System::new();

    let netifs = sys
        .networks()
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))?;
    let mut result = Vec::new();
    for netif in netifs.values() {
        let net_stats = sys
            .network_stats(&netif.name)
            .or_else(|e| Err(status::NotFound(Json(e.to_string()))))?;
        result.push(NetworkStats {
            network_name: netif.name.clone(),
            rx_bytes: net_stats.rx_bytes.as_u64(),
            tx_bytes: net_stats.tx_bytes.as_u64(),
            rx_packets: net_stats.rx_packets,
            tx_packets: net_stats.tx_packets,
            rx_errors: net_stats.rx_errors,
            tx_errors: net_stats.tx_errors,
        })
    }
    Ok(result)
}
pub fn get_network_stats(name: String) -> Result<NetworkStats, NotFound<Json<String>>> {
    let sys = System::new();
    let net_stats = sys
        .network_stats(&name)
        .or_else(|e| Err(status::NotFound(Json(e.to_string()))))?;

    Ok(NetworkStats {
        network_name: name,
        rx_bytes: net_stats.rx_bytes.as_u64(),
        tx_bytes: net_stats.tx_bytes.as_u64(),
        rx_packets: net_stats.rx_packets,
        tx_packets: net_stats.tx_packets,
        rx_errors: net_stats.rx_errors,
        tx_errors: net_stats.tx_errors,
    })
}

pub fn get_cpu_temp() -> Result<f32, BadRequest<Json<String>>> {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => Ok(cpu_temp),
        Err(x) => Err(BadRequest(Some(Json(x.to_string())))),
    }
}

pub fn get_memory() -> Result<Memory, BadRequest<Json<String>>> {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => Ok(Memory {
            used: saturating_sub_bytes(mem.total, mem.free).as_u64(),
            total: mem.total.as_u64(),
            free: mem.free.as_u64(),
        }),
        Err(x) => Err(BadRequest(Some(Json(x.to_string())))),
    }
}

pub fn get_drives() -> Result<Vec<Filesystem>, BadRequest<Json<String>>> {
    let sys = System::new();

    match sys.mounts() {
        Ok(mounts) => {
            let mut result = Vec::new();
            for mount in mounts.iter() {
                result.push(Filesystem {
                    fs_mounted_from: mount.fs_mounted_from.clone(),
                    fs_type: mount.fs_type.clone(),
                    fs_mounted_on: mount.fs_mounted_on.clone(),
                    free: mount.free.as_u64(),
                    avail: mount.avail.as_u64(),
                    total: mount.total.as_u64(),
                    name_max: mount.name_max,
                    files: mount.files,
                    files_total: mount.files_total,
                    files_avail: mount.files_avail,
                })
            }
            Ok(result)
        }
        Err(x) => Err(BadRequest(Some(Json(x.to_string())))),
    }
}
pub fn get_hostname() -> Result<String, BadRequest<Json<String>>> {
    let os_string = match hostname::get() {
        Ok(name) => Ok(name),
        Err(e) => Err(BadRequest(Some(Json(e.to_string())))),
    }?;

    match os_string.into_string() {
        Ok(name) => Ok(name),
        Err(_e) => Err(BadRequest(Some(Json("Unable to get hostname".to_string())))),
    }
}

pub fn get_cpu_average() -> Result<CPULoad, io::Error> {
    let sys = System::new();
    let future = sys.cpu_load_aggregate()?;
    sleep(Duration::from_secs(1));
    let cpu = future.done()?;
    return Ok(CPULoad {
        user: cpu.user,
        nice: cpu.nice,
        system: cpu.system,
        interrupt: cpu.interrupt,
        idle: cpu.idle,
    });

    //     Ok(cpu)=> {
    //         println!("\nMeasuring CPU load...");
    //         thread::sleep(Duration::from_secs(1));
    //         let cpu = cpu.done().unwrap();
    //         println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
    //             cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
    //     },
    //     Err(x) => println!("\nCPU load: error: {}", x)
    // }
}
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

// match sys.socket_stats() {
//     Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
//     Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
// }

// }
