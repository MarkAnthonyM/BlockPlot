use rocket::config::{ Config, ConfigError };

use std::env;

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
        let secret = env::var("CLIENT_SECRET").unwrap();

        let auth_parameters = Self {
            audience: String::from(config.get_str("audience")?),
            auth0_domain: String::from(config.get_str("auth0_domain")?),
            client_id: String::from(config.get_str("client_id")?),
            client_secret: secret,
            redirect_url: String::from(config.get_str("redirect_url")?),
        };

        Ok(auth_parameters)
    }
    
    pub fn build_authorize_url(&self) -> String {
        format!(
            "https://{}/authorize?audience={}&response_type=code&client_id={}&redirect_uri={}",
            self.auth0_domain,
            self.audience,
            self.client_id,
            self.redirect_url
        )
    }

    pub fn build_token_request(&self, code: &str) -> TokenRequest {
        TokenRequest {
            grant_type: String::from("authorization_code"),
            client_id: self.client_id.clone(),
            client_secrect: self.client_secret.clone(),
            code: code.to_string(),
            redirect_url: self.redirect_url.clone(),
        }
    }
}

// Contains data used as parameters for /oauth/token endpoint
#[derive(Deserialize, Serialize)]
pub struct TokenRequest {
    client_id: String,
    client_secrect: String,
    code: String,
    grant_type: String,
    redirect_url: String,
}