use actix_web::{web, HttpResponse, Responder};
use crate::db::connection::DbPool;

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
    let conn = pool.get().expect("Couldn't get Database connection from pool");
    
    // Check if card is already in DB
    let card = mtg_cards::table
        .filter(mtg_cards::gatherer_id.eq(&card_info.gatherer_id));

    HttpResponse::Ok()
}
