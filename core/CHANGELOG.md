# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/SecurityRonin/livedisk-forensic/compare/livedisk-core-v0.2.1...livedisk-core-v0.2.2) - 2026-08-06

### Fixed

- *(core)* keep le_u64 alongside le_u32 after the rebase
- *(core)* bound the GUID read and delegate byte order to uuid
- *(core)* GREEN — deny unwrap/expect and remove both production panics

## [0.2.1](https://github.com/SecurityRonin/livedisk-forensic/compare/livedisk-core-v0.2.0...livedisk-core-v0.2.1) - 2026-07-25

### Fixed

- *(deps)* depend on published safe-read 0.2 (drop path dep, widen ^0.1)

### Other

- rename forensic-bytes dependency to safe-read
- *(livedisk-core)* use forensic-bytes for bounded byte reads
# Changelog
