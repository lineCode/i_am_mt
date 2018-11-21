use crate::{
    generator::tl_type::{Constructor, TLType},
    schema::Schema,
    tl_ident::TLIdent,
};
use std::collections::HashMap;

pub mod arg;
pub mod tl_type;

pub struct Generator {
    pub types: HashMap<String, Vec<TLType>>, // pub methods
}

impl Generator {
    pub fn new(schema: &Schema) -> Self {
        let mut namespace: HashMap<String, HashMap<TLIdent, TLType>> = HashMap::new();
        for c in schema
            .constructors
            .iter()
            .filter(|x| x.return_type.should_generate())
        {
            let ns = c
                .return_type
                .namespace_split()
                .0
                .unwrap_or_else(|| TLIdent::TOP_MOD.to_string());
            let module = namespace.entry(ns).or_insert_with(HashMap::new);
            module
                .entry(c.return_type.clone())
                .or_insert_with(TLType::default)
                .constructors
                .push(Constructor::new(c));
        }

        Generator {
            types: namespace
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|x| x.1).collect()))
                .collect(),
        }
    }
}
