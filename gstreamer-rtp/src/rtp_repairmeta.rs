// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use crate::ffi;
use glib::translate::*;
use gst::prelude::*;

#[repr(transparent)]
#[doc(alias = "GstRTPRepairMeta")]
pub struct RTPRepairMeta(ffi::GstRTPRepairMeta);

unsafe impl Send for RTPRepairMeta {}
unsafe impl Sync for RTPRepairMeta {}

impl RTPRepairMeta {
    #[doc(alias = "gst_rtp_repair_meta_add")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        idx: u16,
        num_pkts: u16,
        ssrc: u32,
        seqnum: &[u16],
        timestamps: &[u32],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        assert_eq!(seqnum.len(), timestamps.len());
        unsafe {
            let meta = ffi::gst_rtp_repair_meta_add(
                buffer.as_mut_ptr(),
                idx,
                num_pkts,
                ssrc,
                seqnum.as_ptr(),
                timestamps.as_ptr(),
                seqnum.len() as u32,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[inline]
    pub fn get<'a>(buffer: &'a gst::BufferRef) -> Option<gst::MetaRef<'a, Self>> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_rtp_repair_meta_get(buffer.as_mut_ptr());
            match meta.is_null() {
                true => None,
                false => Some(Self::from_ptr(buffer, meta)),
            }
        }
    }

    #[inline]
    pub fn idx(&self) -> Option<u16> {
        if !self.0.seqnums.is_null() {
            Some(self.0.idx_red_packets)
        } else {
            None
        }
    }

    #[inline]
    pub fn num_red_pkts(&self) -> Option<u16> {
        if !self.0.seqnums.is_null() {
            Some(self.0.num_red_packets)
        } else {
            None
        }
    }

    #[inline]
    pub fn ssrc(&mut self) -> Option<u32> {
        if !self.0.seqnums.is_null() {
            Some(self.0.ssrc)
        } else {
            None
        }
    }

    #[inline]
    pub fn repair_seqnum(&self) -> Option<&[u16]> {
        unsafe {
            if !self.0.seqnums.is_null() {
                Some(std::slice::from_raw_parts(
                    (*self.0.seqnums).data as *const u16,
                    (*self.0.seqnums).len as usize,
                ))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn repair_timestamps(&self) -> Option<&[u32]> {
        unsafe {
            if !self.0.timestamps.is_null() {
                Some(std::slice::from_raw_parts(
                    (*self.0.timestamps).data as *const u32,
                    (*self.0.timestamps).len as usize,
                ))
            } else {
                None
            }
        }
    }
}

unsafe impl MetaAPI for RTPRepairMeta {
    type GstType = ffi::GstRTPRepairMeta;

    #[doc(alias = "gst_rtp_repair_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtp_repair_meta_api_get_type()) }
    }
}
