use diesel::prelude::*;
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