//! Models a [distributed slice](linkme::DistributedSlice) as a registry of types, with each
//! [entry](RegistryEntry) indicating to which [logical registry](crate::logical::Registry) it
//! belongs.

mod registration_id;
pub(crate) use registration_id::RegistrationId;

mod registry;
#[doc(hidden)]
pub use registry::REGISTRY;

mod registry_entry;
#[doc(hidden)]
pub use registry_entry::RegistryEntry;

mod registry_id;
pub use registry_id::RegistryId;

mod registry_info;
pub use registry_info::RegistryInfo;
