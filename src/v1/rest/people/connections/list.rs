//! List the authenticated user's contacts
//! (`people.connections.list`).
//!
//! Set `request_sync_token` on the first full listing, then pass the
//! returned `next_sync_token` back as `sync_token` to fetch incremental
//! changes (deleted contacts come back with only `metadata.deleted`).
//!
//! <https://developers.google.com/people/api/rest/v1/people.connections/list>

use alloc::{string::String, vec::Vec};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::{
        query::{to_field_mask, to_query_pairs},
        rest::people::{PeoplePerson, PeoplePersonField, PeopleReadSourceType, PeopleSortOrder},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// Optional query parameters for listing connections
/// (`people.connections.list`).
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleConnectionsListParams<'a> {
    /// Maximum number of connections to return per page (max 1000).
    pub page_size: Option<u32>,
    /// Page token from a previous response, used to retrieve the next page.
    pub page_token: Option<&'a str>,
    /// When true, a `next_sync_token` is included in the final page response.
    #[serde(skip_serializing_if = "crate::v1::query::is_false")]
    pub request_sync_token: bool,
    /// Sync token from a previous full listing, for incremental change fetch.
    pub sync_token: Option<&'a str>,
    /// Sort order for the returned connections.
    pub sort_order: Option<PeopleSortOrder>,
    /// Data sources to include; defaults to all sources when empty.
    pub sources: &'a [PeopleReadSourceType],
}

/// People REST connections listing response (one page of persons).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleConnectionsListResponse {
    /// Contacts returned for this page.
    #[serde(default)]
    pub connections: Vec<PeoplePerson>,
    /// Token for retrieving the next page; absent on the final page.
    #[serde(default)]
    pub next_page_token: Option<String>,
    /// Token for fetching incremental changes on subsequent calls.
    #[serde(default)]
    pub next_sync_token: Option<String>,
    /// Total number of people in the list without pagination.
    #[serde(default)]
    pub total_people: Option<u32>,
    /// Total number of items in the list without pagination.
    #[serde(default)]
    pub total_items: Option<u32>,
}

/// People REST connections listing, wrapping a page of the authenticated
/// user's contacts (only `people/me` is a valid owner).
pub struct PeopleConnectionsList {
    send: PeopleSend<PeopleConnectionsListResponse>,
}

impl PeopleConnectionsList {
    /// Build a new connections listing coroutine.
    ///
    /// `person_fields` must be non-empty; `params` carries optional
    /// pagination and sync-token arguments.
    pub fn new(
        auth: &HttpAuthBearer,
        person_fields: &[PeoplePersonField],
        params: &PeopleConnectionsListParams,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people connections listing");
        trace!("person_fields: {person_fields:?}");
        trace!("params: {params:?}");

        if person_fields.is_empty() {
            let err = PeopleSendError::InvalidRequest("Person fields cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("people/me/connections")?;

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("personFields", &to_field_mask(person_fields));
            pairs.extend_pairs(to_query_pairs(params));
        }

        let send = PeopleSend::get(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleConnectionsList {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleConnectionsListResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people connections listed");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
