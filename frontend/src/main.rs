#![recursion_limit = "512"]

mod api;
mod app;
mod pages;
mod route;
mod types;

use pages::Form;

fn main() {
    yew::start_app::<Form>();
}