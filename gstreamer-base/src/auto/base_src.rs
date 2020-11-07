// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_base_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct BaseSrc(Object<gst_base_sys::GstBaseSrc, gst_base_sys::GstBaseSrcClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || gst_base_sys::gst_base_src_get_type(),
    }
}

unsafe impl Send for BaseSrc {}
unsafe impl Sync for BaseSrc {}

pub const NONE_BASE_SRC: Option<&BaseSrc> = None;

pub trait BaseSrcExt: 'static {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams);

    fn get_blocksize(&self) -> u32;

    fn get_buffer_pool(&self) -> Option<gst::BufferPool>;

    fn get_do_timestamp(&self) -> bool;

    fn is_async(&self) -> bool;

    fn is_live(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn negotiate(&self) -> bool;

    #[cfg_attr(feature = "v1_18", deprecated)]
    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool;

    fn set_async(&self, async: bool);

    fn set_automatic_eos(&self, automatic_eos: bool);

    fn set_blocksize(&self, blocksize: u32);

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), glib::error::BoolError>;

    fn set_do_timestamp(&self, timestamp: bool);

    fn set_dynamic_size(&self, dynamic: bool);

    fn set_format(&self, format: gst::Format);

    fn set_live(&self, live: bool);

    fn get_property_num_buffers(&self) -> i32;

    fn set_property_num_buffers(&self, num_buffers: i32);

    fn get_property_typefind(&self) -> bool;

    fn set_property_typefind(&self, typefind: bool);

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseSrc>> BaseSrcExt for O {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call gst_base_sys:gst_base_src_get_allocator() }
    //}

    fn get_blocksize(&self) -> u32 {
        unsafe { gst_base_sys::gst_base_src_get_blocksize(self.as_ref().to_glib_none().0) }
    }

    fn get_buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(gst_base_sys::gst_base_src_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_do_timestamp(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_src_get_do_timestamp(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_async(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_src_is_async(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_live(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_src_is_live(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn negotiate(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_src_negotiate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn new_seamless_segment(&self, start: i64, stop: i64, time: i64) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_src_new_seamless_segment(
                self.as_ref().to_glib_none().0,
                start,
                stop,
                time,
            ))
        }
    }

    fn set_async(&self, async: bool) {
        unsafe {
            gst_base_sys::gst_base_src_set_async(self.as_ref().to_glib_none().0, async.to_glib());
        }
    }

    fn set_automatic_eos(&self, automatic_eos: bool) {
        unsafe {
            gst_base_sys::gst_base_src_set_automatic_eos(
                self.as_ref().to_glib_none().0,
                automatic_eos.to_glib(),
            );
        }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            gst_base_sys::gst_base_src_set_blocksize(self.as_ref().to_glib_none().0, blocksize);
        }
    }

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_base_sys::gst_base_src_set_caps(
                    self.as_ref().to_glib_none().0,
                    caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    fn set_do_timestamp(&self, timestamp: bool) {
        unsafe {
            gst_base_sys::gst_base_src_set_do_timestamp(
                self.as_ref().to_glib_none().0,
                timestamp.to_glib(),
            );
        }
    }

    fn set_dynamic_size(&self, dynamic: bool) {
        unsafe {
            gst_base_sys::gst_base_src_set_dynamic_size(
                self.as_ref().to_glib_none().0,
                dynamic.to_glib(),
            );
        }
    }

    fn set_format(&self, format: gst::Format) {
        unsafe {
            gst_base_sys::gst_base_src_set_format(self.as_ref().to_glib_none().0, format.to_glib());
        }
    }

    fn set_live(&self, live: bool) {
        unsafe {
            gst_base_sys::gst_base_src_set_live(self.as_ref().to_glib_none().0, live.to_glib());
        }
    }

    fn get_property_num_buffers(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"num-buffers\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `num-buffers` getter")
                .unwrap()
        }
    }

    fn set_property_num_buffers(&self, num_buffers: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"num-buffers\0".as_ptr() as *const _,
                Value::from(&num_buffers).to_glib_none().0,
            );
        }
    }

    fn get_property_typefind(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"typefind\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `typefind` getter")
                .unwrap()
        }
    }

    fn set_property_typefind(&self, typefind: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"typefind\0".as_ptr() as *const _,
                Value::from(&typefind).to_glib_none().0,
            );
        }
    }

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocksize_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSrc>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_do_timestamp_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSrc>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_num_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_num_buffers_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSrc>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_typefind_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_typefind_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSrc,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSrc>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSrc::from_glib_borrow(this).unsafe_cast_ref())
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
