// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{BaseXmlFormatter, Extractable, Formatter};

glib::wrapper! {
    #[doc(alias = "GESXmlFormatter")]
    pub struct XmlFormatter(Object<ffi::GESXmlFormatter, ffi::GESXmlFormatterClass>) @extends BaseXmlFormatter, Formatter, @implements Extractable;

    match fn {
        type_ => || ffi::ges_xml_formatter_get_type(),
    }
}

impl XmlFormatter {
    pub const NONE: Option<&'static XmlFormatter> = None;
}
