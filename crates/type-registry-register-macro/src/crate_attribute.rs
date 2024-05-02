use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Path, Token};
use syn::parse::{Parse, ParseStream};

pub struct CrateAttribute {
    keyword: Token![crate],
    equals: Token![=],
    path: Path
}

impl CrateAttribute {
    pub fn into_path(self) -> Path {
        self.path
    }
}

impl Parse for CrateAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword = Parse::parse(input)?;
        let equals = Parse::parse(input)?;
        let path = Parse::parse(input)?;

        Ok(
            Self {
                keyword,
                equals,
                path
            }
        )
    }
}

impl ToTokens for CrateAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.keyword.to_tokens(tokens);
        self.equals.to_tokens(tokens);
        self.path.to_tokens(tokens);
    }
}
