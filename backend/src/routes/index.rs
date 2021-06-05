use crate::auth::auth0::Session;

use rocket::http::Status;
use rocket::request::FlashMessage;

use rocket_contrib::json::Json;

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Status {
    flash.map(|msg| format!("{}: {}", msg.name(), msg.msg()))
        .unwrap_or_else(|| "Welcome!".to_string());
    
    Status::Unauthorized
}

// Prototype route
#[get("/home")]
fn home(session: Session) -> Result<Json<Session>, Status> {
    Ok(Json(session))
}