#[macro_use] extern crate rocket;
mod controllers;
mod services;
mod routes;
use crate::routes::hello_route::hello;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}









