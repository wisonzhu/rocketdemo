use rocket::get;
use crate::controllers::hello_controller;

#[get("/hello")]
pub fn hello() -> &'static str {
    hello_controller::get_hello()
}