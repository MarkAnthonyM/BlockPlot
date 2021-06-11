#[macro_use]
extern crate diesel_migrations;

use backend::configuration::{get_configuration, DatabaseSettings};
use backend::rocket;
use diesel::Connection;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel_migrations::embed_migrations;
use rocket::config::Value;
use rocket::http::Status;
use rocket::local::Client;
use std::collections::HashMap;
use std::net::TcpListener;
use uuid::Uuid;

embed_migrations!("../migrations/");

// Test app context
struct TestApp {
    address: String,
    base_url: String,
    client: Client,
    db_name: String,
    pg_connection: String,
}

impl TestApp {
    // Create new database for calling app context
    fn new(address: String, config: &DatabaseSettings, client: Client) -> Self {
        // Connect to default database
        let postgres_url = config.without_db();
        let conn = PgConnection::establish(&postgres_url)
            .expect("Cannot connect to postgres database.");

        // Create new database for testing
        let query = diesel::sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str());
        query
            .execute(&conn)
            .expect(format!("Could not create database {}", config.database_name).as_str());
        
        // Migrate test database
        let db_uri = config.with_db();
        let conn = PgConnection::establish(&db_uri)
            .expect(&format!("Cannot connect to {} database", config.database_name));
        
        embedded_migrations::run(&conn);
        
        Self {
            address,
            base_url: postgres_url,
            client,
            db_name: config.database_name.clone(),
            pg_connection: db_uri,
        }
    }
}

// Delete newly created databases for integrations tests at conclusion of tests
impl Drop for TestApp {
    fn drop(&mut self) {
        let conn = PgConnection::establish(&self.base_url).expect("Cannot connect to postgres database.");

        //TODO: Need to figure out how to force disconnect any active connections
        // let disconnect_users = format!(
        //     r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = "{}";"#,
        //     self.db_name
        // );

        // diesel::sql_query(disconnect_users.as_str())
        //     .execute(&conn)
        //     .unwrap();

        let query = diesel::sql_query(format!(r#"DROP DATABASE "{}";"#, self.db_name).as_str());
        query.execute(&conn).expect(&format!("Couldn't drop database {}", self.db_name));
    }
}

// Spawn testing application for integrations tests
fn spawn_app() -> TestApp {
    // Bind server to address using random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("127.0.0.1:{}", port);

    // Read from files in configuration folder and populate Settings struct
    let mut configuration = get_configuration().expect("Failed to read configuration");

    // Configure custom rocket db settings
    configuration.database.database_name = Uuid::new_v4().to_string();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let db_url = configuration.database.with_db();
    database_config.insert("url", Value::from(db_url));
    databases.insert("postgres_blockplot", Value::from(database_config));

    let rocket = rocket(true, Some(listener), Some(databases));
    let client = Client::new(rocket).expect("valid rocket instance");

    // Instantiate test app context
    let app = TestApp::new(address, &configuration.database, client);

    app
}

#[test]
fn health_check_returns_200() {
    let app = spawn_app();

    let req = app.client.get("/health_check");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok)
}