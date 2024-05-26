use std::any::{type_name, TypeId};
use crate::logical::Registered;
use crate::logical::Registry;
use crate::raw::registry_id::RegistryId;
use crate::TypeInfo;

/// The raw entry for a type registered to any [logical registry](Registry).
#[derive(Copy, Clone)]
pub struct RegistryEntry {
    /// The [registry](Registry) to which the entry belongs.
    registry_id: RegistryId,
    /// Function which gets the [information](TypeInfo) about the [registered](Registered) type.
    get_type_info: fn() -> &'static dyn TypeInfo,
    /// Gets the [TypeId] of the [registered](Registered) type.
    get_type_id: fn() -> TypeId,
    /// Gets the [name](type_name) of the [registered](Registered) type.
    get_type_name: fn() -> &'static str
}

impl RegistryEntry {
    /// Creates an entry for a [type](T) [registered](Registered) to a given [registry](R).
    pub const fn new<
        R: Registry + ?Sized,
        T: Registered<R> + ?Sized
    >() -> Self {
        Self {
            registry_id: RegistryId::of::<R>(),
            get_type_info: || <T as Registered<R>>::type_info(),
            get_type_id: TypeId::of::<T>,
            get_type_name: type_name::<T>
        }
    }

    /// Gets the [ID](RegistryId) of the [registry](Registry) to which the type was
    /// [registered](Registered).
    pub const fn registry_id(&self) -> RegistryId {
        self.registry_id
    }

    /// Gets the [information](TypeInfo) about the [registered](Registered) type.
    pub fn type_info(&self) -> &'static dyn TypeInfo {
        (self.get_type_info)()
    }

    /// Gets the [TypeId] of the [registered](Registered) type.
    pub fn type_id(&self) -> TypeId {
        (self.get_type_id)()
    }

    /// Gets the [name](type_name) of the [registered](Registered) type.
    pub fn type_name(&self) -> &'static str {
        (self.get_type_name)()
    }
}
