use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::{self, Formatter};

#[derive(FromMeta, PartialEq, Eq, Copy, Clone, Default)]
#[darling(rename_all = "PascalCase")]
pub enum SelfType {
    Rc,
    #[darling(rename = "owned")]
    #[default]
    Owned,
    Arc,
    Box,
}

impl SelfType {
    /// Get the path to create a new instance of the given self-type, if one is known
    /// from the standard library.
    pub fn new_path(self) -> Option<TokenStream> {
        self.path().map(|p| quote! { #p::new })
    }

    pub fn path(self) -> Option<TokenStream> {
        match self {
            SelfType::Owned => None,
            SelfType::Rc => Some(quote!(std::rc::Rc)),
            SelfType::Box => Some(quote!(std::boxed::Box)),
            SelfType::Arc => Some(quote!(std::sync::Arc)),
        }
    }
}

impl fmt::Display for SelfType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (match self {
            SelfType::Owned => "owned",
            SelfType::Rc => "Rc",
            SelfType::Arc => "Arc",
            SelfType::Box => "Box",
        })
        .fmt(f)
    }
}
