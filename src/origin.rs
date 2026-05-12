use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{ComponentName, EngineId, HostName, NetworkPeer, SystemPrincipal, UnixUserId};

/// Classifies the local or remote connection after ingress has crossed
/// the operating-system trust boundary.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum ConnectionClass {
    /// The engine owner's own local user context.
    Owner,
    /// A different local Unix user.
    NonOwnerUser(UnixUserId),
    /// A local system service principal.
    System(SystemPrincipal),
    /// A different Persona engine.
    OtherPersona {
        /// Source engine identifier.
        engine_id: EngineId,
        /// Source host label.
        host: HostName,
    },
    /// A network peer before stronger remote trust is designed.
    Network(NetworkPeer),
}

impl NotaEncode for ConnectionClass {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Owner => {
                encoder.start_record("Owner")?;
                encoder.end_record()
            }
            Self::NonOwnerUser(user_id) => {
                encoder.start_record("NonOwnerUser")?;
                user_id.encode(encoder)?;
                encoder.end_record()
            }
            Self::System(principal) => {
                encoder.start_record("System")?;
                principal.encode(encoder)?;
                encoder.end_record()
            }
            Self::OtherPersona { engine_id, host } => {
                encoder.start_record("OtherPersona")?;
                engine_id.encode(encoder)?;
                host.encode(encoder)?;
                encoder.end_record()
            }
            Self::Network(peer) => {
                encoder.start_record("Network")?;
                peer.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for ConnectionClass {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Owner" => {
                decoder.expect_record_head("Owner")?;
                decoder.expect_record_end()?;
                Ok(Self::Owner)
            }
            "NonOwnerUser" => {
                decoder.expect_record_head("NonOwnerUser")?;
                let user_id = UnixUserId::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::NonOwnerUser(user_id))
            }
            "System" => {
                decoder.expect_record_head("System")?;
                let principal = SystemPrincipal::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::System(principal))
            }
            "OtherPersona" => {
                decoder.expect_record_head("OtherPersona")?;
                let engine_id = EngineId::decode(decoder)?;
                let host = HostName::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::OtherPersona { engine_id, host })
            }
            "Network" => {
                decoder.expect_record_head("Network")?;
                let peer = NetworkPeer::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Network(peer))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "ConnectionClass",
                got: other.to_string(),
            }),
        }
    }
}

/// Names the typed origin attached to an incoming frame.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum MessageOrigin {
    /// A frame emitted by a known first-stack Persona component.
    Internal(ComponentName),
    /// A frame emitted by something outside the component mesh.
    External(ConnectionClass),
}

impl NotaEncode for MessageOrigin {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Internal(component) => {
                encoder.start_record("Internal")?;
                component.encode(encoder)?;
                encoder.end_record()
            }
            Self::External(connection_class) => {
                encoder.start_record("External")?;
                connection_class.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for MessageOrigin {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Internal" => {
                decoder.expect_record_head("Internal")?;
                let component = ComponentName::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Internal(component))
            }
            "External" => {
                decoder.expect_record_head("External")?;
                let connection_class = ConnectionClass::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::External(connection_class))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "MessageOrigin",
                got: other.to_string(),
            }),
        }
    }
}

/// Origin context attached to a request after local ingress.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaRecord)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct IngressContext {
    origin: MessageOrigin,
}

impl IngressContext {
    /// Creates an ingress context from an already-classified origin.
    pub fn new(origin: MessageOrigin) -> Self {
        Self { origin }
    }

    /// Creates an ingress context for an internal component.
    pub fn internal(component: ComponentName) -> Self {
        Self::new(MessageOrigin::Internal(component))
    }

    /// Creates an ingress context for an external connection class.
    pub fn external(connection_class: ConnectionClass) -> Self {
        Self::new(MessageOrigin::External(connection_class))
    }

    /// Returns the classified message origin.
    pub fn origin(&self) -> &MessageOrigin {
        &self.origin
    }
}
