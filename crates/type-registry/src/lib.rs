//! A library for statically registering types.
//! 
//! E.g.:
//!
//! ```
//! use type_registry::{registration, Registered, Registration, Registry, RegistryExt};
//!
//! struct StringAnalyserTypeInfo {
//!     analyse: fn(&str) -> Option<usize>
//! }
//!
//! struct StringAnalyserRegistry;
//!
//! impl Registry for StringAnalyserRegistry {
//!     type TypeInfo = StringAnalyserTypeInfo;
//! 
//!     fn name() -> &'static str { 
//!         "String Analysers"
//!     }
//! }
//!
//! struct LenAnalyser;
//!
//! unsafe impl Registered<StringAnalyserRegistry> for LenAnalyser {
//!     fn register() -> Registration<StringAnalyserRegistry, Self> {
//!         // NOTE: Can't use generic 'Self' type here
//!         registration!(StringAnalyserRegistry, LenAnalyser)
//!     }
//!
//!     fn type_info() -> &'static StringAnalyserTypeInfo {
//!         static TYPE_INFO: StringAnalyserTypeInfo = StringAnalyserTypeInfo {
//!             analyse: |string| Some(string.len())
//!         };
//!
//!         &TYPE_INFO
//!     }
//! }
//!
//! struct NumFinderAnalyser;
//!
//! unsafe impl Registered<StringAnalyserRegistry> for NumFinderAnalyser {
//!     fn register() -> Registration<StringAnalyserRegistry, Self> {
//!         // NOTE: Can't use generic 'Self' type here
//!         registration!(StringAnalyserRegistry, NumFinderAnalyser)
//!     }
//!
//!     fn type_info() -> &'static StringAnalyserTypeInfo {
//!         fn is_digit(c: char) -> bool {
//!             c.is_ascii_digit()
//!         }
//!         static TYPE_INFO: StringAnalyserTypeInfo = StringAnalyserTypeInfo {
//!             analyse: |string| {
//!                 let start = string.find(is_digit)?;
//!                 let end = string.rfind(is_digit)?;
//!                 std::str::FromStr::from_str(&string[start..=end]).ok()
//!             }
//!         };
//!
//!         &TYPE_INFO
//!     }
//! }
//!
//! fn main() {
//!     for (_id, entry) in StringAnalyserRegistry::iter() {
//!         assert_eq!((entry.type_info().analyse)("I'm 22 characters long"), Some(22))
//!     }
//! }
//! ```
//!
//! If you want all types in a registry to implement some trait, you can statically ensure this
//! by making the [type-info](Registry::TypeInfo) only constructable for types that implement the
//! trait. Furthermore, if you require all implementors of the trait to be registered, you can
//! add a [Registered] bound to the trait.
//!
//! E.g. similar to the example above, but with a `StringAnalyser` trait:
//!
//! ```
//! use type_registry::{register, Registered, Registration, Registry, RegistryExt};
//!
//! trait StringAnalyser: Registered<StringAnalyserRegistry> {
//!     fn analyse(string: &str) -> Option<usize>;
//! }
//!
//! struct StringAnalyserTypeInfo {
//!     analyse: fn(&str) -> Option<usize>
//! }
//!
//! impl StringAnalyserTypeInfo {
//!     pub const fn new<T: StringAnalyser>() -> Self {
//!         Self {
//!             analyse: T::analyse
//!         }
//!     }
//! }
//!
//! struct StringAnalyserRegistry;
//!
//! impl Registry for StringAnalyserRegistry {
//!     type TypeInfo = StringAnalyserTypeInfo;
//!
//!     fn name() -> &'static str {
//!         "String Analysers"
//!     }
//! }
//!
//! #[register(StringAnalyserRegistry)]
//! struct LenAnalyser;
//!
//! impl StringAnalyser for LenAnalyser {
//!     fn analyse(string: &str) -> Option<usize> {
//!         Some(string.len())
//!     }
//! }
//!
//! #[register(StringAnalyserRegistry)]
//! struct NumFinderAnalyser;
//!
//! impl NumFinderAnalyser {
//!     fn is_digit(c: char) -> bool {
//!         c.is_ascii_digit()
//!     }
//! }
//!
//! impl StringAnalyser for NumFinderAnalyser {
//!     fn analyse(string: &str) -> Option<usize> {
//!         let start = string.find(Self::is_digit)?;
//!         let end = string.rfind(Self::is_digit)?;
//!         std::str::FromStr::from_str(&string[start..=end]).ok()
//!     }
//! }
//!
//! fn main() {
//!     for (_id, entry) in StringAnalyserRegistry::iter() {
//!         assert_eq!((entry.type_info().analyse)("I'm 22 characters long"), Some(22))
//!     }
//! }
//! ```

mod logical;
pub use logical::*;

#[doc(hidden)]
pub mod raw;
pub use raw::RegistryId;

#[doc(hidden)]
pub mod reexports {
    pub use linkme;
}

#[cfg(feature = "macro")]
pub use type_registry_register_macro::register;
