// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gst_pbutils_sys;
use DiscovererStreamInfo;

glib_wrapper! {
    pub struct DiscovererAudioInfo(Object<gst_pbutils_sys::GstDiscovererAudioInfo>) @extends DiscovererStreamInfo;

    match fn {
        get_type => || gst_pbutils_sys::gst_discoverer_audio_info_get_type(),
    }
}

impl DiscovererAudioInfo {
    pub fn get_bitrate(&self) -> u32 {
        unsafe { gst_pbutils_sys::gst_discoverer_audio_info_get_bitrate(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_channel_mask(&self) -> u64 {
        unsafe {
            gst_pbutils_sys::gst_discoverer_audio_info_get_channel_mask(self.to_glib_none().0)
        }
    }

    pub fn get_channels(&self) -> u32 {
        unsafe { gst_pbutils_sys::gst_discoverer_audio_info_get_channels(self.to_glib_none().0) }
    }

    pub fn get_depth(&self) -> u32 {
        unsafe { gst_pbutils_sys::gst_discoverer_audio_info_get_depth(self.to_glib_none().0) }
    }

    pub fn get_language(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_discoverer_audio_info_get_language(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_max_bitrate(&self) -> u32 {
        unsafe { gst_pbutils_sys::gst_discoverer_audio_info_get_max_bitrate(self.to_glib_none().0) }
    }

    pub fn get_sample_rate(&self) -> u32 {
        unsafe { gst_pbutils_sys::gst_discoverer_audio_info_get_sample_rate(self.to_glib_none().0) }
    }
}

unsafe impl Send for DiscovererAudioInfo {}
unsafe impl Sync for DiscovererAudioInfo {}
