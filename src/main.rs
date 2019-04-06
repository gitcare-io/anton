#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[cfg(test)]
extern crate mocktopus;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
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
    dotenv::dotenv().ok();

    application::event::register::register_events();

    rocket::ignite()
        .mount("/c/", infrastructure::api::pull_request_routes())
        .launch();
}
