# 1. Two-crate reader / analyzer split (livedisk-core + livedisk-forensic)

Date: 2026-07-24
Status: Accepted

## Context

The repo does two conceptually separate things: it **enumerates** the live
system's physical disks and partitions into a neutral data model, and it
**judges** a given disk for acquisition-integrity concerns. The enumeration is a
general-purpose capability (any tool that needs a cross-platform disk list wants
it); the forensic judgement is DFIR-specific and pulls in the fleet reporting
vocabulary.

The SecurityRonin fleet constitution (`ronin-issen/CLAUDE.md` → "Crate-structure
standard — reader/analyzer split") makes this split binding for every format
repo: a `<x>-core` reader with no findings, and a `<x>-forensic` analyzer that
emits `forensicnomicon::report::Finding`, in one workspace named `<x>-forensic`.

## Decision

Ship two crates in one workspace (`Cargo.toml` `members = ["core", "forensic"]`):

1. **`livedisk-core`** — the reader. `[lib] name = "livedisk"` so consumers write
   `use livedisk::…` while the package is `livedisk-core` on crates.io (the bare
   `livedisk` name is claimed by an unrelated crate; the `-core`-package /
   `livedisk`-lib pattern is the constitution's collision remedy). Depends only on
   `thiserror`, `safe-read`, and the per-target FFI crates. No findings, no
   `forensicnomicon`.
2. **`livedisk-forensic`** — the analyzer. Depends on `livedisk` +
   `forensicnomicon`; converts a `PhysicalDisk` into graded findings
   (`forensic/src/lib.rs::analyse`). It builds *on* the reader's public model,
   because acquisition-integrity conditions (mounted / writable / removable /
   sector-mismatch / synthesized) are all derivable from the already-exposed
   `PhysicalDisk` fields — the auditor does not need a lower-level view here (the
   constitution's "`-forensic` may go lower than `-core`" escape hatch is
   unnecessary for this repo).

## Consequences

- A non-forensic consumer depends on `livedisk-core` alone and never pulls
  `forensicnomicon`.
- Versions and MSRV are shared through `[workspace.package]`, so a bump is one
  edit; the inter-crate dependency is declared once in `[workspace.dependencies]`
  (`livedisk = { path = "core", version = "0.2.0", package = "livedisk-core" }`).
- The workspace is named for the analyzer (`livedisk-forensic`) per the naming
  grammar, even though the reader is the more broadly useful half.
