use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::models::MTGCard;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL found in .env file");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create Pool")
}

// fn test_query() {
//     // Use this so that filtering and stuff is quicker
//     use crate::schema::mtg_cards::dsl::*;
//     let connection_pool = &mut establish_connection();
//     let connection = &mut connection_pool.get().unwrap();
//
//     let result = mtg_cards
//         .filter(foil.eq(false))
//         .limit(5)
//         .select(MTGCard::as_select())
//         .load(connection)
//         .expect("Query couldn't execute");
//         
//     for card in result {
//         println!("Title: {}", card.name);
//     }
// }
