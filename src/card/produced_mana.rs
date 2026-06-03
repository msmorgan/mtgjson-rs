use serde::{Deserialize, Serialize};

/// A type of mana a card can produce.
///
/// In addition to the five colors, cards can produce colorless mana (`C`) and,
/// on some Un-set cards, the tap symbol (`T`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ProducedMana {
    #[serde(rename = "B")]
    Black,

    #[serde(rename = "C")]
    Colorless,

    #[serde(rename = "G")]
    Green,

    #[serde(rename = "R")]
    Red,

    #[serde(rename = "T")]
    Tap,

    #[serde(rename = "U")]
    Blue,

    #[serde(rename = "W")]
    White,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
