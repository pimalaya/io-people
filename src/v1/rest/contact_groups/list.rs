//! List the People contact groups (`contactGroups.list`).
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/list>

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
        rest::contact_groups::{PeopleContactGroup, PeopleGroupField},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// Optional query parameters for listing contact groups
/// (`contactGroups.list`).
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupsListParams<'a> {
    pub page_size: Option<u32>,
    pub page_token: Option<&'a str>,
    pub sync_token: Option<&'a str>,
}

/// People REST contact groups listing response (one page of groups).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupsListResponse {
    #[serde(default)]
    pub contact_groups: Vec<PeopleContactGroup>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub next_sync_token: Option<String>,
    #[serde(default)]
    pub total_items: Option<u32>,
}

/// People REST contact groups listing, wrapping a page of groups.
pub struct PeopleContactGroupsList {
    send: PeopleSend<PeopleContactGroupsListResponse>,
}

impl PeopleContactGroupsList {
    pub fn new(
        auth: &HttpAuthBearer,
        group_fields: &[PeopleGroupField],
        params: &PeopleContactGroupsListParams,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact groups listing");
        trace!("group_fields: {group_fields:?}");
        trace!("params: {params:?}");

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("contactGroups")?;

        {
            let mut pairs = url.query_pairs_mut();
            if !group_fields.is_empty() {
                pairs.append_pair("groupFields", &to_field_mask(group_fields));
            }
            pairs.extend_pairs(to_query_pairs(params));
        }

        let send = PeopleSend::get(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupsList {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactGroupsListResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact groups listed");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
