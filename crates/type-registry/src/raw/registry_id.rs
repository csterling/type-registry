use std::hash::{Hash, Hasher};
use crate::logical::Registry;
use crate::raw::RegistryInfo;

/// Identifier of a particular [type-registry](Registry).
#[derive(Copy, Clone, Debug, Eq)]
pub struct RegistryId {
    /// Function which gets the [information](RegistryInfo) about the identified
    /// [registry](Registry). This function is unique for each type of registry, so its address is
    /// used as the identifier internally.
    get_registry_info: fn() -> RegistryInfo
}

impl RegistryId {
    /// Gets the ID for a particular [registry](Registry).
    pub const fn of<R: Registry + ?Sized>() -> Self {
        Self {
            get_registry_info: RegistryInfo::of::<R>
        }
    }
    
    /// Gets the [information](RegistryInfo) about the identified [registry](Registry).
    pub fn info(&self) -> RegistryInfo {
        (self.get_registry_info)()
    }
}

impl PartialEq for RegistryId {
    fn eq(&self, other: &Self) -> bool {
        self.info().type_id() == other.info().type_id()
    }
}

impl Hash for RegistryId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info().type_id().hash(state)
    }
}
