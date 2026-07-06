//! People contact groups (`contactGroups`): list, get, batchGet, create,
//! update, delete.
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups>

mod types;
#[doc(inline)]
pub use types::*;

pub mod batch_get;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod members;
pub mod update;
