#![recursion_limit = "128"]
//! Generate code from MTProto schema to Rust code

use std::{
    env::current_dir,
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::Path,
};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::Generator,
    schema::Schema,
    tl_ident::TLIdent,
    utils::{ident, MyResult},
};

pub mod generator;
pub mod schema;
pub mod tl_ident;
pub mod utils;

fn main() {
    generate(
        &current_dir().unwrap().join("src").join("generate_proto"),
        &Schema::proto_schema(),
    )
    .unwrap();

    generate(
        &current_dir().unwrap().join("src").join("generate_rpc"),
        &Schema::rpc_schema(),
    )
    .unwrap();
}

fn generate(module_dir: &Path, schema: &Schema) -> MyResult<()> {
    create_dir_all(module_dir)?; // create module dir

    File::create(module_dir.join("mod.rs"))?
        .write_all(format!("pub mod {};\n", TLIdent::PRELUDE_MOD).as_bytes())?; // init mod.rs

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(module_dir.join(format!("{}.rs", TLIdent::PRELUDE_MOD)))?
        .write_all(&[][..])?; // create prelude module, if not exist

    let generator = Generator::new(schema);
    for (namespace, types) in generator.types.iter() {
        let prelude_module_name = ident(TLIdent::PRELUDE_MOD);
        let types = types.into_iter().map(|x| x.compile());
        let tokens = quote!(
            use super::#prelude_module_name::*;
            #(#types)*
        );
        new_module(module_dir, namespace, &tokens)?;
    }
    Ok(())
}

pub fn new_module(
    module_dir: &Path,
    module_name: impl AsRef<str>,
    content: &TokenStream,
) -> MyResult<()> {
    let target_path = module_dir.join(format!("{}.rs", module_name.as_ref()));
    let mut target_file = File::create(target_path)?;
    target_file.write_all(content.to_string().as_bytes())?;

    let module_path = module_dir.join("mod.rs");
    let mut module_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(module_path)?;
    module_file.write_all(format!("pub mod {};\n", module_name.as_ref()).as_bytes())?;

    Ok(())
}
