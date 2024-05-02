use linkme::distributed_slice;
use crate::raw::RegistryEntry;

/// The distributed slice which contains all the [raw entries](RegistryEntry) for all logical
/// [registries](crate::logical::Registry). Each entry indicates to which 
/// [logical registry](crate::logical::Registry) it belongs.
#[distributed_slice]
pub static REGISTRY: [RegistryEntry] = [..];
