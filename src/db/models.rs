use diesel::{Insertable, Queryable};
use super::schema::{mtg_cards, mtg_collection};

// this needs to be expanded for actual use and is kept minimal 
// for development
#[derive(Queryable, Insertable)]
#[table_name = "mtg_cards"]
pub struct MTGCard {
    // Internal id in the database
    pub id: i32,
    // Name on the card
    pub name: String,
    // Type of card (Creature, Instant, ...)
    pub type_line: String,
    // CMC
    pub cmc: i8,
    // Keywords
    pub keywords: Vec<String>,
    // Text on the card
    pub oracle_text: String,
    // Is it foil?
    pub foil: bool,
    // Power/Toughness in case it's a creature
    pub power: String,
    pub toughnes: String,
    // Price that scyfall sent on request, maybe update in regular intervals
    pub price_regular: f64,
    pub price_foil: f64,
    // Language of the card
    pub language: String,
    // Urls to the card on different platforms
    pub edhrec_url: String,
    pub cardmarket_id: i32,
    pub scryfall_id: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "mtg_collection"]
pub struct MTGCollection {
    pub id: i32,
    pub card_id: i32,
    pub count: i32
}
