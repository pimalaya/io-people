//! People persons (`people`): get, getBatchGet, createContact,
//! updateContact, deleteContact, updateContactPhoto, deleteContactPhoto,
//! batchCreateContacts, batchUpdateContacts, batchDeleteContacts,
//! searchContacts, listDirectoryPeople, searchDirectoryPeople.
//!
//! <https://developers.google.com/people/api/rest/v1/people>

mod types;
#[doc(inline)]
pub use types::*;

pub mod batch_create_contacts;
pub mod batch_delete_contacts;
pub mod batch_update_contacts;
pub mod connections;
pub mod create_contact;
pub mod delete_contact;
pub mod delete_contact_photo;
pub mod get;
pub mod get_batch_get;
pub mod list_directory_people;
pub mod search_contacts;
pub mod search_directory_people;
pub mod update_contact;
pub mod update_contact_photo;
