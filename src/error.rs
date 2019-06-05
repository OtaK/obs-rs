use failure_derive::Fail;

macro_rules! from_error {
    ($type:ty, $target:ident, $targetvar:expr) => {
        impl From<$type> for $target {
            fn from(s: $type) -> Self {
                $targetvar(s.into())
            }
        }
    };
}

#[derive(Debug, Fail)]
pub enum ObsError {
    #[fail(display = "Couldn't convert unsafe string into Rust string: {}", _0)]
    FfiStringError(std::ffi::IntoStringError),
    #[fail(display = "Encountered a null pointer across the FFI boundary")]
    FfiNullPointer,
    #[fail(display = "Unknown error")]
    Unknown,
}

from_error!(
    std::ffi::IntoStringError,
    ObsError,
    ObsError::FfiStringError
);
