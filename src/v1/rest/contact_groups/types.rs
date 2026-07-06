//! People contact group resource types.
//!
//! <https://developers.google.com/people/api/rest/v1/contactGroups#ContactGroup>

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::people::PeopleStatus;

/// Contact group owned by the authenticated user.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroup {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub etag: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleContactGroupMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<PeopleContactGroupType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_resource_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub client_data: Vec<PeopleGroupClientData>,
}

/// Metadata about a contact group.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// Owner of the contact group: created by Google or by the user.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleContactGroupType {
    GroupTypeUnspecified,
    UserContactGroup,
    SystemContactGroup,
}

/// Arbitrary client data attached to a contact group.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleGroupClientData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Response for a single requested contact group in a batch method.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_group: Option<PeopleContactGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_resource_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PeopleStatus>,
}

/// Contact group field selectable in a `groupFields` field mask.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PeopleGroupField {
    ClientData,
    GroupType,
    MemberCount,
    Metadata,
    Name,
}
