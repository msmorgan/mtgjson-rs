use serde::{Deserialize, Serialize};

/// The rarity of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum Rarity {
    Bonus,
    Common,
    Mythic,
    Rare,
    Special,
    Uncommon,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
