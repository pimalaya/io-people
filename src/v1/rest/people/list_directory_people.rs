//! List the domain directory people (`people.listDirectoryPeople`).
//!
//! Google Workspace only: requires a domain-wide directory.
//!
//! <https://developers.google.com/people/api/rest/v1/people/listDirectoryPeople>

use alloc::{string::String, vec::Vec};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::{
        query::{to_field_mask, to_query_pairs},
        rest::people::{
            PeopleDirectoryMergeSourceType, PeopleDirectorySourceType, PeoplePerson,
            PeoplePersonField,
        },
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// Optional query parameters for listing directory people
/// (`people.listDirectoryPeople`).
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDirectoryListParams<'a> {
    pub merge_sources: &'a [PeopleDirectoryMergeSourceType],
    pub page_size: Option<u32>,
    pub page_token: Option<&'a str>,
    #[serde(skip_serializing_if = "crate::v1::query::is_false")]
    pub request_sync_token: bool,
    pub sync_token: Option<&'a str>,
}

/// People REST directory people listing response (one page of persons).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDirectoryListResponse {
    #[serde(default)]
    pub people: Vec<PeoplePerson>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub next_sync_token: Option<String>,
}

/// People REST directory people listing, wrapping a page of persons.
pub struct PeopleDirectoryList {
    send: PeopleSend<PeopleDirectoryListResponse>,
}

impl PeopleDirectoryList {
    pub fn new(
        auth: &HttpAuthBearer,
        read_mask: &[PeoplePersonField],
        sources: &[PeopleDirectorySourceType],
        params: &PeopleDirectoryListParams,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people directory listing");
        trace!("read_mask: {read_mask:?}");
        trace!("sources: {sources:?}");
        trace!("params: {params:?}");

        if read_mask.is_empty() {
            let err = PeopleSendError::InvalidRequest("Read mask cannot be empty".into());
            return Err(err);
        }

        if sources.is_empty() {
            let err = PeopleSendError::InvalidRequest("Directory sources cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("./people:listDirectoryPeople")?;

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("readMask", &to_field_mask(read_mask));
            for source in sources {
                pairs.append_pair("sources", to_variant_name(source).unwrap_or_default());
            }
            pairs.extend_pairs(to_query_pairs(params));
        }

        let send = PeopleSend::get(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleDirectoryList {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleDirectoryListResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people directory listed");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
