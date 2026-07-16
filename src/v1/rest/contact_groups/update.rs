//! Update a People contact group (`contactGroups.update`).
//!
//! The group must carry its server-assigned resource name and the etag
//! from the latest read; every field named in the update mask is fully
//! replaced.
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/update>

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
    update_group_fields: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    read_group_fields: String,
}

/// People REST contact group update, replacing the masked fields (only
/// `name` and `clientData` can be updated).
pub struct PeopleContactGroupUpdate {
    send: PeopleSend<PeopleContactGroup>,
}

impl PeopleContactGroupUpdate {
    /// Build a contact-group update coroutine. `update_group_fields` names
    /// the fields to replace; `read_group_fields` controls the response.
    pub fn new(
        auth: &HttpAuthBearer,
        group: &PeopleContactGroup,
        update_group_fields: &[PeopleGroupField],
        read_group_fields: &[PeopleGroupField],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact group for update");
        trace!("group: {group:?}");
        trace!("update_group_fields: {update_group_fields:?}");
        trace!("read_group_fields: {read_group_fields:?}");

        if group.resource_name.trim().is_empty() {
            let err = PeopleSendError::InvalidRequest("Group resource name cannot be empty".into());
            return Err(err);
        }

        let url = Url::parse(PEOPLE_API_BASE)?.join(&group.resource_name)?;

        let request = Request {
            contact_group: group,
            update_group_fields: to_field_mask(update_group_fields),
            read_group_fields: to_field_mask(read_group_fields),
        };

        let send = PeopleSend::put_json(auth, url, &request)?;

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupUpdate {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactGroup>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact group updated");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
