use rocket::http::Status;

#[get("/health_check")]
pub fn health_check() -> Status {
    Status::Ok
}