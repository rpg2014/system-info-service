#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket_cors::{AllowedOrigins};
use rocket::http::Method;
use clap::Clap;
mod system_wrapper;

#[derive(Clap)]
struct Opts {
    #[clap(short)]
    debug: bool,
}


fn main() {
    let opts: Opts = Opts::parse();
    let mut allowed_origins = AllowedOrigins::some_regex(&["^http://192.168.0.[0-9]{3}:*"]);
    if opts.debug {
        allowed_origins = AllowedOrigins::all();
    }
    // let allowed_origins = AllowedOrigins::all();//
    

    // You can also deserialize this
    let cors_result = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        // allow_credentials: true,
        send_wildcard: true,
        fairing_route_base: "/".to_string(),
        ..Default::default()
    }
    .to_cors();

    let cors = match cors_result{
        Ok(cors) => cors,
        Err(e) => panic!("Cors is fucc: {}", e)
    };


    rocket::ignite()
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
        .attach(cors)
        .launch();
}
