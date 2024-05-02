use crate::logical::registration::Registration;
use crate::logical::registry::Registry;

/// Trait which marks a type as registered with a particular [registry](Registry).
pub trait Registered<R: Registry + ?Sized> {
    /// Register the type with the [registry](Registry). Should internally use the
    /// [registration! macro (click for example)](crate::registration).
    fn register() -> Registration<R, Self>;

    /// Statically allocate and return the [information](crate::logical::TypeInfo) about the registered type.
    fn type_info() -> &'static <R as Registry>::TypeInfo;
}
