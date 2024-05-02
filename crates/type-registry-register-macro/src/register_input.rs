use std::mem::discriminant;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Attribute, AttrStyle, Generics, Item, Meta, Path};
use syn::parse::{Parse, ParseStream};
use crate::crate_attribute::CrateAttribute;

pub(crate) enum RegisterInput {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum),
    Union(syn::ItemUnion)
}

impl RegisterInput {
    pub fn ident(&self) -> &Ident {
        match self {
            RegisterInput::Struct(item) => &item.ident,
            RegisterInput::Enum(item) => &item.ident,
            RegisterInput::Union(item) => &item.ident
        }
    }
    
    pub fn generics(&self) -> &Generics {
        match self {
            RegisterInput::Struct(item) => &item.generics,
            RegisterInput::Enum(item) => &item.generics,
            RegisterInput::Union(item) => &item.generics
        }
    }
    
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        match self {
            RegisterInput::Struct(item) => &mut item.attrs,
            RegisterInput::Enum(item) => &mut item.attrs,
            RegisterInput::Union(item) => &mut item.attrs
        }
    }
    
    fn type_registry_attributes(&mut self) -> Vec<TokenStream> {
        let mut our_attrs = Vec::new();

        let extract = |attr: &Attribute| {
            if discriminant(&attr.style) != discriminant(&AttrStyle::Outer) { return true }
            let meta = match &attr.meta {
                Meta::List(meta) => meta,
                _ => return true
            };
            if !meta.path.is_ident("type_registry") { return true }
            our_attrs.push(meta.tokens.clone());
            false
        };

        self.attrs().retain(extract);
        
        our_attrs
    }
    
    pub fn crate_(&mut self) -> syn::Result<Path> {
        let our_attrs = self.type_registry_attributes();
        
        for attr in our_attrs {
            return Ok(syn::parse2::<CrateAttribute>(attr)?.into_path())
        }
        
        Ok(Self::default_crate())
    }
    
    fn default_crate() -> Path {
        syn::parse_str("::type_registry").expect("path is well-formed")
    }
}

impl Parse for RegisterInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed = match Item::parse(input)? {
            Item::Enum(item) => Self::Enum(item),
            Item::Struct(item) => Self::Struct(item),
            Item::Union(item) => Self::Union(item),
            other => return Err(syn::Error::new_spanned(other, "only structs, enums and unions can be registered"))
        };

        Ok(parsed)
    }
}

impl ToTokens for RegisterInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RegisterInput::Struct(item) => item.to_tokens(tokens),
            RegisterInput::Enum(item) => item.to_tokens(tokens),
            RegisterInput::Union(item) => item.to_tokens(tokens),
        }
    }
}
