// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Clip;
use Container;
use Extractable;
use TimelineElement;

glib_wrapper! {
    pub struct UriClip(Object<ges_sys::GESUriClip, ges_sys::GESUriClipClass, UriClipClass>) @extends Clip, Container, TimelineElement, @implements Extractable;

    match fn {
        get_type => || ges_sys::ges_uri_clip_get_type(),
    }
}

impl UriClip {
    pub fn new(uri: &str) -> Option<UriClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ges_sys::ges_uri_clip_new(uri.to_glib_none().0)) }
    }
}

pub const NONE_URI_CLIP: Option<&UriClip> = None;

pub trait UriClipExt: 'static {
    fn get_uri(&self) -> Option<GString>;

    fn is_image(&self) -> bool;

    fn is_muted(&self) -> bool;

    fn set_is_image(&self, is_image: bool);

    fn set_mute(&self, mute: bool);

    fn get_property_is_image(&self) -> bool;

    fn get_property_mute(&self) -> bool;

    fn connect_property_is_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_supported_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<UriClip>> UriClipExt for O {
    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ges_sys::ges_uri_clip_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_image(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_uri_clip_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_muted(&self) -> bool {
        unsafe {
            from_glib(ges_sys::ges_uri_clip_is_muted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_is_image(&self, is_image: bool) {
        unsafe {
            ges_sys::ges_uri_clip_set_is_image(self.as_ref().to_glib_none().0, is_image.to_glib());
        }
    }

    fn set_mute(&self, mute: bool) {
        unsafe {
            ges_sys::ges_uri_clip_set_mute(self.as_ref().to_glib_none().0, mute.to_glib());
        }
    }

    fn get_property_is_image(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"is-image\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn get_property_mute(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mute\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_property_is_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_image_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESUriClip,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UriClip>,
        {
            let f: &F = &*(f as *const F);
            f(&UriClip::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-image\0".as_ptr() as *const _,
                Some(transmute(notify_is_image_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESUriClip,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UriClip>,
        {
            let f: &F = &*(f as *const F);
            f(&UriClip::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mute\0".as_ptr() as *const _,
                Some(transmute(notify_mute_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_supported_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_formats_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESUriClip,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UriClip>,
        {
            let f: &F = &*(f as *const F);
            f(&UriClip::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-formats\0".as_ptr() as *const _,
                Some(transmute(
                    notify_supported_formats_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
