// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Reporter;
use crate::Runner;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstValidateMonitor")]
    pub struct Monitor(Object<ffi::GstValidateMonitor, ffi::GstValidateMonitorClass>) @extends gst::Object, @implements Reporter;

    match fn {
        type_ => || ffi::gst_validate_monitor_get_type(),
    }
}

impl Monitor {
    pub const NONE: Option<&'static Monitor> = None;

    #[doc(alias = "gst_validate_monitor_factory_create")]
    pub fn factory_create(
        target: &impl IsA<gst::Object>,
        runner: &impl IsA<Runner>,
        parent: Option<&impl IsA<Monitor>>,
    ) -> Monitor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_validate_monitor_factory_create(
                target.as_ref().to_glib_none().0,
                runner.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}

pub trait MonitorExt: 'static {
    //#[doc(alias = "gst_validate_monitor_attach_override")]
    //fn attach_override(&self, override_: /*Ignored*/&Override);

    #[doc(alias = "gst_validate_monitor_get_element")]
    #[doc(alias = "get_element")]
    fn element(&self) -> Option<gst::Element>;

    #[doc(alias = "gst_validate_monitor_get_element_name")]
    #[doc(alias = "get_element_name")]
    fn element_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_validate_monitor_get_pipeline")]
    #[doc(alias = "get_pipeline")]
    fn pipeline(&self) -> Option<gst::Pipeline>;

    #[doc(alias = "gst_validate_monitor_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Option<gst::Object>;

    //#[doc(alias = "gst_validate_monitor_set_media_descriptor")]
    //fn set_media_descriptor(&self, media_descriptor: /*Ignored*/&MediaDescriptor);

    fn object(&self) -> Option<glib::Object>;

    fn set_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: Option<&P>);

    #[doc(alias = "validate-parent")]
    fn validate_parent(&self) -> Option<Monitor>;

    //fn verbosity(&self) -> /*Ignored*/VerbosityFlags;

    //fn set_verbosity(&self, verbosity: /*Ignored*/VerbosityFlags);

    #[doc(alias = "pipeline")]
    fn connect_pipeline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "verbosity")]
    fn connect_verbosity_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Monitor>> MonitorExt for O {
    //fn attach_override(&self, override_: /*Ignored*/&Override) {
    //    unsafe { TODO: call ffi:gst_validate_monitor_attach_override() }
    //}

    fn element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::gst_validate_monitor_get_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn element_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_validate_monitor_get_element_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pipeline(&self) -> Option<gst::Pipeline> {
        unsafe {
            from_glib_full(ffi::gst_validate_monitor_get_pipeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn target(&self) -> Option<gst::Object> {
        unsafe {
            from_glib_full(ffi::gst_validate_monitor_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn set_media_descriptor(&self, media_descriptor: /*Ignored*/&MediaDescriptor) {
    //    unsafe { TODO: call ffi:gst_validate_monitor_set_media_descriptor() }
    //}

    fn object(&self) -> Option<glib::Object> {
        glib::ObjectExt::property(self.as_ref(), "object")
    }

    fn set_pipeline<P: IsA<gst::Pipeline>>(&self, pipeline: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "pipeline", &pipeline)
    }

    fn validate_parent(&self) -> Option<Monitor> {
        glib::ObjectExt::property(self.as_ref(), "validate-parent")
    }

    //fn verbosity(&self) -> /*Ignored*/VerbosityFlags {
    //    glib::ObjectExt::property(self.as_ref(), "verbosity")
    //}

    //fn set_verbosity(&self, verbosity: /*Ignored*/VerbosityFlags) {
    //    glib::ObjectExt::set_property(self.as_ref(),"verbosity", &verbosity)
    //}

    fn connect_pipeline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pipeline_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pipeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pipeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_verbosity_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_verbosity_trampoline<
            P: IsA<Monitor>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Monitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::verbosity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_verbosity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
