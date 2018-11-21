use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    generator::arg::Arg,
    schema,
    tl_ident::TLIdent,
    utils::{i32_suffixed, ident},
};

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: TLIdent,
    pub fields: Vec<Arg>,
    pub return_type: TLIdent,
    pub id: i32,
}

impl Constructor {
    pub fn new(info: &schema::Constructor) -> Self {
        Constructor {
            name: info.predicate.clone(),
            fields: info
                .params
                .iter()
                .cloned()
                .map(|x| Arg {
                    name: x.name,
                    kind: x.kind,
                })
                .collect_vec(),
            return_type: info.return_type.clone(),
            id: info.id,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TLType {
    pub constructors: Vec<Constructor>,
}

impl TLType {
    pub fn compile(&self) -> TokenStream {
        match self.constructors.len() {
            0 => unreachable!(),
            1 => self.compile_as_struct(),
            _ => self.compile_as_enum(),
        }
    }

    fn type_name(&self) -> Ident {
        ident(self.constructors[0].return_type.as_rust_type_name())
    }

    fn compile_as_enum(&self) -> TokenStream {
        let type_name = self.type_name();
        let definition = self.compile_enum_definition();
        let write_impl = self.compile_enum_write_impl();
        let read_impl = self.compile_enum_read_impl();
        quote! {
            #definition

            impl TLType for #type_name {
                #read_impl
                #write_impl
            }
        }
    }

    /// Generate enum definition
    fn compile_enum_definition(&self) -> TokenStream {
        let type_name = self.type_name();
        let variants: Vec<TokenStream> = self
            .constructors
            .iter()
            .map(|x| {
                let variant_name = ident(x.name.as_variant_name());
                let fields = x
                    .fields
                    .iter()
                    .map(|x| {
                        let field_name = ident(x.name.as_field_name());
                        let type_path = x.kind.as_rust_type_path().parse::<TokenStream>().unwrap();
                        quote!(
                            #field_name: #type_path
                        )
                    })
                    .collect_vec();
                quote! {
                    #variant_name {
                        #(#fields,)*
                    }
                }
            })
            .collect_vec();
        quote! {
            #[derive(Debug, Clone)]
            pub enum #type_name {
                #(#variants,)*
            }
        }
    }

    /// Generate tl_write
    fn compile_enum_write_impl(&self) -> TokenStream {
        let type_name = &self.type_name();
        let variants: Vec<TokenStream> = self
            .constructors
            .iter()
            .map(|x| {
                let variant_name = ident(x.name.as_variant_name());
                let id = i32_suffixed(x.id);
                let field_names = &x
                    .fields
                    .iter()
                    .map(|x| ident(x.name.as_field_name()))
                    .collect_vec();
                quote! {
                    #type_name::#variant_name {
                        #(#field_names,)*
                    } => {
                        (#id).tl_write(output)?;
                        #(result += #field_names.tl_write(output)?;)*
                    }
                }
            })
            .collect_vec();
        quote! {
            fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
                let mut result = 4usize;
                match self {
                    #(#variants)*
                }
                Ok(result)
            }
        }
    }

    /// Generate tl_read
    fn compile_enum_read_impl(&self) -> TokenStream {
        let type_name = &self.type_name();
        let variants: Vec<TokenStream> = self
            .constructors
            .iter()
            .map(|x| {
                let id = i32_suffixed(x.id);
                let variant_name = ident(x.name.as_variant_name());
                let field_names = x
                    .fields
                    .iter()
                    .map(|x| ident(x.name.as_field_name()))
                    .collect_vec();
                quote! {
                    #id => #type_name::#variant_name {
                        #(#field_names: TLType::tl_read(input)?,)*
                    }
                }
            })
            .collect_vec();
        quote! {
            fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
                let id:i32 = TLType::tl_read(input)?;
                Ok(match id{
                    #(#variants,)*
                    _ => unreachable!(),
                })
            }
        }
    }

    fn compile_as_struct(&self) -> TokenStream {
        let type_name = &self.type_name();
        let constructor = &self.constructors[0];
        let field_exprs: Vec<TokenStream> = constructor
            .fields
            .iter()
            .map(|x| {
                let field_name = ident(x.name.as_field_name());
                let field_path = x.kind.as_rust_type_path().parse::<TokenStream>().unwrap();
                quote! {
                    #field_name: #field_path
                }
            })
            .collect_vec();
        let field_names = &constructor
            .fields
            .iter()
            .map(|x| ident(x.name.as_field_name()))
            .collect_vec();
        let id = &i32_suffixed(constructor.id);
        quote! {
            #[derive(Debug, Clone)]
            pub struct #type_name{
                #(#field_exprs, )*
            }

            impl TLType for #type_name {
                fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
                    let id: i32 = TLType::tl_read(input)?;
                    assert_eq!(#id, id);
                    Ok(#type_name {
                        #(#field_names: TLType::tl_read(input)?,)*
                    })
                }

                fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
                    (#id).tl_write(output)?;
                    let mut result = 4usize;
                    #(result += self.#field_names.tl_write(output)?;)*
                    Ok(result)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;
    use quote::quote;
    use serde_json::{json, Value};

    use super::*;
    use crate::utils::i32_suffixed;

    fn documents() -> Vec<Value> {
        vec![
            json!({
                "id": "1013613780",
                "predicate": "p_q_inner_data_temp",
                "params": [
                    {
                      "name": "pq",
                      "type": "bytes"
                    },
                    {
                      "name": "p",
                      "type": "bytes"
                    },
                    {
                      "name": "q",
                      "type": "bytes"
                    },
                    {
                      "name": "nonce",
                      "type": "int128"
                    },
                    {
                      "name": "server_nonce",
                      "type": "int128"
                    },
                    {
                      "name": "new_nonce",
                      "type": "int256"
                    },
                    {
                      "name": "expires_in",
                      "type": "int"
                }],
                "type": "P_Q_inner_data"
            }),
            json!({
                    "id": "-2083955988",
                    "predicate": "p_q_inner_data",
                    "params": [
                    {
                      "name": "pq",
                      "type": "bytes"
                    },
                    {
                      "name": "p",
                      "type": "bytes"
                    },
                    {
                      "name": "q",
                      "type": "bytes"
                    },
                    {
                      "name": "nonce",
                      "type": "int128"
                    },
                    {
                      "name": "server_nonce",
                      "type": "int128"
                    },
                    {
                      "name": "new_nonce",
                      "type": "int256"
                    }
                    ],
                    "type": "P_Q_inner_data"
            }),
        ]
    }

    fn target_enum() -> TokenStream {
        let pq_inner_data_temp_id = &i32_suffixed(-2083955988);
        quote! {
            #[derive(Debug, Clone)]
            pub enum PQInnerData {
                PQInnerDataTemp {
                    pq: TLBytes,
                    p: TLBytes,
                    q: TLBytes,
                    nonce: [u8; 16],
                    server_nonce: [u8; 16],
                    new_nonce: [u8; 32],
                    expires_in: i32,
                },
                PQInnerData {
                    pq: TLBytes,
                    p: TLBytes,
                    q: TLBytes,
                    nonce: [u8; 16],
                    server_nonce: [u8; 16],
                    new_nonce: [u8; 32],
                },
            }
            impl TLType for PQInnerData {
                fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
                    let id: i32 = TLType::tl_read(input)?;
                    Ok(match id {
                        0x3c6a_84d4i32 => PQInnerData::PQInnerDataTemp {
                            pq: TLType::tl_read(input)?,
                            p: TLType::tl_read(input)?,
                            q: TLType::tl_read(input)?,
                            nonce: TLType::tl_read(input)?,
                            server_nonce: TLType::tl_read(input)?,
                            new_nonce: TLType::tl_read(input)?,
                            expires_in: TLType::tl_read(input)?,
                        },
                        #pq_inner_data_temp_id => PQInnerData::PQInnerData {
                            pq: TLType::tl_read(input)?,
                            p: TLType::tl_read(input)?,
                            q: TLType::tl_read(input)?,
                            nonce: TLType::tl_read(input)?,
                            server_nonce: TLType::tl_read(input)?,
                            new_nonce: TLType::tl_read(input)?,
                        },
                        _ => unreachable!(),
                    })
                }

                fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
                    let mut result = 4usize;
                    match self {
                        PQInnerData::PQInnerDataTemp {
                            pq,
                            p,
                            q,
                            nonce,
                            server_nonce,
                            new_nonce,
                            expires_in,
                        } => {
                            (0x3c6a_84d4i32).tl_write(output)?;
                            result += pq.tl_write(output)?;
                            result += p.tl_write(output)?;
                            result += q.tl_write(output)?;
                            result += nonce.tl_write(output)?;
                            result += server_nonce.tl_write(output)?;
                            result += new_nonce.tl_write(output)?;
                            result += expires_in.tl_write(output)?;
                        }
                        PQInnerData::PQInnerData {
                            pq,
                            p,
                            q,
                            nonce,
                            server_nonce,
                            new_nonce,
                        } => {
                            (#pq_inner_data_temp_id).tl_write(output)?;
                            result += pq.tl_write(output)?;
                            result += p.tl_write(output)?;
                            result += q.tl_write(output)?;
                            result += nonce.tl_write(output)?;
                            result += server_nonce.tl_write(output)?;
                            result += new_nonce.tl_write(output)?;
                        }
                    }
                    Ok(result)
                }
            }
        }
    }

    fn target_struct() -> TokenStream {
        let pq_inner_data_temp_id = &i32_suffixed(-2083955988);
        quote! {
            #[derive(Debug, Clone)]
            pub struct PQInnerData {
                pq: TLBytes,
                p: TLBytes,
                q: TLBytes,
                nonce: [u8; 16],
                server_nonce: [u8; 16],
                new_nonce: [u8; 32],
            }
            impl TLType for PQInnerData {
                fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
                    let id: i32 = TLType::tl_read(input)?;
                    assert_eq!(#pq_inner_data_temp_id, id);
                    Ok(PQInnerData {
                        pq: TLType::tl_read(input)?,
                        p: TLType::tl_read(input)?,
                        q: TLType::tl_read(input)?,
                        nonce: TLType::tl_read(input)?,
                        server_nonce: TLType::tl_read(input)?,
                        new_nonce: TLType::tl_read(input)?,
                    })
                }

                fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
                    (#pq_inner_data_temp_id).tl_write(output)?;
                    let mut result = 4usize;
                    result += self.pq.tl_write(output)?;
                    result += self.p.tl_write(output)?;
                    result += self.q.tl_write(output)?;
                    result += self.nonce.tl_write(output)?;
                    result += self.server_nonce.tl_write(output)?;
                    result += self.new_nonce.tl_write(output)?;
                    Ok(result)
                }
            }
        }
    }

    #[test]
    fn test_enum_compile() {
        let documents = documents();
        let ty = TLType {
            constructors: documents
                .into_iter()
                .map(|x| serde_json::from_value(x).unwrap())
                .map(|x| Constructor::new(&x))
                .collect_vec(),
        };
        assert_eq!(target_enum().to_string(), ty.compile().to_string());
    }

    #[test]
    fn test_struct_compile() {
        let documents = &documents()[1..];
        let ty = TLType {
            constructors: documents
                .into_iter()
                .map(|x| serde_json::from_value(x.clone()).unwrap())
                .map(|x| Constructor::new(&x))
                .collect_vec(),
        };
        assert_eq!(target_struct().to_string(), ty.compile().to_string());
    }
}
