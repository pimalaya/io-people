//! Get a batch of People contact groups (`contactGroups.batchGet`).
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/batchGet>

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::{
        query::to_field_mask,
        rest::contact_groups::{PeopleContactGroupResponse, PeopleGroupField},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// People REST contact groups batch retrieval response (one entry per
/// requested resource name).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupsBatchGetResponse {
    #[serde(default)]
    pub responses: Vec<PeopleContactGroupResponse>,
}

/// People REST contact groups batch retrieval, by full resource names
/// (200 max).
pub struct PeopleContactGroupsBatchGet {
    send: PeopleSend<PeopleContactGroupsBatchGetResponse>,
}

impl PeopleContactGroupsBatchGet {
    pub fn new(
        auth: &HttpAuthBearer,
        resource_names: &[String],
        max_members: Option<u32>,
        group_fields: &[PeopleGroupField],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact groups batch retrieval");
        trace!("resource_names: {resource_names:?}");
        trace!("max_members: {max_members:?}");
        trace!("group_fields: {group_fields:?}");

        if resource_names.is_empty() {
            let err = PeopleSendError::InvalidRequest("Resource names cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join("./contactGroups:batchGet")?;

        {
            let mut pairs = url.query_pairs_mut();
            for resource_name in resource_names {
                pairs.append_pair("resourceNames", resource_name);
            }
            if let Some(max_members) = max_members {
                pairs.append_pair("maxMembers", &max_members.to_string());
            }
            if !group_fields.is_empty() {
                pairs.append_pair("groupFields", &to_field_mask(group_fields));
            }
        }

        let send = PeopleSend::get(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupsBatchGet {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactGroupsBatchGetResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact groups batch retrieved");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
