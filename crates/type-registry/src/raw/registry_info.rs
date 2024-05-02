use std::any::TypeId;
use crate::{Registry, RegistryId};

/// Information about a particular [type-registry](Registry).
#[derive(Copy, Clone, Debug, Eq)]
pub struct RegistryInfo {
    /// The [ID](RegistryId) of the [registry](Registry).
    id: RegistryId,
    /// Function which returns the [TypeId] of the [registry](Registry). Currently used in-place of
    /// the [TypeId] directly as [TypeId::of] is const-unstable.
    type_id: fn() -> TypeId,
    /// The [name](Registry::NAME) of the [registry](Registry).
    name: fn() -> &'static str
}

impl RegistryInfo {
    /// Gets the information about a particular [registry](R).
    pub const fn of<R: Registry + ?Sized>() -> Self {
        Self {
            id: RegistryId::of::<R>(),
            type_id: TypeId::of::<R>,
            name: R::name
        }
    }
    
    /// The [ID](RegistryId) of the [registry](Registry).
    pub const fn id(&self) -> RegistryId {
        self.id
    }

    /// The [TypeId] of the [registry](Registry).
    pub fn type_id(&self) -> TypeId {
        (self.type_id)()
    }

    /// The [name](Registry::NAME) of the [registry](Registry).
    pub fn name(&self) -> &'static str {
        (self.name)()
    }
}

impl PartialEq for RegistryInfo {
    fn eq(&self, other: &Self) -> bool {
        &self.id == &other.id
    }
}

