use serde::{Deserialize, Serialize};

/// Languages that cards are printed in and translated to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Language {
    #[serde(rename = "English")]
    English,

    #[serde(rename = "Ancient Greek")]
    AncientGreek,

    #[serde(rename = "Arabic")]
    Arabic,

    #[serde(rename = "Chinese Simplified")]
    ChineseSimplified,

    #[serde(rename = "Chinese Traditional")]
    ChineseTraditional,

    #[serde(rename = "French")]
    French,

    #[serde(rename = "German")]
    German,

    #[serde(rename = "Hebrew")]
    Hebrew,

    #[serde(rename = "Italian")]
    Italian,

    #[serde(rename = "Japanese")]
    Japanese,

    #[serde(rename = "Korean")]
    Korean,

    #[serde(rename = "Latin")]
    Latin,

    #[serde(rename = "Phyrexian")]
    Phyrexian,

    #[serde(rename = "Portuguese (Brazil)")]
    Portuguese,

    #[serde(rename = "Quenya")]
    Quenya,

    #[serde(rename = "Russian")]
    Russian,

    #[serde(rename = "Sanskrit")]
    Sanskrit,

    #[serde(rename = "Spanish")]
    Spanish,

    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    Unknown(crate::unknown::UnknownStr),
}

#[cfg(all(test, feature = "unknown_variants"))]
mod tests {
    use super::Language;

    #[test]
    fn unknown_language_as_map_key() {
        use std::collections::HashMap;

        // Mirrors `HashMap<Language, Option<String>>` in `Set`.
        let json = r#"{"English": "Lightning Bolt", "Klingon": "'ul tlhegh", "Crow": "SQUAWK"}"#;
        let m: HashMap<Language, Option<String>> = serde_json::from_str(json).unwrap();

        assert_eq!(m[&Language::English].as_ref().unwrap(), "Lightning Bolt");
        assert_eq!(
            m[&Language::Unknown("Klingon".into())].as_ref().unwrap(),
            "'ul tlhegh"
        );
        assert_eq!(
            m[&Language::Unknown("Crow".into())].as_ref().unwrap(),
            "SQUAWK"
        );
    }
}
