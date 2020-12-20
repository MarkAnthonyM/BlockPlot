use crate::auth::auth0::AccessToken;
use diesel::prelude::*;
use jsonwebtoken::TokenData;

use super::{ models, schema };

// Insert skillblock record into database
pub fn create_skillblock(connection: &PgConnection, db_struct: models::NewSkillblock) {
    diesel::insert_into(schema::skillblocks::table)
        .values(&db_struct)
        .execute(connection)
        .expect("Error inserting into database");
}

// Query skillblock record from database
pub fn query_skillblock(connection: &PgConnection) -> Vec<models::Skillblock> {
    schema::skillblocks::table.load::<models::Skillblock>(connection).expect("Error querying record")
}

pub fn query_user(connection: &PgConnection, payload: &TokenData<AccessToken>) -> Option<models::User> {
    use self::schema::users::dsl::*;
    
    let user = users.filter(auth_id.eq(payload.claims.sub.to_string()))
        .first::<models::User>(connection);

    match user {
        Ok(record) => Some(record),
        Err(_) => None
    }
}