// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Extractable;
use Timeline;
use TrackType;

glib_wrapper! {
    pub struct TimelineElement(Object<ges_sys::GESTimelineElement, ges_sys::GESTimelineElementClass, TimelineElementClass>) @implements Extractable;

    match fn {
        get_type => || ges_sys::ges_timeline_element_get_type(),
    }
}

pub const NONE_TIMELINE_ELEMENT: Option<&TimelineElement> = None;

pub trait TimelineElementExt: 'static {
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool;

    fn copy(&self, deep: bool) -> Option<TimelineElement>;

    //fn get_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn get_child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool;

    //fn get_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value);

    //fn get_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn get_duration(&self) -> gst::ClockTime;

    fn get_inpoint(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_layer_priority(&self) -> u32;

    fn get_max_duration(&self) -> gst::ClockTime;

    fn get_name(&self) -> Option<GString>;

    fn get_parent(&self) -> Option<TimelineElement>;

    fn get_priority(&self) -> u32;

    fn get_start(&self) -> gst::ClockTime;

    fn get_timeline(&self) -> Option<Timeline>;

    fn get_toplevel_parent(&self) -> Option<TimelineElement>;

    fn get_track_types(&self) -> TrackType;

    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec>;

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object>;

    fn paste(&self, paste_position: gst::ClockTime) -> Option<TimelineElement>;

    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool;

    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool;

    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value);

    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn set_duration(&self, duration: gst::ClockTime) -> bool;

    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool;

    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool;

    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError>;

    fn set_parent<P: IsA<TimelineElement>>(&self, parent: &P)
        -> Result<(), glib::error::BoolError>;

    fn set_priority(&self, priority: u32) -> bool;

    fn set_start(&self, start: gst::ClockTime) -> bool;

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError>;

    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    fn get_property_in_point(&self) -> u64;

    fn set_property_in_point(&self, in_point: u64);

    fn get_property_serialize(&self) -> bool;

    fn set_property_serialize(&self, serialize: bool);

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TimelineElement>> TimelineElementExt for O {
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_add_child_property() }
    //}

    fn copy(&self, deep: bool) -> Option<TimelineElement> {
        unsafe {
            from_glib_none(ges_sys::ges_timeline_element_copy(
                self.as_ref().to_glib_none().0,
                deep.to_glib(),
            ))
        }
    }

    //fn get_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_get_child_properties() }
    //}

    //fn get_child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_get_child_property() }
    //}

    //fn get_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_get_child_property_by_pspec() }
    //}

    //fn get_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_get_child_property_valist() }
    //}

    fn get_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_get_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_inpoint(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_get_inpoint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_layer_priority(&self) -> u32 {
        unsafe { ges_sys::ges_timeline_element_get_layer_priority(self.as_ref().to_glib_none().0) }
    }

    fn get_max_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ges_sys::ges_timeline_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ges_sys::ges_timeline_element_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_priority(&self) -> u32 {
        unsafe { ges_sys::ges_timeline_element_get_priority(self.as_ref().to_glib_none().0) }
    }

    fn get_start(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_get_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_full(ges_sys::ges_timeline_element_get_timeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_toplevel_parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ges_sys::ges_timeline_element_get_toplevel_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_track_types(&self) -> TrackType {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_get_track_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec> {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_list_children_properties() }
    //}

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object> {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_lookup_child() }
    //}

    fn paste(&self, paste_position: gst::ClockTime) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ges_sys::ges_timeline_element_paste(
                self.as_ref().to_glib_none().0,
                paste_position.to_glib(),
            ))
        }
    }

    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_remove_child_property() }
    //}

    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_ripple(
                    self.as_ref().to_glib_none().0,
                    start.to_glib()
                ),
                "Failed to ripple"
            )
        }
    }

    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_ripple_end(
                    self.as_ref().to_glib_none().0,
                    end.to_glib()
                ),
                "Failed to ripple"
            )
        }
    }

    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_roll_end(
                    self.as_ref().to_glib_none().0,
                    end.to_glib()
                ),
                "Failed to roll"
            )
        }
    }

    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_roll_start(
                    self.as_ref().to_glib_none().0,
                    start.to_glib()
                ),
                "Failed to roll"
            )
        }
    }

    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_set_child_properties() }
    //}

    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_set_child_property() }
    //}

    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_set_child_property_by_pspec() }
    //}

    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ges_sys:ges_timeline_element_set_child_property_valist() }
    //}

    fn set_duration(&self, duration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_set_duration(
                self.as_ref().to_glib_none().0,
                duration.to_glib(),
            ))
        }
    }

    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_set_inpoint(
                self.as_ref().to_glib_none().0,
                inpoint.to_glib(),
            ))
        }
    }

    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_set_max_duration(
                self.as_ref().to_glib_none().0,
                maxduration.to_glib(),
            ))
        }
    }

    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_set_name(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0
                ),
                "Failed to set name"
            )
        }
    }

    fn set_parent<P: IsA<TimelineElement>>(
        &self,
        parent: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "`TimelineElement` already had a parent or its parent was the same as specified"
            )
        }
    }

    fn set_priority(&self, priority: u32) -> bool {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_set_priority(
                self.as_ref().to_glib_none().0,
                priority,
            ))
        }
    }

    fn set_start(&self, start: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ges_sys::ges_timeline_element_set_start(
                self.as_ref().to_glib_none().0,
                start.to_glib(),
            ))
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_none().0
                ),
                "`Failed to set timeline"
            )
        }
    }

    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_timeline_element_trim(self.as_ref().to_glib_none().0, start.to_glib()),
                "`Failed to trim"
            )
        }
    }

    fn get_property_in_point(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"in-point\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `in-point` getter")
                .unwrap()
        }
    }

    fn set_property_in_point(&self, in_point: u64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"in-point\0".as_ptr() as *const _,
                Value::from(&in_point).to_glib_none().0,
            );
        }
    }

    fn get_property_serialize(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"serialize\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `serialize` getter")
                .unwrap()
        }
    }

    fn set_property_serialize(&self, serialize: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"serialize\0".as_ptr() as *const _,
                Value::from(&serialize).to_glib_none().0,
            );
        }
    }

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute(notify_duration_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_in_point_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::in-point\0".as_ptr() as *const _,
                Some(transmute(notify_in_point_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-duration\0".as_ptr() as *const _,
                Some(transmute(
                    notify_max_duration_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute(notify_parent_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute(notify_priority_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_serialize_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::serialize\0".as_ptr() as *const _,
                Some(transmute(notify_serialize_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start\0".as_ptr() as *const _,
                Some(transmute(notify_start_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESTimelineElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeline\0".as_ptr() as *const _,
                Some(transmute(notify_timeline_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}
