# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added the I/O-free coroutines covering version 1 of the Google People API: the `people` resource (get, batch get, create, update and delete contacts, batch create, update and delete, search contacts, list and search directory people, contact photo upload and delete), its `connections` sub-resource (list, with incremental sync via a sync token), the `contactGroups` resource (list, get, batch get, create, update, delete) and its `members` sub-resource (modify), and the `otherContacts` resource (list, search, copy to the personal contacts group).
- Added the shared `PeopleSend` request primitive over io-http, the `PeopleCoroutine` contract and the `PeopleClientStd` blocking client (`client` feature), with `connect` opening the TCP and TLS connection behind a TLS feature (`rustls-ring` default, `rustls-aws`, `native-tls`).

### Changed

- Aligned the repository with the Pimalaya documentation and naming guidelines: the README dropped all code in favor of redirects, the crate-level rustdoc became the architecture document, and the per-repo ARCHITECTURE.md was retired into the src/lib.rs header and the docs/ folder.
