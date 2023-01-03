// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};

bitflags! {
    #[doc(alias = "GstValidateIssueFlags")]
    pub struct IssueFlags: u32 {
        #[doc(alias = "GST_VALIDATE_ISSUE_FLAGS_NONE")]
        const NONE = ffi::GST_VALIDATE_ISSUE_FLAGS_NONE as _;
        #[doc(alias = "GST_VALIDATE_ISSUE_FLAGS_FULL_DETAILS")]
        const FULL_DETAILS = ffi::GST_VALIDATE_ISSUE_FLAGS_FULL_DETAILS as _;
        #[doc(alias = "GST_VALIDATE_ISSUE_FLAGS_NO_BACKTRACE")]
        const NO_BACKTRACE = ffi::GST_VALIDATE_ISSUE_FLAGS_NO_BACKTRACE as _;
        #[doc(alias = "GST_VALIDATE_ISSUE_FLAGS_FORCE_BACKTRACE")]
        const FORCE_BACKTRACE = ffi::GST_VALIDATE_ISSUE_FLAGS_FORCE_BACKTRACE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for IssueFlags {
    type GlibType = ffi::GstValidateIssueFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstValidateIssueFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstValidateIssueFlags> for IssueFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstValidateIssueFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for IssueFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_validate_issue_flags_get_type()) }
    }
}

impl glib::value::ValueType for IssueFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IssueFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for IssueFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<IssueFlags> for glib::Value {
    #[inline]
    fn from(v: IssueFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
