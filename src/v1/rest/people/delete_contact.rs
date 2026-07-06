//! Delete a People contact (`people.deleteContact`).
//!
//! <https://developers.google.com/people/api/rest/v1/people/deleteContact>

use alloc::format;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::send::{PEOPLE_API_BASE, PeopleNoResponse, PeopleSend, PeopleSendError, PeopleSendOutput},
};

/// People REST contact deletion, by full resource name.
pub struct PeopleContactDelete {
    send: PeopleSend<PeopleNoResponse>,
}

impl PeopleContactDelete {
    pub fn new(auth: &HttpAuthBearer, resource_name: &str) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact for deletion");
        trace!("resource_name: {resource_name:?}");

        if resource_name.trim().is_empty() {
            let err =
                PeopleSendError::InvalidRequest("Person resource name cannot be empty".into());
            return Err(err);
        }

        let url = Url::parse(PEOPLE_API_BASE)?.join(&format!("{resource_name}:deleteContact"))?;
        let send = PeopleSend::delete(auth, url);

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactDelete {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleNoResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact deleted");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
