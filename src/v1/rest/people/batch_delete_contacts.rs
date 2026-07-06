//! Delete a batch of People contacts (`people.batchDeleteContacts`).
//!
//! <https://developers.google.com/people/api/rest/v1/people/batchDeleteContacts>

use alloc::string::String;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::Serialize;
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::send::{PEOPLE_API_BASE, PeopleNoResponse, PeopleSend, PeopleSendError, PeopleSendOutput},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    resource_names: &'a [String],
}

/// People REST contacts batch deletion (500 max), by full resource names.
pub struct PeopleContactsBatchDelete {
    send: PeopleSend<PeopleNoResponse>,
}

impl PeopleContactsBatchDelete {
    pub fn new(auth: &HttpAuthBearer, resource_names: &[String]) -> Result<Self, PeopleSendError> {
        debug!("prepare people contacts batch deletion");
        trace!("resource_names: {resource_names:?}");

        if resource_names.is_empty() {
            let err = PeopleSendError::InvalidRequest("Resource names cannot be empty".into());
            return Err(err);
        }

        let url = Url::parse(PEOPLE_API_BASE)?.join("./people:batchDeleteContacts")?;
        let send = PeopleSend::post_json(auth, url, &Request { resource_names })?;

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactsBatchDelete {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleNoResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contacts batch deleted");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
