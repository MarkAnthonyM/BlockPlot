use anyhow::{ anyhow, Error };

use jsonwebtoken::{ Algorithm, DecodingKey, decode, TokenData, Validation };

use rocket::config::{ Config, ConfigError };
use rocket_contrib::databases::diesel;

use std::env;

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
    jwt: &str
) -> Result<TokenData<AccessToken>, Error> {
    let client = reqwest::blocking::Client::new();
    let jwks: Jwks = client
        .get(&format!("https://{}/.well-known/jwks.json", domain))
        .send()
        .unwrap()
        .json()
        .expect("Error fetching json web keys");
    
    // Decode jwt token with json web key set and validation algorithm
    let payload = decode::<AccessToken>(
        jwt,
        &DecodingKey::from_rsa_components(&jwks.keys[0].n, &jwks.keys[0].e),
        &Validation::new(Algorithm::RS256)
    ).unwrap();

    // Validate for correct audience
    if payload.claims.aud[0] != audience {
        return Err(anyhow!("Failure on audience validation"));
    }

    // Validate for correct auth domain
    if payload.claims.iss != format!("https://{}/", domain) {
        return Err(anyhow!("Failure on domain validation"));
    };

    Ok(payload)
}

pub fn get_or_create_user(db: &diesel::PgConnection, jwt: &TokenData<AccessToken>) {
    let user = "db user get function";
}

// Prototype
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    aud: Vec<String>,
    exp: i32,
    iss: String,
    sub: String,
}

// Prototype
#[derive(Debug, Deserialize)]
pub struct IdToken {
    email: String,
    given_name: String,
    nickname: String,
    picture: String,
    exp: i32,
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