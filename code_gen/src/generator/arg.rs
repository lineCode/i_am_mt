use crate::tl_ident::TLIdent;

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: TLIdent,
    pub kind: TLIdent,
}
