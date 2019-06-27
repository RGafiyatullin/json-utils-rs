
#[macro_export]
macro_rules! enum_variant_from {
    ($target: ident, $variant: ident, $source: ident) => {
        impl From<$source> for $target {
            fn from(inner: $source) -> Self {
                $target::$variant(inner)
            }
        }
    };
}
