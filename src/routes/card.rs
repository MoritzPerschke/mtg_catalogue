use serde::Deserialize;
use diesel::PgConnection;
use scryfall::card::Card;
use actix_web::{get, web, HttpResponse, Responder};
use crate::{db::connection::DbPool, handlers::card::*};

// This should be in the 'handlers'
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_manabox_list);
}

#[derive(Deserialize)]
pub struct CardInfo {
    card_count: usize,
    card_name: String,
    set_code: String,
    number: usize 
}

#[derive(Deserialize)]
struct CardList {
    list: Vec<CardInfo>,
}

#[get("/add/manabox/list")]
pub async fn add_manabox_list(
    pool: web::Data<DbPool>,
    card_list: web::Json<CardList>,
) -> impl Responder {

    let connection: &mut PgConnection = &mut pool.get().unwrap();

    for card in &card_list.list {
        let set_code: &str = &card.set_code; 
        let card_number: usize = card.number;

        match Card::set_and_number(set_code, card_number).await {
            Err(e) => panic!("{:?}", e),
            Ok(card) => add_scryfall_card(connection, card)
        };
    }

    HttpResponse::Ok()
}

#[get("/add/manabox/single")]
pub async fn add_manabox_single() -> impl Responder {

    HttpResponse::Forbidden()
}
