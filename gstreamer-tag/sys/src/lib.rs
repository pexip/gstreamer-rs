// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstTagDemuxResult = c_int;
pub const GST_TAG_DEMUX_RESULT_BROKEN_TAG: GstTagDemuxResult = 0;
pub const GST_TAG_DEMUX_RESULT_AGAIN: GstTagDemuxResult = 1;
pub const GST_TAG_DEMUX_RESULT_OK: GstTagDemuxResult = 2;

pub type GstTagImageType = c_int;
pub const GST_TAG_IMAGE_TYPE_NONE: GstTagImageType = -1;
pub const GST_TAG_IMAGE_TYPE_UNDEFINED: GstTagImageType = 0;
pub const GST_TAG_IMAGE_TYPE_FRONT_COVER: GstTagImageType = 1;
pub const GST_TAG_IMAGE_TYPE_BACK_COVER: GstTagImageType = 2;
pub const GST_TAG_IMAGE_TYPE_LEAFLET_PAGE: GstTagImageType = 3;
pub const GST_TAG_IMAGE_TYPE_MEDIUM: GstTagImageType = 4;
pub const GST_TAG_IMAGE_TYPE_LEAD_ARTIST: GstTagImageType = 5;
pub const GST_TAG_IMAGE_TYPE_ARTIST: GstTagImageType = 6;
pub const GST_TAG_IMAGE_TYPE_CONDUCTOR: GstTagImageType = 7;
pub const GST_TAG_IMAGE_TYPE_BAND_ORCHESTRA: GstTagImageType = 8;
pub const GST_TAG_IMAGE_TYPE_COMPOSER: GstTagImageType = 9;
pub const GST_TAG_IMAGE_TYPE_LYRICIST: GstTagImageType = 10;
pub const GST_TAG_IMAGE_TYPE_RECORDING_LOCATION: GstTagImageType = 11;
pub const GST_TAG_IMAGE_TYPE_DURING_RECORDING: GstTagImageType = 12;
pub const GST_TAG_IMAGE_TYPE_DURING_PERFORMANCE: GstTagImageType = 13;
pub const GST_TAG_IMAGE_TYPE_VIDEO_CAPTURE: GstTagImageType = 14;
pub const GST_TAG_IMAGE_TYPE_FISH: GstTagImageType = 15;
pub const GST_TAG_IMAGE_TYPE_ILLUSTRATION: GstTagImageType = 16;
pub const GST_TAG_IMAGE_TYPE_BAND_ARTIST_LOGO: GstTagImageType = 17;
pub const GST_TAG_IMAGE_TYPE_PUBLISHER_STUDIO_LOGO: GstTagImageType = 18;

// Constants
pub const GST_TAG_ACOUSTID_FINGERPRINT: *const c_char =
    b"chromaprint-fingerprint\0" as *const u8 as *const c_char;
pub const GST_TAG_ACOUSTID_ID: *const c_char = b"acoustid-id\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_CONTRAST: *const c_char =
    b"capturing-contrast\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_DIGITAL_ZOOM_RATIO: *const c_char =
    b"capturing-digital-zoom-ratio\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_COMPENSATION: *const c_char =
    b"capturing-exposure-compensation\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_MODE: *const c_char =
    b"capturing-exposure-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_EXPOSURE_PROGRAM: *const c_char =
    b"capturing-exposure-program\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FLASH_FIRED: *const c_char =
    b"capturing-flash-fired\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FLASH_MODE: *const c_char =
    b"capturing-flash-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_LENGTH: *const c_char =
    b"capturing-focal-length\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_LENGTH_35_MM: *const c_char =
    b"capturing-focal-length-35mm\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_FOCAL_RATIO: *const c_char =
    b"capturing-focal-ratio\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_GAIN_ADJUSTMENT: *const c_char =
    b"capturing-gain-adjustment\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_ISO_SPEED: *const c_char =
    b"capturing-iso-speed\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_METERING_MODE: *const c_char =
    b"capturing-metering-mode\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SATURATION: *const c_char =
    b"capturing-saturation\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SCENE_CAPTURE_TYPE: *const c_char =
    b"capturing-scene-capture-type\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SHARPNESS: *const c_char =
    b"capturing-sharpness\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SHUTTER_SPEED: *const c_char =
    b"capturing-shutter-speed\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_SOURCE: *const c_char =
    b"capturing-source\0" as *const u8 as *const c_char;
pub const GST_TAG_CAPTURING_WHITE_BALANCE: *const c_char =
    b"capturing-white-balance\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_CDDB_DISCID: *const c_char = b"discid\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_CDDB_DISCID_FULL: *const c_char =
    b"discid-full\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_MUSICBRAINZ_DISCID: *const c_char =
    b"musicbrainz-discid\0" as *const u8 as *const c_char;
pub const GST_TAG_CDDA_MUSICBRAINZ_DISCID_FULL: *const c_char =
    b"musicbrainz-discid-full\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_CLIP: *const c_char = b"cmml-clip\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_HEAD: *const c_char = b"cmml-head\0" as *const u8 as *const c_char;
pub const GST_TAG_CMML_STREAM: *const c_char = b"cmml-stream\0" as *const u8 as *const c_char;
pub const GST_TAG_ID3V2_HEADER_SIZE: c_int = 10;
pub const GST_TAG_IMAGE_HORIZONTAL_PPI: *const c_char =
    b"image-horizontal-ppi\0" as *const u8 as *const c_char;
pub const GST_TAG_IMAGE_VERTICAL_PPI: *const c_char =
    b"image-vertical-ppi\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICAL_KEY: *const c_char = b"musical-key\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ALBUMARTISTID: *const c_char =
    b"musicbrainz-albumartistid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ALBUMID: *const c_char =
    b"musicbrainz-albumid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_ARTISTID: *const c_char =
    b"musicbrainz-artistid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_RELEASEGROUPID: *const c_char =
    b"musicbrainz-releasegroupid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_RELEASETRACKID: *const c_char =
    b"musicbrainz-releasetrackid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_TRACKID: *const c_char =
    b"musicbrainz-trackid\0" as *const u8 as *const c_char;
pub const GST_TAG_MUSICBRAINZ_TRMID: *const c_char =
    b"musicbrainz-trmid\0" as *const u8 as *const c_char;

// Flags
pub type GstTagLicenseFlags = c_uint;
pub const GST_TAG_LICENSE_PERMITS_REPRODUCTION: GstTagLicenseFlags = 1;
pub const GST_TAG_LICENSE_PERMITS_DISTRIBUTION: GstTagLicenseFlags = 2;
pub const GST_TAG_LICENSE_PERMITS_DERIVATIVE_WORKS: GstTagLicenseFlags = 4;
pub const GST_TAG_LICENSE_PERMITS_SHARING: GstTagLicenseFlags = 8;
pub const GST_TAG_LICENSE_REQUIRES_NOTICE: GstTagLicenseFlags = 256;
pub const GST_TAG_LICENSE_REQUIRES_ATTRIBUTION: GstTagLicenseFlags = 512;
pub const GST_TAG_LICENSE_REQUIRES_SHARE_ALIKE: GstTagLicenseFlags = 1024;
pub const GST_TAG_LICENSE_REQUIRES_SOURCE_CODE: GstTagLicenseFlags = 2048;
pub const GST_TAG_LICENSE_REQUIRES_COPYLEFT: GstTagLicenseFlags = 4096;
pub const GST_TAG_LICENSE_REQUIRES_LESSER_COPYLEFT: GstTagLicenseFlags = 8192;
pub const GST_TAG_LICENSE_PROHIBITS_COMMERCIAL_USE: GstTagLicenseFlags = 65536;
pub const GST_TAG_LICENSE_PROHIBITS_HIGH_INCOME_NATION_USE: GstTagLicenseFlags = 131072;
pub const GST_TAG_LICENSE_CREATIVE_COMMONS_LICENSE: GstTagLicenseFlags = 16777216;
pub const GST_TAG_LICENSE_FREE_SOFTWARE_FOUNDATION_LICENSE: GstTagLicenseFlags = 33554432;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstTagDemuxClass {
    pub parent_class: gst::GstElementClass,
    pub min_start_size: c_uint,
    pub min_end_size: c_uint,
    pub identify_tag: Option<
        unsafe extern "C" fn(
            *mut GstTagDemux,
            *mut gst::GstBuffer,
            gboolean,
            *mut c_uint,
        ) -> gboolean,
    >,
    pub parse_tag: Option<
        unsafe extern "C" fn(
            *mut GstTagDemux,
            *mut gst::GstBuffer,
            gboolean,
            *mut c_uint,
            *mut *mut gst::GstTagList,
        ) -> GstTagDemuxResult,
    >,
    pub merge_tags: Option<
        unsafe extern "C" fn(
            *mut GstTagDemux,
            *const gst::GstTagList,
            *const gst::GstTagList,
        ) -> *mut gst::GstTagList,
    >,
    pub reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstTagDemuxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTagDemuxClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("min_start_size", &self.min_start_size)
            .field("min_end_size", &self.min_end_size)
            .field("identify_tag", &self.identify_tag)
            .field("parse_tag", &self.parse_tag)
            .field("merge_tags", &self.merge_tags)
            .finish()
    }
}

#[repr(C)]
pub struct _GstTagDemuxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstTagDemuxPrivate = *mut _GstTagDemuxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstTagMuxClass {
    pub parent_class: gst::GstElementClass,
    pub render_start_tag:
        Option<unsafe extern "C" fn(*mut GstTagMux, *const gst::GstTagList) -> *mut gst::GstBuffer>,
    pub render_end_tag:
        Option<unsafe extern "C" fn(*mut GstTagMux, *const gst::GstTagList) -> *mut gst::GstBuffer>,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstTagMuxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTagMuxClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("render_start_tag", &self.render_start_tag)
            .field("render_end_tag", &self.render_end_tag)
            .finish()
    }
}

#[repr(C)]
pub struct _GstTagMuxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstTagMuxPrivate = *mut _GstTagMuxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstTagXmpWriterInterface {
    pub parent: gobject::GTypeInterface,
}

impl ::std::fmt::Debug for GstTagXmpWriterInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTagXmpWriterInterface @ {self:p}"))
            .field("parent", &self.parent)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstTagDemux {
    pub element: gst::GstElement,
    pub priv_: *mut GstTagDemuxPrivate,
    pub reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstTagDemux {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTagDemux @ {self:p}"))
            .field("element", &self.element)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstTagMux {
    pub element: gst::GstElement,
    pub priv_: *mut GstTagMuxPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstTagMux {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTagMux @ {self:p}"))
            .field("element", &self.element)
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct GstTagXmpWriter {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstTagXmpWriter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "GstTagXmpWriter @ {self:p}")
    }
}

extern "C" {

    //=========================================================================
    // GstTagDemuxResult
    //=========================================================================
    pub fn gst_tag_demux_result_get_type() -> GType;

    //=========================================================================
    // GstTagImageType
    //=========================================================================
    pub fn gst_tag_image_type_get_type() -> GType;

    //=========================================================================
    // GstTagLicenseFlags
    //=========================================================================
    pub fn gst_tag_license_flags_get_type() -> GType;

    //=========================================================================
    // GstTagDemux
    //=========================================================================
    pub fn gst_tag_demux_get_type() -> GType;

    //=========================================================================
    // GstTagMux
    //=========================================================================
    pub fn gst_tag_mux_get_type() -> GType;

    //=========================================================================
    // GstTagXmpWriter
    //=========================================================================
    pub fn gst_tag_xmp_writer_get_type() -> GType;
    pub fn gst_tag_xmp_writer_add_all_schemas(config: *mut GstTagXmpWriter);
    pub fn gst_tag_xmp_writer_add_schema(config: *mut GstTagXmpWriter, schema: *const c_char);
    pub fn gst_tag_xmp_writer_has_schema(
        config: *mut GstTagXmpWriter,
        schema: *const c_char,
    ) -> gboolean;
    pub fn gst_tag_xmp_writer_remove_all_schemas(config: *mut GstTagXmpWriter);
    pub fn gst_tag_xmp_writer_remove_schema(config: *mut GstTagXmpWriter, schema: *const c_char);
    pub fn gst_tag_xmp_writer_tag_list_to_xmp_buffer(
        config: *mut GstTagXmpWriter,
        taglist: *const gst::GstTagList,
        read_only: gboolean,
    ) -> *mut gst::GstBuffer;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_tag_check_language_code(lang_code: *const c_char) -> gboolean;
    pub fn gst_tag_freeform_string_to_utf8(
        data: *const c_char,
        size: c_int,
        env_vars: *mut *const c_char,
    ) -> *mut c_char;
    pub fn gst_tag_from_id3_tag(id3_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_from_id3_user_tag(
        type_: *const c_char,
        id3_user_tag: *const c_char,
    ) -> *const c_char;
    pub fn gst_tag_from_vorbis_tag(vorbis_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_get_id3v2_tag_size(buffer: *mut gst::GstBuffer) -> c_uint;
    pub fn gst_tag_get_language_code_iso_639_1(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_code_iso_639_2B(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_code_iso_639_2T(lang_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_language_codes() -> *mut *mut c_char;
    pub fn gst_tag_get_language_name(language_code: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_description(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_flags(license_ref: *const c_char) -> GstTagLicenseFlags;
    pub fn gst_tag_get_license_jurisdiction(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_nick(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_title(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_license_version(license_ref: *const c_char) -> *const c_char;
    pub fn gst_tag_get_licenses() -> *mut *mut c_char;
    pub fn gst_tag_id3_genre_count() -> c_uint;
    pub fn gst_tag_id3_genre_get(id: c_uint) -> *const c_char;
    pub fn gst_tag_image_data_to_image_sample(
        image_data: *const u8,
        image_data_len: c_uint,
        image_type: GstTagImageType,
    ) -> *mut gst::GstSample;
    pub fn gst_tag_list_add_id3_image(
        tag_list: *mut gst::GstTagList,
        image_data: *const u8,
        image_data_len: c_uint,
        id3_picture_type: c_uint,
    ) -> gboolean;
    pub fn gst_tag_list_from_exif_buffer(
        buffer: *mut gst::GstBuffer,
        byte_order: c_int,
        base_offset: u32,
    ) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_exif_buffer_with_tiff_header(
        buffer: *mut gst::GstBuffer,
    ) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_id3v2_tag(buffer: *mut gst::GstBuffer) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_vorbiscomment(
        data: *const u8,
        size: size_t,
        id_data: *const u8,
        id_data_length: c_uint,
        vendor_string: *mut *mut c_char,
    ) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_vorbiscomment_buffer(
        buffer: *mut gst::GstBuffer,
        id_data: *const u8,
        id_data_length: c_uint,
        vendor_string: *mut *mut c_char,
    ) -> *mut gst::GstTagList;
    pub fn gst_tag_list_from_xmp_buffer(buffer: *mut gst::GstBuffer) -> *mut gst::GstTagList;
    pub fn gst_tag_list_new_from_id3v1(data: *const [u8; 128]) -> *mut gst::GstTagList;
    pub fn gst_tag_list_to_exif_buffer(
        taglist: *const gst::GstTagList,
        byte_order: c_int,
        base_offset: u32,
    ) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_exif_buffer_with_tiff_header(
        taglist: *const gst::GstTagList,
    ) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_vorbiscomment_buffer(
        list: *const gst::GstTagList,
        id_data: *const u8,
        id_data_length: c_uint,
        vendor_string: *const c_char,
    ) -> *mut gst::GstBuffer;
    pub fn gst_tag_list_to_xmp_buffer(
        list: *const gst::GstTagList,
        read_only: gboolean,
        schemas: *mut *const c_char,
    ) -> *mut gst::GstBuffer;
    pub fn gst_tag_parse_extended_comment(
        ext_comment: *const c_char,
        key: *mut *mut c_char,
        lang: *mut *mut c_char,
        value: *mut *mut c_char,
        fail_if_no_key: gboolean,
    ) -> gboolean;
    pub fn gst_tag_register_musicbrainz_tags();
    pub fn gst_tag_to_id3_tag(gst_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_to_vorbis_comments(
        list: *const gst::GstTagList,
        tag: *const c_char,
    ) -> *mut glib::GList;
    pub fn gst_tag_to_vorbis_tag(gst_tag: *const c_char) -> *const c_char;
    pub fn gst_tag_xmp_list_schemas() -> *mut *const c_char;
    pub fn gst_vorbis_tag_add(list: *mut gst::GstTagList, tag: *const c_char, value: *const c_char);

}
