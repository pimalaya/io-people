//! People "Other contacts" (`otherContacts`): list, search,
//! copyOtherContactToMyContactsGroup.
//!
//! "Other contacts" are auto-created from interactions (e.g. emailed
//! addresses); only `emailAddresses`, `metadata`, `names`, `phoneNumbers`
//! and `photos` are readable on them.
//!
//! <https://developers.google.com/people/api/rest/v1/otherContacts>

pub mod copy_other_contact_to_my_contacts_group;
pub mod list;
pub mod search;
