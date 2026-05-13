#![allow(missing_docs)]

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use pretty_assertions::assert_eq;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::{Frame, FrameBody, Request};
use signal_persona_auth::{
    ChannelId, ComponentName, ConnectionClass, EngineId, HostName, IngressContext, MessageOrigin,
    NetworkPeer, OwnerIdentity, RouteId, SystemPrincipal, UnixUserId,
};

#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
enum Probe {
    EngineId(EngineId),
    RouteId(RouteId),
    ChannelId(ChannelId),
    ComponentName(ComponentName),
    OwnerIdentity(OwnerIdentity),
    ConnectionClass(ConnectionClass),
    MessageOrigin(MessageOrigin),
    IngressContext(IngressContext),
}

fn round_trip(probe: Probe) -> Probe {
    let frame = Frame::<Probe, Probe>::new(FrameBody::Request(Request::assert(probe)));
    let bytes = frame
        .encode_length_prefixed()
        .expect("frame should serialize");
    let decoded =
        Frame::<Probe, Probe>::decode_length_prefixed(&bytes).expect("frame should deserialize");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => payload,
        _ => panic!("expected request operation frame"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn string_backed_identifiers_round_trip() {
    assert_eq!(
        round_trip(Probe::EngineId(EngineId::new("engine-main"))),
        Probe::EngineId(EngineId::new("engine-main"))
    );
    assert_eq!(
        round_trip(Probe::RouteId(RouteId::new("router-to-message"))),
        Probe::RouteId(RouteId::new("router-to-message"))
    );
    assert_eq!(
        round_trip(Probe::ChannelId(ChannelId::new("message-submit"))),
        Probe::ChannelId(ChannelId::new("message-submit"))
    );
}

#[test]
fn engine_identifier_round_trips_through_nota_text() {
    round_trip_nota(EngineId::new("engine-main"), "engine-main");
}

#[test]
fn component_name_covers_first_stack_components() {
    let components = [
        ComponentName::Mind,
        ComponentName::Message,
        ComponentName::Router,
        ComponentName::Terminal,
        ComponentName::Harness,
        ComponentName::System,
    ];

    for component in components {
        assert_eq!(
            round_trip(Probe::ComponentName(component)),
            Probe::ComponentName(component)
        );
    }
}

#[test]
fn owner_identity_variants_round_trip() {
    let identities = [
        OwnerIdentity::UnixUser(UnixUserId::new(1000)),
        OwnerIdentity::System(SystemPrincipal::new("persona")),
    ];

    for identity in identities {
        assert_eq!(
            round_trip(Probe::OwnerIdentity(identity.clone())),
            Probe::OwnerIdentity(identity)
        );
    }
}

#[test]
fn connection_class_variants_round_trip() {
    let classes = [
        ConnectionClass::Owner,
        ConnectionClass::NonOwnerUser(UnixUserId::new(1000)),
        ConnectionClass::System(SystemPrincipal::new("systemd-user")),
        ConnectionClass::OtherPersona {
            engine_id: EngineId::new("engine-remote"),
            host: HostName::new("workstation"),
        },
        ConnectionClass::Network(NetworkPeer::new("peer-a")),
    ];

    for class in classes {
        assert_eq!(
            round_trip(Probe::ConnectionClass(class.clone())),
            Probe::ConnectionClass(class)
        );
    }
}

#[test]
fn message_origin_variants_round_trip() {
    let origins = [
        MessageOrigin::Internal(ComponentName::Router),
        MessageOrigin::External(ConnectionClass::Owner),
    ];

    for origin in origins {
        assert_eq!(
            round_trip(Probe::MessageOrigin(origin.clone())),
            Probe::MessageOrigin(origin)
        );
    }
}

#[test]
fn ingress_context_carries_origin_without_proof_material() {
    let contexts = [
        IngressContext::internal(ComponentName::Message),
        IngressContext::external(ConnectionClass::NonOwnerUser(UnixUserId::new(2000))),
    ];

    for context in contexts {
        assert_eq!(
            round_trip(Probe::IngressContext(context.clone())),
            Probe::IngressContext(context)
        );
    }
}

#[test]
fn ingress_context_round_trips_through_nota_text_without_proof_material() {
    round_trip_nota(
        IngressContext::external(ConnectionClass::NonOwnerUser(UnixUserId::new(2000))),
        "(IngressContext (External (NonOwnerUser 2000)))",
    );
}

#[test]
fn source_does_not_define_persona_auth_proof() {
    let source_files = [
        std::fs::read_to_string("src/lib.rs").expect("read lib"),
        std::fs::read_to_string("src/names.rs").expect("read names"),
        std::fs::read_to_string("src/origin.rs").expect("read origin"),
    ];

    for source in source_files {
        assert!(
            !source.contains("AuthProof"),
            "signal-persona-auth must not define a Persona-specific AuthProof type"
        );
    }
}
