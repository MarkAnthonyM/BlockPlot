use backend::configuration::{get_configuration, DatabaseSettings};
use backend::rocket;
use diesel::Connection;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::local::Client;
use std::net::TcpListener;
use uuid::Uuid;

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
        
        Self {
            address,
            base_url: postgres_url,
            client,
            db_name: config.database_name.clone(),
            pg_connection: config.with_db(),
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
    configuration.database.database_name = Uuid::new_v4().to_string();

    let rocket = rocket(true, Some(listener));
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