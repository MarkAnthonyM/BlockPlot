use crate::auth::auth0::{
    build_random_state, decode_and_validate, get_or_create_user, AuthParameters, Session,
    SessionDB, TokenResponse,
};
use crate::db::operations::{BlockplotDbConn, update_user_login_timestamp};

use rocket::http::{Cookie, Cookies, Status};
use rocket::response::Redirect;
use rocket::State;

use uuid::Uuid;

// Route redirects to auth0 login page. Redirection link is built from
// AuthParameters instance that is managed by rocket application State
#[get("/auth0")]
pub fn auth0_login(
    mut cookies: Cookies,
    settings: State<AuthParameters>,
) -> Result<Redirect, Status> {
    let state_code = build_random_state();
    cookies.add(Cookie::new("state", state_code.clone()));

    let auth0_uri = settings.build_authorize_url(&state_code);

    Ok(Redirect::to(auth0_uri))
}

// Route for testing authentication routine
#[get("/process?<code>&<state>")]
pub fn process_login(
    session_db: State<SessionDB>,
    code: String,
    mut cookies: Cookies,
    conn: BlockplotDbConn,
    state: String,
    settings: State<AuthParameters>,
) -> Result<Redirect, Status> {
    if let Some(cookie) = cookies.get("state") {
        if state != cookie.value() {
            return Err(Status::Forbidden);
        }
    } else {
        return Err(Status::BadRequest);
    }
    cookies.remove(Cookie::named("state"));

    let token_parameters = settings.build_token_request(&code);
    let serialized = serde_json::to_string(&token_parameters).unwrap();
    let token_url = format!("https://{}/oauth/token", settings.auth0_domain);

    let client = reqwest::blocking::Client::new();
    let token_response: TokenResponse = client
        .post(&token_url)
        .header("content-type", "application/json")
        .body(serialized)
        .send()
        .unwrap()
        .json()
        .expect("Error with token request");

    let token_payload = decode_and_validate(
        settings.client_id.as_str(),
        settings.auth0_domain.as_str(),
        token_response.id_token.as_str(),
        settings.client_secret.as_str(),
    )
    .map_err(|_| Status::Unauthorized)
    .unwrap();

    let user =
        get_or_create_user(&conn, &token_payload).map_err(|_| Status::InternalServerError)?;
    
    let user_id = user.auth_id.clone();

    let new_session = Session {
        block_count: user.block_count,
        email: token_payload.claims.email,
        expires: token_payload.claims.exp,
        given_name: token_payload.claims.given_name,
        key_present: user.key_present,
        nickname: token_payload.claims.nickname,
        picture: token_payload.claims.picture,
        user_id: user.auth_id,
    };

    let session_token = Uuid::new_v4().to_string();

    session_db
        .0
        .insert(session_token.to_string(), Some(new_session));

    let cookie = Cookie::build("session", session_token)
        .path("/")
        // Remember to change this to true for production
        .secure(false)
        .http_only(true)
        .finish();
    cookies.add(cookie);

    update_user_login_timestamp(&conn, user_id).map_err(|_| Status::InternalServerError).unwrap();

    Ok(Redirect::to(format!("http://localhost:8080/user")))
}

// Route logs user out by retriving session_id cookie,
// uses cookie to destory session record,
// and removes session cookie.
// Redirect is then made to auth0 logout api endpoint,
// which logs user out of auth0 service and redirects
// to blockplot homepage.
#[get("/logout")]
pub fn process_logout(
    mut cookies: Cookies,
    session_db: State<SessionDB>,
    settings: State<AuthParameters>,
) -> Redirect {
    let session_id: Option<String> = cookies
        .get("session")
        .and_then(|cookie| cookie.value().parse().ok());
    if let Some(id) = session_id {
        session_db.0.remove(&id);
    }
    cookies.remove(Cookie::named("session"));

    let return_url = format!("http://localhost:8080/index");
    let logout_request = format!(
        "https://{}/v2/logout?client_id={}&returnTo={}",
        settings.auth0_domain, settings.client_id, return_url
    );

    Redirect::to(logout_request)
}
