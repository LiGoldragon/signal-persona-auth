use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Stable identifier for one Persona engine instance.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct EngineId {
    value: String,
}

impl EngineId {
    /// Creates an engine identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the engine identifier text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Stable identifier for a route known by a Persona engine.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct RouteId {
    value: String,
}

impl RouteId {
    /// Creates a route identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the route identifier text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Stable identifier for one communication channel.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct ChannelId {
    value: String,
}

impl ChannelId {
    /// Creates a channel identifier from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the channel identifier text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Host label for remote or local routing provenance.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct HostName {
    value: String,
}

impl HostName {
    /// Creates a host name from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the host name text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Operating-system principal used by a local system service.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct SystemPrincipal {
    value: String,
}

impl SystemPrincipal {
    /// Creates a system principal from an external label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the system principal text.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Unix user identifier captured from the local operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct UnixUserId {
    value: u32,
}

impl UnixUserId {
    /// Creates a Unix user identifier.
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Returns the raw Unix user identifier.
    pub fn as_u32(&self) -> u32 {
        self.value
    }
}

/// Network peer label captured before cross-host authentication matures.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct NetworkPeer {
    value: String,
}

impl NetworkPeer {
    /// Creates a network peer label.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the network peer label.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// First-stack Persona component names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum ComponentName {
    /// Persona infrastructure supervisor.
    PersonaDaemon,
    /// Persona central work graph and orchestration component.
    Mind,
    /// Persona message ingress and delivery proxy.
    Message,
    /// Persona router component.
    Router,
    /// Persona terminal component.
    Terminal,
    /// Persona harness component.
    Harness,
    /// Persona system integration component.
    System,
}
