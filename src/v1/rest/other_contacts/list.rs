//! List the authenticated user's "Other contacts"
//! (`otherContacts.list`).
//!
//! Set `request_sync_token` on the first full listing, then pass the
//! returned `next_sync_token` back as `sync_token` to fetch incremental
//! changes.
//!
//! <https://developers.google.com/people/api/rest/v1/otherContacts/list>

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
        rest::people::{PeoplePerson, PeoplePersonField, PeopleReadSourceType},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// Optional query parameters for listing "Other contacts"
/// (`otherContacts.list`).
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleOtherContactsListParams<'a> {
    /// Maximum number of contacts to return per page (1–1000; default 100).
    pub page_size: Option<u32>,
    /// Page token received from a previous response's `next_page_token`.
    pub page_token: Option<&'a str>,
    /// When `true`, a `next_sync_token` is included in the last response
    /// page for use in subsequent incremental sync requests.
    #[serde(skip_serializing_if = "crate::v1::query::is_false")]
    pub request_sync_token: bool,
    /// Token from a prior `next_sync_token`; causes the response to
    /// return only changes since the previous full sync.
    pub sync_token: Option<&'a str>,
    /// Data sources to include in the response.
    pub sources: &'a [PeopleReadSourceType],
}

/// People REST "Other contacts" listing response (one page of persons).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleOtherContactsListResponse {
    /// Persons returned for this page of results.
    #[serde(default)]
    pub other_contacts: Vec<PeoplePerson>,
    /// Token to retrieve the next page; absent on the last page.
    #[serde(default)]
    pub next_page_token: Option<String>,
    /// Token to use in a future incremental sync; present only on the
    /// last page when `request_sync_token` was set.
    #[serde(default)]
    pub next_sync_token: Option<String>,
    /// Total number of contacts in the list, without page filtering.
    #[serde(default)]
    pub total_size: Option<u32>,
}

/// People REST "Other contacts" listing, wrapping a page of persons.
pub struct PeopleOtherContactsList {
    send: PeopleSend<PeopleOtherContactsListResponse>,
}

impl PeopleOtherContactsList {
    /// Build a coroutine that lists "Other contacts" with the given
    /// `read_mask` fields and optional query `params`.
    pub fn new(
        auth: &HttpAuthBearer,
        read_mask: &[PeoplePersonField],
        params: &PeopleOtherContactsListParams,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people other contacts listing");
        trace!("read_mask: {read_mask:?}");
        trace!("params: {params:?}");

        if read_mask.is_empty() {
            let err = PeopleSendError::InvalidRequest("Read mask cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("otherContacts")?;

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("readMask", &to_field_mask(read_mask));
            pairs.extend_pairs(to_query_pairs(params));
        }

        let send = PeopleSend::get(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleOtherContactsList {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleOtherContactsListResponse>, PeopleSendError>;

    /// Drive the HTTP exchange one step; yields I/O wants until the
    /// response is fully received, then completes with the parsed page.
    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people other contacts listed");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
