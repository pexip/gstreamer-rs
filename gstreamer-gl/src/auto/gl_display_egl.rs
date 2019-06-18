// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst;
use gst_gl_sys;
use GLDisplay;

glib_wrapper! {
    pub struct GLDisplayEGL(Object<gst_gl_sys::GstGLDisplayEGL, gst_gl_sys::GstGLDisplayEGLClass, GLDisplayEGLClass>) @extends GLDisplay, gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_display_egl_get_type(),
    }
}

impl GLDisplayEGL {
    pub fn new() -> GLDisplayEGL {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_gl_sys::gst_gl_display_egl_new()) }
    }

    //pub fn new_with_egl_display(display: /*Unimplemented*/Option<Fundamental: Pointer>) -> GLDisplayEGL {
    //    unsafe { TODO: call gst_gl_sys:gst_gl_display_egl_new_with_egl_display() }
    //}

    //pub fn get_from_native(type_: GLDisplayType, display: /*Unimplemented*/Fundamental: UIntPtr) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gst_gl_sys:gst_gl_display_egl_get_from_native() }
    //}
}

impl Default for GLDisplayEGL {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLDisplayEGL {}
unsafe impl Sync for GLDisplayEGL {}
