// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst;
use gst_gl_sys;
use GLDisplay;

glib_wrapper! {
    pub struct GLDisplayWayland(Object<gst_gl_sys::GstGLDisplayWayland, gst_gl_sys::GstGLDisplayWaylandClass, GLDisplayWaylandClass>) @extends GLDisplay, gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_display_wayland_get_type(),
    }
}

impl GLDisplayWayland {
    pub fn new(name: Option<&str>) -> GLDisplayWayland {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_display_wayland_new(
                name.to_glib_none().0,
            ))
        }
    }

    //pub fn new_with_display(display: /*Unimplemented*/Option<Fundamental: Pointer>) -> GLDisplayWayland {
    //    unsafe { TODO: call gst_gl_sys:gst_gl_display_wayland_new_with_display() }
    //}
}

unsafe impl Send for GLDisplayWayland {}
unsafe impl Sync for GLDisplayWayland {}
