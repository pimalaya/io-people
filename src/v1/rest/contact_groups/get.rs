//! Get a People contact group (`contactGroups.get`).
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/get>

use alloc::string::ToString;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::{
        query::to_field_mask,
        rest::contact_groups::{PeopleContactGroup, PeopleGroupField},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// People REST contact group retrieval, by full resource name.
pub struct PeopleContactGroupGet {
    send: PeopleSend<PeopleContactGroup>,
}

impl PeopleContactGroupGet {
    pub fn new(
        auth: &HttpAuthBearer,
        resource_name: &str,
        max_members: Option<u32>,
        group_fields: &[PeopleGroupField],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact group retrieval");
        trace!("resource_name: {resource_name:?}");
        trace!("max_members: {max_members:?}");
        trace!("group_fields: {group_fields:?}");

        if resource_name.trim().is_empty() {
            let err = PeopleSendError::InvalidRequest("Group resource name cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join(resource_name)?;

        {
            let mut pairs = url.query_pairs_mut();
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

impl PeopleCoroutine for PeopleContactGroupGet {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactGroup>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact group retrieved");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
