// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gst_sys;
use Object;

glib_wrapper! {
    pub struct Allocator(Object<gst_sys::GstAllocator, gst_sys::GstAllocatorClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_allocator_get_type(),
    }
}

impl Allocator {
    pub fn find(name: Option<&str>) -> Option<Allocator> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_sys::gst_allocator_find(name.to_glib_none().0)) }
    }

    pub fn register<P: IsA<Allocator>>(name: &str, allocator: &P) {
        skip_assert_initialized!();
        unsafe {
            gst_sys::gst_allocator_register(
                name.to_glib_none().0,
                allocator.as_ref().to_glib_full(),
            );
        }
    }
}

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

pub const NONE_ALLOCATOR: Option<&Allocator> = None;

pub trait AllocatorExt: 'static {
    //fn alloc(&self, size: usize, params: /*Ignored*/Option<&mut AllocationParams>) -> /*Ignored*/Option<Memory>;

    //fn free(&self, memory: /*Ignored*/&Memory);

    fn set_default(&self);
}

impl<O: IsA<Allocator>> AllocatorExt for O {
    //fn alloc(&self, size: usize, params: /*Ignored*/Option<&mut AllocationParams>) -> /*Ignored*/Option<Memory> {
    //    unsafe { TODO: call gst_sys:gst_allocator_alloc() }
    //}

    //fn free(&self, memory: /*Ignored*/&Memory) {
    //    unsafe { TODO: call gst_sys:gst_allocator_free() }
    //}

    fn set_default(&self) {
        unsafe {
            gst_sys::gst_allocator_set_default(self.as_ref().to_glib_full());
        }
    }
}
