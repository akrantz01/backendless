mod authentication;
mod deployments;
mod projects;
mod users;
mod utils;

pub use authentication::init_routes as authentication;
pub use deployments::init_routes as deployments;
pub use projects::init_routes as projects;
pub use users::init_routes as users;
