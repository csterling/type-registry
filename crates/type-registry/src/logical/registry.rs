use crate::TypeInfo;

/// A logical registry of types.
pub trait Registry: 'static {
    /// The type of [information](TypeInfo) that needs to be provided for each registered type.
    type TypeInfo: TypeInfo + Sized;

    /// A name for the registry. Only used for informational purposes.
    fn name() -> &'static str;
}
