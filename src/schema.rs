// @generated automatically by Diesel CLI.

diesel::table! {
    mtg_card_collection (id) {
        id -> Int4,
        count -> Int4,
    }
}

diesel::table! {
    mtg_cards (id) {
        id -> Int4,
        name -> Varchar,
        type_line -> Varchar,
        cmc -> Int4,
        keywords -> Nullable<Array<Text>>,
        oracle_text -> Nullable<Varchar>,
        foil -> Bool,
        power -> Varchar,
        toughness -> Varchar,
        price_regular -> Float,
        price_foil -> Float,
        language -> Varchar,
        cardmarket_id -> Int4,
        scryfall_id -> Varchar,
        edhrec_url -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mtg_card_collection,
    mtg_cards,
);
