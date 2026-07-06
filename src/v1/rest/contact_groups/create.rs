//! Create a People contact group (`contactGroups.create`).
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/create>

use alloc::string::String;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::Serialize;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    contact_group: &'a PeopleContactGroup,
    #[serde(skip_serializing_if = "String::is_empty")]
    read_group_fields: String,
}

/// People REST contact group creation, from a whole group resource.
pub struct PeopleContactGroupCreate {
    send: PeopleSend<PeopleContactGroup>,
}

impl PeopleContactGroupCreate {
    pub fn new(
        auth: &HttpAuthBearer,
        group: &PeopleContactGroup,
        read_group_fields: &[PeopleGroupField],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact group for creation");
        trace!("group: {group:?}");
        trace!("read_group_fields: {read_group_fields:?}");

        let name_is_empty = group
            .name
            .as_deref()
            .is_none_or(|name| name.trim().is_empty());

        if name_is_empty {
            let err = PeopleSendError::InvalidRequest("Group name cannot be empty".into());
            return Err(err);
        }

        let url = Url::parse(PEOPLE_API_BASE)?.join("contactGroups")?;

        let request = Request {
            contact_group: group,
            read_group_fields: to_field_mask(read_group_fields),
        };

        let send = PeopleSend::post_json(auth, url, &request)?;

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupCreate {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactGroup>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact group created");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
