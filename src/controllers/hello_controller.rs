use crate::services::hello_service;

pub fn get_hello() -> &'static str {
    hello_service::get_hello_message()
}