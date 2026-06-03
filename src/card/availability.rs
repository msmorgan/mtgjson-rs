use serde::{Deserialize, Serialize};

/// The availability of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Availability {
    #[serde(rename = "arena")]
    Arena,

    #[serde(rename = "dreamcast")]
    Dreamcast,

    #[serde(rename = "mtgo")]
    Mtgo,

    #[serde(rename = "paper")]
    Paper,

    #[serde(rename = "shandalar")]
    Shandalar,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
