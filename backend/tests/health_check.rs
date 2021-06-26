#[macro_use]
extern crate diesel_migrations;

use backend::auth::auth0::AuthParameters;
use backend::configuration::{get_configuration, DatabaseSettings};
use backend::rocket;
use diesel::Connection;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel_migrations::embed_migrations;
use rocket::config::Value;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};
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
    _pg_connection: String,
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
            _pg_connection: db_uri,
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

    // Crawl to google sign-in button element
    // and simulate click
    let google_button = driver.find_element(By::ClassName("auth0-lock-social-button"))?;
    google_button.click()?;
    sleep(delay);

    // Crawl to email input element, populate text box with user email address.
    // Crawl to next button and simulate click
    let email_text = driver.find_element(By::Id("identifierId"))?;
    email_text.send_keys(test_email)?;
    let button_container = driver.find_element(By::ClassName("qhFLie"))?;
    let next_button = button_container.find_element(By::Css("button[type='button']"))?;
    next_button.click()?;
    sleep(delay);

    // Crawl to user password input element, populate text box with user password.
    // Crawl to submit button and simulate click
    let elem_password = driver.find_element(By::Id("password"))?;
    let password_text = elem_password.find_element(By::Css("input[type='password']"))?;
    password_text.send_keys(test_password)?;
    let button_container = driver.find_element(By::ClassName("qhFLie"))?;
    let next_button = button_container.find_element(By::Css("button[type='button']"))?;
    let click_result = next_button.click();
    sleep(delay);
    match click_result {
        Ok(val) => println!("result is success: {:?}", val),
        Err(err) => println!("result is error: {:?}", err),
    }

    //TODO: Find solution to incorporating code below for situation
    // where user isn't previously authorized
    // sleep(delay);
    // let allow_button = driver.find_element(By::Id("allow"))?;
    // let click_result = allow_button.click();
    // match click_result {
    //     Ok(val) => println!("result is success: {:?}", val),
    //     Err(err) => println!("result is error: {:?}", err),
    // }

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

// #[test]
// fn get_skillblocks_returns_401_if_user_guard_fails() {
//     let app = spawn_app();

//     let req = app.client.get("/api/skillblocks");
//     let response = req.dispatch();

//     assert_eq!(response.status(), Status::Unauthorized);
// }

#[test]
fn get_skillblocks_returns_404_if_key_not_found() {
    let app = spawn_app();
    let _config_result = configure_testuser(&app);

    let req = app.client.get("/api/skillblocks");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

// #[test]
// fn new_skillblocks_successfully_returns_303() {
//     let app = spawn_app();
//     let _config_result = configure_testuser(&app);
//     let response = create_mock_skillblock(&app);

//     assert_eq!(response.status(), Status::SeeOther);
// }

// #[test]
// fn new_skillblock_returns_401_if_user_not_logged_in() {
//     let app = spawn_app();

//     let req = app.client.post("/api/new_skillblock");
//     let response = req.dispatch();

//     assert_eq!(response.status(), Status::Unauthorized);
// }

// #[test]
// fn health_check_returns_200() {
//     let app = spawn_app();

//     let req = app.client.get("/health_check");
//     let response = req.dispatch();

//     assert_eq!(response.status(), Status::Ok)
// }

// #[test]
// fn process_login_successfully_returns_303() {
//     let app = spawn_app();
//     let config_result = configure_testuser(&app).unwrap();

//     assert_eq!(config_result.status(), Status::SeeOther);
// }

// #[test]
// fn process_logout_successfully_returns_303() {
//     let app = spawn_app();
//     let _config_result = configure_testuser(&app);

//     let req = app.client.get("/logout");
//     let response = req.dispatch();

//     assert_eq!(response.status(), Status::SeeOther);
// }

// #[test]
// fn test_github_actions() {
//     let app = spawn_app();
//     let mut caps = DesiredCapabilities::firefox();
//     caps.set_headless().unwrap();
//     let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();
//     driver.get("https://www.google.com").unwrap();
//     let google_title = driver.title().unwrap();

//     assert_eq!(google_title, "Google");
// }
