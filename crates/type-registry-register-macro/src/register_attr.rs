use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Expr, Path, Token, Type};
use syn::parse::{Parse, ParseStream};

pub(crate) struct RegisterAttr {
    registry: Type,
    comma: Option<Token![,]>,
    init_type_info_expr: Option<Expr>
}

impl RegisterAttr {
    pub fn into_parts(self, ident: &Ident, crate_: &Path) -> (Type, Expr) {
        let Self {
            registry,
            init_type_info_expr,
            ..
        } = self;
        
        let init_type_info_expr = init_type_info_expr.unwrap_or_else(
            || Self::default_init_type_info_expr(&registry, ident, crate_)
        );

        (registry, init_type_info_expr)
    }
    
    fn default_init_type_info_expr(registry: &Type, ident: &Ident, crate_: &Path) -> Expr {
        let default_expr: proc_macro::TokenStream = quote! { 
            <#registry as #crate_::Registry>::TypeInfo::new::<#ident>()
        }.into();
        
        syn::parse(default_expr).expect("default expression is well-formed")
    }
}

impl Parse for RegisterAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let registry = Parse::parse(input)?;
        let comma: Option<Token![,]> = Parse::parse(input)?;
        let init_type_info_expr: Option<Expr> = if comma.is_some() && !input.is_empty() {
            Some(Parse::parse(input)?)
        } else {
            None
        };
        
        if !input.is_empty() {
            return Err(input.error("extra tokens at end of input"))
        }
        
        Ok(
            Self {
                registry,
                comma,
                init_type_info_expr
            }
        )
    }
}

impl ToTokens for RegisterAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.registry.to_tokens(tokens);
        if let Some(comma) = self.comma {
            comma.to_tokens(tokens);
        }
        if let Some(init_type_info_expr) = &self.init_type_info_expr {
            init_type_info_expr.to_tokens(tokens);
        }
    }
}