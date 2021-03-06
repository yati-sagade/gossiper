#![feature(globs, macro_rules, unsafe_destructor, plugin_registrar, struct_inherit)]
#![allow(missing_doc)]

//! Gossip.rs is a gossip/epidemic protocol based on the
//! paper "Epidemic Broadcast Trees" in which it introduced
//! the novel idea of the Plumtree.
//!
//! Gossip provides a way for a group of nodes, otherwise
//! known as a cluster, to communicate and achieve
//! consensus. However, it's specifically meant to be a
//! highly available system (AP); thus, it supports
//! eventual consistency in face of partitions.
//!
//! This trade-off is fine for many distributed systems
//! that need to be highly available and fault-tolerant. Things
//! such as data processing, analytics, etc... are all great
//! candidates.

extern crate collections;
extern crate uuid;
extern crate rand;
extern crate serialize;
extern crate core;
extern crate sync;
extern crate time;
extern crate msgpack;

pub use result::{GossipResult, GossipError};
pub use protocol::{Node};
pub use stream::{Callback, SockAddr};

mod result;
mod stream;
mod tag;
mod state;
mod protocol;
mod broadcast;
