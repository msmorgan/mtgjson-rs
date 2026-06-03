use serde::{Deserialize, Serialize};

/// The finish of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Finish {
    #[serde(rename = "etched")]
    Etched,

    #[serde(rename = "foil")]
    Foil,

    #[serde(rename = "nonfoil")]
    Nonfoil,

    #[serde(rename = "signed")]
    Signed,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
