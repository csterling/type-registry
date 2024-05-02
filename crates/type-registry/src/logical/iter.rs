use std::marker::PhantomData;
use crate::logical::registry::Registry;
use crate::logical::registration_id::RegistrationId;
use crate::logical::registry_entry::RegistryEntry;
use crate::raw::{REGISTRY, RegistryEntry as RawRegistryEntry, RegistryId};

/// Iterator over the entries in a [registry](Registry).
pub struct Iter<R: Registry + ?Sized> {
    /// The iterator over all [entries](RawRegistryEntry) across all [registries](Registry).
    raw_iter: std::iter::Enumerate<std::slice::Iter<'static, RawRegistryEntry>>,
    /// Keeps track of the next entry's position in its logical [registry](Registry).
    next_index: usize,
    /// Marker of the [registry](Registry) being iterated over.
    registry: PhantomData<fn(R)>,
}

impl<R: Registry + ?Sized> Iter<R> {
    /// Creates a new iterator over the [entries](RegistryEntry) (and their associated
    /// [IDs](RegistrationId)) in a [registry](Registry).
    pub fn new() -> Self {
        Self {
            raw_iter: REGISTRY.iter().enumerate(),
            next_index: 0,
            registry: PhantomData
        }
    }
}

impl<R: Registry + ?Sized> Iterator for Iter<R> {
    type Item = (RegistrationId<R>, RegistryEntry<R>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (raw_index, next_raw) = self.raw_iter.next()?;
            
            if next_raw.registry_id() == RegistryId::of::<R>() {
                let registry_index = self.next_index;
                self.next_index += 1;
                
                return Some((
                    RegistrationId::new(raw_index, registry_index),
                    RegistryEntry::new(next_raw)
                ))
            }
        }
    }
}