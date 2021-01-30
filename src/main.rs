#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod system_wrapper;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/system",
            routes![
                system_wrapper::handlers::uptime_handler,
                system_wrapper::handlers::load_average_handler,
                system_wrapper::handlers::networks_handler,
                system_wrapper::handlers::net_stats_handler,
                system_wrapper::handlers::cpu_temp_handler,
                system_wrapper::handlers::memory_handler,
                system_wrapper::handlers::disk_handler
            ],
        )
        .launch();
}
