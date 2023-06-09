// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::SourceClip;
use crate::TimelineElement;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GESUriClip")]
    pub struct UriClip(Object<ffi::GESUriClip, ffi::GESUriClipClass>) @extends SourceClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_uri_clip_get_type(),
    }
}

impl UriClip {
    pub const NONE: Option<&'static UriClip> = None;

    #[doc(alias = "ges_uri_clip_new")]
    pub fn new(uri: &str) -> Result<UriClip, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_uri_clip_new(uri.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to create Uri clip from Uri"))
        }
    }
}

pub trait UriClipExt: 'static {
    #[doc(alias = "ges_uri_clip_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> glib::GString;

    #[doc(alias = "ges_uri_clip_is_image")]
    fn is_image(&self) -> bool;

    #[doc(alias = "ges_uri_clip_is_muted")]
    fn is_muted(&self) -> bool;

    #[doc(alias = "ges_uri_clip_set_is_image")]
    fn set_is_image(&self, is_image: bool);

    #[doc(alias = "ges_uri_clip_set_mute")]
    fn set_mute(&self, mute: bool);

    #[doc(alias = "is-image")]
    fn connect_is_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mute")]
    fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UriClip>> UriClipExt for O {
    fn uri(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::ges_uri_clip_get_uri(self.as_ref().to_glib_none().0)) }
    }

    fn is_image(&self) -> bool {
        unsafe { from_glib(ffi::ges_uri_clip_is_image(self.as_ref().to_glib_none().0)) }
    }

    fn is_muted(&self) -> bool {
        unsafe { from_glib(ffi::ges_uri_clip_is_muted(self.as_ref().to_glib_none().0)) }
    }

    fn set_is_image(&self, is_image: bool) {
        unsafe {
            ffi::ges_uri_clip_set_is_image(self.as_ref().to_glib_none().0, is_image.into_glib());
        }
    }

    fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::ges_uri_clip_set_mute(self.as_ref().to_glib_none().0, mute.into_glib());
        }
    }

    fn connect_is_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_image_trampoline<P: IsA<UriClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESUriClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UriClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-image\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_image_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<P: IsA<UriClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESUriClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UriClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mute\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mute_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
