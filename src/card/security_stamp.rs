use serde::{Deserialize, Serialize};

/// The security stamp of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum SecurityStamp {
    Acorn,
    Arena,
    Circle,
    Heart,
    Oval,
    Triangle,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
