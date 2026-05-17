//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end and asserts each
//! record's NOTA round-trip equals the canonical form.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona_auth::{
    ChannelId, ComponentInstanceName, ComponentName, ConnectionClass, EngineId, HostName,
    IngressContext, InternalComponentInstanceOrigin, MessageOrigin, NetworkPeer, OwnerIdentity,
    RouteId, SystemPrincipal, UnixUserId,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn round_trip<T>(value: T, canonical_text: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    assert_eq!(text, canonical_text, "encode for {value:?}");

    let mut decoder = Decoder::new(canonical_text);
    let decoded = T::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, value, "decode for {canonical_text}");

    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.nota missing line: {canonical_text}",
    );
}

#[test]
fn canonical_identifiers_round_trip() {
    round_trip(EngineId::new("prototype"), "prototype");
    round_trip(
        RouteId::new("internal-message-router"),
        "internal-message-router",
    );
    round_trip(
        ChannelId::new("internal-message-router"),
        "internal-message-router",
    );
    round_trip(ComponentInstanceName::new("initiator"), "initiator");
    round_trip(HostName::new("goldragon"), "goldragon");
    round_trip(SystemPrincipal::new("persona-system"), "persona-system");
    round_trip(UnixUserId::new(1000), "1000");
    round_trip(NetworkPeer::new("peer-7"), "peer-7");
}

#[test]
fn canonical_component_names_round_trip() {
    round_trip(ComponentName::Mind, "Mind");
    round_trip(ComponentName::Message, "Message");
    round_trip(ComponentName::Router, "Router");
    round_trip(ComponentName::Terminal, "Terminal");
    round_trip(ComponentName::Harness, "Harness");
    round_trip(ComponentName::Introspect, "Introspect");
}

#[test]
fn canonical_owner_identity_round_trips() {
    round_trip(
        OwnerIdentity::UnixUser(UnixUserId::new(1000)),
        "(UnixUser 1000)",
    );
}

#[test]
fn canonical_connection_class_round_trips() {
    round_trip(ConnectionClass::Owner, "(Owner)");
    round_trip(
        ConnectionClass::NonOwnerUser(UnixUserId::new(1001)),
        "(NonOwnerUser 1001)",
    );
    round_trip(
        ConnectionClass::System(SystemPrincipal::new("persona-system")),
        "(System persona-system)",
    );
    round_trip(
        ConnectionClass::OtherPersona {
            engine_id: EngineId::new("other-engine"),
            host: HostName::new("other-host"),
        },
        "(OtherPersona other-engine other-host)",
    );
    round_trip(
        ConnectionClass::Network(NetworkPeer::new("peer-7")),
        "(Network peer-7)",
    );
}

#[test]
fn canonical_message_origin_round_trips() {
    round_trip(
        MessageOrigin::Internal(ComponentName::Router),
        "(Internal Router)",
    );
    round_trip(
        MessageOrigin::InternalComponentInstance(InternalComponentInstanceOrigin::new(
            ComponentName::Harness,
            ComponentInstanceName::new("initiator"),
        )),
        "(InternalComponentInstance (InternalComponentInstanceOrigin Harness initiator))",
    );
    round_trip(
        MessageOrigin::External(ConnectionClass::Owner),
        "(External (Owner))",
    );
}

#[test]
fn canonical_ingress_context_round_trips() {
    round_trip(
        IngressContext::internal(ComponentName::Router),
        "(IngressContext (Internal Router))",
    );
    round_trip(
        IngressContext::internal_component_instance(InternalComponentInstanceOrigin::new(
            ComponentName::Harness,
            ComponentInstanceName::new("reviewer"),
        )),
        "(IngressContext (InternalComponentInstance (InternalComponentInstanceOrigin Harness reviewer)))",
    );
    round_trip(
        IngressContext::external(ConnectionClass::Owner),
        "(IngressContext (External (Owner)))",
    );
}
