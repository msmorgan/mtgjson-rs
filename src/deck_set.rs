use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::deck_set::CardSetDeck;

mod card_set_deck;

pub use card_set_deck::*;

/// The Deck (Set) Data Model describes the properties of an individual Deck within a Set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckSet {
    /// The printing set code for the deck.
    pub code: String,

    /// The card that is the Commander in this deck.
    pub commander: Option<Vec<CardSetDeck>>,

    /// The cards in the main-board.
    pub main_board: Vec<CardSetDeck>,

    /// The name of the deck.
    pub name: String,

    /// The release date in ISO 8601 format for the set.
    pub release_date: Option<String>,

    /// A list of UUID's associated to this Deck in a Sealed Product.
    pub sealed_product_uuids: Option<Vec<Uuid>>,

    /// The cards in the side-board.
    pub side_board: Vec<CardSetDeck>,

    /// The printing set codes that the cards in the deck are sourced from.
    pub source_set_codes: Vec<String>,

    /// The token cards in the deck.
    pub tokens: Option<Vec<CardSetDeck>>,

    /// The type of deck.
    #[serde(rename = "type")]
    pub deck_set_type: String,
}
