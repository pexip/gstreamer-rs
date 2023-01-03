// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstPlayerStreamInfo")]
    pub struct PlayerStreamInfo(Object<ffi::GstPlayerStreamInfo, ffi::GstPlayerStreamInfoClass>);

    match fn {
        type_ => || ffi::gst_player_stream_info_get_type(),
    }
}

impl PlayerStreamInfo {
    pub const NONE: Option<&'static PlayerStreamInfo> = None;
}

unsafe impl Send for PlayerStreamInfo {}
unsafe impl Sync for PlayerStreamInfo {}

pub trait PlayerStreamInfoExt: 'static {
    #[doc(alias = "gst_player_stream_info_get_caps")]
    #[doc(alias = "get_caps")]
    fn caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "gst_player_stream_info_get_codec")]
    #[doc(alias = "get_codec")]
    fn codec(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_player_stream_info_get_index")]
    #[doc(alias = "get_index")]
    fn index(&self) -> i32;

    #[doc(alias = "gst_player_stream_info_get_stream_type")]
    #[doc(alias = "get_stream_type")]
    fn stream_type(&self) -> glib::GString;

    #[doc(alias = "gst_player_stream_info_get_tags")]
    #[doc(alias = "get_tags")]
    fn tags(&self) -> Option<gst::TagList>;
}

impl<O: IsA<PlayerStreamInfo>> PlayerStreamInfoExt for O {
    fn caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_caps(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    fn codec(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_codec(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    fn index(&self) -> i32 {
        unsafe {
            ffi::gst_player_stream_info_get_index(const_override(self.as_ref().to_glib_none().0))
        }
    }

    fn stream_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_stream_type(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    fn tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_player_stream_info_get_tags(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }
}
