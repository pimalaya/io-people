//! People person resource types.
//!
//! Fields the reference marks as deprecated with "no data will be
//! returned" (ageRange, braggingRights, relationshipInterests,
//! relationshipStatuses, residences, taglines) are omitted.
//!
//! <https://developers.google.com/people/api/rest/v1/people#Person>

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

/// Information about a person merged from various data sources such as
/// the authenticated user's contacts and profile data.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeoplePerson {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resource_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub etag: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeoplePersonMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<PeopleAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<PeopleAgeRangeType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub biographies: Vec<PeopleBiography>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub birthdays: Vec<PeopleBirthday>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calendar_urls: Vec<PeopleCalendarUrl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub client_data: Vec<PeopleClientData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cover_photos: Vec<PeopleCoverPhoto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<PeopleEmailAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<PeopleEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ids: Vec<PeopleExternalId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_ases: Vec<PeopleFileAs>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub genders: Vec<PeopleGender>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub im_clients: Vec<PeopleImClient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interests: Vec<PeopleInterest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locales: Vec<PeopleLocale>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<PeopleLocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memberships: Vec<PeopleMembership>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub misc_keywords: Vec<PeopleMiscKeyword>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<PeopleName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nicknames: Vec<PeopleNickname>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub occupations: Vec<PeopleOccupation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<PeopleOrganization>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phone_numbers: Vec<PeoplePhoneNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photos: Vec<PeoplePhoto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<PeopleRelation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sip_addresses: Vec<PeopleSipAddress>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<PeopleSkill>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<PeopleUrl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_defined: Vec<PeopleUserDefined>,
}

/// Metadata about a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeoplePersonMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<PeopleSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub previous_resource_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_people_resource_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<PeopleObjectType>,
}

/// Source of a person field.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleSource {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PeopleSourceType>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub etag: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_metadata: Option<PeopleProfileMetadata>,
}

/// Type of a person field source.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleSourceType {
    SourceTypeUnspecified,
    Account,
    Profile,
    DomainProfile,
    Contact,
    OtherContact,
    DomainContact,
}

/// Metadata about a profile.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleProfileMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<PeopleObjectType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_types: Vec<String>,
}

/// Type of a person object: person or (Currents) page.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleObjectType {
    ObjectTypeUnspecified,
    Person,
    Page,
}

/// Metadata about a person field: its source and primary/verified flags.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleFieldMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PeopleSource>,
}

/// Whole or partial calendar date, such as a birthday.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
}

/// Person's physical address.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub po_box: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// Person's age range.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleAgeRangeType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_range: Option<PeopleAgeRange>,
}

/// Age range bucket of a person.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleAgeRange {
    AgeRangeUnspecified,
    LessThanEighteen,
    EighteenToTwenty,
    TwentyOneOrOlder,
}

/// Person's short biography.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleBiography {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<PeopleContentType>,
}

/// Content type of a biography.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleContentType {
    ContentTypeUnspecified,
    TextPlain,
    TextHtml,
}

/// Person's birthday, as a structured date and/or free-form text.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleBirthday {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<PeopleDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Person's calendar URL.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleCalendarUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub calendar_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Arbitrary client data attached to a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleClientData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's cover photo.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleCoverPhoto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// Person's email address.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleEmailAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Event related to the person, such as an anniversary.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<PeopleDate>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Identifier from an external entity related to the person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleExternalId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Name that should be used to sort the person in a list.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleFileAs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's gender.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleGender {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_me_as: Option<String>,
}

/// Person's instant messaging client.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleImClient {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub im_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_protocol: Option<String>,
}

/// One of the person's interests.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleInterest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's locale preference, as an IETF BCP 47 language tag.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleLocale {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's location.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_section: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desk_code: Option<String>,
}

/// Person's membership in a group.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleMembership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_group_membership: Option<PeopleContactGroupMembership>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_membership: Option<PeopleDomainMembership>,
}

/// Contact group membership of a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleContactGroupMembership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_group_resource_name: Option<String>,
}

/// Google Workspace domain membership of a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDomainMembership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_viewer_domain: Option<bool>,
}

/// Person's miscellaneous keyword.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleMiscKeyword {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub keyword_type: Option<PeopleMiscKeywordType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Type of a miscellaneous keyword.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleMiscKeywordType {
    TypeUnspecified,
    OutlookBillingInformation,
    OutlookDirectoryServer,
    OutlookKeyword,
    OutlookMileage,
    OutlookPriority,
    OutlookSensitivity,
    OutlookSubject,
    OutlookUser,
    Home,
    Work,
    Other,
}

/// Person's name.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unstructured_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_last_first: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_family_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_middle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_honorific_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_honorific_suffix: Option<String>,
}

/// Person's nickname.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleNickname {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub nickname_type: Option<PeopleNicknameType>,
}

/// Type of a nickname.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleNicknameType {
    Default,
    MaidenName,
    Initials,
    Gplus,
    OtherName,
    AlternateName,
    ShortName,
}

/// Person's occupation.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleOccupation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's past or current organization.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleOrganization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub org_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<PeopleDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<PeopleDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_time_equivalent_millipercent: Option<i32>,
}

/// Person's phone number.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeoplePhoneNumber {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_form: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Person's photo.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeoplePhoto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// Person's relation to another person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleRelation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Person's SIP address, for VoIP calls.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleSipAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub sip_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Skill that the person has.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleSkill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Person's associated URL.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub url_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted_type: Option<String>,
}

/// Arbitrary user data attached to a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleUserDefined {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeopleFieldMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Response for a single requested person in a batch method.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeoplePersonResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PeoplePerson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_resource_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PeopleStatus>,
}

/// Logical error model of a failed item in a batch method.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<serde_json::Value>,
}

/// Response to a search request, shared by `people.searchContacts` and
/// `otherContacts.search`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleSearchResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<PeopleSearchResult>,
}

/// Single result of a search query.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PeopleSearchResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PeoplePerson>,
}

/// Person field selectable in a `personFields`/`readMask` field mask.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PeoplePersonField {
    Addresses,
    AgeRanges,
    Biographies,
    Birthdays,
    CalendarUrls,
    ClientData,
    CoverPhotos,
    EmailAddresses,
    Events,
    ExternalIds,
    Genders,
    ImClients,
    Interests,
    Locales,
    Locations,
    Memberships,
    Metadata,
    MiscKeywords,
    Names,
    Nicknames,
    Occupations,
    Organizations,
    PhoneNumbers,
    Photos,
    Relations,
    SipAddresses,
    Skills,
    Urls,
    UserDefined,
}

/// Source type selectable in a `sources` query parameter.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleReadSourceType {
    ReadSourceTypeProfile,
    ReadSourceTypeContact,
    ReadSourceTypeDomainContact,
    ReadSourceTypeOtherContact,
}

/// Order in which connections are sorted (`people.connections.list`).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleSortOrder {
    LastModifiedAscending,
    LastModifiedDescending,
    FirstNameAscending,
    LastNameAscending,
}

/// Directory source selectable in the directory methods.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleDirectorySourceType {
    DirectorySourceTypeDomainContact,
    DirectorySourceTypeDomainProfile,
}

/// Additional data merged into the directory sources.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeopleDirectoryMergeSourceType {
    DirectoryMergeSourceTypeContact,
}
