// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::RTPHeaderExtension;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstRTPBaseDepayload")]
    pub struct RTPBaseDepayload(Object<ffi::GstRTPBaseDepayload, ffi::GstRTPBaseDepayloadClass>) @extends gst::Element;

    match fn {
        type_ => || ffi::gst_rtp_base_depayload_get_type(),
    }
}

impl RTPBaseDepayload {
    pub const NONE: Option<&'static RTPBaseDepayload> = None;
}

unsafe impl Send for RTPBaseDepayload {}
unsafe impl Sync for RTPBaseDepayload {}

pub trait RTPBaseDepayloadExt: 'static {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_depayload_is_source_info_enabled")]
    fn is_source_info_enabled(&self) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_depayload_set_source_info_enabled")]
    fn set_source_info_enabled(&self, enable: bool);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn is_auto_header_extension(&self) -> bool;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn set_auto_header_extension(&self, auto_header_extension: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "max-reorder")]
    fn max_reorder(&self) -> i32;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "max-reorder")]
    fn set_max_reorder(&self, max_reorder: i32);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn is_source_info(&self) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn set_source_info(&self, source_info: bool);

    fn stats(&self) -> Option<gst::Structure>;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "add-extension")]
    fn connect_add_extension<F: Fn(&Self, &RTPHeaderExtension) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn emit_add_extension(&self, ext: &RTPHeaderExtension);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "clear-extensions")]
    fn connect_clear_extensions<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn emit_clear_extensions(&self);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "request-extension")]
    fn connect_request_extension<
        F: Fn(&Self, u32, Option<&str>) -> Option<RTPHeaderExtension> + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn connect_auto_header_extension_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "max-reorder")]
    fn connect_max_reorder_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn connect_source_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "stats")]
    fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTPBaseDepayload>> RTPBaseDepayloadExt for O {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn is_source_info_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_base_depayload_is_source_info_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn set_source_info_enabled(&self, enable: bool) {
        unsafe {
            ffi::gst_rtp_base_depayload_set_source_info_enabled(
                self.as_ref().to_glib_none().0,
                enable.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn is_auto_header_extension(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "auto-header-extension")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn set_auto_header_extension(&self, auto_header_extension: bool) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "auto-header-extension",
            &auto_header_extension,
        )
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn max_reorder(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "max-reorder")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_max_reorder(&self, max_reorder: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "max-reorder", &max_reorder)
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn is_source_info(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "source-info")
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn set_source_info(&self, source_info: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "source-info", &source_info)
    }

    fn stats(&self) -> Option<gst::Structure> {
        glib::ObjectExt::property(self.as_ref(), "stats")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_add_extension<F: Fn(&Self, &RTPHeaderExtension) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_extension_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P, &RTPHeaderExtension) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            ext: *mut ffi::GstRTPHeaderExtension,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_full(ext),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-extension\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    add_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn emit_add_extension(&self, ext: &RTPHeaderExtension) {
        self.emit_by_name::<()>("add-extension", &[&ext]);
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_clear_extensions<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn clear_extensions_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clear-extensions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clear_extensions_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn emit_clear_extensions(&self) {
        self.emit_by_name::<()>("clear-extensions", &[]);
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_request_extension<
        F: Fn(&Self, u32, Option<&str>) -> Option<RTPHeaderExtension> + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn request_extension_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P, u32, Option<&str>) -> Option<RTPHeaderExtension> + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            ext_id: libc::c_uint,
            ext_uri: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::GstRTPHeaderExtension {
            let f: &F = &*(f as *const F);
            f(
                RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref(),
                ext_id,
                Option::<glib::GString>::from_glib_borrow(ext_uri)
                    .as_ref()
                    .as_ref()
                    .map(|s| s.as_str()),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"request-extension\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    request_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_auto_header_extension_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_header_extension_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-header-extension\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_header_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_max_reorder_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_reorder_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-reorder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_reorder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn connect_source_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_info_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::source-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_source_info_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stats_trampoline<
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
