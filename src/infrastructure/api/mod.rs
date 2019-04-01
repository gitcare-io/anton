pub mod controller;

pub fn pull_request_routes() -> Vec<rocket::Route> {
    routes![
        controller::pull_request_controller::open,
        controller::pull_request_controller::assign,
        controller::pull_request_controller::close,
    ]
}
