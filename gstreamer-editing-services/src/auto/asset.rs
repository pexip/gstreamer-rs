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
use glib::GString;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;
use Extractable;

glib_wrapper! {
    pub struct Asset(Object<ges_sys::GESAsset, ges_sys::GESAssetClass>);

    match fn {
        get_type => || ges_sys::ges_asset_get_type(),
    }
}

impl Asset {
    pub fn needs_reload(extractable_type: glib::types::Type, id: Option<&str>) -> bool {
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

    pub fn request_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Asset, glib::Error>) + Send + 'static,
    >(
        extractable_type: glib::types::Type,
        id: Option<&str>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        assert_initialized_main_thread!();
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn request_async_trampoline<
            Q: FnOnce(Result<Asset, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ges_sys::ges_asset_request_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_async_trampoline::<Q>;
        unsafe {
            ges_sys::ges_asset_request_async(
                extractable_type.to_glib(),
                id.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn request_async_future(
        extractable_type: glib::types::Type,
        id: Option<&str>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Asset, glib::Error>> + 'static>> {
        skip_assert_initialized!();
        let id = id.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::request_async(
                extractable_type,
                id.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}

pub const NONE_ASSET: Option<&Asset> = None;

pub trait AssetExt: 'static {
    fn extract(&self) -> Result<Extractable, glib::Error>;

    fn get_error(&self) -> Option<glib::Error>;

    fn get_extractable_type(&self) -> glib::types::Type;

    fn get_id(&self) -> Option<GString>;

    fn get_proxy(&self) -> Option<Asset>;

    fn get_proxy_target(&self) -> Option<Asset>;

    fn list_proxies(&self) -> Vec<Asset>;

    fn set_proxy<P: IsA<Asset>>(&self, proxy: Option<&P>) -> Result<(), glib::error::BoolError>;

    fn unproxy<P: IsA<Asset>>(&self, proxy: &P) -> Result<(), glib::error::BoolError>;

    fn connect_property_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_target_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Asset>> AssetExt for O {
    fn extract(&self) -> Result<Extractable, glib::Error> {
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_trampoline::<Self, F> as *const (),
                )),
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
