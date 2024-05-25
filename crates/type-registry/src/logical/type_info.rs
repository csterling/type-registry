use std::any::Any;

/// Information that describes a type registered in a [registry](crate::logical::Registry).
pub trait TypeInfo: Any + Sync {
    fn as_any(&self) -> &(dyn Any + Sync);
}

// Blanket implementation for all Sized TypeInfo types to convert to a &(dyn Any + Sync). This 
// leaves the possibility of other implementations for other types T: TypeInfo + ?Sized, but as the
// Registry trait requires Self::TypeInfo: TypeInfo + Sized, these other implementations cannot
// be used with this crate. The notable exception being dyn TypeInfo, but its implementation is
// deferring to this one as required.

impl<T: Any + Sync> TypeInfo for T {
    fn as_any(&self) -> &(dyn Any + Sync) {
        self
    }
}
