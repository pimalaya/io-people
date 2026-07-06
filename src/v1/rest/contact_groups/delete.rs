//! Delete a People contact group (`contactGroups.delete`).
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups/delete>

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::send::{PEOPLE_API_BASE, PeopleNoResponse, PeopleSend, PeopleSendError, PeopleSendOutput},
};

/// People REST contact group deletion, optionally deleting its contacts
/// too.
pub struct PeopleContactGroupDelete {
    send: PeopleSend<PeopleNoResponse>,
}

impl PeopleContactGroupDelete {
    pub fn new(
        auth: &HttpAuthBearer,
        resource_name: &str,
        delete_contacts: bool,
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact group for deletion");
        trace!("resource_name: {resource_name:?}");
        trace!("delete_contacts: {delete_contacts:?}");

        if resource_name.trim().is_empty() {
            let err = PeopleSendError::InvalidRequest("Group resource name cannot be empty".into());
            return Err(err);
        }

        let mut url = Url::parse(PEOPLE_API_BASE)?.join(resource_name)?;

        if delete_contacts {
            url.query_pairs_mut().append_pair("deleteContacts", "true");
        }

        let send = PeopleSend::delete(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactGroupDelete {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleNoResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact group deleted");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
