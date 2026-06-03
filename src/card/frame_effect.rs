use serde::{Deserialize, Serialize};

/// The frame effect of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum FrameEffect {
    BoosterFun,
    ColorShifted,
    Companion,
    CompassLandDfc,
    ConvertDfc,
    Devoid,
    Draft,
    Enchantment,
    Etched,
    ExtendedArt,
    FanDfc,
    FullArt,
    Gilded,
    Inverted,
    Legendary,
    Lesson,
    Miracle,
    MoonEldraziDfc,
    NyxTouched,
    OriginPwDfc,
    ShatteredGlass,
    Showcase,
    Snow,
    Spree,
    SunMoonDfc,
    Textless,
    Tombstone,
    UpsideDownDfc,
    WaxingAndWaningMoonDfc,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}
