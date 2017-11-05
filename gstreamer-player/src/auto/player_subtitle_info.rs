// This file was generated by gir (94e079d) from gir-files (???)
// DO NOT EDIT

use PlayerStreamInfo;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerSubtitleInfo(Object<ffi::GstPlayerSubtitleInfo>): PlayerStreamInfo;

    match fn {
        get_type => || ffi::gst_player_subtitle_info_get_type(),
    }
}

impl PlayerSubtitleInfo {
    pub fn get_language(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_player_subtitle_info_get_language(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for PlayerSubtitleInfo {}
unsafe impl Sync for PlayerSubtitleInfo {}
