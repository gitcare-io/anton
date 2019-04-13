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
use std::env;

pub mod application;
pub mod domain;
pub mod infrastructure;

fn main() {
    load_config();
    application::event::register::register_events();

    rocket::ignite()
        .mount("/c/", infrastructure::api::pull_request_routes())
        .launch();
}

pub fn load_config() -> () {
    let env = env::var("ENV");
    match env {
        Ok(env) => match &env[..] {
            "staging" => dotenv::from_path("config/.env-staging").ok(),
            "production" => dotenv::from_path("config/.env-production").ok(),
            _ => dotenv::from_path("config/.env-development").ok(),
        },
        _ => dotenv::from_path("config/.env-development").ok(),
    };
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
