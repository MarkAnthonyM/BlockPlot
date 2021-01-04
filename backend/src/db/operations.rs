use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::select;
use diesel::dsl::exists;
use rocket_contrib::database;
use super::{ models, schema };

// Rocket connection pool
#[database("postgres_blockplot")]
pub struct BlockplotDbConn(diesel::PgConnection);

// prototype skillblock check operation. Might be unneeded
pub fn check_for_skillblock(conn: &PgConnection, cate: String) -> Result<bool, ()> {
    use self::schema::skillblocks::dsl::*;

    let skillblock_exist = select(
        exists(skillblocks.filter(category.eq(cate)))
    ).get_result(conn);
    match skillblock_exist {
        Ok(exists) => {
            Ok(exists)
        },
        Err(_erorr) => {
            Err(())
        }
    }
}

// Insert skillblock record into database
pub fn create_skillblock(connection: &PgConnection, db_struct: models::NewSkillblock) {
    diesel::insert_into(schema::skillblocks::table)
        .values(&db_struct)
        .execute(connection)
        .expect("Error inserting into database");
}

// Insert user record into database
pub fn create_user(connection: &PgConnection, new_user: models::NewUser) -> Result<models::User, diesel::result::Error> {
    let inserted_user = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error inserting user into database");

    Ok(inserted_user)
}

// Prototype Delete time_date operation. Probably need to change how
// date_time records are fetched from db
pub fn delete_date_time(connection: &PgConnection, dt_id: i32) -> usize {
    use self::schema::date_times::dsl::*;

    let target = diesel
        ::delete(date_times.find(dt_id))
        .execute(connection)
        .expect("Error deleting date time record");

    target
}

// Prototype date_time query operation
pub fn query_date_times(connection: &PgConnection, skillblock: &models::Skillblock) -> Result<Vec<models::DateTime>, diesel::result::Error> {
    let date_time_records = models
        ::DateTime
        ::belonging_to(skillblock)
        .load::<models::DateTime>(connection);

    date_time_records
}

// Query skillblock record from database
pub fn query_skillblocks(connection: &PgConnection, user: &models::User) -> Result<Vec<models::Skillblock>, diesel::result::Error> {
    let skillblock_records = models
        ::Skillblock
        ::belonging_to(user)
        .load::<models::Skillblock>(connection);
    
        skillblock_records
}

// Query user record from database
pub fn query_user(connection: &PgConnection, id: String) -> Option<models::User> {
    use self::schema::users::dsl::*;
    
    let user = users.filter(auth_id.eq(id))
        .first::<models::User>(connection);

    match user {
        Ok(record) => Some(record),
        Err(_) => None
    }
}

// Prototype add date_time query
pub fn add_date_time(connection: &PgConnection, date_time: models::NewDateTime) -> Result<usize, diesel::result::Error> {
    let result = diesel::insert_into(schema::date_times::table)
        .values(&date_time)
        .execute(connection)
        .expect("Error inserting date_time into database");
    
    Ok(result)
}

// Prototype function adds whole vector of date times in a single query
pub fn batch_add_date_times(connection: &PgConnection, date_data: &Vec<models::NewDateTime>) -> Result<usize, Error> {
    use schema::date_times::dsl::*;
    
    let result = diesel::insert_into(date_times)
        .values(date_data)
        .execute(connection);

    result
}

// Prototype update query
pub fn add_user_key(connection: &PgConnection, id: String, key: &String) -> Result<(usize, usize), diesel::result::Error> {
    use self::schema::users::dsl::*;

    let target = users.filter(auth_id.eq(&id));
    let key_result = diesel::update(target).set(api_key.eq(key)).execute(connection)?;
    let target = users.filter(auth_id.eq(id));
    let bool_result = diesel::update(target).set(key_present.eq(true)).execute(connection)?;

    Ok((key_result, bool_result))
}

// Prototype block_count update query
pub fn update_block_count(connection: &PgConnection, count: i32, id: String) -> Result<usize, diesel::result::Error> {
    use self::schema::users::dsl::*;
    let increase_count = count + 1;

    let target = users.filter(auth_id.eq(&id));
    let result = diesel::update(target)
        .set(block_count
            .eq(increase_count))
            .execute(connection)?;
    
    Ok(result)
}

// Prototype date_time update query
pub fn update_date_time(connection: &PgConnection, date: NaiveDateTime, time: i32) -> Result<usize, diesel::result::Error> {
    use self::schema::date_times::dsl::*;

    let target = date_times.filter(day_date.eq(date));

    let result = diesel::update(target)
        .set(day_time.eq(time)).execute(connection);

    result
}