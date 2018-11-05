// This file was generated by gir (https://github.com/gtk-rs/gir @ b5ea065)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(feature = "cargo-clippy", allow(approx_constant, type_complexity, unreadable_literal))]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gstreamer_sys as gst;
extern crate gstreamer_sdp_sys as gst_sdp;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstWebRTCDTLSSetup = c_int;
pub const GST_WEBRTC_DTLS_SETUP_NONE: GstWebRTCDTLSSetup = 0;
pub const GST_WEBRTC_DTLS_SETUP_ACTPASS: GstWebRTCDTLSSetup = 1;
pub const GST_WEBRTC_DTLS_SETUP_ACTIVE: GstWebRTCDTLSSetup = 2;
pub const GST_WEBRTC_DTLS_SETUP_PASSIVE: GstWebRTCDTLSSetup = 3;

pub type GstWebRTCDTLSTransportState = c_int;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW: GstWebRTCDTLSTransportState = 0;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED: GstWebRTCDTLSTransportState = 1;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED: GstWebRTCDTLSTransportState = 2;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING: GstWebRTCDTLSTransportState = 3;
pub const GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED: GstWebRTCDTLSTransportState = 4;

pub type GstWebRTCFECType = c_int;
pub const GST_WEBRTC_FEC_TYPE_NONE: GstWebRTCFECType = 0;
pub const GST_WEBRTC_FEC_TYPE_ULP_RED: GstWebRTCFECType = 1;

pub type GstWebRTCICEComponent = c_int;
pub const GST_WEBRTC_ICE_COMPONENT_RTP: GstWebRTCICEComponent = 0;
pub const GST_WEBRTC_ICE_COMPONENT_RTCP: GstWebRTCICEComponent = 1;

pub type GstWebRTCICEConnectionState = c_int;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_NEW: GstWebRTCICEConnectionState = 0;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING: GstWebRTCICEConnectionState = 1;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED: GstWebRTCICEConnectionState = 2;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED: GstWebRTCICEConnectionState = 3;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_FAILED: GstWebRTCICEConnectionState = 4;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED: GstWebRTCICEConnectionState = 5;
pub const GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED: GstWebRTCICEConnectionState = 6;

pub type GstWebRTCICEGatheringState = c_int;
pub const GST_WEBRTC_ICE_GATHERING_STATE_NEW: GstWebRTCICEGatheringState = 0;
pub const GST_WEBRTC_ICE_GATHERING_STATE_GATHERING: GstWebRTCICEGatheringState = 1;
pub const GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE: GstWebRTCICEGatheringState = 2;

pub type GstWebRTCICERole = c_int;
pub const GST_WEBRTC_ICE_ROLE_CONTROLLED: GstWebRTCICERole = 0;
pub const GST_WEBRTC_ICE_ROLE_CONTROLLING: GstWebRTCICERole = 1;

pub type GstWebRTCPeerConnectionState = c_int;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_NEW: GstWebRTCPeerConnectionState = 0;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING: GstWebRTCPeerConnectionState = 1;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED: GstWebRTCPeerConnectionState = 2;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED: GstWebRTCPeerConnectionState = 3;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_FAILED: GstWebRTCPeerConnectionState = 4;
pub const GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED: GstWebRTCPeerConnectionState = 5;

pub type GstWebRTCRTPTransceiverDirection = c_int;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_NONE: GstWebRTCRTPTransceiverDirection = 0;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_INACTIVE: GstWebRTCRTPTransceiverDirection = 1;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDONLY: GstWebRTCRTPTransceiverDirection = 2;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_RECVONLY: GstWebRTCRTPTransceiverDirection = 3;
pub const GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDRECV: GstWebRTCRTPTransceiverDirection = 4;

pub type GstWebRTCSDPType = c_int;
pub const GST_WEBRTC_SDP_TYPE_OFFER: GstWebRTCSDPType = 1;
pub const GST_WEBRTC_SDP_TYPE_PRANSWER: GstWebRTCSDPType = 2;
pub const GST_WEBRTC_SDP_TYPE_ANSWER: GstWebRTCSDPType = 3;
pub const GST_WEBRTC_SDP_TYPE_ROLLBACK: GstWebRTCSDPType = 4;

pub type GstWebRTCSignalingState = c_int;
pub const GST_WEBRTC_SIGNALING_STATE_STABLE: GstWebRTCSignalingState = 0;
pub const GST_WEBRTC_SIGNALING_STATE_CLOSED: GstWebRTCSignalingState = 1;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER: GstWebRTCSignalingState = 2;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER: GstWebRTCSignalingState = 3;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER: GstWebRTCSignalingState = 4;
pub const GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER: GstWebRTCSignalingState = 5;

pub type GstWebRTCStatsType = c_int;
pub const GST_WEBRTC_STATS_CODEC: GstWebRTCStatsType = 1;
pub const GST_WEBRTC_STATS_INBOUND_RTP: GstWebRTCStatsType = 2;
pub const GST_WEBRTC_STATS_OUTBOUND_RTP: GstWebRTCStatsType = 3;
pub const GST_WEBRTC_STATS_REMOTE_INBOUND_RTP: GstWebRTCStatsType = 4;
pub const GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP: GstWebRTCStatsType = 5;
pub const GST_WEBRTC_STATS_CSRC: GstWebRTCStatsType = 6;
pub const GST_WEBRTC_STATS_PEER_CONNECTION: GstWebRTCStatsType = 7;
pub const GST_WEBRTC_STATS_DATA_CHANNEL: GstWebRTCStatsType = 8;
pub const GST_WEBRTC_STATS_STREAM: GstWebRTCStatsType = 9;
pub const GST_WEBRTC_STATS_TRANSPORT: GstWebRTCStatsType = 10;
pub const GST_WEBRTC_STATS_CANDIDATE_PAIR: GstWebRTCStatsType = 11;
pub const GST_WEBRTC_STATS_LOCAL_CANDIDATE: GstWebRTCStatsType = 12;
pub const GST_WEBRTC_STATS_REMOTE_CANDIDATE: GstWebRTCStatsType = 13;
pub const GST_WEBRTC_STATS_CERTIFICATE: GstWebRTCStatsType = 14;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCDTLSTransportClass {
    pub parent_class: gst::GstObjectClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCDTLSTransportClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCDTLSTransportClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCICETransportClass {
    pub parent_class: gst::GstObjectClass,
    pub gather_candidates: Option<unsafe extern "C" fn(*mut GstWebRTCICETransport) -> gboolean>,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICETransportClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICETransportClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("gather_candidates", &self.gather_candidates)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPReceiverClass {
    pub parent_class: gst::GstObjectClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPReceiverClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPReceiverClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPSenderClass {
    pub parent_class: gst::GstObjectClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPSenderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPSenderClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPTransceiverClass {
    pub parent_class: gst::GstObjectClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPTransceiverClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPTransceiverClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCSessionDescription {
    pub type_: GstWebRTCSDPType,
    pub sdp: *mut gst_sdp::GstSDPMessage,
}

impl ::std::fmt::Debug for GstWebRTCSessionDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCSessionDescription @ {:?}", self as *const _))
         .field("type_", &self.type_)
         .field("sdp", &self.sdp)
         .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCDTLSTransport {
    pub parent: gst::GstObject,
    pub transport: *mut GstWebRTCICETransport,
    pub state: GstWebRTCDTLSTransportState,
    pub is_rtcp: gboolean,
    pub client: gboolean,
    pub session_id: c_uint,
    pub dtlssrtpenc: *mut gst::GstElement,
    pub dtlssrtpdec: *mut gst::GstElement,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCDTLSTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCDTLSTransport @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("transport", &self.transport)
         .field("state", &self.state)
         .field("is_rtcp", &self.is_rtcp)
         .field("client", &self.client)
         .field("session_id", &self.session_id)
         .field("dtlssrtpenc", &self.dtlssrtpenc)
         .field("dtlssrtpdec", &self.dtlssrtpdec)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCICETransport {
    pub parent: gst::GstObject,
    pub role: GstWebRTCICERole,
    pub component: GstWebRTCICEComponent,
    pub state: GstWebRTCICEConnectionState,
    pub gathering_state: GstWebRTCICEGatheringState,
    pub src: *mut gst::GstElement,
    pub sink: *mut gst::GstElement,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCICETransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCICETransport @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("role", &self.role)
         .field("component", &self.component)
         .field("state", &self.state)
         .field("gathering_state", &self.gathering_state)
         .field("src", &self.src)
         .field("sink", &self.sink)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPReceiver {
    pub parent: gst::GstObject,
    pub transport: *mut GstWebRTCDTLSTransport,
    pub rtcp_transport: *mut GstWebRTCDTLSTransport,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPReceiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPReceiver @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("transport", &self.transport)
         .field("rtcp_transport", &self.rtcp_transport)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPSender {
    pub parent: gst::GstObject,
    pub transport: *mut GstWebRTCDTLSTransport,
    pub rtcp_transport: *mut GstWebRTCDTLSTransport,
    pub send_encodings: *mut glib::GArray,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPSender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPSender @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("transport", &self.transport)
         .field("rtcp_transport", &self.rtcp_transport)
         .field("send_encodings", &self.send_encodings)
         .field("_padding", &self._padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstWebRTCRTPTransceiver {
    pub parent: gst::GstObject,
    pub mline: c_uint,
    pub mid: *mut c_char,
    pub stopped: gboolean,
    pub sender: *mut GstWebRTCRTPSender,
    pub receiver: *mut GstWebRTCRTPReceiver,
    pub direction: GstWebRTCRTPTransceiverDirection,
    pub current_direction: GstWebRTCRTPTransceiverDirection,
    pub codec_preferences: *mut gst::GstCaps,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstWebRTCRTPTransceiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstWebRTCRTPTransceiver @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("mline", &self.mline)
         .field("mid", &self.mid)
         .field("stopped", &self.stopped)
         .field("sender", &self.sender)
         .field("receiver", &self.receiver)
         .field("direction", &self.direction)
         .field("current_direction", &self.current_direction)
         .field("codec_preferences", &self.codec_preferences)
         .field("_padding", &self._padding)
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // GstWebRTCDTLSSetup
    //=========================================================================
    pub fn gst_webrtc_dtls_setup_get_type() -> GType;

    //=========================================================================
    // GstWebRTCDTLSTransportState
    //=========================================================================
    pub fn gst_webrtc_dtls_transport_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCFECType
    //=========================================================================
    pub fn gst_webrtc_fec_type_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEComponent
    //=========================================================================
    pub fn gst_webrtc_ice_component_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEConnectionState
    //=========================================================================
    pub fn gst_webrtc_ice_connection_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICEGatheringState
    //=========================================================================
    pub fn gst_webrtc_ice_gathering_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCICERole
    //=========================================================================
    pub fn gst_webrtc_ice_role_get_type() -> GType;

    //=========================================================================
    // GstWebRTCPeerConnectionState
    //=========================================================================
    pub fn gst_webrtc_peer_connection_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCRTPTransceiverDirection
    //=========================================================================
    pub fn gst_webrtc_rtp_transceiver_direction_get_type() -> GType;

    //=========================================================================
    // GstWebRTCSDPType
    //=========================================================================
    pub fn gst_webrtc_sdp_type_get_type() -> GType;
    pub fn gst_webrtc_sdp_type_to_string(type_: GstWebRTCSDPType) -> *const c_char;

    //=========================================================================
    // GstWebRTCSignalingState
    //=========================================================================
    pub fn gst_webrtc_signaling_state_get_type() -> GType;

    //=========================================================================
    // GstWebRTCStatsType
    //=========================================================================
    pub fn gst_webrtc_stats_type_get_type() -> GType;

    //=========================================================================
    // GstWebRTCSessionDescription
    //=========================================================================
    pub fn gst_webrtc_session_description_get_type() -> GType;
    pub fn gst_webrtc_session_description_new(type_: GstWebRTCSDPType, sdp: *mut gst_sdp::GstSDPMessage) -> *mut GstWebRTCSessionDescription;
    pub fn gst_webrtc_session_description_copy(src: *const GstWebRTCSessionDescription) -> *mut GstWebRTCSessionDescription;
    pub fn gst_webrtc_session_description_free(desc: *mut GstWebRTCSessionDescription);

    //=========================================================================
    // GstWebRTCDTLSTransport
    //=========================================================================
    pub fn gst_webrtc_dtls_transport_get_type() -> GType;
    pub fn gst_webrtc_dtls_transport_new(session_id: c_uint, rtcp: gboolean) -> *mut GstWebRTCDTLSTransport;
    pub fn gst_webrtc_dtls_transport_set_transport(transport: *mut GstWebRTCDTLSTransport, ice: *mut GstWebRTCICETransport);

    //=========================================================================
    // GstWebRTCICETransport
    //=========================================================================
    pub fn gst_webrtc_ice_transport_get_type() -> GType;
    pub fn gst_webrtc_ice_transport_connection_state_change(ice: *mut GstWebRTCICETransport, new_state: GstWebRTCICEConnectionState);
    pub fn gst_webrtc_ice_transport_gathering_state_change(ice: *mut GstWebRTCICETransport, new_state: GstWebRTCICEGatheringState);
    pub fn gst_webrtc_ice_transport_new_candidate(ice: *mut GstWebRTCICETransport, stream_id: c_uint, component: GstWebRTCICEComponent, attr: *mut c_char);
    pub fn gst_webrtc_ice_transport_selected_pair_change(ice: *mut GstWebRTCICETransport);

    //=========================================================================
    // GstWebRTCRTPReceiver
    //=========================================================================
    pub fn gst_webrtc_rtp_receiver_get_type() -> GType;
    pub fn gst_webrtc_rtp_receiver_new() -> *mut GstWebRTCRTPReceiver;
    pub fn gst_webrtc_rtp_receiver_set_rtcp_transport(receiver: *mut GstWebRTCRTPReceiver, transport: *mut GstWebRTCDTLSTransport);
    pub fn gst_webrtc_rtp_receiver_set_transport(receiver: *mut GstWebRTCRTPReceiver, transport: *mut GstWebRTCDTLSTransport);

    //=========================================================================
    // GstWebRTCRTPSender
    //=========================================================================
    pub fn gst_webrtc_rtp_sender_get_type() -> GType;
    pub fn gst_webrtc_rtp_sender_new() -> *mut GstWebRTCRTPSender;
    pub fn gst_webrtc_rtp_sender_set_rtcp_transport(sender: *mut GstWebRTCRTPSender, transport: *mut GstWebRTCDTLSTransport);
    pub fn gst_webrtc_rtp_sender_set_transport(sender: *mut GstWebRTCRTPSender, transport: *mut GstWebRTCDTLSTransport);

    //=========================================================================
    // GstWebRTCRTPTransceiver
    //=========================================================================
    pub fn gst_webrtc_rtp_transceiver_get_type() -> GType;

}
