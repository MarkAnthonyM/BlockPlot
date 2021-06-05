use backend::rocket;

use rocket::http::Status;
use rocket::local::Client;

fn spawn_app() -> Client {
    let rocket = rocket();
    let client = Client::new(rocket).expect("valid rocket instance");

    client
}

#[test]
fn health_check_returns_200() {
    let client = spawn_app();

    let req = client.get("/health_check");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok)
}