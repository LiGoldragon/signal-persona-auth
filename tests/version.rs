#![allow(missing_docs)]

use signal_core::{HandshakeRequest, SIGNAL_CORE_PROTOCOL_VERSION};

#[test]
fn contract_tracks_current_signal_core_protocol_version() {
    let request = HandshakeRequest::current();

    assert_eq!(request.version(), SIGNAL_CORE_PROTOCOL_VERSION);
    assert!(SIGNAL_CORE_PROTOCOL_VERSION.accepts(request.version()));
}
