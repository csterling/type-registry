use crate::raw::{REGISTRY, RegistryEntry, RegistryId};

/// An identifier of a single [registered](crate::logical::Registered) type (in any logical
/// [registry](crate::logical::Registry)). Directly corresponds to the type's index in the
/// [distributed slice](REGISTRY).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegistrationId {
    /// The index of the [registered](crate::logical::Registered) type in the
    /// [distributed slice](REGISTRY).
    index: usize
}

impl RegistrationId {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }
    
    #[inline]
    pub(crate) fn index(self) -> usize {
        self.index
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn registry(self) -> RegistryId {
        self.entry().registry_id()
    }

    #[inline]
    pub(crate) fn entry(self) -> &'static RegistryEntry {
        &REGISTRY[self.index]
    }
}
