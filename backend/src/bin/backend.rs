#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

use rusty_rescuetime;

// Rocket connection pool
#[database("postgres_blockplot")]
struct BlockplotDbConn(diesel::PgConnection);

fn main() {
    println!("Hello, World!");
}