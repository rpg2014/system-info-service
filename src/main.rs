#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hhmmss::Hhmmss;
use rocket::response::status::NotFound;
mod system_wrapper;
use rocket_contrib::json::Json;

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

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                uptime_handler,
                load_average_handler,
                networks_handler
            ],
        )
        .launch();
}
