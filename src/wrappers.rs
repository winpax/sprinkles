//! Some types do not appear "nicely" when serialized
//! This module provides wrappers for them that implements custom serialization and deserialization

#![allow(clippy::module_name_repetitions)]

pub mod author;
pub mod bool;
pub mod header;
pub mod serialize;
pub mod sizes;
pub mod time;
