use crate::ObsError;
use bitflags::bitflags;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum SourceType {
    Input = libobs_sys::obs_source_type_OBS_SOURCE_TYPE_INPUT,
    Filter = libobs_sys::obs_source_type_OBS_SOURCE_TYPE_FILTER,
    Transition = libobs_sys::obs_source_type_OBS_SOURCE_TYPE_TRANSITION,
}

impl From<libobs_sys::obs_source_type> for SourceType {
    fn from(value: libobs_sys::obs_source_type) -> Self {
        match value {
            libobs_sys::obs_source_type_OBS_SOURCE_TYPE_INPUT => SourceType::Input,
            libobs_sys::obs_source_type_OBS_SOURCE_TYPE_FILTER => SourceType::Filter,
            libobs_sys::obs_source_type_OBS_SOURCE_TYPE_TRANSITION => SourceType::Transition,
            _ => panic!("Encountered an unknown source type"),
        }
    }
}

bitflags! {
    pub struct SourceCapabilityFlags: u32 {
        const OBS_SOURCE_VIDEO = libobs_sys::OBS_SOURCE_VIDEO;
        const OBS_SOURCE_AUDIO = libobs_sys::OBS_SOURCE_AUDIO;
        const OBS_SOURCE_ASYNC = libobs_sys::OBS_SOURCE_ASYNC;
        const OBS_SOURCE_ASYNC_VIDEO = libobs_sys::OBS_SOURCE_ASYNC_VIDEO;
        const OBS_SOURCE_CUSTOM_DRAW = libobs_sys::OBS_SOURCE_CUSTOM_DRAW;
        const OBS_SOURCE_INTERACTION = libobs_sys::OBS_SOURCE_INTERACTION;
        const OBS_SOURCE_COMPOSITE = libobs_sys::OBS_SOURCE_COMPOSITE;
        const OBS_SOURCE_DO_NOT_DUPLICATE = libobs_sys::OBS_SOURCE_DO_NOT_DUPLICATE;
        const OBS_SOURCE_DEPRECATED = libobs_sys::OBS_SOURCE_DEPRECATED;
        const OBS_SOURCE_DO_NOT_SELF_MONITOR = libobs_sys::OBS_SOURCE_DO_NOT_SELF_MONITOR;
    }
}

pub type SourceOutputFlags = SourceCapabilityFlags;

pub trait SourceDelegate: std::fmt::Debug {
    fn id(&self) -> &'static str;
    fn source_type(&self) -> SourceType;
    fn output_flags(&self) -> SourceOutputFlags;

    fn get_name(&self) -> &str;
    fn create(settings: (), source: libobs_sys::obs_source_t);
    fn destroy();

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn get_defaults();
}

#[derive(Debug)]
pub struct Source {
    inner: libobs_sys::obs_source_info,
    id: String,
    source_type: SourceType,
    output_flags: SourceCapabilityFlags,
}

impl TryFrom<libobs_sys::obs_source_info> for Source {
    type Error = ObsError;
    fn try_from(s: libobs_sys::obs_source_info) -> Result<Self, Self::Error> {
        if s.id.is_null() {
            return Err(ObsError::FfiNullPointer);
        }

        let id = unsafe { std::ffi::CString::from_raw(s.id as *mut i8).into_string()? }.into();
        let source_type = s.type_.into();
        let output_flags = SourceCapabilityFlags::from_bits_truncate(s.output_flags);

        Ok(Source {
            inner: s,
            id,
            source_type,
            output_flags,
        })
    }
}

impl Source {
    pub fn create() -> Result<Self, ObsError> {
        unimplemented!()
    }
}
