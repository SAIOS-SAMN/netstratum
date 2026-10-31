//! Attachment to Netstratum through the Mycelium hypha (docs/109).
//!
//! An external agent or sandbox connects to a hypha with its own key, calls
//! `attach` on the house's door seat with the metabolism it needs, and is
//! placed on the most viable free world, where it holds a world seat and acts
//! through it. This crate is the single definition of that interface:
//!
//! - [`iface`]: the seats, functions, percept kinds and codes;
//! - [`files`]: the file exchange between the house and the conductor;
//! - [`client`]: the library an external model uses to attach and act.

pub mod client;
pub mod files;
pub mod iface;

pub use iface::*;
