use anyhow::{ anyhow, Error };
use chrono::Utc;
use crate::db::models::{ NewUser, User };
use crate::db::operations::{ create_user, query_user };

use dashmap::DashMap;

use jsonwebtoken::{ Algorithm, DecodingKey, decode, TokenData, Validation };

use rocket::config::{ Config, ConfigError };
use rocket_contrib::databases::diesel;
use rocket::request::{ FromRequest, Request, self };
use rocket::State;

pub fn build_random_state() -> String {
    use rand::{ distributions::Alphanumeric, thread_rng };
    use rand::Rng;
    use std::iter;
    
    let mut rng = thread_rng();

    let random: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect();
    
    random
}

// Store various parameters needed to build authorization link
// that fetches auth0 login page. Parameters are read from
// Rocket.toml configuration file
pub struct AuthParameters {
    pub audience: String,
    pub auth0_domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

impl AuthParameters {
    pub fn new(config: &Config) -> Result<AuthParameters, ConfigError> {
        // let secret = env::var("CLIENT_SECRET").unwrap();
        let secret = "Ib57BXDC-_Wqmszh1KQsDeZ23SQ64iRwxGELa6qkjE33-eIq0Xdzq2qv83hDWk1G";

        let auth_parameters = Self {
            audience: String::from(config.get_str("audience")?),
            auth0_domain: String::from(config.get_str("auth0_domain")?),
            client_id: String::from(config.get_str("client_id")?),
            client_secret: secret.to_string(),
            redirect_url: String::from(config.get_str("redirect_url")?),
        };

        Ok(auth_parameters)
    }
    
    pub fn build_authorize_url(&self, state: &str) -> String {
        format!(
            "https://{}/authorize?audience={}&response_type=code&client_id={}&redirect_uri={}&state={}",
            self.auth0_domain,
            self.audience,
            self.client_id,
            self.redirect_url,
            state
        )
    }

    pub fn build_token_request(&self, code: &str) -> TokenRequest {
        TokenRequest {
            grant_type: String::from("authorization_code"),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: code.to_string(),
            redirect_uri: self.redirect_url.clone(),
        }
    }
}

// Take json web token, decode using json web key set, and validate
// against proper audience/auth0 domain
pub fn decode_and_validate(
    audience: &str,
    domain: &str,
    jwt: &str,
    secret: &str,
) -> Result<TokenData<IdToken>, Error> {
    // Decode jwt token with json web key set and validation algorithm
    let payload = decode::<IdToken>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256)
    ).unwrap();

    // Validate for correct audience
    if payload.claims.aud != audience {
        return Err(anyhow!("Failure on audience validation"));
    }

    // Validate for correct auth domain
    if payload.claims.iss != format!("https://{}/", domain) {
        return Err(anyhow!("Failure on domain validation"));
    };

    Ok(payload)
}

// Get user record from data base using information from decoded jwt payload.
// If no user found, create and insert user into database using sub claim
// from jwt payload
pub fn get_or_create_user(db: &diesel::PgConnection, jwt_payload: &TokenData<IdToken>) -> Result<User, diesel::result::Error> {
    // Query database for user. Returns Option containing user struct if found.
    // Returns None if user not found
    let user = query_user(db, jwt_payload.claims.sub.to_string());

    // Returns user database information as a Result type
    // if user variable matches Some.
    // If user variable matches None, instantiates new user
    // and inserts into data base
    match user {
        Some(record) => {
            Ok(record)
        },
        None => {
            let new_user = NewUser {
                auth_id: jwt_payload.claims.sub.to_string(),
                api_key: None,
                key_present: false,
                block_count: 0,
            };

            let new_record = create_user(db, new_user);

            new_record
        }
    }
}

// Prototype
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub aud: Vec<String>,
    pub exp: i64,
    pub iss: String,
    pub sub: String,
}

// Prototype
#[derive(Debug, Deserialize)]
pub struct IdToken {
    pub aud: String,
    pub email: String,
    pub exp: i64,
    pub given_name: String,
    pub iss: String,
    pub nickname: String,
    pub picture: String,
    pub sub: String,
}

#[derive(Debug, Deserialize)]
struct Jwks {
    keys: Vec<Key>,
}

#[derive(Debug, Deserialize)]
struct Key {
    alg: String,
    kty: String,
    n: String,
    e: String,
    kid: String,
    x5t: String,
    x5c: Vec<String>,
}

pub struct SessionDB(pub DashMap<String, Option<Session>>);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub email: String,
    pub expires: i64,
    pub given_name: String,
    pub nickname: String,
    pub picture: String,
    pub user_id: String,
}

impl Session {
    pub fn session_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        self.expires <= now
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Session, ()> {
        let session_id: Option<String> = request
            .cookies()
            .get("session")
            .and_then(|cookie| cookie.value().parse().ok());
        if let Some(id) = session_id {
            let session_db = request.guard::<State<SessionDB>>().unwrap().inner();
            let session_map = session_db.0.get(&id).unwrap();
            match *session_map {
                Some(ref session) => {
                    return rocket::Outcome::Success(session.clone());
                },
                None => {
                    return rocket::Outcome::Forward(());
                }
            }
        } else {
            rocket::Outcome::Forward(())
        }
    }
}

// Contains data used as parameters for /oauth/token endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

// Contains data returned from call to /oauth/token endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub id_token: String,
    pub token_type: String,
}

// Prototype struct for user json object
#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub email: String,
    pub user_id: String,
}