//! Module for the `unknown_variants` feature.
//!
//! Unrecognized enum values deserialize into an [`UnknownStr`], wrapping an
//! interned `&'static str` obtained through [`Box::leak`] and deduplicated
//! with a `HashSet`.

use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt;
use std::ops::Deref;
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

static INTERNER: LazyLock<Mutex<HashSet<&'static str>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Intern `s`, returning a `'static` reference shared by all equal inputs.
fn intern(s: &str) -> &'static str {
    let mut set = INTERNER.lock().expect("interner mutex poisoned");
    // `get(&str)` works on a `HashSet<&'static str>` because `&'static str: Borrow<str>`.
    if let Some(&existing) = set.get(s) {
        return existing;
    }
    let leaked: &'static str = Box::leak(Box::from(s));
    set.insert(leaked);
    #[cfg(test)]
    testing::maybe_warn(leaked);
    leaked
}

/// A string-valued wrapper for an enum member that this build of the crate does
/// not recognize. Wraps an interned `&'static str` (heap allocated and leaked).
///
/// It is `Copy`, and derefs to `str`, displays as the string, and compares equal
/// to `str`/`&str`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnknownStr(&'static str);

impl UnknownStr {
    /// Create an `UnknownStr` by interning `s`.
    pub fn new(s: &str) -> Self {
        Self(intern(s))
    }

    /// The underlying string, with its full `'static` lifetime.
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl From<&str> for UnknownStr {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl Deref for UnknownStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for UnknownStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for UnknownStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for UnknownStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl PartialEq<str> for UnknownStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for UnknownStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

/// Visitor allowing deserialization of a borrowed string, e.g. from a [`std::io::Reader`].
struct UnknownStrVisitor;
impl<'de> serde::de::Visitor<'de> for UnknownStrVisitor {
    type Value = UnknownStr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(UnknownStr::new(v))
    }
}

impl<'de> Deserialize<'de> for UnknownStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(UnknownStrVisitor)
    }
}

impl Serialize for UnknownStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
pub(crate) mod testing {
    thread_local! {
        static ENABLE_WARNINGS: std::cell::Cell<bool> = Default::default();
    }

    /// RAII guard to show warnings about unknown variants for the current test.
    /// Add at the beginning of a test: `let _guard = EnableWarnings::guard();`.
    ///
    /// Use `cargo test -- --no-capture` to see these warnings.
    pub(crate) struct EnableWarnings(());
    impl EnableWarnings {
        #[must_use]
        pub fn guard() -> Self {
            assert!(!ENABLE_WARNINGS.replace(true), "already enabled");
            Self(())
        }
    }
    impl Drop for EnableWarnings {
        fn drop(&mut self) {
            ENABLE_WARNINGS.set(false);
        }
    }

    pub(super) fn maybe_warn(interned: &'static str) {
        if ENABLE_WARNINGS.get() {
            eprintln!("unknown::UnknownStr({interned:?})");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnknownStr, intern};

    #[test]
    fn intern_deduplicates() {
        let a = intern("a_value");
        let b = intern(&"A_VALUE".to_ascii_lowercase());
        let c = intern("different_value");

        assert_eq!(a, b);
        assert_eq!(a.as_ptr(), b.as_ptr());

        assert_ne!(a, c);
        assert_ne!(a.as_ptr(), c.as_ptr());
    }

    #[test]
    fn unknown_str_behavior() {
        use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher};

        let s = "weird_value";
        let u: UnknownStr = s.into();
        assert_eq!(u, s); // PartialEq<&str>
        assert_eq!(&*u, s); // Deref<Target = str>
        assert_eq!(u.len(), 11); // str method via Deref

        // Hash must agree with `str`'s (required for the `Borrow<str>` impl).
        let bh: BuildHasherDefault<DefaultHasher> = Default::default();
        assert_eq!(bh.hash_one(u), bh.hash_one(s));

        assert_eq!(serde_json::to_string(&u).unwrap(), format!("\"{s}\""));
    }

    #[test]
    fn unknown_variant_round_trips() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct AllUnknowns {
            availability: Vec<crate::card::Availability>,
            booster_types: Option<Vec<crate::card::BoosterType>>,
            border_color: crate::card::BorderColor,
            finishes: Vec<crate::card::Finish>,
            frame_effects: Option<Vec<crate::card::FrameEffect>>,
            frame_version: crate::card::FrameVersion,
            layout: crate::card::Layout,
            promo_types: Option<Vec<crate::card::PromoType>>,
            rarity: crate::card::Rarity,
            security_stamp: Option<crate::card::SecurityStamp>,
            side: Option<crate::card::Side>,
            translations: std::collections::HashMap<crate::Language, Option<String>>,
        }

        // With escape sequences thrown in for good measure!
        let json = r#"[{
            "availability": ["un-known"],
            "boosterTypes": ["unKNOWN"],
            "borderColor": "un\t\tknown",
            "finishes": ["un\nknown"],
            "frameEffects": ["un    known"],
            "frameVersion": "__unknown__",
            "layout": "__unknown__",
            "promoTypes": ["__unknown__"],
            "rarity": "__unknown__",
            "securityStamp": "__unknown__",
            "side": "__unknown__",
            "translations": {"__unknown__": "we don't know"}
        }]"#
        .replace(char::is_whitespace, "");

        let obj: Vec<AllUnknowns> =
            serde_json::from_reader(std::io::BufReader::new(json.as_bytes())).unwrap();
        assert_eq!(serde_json::to_string(&obj).unwrap(), json.as_str());
    }
}
