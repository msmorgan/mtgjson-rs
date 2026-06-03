use serde::{Deserialize, Serialize};

/// The border color of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum BorderColor {
    Black,
    Borderless,
    Gold,
    Silver,
    White,
    Yellow,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
