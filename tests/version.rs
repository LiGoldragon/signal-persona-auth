#![allow(missing_docs)]

use signal_frame::{HandshakeRequest, SIGNAL_FRAME_PROTOCOL_VERSION};

#[test]
fn contract_tracks_current_signal_frame_protocol_version() {
    let request = HandshakeRequest::current();

    assert_eq!(request.version(), SIGNAL_FRAME_PROTOCOL_VERSION);
    assert!(SIGNAL_FRAME_PROTOCOL_VERSION.accepts(request.version()));
}
