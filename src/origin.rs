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

/// Names the typed origin attached to an incoming frame.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub enum MessageOrigin {
    /// A frame emitted by a known first-stack Persona component.
    Internal(ComponentName),
    /// A frame emitted by something outside the component mesh.
    External(ConnectionClass),
}

/// Origin context attached to a request after local ingress.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
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
