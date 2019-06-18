// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use gobject_sys;
use gst;
use gst_gl_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::mem::transmute;
use GLContext;

glib_wrapper! {
    pub struct GLOverlayCompositor(Object<gst_gl_sys::GstGLOverlayCompositor, gst_gl_sys::GstGLOverlayCompositorClass, GLOverlayCompositorClass>) @extends gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_overlay_compositor_get_type(),
    }
}

impl GLOverlayCompositor {
    pub fn new<P: IsA<GLContext>>(context: &P) -> GLOverlayCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(gst_gl_sys::gst_gl_overlay_compositor_new(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn draw_overlays(&self) {
        unsafe {
            gst_gl_sys::gst_gl_overlay_compositor_draw_overlays(self.to_glib_none().0);
        }
    }

    pub fn free_overlays(&self) {
        unsafe {
            gst_gl_sys::gst_gl_overlay_compositor_free_overlays(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_property_yinvert(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"yinvert\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_property_yinvert(&self, yinvert: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"yinvert\0".as_ptr() as *const _,
                Value::from(&yinvert).to_glib_none().0,
            );
        }
    }

    pub fn add_caps(caps: &gst::Caps) -> Option<gst::Caps> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_overlay_compositor_add_caps(
                caps.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn connect_property_yinvert_notify<F: Fn(&GLOverlayCompositor) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_yinvert_trampoline<
            F: Fn(&GLOverlayCompositor) + Send + Sync + 'static,
        >(
            this: *mut gst_gl_sys::GstGLOverlayCompositor,
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
                b"notify::yinvert\0".as_ptr() as *const _,
                Some(transmute(notify_yinvert_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for GLOverlayCompositor {}
unsafe impl Sync for GLOverlayCompositor {}
