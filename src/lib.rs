// Shared: protocol, components, network constants accessible to both client and server
pub mod shared;
// Client: rendering, input, camera, world
#[cfg(feature = "client")]
pub mod client;

// Server: transport, receive loop, authoritative game logic
#[cfg(feature = "server")]
pub mod server;
// Combat: shared between client and server
pub mod combat;

