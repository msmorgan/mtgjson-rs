use serde::{Deserialize, Serialize};

/// Describes a list of identifiers associated to a Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct Identifiers {
    pub abu_id: Option<String>,
    pub card_kingdom_etched_id: Option<String>,
    pub card_kingdom_foil_id: Option<String>,
    pub card_kingdom_id: Option<String>,
    pub cardsphere_alternative_foil_id: Option<String>,
    pub cardsphere_etched_id: Option<String>,
    pub cardsphere_foil_id: Option<String>,
    pub cardsphere_id: Option<String>,
    pub cardtrader_id: Option<String>,
    pub csi_id: Option<String>,
    pub deckbox_id: Option<String>,
    pub mcm_id: Option<String>,
    pub mcm_meta_id: Option<String>,
    pub miniaturemarket_id: Option<String>,
    pub mtg_arena_id: Option<String>,
    pub mtgo_foil_id: Option<String>,
    pub mtgo_id: Option<String>,
    pub mtgjson_foil_version_id: Option<String>,
    pub mtgjson_non_foil_version_id: Option<String>,
    pub mtgjson_v4_id: Option<String>,
    pub multiverse_id: Option<String>,
    pub scg_id: Option<String>,
    pub scryfall_card_back_id: Option<String>,
    pub scryfall_id: Option<String>,
    pub scryfall_oracle_id: Option<String>,
    pub scryfall_illustration_id: Option<String>,
    pub tcgplayer_product_id: Option<String>,
    pub tcgplayer_alternative_foil_product_id: Option<String>,
    pub tcgplayer_etched_product_id: Option<String>,
    pub tnt_id: Option<String>,
}
