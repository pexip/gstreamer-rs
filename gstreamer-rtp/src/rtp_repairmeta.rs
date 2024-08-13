// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;
use gst::prelude::*;

use crate::ffi::{GstRTPRepairMeta, gst_buffer_add_rtp_repair_meta,
                 gst_rtp_repair_meta_api_get_type,
                 gst_buffer_get_rtp_repair_meta};

#[repr(transparent)]
#[doc(alias = "GstRTPRepairMeta")]
pub struct RTPRepairMeta(GstRTPRepairMeta);

unsafe impl Send for RTPRepairMeta {}
unsafe impl Sync for RTPRepairMeta {}

impl RTPRepairMeta {
    #[doc(alias = "gst_buffer_add_rtp_repair_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        ssrc: u32,
        seqnum: &[u16],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_buffer_add_rtp_repair_meta(
                buffer.as_mut_ptr(),
                ssrc,
                seqnum.as_ptr(),
                seqnum.len() as u32,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[inline]
    pub fn get<'a>(
        buffer: &'a mut gst::BufferRef
    ) -> gst::MetaRefMut<'a,Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_buffer_get_rtp_repair_meta(buffer.as_mut_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[inline]
    pub fn ssrc(&mut self) -> Option::<u32> {
        if self.0.seqnums != ptr::null_mut() {
            Some(self.0.ssrc)
        } else {
            None
        }
    }

    #[inline]
    pub fn seqnum(&self) -> Option::<&[u16]> {
        unsafe {
            if self.0.seqnums != ptr::null_mut() {
                Some(std::slice::from_raw_parts(
                    (*self.0.seqnums).data as *const u16,
                    (*self.0.seqnums).len as usize,
                ))
            } else {
                None
            }
        }
    }
}

unsafe impl MetaAPI for RTPRepairMeta {
    type GstType = GstRTPRepairMeta;

    #[doc(alias = "gst_rtp_repair_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(gst_rtp_repair_meta_api_get_type()) }
    }
}

