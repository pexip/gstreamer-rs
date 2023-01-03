// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Issue, ReportLevel, Reporter, ReportingDetails};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Report(Shared<ffi::GstValidateReport>);

    match fn {
        ref => |ptr| ffi::gst_validate_report_ref(ptr),
        unref => |ptr| ffi::gst_validate_report_unref(ptr),
        type_ => || ffi::gst_validate_report_get_type(),
    }
}

impl Report {
    #[doc(alias = "gst_validate_report_new")]
    pub fn new(issue: &mut Issue, reporter: &impl IsA<Reporter>, message: &str) -> Report {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_validate_report_new(
                issue.to_glib_none_mut().0,
                reporter.as_ref().to_glib_none().0,
                message.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_report_add_message")]
    pub fn add_message(&self, message: &str) {
        unsafe {
            ffi::gst_validate_report_add_message(self.to_glib_none().0, message.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_add_repeated_report")]
    pub fn add_repeated_report(&self, repeated_report: &Report) {
        unsafe {
            ffi::gst_validate_report_add_repeated_report(
                self.to_glib_none().0,
                repeated_report.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_validate_report_check_abort")]
    pub fn check_abort(&self) -> bool {
        unsafe { from_glib(ffi::gst_validate_report_check_abort(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_get_dotfile_name")]
    #[doc(alias = "get_dotfile_name")]
    pub fn dotfile_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_validate_report_get_dotfile_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_report_get_issue")]
    #[doc(alias = "get_issue")]
    pub fn issue(&self) -> Issue {
        unsafe { from_glib_full(ffi::gst_validate_report_get_issue(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_get_issue_id")]
    #[doc(alias = "get_issue_id")]
    pub fn issue_id(&self) -> u32 {
        unsafe { ffi::gst_validate_report_get_issue_id(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_validate_report_get_level")]
    #[doc(alias = "get_level")]
    pub fn level(&self) -> ReportLevel {
        unsafe { from_glib(ffi::gst_validate_report_get_level(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_get_message")]
    #[doc(alias = "get_message")]
    pub fn message(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_validate_report_get_message(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_get_reporter")]
    #[doc(alias = "get_reporter")]
    pub fn reporter(&self) -> Reporter {
        unsafe { from_glib_full(ffi::gst_validate_report_get_reporter(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_get_reporter_name")]
    #[doc(alias = "get_reporter_name")]
    pub fn reporter_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_validate_report_get_reporter_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_report_get_reporting_level")]
    #[doc(alias = "get_reporting_level")]
    pub fn reporting_level(&self) -> ReportingDetails {
        unsafe {
            from_glib(ffi::gst_validate_report_get_reporting_level(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_validate_report_get_timestamp")]
    //#[doc(alias = "get_timestamp")]
    //pub fn timestamp(&self) -> /*Ignored*/gst::ClockTime {
    //    unsafe { TODO: call ffi:gst_validate_report_get_timestamp() }
    //}

    #[doc(alias = "gst_validate_report_get_trace")]
    #[doc(alias = "get_trace")]
    pub fn trace(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_validate_report_get_trace(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_validate_report_print_description")]
    pub fn print_description(&self) {
        unsafe {
            ffi::gst_validate_report_print_description(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_print_details")]
    pub fn print_details(&self) {
        unsafe {
            ffi::gst_validate_report_print_details(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_print_detected_on")]
    pub fn print_detected_on(&self) {
        unsafe {
            ffi::gst_validate_report_print_detected_on(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_print_level")]
    pub fn print_level(&self) {
        unsafe {
            ffi::gst_validate_report_print_level(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_printf")]
    pub fn printf(&self) {
        unsafe {
            ffi::gst_validate_report_printf(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_validate_report_set_master_report")]
    pub fn set_master_report(&self, master_report: &Report) -> bool {
        unsafe {
            from_glib(ffi::gst_validate_report_set_master_report(
                self.to_glib_none().0,
                master_report.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_report_set_reporting_level")]
    pub fn set_reporting_level(&self, level: ReportingDetails) {
        unsafe {
            ffi::gst_validate_report_set_reporting_level(self.to_glib_none().0, level.into_glib());
        }
    }

    #[doc(alias = "gst_validate_report_should_print")]
    pub fn should_print(&self) -> bool {
        unsafe { from_glib(ffi::gst_validate_report_should_print(self.to_glib_none().0)) }
    }

    //#[doc(alias = "gst_validate_report_action")]
    //pub fn action(reporter: &impl IsA<Reporter>, action: &Action, issue_id: /*Ignored*/IssueId, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gst_validate_report_action() }
    //}

    #[doc(alias = "gst_validate_report_init")]
    pub fn init() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_validate_report_init();
        }
    }

    //#[doc(alias = "gst_validate_report_valist")]
    //pub fn valist(reporter: &impl IsA<Reporter>, issue_id: /*Ignored*/IssueId, format: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gst_validate_report_valist() }
    //}
}

unsafe impl Send for Report {}
unsafe impl Sync for Report {}
