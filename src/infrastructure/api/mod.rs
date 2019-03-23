pub mod controller;

pub fn pull_request_routes() -> Vec<rocket::Route> {
    routes![controller::pull_request_controller::open]
}
