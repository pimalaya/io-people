//! HTTP/JSON transport every People coroutine delegates to: builds the
//! authorized request and parses the JSON response, or the People error
//! envelope on failure.
//!
//! People API reference: <https://developers.google.com/people/api/rest>.

use core::{fmt, marker::PhantomData};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_http::{
    coroutine::{HttpCoroutine, HttpCoroutineState},
    rfc6750::bearer::HttpAuthBearer,
    rfc9110::{
        request::HttpRequest,
        send::{HttpSendOutput, HttpSendYield},
    },
    rfc9112::send::{Http11Send, Http11SendError},
};
use log::trace;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use thiserror::Error;
use url::Url;

use crate::coroutine::{PeopleCoroutine, PeopleCoroutineState, PeopleYield};

pub const PEOPLE_API_BASE: &str = "https://people.googleapis.com/v1/";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PeopleNoResponse;

impl<'de> Deserialize<'de> for PeopleNoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Self)
    }
}

#[derive(Debug, Error)]
pub enum PeopleSendError {
    #[error("People HTTP request failed: {0}")]
    Send(#[from] Http11SendError),
    #[error("People request serialization failed: {0}")]
    SerializeRequest(#[source] serde_json::Error),
    #[error("People response parsing failed: {0}")]
    ParseResponse(#[source] serde_json::Error),
    #[error("People URL parsing failed: {0}")]
    ParseUrl(#[from] url::ParseError),
    #[error("Invalid People request: {0}")]
    InvalidRequest(String),
    #[error("People API returned HTTP {status}: {message}")]
    Api { status: u16, message: String },
    #[error("People server returned an unexpected redirect")]
    UnexpectedRedirect,
}

impl PeopleSendError {
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.status(), Some(429 | 500 | 502 | 503 | 504))
    }
}

#[derive(Clone, Debug)]
pub struct PeopleSendOutput<T> {
    pub response: T,
    pub keep_alive: bool,
}

pub struct PeopleSend<T> {
    state: State,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> PeopleSend<T> {
    pub fn get(auth: &HttpAuthBearer, url: Url) -> Self {
        Self::with_method(auth, "GET", url, None, Vec::new())
    }

    pub fn delete(auth: &HttpAuthBearer, url: Url) -> Self {
        Self::with_method(auth, "DELETE", url, None, Vec::new())
    }

    pub fn post_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, PeopleSendError> {
        let body = serde_json::to_vec(body).map_err(PeopleSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "POST",
            url,
            Some("application/json"),
            body,
        ))
    }

    pub fn put_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, PeopleSendError> {
        let body = serde_json::to_vec(body).map_err(PeopleSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "PUT",
            url,
            Some("application/json"),
            body,
        ))
    }

    pub fn patch_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, PeopleSendError> {
        let body = serde_json::to_vec(body).map_err(PeopleSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "PATCH",
            url,
            Some("application/json"),
            body,
        ))
    }

    pub fn with_method(
        auth: &HttpAuthBearer,
        method: &str,
        url: Url,
        content_type: Option<&str>,
        body: Vec<u8>,
    ) -> Self {
        let host = url.host_str().unwrap_or("localhost");

        let mut request = HttpRequest::get(url.clone())
            .header("Host", host)
            .header("Accept", "application/json")
            .header("Authorization", auth.to_authorization())
            .body(body);

        if let Some(content_type) = content_type {
            request = request.header("Content-Type", content_type);
        }

        request.method = method.into();

        trace!("send People {method} request to {url}");

        Self {
            state: State::Send(Http11Send::new(request)),
            _phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> PeopleCoroutine for PeopleSend<T> {
    type Yield = PeopleYield;
    type Return = Result<PeopleSendOutput<T>, PeopleSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
        trace!("send: {}", self.state);
        match &mut self.state {
            State::Send(send) => match send.resume(arg) {
                HttpCoroutineState::Yielded(HttpSendYield::WantsRead) => {
                    PeopleCoroutineState::Yielded(PeopleYield::WantsRead)
                }
                HttpCoroutineState::Yielded(HttpSendYield::WantsWrite(bytes)) => {
                    PeopleCoroutineState::Yielded(PeopleYield::WantsWrite(bytes))
                }
                HttpCoroutineState::Yielded(HttpSendYield::WantsRedirect { .. }) => {
                    PeopleCoroutineState::Complete(Err(PeopleSendError::UnexpectedRedirect))
                }
                HttpCoroutineState::Complete(Err(err)) => {
                    PeopleCoroutineState::Complete(Err(err.into()))
                }
                HttpCoroutineState::Complete(Ok(HttpSendOutput {
                    response,
                    keep_alive,
                    ..
                })) => {
                    if response.status.is_success() {
                        let body = if response.body.is_empty() {
                            b"null".as_slice()
                        } else {
                            response.body.as_slice()
                        };

                        match serde_json::from_slice::<T>(body) {
                            Ok(response) => PeopleCoroutineState::Complete(Ok(PeopleSendOutput {
                                response,
                                keep_alive,
                            })),
                            Err(err) => PeopleCoroutineState::Complete(Err(
                                PeopleSendError::ParseResponse(err),
                            )),
                        }
                    } else {
                        let (status, message) = parse_api_error(*response.status, &response.body);
                        PeopleCoroutineState::Complete(Err(PeopleSendError::Api {
                            status,
                            message,
                        }))
                    }
                }
            },
        }
    }
}

enum State {
    Send(Http11Send),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Send(_) => f.write_str("send"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: Option<u16>,
    message: Option<String>,
}

pub fn parse_api_error(http_status: u16, body: &[u8]) -> (u16, String) {
    if let Ok(envelope) = serde_json::from_slice::<ErrorEnvelope>(body) {
        let status = envelope.error.code.unwrap_or(http_status);
        let message = envelope
            .error
            .message
            .filter(|message| !message.trim().is_empty())
            .unwrap_or_else(|| String::from("unknown People API error"));
        return (status, message);
    }

    let message = String::from_utf8_lossy(body).trim().to_string();

    if message.is_empty() {
        (http_status, String::from("unknown People API error"))
    } else {
        (http_status, message)
    }
}
