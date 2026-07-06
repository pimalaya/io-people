//! People API v1.
//!
//! `rest` mirrors the REST resource tree (`people`, `contactGroups`,
//! `otherContacts`); `send` and `query` are the shared request
//! primitives, and `client` the optional std driver.

#[cfg(feature = "client")]
pub mod client;
pub mod query;
pub mod rest;
pub mod send;
