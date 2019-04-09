#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

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
extern crate chrono;
extern crate chrono_tz;

pub mod application;
pub mod domain;
pub mod infrastructure;

fn main() {
    dotenv::dotenv().ok();

    application::event::register::register_events();

    rocket::ignite()
        .mount("/c/", infrastructure::api::pull_request_routes())
        .launch();
}
