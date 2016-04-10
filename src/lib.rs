#[macro_use]
extern crate diesel;

use std::env;

use diesel::*;
use diesel::sqlite::SqliteConnection;

mod models;
mod schema;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn foo() {
    use schema::users::dsl::*;
    use models::*;
    let idd: i64 = 5;
    users.filter(additional_id.eq(idd)).load::<User>(&establish_connection());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
