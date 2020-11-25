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
            client_secret: self.client_secret.clone(),
            code: code.to_string(),
            redirect_uri: self.redirect_url.clone(),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub id_token: String,
    pub token_type: String,
}