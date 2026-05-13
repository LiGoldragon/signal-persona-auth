use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Stable identifier for one Persona engine instance.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct EngineId(String);

impl EngineId {
    /// Creates an engine identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the engine identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier for a route known by a Persona engine.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct RouteId(String);

impl RouteId {
    /// Creates a route identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the route identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identifier for one communication channel.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct ChannelId(String);

impl ChannelId {
    /// Creates a channel identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the channel identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Host label for remote or local routing provenance.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct HostName(String);

impl HostName {
    /// Creates a host name from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the host name text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Operating-system principal used by a local system service.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct SystemPrincipal(String);

impl SystemPrincipal {
    /// Creates a system principal from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the system principal text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Engine owner identity recorded from local system context.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum OwnerIdentity {
    /// A Unix user owns the engine.
    UnixUser(UnixUserId),
    /// A system principal owns the engine.
    System(SystemPrincipal),
}

impl NotaEncode for OwnerIdentity {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::UnixUser(user_id) => {
                encoder.start_record("UnixUser")?;
                user_id.encode(encoder)?;
                encoder.end_record()
            }
            Self::System(principal) => {
                encoder.start_record("System")?;
                principal.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for OwnerIdentity {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "UnixUser" => {
                decoder.expect_record_head("UnixUser")?;
                let user_id = UnixUserId::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::UnixUser(user_id))
            }
            "System" => {
                decoder.expect_record_head("System")?;
                let principal = SystemPrincipal::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::System(principal))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "OwnerIdentity",
                got: other.to_string(),
            }),
        }
    }
}

/// Unix user identifier captured from the local operating system.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent,
)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct UnixUserId(u32);

impl UnixUserId {
    /// Creates a Unix user identifier.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw Unix user identifier.
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Network peer label captured before cross-host authentication matures.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct NetworkPeer(String);

impl NetworkPeer {
    /// Creates a network peer label.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the network peer label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Supervised local Persona component names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaEnum)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum ComponentName {
    /// Persona central work graph and orchestration component.
    Mind,
    /// Persona message ingress component.
    Message,
    /// Persona router component.
    Router,
    /// Persona terminal component.
    Terminal,
    /// Persona harness component.
    Harness,
    /// Persona system integration component.
    System,
    /// Persona introspection component.
    Introspect,
}
