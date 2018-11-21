use derive_more::FromStr;
use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};

/// Variable name and Type name in TL language
///
/// # Warning
///
/// + Generic support: only includes Vector
/// + Namespace support: maximum two layers
#[derive(Debug, FromStr, Clone, Eq, PartialEq, Hash)]
pub struct TLIdent(String);

impl TLIdent {
    pub const PRELUDE_MOD: &'static str = "prelude";
    pub const TOP_MOD: &'static str = "tl";

    pub fn is_built_in_type(&self) -> bool {
        [
            "int", "long", "double", "bytes", "string", "int128", "int256", "Vector t", "Bool",
        ]
        .contains(&self.0.as_str())
    }

    pub fn is_special(&self) -> bool {
        ["True", "Null", "PeerSettings", "MessageContainer"].contains(&self.0.as_str())
    }

    pub fn should_generate(&self) -> bool {
        !self.is_built_in_type() && !self.is_special() && !self.is_vector()
    }

    fn generic_split(&self) -> (Option<String>, String) {
        lazy_static! {
            static ref GENERIC_PATTERN: Regex =
                Regex::new(r"(?i)^(?P<outer>.+?)<(?P<inner>.+?)>$").unwrap();
        }
        if let Some(cap) = GENERIC_PATTERN.captures(self.0.as_str()) {
            let inner: &str = cap.name("inner").unwrap().as_str();
            let outer: &str = cap.name("outer").unwrap().as_str();
            assert_eq!(outer.to_lowercase(), "vector".to_string());
            assert!(!inner.contains('<'));
            (Some(outer.to_string()), inner.to_string())
        } else {
            (None, self.0.clone())
        }
    }

    pub fn namespace_split(&self) -> (Option<String>, String) {
        assert!(!self.0.contains('<'));
        let parts: Vec<&str> = self.0.split('.').collect();
        match parts.len() {
            1 => (None, parts[0].to_string()),
            2 => (Some(parts[0].to_string()), parts[1].to_string()),
            _ => unreachable!(),
        }
    }

    fn is_vector(&self) -> bool {
        let (outer, _) = self.generic_split();
        outer.map(|x| x.to_lowercase()) == Some("vector".to_string())
    }

    /// aaa.Bbb -> Bbb
    /// Vector<aaa.Bbb> -> Vec<super::aaa::Bbb>
    /// Vector<Bbb> -> Vec<super::prelude::Bbb>
    pub fn as_rust_type_name(&self) -> String {
        let (_, inner) = self.generic_split();
        let inner = TLIdent(inner);

        if self.is_vector() {
            format!("Vec<{}>", inner.as_rust_type_path())
        } else {
            inner.namespace_split().1.to_camel_case()
        }
    }

    /// aaa.Bbb -> super::aaa::Bbb
    /// Vector<aaa.Bbb> -> Vec<super::aaa::Bbb>
    /// Vector<Bbb> -> Vec<super::prelude::Bbb>
    pub fn as_rust_type_path(&self) -> String {
        let (_, inner) = self.generic_split();
        let inner = TLIdent(inner);
        if self.is_vector() {
            format!("Vec<{}>", inner.as_rust_type_path())
        } else {
            assert!(!self.0.contains('<'));
            assert!(!self.0.contains('>'));
            let (ns, name) = inner.namespace_split();
            if ns.is_some() {
                return format!("super::{}::{}", ns.unwrap(), name);
            }
            match name.as_str() {
                "int" => "i32".to_string(),
                "long" => "i64".to_string(),
                "double" => "f64".to_string(),
                "bytes" => "TLBytes".to_string(),
                "string" => "String".to_string(),
                "int128" => "[u8; 16]".to_string(),
                "int256" => "[u8; 32]".to_string(),
                "Bool" => "bool".to_string(),
                "PeerSettings" => unreachable!(),
                _ => format!("super::{}::{}", Self::TOP_MOD, name.to_camel_case()),
            }
        }
    }

    pub fn as_field_name(&self) -> String {
        assert!(!self.0.contains('.'));
        assert!(!self.0.contains('<'));
        assert!(!self.0.contains('>'));
        match self.0.as_str() {
            "type" => "ty".to_string(),
            _ => self.0.to_snake_case(),
        }
    }

    pub fn as_variant_name(&self) -> String {
        assert!(!self.0.contains('<'));
        assert!(!self.0.contains('>'));
        let (_, inner) = self.namespace_split();
        assert!(!inner.contains('.'));
        inner.to_camel_case()
    }
}

impl<'de> Deserialize<'de> for TLIdent {
    fn deserialize<D>(deserializer: D) -> Result<TLIdent, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TLIdent(Deserialize::deserialize(deserializer)?))
    }
}
