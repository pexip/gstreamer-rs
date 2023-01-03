// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    Clip, Container, Extractable, MetaContainer, SourceClip, TextHAlign, TextVAlign,
    TimelineElement,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GESTitleClip")]
    pub struct TitleClip(Object<ffi::GESTitleClip, ffi::GESTitleClipClass>) @extends SourceClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_title_clip_get_type(),
    }
}

impl TitleClip {
    pub const NONE: Option<&'static TitleClip> = None;

    #[doc(alias = "ges_title_clip_new")]
    pub fn new() -> Option<TitleClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_title_clip_new()) }
    }
}

pub trait TitleClipExt: 'static {
    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_background_color")]
    #[doc(alias = "get_background_color")]
    fn background_color(&self) -> u32;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    fn font_desc(&self) -> Option<glib::GString>;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_halignment")]
    #[doc(alias = "get_halignment")]
    fn halignment(&self) -> TextHAlign;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString>;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_text_color")]
    #[doc(alias = "get_text_color")]
    fn text_color(&self) -> u32;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_valignment")]
    #[doc(alias = "get_valignment")]
    fn valignment(&self) -> TextVAlign;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_xpos")]
    #[doc(alias = "get_xpos")]
    fn xpos(&self) -> f64;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_get_ypos")]
    #[doc(alias = "get_ypos")]
    fn ypos(&self) -> f64;

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_background")]
    fn set_background(&self, background: u32);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_color")]
    fn set_color(&self, color: u32);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_font_desc")]
    fn set_font_desc(&self, font_desc: Option<&str>);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_halignment")]
    fn set_halignment(&self, halign: TextHAlign);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_text")]
    fn set_text(&self, text: Option<&str>);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_valignment")]
    fn set_valignment(&self, valign: TextVAlign);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_xpos")]
    fn set_xpos(&self, position: f64);

    #[deprecated = "Since 1.6"]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_clip_set_ypos")]
    fn set_ypos(&self, position: f64);

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    fn background(&self) -> u32;

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    fn color(&self) -> u32;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "background")]
    fn connect_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "color")]
    fn connect_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "font-desc")]
    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "halignment")]
    fn connect_halignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "valignment")]
    fn connect_valignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "xpos")]
    fn connect_xpos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated = "Since 1.6"]
    #[doc(alias = "ypos")]
    fn connect_ypos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TitleClip>> TitleClipExt for O {
    #[allow(deprecated)]
    fn background_color(&self) -> u32 {
        unsafe { ffi::ges_title_clip_get_background_color(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn font_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_title_clip_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn halignment(&self) -> TextHAlign {
        unsafe {
            from_glib(ffi::ges_title_clip_get_halignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn text(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::ges_title_clip_get_text(self.as_ref().to_glib_none().0)) }
    }

    #[allow(deprecated)]
    fn text_color(&self) -> u32 {
        unsafe { ffi::ges_title_clip_get_text_color(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn valignment(&self) -> TextVAlign {
        unsafe {
            from_glib(ffi::ges_title_clip_get_valignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn xpos(&self) -> f64 {
        unsafe { ffi::ges_title_clip_get_xpos(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn ypos(&self) -> f64 {
        unsafe { ffi::ges_title_clip_get_ypos(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn set_background(&self, background: u32) {
        unsafe {
            ffi::ges_title_clip_set_background(self.as_ref().to_glib_none().0, background);
        }
    }

    #[allow(deprecated)]
    fn set_color(&self, color: u32) {
        unsafe {
            ffi::ges_title_clip_set_color(self.as_ref().to_glib_none().0, color);
        }
    }

    #[allow(deprecated)]
    fn set_font_desc(&self, font_desc: Option<&str>) {
        unsafe {
            ffi::ges_title_clip_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_halignment(&self, halign: TextHAlign) {
        unsafe {
            ffi::ges_title_clip_set_halignment(self.as_ref().to_glib_none().0, halign.into_glib());
        }
    }

    #[allow(deprecated)]
    fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::ges_title_clip_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[allow(deprecated)]
    fn set_valignment(&self, valign: TextVAlign) {
        unsafe {
            ffi::ges_title_clip_set_valignment(self.as_ref().to_glib_none().0, valign.into_glib());
        }
    }

    #[allow(deprecated)]
    fn set_xpos(&self, position: f64) {
        unsafe {
            ffi::ges_title_clip_set_xpos(self.as_ref().to_glib_none().0, position);
        }
    }

    #[allow(deprecated)]
    fn set_ypos(&self, position: f64) {
        unsafe {
            ffi::ges_title_clip_set_ypos(self.as_ref().to_glib_none().0, position);
        }
    }

    fn background(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "background")
    }

    fn color(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "color")
    }

    fn connect_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<
            P: IsA<TitleClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<P: IsA<TitleClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<P: IsA<TitleClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_halignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_halignment_trampoline<
            P: IsA<TitleClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::halignment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_halignment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P: IsA<TitleClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_valignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_valignment_trampoline<
            P: IsA<TitleClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::valignment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_valignment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_xpos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpos_trampoline<P: IsA<TitleClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xpos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ypos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypos_trampoline<P: IsA<TitleClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTitleClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TitleClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ypos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
