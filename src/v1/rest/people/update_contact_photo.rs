//! Update a People contact's photo (`people.updateContactPhoto`).
//!
//! Takes the raw photo bytes (JPEG or PNG) and base64-encodes them into
//! the request.
//!
//! <https://developers.google.com/people/api/rest/v1/people/updateContactPhoto>

use alloc::{format, string::String, vec::Vec};

use base64::{Engine, engine::general_purpose::STANDARD};
use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use url::Url;

use crate::{
    coroutine::*,
    people_try,
    v1::{
        query::to_field_mask,
        rest::people::{PeoplePerson, PeoplePersonField, PeopleReadSourceType},
        send::{PEOPLE_API_BASE, PeopleSend, PeopleSendError, PeopleSendOutput},
    },
};

/// People REST contact photo update response.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactPhotoUpdateResponse {
    #[serde(default)]
    pub person: Option<PeoplePerson>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    photo_bytes: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    person_fields: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    sources: Vec<&'static str>,
}

/// People REST contact photo update, from raw JPEG or PNG bytes.
pub struct PeopleContactPhotoUpdate {
    send: PeopleSend<PeopleContactPhotoUpdateResponse>,
}

impl PeopleContactPhotoUpdate {
    pub fn new(
        auth: &HttpAuthBearer,
        resource_name: &str,
        photo: &[u8],
        person_fields: &[PeoplePersonField],
        sources: &[PeopleReadSourceType],
    ) -> Result<Self, PeopleSendError> {
        debug!("prepare people contact photo for update");
        trace!("resource_name: {resource_name:?}");
        trace!("person_fields: {person_fields:?}");
        trace!("sources: {sources:?}");

        if resource_name.trim().is_empty() {
            let err =
                PeopleSendError::InvalidRequest("Person resource name cannot be empty".into());
            return Err(err);
        }

        if photo.is_empty() {
            let err = PeopleSendError::InvalidRequest("Photo bytes cannot be empty".into());
            return Err(err);
        }

        let url =
            Url::parse(PEOPLE_API_BASE)?.join(&format!("{resource_name}:updateContactPhoto"))?;

        let request = Request {
            photo_bytes: STANDARD.encode(photo),
            person_fields: to_field_mask(person_fields),
            sources: sources
                .iter()
                .filter_map(|source| to_variant_name(source).ok())
                .collect(),
        };

        let send = PeopleSend::patch_json(auth, url, &request)?;

        Ok(Self { send })
    }
}

impl PeopleCoroutine for PeopleContactPhotoUpdate {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<PeopleContactPhotoUpdateResponse>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        let out = people_try!(&mut self.send, arg);
        debug!("people contact photo updated");
        trace!("out: {out:?}");
        PeopleCoroutineState::Complete(Ok(out))
    }
}
