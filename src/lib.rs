pub mod types;

pub const LIBOBS_API: (u32, u32, u32) = (
    libobs_sys::LIBOBS_API_MAJOR_VER,
    libobs_sys::LIBOBS_API_MINOR_VER,
    libobs_sys::LIBOBS_API_PATCH_VER,
);
