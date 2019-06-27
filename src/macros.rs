// #[macro_export]
macro_rules! enum_variant_from {
    ($target: ident, $variant: ident, $source: ident) => {
        impl From<$source> for $target {
            fn from(inner: $source) -> Self {
                $target::$variant(inner)
            }
        }
    };
}

// #[macro_export]
macro_rules! enum_variant_into_result {
    ($source: ident, $variant: ident, $target: ident) => {
        impl Into<Result<$target, $source>> for $source {
            fn into(self) -> Result<$target, $source> {
                match self {
                    $source::$variant(target) => Ok(target),
                    other => Err(other),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! include_json_as {
    ($deserialize_as: ident, $file: expr) => {
        serde_json::from_str::<$deserialize_as>(include_str!($file)).unwrap()
    };
}
