// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gst;
use gst_gl_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;
use Error;
use GLContext;
use GLDisplayType;
use GLWindow;
use GLAPI;

glib_wrapper! {
    pub struct GLDisplay(Object<gst_gl_sys::GstGLDisplay, gst_gl_sys::GstGLDisplayClass, GLDisplayClass>) @extends gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_display_get_type(),
    }
}

impl GLDisplay {
    pub fn new() -> GLDisplay {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_gl_sys::gst_gl_display_new()) }
    }
}

impl Default for GLDisplay {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLDisplay {}
unsafe impl Sync for GLDisplay {}

pub const NONE_GL_DISPLAY: Option<&GLDisplay> = None;

pub trait GLDisplayExt: 'static {
    fn add_context<P: IsA<GLContext>>(&self, context: &P) -> Result<(), glib::error::BoolError>;

    fn create_context<P: IsA<GLContext>>(&self, other_context: &P) -> Result<GLContext, Error>;

    fn create_window(&self) -> Option<GLWindow>;

    fn filter_gl_api(&self, gl_api: GLAPI);

    fn get_gl_api(&self) -> GLAPI;

    fn get_gl_api_unlocked(&self) -> GLAPI;

    fn get_handle_type(&self) -> GLDisplayType;

    fn remove_window<P: IsA<GLWindow>>(&self, window: &P) -> Result<(), glib::error::BoolError>;

    fn connect_create_context<F: Fn(&Self, &GLContext) -> GLContext + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<GLDisplay>> GLDisplayExt for O {
    fn add_context<P: IsA<GLContext>>(&self, context: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_gl_sys::gst_gl_display_add_context(
                    self.as_ref().to_glib_none().0,
                    context.as_ref().to_glib_none().0
                ),
                "Failed to add OpenGL context"
            )
        }
    }

    fn create_context<P: IsA<GLContext>>(&self, other_context: &P) -> Result<GLContext, Error> {
        unsafe {
            let mut p_context = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = gst_gl_sys::gst_gl_display_create_context(
                self.as_ref().to_glib_none().0,
                other_context.as_ref().to_glib_none().0,
                &mut p_context,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(p_context))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_window(&self) -> Option<GLWindow> {
        unsafe {
            from_glib_full(gst_gl_sys::gst_gl_display_create_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn filter_gl_api(&self, gl_api: GLAPI) {
        unsafe {
            gst_gl_sys::gst_gl_display_filter_gl_api(
                self.as_ref().to_glib_none().0,
                gl_api.to_glib(),
            );
        }
    }

    fn get_gl_api(&self) -> GLAPI {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_display_get_gl_api(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_gl_api_unlocked(&self) -> GLAPI {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_display_get_gl_api_unlocked(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_handle_type(&self) -> GLDisplayType {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_display_get_handle_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_window<P: IsA<GLWindow>>(&self, window: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_gl_sys::gst_gl_display_remove_window(
                    self.as_ref().to_glib_none().0,
                    window.as_ref().to_glib_none().0
                ),
                "Failed to remove window"
            )
        }
    }

    fn connect_create_context<F: Fn(&Self, &GLContext) -> GLContext + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_context_trampoline<
            P,
            F: Fn(&P, &GLContext) -> GLContext + Send + Sync + 'static,
        >(
            this: *mut gst_gl_sys::GstGLDisplay,
            context: *mut gst_gl_sys::GstGLContext,
            f: glib_sys::gpointer,
        ) -> *mut gst_gl_sys::GstGLContext
        where
            P: IsA<GLDisplay>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GLDisplay::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(context),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-context\0".as_ptr() as *const _,
                Some(transmute(create_context_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}
