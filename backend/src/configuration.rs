use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> String {
        let db_uri = format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.username,
            self.password,
            self.host,
            self.port,
        );

        db_uri
    }

    pub fn with_db(&self) -> String {
        let db_uri = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        );

        db_uri
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize configuration reader
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Parse and store values from configuration files
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    settings.merge(config::File::from(configuration_directory.join("local")).required(true))?;

    settings.try_into()
}