//! Represents logical type-registries built on top of the [raw registry](crate::raw::REGISTRY).

mod index;
pub use index::Index;

mod iter;
pub use iter::Iter;

mod registered;
pub use registered::Registered;

mod registration;
pub use registration::Registration;

mod registration_id;
pub use registration_id::RegistrationId;

mod registry;
pub use registry::Registry;

mod registry_entry;
pub use registry_entry::RegistryEntry;

mod registry_ext;
pub use registry_ext::RegistryExt;

mod type_info;
pub use type_info::TypeInfo;
