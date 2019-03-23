#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate eventbus;
#[macro_use]
extern crate lazy_static;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod read_schema;
pub mod write_schema;

fn main() {
    infrastructure::event_bus::register::register_events();

    rocket::ignite()
        .mount("/c/", infrastructure::api::pull_request_routes())
        .launch();
}
