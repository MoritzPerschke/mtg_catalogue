use actix_web::{web, HttpResponse, Responder};
use crate::db::connection::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardInfo {
    // this migth have to be changed according 
    // to what the scanner output ends up being
    gatherer_id: String
}

pub async fn add_card_to_collection(
    pool: web::Data<DbPool>,
    card_info: web::Json<CardInfo>,
) -> impl Responder{
    let _conn = &mut pool.get().expect("Couldn't get Database connection from pool");
    
    // Check if card is already in DB

    HttpResponse::Ok()
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
