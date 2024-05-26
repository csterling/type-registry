use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::logical::{Registered, RegistryEntry, RegistryExt};
use crate::logical::registry::Registry;
use crate::raw::{RegistrationId as RawRegistrationId, RegistryEntry as RawRegistryEntry};

/// Identifies a [registered](Registered) type in a particular [registry](Registry).
pub struct RegistrationId<R: Registry + ?Sized> {
    /// The registration's raw identifier 
    raw_id: RawRegistrationId,
    /// The position of the type's registration within the particular registry
    registry_index: usize,
    /// Marker of the registry to which the type is registered
    registry: PhantomData<fn(R)>
}

impl<R: Registry + ?Sized> RegistrationId<R> {
    pub(crate) fn new(
        raw_index: usize,
        registry_index: usize
    ) -> Self {
        Self {
            raw_id: RawRegistrationId::new(raw_index),
            registry_index,
            registry: PhantomData
        }
    }

    #[allow(dead_code)]
    pub(crate) fn raw(self) -> RawRegistrationId {
        self.raw_id
    }
    
    /// The position of the identified type in the registry.
    pub fn index(self) -> usize {
        self.registry_index
    }

    /// Gets the 
    pub fn entry(self) -> RegistryEntry<R> {
        // SAFETY: raw_id is for R
        unsafe { RegistryEntry::new_unchecked(self.raw_id.entry()) }
    }

    pub fn metadata(self) -> &'static R::TypeInfo {
        self.entry().type_info()
    }

    pub fn of<T: Registered<R> + ?Sized>() -> Self {
        // SAFETY: T is registered to R
        unsafe {
            Self::from_type_id_unchecked(TypeId::of::<T>())
        }
    }

    /// SAFETY: raw_entry must be for R
    pub(crate) unsafe fn from_raw_entry_unchecked(raw_entry: &RawRegistryEntry) -> Self {
        Self::from_type_id_unchecked(raw_entry.type_id())
    }

    /// SAFETY: type_id must be a type registered to R
    pub(crate) unsafe fn from_type_id_unchecked(type_id: TypeId) -> Self {
        *R::index::<RawRegistryEntry>().get(&type_id).expect("type is registered")
    }
}

// Have to manually derive Copy/Clone, as the derive macro requires a Sized bound.

impl<R: Registry + ?Sized> Clone for RegistrationId<R> {
    fn clone(&self) -> Self {
        Self::new(self.raw_id.index(), self.registry_index)
    }
}

impl<R: Registry + ?Sized> Copy for RegistrationId<R> {}

// Have to manually derive PartialEq, Eq, Debug, Display and Hash in presence of PhantomData.
// Although the implementation of these traits for PhantomData<R> in no way depends on
// implementations for R, the derive macro requires that R does implement them. We don't want to
// burden implementations of Registry with implementing these traits (as all implementations of
// Registry are ZSTs and therefore there is no non-trivial implementation).

impl<R: Registry + ?Sized> Debug for RegistrationId<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: Const formatting of type name?
        f.debug_struct(&format!("RegistrationId<{}>", std::any::type_name::<R>()))
            .field("raw_id", &self.raw_id)
            .field("registry_index", &self.registry_index)
            .field("registry", &R::name())
            .finish()
    }
}

impl<R: Registry + ?Sized> PartialEq for RegistrationId<R> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_id == other.raw_id
    }
}

impl<R: Registry + ?Sized> Eq for RegistrationId<R> {}

impl<R: Registry + ?Sized> Hash for RegistrationId<R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw_id.hash(state)
    }
}

impl<R: Registry + ?Sized> Display for RegistrationId<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let index = self.registry_index;
        let name = R::name();
        f.write_fmt(format_args!("'{name}' registration ID #{index}"))
    }
}

impl<R: Registry + ?Sized> From<RegistrationId<R>> for RawRegistrationId {
    fn from(value: RegistrationId<R>) -> Self {
        value.raw_id
    }
}

impl<R: Registry + ?Sized> PartialEq<RawRegistrationId> for RegistrationId<R> {
    fn eq(&self, other: &RawRegistrationId) -> bool {
        &self.raw_id == other
    }
}

impl<R: Registry + ?Sized> PartialEq<RegistrationId<R>> for RawRegistrationId {
    fn eq(&self, other: &RegistrationId<R>) -> bool {
        self == &other.raw_id
    }
}

impl<R: Registry + ?Sized> Borrow<RawRegistrationId> for RegistrationId<R> {
    fn borrow(&self) -> &RawRegistrationId {
        &self.raw_id
    }
}
