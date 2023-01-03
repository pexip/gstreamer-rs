// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Section(Boxed<ffi::GstMpegtsSection>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gst_mpegts_section_get_type(), ptr as *mut _) as *mut ffi::GstMpegtsSection,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gst_mpegts_section_get_type(), ptr as *mut _),
        type_ => || ffi::gst_mpegts_section_get_type(),
    }
}

impl Section {
    //#[doc(alias = "gst_mpegts_section_get_atsc_cvct")]
    //#[doc(alias = "get_atsc_cvct")]
    //pub fn atsc_cvct(&mut self) -> /*Ignored*/AtscVCT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_cvct() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_eit")]
    //#[doc(alias = "get_atsc_eit")]
    //pub fn atsc_eit(&mut self) -> /*Ignored*/AtscEIT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_eit() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_ett")]
    //#[doc(alias = "get_atsc_ett")]
    //pub fn atsc_ett(&mut self) -> /*Ignored*/AtscETT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_ett() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_mgt")]
    //#[doc(alias = "get_atsc_mgt")]
    //pub fn atsc_mgt(&mut self) -> /*Ignored*/AtscMGT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_mgt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_rrt")]
    //#[doc(alias = "get_atsc_rrt")]
    //pub fn atsc_rrt(&mut self) -> /*Ignored*/AtscRRT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_rrt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_stt")]
    //#[doc(alias = "get_atsc_stt")]
    //pub fn atsc_stt(&mut self) -> /*Ignored*/AtscSTT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_stt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_atsc_tvct")]
    //#[doc(alias = "get_atsc_tvct")]
    //pub fn atsc_tvct(&mut self) -> /*Ignored*/AtscVCT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_atsc_tvct() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_bat")]
    //#[doc(alias = "get_bat")]
    //pub fn bat(&mut self) -> /*Ignored*/BAT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_bat() }
    //}

    #[doc(alias = "gst_mpegts_section_get_data")]
    #[doc(alias = "get_data")]
    pub fn data(&mut self) -> glib::Bytes {
        unsafe { from_glib_full(ffi::gst_mpegts_section_get_data(self.to_glib_none_mut().0)) }
    }

    //#[doc(alias = "gst_mpegts_section_get_eit")]
    //#[doc(alias = "get_eit")]
    //pub fn eit(&mut self) -> /*Ignored*/EIT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_eit() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_nit")]
    //#[doc(alias = "get_nit")]
    //pub fn nit(&mut self) -> /*Ignored*/NIT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_nit() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_pat")]
    //#[doc(alias = "get_pat")]
    //pub fn pat(&mut self) -> /*Ignored*/Vec<PatProgram> {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_pat() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_pmt")]
    //#[doc(alias = "get_pmt")]
    //pub fn pmt(&mut self) -> /*Ignored*/PMT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_pmt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_sdt")]
    //#[doc(alias = "get_sdt")]
    //pub fn sdt(&mut self) -> /*Ignored*/SDT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_sdt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_sit")]
    //#[doc(alias = "get_sit")]
    //pub fn sit(&mut self) -> /*Ignored*/SIT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_sit() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_tdt")]
    //#[doc(alias = "get_tdt")]
    //pub fn tdt(&mut self) -> /*Ignored*/gst::DateTime {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_tdt() }
    //}

    //#[doc(alias = "gst_mpegts_section_get_tot")]
    //#[doc(alias = "get_tot")]
    //pub fn tot(&mut self) -> /*Ignored*/TOT {
    //    unsafe { TODO: call ffi:gst_mpegts_section_get_tot() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_atsc_mgt")]
    //pub fn from_atsc_mgt(mgt: /*Ignored*/AtscMGT) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_atsc_mgt() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_atsc_rrt")]
    //pub fn from_atsc_rrt(rrt: /*Ignored*/&mut AtscRRT) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_atsc_rrt() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_atsc_stt")]
    //pub fn from_atsc_stt(stt: /*Ignored*/&mut AtscSTT) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_atsc_stt() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_nit")]
    //pub fn from_nit(nit: /*Ignored*/NIT) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_nit() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_pat")]
    //pub fn from_pat(programs: /*Ignored*/Vec<PatProgram>, ts_id: u16) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_pat() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_pmt")]
    //pub fn from_pmt(pmt: /*Ignored*/PMT, pid: u16) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_pmt() }
    //}

    //#[doc(alias = "gst_mpegts_section_from_sdt")]
    //pub fn from_sdt(sdt: /*Ignored*/SDT) -> Section {
    //    unsafe { TODO: call ffi:gst_mpegts_section_from_sdt() }
    //}
}

unsafe impl Send for Section {}
unsafe impl Sync for Section {}
