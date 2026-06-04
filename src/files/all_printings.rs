use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{meta::Meta, set::Set};

/// Every printing of every card, grouped by set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPrintings {
    /// Metadata about the data set.
    pub meta: Meta,

    /// Sets grouped by name.
    pub data: HashMap<String, Set>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[cfg_attr(not(feature = "local_tests"), ignore)]
    #[test]
    fn test_all_printings_local() {
        #[cfg(feature = "unknown_variants")]
        let _warnings = crate::unknown::testing::EnableWarnings::guard();
        let file = File::open("testdata/AllPrintings.json").unwrap();
        let reader = BufReader::new(file);
        let _: AllPrintings = serde_json::from_reader(reader).unwrap();
    }

    #[cfg_attr(not(feature = "network_tests"), ignore)]
    #[test]
    fn test_all_printings_network() {
        #[cfg(feature = "unknown_variants")]
        let _warnings = crate::unknown::testing::EnableWarnings::guard();
        let _: AllPrintings =
            reqwest::blocking::get("https://mtgjson.com/api/v5/AllPrintings.json")
                .unwrap()
                .json()
                .unwrap();
    }
}
