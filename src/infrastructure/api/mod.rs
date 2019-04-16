pub mod controller;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // commands
        controller::command::pull_request_controller::open,
        controller::command::pull_request_controller::assign,
        controller::command::pull_request_controller::close,
        //queries
        controller::query::dmr_controller::index,
    ]
}
