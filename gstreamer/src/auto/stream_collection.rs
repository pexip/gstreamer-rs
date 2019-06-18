// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Object;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;

glib_wrapper! {
    pub struct StreamCollection(Object<gst_sys::GstStreamCollection, gst_sys::GstStreamCollectionClass, StreamCollectionClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_size(&self) -> u32 {
        unsafe { gst_sys::gst_stream_collection_get_size(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(gst_sys::gst_stream_collection_get_stream(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_upstream_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_sys::gst_stream_collection_get_upstream_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_upstream_id(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"upstream-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    pub fn set_property_upstream_id(&self, upstream_id: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"upstream-id\0".as_ptr() as *const _,
                Value::from(upstream_id).to_glib_none().0,
            );
        }
    }

    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}

    pub fn connect_property_upstream_id_notify<F: Fn(&StreamCollection) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_upstream_id_trampoline<
            F: Fn(&StreamCollection) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstStreamCollection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::upstream-id\0".as_ptr() as *const _,
                Some(transmute(notify_upstream_id_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}
