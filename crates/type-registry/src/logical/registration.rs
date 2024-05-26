use std::marker::PhantomData;
use crate::logical::registered::Registered;
use crate::logical::registry::Registry;
use crate::raw::RegistryEntry;

/// Existence of this type indicates that registration has occurred.
pub struct Registration<
    R: Registry + ?Sized,
    T: Registered<R> + ?Sized
> {
    #[allow(dead_code)]
    raw_entry: &'static RegistryEntry,
    registry: PhantomData<fn(R)>,
    marker: PhantomData<fn(T) -> T>
}

impl<
    R: Registry + ?Sized,
    T: Registered<R> + ?Sized
> Registration<R, T> {
    /// Creates a new registration.
    /// 
    /// SAFETY: [raw_entry] must have been created with the same types for [R]/[T].
    #[doc(hidden)]
    pub const unsafe fn new(raw_entry: &'static RegistryEntry) -> Self {
        Registration {
            raw_entry,
            registry: PhantomData,
            marker: PhantomData,
        }
    }
}

/// Should be used to implement [Registered::register].
/// 
/// For example:
/// ```
/// use type_registry::{registration, Registered, Registration, Registry};
/// 
/// struct MyRegistry;
///
/// impl Registry for MyRegistry {
///     type TypeInfo = ();
/// 
///     fn name() -> &'static str {
///         "My Registry"
///     }
/// }
///
/// struct MyType;
///
/// unsafe impl Registered<MyRegistry> for MyType {
///     fn register() -> Registration<MyRegistry, Self> {
///         registration!(MyRegistry, MyType)
///     }
///
///     fn type_info() -> &'static <MyRegistry as Registry>::TypeInfo {
///         todo!()
///     }
/// }
///
///
/// ```
#[macro_export]
macro_rules! registration {
    ($registry_type:ty, $registered_type:ty) => {
        {
            use $crate::reexports::linkme::distributed_slice;
            use $crate::raw::{RegistryEntry, REGISTRY};
            use $crate::{Registration};

            #[distributed_slice(REGISTRY)]
            #[linkme(crate=$crate::reexports::linkme)]
            static REGISTRATION: RegistryEntry = RegistryEntry::new::<$registry_type, $registered_type>();

            // SAFETY: Created with same types immediately above.
            unsafe { Registration::<$registry_type, $registered_type>::new(&REGISTRATION) }
        }
    };
}