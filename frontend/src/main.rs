#![recursion_limit = "512"]

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

fn main() {
    yew::start_app::<app::App>();
}
