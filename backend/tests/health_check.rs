#[macro_use]
extern crate diesel_migrations;

use backend::auth::auth0::{AuthParameters, SessionDB};
use backend::configuration::{get_configuration, DatabaseSettings};
use backend::db::models::User;
use backend::db::operations::query_user;
use backend::rocket;
use diesel::Connection;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel_migrations::embed_migrations;
use rocket::config::Value;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
use rocket::Rocket;
use rocket::State;
use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use thirtyfour_sync::prelude::*;
use uuid::Uuid;

embed_migrations!("../migrations/");

// Test app context
//TODO: Evaluate if address/pg_connection struct fields are neccessary
struct TestApp {
    _address: String,
    base_url: String,
    client: Client,
    db_name: String,
    pg_connection: String,
}

impl TestApp {
    // Create new database for calling app context
    fn new(_address: String, config: &DatabaseSettings, client: Client) -> Self {
        let postgres_url = config.without_db();
        let db_uri = config.with_db();
        Self {
            _address,
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
        let conn =
            PgConnection::establish(&self.base_url).expect("Cannot connect to postgres database.");

        let disconnect_users = format!(
            r#"SELECT pg_terminate_backend(pg_stat_activity.pid) FROM pg_stat_activity WHERE pg_stat_activity.datname = '{}';"#,
            self.db_name
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&conn)
            .unwrap();

        let query = diesel::sql_query(format!(r#"DROP DATABASE "{}";"#, self.db_name).as_str());
        query
            .execute(&conn)
            .expect(&format!("Couldn't drop database {}", self.db_name));
    }
}

fn configure_database(config: &DatabaseSettings) {
    // Connect to default database
    let postgres_url = config.without_db();
    let conn =
        PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

    // Create new database for testing
    let query =
        diesel::sql_query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str());
    query
        .execute(&conn)
        .expect(format!("Could not create database {}", config.database_name).as_str());

    // Migrate test database
    let db_uri = config.with_db();
    let conn = PgConnection::establish(&db_uri).expect(&format!(
        "Cannot connect to {} database",
        config.database_name
    ));

    let migration_result = embedded_migrations::run(&conn);
    match migration_result {
        Ok(_) => println!("Migration successful!"),
        Err(error) => println!("Error migrating database: {}", error),
    }
}

// Configure and store new testuser in database
fn configure_testuser(app: &TestApp) -> WebDriverResult<LocalResponse> {
    use dotenv::dotenv;
    dotenv().ok();

    // Grab testing login credentials
    let test_email = std::env::var("TESTEMAIL").unwrap();
    let test_password = std::env::var("TESTPASSWORD").unwrap();

    // Build auth0 authorization uri using state code.
    // State code retrived from cookie created by /auth0 endpoint
    let req = app.client.get("/auth0");
    let response = req.dispatch();
    let cookies = response.cookies();
    let state_code = cookies[0].value();
    let rocket_instance = app.client.rocket();
    let app_state: Option<rocket::State<AuthParameters>> = State::from(rocket_instance);
    let auth_uri;
    match app_state {
        Some(state) => {
            auth_uri = state.build_authorize_url(state_code);
        }
        None => {
            panic!("App state not found!");
        }
    }

    // Create selenium browser session using gecko as webdriver
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", &caps)?;

    // Navigate to auth0 authorization login/signup page.
    // Delays necessary to give webpage dom elements enough time
    // to load up properly
    driver.get(&auth_uri)?;
    let delay = Duration::new(2, 0);
    sleep(delay);

    // Crawl to email input element, populate text box with user email address.
    let email_text = driver.find_element(By::Id("1-email"))?;
    email_text.send_keys(test_email)?;

    // Crawl to user password input element, populate text box with user password.
    // Crawl to login button and simulate click
    let password_text = driver.find_element(By::Css("input[type='password']"))?;
    password_text.send_keys(test_password)?;
    let login_button = driver.find_element(By::ClassName("auth0-lock-submit"))?;
    let click_result = login_button.click();
    sleep(delay);

    // TODO: Find solution to incorporating code below for situation
    // where user isn't previously authorized
    // let allow_button = driver.find_element(By::Id("allow"))?;
    // let click_result = allow_button.click();

    match click_result {
        Ok(val) => println!("result is success: {:?}", val),
        Err(err) => println!("result is error: {:?}", err),
    }

    // Grab callback url returned by authorized login.
    // Retrive response_code and state_code query parameters.
    // Build process_login endpoint url
    let callback_url = driver.current_url()?;
    let split_string: Vec<&str> = callback_url.split("?").collect();
    let parameters = split_string[1].to_string();
    let process_url = format!("/process?{}", parameters);

    // Hit process_login endpoint, creating test user
    // and storing in database.
    let req = app.client.get(process_url);
    let response = req.dispatch();

    Ok(response)
}

// Generate testing skillblock and store in database
fn create_mock_skillblock(app: &TestApp) -> LocalResponse {
    use dotenv::dotenv;
    dotenv().ok();

    // Create form data
    let rescuetime_api_key = std::env::var("RESCUETIME_API_KEY").unwrap();
    let _config_result = configure_testuser(&app);
    let mock_form_data = format!(
        "api_key={}&category=software%20&offline_category=false&description=Programming%20skillblock&skill_name=Programming",
        rescuetime_api_key
    );

    // Dispatch post request using mock form data
    let req = app
        .client
        .post("/api/new_skillblock")
        .body(mock_form_data)
        .header(ContentType::Form);
    let response = req.dispatch();

    response
}

// Return User struct from postgres database
fn retrieve_user(app: &TestApp, response: LocalResponse, rocket_instance: &Rocket) -> Option<User> {
    let session_state: Option<State<SessionDB>> = State::from(rocket_instance);
    let cookies = response.cookies();
    let session_cookie = cookies.into_iter().find(|x| x.name() == "session");
    let conn =
        PgConnection::establish(&app.pg_connection).expect("Error connecting to postgres database");

    // Retrieve user id from in memory session database
    let user_id = match session_cookie {
        Some(session) => {
            let session_token = session.value();
            let session_db = session_state.unwrap();
            let session_map = session_db.0.get(session_token).unwrap();
            match *session_map {
                Some(ref session) => session.user_id.to_string(),
                None => String::from("User id not found in session database"),
            }
        }
        None => String::from("Session cookie not found"),
    };

    // Retrieve user from postgres database
    let pg_user = query_user(&conn, user_id);

    pg_user
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
    configure_database(&configuration.database);

    let rocket = rocket(true, Some(listener), Some(databases));
    let client = Client::new(rocket).expect("valid rocket instance");

    // Instantiate test app context
    let app = TestApp::new(address, &configuration.database, client);

    app
}

#[test]
fn get_skillblocks_returns_401_if_user_guard_fails() {
    let app = spawn_app();

    let req = app.client.get("/api/skillblocks");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn get_skillblocks_returns_404_if_key_not_found() {
    let app = spawn_app();
    let _config_result = configure_testuser(&app);

    let req = app.client.get("/api/skillblocks");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn new_skillblocks_successfully_returns_303() {
    let app = spawn_app();
    let config_result = configure_testuser(&app).unwrap();
    let rocket_instance = app.client.rocket();

    // Hit "/api/new_skillblock" endpoint
    let response = create_mock_skillblock(&app);

    // Retrieve user record from database
    // Store number of skillblocks associated with test user
    let user = retrieve_user(&app, config_result, rocket_instance).unwrap();
    let block_count = user.block_count;

    // Check database for correct block count value
    assert_eq!(block_count, 1);
    assert_eq!(response.status(), Status::SeeOther);
}

#[test]
fn new_skillblock_returns_401_if_user_not_logged_in() {
    let app = spawn_app();

    let req = app.client.post("/api/new_skillblock");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn health_check_returns_200() {
    let app = spawn_app();

    let req = app.client.get("/health_check");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok)
}

#[test]
fn process_login_successfully_stores_user_and_returns_303() {
    // Arrange
    let app = spawn_app();
    let config_result = configure_testuser(&app).unwrap();
    let response_status = config_result.status();
    let rocket_instance = app.client.rocket();

    // Retrieve user from postgres database
    let pg_user = retrieve_user(&app, config_result, rocket_instance);

    // Assert
    assert_eq!(pg_user.is_some(), true);
    assert_eq!(response_status, Status::SeeOther);
}

#[test]
fn process_logout_successfully_returns_303() {
    let app = spawn_app();
    let _config_result = configure_testuser(&app);

    let req = app.client.get("/logout");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::SeeOther);
}
