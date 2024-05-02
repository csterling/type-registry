use std::collections::HashMap;
use std::ops::ControlFlow;
use crate::logical::registration_id::RegistrationId;
use crate::logical::registry::Registry;
use crate::logical::registry_entry::RegistryEntry;
use crate::raw::{RegistryEntry as RawRegistryEntry};

/// A static index over a [registry](Registry).
pub trait Index<R: Registry + ?Sized>: 'static {
    /// The type used to store the index.
    type Storage: 'static
        + Sync
        + Send;

    /// Allocates the storage (initially empty).
    fn allocate() -> Self::Storage;

    /// Inserts a registration into the index. The return value indicates whether to continue
    /// building the index or to early-abort.
    fn associate(
        storage: &mut Self::Storage,
        id: RegistrationId<R>,
        entry: RegistryEntry<R>
    ) -> ControlFlow<()>;
}

impl<R: Registry + ?Sized> Index<R> for RawRegistryEntry {
    type Storage = HashMap<&'static RawRegistryEntry, RegistrationId<R>>;

    fn allocate() -> Self::Storage {
        HashMap::new()
    }

    fn associate(
        storage: &mut Self::Storage,
        id: RegistrationId<R>,
        entry: RegistryEntry<R>
    ) -> ControlFlow<()> {
        storage.insert(entry.raw(), id);
        ControlFlow::Continue(())
    }
}
