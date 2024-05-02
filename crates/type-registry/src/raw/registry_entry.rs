use crate::logical::Registered;
use crate::logical::Registry;
use crate::raw::registry_id::RegistryId;
use crate::TypeInfo;

/// The raw entry for a type registered to any [logical registry](Registry).
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegistryEntry {
    /// The [registry](Registry) to which the entry belongs.
    registry_id: RegistryId,
    /// Function which gets the [information](TypeInfo) about the [registered](Registered) type.
    get_type_info: fn() -> &'static dyn TypeInfo
}

impl RegistryEntry {
    /// Creates an entry for a [type](T) [registered](Registered) to a given [registry](R).
    pub const fn new<
        R: Registry + ?Sized,
        T: Registered<R> + ?Sized
    >() -> Self {
        Self {
            registry_id: RegistryId::of::<R>(),
            get_type_info: || T::type_info()
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
}
