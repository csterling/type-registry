use std::any::Any;
use std::marker::PhantomData;
use crate::logical::registry::Registry;
use crate::raw::{RegistryEntry as RawRegistryEntry};
use crate::raw::RegistryId;
use crate::{RegistrationId};

/// An entry describing a type registered to a [registry](Registry).
#[derive(Copy, Clone)]
pub struct RegistryEntry<R: Registry + ?Sized> {
    /// The entry's corresponding raw entry
    raw_entry: &'static RawRegistryEntry,
    /// Marker of the relevant registry
    registry: PhantomData<fn(R)>
}

impl<R: Registry + ?Sized> RegistryEntry<R> {
    pub(crate) fn new(raw_entry: &'static RawRegistryEntry) -> Self {
        assert_eq!(
            raw_entry.registry_id(),
            RegistryId::of::<R>(),
            "registry mismatch"
        );

        // SAFETY: Above assertion
        unsafe { Self::new_unchecked(raw_entry) }
    }

    /// SAFETY: raw_entry must be an entry for R
    pub(crate) unsafe fn new_unchecked(raw_entry: &'static RawRegistryEntry) -> Self {
        RegistryEntry {
            raw_entry,
            registry: PhantomData
        }
    }

    pub(crate) fn raw(&self) -> &'static RawRegistryEntry {
        self.raw_entry
    }

    /// Gets the [ID](RegistrationId) of this entry.
    pub fn registration_id(&self) -> RegistrationId<R> {
        // SAFETY: Self's invariant that raw_entry is for R
        unsafe {
            RegistrationId::from_raw_entry_unchecked(self.raw_entry)
        }
    }

    /// Gets the [type-info](Registry::TypeInfo) that was provided for this entry.
    pub fn type_info(&self) -> &'static R::TypeInfo {
        let any = self.raw_entry.type_info().as_any() as &dyn Any;
        any.downcast_ref().expect("protected by generics")
    }
}
