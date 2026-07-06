//! Std-blocking People client: wraps a `Read + Write` stream plus the
//! bearer credential and runs the coroutines against
//! `people.googleapis.com`. Gated behind the `client` feature.

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use core::time::Duration;
use core::{any::Any, fmt};

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use alloc::string::ToString;
use alloc::{boxed::Box, string::String};
use io_http::rfc6750::bearer::HttpAuthBearer;

use std::io::{self, Read, Write};

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use pimalaya_stream::std::stream::StreamStd;
#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
pub use pimalaya_stream::tls::*;
use thiserror::Error;
#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use url::Url;

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use crate::v1::send::PEOPLE_API_BASE;
use crate::{
    coroutine::*,
    v1::rest::contact_groups::{
        PeopleContactGroup, PeopleGroupField, create::PeopleContactGroupCreate,
        delete::PeopleContactGroupDelete, get::PeopleContactGroupGet,
        list::PeopleContactGroupsList, list::PeopleContactGroupsListParams,
        list::PeopleContactGroupsListResponse, members::modify::PeopleContactGroupMembersModify,
        members::modify::PeopleContactGroupMembersModifyResponse, update::PeopleContactGroupUpdate,
    },
    v1::rest::other_contacts::{
        copy_other_contact_to_my_contacts_group::PeopleOtherContactCopy,
        list::PeopleOtherContactsList, list::PeopleOtherContactsListParams,
        list::PeopleOtherContactsListResponse, search::PeopleOtherContactsSearch,
    },
    v1::rest::people::{
        PeoplePerson, PeoplePersonField, PeopleReadSourceType, PeopleSearchResponse,
        connections::list::PeopleConnectionsList, connections::list::PeopleConnectionsListParams,
        connections::list::PeopleConnectionsListResponse, create_contact::PeopleContactCreate,
        delete_contact::PeopleContactDelete, get::PeoplePersonGet,
        search_contacts::PeopleContactsSearch, update_contact::PeopleContactUpdate,
    },
    v1::send::{PeopleNoResponse, PeopleSendError, PeopleSendOutput},
};

#[derive(Debug, Error)]
pub enum PeopleClientStdError {
    #[error(transparent)]
    Send(#[from] PeopleSendError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error(transparent)]
    Tls(#[from] anyhow::Error),
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error("People URL `{0}` has no host")]
    UrlMissingHost(String),
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error("People URL `{0}` has unsupported scheme `{1}` (expected `http` or `https`)")]
    UrlUnsupportedScheme(String, String),
}

/// Optional settings for [`PeopleClientStd::connect`]; every field has a
/// default (the TLS backend default).
#[derive(Default)]
pub struct PeopleClientStdConnectOptions {
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    pub tls: Tls,
}

const READ_BUFFER_SIZE: usize = 16 * 1024;

pub struct PeopleClientStd {
    pub stream: Box<dyn PeopleStream>,
    pub auth: HttpAuthBearer,
}

impl PeopleClientStd {
    pub fn new<S: Read + Write + Send + 'static>(stream: S, token: impl ToString) -> Self {
        Self {
            stream: Box::new(stream),
            auth: HttpAuthBearer::new(token.to_string()),
        }
    }

    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    pub fn connect(
        token: impl ToString,
        options: PeopleClientStdConnectOptions,
    ) -> Result<Self, PeopleClientStdError> {
        let PeopleClientStdConnectOptions { tls } = options;

        let url = Url::parse(PEOPLE_API_BASE).expect("People API base URL is valid");
        let host = url
            .host_str()
            .ok_or_else(|| PeopleClientStdError::UrlMissingHost(url.to_string()))?;

        let stream = match url.scheme() {
            "http" => StreamStd::connect_tcp(host, url.port().unwrap_or(80))?,
            "https" => StreamStd::connect_tls(host, url.port().unwrap_or(443), &tls)?,
            scheme => {
                return Err(PeopleClientStdError::UrlUnsupportedScheme(
                    url.to_string(),
                    scheme.to_string(),
                ));
            }
        };

        stream.set_read_timeout(Some(Duration::from_secs(30)))?;

        Ok(Self {
            stream: Box::new(stream),
            auth: HttpAuthBearer::new(token.to_string()),
        })
    }

    pub fn set_stream<S: Read + Write + Send + 'static>(&mut self, stream: S) {
        self.stream = Box::new(stream);
    }

    pub fn run<C, T>(
        &mut self,
        mut coroutine: C,
    ) -> Result<PeopleSendOutput<T>, PeopleClientStdError>
    where
        C: PeopleCoroutine<
                Yield = PeopleYield,
                Return = Result<PeopleSendOutput<T>, PeopleSendError>,
            >,
    {
        let mut buf = [0u8; READ_BUFFER_SIZE];
        let mut arg: Option<&[u8]> = None;

        loop {
            match coroutine.resume(arg.take()) {
                PeopleCoroutineState::Complete(Ok(out)) => return Ok(out),
                PeopleCoroutineState::Complete(Err(err)) => return Err(err.into()),
                PeopleCoroutineState::Yielded(PeopleYield::WantsRead) => {
                    let n = self.stream.read(&mut buf)?;
                    arg = Some(&buf[..n]);
                }
                PeopleCoroutineState::Yielded(PeopleYield::WantsWrite(bytes)) => {
                    self.stream.write_all(&bytes)?;
                    arg = None;
                }
            }
        }
    }

    pub fn connections_list(
        &mut self,
        person_fields: &[PeoplePersonField],
        params: &PeopleConnectionsListParams,
    ) -> Result<PeopleSendOutput<PeopleConnectionsListResponse>, PeopleClientStdError> {
        let coroutine = PeopleConnectionsList::new(&self.auth, person_fields, params)?;
        self.run(coroutine)
    }

    pub fn person_get(
        &mut self,
        resource_name: &str,
        person_fields: &[PeoplePersonField],
        sources: &[PeopleReadSourceType],
    ) -> Result<PeopleSendOutput<PeoplePerson>, PeopleClientStdError> {
        let coroutine = PeoplePersonGet::new(&self.auth, resource_name, person_fields, sources)?;
        self.run(coroutine)
    }

    pub fn contact_create(
        &mut self,
        person: &PeoplePerson,
        person_fields: &[PeoplePersonField],
        sources: &[PeopleReadSourceType],
    ) -> Result<PeopleSendOutput<PeoplePerson>, PeopleClientStdError> {
        let coroutine = PeopleContactCreate::new(&self.auth, person, person_fields, sources)?;
        self.run(coroutine)
    }

    pub fn contact_update(
        &mut self,
        person: &PeoplePerson,
        update_person_fields: &[PeoplePersonField],
        person_fields: &[PeoplePersonField],
        sources: &[PeopleReadSourceType],
    ) -> Result<PeopleSendOutput<PeoplePerson>, PeopleClientStdError> {
        let coroutine = PeopleContactUpdate::new(
            &self.auth,
            person,
            update_person_fields,
            person_fields,
            sources,
        )?;
        self.run(coroutine)
    }

    pub fn contact_delete(
        &mut self,
        resource_name: &str,
    ) -> Result<PeopleSendOutput<PeopleNoResponse>, PeopleClientStdError> {
        let coroutine = PeopleContactDelete::new(&self.auth, resource_name)?;
        self.run(coroutine)
    }

    pub fn contacts_search(
        &mut self,
        query: &str,
        read_mask: &[PeoplePersonField],
        page_size: Option<u32>,
        sources: &[PeopleReadSourceType],
    ) -> Result<PeopleSendOutput<PeopleSearchResponse>, PeopleClientStdError> {
        let coroutine =
            PeopleContactsSearch::new(&self.auth, query, read_mask, page_size, sources)?;
        self.run(coroutine)
    }

    pub fn contact_groups_list(
        &mut self,
        group_fields: &[PeopleGroupField],
        params: &PeopleContactGroupsListParams,
    ) -> Result<PeopleSendOutput<PeopleContactGroupsListResponse>, PeopleClientStdError> {
        let coroutine = PeopleContactGroupsList::new(&self.auth, group_fields, params)?;
        self.run(coroutine)
    }

    pub fn contact_group_get(
        &mut self,
        resource_name: &str,
        max_members: Option<u32>,
        group_fields: &[PeopleGroupField],
    ) -> Result<PeopleSendOutput<PeopleContactGroup>, PeopleClientStdError> {
        let coroutine =
            PeopleContactGroupGet::new(&self.auth, resource_name, max_members, group_fields)?;
        self.run(coroutine)
    }

    pub fn contact_group_create(
        &mut self,
        group: &PeopleContactGroup,
        read_group_fields: &[PeopleGroupField],
    ) -> Result<PeopleSendOutput<PeopleContactGroup>, PeopleClientStdError> {
        let coroutine = PeopleContactGroupCreate::new(&self.auth, group, read_group_fields)?;
        self.run(coroutine)
    }

    pub fn contact_group_update(
        &mut self,
        group: &PeopleContactGroup,
        update_group_fields: &[PeopleGroupField],
        read_group_fields: &[PeopleGroupField],
    ) -> Result<PeopleSendOutput<PeopleContactGroup>, PeopleClientStdError> {
        let coroutine = PeopleContactGroupUpdate::new(
            &self.auth,
            group,
            update_group_fields,
            read_group_fields,
        )?;
        self.run(coroutine)
    }

    pub fn contact_group_delete(
        &mut self,
        resource_name: &str,
        delete_contacts: bool,
    ) -> Result<PeopleSendOutput<PeopleNoResponse>, PeopleClientStdError> {
        let coroutine = PeopleContactGroupDelete::new(&self.auth, resource_name, delete_contacts)?;
        self.run(coroutine)
    }

    pub fn contact_group_members_modify(
        &mut self,
        resource_name: &str,
        resource_names_to_add: &[String],
        resource_names_to_remove: &[String],
    ) -> Result<PeopleSendOutput<PeopleContactGroupMembersModifyResponse>, PeopleClientStdError>
    {
        let coroutine = PeopleContactGroupMembersModify::new(
            &self.auth,
            resource_name,
            resource_names_to_add,
            resource_names_to_remove,
        )?;
        self.run(coroutine)
    }

    pub fn other_contacts_list(
        &mut self,
        read_mask: &[PeoplePersonField],
        params: &PeopleOtherContactsListParams,
    ) -> Result<PeopleSendOutput<PeopleOtherContactsListResponse>, PeopleClientStdError> {
        let coroutine = PeopleOtherContactsList::new(&self.auth, read_mask, params)?;
        self.run(coroutine)
    }

    pub fn other_contacts_search(
        &mut self,
        query: &str,
        read_mask: &[PeoplePersonField],
        page_size: Option<u32>,
    ) -> Result<PeopleSendOutput<PeopleSearchResponse>, PeopleClientStdError> {
        let coroutine = PeopleOtherContactsSearch::new(&self.auth, query, read_mask, page_size)?;
        self.run(coroutine)
    }

    pub fn other_contact_copy(
        &mut self,
        resource_name: &str,
        copy_mask: &[PeoplePersonField],
        read_mask: &[PeoplePersonField],
        sources: &[PeopleReadSourceType],
    ) -> Result<PeopleSendOutput<PeoplePerson>, PeopleClientStdError> {
        let coroutine =
            PeopleOtherContactCopy::new(&self.auth, resource_name, copy_mask, read_mask, sources)?;
        self.run(coroutine)
    }
}

impl fmt::Debug for PeopleClientStd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeopleClientStd")
            .field("auth", &self.auth)
            .finish_non_exhaustive()
    }
}

pub trait PeopleStream: Read + Write + Send + Any {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Read + Write + Send + Any> PeopleStream for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
