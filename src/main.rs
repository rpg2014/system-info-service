#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hhmmss::Hhmmss;
use rocket::response::status::{BadRequest, NotFound};
mod system_wrapper;
use rocket_contrib::json::Json;
use system_wrapper::{Filesystem, Memory, NetworkStatsResults};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/uptime")]
fn uptime_handler() -> Result<Json<String>, NotFound<Json<String>>> {
    Ok(Json(system_wrapper::get_uptime()?.hhmmss()))
}

#[get("/load_average")]
fn load_average_handler() -> Result<Json<system_wrapper::LoadAverageCopy>, NotFound<Json<String>>> {
    Ok(Json(system_wrapper::get_load_average()?))
}
#[get("/networks")]
fn networks_handler() -> Result<Json<system_wrapper::NetworkResult>, NotFound<Json<String>>> {
    Ok(Json(system_wrapper::get_networks()?))
}

#[get("/net_stats?<name>")]
fn net_stats_handler(
    name: Option<String>,
) -> Result<Json<system_wrapper::NetworkStatsResults>, NotFound<Json<String>>> {
    match name {
        Some(name) => Ok(Json(system_wrapper::NetworkStatsResults::One(
            system_wrapper::get_network_stats(name)?,
        ))),
        None => Ok(Json(NetworkStatsResults::List(
            system_wrapper::get_networks_stats()?,
        ))),
    }
}

#[get("/cpu_temp")]
fn cpu_temp_handler() -> Result<Json<f32>, BadRequest<Json<String>>> {
    Ok(Json(system_wrapper::get_cpu_temp()?))
}

#[get("/memory")]
fn memory_handler() -> Result<Json<Memory>, BadRequest<Json<String>>> {
    Ok(Json(system_wrapper::get_memory()?))
}

#[get("/disk_info")]
fn disk_handler() -> Result<Json<Vec<Filesystem>>, BadRequest<Json<String>>> {
    Ok(Json(system_wrapper::get_drives()?))
}

fn main() {
    rocket::ignite()
        .mount(
            "/system",
            routes![
                index,
                uptime_handler,
                load_average_handler,
                networks_handler,
                net_stats_handler,
                cpu_temp_handler,
                memory_handler,
                disk_handler
            ],
        )
        .launch();
}
