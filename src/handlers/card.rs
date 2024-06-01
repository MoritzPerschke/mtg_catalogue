use diesel::prelude::*;
use diesel::PgConnection;
use scryfall::card::Card;
use crate::schema::mtg_card_collection::dsl::*;
use crate::db::models::{MTGCard, MTGCollection};

pub fn add_scryfall_card(
    connection: &mut PgConnection,
    card_info: Card,
) -> bool {

    use crate::schema::mtg_cards::dsl::*;
    // let conn = &mut pool.get().expect("Couldn't get Database connection from pool");
    
    // There HAS to be a simpler way no?
    let card_id: String = card_info.id
        .try_into()
        .unwrap();

    // Check if card is already in DB
    let card_db: MTGCard = mtg_cards
        .filter(scryfall_id.eq(card_id))
        // .select(MTGCard::as_select())
        .first(connection)
        .unwrap();


    // Check if card is in collection
    let card_collection = mtg_card_collection
        .filter(id.eq(card_db.id))
        .first(connection)
        .unwrap();

    // diesel::update(mtg_card_collection)
    //     .filter(id.eq(card.id))
    //     .set(count.eq(count + 1))
    //     .execute(connection);


    true
}
