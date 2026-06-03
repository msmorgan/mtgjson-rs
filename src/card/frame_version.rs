use serde::{Deserialize, Serialize};

/// The version of the frame of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum FrameVersion {
    #[serde(rename = "1993")]
    Y1993,

    #[serde(rename = "1997")]
    Y1997,

    #[serde(rename = "2003")]
    Y2003,

    #[serde(rename = "2015")]
    Y2015,

    #[serde(rename = "future")]
    Future,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
