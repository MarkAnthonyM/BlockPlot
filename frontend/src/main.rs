#![recursion_limit = "256"]

mod api;
mod pages;
mod types;

use pages::User;

fn main() {
    yew::start_app::<User>();
}