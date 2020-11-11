#![recursion_limit = "256"]

mod api;
mod pages;
mod types;

use pages::Form;

fn main() {
    yew::start_app::<Form>();
}