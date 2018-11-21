use crate::tl_ident::TLIdent;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Param {
    pub name: TLIdent,
    #[serde(rename = "type")]
    pub kind: TLIdent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Constructor {
    #[serde(deserialize_with = "super::utils::deserialize_from_str")]
    pub id: i32,
    pub predicate: TLIdent,
    pub params: Vec<Param>,
    #[serde(rename = "type")]
    pub return_type: TLIdent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Method {
    #[serde(deserialize_with = "super::utils::deserialize_from_str")]
    pub id: i32,
    pub method: TLIdent,
    pub params: Vec<Param>,
    #[serde(rename = "type")]
    pub return_type: TLIdent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    pub constructors: Vec<Constructor>,
    pub methods: Vec<Method>,
}

impl Schema {
    pub fn new(s: &str) -> Schema {
        serde_json::from_str(&s).unwrap()
    }

    pub fn proto_schema() -> Schema {
        Self::new(include_str!("./proto.json"))
    }

    pub fn rpc_schema() -> Schema {
        Self::new(include_str!("./rpc.json"))
    }
}
