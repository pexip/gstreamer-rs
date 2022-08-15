// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use crate::WebRTCICEComponent;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use crate::WebRTCICEStream;
#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use crate::WebRTCICETransport;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::StaticType;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstWebRTCICE")]
    pub struct WebRTCICE(Object<ffi::GstWebRTCICE, ffi::GstWebRTCICEClass>);

    match fn {
        type_ => || ffi::gst_webrtc_ice_get_type(),
    }
}

impl WebRTCICE {
    pub const NONE: Option<&'static WebRTCICE> = None;

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //#[doc(alias = "gst_webrtc_ice_candidate_stats_free")]
    //pub fn candidate_stats_free(stats: /*Ignored*/&mut WebRTCICECandidateStats) {
    //    unsafe { TODO: call ffi:gst_webrtc_ice_candidate_stats_free() }
    //}
}

unsafe impl Send for WebRTCICE {}
unsafe impl Sync for WebRTCICE {}

pub trait WebRTCICEExt: 'static {
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_add_candidate")]
    fn add_candidate(&self, stream: &impl IsA<WebRTCICEStream>, candidate: &str);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_add_stream")]
    fn add_stream(&self, session_id: u32) -> Option<WebRTCICEStream>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_add_turn_server")]
    fn add_turn_server(&self, uri: &str) -> bool;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_find_transport")]
    fn find_transport(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        component: WebRTCICEComponent,
    ) -> Option<WebRTCICETransport>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_gather_candidates")]
    fn gather_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> bool;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_get_is_controller")]
    #[doc(alias = "get_is_controller")]
    fn is_controller(&self) -> bool;

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //#[doc(alias = "gst_webrtc_ice_get_local_candidates")]
    //#[doc(alias = "get_local_candidates")]
    //fn local_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 12 };

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //#[doc(alias = "gst_webrtc_ice_get_remote_candidates")]
    //#[doc(alias = "get_remote_candidates")]
    //fn remote_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 12 };

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //#[doc(alias = "gst_webrtc_ice_get_selected_pair")]
    //#[doc(alias = "get_selected_pair")]
    //fn is_selected_pair(&self, stream: &impl IsA<WebRTCICEStream>, local_stats: /*Ignored*/&mut WebRTCICECandidateStats, remote_stats: /*Ignored*/&mut WebRTCICECandidateStats) -> bool;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_get_stun_server")]
    #[doc(alias = "get_stun_server")]
    fn stun_server(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_get_turn_server")]
    #[doc(alias = "get_turn_server")]
    fn turn_server(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_force_relay")]
    fn set_force_relay(&self, force_relay: bool);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_is_controller")]
    fn set_is_controller(&self, controller: bool);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_local_credentials")]
    fn set_local_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_on_ice_candidate")]
    fn set_on_ice_candidate<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(&self, func: P);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_remote_credentials")]
    fn set_remote_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool;

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_stun_server")]
    fn set_stun_server(&self, uri: &str);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_tos")]
    fn set_tos(&self, stream: &impl IsA<WebRTCICEStream>, tos: u32);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_ice_set_turn_server")]
    fn set_turn_server(&self, uri: &str);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn max_rtp_port(&self) -> u32;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn set_max_rtp_port(&self, max_rtp_port: u32);

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn min_rtp_port(&self) -> u32;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn set_min_rtp_port(&self, min_rtp_port: u32);

    #[doc(alias = "add-local-ip-address")]
    fn connect_add_local_ip_address<F: Fn(&Self, &str) -> bool + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_add_local_ip_address(&self, address: &str) -> bool;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn connect_max_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn connect_min_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<WebRTCICE>> WebRTCICEExt for O {
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn add_candidate(&self, stream: &impl IsA<WebRTCICEStream>, candidate: &str) {
        unsafe {
            ffi::gst_webrtc_ice_add_candidate(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                candidate.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn add_stream(&self, session_id: u32) -> Option<WebRTCICEStream> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_add_stream(
                self.as_ref().to_glib_none().0,
                session_id,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn add_turn_server(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_add_turn_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn find_transport(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        component: WebRTCICEComponent,
    ) -> Option<WebRTCICETransport> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_find_transport(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                component.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn gather_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_gather_candidates(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn is_controller(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_get_is_controller(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //fn local_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 12 } {
    //    unsafe { TODO: call ffi:gst_webrtc_ice_get_local_candidates() }
    //}

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //fn remote_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 12 } {
    //    unsafe { TODO: call ffi:gst_webrtc_ice_get_remote_candidates() }
    //}

    //#[cfg(any(feature = "v1_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    //fn is_selected_pair(&self, stream: &impl IsA<WebRTCICEStream>, local_stats: /*Ignored*/&mut WebRTCICECandidateStats, remote_stats: /*Ignored*/&mut WebRTCICECandidateStats) -> bool {
    //    unsafe { TODO: call ffi:gst_webrtc_ice_get_selected_pair() }
    //}

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn stun_server(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_get_stun_server(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn turn_server(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_get_turn_server(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_force_relay(&self, force_relay: bool) {
        unsafe {
            ffi::gst_webrtc_ice_set_force_relay(
                self.as_ref().to_glib_none().0,
                force_relay.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_is_controller(&self, controller: bool) {
        unsafe {
            ffi::gst_webrtc_ice_set_is_controller(
                self.as_ref().to_glib_none().0,
                controller.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_local_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_set_local_credentials(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                ufrag.to_glib_none().0,
                pwd.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_on_ice_candidate<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(
            ice: *mut ffi::GstWebRTCICE,
            stream_id: libc::c_uint,
            candidate: *mut libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let ice = from_glib_borrow(ice);
            let candidate: Borrowed<glib::GString> = from_glib_borrow(candidate);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&ice, stream_id, candidate.as_str());
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn notify_func<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gst_webrtc_ice_set_on_ice_candidate(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_remote_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_set_remote_credentials(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                ufrag.to_glib_none().0,
                pwd.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_stun_server(&self, uri: &str) {
        unsafe {
            ffi::gst_webrtc_ice_set_stun_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_tos(&self, stream: &impl IsA<WebRTCICEStream>, tos: u32) {
        unsafe {
            ffi::gst_webrtc_ice_set_tos(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                tos,
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn set_turn_server(&self, uri: &str) {
        unsafe {
            ffi::gst_webrtc_ice_set_turn_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn max_rtp_port(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "max-rtp-port")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn set_max_rtp_port(&self, max_rtp_port: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "max-rtp-port", &max_rtp_port)
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn min_rtp_port(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "min-rtp-port")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn set_min_rtp_port(&self, min_rtp_port: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "min-rtp-port", &min_rtp_port)
    }

    fn connect_add_local_ip_address<F: Fn(&Self, &str) -> bool + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_local_ip_address_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P, &str) -> bool + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            address: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                WebRTCICE::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(address),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-local-ip-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    add_local_ip_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_add_local_ip_address(&self, address: &str) -> bool {
        self.emit_by_name("add-local-ip-address", &[&address])
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_max_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_rtp_port_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WebRTCICE::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-rtp-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_rtp_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn connect_min_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_rtp_port_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WebRTCICE::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-rtp-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_rtp_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}