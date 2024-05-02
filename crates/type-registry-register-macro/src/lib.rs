mod register_input;
mod register_attr;
mod crate_attribute;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use crate::register_attr::RegisterAttr;
use crate::register_input::RegisterInput;

/// Implements the Registered trait for a type and registers the type with the given
/// type-registry. Can be applied to structs, enums and unions.
///
/// ```
/// use std::fmt::Debug;
/// use type_registry::Registry;
/// use type_registry_register_macro::register;
///
/// struct MyTypeInfo {
///     very_important_info: usize
/// }
///
/// impl MyTypeInfo {
///     // The macro assumes that the type-info type has a const, no-argument `new` fn like this,
///     // which is generic on the type being registered (the generic parameter can have bounds,
///     // like the implicit `Sized` bound here.
///     pub const fn new<T>() -> Self {
///         Self {
///             very_important_info: std::mem::size_of::<T>()
///         }
///     }
///
///     // If this is not the case, or custom initialisation is needed for some other reason, the
///     // initialisation can be overridden.
///     pub const fn new_unsized(number: usize) -> Self {
///         Self {
///             very_important_info: number
///         }
///     }
/// }
///
/// struct MyRegistry;
///
/// impl Registry for MyRegistry {
///     type TypeInfo = MyTypeInfo;
///
///     fn name() -> &'static str {
///         "My Registry"
///     }
/// }
///
/// #[register(MyRegistry, MyTypeInfo::new_unsized(42))]
/// struct MyStruct([u8]);
///
/// #[register(MyRegistry)]
/// enum MyEnum {}
///
/// pub mod reexport {
///     pub use type_registry as type_registry_reexported;
/// }
///
/// #[register(MyRegistry)]
/// // If the macro has been exported from a non-standard path, use this attribute to customise it
/// #[type_registry(crate = reexport::type_registry_reexported)]
/// union MyUnion { u8: u8, u16: u16 }
/// ```
#[proc_macro_attribute]
pub fn register(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as RegisterInput);

    let generics = input.generics();
    let generic_tokens = quote! { #generics };
    if !generic_tokens.is_empty() {
        return syn::Error::new_spanned(
            generic_tokens,
            "registration does not support generics"
        ).into_compile_error().into()
    }

    let crate_ = match input.crate_() {
        Ok(path) => path,
        Err(error) => return error.into_compile_error().into()
    };
    let ident = input.ident();

    let attr = parse_macro_input!(attr as RegisterAttr);

    let (
        registry,
        init_type_info_expr
    ) = attr.into_parts(ident, &crate_);

    quote!(
        #input
        
        impl #crate_::Registered<#registry> for #ident {
            fn register() -> #crate_::Registration<#registry, Self> {
                 #crate_::registration!(#registry, #ident)
            }

            fn type_info() -> &'static <#registry as #crate_::Registry>::TypeInfo {
                 static TYPE_INFO: <#registry as #crate_::Registry>::TypeInfo = #init_type_info_expr;
                 &TYPE_INFO
            }
        }
    ).into()
}
