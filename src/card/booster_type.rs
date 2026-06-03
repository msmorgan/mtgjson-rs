use serde::{Deserialize, Serialize};

/// The type of a booster.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum BoosterType {
    Deck,
    Default,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
