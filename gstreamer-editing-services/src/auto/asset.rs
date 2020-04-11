// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use gio;
use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use Extractable;

glib_wrapper! {
    pub struct Asset(Object<ges_sys::GESAsset, ges_sys::GESAssetClass, AssetClass>);

    match fn {
        get_type => || ges_sys::ges_asset_get_type(),
    }
}

impl Asset {
    pub fn needs_reload(extractable_type: glib::types::Type, id: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ges_sys::ges_asset_needs_reload(
                extractable_type.to_glib(),
                id.to_glib_none().0,
            ))
        }
    }

    pub fn request(
        extractable_type: glib::types::Type,
        id: Option<&str>,
    ) -> Result<Option<Asset>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ges_sys::ges_asset_request(
                extractable_type.to_glib(),
                id.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_ASSET: Option<&Asset> = None;

pub trait AssetExt: 'static {
    fn extract(&self) -> Result<Option<Extractable>, glib::Error>;

    fn get_error(&self) -> Option<glib::Error>;

    fn get_extractable_type(&self) -> glib::types::Type;

    fn get_id(&self) -> Option<GString>;

    fn get_proxy(&self) -> Option<Asset>;

    fn get_proxy_target(&self) -> Option<Asset>;

    fn list_proxies(&self) -> Vec<Asset>;

    fn set_proxy<P: IsA<Asset>>(&self, proxy: Option<&P>) -> Result<(), glib::error::BoolError>;

    fn unproxy<P: IsA<Asset>>(&self, proxy: &P) -> Result<(), glib::error::BoolError>;

    fn set_property_proxy_target<P: IsA<Asset> + SetValueOptional>(&self, proxy_target: Option<&P>);

    fn connect_property_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_target_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Asset>> AssetExt for O {
    fn extract(&self) -> Result<Option<Extractable>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ges_sys::ges_asset_extract(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ges_sys::ges_asset_get_error(self.as_ref().to_glib_none().0)) }
    }

    fn get_extractable_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ges_sys::ges_asset_get_extractable_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id(&self) -> Option<GString> {
        unsafe { from_glib_none(ges_sys::ges_asset_get_id(self.as_ref().to_glib_none().0)) }
    }

    fn get_proxy(&self) -> Option<Asset> {
        unsafe { from_glib_none(ges_sys::ges_asset_get_proxy(self.as_ref().to_glib_none().0)) }
    }

    fn get_proxy_target(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ges_sys::ges_asset_get_proxy_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_proxies(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ges_sys::ges_asset_list_proxies(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_proxy<P: IsA<Asset>>(&self, proxy: Option<&P>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_asset_set_proxy(
                    self.as_ref().to_glib_none().0,
                    proxy.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Failed to set proxy"
            )
        }
    }

    fn unproxy<P: IsA<Asset>>(&self, proxy: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_asset_unproxy(
                    self.as_ref().to_glib_none().0,
                    proxy.as_ref().to_glib_none().0
                ),
                "Failed to unproxy asset"
            )
        }
    }

    fn set_property_proxy_target<P: IsA<Asset> + SetValueOptional>(
        &self,
        proxy_target: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"proxy-target\0".as_ptr() as *const _,
                Value::from(proxy_target).to_glib_none().0,
            );
        }
    }

    fn connect_property_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESAsset,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Asset>,
        {
            let f: &F = &*(f as *const F);
            f(&Asset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy\0".as_ptr() as *const _,
                Some(transmute(notify_proxy_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_proxy_target_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_target_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESAsset,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Asset>,
        {
            let f: &F = &*(f as *const F);
            f(&Asset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-target\0".as_ptr() as *const _,
                Some(transmute(
                    notify_proxy_target_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
