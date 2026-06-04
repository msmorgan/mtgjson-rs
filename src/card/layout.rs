use serde::{Deserialize, Serialize};

/// The layout of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Layout {
    Adventure,
    Aftermath,
    ArtSeries,
    Augment,
    Case,
    Class,
    DoubleFacedToken,
    Emblem,
    Flip,
    Host,
    Leveler,
    Meld,
    ModalDfc,
    Mutate,
    Normal,
    Planar,
    Prepare,
    Prototype,
    ReversibleCard,
    Saga,
    Scheme,
    Split,
    Token,
    Transform,
    Vanguard,
}
