// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstBaseSrc")]
    pub struct BaseSrc(Object<ffi::GstBaseSrc, ffi::GstBaseSrcClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_base_src_get_type(),
    }
}

impl BaseSrc {
    pub const NONE: Option<&'static BaseSrc> = None;
}

unsafe impl Send for BaseSrc {}
unsafe impl Sync for BaseSrc {}

pub trait BaseSrcExt: 'static {
    #[doc(alias = "gst_base_src_get_blocksize")]
    #[doc(alias = "get_blocksize")]
    fn blocksize(&self) -> u32;

    #[doc(alias = "gst_base_src_get_buffer_pool")]
    #[doc(alias = "get_buffer_pool")]
    fn buffer_pool(&self) -> Option<gst::BufferPool>;

    #[doc(alias = "gst_base_src_get_do_timestamp")]
    #[doc(alias = "get_do_timestamp")]
    fn does_timestamp(&self) -> bool;

    #[doc(alias = "gst_base_src_is_async")]
    fn is_async(&self) -> bool;

    #[doc(alias = "gst_base_src_is_live")]
    fn is_live(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_base_src_negotiate")]
    fn negotiate(&self) -> bool;

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "gst_base_src_new_seamless_segment")]
    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool;

    #[doc(alias = "gst_base_src_set_async")]
    fn set_async(&self, async_: bool);

    #[doc(alias = "gst_base_src_set_automatic_eos")]
    fn set_automatic_eos(&self, automatic_eos: bool);

    #[doc(alias = "gst_base_src_set_blocksize")]
    fn set_blocksize(&self, blocksize: u32);

    #[doc(alias = "gst_base_src_set_caps")]
    fn set_caps(&self, caps: &gst::Caps) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_base_src_set_do_timestamp")]
    fn set_do_timestamp(&self, timestamp: bool);

    #[doc(alias = "gst_base_src_set_dynamic_size")]
    fn set_dynamic_size(&self, dynamic: bool);

    #[doc(alias = "gst_base_src_set_format")]
    fn set_format(&self, format: gst::Format);

    #[doc(alias = "gst_base_src_set_live")]
    fn set_live(&self, live: bool);

    #[doc(alias = "gst_base_src_start_complete")]
    fn start_complete(&self, ret: impl Into<gst::FlowReturn>);

    #[doc(alias = "gst_base_src_start_wait")]
    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_base_src_wait_playing")]
    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "num-buffers")]
    fn num_buffers(&self) -> i32;

    #[doc(alias = "num-buffers")]
    fn set_num_buffers(&self, num_buffers: i32);

    fn is_typefind(&self) -> bool;

    fn set_typefind(&self, typefind: bool);

    #[doc(alias = "blocksize")]
    fn connect_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "do-timestamp")]
    fn connect_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "num-buffers")]
    fn connect_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "typefind")]
    fn connect_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseSrc>> BaseSrcExt for O {
    fn blocksize(&self) -> u32 {
        unsafe { ffi::gst_base_src_get_blocksize(self.as_ref().to_glib_none().0) }
    }

    fn buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_base_src_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn does_timestamp(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_get_do_timestamp(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_async(&self) -> bool {
        unsafe { from_glib(ffi::gst_base_src_is_async(self.as_ref().to_glib_none().0)) }
    }

    fn is_live(&self) -> bool {
        unsafe { from_glib(ffi::gst_base_src_is_live(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn negotiate(&self) -> bool {
        unsafe { from_glib(ffi::gst_base_src_negotiate(self.as_ref().to_glib_none().0)) }
    }

    #[allow(deprecated)]
    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_base_src_new_seamless_segment(
                self.as_ref().to_glib_none().0,
                start,
                stop,
                time,
            ))
        }
    }

    fn set_async(&self, async_: bool) {
        unsafe {
            ffi::gst_base_src_set_async(self.as_ref().to_glib_none().0, async_.into_glib());
        }
    }

    fn set_automatic_eos(&self, automatic_eos: bool) {
        unsafe {
            ffi::gst_base_src_set_automatic_eos(
                self.as_ref().to_glib_none().0,
                automatic_eos.into_glib(),
            );
        }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            ffi::gst_base_src_set_blocksize(self.as_ref().to_glib_none().0, blocksize);
        }
    }

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_base_src_set_caps(self.as_ref().to_glib_none().0, caps.to_glib_none().0),
                "Failed to set caps"
            )
        }
    }

    fn set_do_timestamp(&self, timestamp: bool) {
        unsafe {
            ffi::gst_base_src_set_do_timestamp(
                self.as_ref().to_glib_none().0,
                timestamp.into_glib(),
            );
        }
    }

    fn set_dynamic_size(&self, dynamic: bool) {
        unsafe {
            ffi::gst_base_src_set_dynamic_size(self.as_ref().to_glib_none().0, dynamic.into_glib());
        }
    }

    fn set_format(&self, format: gst::Format) {
        unsafe {
            ffi::gst_base_src_set_format(self.as_ref().to_glib_none().0, format.into_glib());
        }
    }

    fn set_live(&self, live: bool) {
        unsafe {
            ffi::gst_base_src_set_live(self.as_ref().to_glib_none().0, live.into_glib());
        }
    }

    fn start_complete(&self, ret: impl Into<gst::FlowReturn>) {
        unsafe {
            ffi::gst_base_src_start_complete(
                self.as_ref().to_glib_none().0,
                ret.into().into_glib(),
            );
        }
    }

    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe { try_from_glib(ffi::gst_base_src_start_wait(self.as_ref().to_glib_none().0)) }
    }

    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_base_src_wait_playing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn num_buffers(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "num-buffers")
    }

    fn set_num_buffers(&self, num_buffers: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "num-buffers", &num_buffers)
    }

    fn is_typefind(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "typefind")
    }

    fn set_typefind(&self, typefind: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "typefind", &typefind)
    }

    fn connect_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocksize_trampoline<
            P: IsA<BaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocksize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_blocksize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_do_timestamp_trampoline<
            P: IsA<BaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::do-timestamp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_do_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_num_buffers_trampoline<
            P: IsA<BaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::num-buffers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_num_buffers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_typefind_trampoline<
            P: IsA<BaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::typefind\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_typefind_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
