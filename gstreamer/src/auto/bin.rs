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
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use ChildProxy;
use Element;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use ElementFlags;
use Object;
use Pad;
use PadDirection;

glib_wrapper! {
    pub struct Bin(Object<gst_sys::GstBin, gst_sys::GstBinClass>) @extends Element, Object, @implements ChildProxy;

    match fn {
        get_type => || gst_sys::gst_bin_get_type(),
    }
}

impl Bin {
    pub fn new(name: Option<&str>) -> Bin {
        assert_initialized_main_thread!();
        unsafe {
            Element::from_glib_none(gst_sys::gst_bin_new(name.to_glib_none().0)).unsafe_cast()
        }
    }
}

unsafe impl Send for Bin {}
unsafe impl Sync for Bin {}

pub const NONE_BIN: Option<&Bin> = None;

pub trait GstBinExt: 'static {
    fn add<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError>;

    //fn add_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad>;

    fn get_by_interface(&self, iface: glib::types::Type) -> Option<Element>;

    fn get_by_name(&self, name: &str) -> Option<Element>;

    fn get_by_name_recurse_up(&self, name: &str) -> Option<Element>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_suppressed_flags(&self) -> ElementFlags;

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> /*Ignored*/Option<Iterator>;

    //fn iterate_all_by_interface(&self, iface: glib::types::Type) -> /*Ignored*/Option<Iterator>;

    //fn iterate_elements(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_recurse(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sinks(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sorted(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_sources(&self) -> /*Ignored*/Option<Iterator>;

    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError>;

    fn remove<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError>;

    //fn remove_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_suppressed_flags(&self, flags: ElementFlags);

    fn sync_children_states(&self) -> Result<(), glib::error::BoolError>;

    fn get_property_async_handling(&self) -> bool;

    fn set_property_async_handling(&self, async_handling: bool);

    fn get_property_message_forward(&self) -> bool;

    fn set_property_message_forward(&self, message_forward: bool);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Bin>> GstBinExt for O {
    fn add<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_bin_add(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    //fn add_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gst_sys:gst_bin_add_many() }
    //}

    fn find_unlinked_pad(&self, direction: PadDirection) -> Option<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_find_unlinked_pad(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
            ))
        }
    }

    fn get_by_interface(&self, iface: glib::types::Type) -> Option<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_get_by_interface(
                self.as_ref().to_glib_none().0,
                iface.to_glib(),
            ))
        }
    }

    fn get_by_name(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_get_by_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_by_name_recurse_up(&self, name: &str) -> Option<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_get_by_name_recurse_up(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_suppressed_flags(&self) -> ElementFlags {
        unsafe {
            from_glib(gst_sys::gst_bin_get_suppressed_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_all_by_element_factory_name() }
    //}

    //fn iterate_all_by_interface(&self, iface: glib::types::Type) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_all_by_interface() }
    //}

    //fn iterate_elements(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_elements() }
    //}

    //fn iterate_recurse(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_recurse() }
    //}

    //fn iterate_sinks(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_sinks() }
    //}

    //fn iterate_sorted(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_sorted() }
    //}

    //fn iterate_sources(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_bin_iterate_sources() }
    //}

    fn recalculate_latency(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_bin_recalculate_latency(self.as_ref().to_glib_none().0),
                "Failed to recalculate latency"
            )
        }
    }

    fn remove<P: IsA<Element>>(&self, element: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_bin_remove(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    //fn remove_many<P: IsA<Element>>(&self, element_1: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gst_sys:gst_bin_remove_many() }
    //}

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_suppressed_flags(&self, flags: ElementFlags) {
        unsafe {
            gst_sys::gst_bin_set_suppressed_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn sync_children_states(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_bin_sync_children_states(self.as_ref().to_glib_none().0),
                "Failed to sync children states"
            )
        }
    }

    fn get_property_async_handling(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"async-handling\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `async-handling` getter")
                .unwrap()
        }
    }

    fn set_property_async_handling(&self, async_handling: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"async-handling\0".as_ptr() as *const _,
                Value::from(&async_handling).to_glib_none().0,
            );
        }
    }

    fn get_property_message_forward(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"message-forward\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `message-forward` getter")
                .unwrap()
        }
    }

    fn set_property_message_forward(&self, message_forward: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"message-forward\0".as_ptr() as *const _,
                Value::from(&message_forward).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_added<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deep_element_added_trampoline<
            P,
            F: Fn(&P, &Bin, &Element) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            sub_bin: *mut gst_sys::GstBin,
            element: *mut gst_sys::GstElement,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(sub_bin),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deep-element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deep_element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_deep_element_removed<F: Fn(&Self, &Bin, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deep_element_removed_trampoline<
            P,
            F: Fn(&P, &Bin, &Element) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            sub_bin: *mut gst_sys::GstBin,
            element: *mut gst_sys::GstElement,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(sub_bin),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deep-element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deep_element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_element_added<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn element_added_trampoline<
            P,
            F: Fn(&P, &Element) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            element: *mut gst_sys::GstElement,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_element_removed<F: Fn(&Self, &Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn element_removed_trampoline<
            P,
            F: Fn(&P, &Element) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            element: *mut gst_sys::GstElement,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Bin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_async_handling_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_async_handling_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(&Bin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::async-handling\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_async_handling_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_message_forward_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_message_forward_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_sys::GstBin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Bin>,
        {
            let f: &F = &*(f as *const F);
            f(&Bin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::message-forward\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_message_forward_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
