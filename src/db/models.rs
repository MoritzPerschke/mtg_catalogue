use diesel::prelude::*;

// this needs to be expanded for actual use and is kept minimal 
// for development
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::mtg_cards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MTGCard {
    // Internal id in the database
    pub id: i32,
    // Name on the card
    pub name: String,
    // Type of card (Creature, Instant, ...)
    pub type_line: String,
    // CMC
    pub cmc: i32,
    // Keywords
    pub keywords: Option<Vec<String>>,
    // Text on the card
    pub oracle_text: Option<String>,
    // Is it foil?
    pub foil: bool,
    // Power/Toughness in case it's a creature
    pub power: String,
    pub toughness: String,
    // Price that scyfall sent on request, maybe update in regular intervals
    pub price_regular: f32,
    pub price_foil: f32,
    // Language of the card
    pub language: String,
    // Urls to the card on different platforms
    pub edhrec_url: String,
    pub cardmarket_id: i32,
    pub scryfall_id: String,
    // Color identity
    // set
    // Multiverse ID
    // Images
    // Colors
    // keywords
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::mtg_card_collection)]
pub struct MTGCollection {
    pub id: i32,
    pub count: i32
    // a new table for each user would be nice,
    // otherwise just extend Cards table?
    // Neither seems like the right thing,
    // DBS course was a while ago
}
