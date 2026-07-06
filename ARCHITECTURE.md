# Architecture guide

Read the [Pimalaya ARCHITECTURE](https://github.com/pimalaya/.github/blob/master/ARCHITECTURE.md) first: it describes the conventions every Pimalaya repository shares (the sans-I/O coroutine approach, `no_std`, module and error rules, code style, licensing). This document only covers what is specific to io-google-people, and assumes you know that shared context.

If a statement here conflicts with the code, the code wins; please flag it.

## Where io-google-people fits

io-google-people is a **protocol library**: a set of I/O-free coroutines for the [People API](https://developers.google.com/people/api/rest). It sits one layer above [io-http](https://github.com/pimalaya/io-http) (HTTP/1.1) and [pimalaya-stream](https://github.com/pimalaya/stream) (TCP + TLS), and is consumed by contacts-domain tools such as [cardamum](https://github.com/pimalaya/cardamum). It is the contacts equivalent of [io-gmail](https://github.com/pimalaya/io-gmail) / [io-msgraph](https://github.com/pimalaya/io-msgraph): same shape, different Google API.

The crate has two of the three standard layers; there is no CLI:

1. **I/O-free coroutines** (`no_std` core, always present): the whole People REST logic.
2. **Std client** (`client` feature): a blocking driver, `PeopleClientStd`, with `connect` gated behind a TLS feature (`rustls-ring` default, `rustls-aws`, `native-tls`).

## API versioning: everything lives under `v1`

The People API is versioned (`/v1/`), so the crate is too. The version-agnostic primitives stay at the crate root; everything that is v1-specific lives under `src/v1/`. The day People ships a v2, a sibling `src/v2/` is added without breaking `v1` consumers.

- `src/lib.rs`, `src/coroutine.rs`: crate root, shared across versions.
- `src/v1/`: the v1 surface (`send.rs`, `query.rs`, `client.rs`, and the whole `rest/` tree).

Callers always import through the version, e.g. `io_google_people::v1::rest::people::PeoplePerson`, `io_google_people::v1::client::PeopleClientStd`.

## The send primitive

Unlike IMAP or JMAP, every People call is an independent HTTP request/response, so io-google-people has a single shared primitive that all coroutines delegate to: `v1::send::PeopleSend<T>` (`src/v1/send.rs`). It wraps io-http's `Http11Send`, builds the request (the `Authorization` header from the caller's bearer token via `HttpAuthBearer::to_authorization`, `Accept: application/json`, an optional JSON body), and on completion either deserialises the 2xx body into `T` or parses People's JSON error envelope into `PeopleSendError::Api { status, message }`. A 3xx surfaces as `PeopleSendError::UnexpectedRedirect` (redirects are not followed). `PeopleSend<T>` exposes `get` / `post_json` / `put_json` / `patch_json` / `delete` / `with_method` constructors.

Its terminal value is `PeopleSendOutput<T> { response: T, keep_alive: bool }`. `keep_alive` lets a driver reuse the TCP/TLS connection across the many small requests a People session makes. Empty 2xx bodies (DELETE, batch ops, stop) deserialise into the `PeopleNoResponse` unit marker.

## The coroutine contract

io-google-people follows the standard Pimalaya coroutine shape with crate-local names (`src/coroutine.rs`, version-agnostic):

- trait `PeopleCoroutine` with `resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return>`;
- `PeopleCoroutineState` is `Yielded(Y)` or `Complete(R)`;
- the standard yield is `PeopleYield { WantsRead, WantsWrite(Vec<u8>) }` (a People REST call is I/O-only: no clock, randomness or filesystem);
- the `people_try!` macro is the coroutine `?`: it forwards `Yielded` and short-circuits `Complete(Err(_))`.

Every REST coroutine is a thin, single-step wrapper. Because it has exactly one I/O step, it does **not** carry a `State` enum: the struct holds the send directly as `struct PeopleX { send: PeopleSend<T> }`. `new(auth, user_id, ...)` builds the URL and body and stores the `PeopleSend<T>`; `resume` is just:

```rust
fn resume(&mut self, arg: Option<&[u8]>) -> PeopleCoroutineState<Self::Yield, Self::Return> {
    let out = people_try!(&mut self.send, arg);
    debug!("people <thing> <verbed>");
    trace!("out: {out:?}");
    PeopleCoroutineState::Complete(Ok(out))
}
```

The canonical reference template is the `contact_groups` module (`src/v1/rest/contact_groups/{get,create,delete}.rs`). A multi-variant `State` enum + `fmt::Display` is reserved for genuine multi-step coroutines; this crate currently has none (every People call is one request/response).

### Logging

Each coroutine logs at two levels, via `use log::{debug, trace};`:

- `new()` opens with `debug!("prepare people <thing> <op>")` (a static, human-readable lifecycle line), then **one `trace!` per input variable**, each in the form `trace!("var_name: {var_name:?}")`.
- `resume()`, right after the send resolves, does `debug!("people <thing> <verbed>")` then `trace!("out: {out:?}")`.

So `debug!` carries the readable lifecycle, `trace!` dumps the raw values; never combine several variables in one `trace!`.

## Authentication

io-google-people does no OAuth itself. The People API only accepts OAuth 2.0 Bearer tokens (no Basic, no app passwords, no arbitrary header), so the credential is exactly a bare access token: the std client takes it as `impl ToString` and stores an `io_http::rfc6750::bearer::HttpAuthBearer`; coroutines take `auth: &HttpAuthBearer`; and `send` adds the `Bearer ` prefix via `auth.to_authorization()` when building each request. Tokens are short-lived; minting and refreshing them is the caller's responsibility (cardamum, for example, reads the token from a config command). The base URL is fixed (`PEOPLE_API_BASE`, `https://people.googleapis.com/v1/`); there is no per-user path segment, the owner is always the authenticated user and entities are addressed by full resource names (`people/me`, `people/{id}`, `contactGroups/{id}`, `otherContacts/{id}`).

## Module layout: `v1/rest` mirrors the REST tree

`src/v1/rest/` mirrors the People REST reference one-to-one: each resource is a directory and each method is a file named after the API method in snake_case (`getBatchGet` -> `get_batch_get.rs`, `copyOtherContactToMyContactsGroup` -> `copy_other_contact_to_my_contacts_group.rs`). A reader who knows the reference knows where to look.

```
src/
  lib.rs            crate root: no_std, `pub mod coroutine; pub mod v1;`
  coroutine.rs      PeopleCoroutine / PeopleCoroutineState / PeopleYield + people_try!
  v1/
    mod.rs
    send.rs         PeopleSend<T>, PeopleSendError/Output, PeopleNoResponse, base URL
    client.rs       (client) PeopleClientStd: boxed stream + auth (HttpAuthBearer)
    query.rs        Serialize-struct -> URL query pairs + to_field_mask (no_std serde)
    rest/
      people/          get, get_batch_get, create_contact, update_contact,
                       delete_contact, update_contact_photo, delete_contact_photo,
                       batch_create_contacts, batch_update_contacts,
                       batch_delete_contacts, search_contacts,
                       list_directory_people, search_directory_people (+ types)
        connections/   people.connections.list
      contact_groups/  list, get, batch_get, create, update, delete (+ types)
        members/       contactGroups.members.modify
      other_contacts/  list, search, copy_other_contact_to_my_contacts_group
```

Every `rest/` file carries a short `//!` header naming its operation and the REST method in backticks, followed by a link to the matching People REST reference page (method pages for the verb files, the resource page for `mod.rs`/`types.rs`), e.g. `//! Get a People person (\`people.get\`).` linking `.../rest/v1/people/get`.

Each directory follows the standard module rules: a private `types` submodule re-exported via `#[doc(inline)] pub use types::*;` in `mod.rs`, then one file per method. `mod.rs` holds only module declarations.

## Types: a faithful, complete mapping of the resource

Domain types are `People`-prefixed (`PeoplePerson`, `PeopleName`, `PeopleEmailAddress`, `PeopleContactGroup`, ...), are never re-exported at the crate root (callers use module-qualified paths), and aim to mirror the REST schema fully. The only deliberate gap: `Person` fields the reference marks as deprecated with "no data will be returned" (`ageRange`, `braggingRights`, `relationshipInterests`, `relationshipStatuses`, `residences`, `taglines`) are omitted, as is the deprecated `requestMask.includeField` query parameter (superseded by `personFields`).

- **Full-resource request bodies.** A method whose body is a resource instance takes the whole resource by reference, not a hand-rolled subset: `people::create_contact` / `update_contact` and the batch variants take `&PeoplePerson`, `contact_groups::create` / `update` take `&PeopleContactGroup`. Resources are `Default` with `skip_serializing_if` on optional/empty fields, so a partially-filled value serialises cleanly (updates must carry the `resource_name` and latest `etag`). Methods whose body is a dedicated request object (photo update, members modify, batch requests, other-contact copy) keep that small request shape as a private `Request` struct in the method file.
- **Enums for enum-valued fields.** Wire strings that the reference documents as enums are typed `People`-prefixed enums (`PeopleSourceType`, `PeopleContactGroupType`, `PeopleNicknameType`, `PeopleMiscKeywordType`, ...), each defined next to the type that carries them. Body enums derive serde with `#[serde(rename_all = ...)]`; free-form `type` fields (email, phone, address, ...) stay `Option<String>` because the API accepts custom labels there.
- **Field masks.** `personFields`/`readMask`/`updatePersonFields` take `&[PeoplePersonField]` and `groupFields` takes `&[PeopleGroupField]`; `v1::query::to_field_mask` joins the serde-renamed variant names into the comma-separated string the API expects. Masks the API requires are validated non-empty at `new()` time (`PeopleSendError::InvalidRequest`).
- **Query parameters.** Every list method takes a borrowed `*Params<'a>` struct (`PeopleConnectionsListParams`, `PeopleContactGroupsListParams`, `PeopleOtherContactsListParams`, ...) rather than a long positional argument list. Each `*Params` derives `Serialize` + `#[serde(rename_all = "camelCase")]`, and `v1::query::to_query_pairs` (a tiny custom `no_std` serde serializer in `src/v1/query.rs`) flattens it into the `(key, value)` pairs fed to `url`'s `query_pairs_mut().extend_pairs(...)`. The field name *is* the query key; `None` and empty sequences serialise to nothing, a sequence becomes one repeated-key pair per element (the `sources`/`resourceNames` shape People needs, which `serde_urlencoded` cannot emit and which is also not `no_std`), and bool flags use `#[serde(skip_serializing_if = "query::is_false")]`. Required masks and repeated `sources` on non-list methods are appended manually with `serde_variant::to_variant_name(&value)` at the call site.
- One `url` crate subtlety: the collection-level custom verbs (`people:searchContacts`, `contactGroups:batchGet`, ...) must be joined as `./people:searchContacts`, otherwise the colon in the first path segment is parsed as a URL scheme.

## Incremental sync

The People API has no push channel and no history log; incremental sync is a token loop on the list methods. `people.connections.list` and `otherContacts.list` take `request_sync_token` on the first full listing and return a `next_sync_token` on the last page; passing it back as `sync_token` returns only the resources changed since (deleted ones come back with just `metadata.deleted = true`). An expired token surfaces as an HTTP 410 `PeopleSendError::Api`, which callers handle by re-baselining with a full listing. Polling cadence is the caller's business, so the crate ships no composite watcher coroutine.

## The std client

`PeopleClientStd` (`client` feature, `src/v1/client.rs`) wraps a boxed `Read + Write + Send` stream plus the `HttpAuthBearer`. Its generic `run<C, T>(coroutine)` is the blocking driver loop (read on `WantsRead`, write on `WantsWrite`), returning `PeopleSendOutput<T>`. It offers one convenience method per first-class verb (`connections_list`, `person_get`, `contact_create`, `contact_update`, `contact_delete`, `contacts_search`, `contact_groups_list`, `contact_group_*`, `contact_group_members_modify`, `other_contacts_list`, `other_contacts_search`, `other_contact_copy`); the photo, batch and directory coroutines are driven by passing them to `run`. `connect(token, PeopleClientStdConnectOptions)` (TLS features) opens `people.googleapis.com:443` through pimalaya-stream and is the entry point the integration test and downstream clients use. The bare bearer token is the only required argument; the People base URL is fixed (so not a parameter) and the rest live in `PeopleClientStdConnectOptions { tls }`, which is `Default` (default TLS backend).

## Testing

`tests/coroutines.rs` runs coroutines against in-memory HTTP responses (no network) using a `run` helper, covering request-line and body shapes, response parsing, local `InvalidRequest` validation, error surfacing, field-mask joining and `*Params` query serialisation. `tests/people.rs` is an `#[ignore]`d end-to-end test against the live API, gated behind a TLS feature and driven by `PEOPLE_ACCESS_TOKEN`; it walks the whole CRUD surface (group create/get/update, contact create/get/update, membership add/verify/remove, the batch create/get/update/delete cycle, search warmup, other-contacts listing) and deletes everything it created.
