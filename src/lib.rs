pub mod networking;
pub mod encryption;
pub mod heartbeat;

// High-level functions to start the server, send messages, etc.
pub use networking::{start_server, send_message, broadcast_presence, listen_for_peers};
pub use encryption::{encrypt_message, decrypt_message};
pub use heartbeat::{send_heartbeat, handle_heartbeat};
