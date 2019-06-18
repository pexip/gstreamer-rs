// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gst;
use gst_pbutils_sys;
use DiscovererInfo;

glib_wrapper! {
    pub struct EncodingProfile(Object<gst_pbutils_sys::GstEncodingProfile, gst_pbutils_sys::GstEncodingProfileClass, EncodingProfileClass>);

    match fn {
        get_type => || gst_pbutils_sys::gst_encoding_profile_get_type(),
    }
}

impl EncodingProfile {
    pub fn find(
        targetname: &str,
        profilename: Option<&str>,
        category: Option<&str>,
    ) -> Option<EncodingProfile> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_find(
                targetname.to_glib_none().0,
                profilename.to_glib_none().0,
                category.to_glib_none().0,
            ))
        }
    }

    pub fn from_discoverer(info: &DiscovererInfo) -> Option<EncodingProfile> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_from_discoverer(
                info.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for EncodingProfile {}
unsafe impl Sync for EncodingProfile {}

pub const NONE_ENCODING_PROFILE: Option<&EncodingProfile> = None;

pub trait EncodingProfileExt: 'static {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn copy(&self) -> EncodingProfile;

    fn get_allow_dynamic_output(&self) -> bool;

    fn get_description(&self) -> Option<GString>;

    fn get_file_extension(&self) -> Option<GString>;

    fn get_format(&self) -> gst::Caps;

    fn get_input_caps(&self) -> gst::Caps;

    fn get_name(&self) -> Option<GString>;

    fn get_presence(&self) -> u32;

    fn get_preset(&self) -> Option<GString>;

    fn get_preset_name(&self) -> Option<GString>;

    fn get_restriction(&self) -> Option<gst::Caps>;

    fn get_type_nick(&self) -> Option<GString>;

    fn is_enabled(&self) -> bool;

    fn is_equal<P: IsA<EncodingProfile>>(&self, b: &P) -> bool;
}

impl<O: IsA<EncodingProfile>> EncodingProfileExt for O {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn copy(&self) -> EncodingProfile {
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_copy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_allow_dynamic_output(&self) -> bool {
        unsafe {
            from_glib(
                gst_pbutils_sys::gst_encoding_profile_get_allow_dynamic_output(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_file_extension(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_file_extension(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_format(&self) -> gst::Caps {
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_get_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_input_caps(&self) -> gst::Caps {
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_get_input_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_presence(&self) -> u32 {
        unsafe {
            gst_pbutils_sys::gst_encoding_profile_get_presence(self.as_ref().to_glib_none().0)
        }
    }

    fn get_preset(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_preset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_preset_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_preset_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_restriction(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(gst_pbutils_sys::gst_encoding_profile_get_restriction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_type_nick(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_pbutils_sys::gst_encoding_profile_get_type_nick(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(gst_pbutils_sys::gst_encoding_profile_is_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_equal<P: IsA<EncodingProfile>>(&self, b: &P) -> bool {
        unsafe {
            from_glib(gst_pbutils_sys::gst_encoding_profile_is_equal(
                self.as_ref().to_glib_none().0,
                b.as_ref().to_glib_none().0,
            ))
        }
    }
}
