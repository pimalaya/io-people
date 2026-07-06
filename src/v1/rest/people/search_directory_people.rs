//! Search the domain directory people (`people.searchDirectoryPeople`).
//!
//! Google Workspace only: requires a domain-wide directory.
//!
//! <https://developers.google.com/people/api/rest/v1/people/searchDirectoryPeople>

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

/// Optional query parameters for searching directory people
/// (`people.searchDirectoryPeople`).
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDirectorySearchParams<'a> {
    pub merge_sources: &'a [PeopleDirectoryMergeSourceType],
    pub page_size: Option<u32>,
    pub page_token: Option<&'a str>,
}

/// People REST directory people search response (one page of persons).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDirectorySearchResponse {
    #[serde(default)]
    pub people: Vec<PeoplePerson>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub total_size: Option<u32>,
}

/// People REST directory people search, by plain-text prefix query.
pub struct PeopleDirectorySearch {
    send: PeopleSend<PeopleDirectorySearchResponse>,
}

impl PeopleDirectorySearch {
    pub fn new(
        auth: &HttpAuthBearer,
        query: &str,
        read_mask: &[PeoplePersonField],
        sources: &[PeopleDirectorySourceType],
        params: &PeopleDirectorySearchParams,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people directory search");
        trace!("query: {query:?}");
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

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("./people:searchDirectoryPeople")?;

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("query", query);
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

impl PeopleCoroutine for PeopleDirectorySearch {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleDirectorySearchResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people directory searched");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
