//! Modify the members of a People contact group
//! (`contactGroups.members.modify`).
//!
//! Contacts can be removed from any group but can only be added to a
//! user group.
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups.members/modify>

use alloc::{format, string::String, vec::Vec};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
};

/// People REST contact group members modification response (the person
/// resource names that could not be processed).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupMembersModifyResponse {
    #[serde(default)]
    pub not_found_resource_names: Vec<String>,
    #[serde(default)]
    pub can_not_remove_last_contact_group_resource_names: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    resource_names_to_add: &'a [String],
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    resource_names_to_remove: &'a [String],
}

/// People REST contact group members modification, adding and/or
/// removing person resource names.
pub struct PeopleContactGroupMembersModify {
    send: PeopleSend<PeopleContactGroupMembersModifyResponse>,
}

impl PeopleContactGroupMembersModify {
    pub fn new(
        auth: &HttpAuthBearer,
        resource_name: &str,
        resource_names_to_add: &[String],
        resource_names_to_remove: &[String],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact group members modification");
        trace!("resource_name: {resource_name:?}");
        trace!("resource_names_to_add: {resource_names_to_add:?}");
        trace!("resource_names_to_remove: {resource_names_to_remove:?}");

        if resource_name.trim().is_empty() {
            let err = PeopleSendError::InvalidRequest("Group resource name cannot be empty".into());
            return Err(err);
        }

        if resource_names_to_add.is_empty() && resource_names_to_remove.is_empty() {
            let err = PeopleSendError::InvalidRequest(
                "Resource names to add and to remove cannot both be empty".into(),
            );
            return Err(err);
        }

        let url = Url::parse(PEOPLE_API_BASE)?.join(&format!("{resource_name}/members:modify"))?;

        let request = Request {
            resource_names_to_add,
            resource_names_to_remove,
        };

        let send = PeopleSend::post_json(auth, url, &request)?;

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupMembersModify {
    type Yield = PeopleYield;
    type Return =
        Result<PeopleSendOutput<PeopleContactGroupMembersModifyResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact group members modified");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
