// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Asset, MetaContainer, TrackElementAsset, UriClipAsset};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESUriSourceAsset")]
    pub struct UriSourceAsset(Object<ffi::GESUriSourceAsset, ffi::GESUriSourceAssetClass>) @extends TrackElementAsset, Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_uri_source_asset_get_type(),
    }
}

impl UriSourceAsset {
    pub const NONE: Option<&'static UriSourceAsset> = None;
}

pub trait UriSourceAssetExt: 'static {
    #[doc(alias = "ges_uri_source_asset_get_filesource_asset")]
    #[doc(alias = "get_filesource_asset")]
    fn filesource_asset(&self) -> UriClipAsset;

    #[doc(alias = "ges_uri_source_asset_get_stream_info")]
    #[doc(alias = "get_stream_info")]
    fn stream_info(&self) -> gst_pbutils::DiscovererStreamInfo;

    #[doc(alias = "ges_uri_source_asset_get_stream_uri")]
    #[doc(alias = "get_stream_uri")]
    fn stream_uri(&self) -> glib::GString;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_uri_source_asset_is_image")]
    fn is_image(&self) -> bool;
}

impl<O: IsA<UriSourceAsset>> UriSourceAssetExt for O {
    fn filesource_asset(&self) -> UriClipAsset {
        unsafe {
            from_glib_none(ffi::ges_uri_source_asset_get_filesource_asset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn stream_info(&self) -> gst_pbutils::DiscovererStreamInfo {
        unsafe {
            from_glib_none(ffi::ges_uri_source_asset_get_stream_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn stream_uri(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ges_uri_source_asset_get_stream_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_uri_source_asset_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
