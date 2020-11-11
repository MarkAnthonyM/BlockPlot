#![recursion_limit = "512"]

mod api;
mod pages;
mod types;

use pages::Form;

fn main() {
    yew::start_app::<Form>();
}