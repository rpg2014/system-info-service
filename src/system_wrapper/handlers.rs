use crate::system_wrapper::internal::{
    get_cpu_average, get_cpu_temp, get_drives, get_hostname, get_load_average, get_memory,
    get_network_stats, get_networks, get_networks_stats, get_uptime,
};
use crate::system_wrapper::types::{
    CPULoad, Filesystem, LoadAverageCopy, Memory, NetworkResult, NetworkStatsResults,
};
use hhmmss::Hhmmss;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use std::io;

#[get("/uptime")]
pub fn uptime_handler() -> Result<Json<String>, NotFound<Json<String>>> {
    Ok(Json(get_uptime()?.hhmmss()))
}

#[get("/load_average")]
pub fn load_average_handler() -> Result<Json<LoadAverageCopy>, NotFound<Json<String>>> {
    Ok(Json(get_load_average()?))
}
#[get("/networks")]
pub fn networks_handler() -> Result<Json<NetworkResult>, NotFound<Json<String>>> {
    Ok(Json(get_networks()?))
}

#[get("/net_stats?<name>")]
pub fn net_stats_handler(
    name: Option<String>,
) -> Result<Json<NetworkStatsResults>, NotFound<Json<String>>> {
    match name {
        Some(name) => Ok(Json(NetworkStatsResults::One(get_network_stats(name)?))),
        None => Ok(Json(NetworkStatsResults::List(get_networks_stats()?))),
    }
}

#[get("/cpu_temp")]
pub fn cpu_temp_handler() -> Result<Json<f32>, BadRequest<Json<String>>> {
    Ok(Json(get_cpu_temp()?))
}

#[get("/memory")]
pub fn memory_handler() -> Result<Json<Memory>, BadRequest<Json<String>>> {
    Ok(Json(get_memory()?))
}
#[get("/hostname")]
pub fn hostname_handler() -> Result<Json<String>, BadRequest<Json<String>>> {
    Ok(Json(get_hostname()?))
}

#[get("/disk_info")]
pub fn disk_handler() -> Result<Json<Vec<Filesystem>>, BadRequest<Json<String>>> {
    Ok(Json(get_drives()?))
}

#[get("/cpu_average")]
pub fn cpu_average() -> Result<Json<CPULoad>, io::Error> {
    Ok(Json(get_cpu_average()?))
}

// Phase 1: Health Check Endpoint
#[get("/health")]
pub fn health_check_handler() -> Json<crate::system_wrapper::types::HealthCheckResponse> {
    use chrono::Utc;
    
    let timestamp = Utc::now().to_rfc3339();
    let uptime = get_uptime().ok().map(|u| u.hhmmss());
    let hostname = get_hostname().ok();
    
    Json(crate::system_wrapper::types::HealthCheckResponse {
        status: "healthy".to_string(),
        timestamp,
        uptime,
        hostname,
    })
}

// Phase 1: Batch Endpoint - Get all system metrics in one request
#[get("/all")]
pub fn system_all_handler() -> Result<Json<crate::system_wrapper::types::SystemAllResponse>, BadRequest<Json<String>>> {
    use chrono::Utc;
    
    let timestamp = Utc::now().to_rfc3339();
    
    // Get required fields (hostname and uptime)
    let hostname = get_hostname()
        .map_err(|e| BadRequest(Json(format!("Failed to get hostname: {:?}", e))))?;
    
    let uptime = get_uptime()
        .map_err(|e| BadRequest(Json(format!("Failed to get uptime: {:?}", e))))?
        .hhmmss();
    
    // Get optional fields - if they fail, we'll return None instead of erroring
    let cpu_temp = get_cpu_temp().ok();
    let load_average = get_load_average().ok();
    let networks = get_networks().ok();
    let net_stats = get_networks_stats().ok().map(NetworkStatsResults::List);
    // let cpu_average = get_cpu_average().ok();
    
    Ok(Json(crate::system_wrapper::types::SystemAllResponse {
        timestamp,
        hostname,
        uptime,
        cpu_temp,
        load_average,
        networks,
        net_stats,
        // cpu_average,
    }))
}
