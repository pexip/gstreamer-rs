// This file was generated by gir (94e079d) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_base;
use gst_base_ffi;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AppSink(Object<ffi::GstAppSink>): [
        gst_base::BaseSink => gst_base_ffi::GstBaseSink,
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
        gst::URIHandler => gst_ffi::GstURIHandler,
    ];

    match fn {
        get_type => || ffi::gst_app_sink_get_type(),
    }
}

impl AppSink {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_buffer_list_support(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_get_buffer_list_support(self.to_glib_none().0))
        }
    }

    pub fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_get_caps(self.to_glib_none().0))
        }
    }

    pub fn get_drop(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_get_drop(self.to_glib_none().0))
        }
    }

    pub fn get_emit_signals(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_get_emit_signals(self.to_glib_none().0))
        }
    }

    pub fn get_max_buffers(&self) -> u32 {
        unsafe {
            ffi::gst_app_sink_get_max_buffers(self.to_glib_none().0)
        }
    }

    pub fn get_wait_on_eos(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_get_wait_on_eos(self.to_glib_none().0))
        }
    }

    pub fn is_eos(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_is_eos(self.to_glib_none().0))
        }
    }

    pub fn pull_preroll(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_pull_preroll(self.to_glib_none().0))
        }
    }

    pub fn pull_sample(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_pull_sample(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn set_buffer_list_support(&self, enable_lists: bool) {
        unsafe {
            ffi::gst_app_sink_set_buffer_list_support(self.to_glib_none().0, enable_lists.to_glib());
        }
    }

    //pub fn set_callbacks<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, callbacks: /*Ignored*/&mut AppSinkCallbacks, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_app_sink_set_callbacks() }
    //}

    pub fn set_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::gst_app_sink_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    pub fn set_drop(&self, drop: bool) {
        unsafe {
            ffi::gst_app_sink_set_drop(self.to_glib_none().0, drop.to_glib());
        }
    }

    pub fn set_emit_signals(&self, emit: bool) {
        unsafe {
            ffi::gst_app_sink_set_emit_signals(self.to_glib_none().0, emit.to_glib());
        }
    }

    pub fn set_max_buffers(&self, max: u32) {
        unsafe {
            ffi::gst_app_sink_set_max_buffers(self.to_glib_none().0, max);
        }
    }

    pub fn set_wait_on_eos(&self, wait: bool) {
        unsafe {
            ffi::gst_app_sink_set_wait_on_eos(self.to_glib_none().0, wait.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn try_pull_preroll(&self, timeout: gst::ClockTime) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_try_pull_preroll(self.to_glib_none().0, timeout))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn try_pull_sample(&self, timeout: gst::ClockTime) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_try_pull_sample(self.to_glib_none().0, timeout))
        }
    }

    pub fn get_property_buffer_list(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "buffer-list".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_buffer_list(&self, buffer_list: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "buffer-list".to_glib_none().0, Value::from(&buffer_list).to_glib_none().0);
        }
    }

    pub fn get_property_eos(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "eos".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn connect_eos<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "eos",
                transmute(eos_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_new_preroll<F: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "new-preroll",
                transmute(new_preroll_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_new_sample<F: Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "new-sample",
                transmute(new_sample_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_buffer_list_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer-list",
                transmute(notify_buffer_list_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_drop_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::drop",
                transmute(notify_drop_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_emit_signals_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::emit-signals",
                transmute(notify_emit_signals_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_eos_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::eos",
                transmute(notify_eos_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_max_buffers_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-buffers",
                transmute(notify_max_buffers_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_wait_on_eos_notify<F: Fn(&AppSink) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&AppSink) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wait-on-eos",
                transmute(notify_wait_on_eos_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for AppSink {}
unsafe impl Sync for AppSink {}

unsafe extern "C" fn eos_trampoline(this: *mut ffi::GstAppSink, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn new_preroll_trampoline(this: *mut ffi::GstAppSink, f: glib_ffi::gpointer) -> gst_ffi::GstFlowReturn {
    callback_guard!();
    let f: &&(Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this)).to_glib()
}

unsafe extern "C" fn new_sample_trampoline(this: *mut ffi::GstAppSink, f: glib_ffi::gpointer) -> gst_ffi::GstFlowReturn {
    callback_guard!();
    let f: &&(Fn(&AppSink) -> gst::FlowReturn + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this)).to_glib()
}

unsafe extern "C" fn notify_buffer_list_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_caps_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_drop_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_emit_signals_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_eos_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_max_buffers_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_wait_on_eos_trampoline(this: *mut ffi::GstAppSink, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&AppSink) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
