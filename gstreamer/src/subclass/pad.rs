// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;

use glib;
use glib::translate::*;

use glib::subclass::prelude::*;

use Pad;
use PadClass;

pub trait PadImpl: PadImplExt + ObjectImpl + Send + Sync + 'static {
    fn linked(&self, pad: &Pad, peer: &Pad) {
        self.parent_linked(pad, peer)
    }

    fn unlinked(&self, pad: &Pad, peer: &Pad) {
        self.parent_unlinked(pad, peer)
    }
}

pub trait PadImplExt {
    fn parent_linked(&self, pad: &Pad, peer: &Pad);

    fn parent_unlinked(&self, pad: &Pad, peer: &Pad);
}

impl<T: PadImpl + ObjectImpl> PadImplExt for T {
    fn parent_linked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstPadClass;

            (*parent_class)
                .linked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_unlinked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstPadClass;

            (*parent_class)
                .unlinked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }
}

unsafe impl<T: ObjectSubclass + PadImpl> IsSubclassable<T> for PadClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstPadClass);
            klass.linked = Some(pad_linked::<T>);
            klass.unlinked = Some(pad_unlinked::<T>);
        }
    }
}

unsafe extern "C" fn pad_linked<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstPad,
    peer: *mut gst_sys::GstPad,
) where
    T: PadImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Pad = from_glib_borrow(ptr);

    imp.linked(&wrap, &from_glib_borrow(peer))
}

unsafe extern "C" fn pad_unlinked<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstPad,
    peer: *mut gst_sys::GstPad,
) where
    T: PadImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Pad = from_glib_borrow(ptr);

    imp.unlinked(&wrap, &from_glib_borrow(peer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use glib;
    use glib::subclass;
    use std::sync::Mutex;

    struct TestPad {
        linked: Mutex<bool>,
        unlinked: Mutex<bool>,
    }

    impl ObjectSubclass for TestPad {
        const NAME: &'static str = "TestPad";
        type ParentType = ::Pad;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                linked: Mutex::new(false),
                unlinked: Mutex::new(false),
            }
        }
    }

    impl ObjectImpl for TestPad {
        glib_object_impl!();
    }

    impl PadImpl for TestPad {
        fn linked(&self, pad: &Pad, peer: &Pad) {
            *self.linked.lock().unwrap() = true;
            self.parent_linked(pad, peer)
        }

        fn unlinked(&self, pad: &Pad, peer: &Pad) {
            *self.unlinked.lock().unwrap() = true;
            self.parent_unlinked(pad, peer)
        }
    }

    #[test]
    fn test_pad_subclass() {
        ::init().unwrap();

        let pad = glib::Object::new(
            TestPad::get_type(),
            &[("name", &"test"), ("direction", &::PadDirection::Src)],
        )
        .unwrap()
        .downcast::<::Pad>()
        .unwrap();

        assert_eq!(pad.get_name(), "test");

        let otherpad = ::Pad::new(Some("other-test"), ::PadDirection::Sink);
        pad.link(&otherpad).unwrap();
        pad.unlink(&otherpad).unwrap();

        let imp = TestPad::from_instance(&pad);
        assert!(*imp.linked.lock().unwrap());
        assert!(*imp.unlinked.lock().unwrap());
    }
}
