use actix_web::{get, web, HttpResponse, Responder};
use crate::db::connection::DbPool;
use serde::Deserialize;
use diesel::prelude::*;
use crate::db::models::MTGCard;
use serde_json::json;

// This should be in the 'handlers'
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_card_scryfall_id);
}

#[get("/add/scryfallid")]
pub async fn add_card_scryfall_id(
    pool: web::Data<DbPool>,
    card_info: web::Json<CardInfo>,
) -> impl Responder{

    use crate::schema::mtg_cards::dsl::*;
    let conn = &mut pool.get().expect("Couldn't get Database connection from pool");
    
    // Check if card is already in DB
    let card = mtg_cards
        .filter(scryfall_id.eq(&card_info.scryfall_id))
        .select(MTGCard::as_select())
        .first(conn)
        .optional()
        .unwrap();

    HttpResponse::Ok().json(json!(card))
}

#[derive(Deserialize)]
pub struct CardInfo {
    // this might have to be changed according 
    // to what the scanner output ends up being
    scryfall_id: String
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
