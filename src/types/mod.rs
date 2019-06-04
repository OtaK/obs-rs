#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ObsAlign {
    Center = libobs_sys::OBS_ALIGN_CENTER,
    Left = libobs_sys::OBS_ALIGN_LEFT,
    Right = libobs_sys::OBS_ALIGN_RIGHT,
    Top = libobs_sys::OBS_ALIGN_TOP,
    Bottom = libobs_sys::OBS_ALIGN_BOTTOM,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ObsOutputStatus {
    Success = libobs_sys::OBS_OUTPUT_SUCCESS as i32,
    BadPath = libobs_sys::OBS_OUTPUT_BAD_PATH,
    ConnectFailed = libobs_sys::OBS_OUTPUT_CONNECT_FAILED,
    InvalidStream = libobs_sys::OBS_OUTPUT_INVALID_STREAM,
    OutputError = libobs_sys::OBS_OUTPUT_ERROR,
    Disconnected = libobs_sys::OBS_OUTPUT_DISCONNECTED,
    Unsupported = libobs_sys::OBS_OUTPUT_UNSUPPORTED,
    NoSpace = libobs_sys::OBS_OUTPUT_NO_SPACE,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ObsVideoStatus {
    Success = libobs_sys::OBS_VIDEO_SUCCESS as i32,
    Fail = libobs_sys::OBS_VIDEO_FAIL,
    NotSupported = libobs_sys::OBS_VIDEO_NOT_SUPPORTED,
    InvalidParam = libobs_sys::OBS_VIDEO_INVALID_PARAM,
    CurrentlyActive = libobs_sys::OBS_VIDEO_CURRENTLY_ACTIVE,
    ModuleNotFound = libobs_sys::OBS_VIDEO_MODULE_NOT_FOUND,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ObsUiStatus {
    Success = libobs_sys::OBS_UI_SUCCESS as i32,
    Cancel = libobs_sys::OBS_UI_CANCEL,
    NotFound = libobs_sys::OBS_UI_NOTFOUND,
}
