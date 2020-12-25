use diesel::prelude::*;
use super::{ models, schema };

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

// Query skillblock record from database
pub fn query_skillblock(connection: &PgConnection) -> Vec<models::Skillblock> {
    schema::skillblocks::table.load::<models::Skillblock>(connection).expect("Error querying record")
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