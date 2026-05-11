//! Persona origin-context Signal contract records.
//!
//! This crate carries typed provenance records after local ingress has
//! already crossed the operating-system trust boundary. It does not
//! define Persona-specific proof material.

mod names;
mod origin;

pub use names::{
    ChannelId, ComponentName, EngineId, HostName, NetworkPeer, OwnerIdentity, RouteId,
    SystemPrincipal, UnixUserId,
};
pub use origin::{ConnectionClass, IngressContext, MessageOrigin};
