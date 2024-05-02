use std::any::Any;
use std::ops::ControlFlow;
use std::sync::OnceLock;
use generic_static::StaticTypeMap;
use crate::logical::index::Index;
use crate::logical::Iter;
use crate::logical::registry::Registry;

/// Extension methods for [registries](Registry).
pub trait RegistryExt: Registry {
    /// Iterates over the entries in a [registry](Registry).
    fn iter() -> Iter<Self>;

    /// Accesses an [index](Index) associated with a [registry](Registry).
    fn index<I: Index<Self>>() -> &'static I::Storage;
}

impl<R: Registry + ?Sized> RegistryExt for R {
    fn iter() -> Iter<Self> {
        Iter::new()
    }
    
    fn index<I: Index<Self>>() -> &'static I::Storage {
        static STORAGE_TYPE_MAP: OnceLock<StaticTypeMap<Box<dyn Any + Send + Sync>>> = OnceLock::new();
        let storage_type_map = STORAGE_TYPE_MAP.get_or_init(|| StaticTypeMap::new());

        let any = storage_type_map.call_once::<(fn(R), I), _>(
            || {
                let mut storage = I::allocate();
                
                for (id, entry) in R::iter() {
                    match I::associate(&mut storage, id, entry) {
                        ControlFlow::Continue(_) => continue,
                        ControlFlow::Break(_) => break
                    }
                }
                
                Box::new(storage)
            }
        );

        any.downcast_ref().expect("index storage is associated to type")
    }
}
