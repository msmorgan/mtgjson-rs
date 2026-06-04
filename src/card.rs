use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{identifiers::Identifiers, language::Language, purchase_urls::PurchaseUrls};

mod availability;
mod booster_type;
mod border_color;
mod color;
mod duel_deck;
mod finish;
mod foreign_data;
mod frame_effect;
mod frame_version;
mod layout;
mod leadership_skills;
mod legalities;
mod produced_mana;
mod promo_type;
mod rarity;
mod related_cards;
mod ruling;
mod security_stamp;
mod side;
mod token_product;

pub use availability::*;
pub use booster_type::*;
pub use border_color::*;
pub use color::*;
pub use duel_deck::*;
pub use finish::*;
pub use foreign_data::*;
pub use frame_effect::*;
pub use frame_version::*;
pub use layout::*;
pub use leadership_skills::*;
pub use legalities::*;
pub use produced_mana::*;
pub use promo_type::*;
pub use rarity::*;
pub use related_cards::*;
pub use ruling::*;
pub use security_stamp::*;
pub use side::*;
pub use token_product::*;

/// Represents a unique card, not specific to any one printing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCard {
    /// The ASCII (Basic/128) code formatted card name with no special unicode characters.
    pub ascii_name: Option<String>,

    /// The attraction lights on the card.
    pub attraction_lights: Option<Vec<u8>>,

    /// The related parts of the card.
    pub card_parts: Option<Vec<String>>,

    /// A list of all the colors found in `mana_cost`, `color_indicator`, and `text`.
    pub color_identity: Vec<Color>,

    /// A list of all the colors in the color indicator (The symbol prefixed to a card's types).
    pub color_indicator: Option<Vec<Color>>,

    /// A list of all the colors in `mana_cost` and `color_indicator`. Some cards may not have values, such as cards with "Devoid" in its text.
    pub colors: Vec<Color>,

    /// The converted mana cost of the card. Use the `mana_value` property.
    #[deprecated(since = "5.2.0", note = "Will be removed in 6.0.0")]
    pub converted_mana_cost: Option<f32>,

    /// The card rank on EDHRec.
    pub edhrec_rank: Option<u32>,

    /// The converted mana cost or mana value for the face for either half or part of the card. Use the `face_mana_value` property.
    #[deprecated(since = "5.2.0", note = "Will be removed in 6.0.0")]
    pub face_converted_mana_cost: Option<f32>,

    /// The mana value of the face for either half or part of the card. Formally known as "converted mana cost".
    pub face_mana_value: Option<f32>,

    /// The name on the face of the card.
    pub face_name: Option<String>,

    /// A list of data properties in other languages.
    pub foreign_data: Vec<ForeignData>,

    /// The starting maximum hand size total modifier. A + or - character precedes an integer.
    pub hand: Option<String>,

    /// If the card allows a value other than 4 copies in a deck.
    pub has_alternative_deck_limit: Option<bool>,

    /// A list of identifiers associated to a card.
    pub identifiers: Identifiers,

    /// If the card is part of a funny set.
    pub is_funny: Option<bool>,

    /// If the card is on the Magic: The Gathering Reserved List.
    pub is_reserved: Option<bool>,

    /// A list of keywords found on the card.
    pub keywords: Option<Vec<String>>,

    /// The type of card layout. For a token card, this will be "token".
    pub layout: Layout,

    /// A list of formats the card is legal to be a commander in.
    pub leadership_skills: Option<LeadershipSkills>,

    /// A list of play formats the card the card is legal in.
    pub legalities: Legalities,

    /// The starting life total modifier. A plus or minus character precedes an integer. Used only on cards with "Vanguard" in its types.
    pub life: Option<String>,

    /// The starting loyalty value of the card. Used only on cards with "Planeswalker" in its types.
    pub loyalty: Option<String>,

    /// The mana cost of the card wrapped in brackets for each value.
    pub mana_cost: Option<String>,

    /// The mana value of the card. Formally known as "converted mana cost".
    pub mana_value: Option<f32>,

    /// The name of the card. Cards with multiple faces, like "Split" and "Meld" cards are given a delimiter.
    pub name: String,

    /// The power of the card.
    pub power: Option<String>,

    /// A list of set printing codes the card was printed in, formatted in uppercase.
    pub printings: Option<Vec<String>>,

    /// A list of colors of mana the card can produce.
    pub produced_mana: Option<Vec<ProducedMana>>,

    /// Links that navigate to websites where the card can be purchased.
    pub purchase_urls: PurchaseUrls,

    /// The official rulings of the card.
    pub rulings: Option<Vec<Ruling>>,

    /// The identifier of the card side. Used on cards with multiple faces on the same card.
    pub side: Option<Side>,

    /// A list of card subtypes found after em-dash.
    pub subtypes: Vec<String>,

    /// A list of card supertypes found before em-dash.
    pub supertypes: Vec<String>,

    /// The rules text of the card.
    pub text: Option<String>,

    /// The toughness of the card.
    pub toughness: Option<String>,

    /// The type of the card as visible, including any supertypes and subtypes.
    #[serde(rename = "type")]
    pub card_type: String,

    /// A list of all card types of the card, including Un‑sets and gameplay variants.
    #[serde(rename = "types")]
    pub card_types: Vec<String>,
}

/// Represents a card printed in a set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCard {
    /// The name of the artist that illustrated the card art.
    pub artist: Option<String>,

    /// A list of identifiers for the artists that illustrated the card art.
    pub artist_ids: Option<Vec<String>>,

    /// The ASCII (Basic/128) code formatted card name with no special unicode characters.
    pub ascii_name: Option<String>,

    /// The attraction lights on the card.
    pub attraction_lights: Option<Vec<u8>>,

    /// A list of the card's available printing types.
    pub availability: Vec<Availability>,

    /// A list of types this card is in a booster pack.
    pub booster_types: Option<Vec<BoosterType>>,

    /// The color of the card border.
    pub border_color: BorderColor,

    /// The related parts of the card.
    pub card_parts: Option<Vec<String>>,

    /// A list of all the colors found in `mana_cost`, `color_indicator`, and `text`.
    pub color_identity: Vec<Color>,

    /// A list of all the colors in the color indicator (The symbol prefixed to a card's types).
    pub color_indicator: Option<Vec<Color>>,

    /// A list of all the colors in `mana_cost` and `color_indicator`. Some cards may not have values, such as cards with "Devoid" in its text.
    pub colors: Vec<Color>,

    /// The converted mana cost of the card. Use the `mana_value` property.
    #[deprecated(since = "5.2.0", note = "Will be removed in 6.0.0")]
    pub converted_mana_cost: f32,

    /// The duel deck this card was included in.
    pub duel_deck: Option<DuelDeck>,

    /// The card rank on EDHRec.
    pub edhrec_rank: Option<u32>,

    /// The converted mana cost or mana value for the face for either half or part of the card. Use the `face_mana_value` property.
    #[deprecated(since = "5.2.0", note = "Will be removed in 6.0.0")]
    pub face_converted_mana_cost: Option<f32>,

    /// The flavor name on the face of the card.
    pub face_flavor_name: Option<String>,

    /// The mana value of the face for either half or part of the card. Formally known as "converted mana cost".
    pub face_mana_value: Option<f32>,

    /// The name on the face of the card.
    pub face_name: Option<String>,

    /// The finishes of the card.
    pub finishes: Vec<Finish>,

    /// The promotional card name printed above the true card name on special cards that has no game function.
    pub flavor_name: Option<String>,

    /// The italicized text found below the rules text that has no game function.
    pub flavor_text: Option<String>,

    /// A list of data properties in other languages.
    pub foreign_data: Vec<ForeignData>,

    /// The visual frame effects.
    pub frame_effects: Option<Vec<FrameEffect>>,

    /// The version of the card frame style.
    pub frame_version: FrameVersion,

    /// The starting maximum hand size total modifier. A + or - character precedes an integer.
    pub hand: Option<String>,

    /// If the card allows a value other than 4 copies in a deck.
    pub has_alternative_deck_limit: Option<bool>,

    /// If the card marked by Wizards of the Coast for having sensitive content. Cards with this property may have missing or degraded properties and values.
    pub has_content_warning: Option<bool>,

    /// A list of identifiers associated to a card.
    pub identifiers: Identifiers,

    /// If the card is an alternate variation to an original printing.
    pub is_alternative: Option<bool>,

    /// If the card has full artwork.
    pub is_full_art: Option<bool>,
    /// If the card is a Game Changer card.
    pub is_game_changer: Option<bool>,

    /// If the card is part of a funny set.
    pub is_funny: Option<bool>,

    /// If the card is only available in online game variations.
    pub is_online_only: Option<bool>,

    /// If the card is oversized.
    pub is_oversized: Option<bool>,

    /// If the card is a promotional printing.
    pub is_promo: Option<bool>,

    /// If the card is rebalanced for the Alchemy play format.
    pub is_rebalanced: Option<bool>,

    /// If the card has been reprinted.
    pub is_reprint: Option<bool>,

    /// If the card is on the Magic: The Gathering Reserved List.
    pub is_reserved: Option<bool>,

    /// If the card is a Story Spotlight card.
    pub is_story_spotlight: Option<bool>,

    /// If the card does not have a text box.
    pub is_textless: Option<bool>,

    /// If the card is "timeshifted", a feature of certain sets where a card will have a different `frame_version`.
    pub is_timeshifted: Option<bool>,

    /// A list of keywords found on the card.
    pub keywords: Option<Vec<String>>,

    /// The language the card is printed in.
    pub language: Language,

    /// The type of card layout. For a token card, this will be "token".
    pub layout: Layout,

    /// A list of formats the card is legal to be a commander in.
    pub leadership_skills: Option<LeadershipSkills>,

    /// A list of play formats the card the card is legal in.
    pub legalities: Legalities,

    /// The starting life total modifier. A plus or minus character precedes an integer. Used only on cards with "Vanguard" in its types.
    pub life: Option<String>,

    /// The starting loyalty value of the card. Used only on cards with "Planeswalker" in its types.
    pub loyalty: Option<String>,

    /// The mana cost of the card wrapped in brackets for each value.
    pub mana_cost: Option<String>,

    /// The mana value of the card. Formally known as "converted mana cost".
    pub mana_value: f32,

    /// The name of the card. Cards with multiple faces, like "Split" and "Meld" cards are given a delimiter.
    pub name: String,

    /// The number of the card. Can be prefixed or suffixed with a * or other characters for promotional sets.
    pub number: String,

    /// A list of card UUID's to original printings of the card if this card is somehow different from its original, such as rebalanced cards.
    pub original_printings: Option<Vec<Uuid>>,

    /// The original release date in ISO 8601 format for a promotional card printed outside of a cycle window, such as Secret Lair Drop promotions.
    pub original_release_date: Option<NaiveDate>,

    /// The text on the card as originally printed.
    pub original_text: Option<String>,

    /// The type of the card as originally printed. Includes any supertypes and subtypes.
    pub original_type: Option<String>,

    /// A list of card UUID's to this card's counterparts, such as transformed or melded faces.
    pub other_face_ids: Option<Vec<Uuid>>,
    /// The actual printed name on the card face.
    pub printed_name: Option<String>,

    /// The actual printed rules text on the card face.
    pub printed_text: Option<String>,

    /// The actual printed type line on the card face.
    pub printed_type: Option<String>,

    /// The power of the card.
    pub power: Option<String>,
    /// A list of colors of mana the card can produce.
    pub produced_mana: Option<Vec<ProducedMana>>,

    /// A list of set printing codes the card was printed in, formatted in uppercase.
    pub printings: Option<Vec<String>>,

    /// A list of promotional types for a card.
    pub promo_types: Option<Vec<PromoType>>,

    /// Links that navigate to websites where the card can be purchased.
    pub purchase_urls: PurchaseUrls,

    /// The card printing rarity. Rarity bonus relates to cards that have an alternate availability in booster packs, while special relates to "Timeshifted" cards.
    pub rarity: Rarity,

    /// Rebalanced digital printings.
    pub rebalanced_printings: Option<Vec<Uuid>>,

    /// Related cards.
    pub related_cards: Option<RelatedCards>,

    /// Reverse related cards.
    pub reverse_related: Option<Vec<String>>,

    /// The official rulings of the card.
    pub rulings: Option<Vec<Ruling>>,

    /// The security stamp printed on the card.
    pub security_stamp: Option<SecurityStamp>,

    /// The set printing code that the card is from.
    pub set_code: String,

    /// The identifier of the card side. Used on cards with multiple faces on the same card.
    pub side: Option<Side>,

    /// The name of the signature on the card.
    pub signature: Option<String>,

    /// The subset of the card.
    pub subset: Option<Vec<String>>,

    /// A list of card subtypes found after em-dash.
    pub subtypes: Vec<String>,

    /// A list of card supertypes found before em-dash.
    pub supertypes: Vec<String>,
    /// A list of UUID's of tokens created by this card within the set.
    pub tokens: Option<Vec<Uuid>>,

    /// The rules text of the card.
    pub text: Option<String>,

    /// The toughness of the card.
    pub toughness: Option<String>,

    /// The type of the card as visible, including any supertypes and subtypes.
    #[serde(rename = "type")]
    pub card_type: String,

    /// A list of all card types of the card, including Un‑sets and gameplay variants.
    #[serde(rename = "types")]
    pub card_types: Vec<String>,

    /// The universal unique identifier (v5) generated by MTGJSON. Each entry is unique.
    pub uuid: Uuid,

    /// A list of card UUID's of this card with alternate printings in the same set. Excludes Un‑sets.
    #[serde(default)]
    pub variations: Vec<Uuid>,

    /// The name of the watermark on the card.
    pub watermark: Option<String>,
}

/// Represents a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCard {
    /// The name of the artist that illustrated the card art.
    pub artist: Option<String>,

    /// A list of identifiers for the artists that illustrated the card art.
    pub artist_ids: Option<Vec<String>>,

    /// The ASCII (Basic/128) code formatted card name with no special unicode characters.
    pub ascii_name: Option<String>,

    /// A list of the card's available printing types.
    pub availability: Vec<Availability>,

    /// A list of types this card is in a booster pack.
    pub booster_types: Option<Vec<BoosterType>>,

    /// The color of the card border.
    pub border_color: BorderColor,

    /// The related parts of the card.
    pub card_parts: Option<Vec<String>>,

    /// A list of all the colors found in `mana_cost`, `color_indicator`, and `text`.
    pub color_identity: Vec<Color>,

    /// A list of all the colors in the color indicator (The symbol prefixed to a card's types).
    pub color_indicator: Option<Vec<Color>>,

    /// A list of all the colors in `mana_cost` and `color_indicator`. Some cards may not have values, such as cards with "Devoid" in its text.
    pub colors: Vec<Color>,

    /// The flavor name on the face of the card.
    pub face_flavor_name: Option<String>,

    /// The name on the face of the card.
    pub face_name: Option<String>,

    /// The finishes of the card.
    pub finishes: Vec<Finish>,

    /// The italicized text found below the rules text that has no game function.
    pub flavor_text: Option<String>,

    /// The visual frame effects.
    pub frame_effects: Option<Vec<FrameEffect>>,

    /// The version of the card frame style.
    pub frame_version: FrameVersion,

    /// A list of identifiers associated to a card.
    pub identifiers: Identifiers,

    /// If the card has full artwork.
    pub is_full_art: Option<bool>,

    /// If the card is part of a funny set.
    pub is_funny: Option<bool>,

    /// If the card is only available in online game variations.
    pub is_online_only: Option<bool>,

    /// If the card is a promotional printing.
    pub is_promo: Option<bool>,

    /// If the card has been reprinted.
    pub is_reprint: Option<bool>,

    /// If the card does not have a text box.
    pub is_textless: Option<bool>,

    /// A list of keywords found on the card.
    pub keywords: Option<Vec<String>>,

    /// The language the card is printed in.
    pub language: Language,

    /// The type of card layout. For a token card, this will be "token".
    pub layout: Layout,

    /// The starting loyalty value of the card. Used only on cards with "Planeswalker" in its types.
    pub loyalty: Option<String>,

    /// The name of the card. Cards with multiple faces, like "Split" and "Meld" cards are given a delimiter.
    pub name: String,

    /// The number of the card. Can be prefixed or suffixed with a * or other characters for promotional sets.
    pub number: String,

    /// The text on the card as originally printed.
    pub original_text: Option<String>,

    /// The type of the card as originally printed. Includes any supertypes and subtypes.
    pub original_type: Option<String>,

    /// A list of card UUID's to this card's counterparts, such as transformed or melded faces.
    pub other_face_ids: Option<Vec<Uuid>>,

    /// The power of the card.
    pub power: Option<String>,

    /// A list of promotional types for a card.
    pub promo_types: Option<Vec<PromoType>>,

    /// Related cards.
    pub related_cards: Option<RelatedCards>,

    /// The security stamp printed on the card.
    pub security_stamp: Option<SecurityStamp>,

    /// The set printing code that the card is from.
    pub set_code: String,

    /// The identifier of the card side. Used on cards with multiple faces on the same card.
    pub side: Option<Side>,

    /// The name of the signature on the card.
    pub signature: Option<String>,

    /// A list of card subtypes found after em-dash.
    pub subtypes: Vec<String>,

    /// A list of card supertypes found before em-dash.
    pub supertypes: Vec<String>,
    /// A list of purchasable products containing the token, linking together
    /// the faces of double-sided tokens.
    pub token_products: Option<Vec<TokenProduct>>,

    /// A list of UUID's of tokens created by this card within the set.
    pub tokens: Option<Vec<Uuid>>,

    /// The rules text of the card.
    pub text: Option<String>,

    /// The toughness of the card.
    pub toughness: Option<String>,

    /// The type of the card as visible, including any supertypes and subtypes.
    #[serde(rename = "type")]
    pub card_type: String,

    /// A list of all card types of the card, including Un‑sets and gameplay variants.
    #[serde(rename = "types")]
    pub card_types: Vec<String>,

    /// The universal unique identifier (v5) generated by MTGJSON. Each entry is unique.
    pub uuid: Uuid,

    /// The name of the watermark on the card.
    pub watermark: Option<String>,
}
