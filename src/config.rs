use dotenv;
use std::env;
use std::env::VarError;

pub fn load() -> () {
    let env = env::var("ENV");
    let port = env::var("PORT");

    set_port(port);
    set_env_config(env);
}

fn set_port(port: Result<String, VarError>) -> () {
    match port {
        Ok(port) => env::set_var("ROCKET_PORT", port),
        _ => {}
    };
}

fn set_env_config(env: Result<String, VarError>) -> () {
    match env {
        Ok(env) => match &env[..] {
            "staging" => dotenv::from_path("config/.env-staging").ok(),
            "production" => dotenv::from_path("config/.env-production").ok(),
            _ => dotenv::from_path("config/.env-development").ok(),
        },
        _ => dotenv::from_path("config/.env-development").ok(),
    };
}
