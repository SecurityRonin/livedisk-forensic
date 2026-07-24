# livedisk — Design: Purpose & Scope

*A library, not a product. This document states what `livedisk-core` /
`livedisk-forensic` are for, where their boundaries sit, and how correctness is
established. Every current-state claim is grounded in a same-session read of the
repo (2026-07-24); the load-bearing decisions live as ADRs under
[`decisions/`](decisions/). This is a DESIGN note, not a PRD — the crates ship no
binary an examiner runs; they are linked by other tools (Issen, disk4n6, any
cross-platform disk enumerator).*

## Purpose

Answer one question uniformly on macOS, Linux, and Windows: **what physical disks
and partitions does the running machine have, and does any of them bear on a
forensically sound acquisition?** `diskutil list`, `lsblk`, and `diskpart` each
answer the first half in an incompatible dialect and require their binaries on
the host. `livedisk` answers it as one set of Rust structs, natively, with no
subprocess and no daemon.

## What it does

- **`livedisk-core`** (imported as `livedisk`) — enumerates the live system's
  disks/partitions into a unified `PhysicalDisk` / `Partition` model
  ([ADR 0002](decisions/0002-unified-model-per-os-backends.md)), fills it from a
  native per-OS backend (IOKit `IOMedia`, `/sys/block` sysfs, `DeviceIoControl`),
  renders proportional partition-layout bars (`render_overview`,
  `render_disk_bar`, `render_listing`), offers an optional `serde` JSON form, and
  hands out a sized `Read + Seek` via `open_device` so a downstream
  partition/filesystem analyzer runs on a live disk exactly as on an image file.
- **`livedisk-forensic`** — turns a `PhysicalDisk` into graded
  `forensicnomicon::report::Finding`s flagging acquisition-integrity conditions
  ([ADR 0006](decisions/0006-findings-forensicnomicon-observations.md)):
  `LIVE-MOUNTED`, `LIVE-WRITABLE` (target-only —
  [ADR 0007](decisions/0007-context-aware-live-writable.md)), `LIVE-REMOVABLE`,
  `LIVE-SECTOR-4KN`, `LIVE-SYNTHESIZED` — always observations, never verdicts.

## Users

Library consumers, two kinds: (1) fleet forensic tooling that needs a
cross-platform live-disk inventory and pre-acquisition triage (Issen / disk4n6,
which aggregate the findings into one `Report`); (2) any Rust developer who just
wants `livedisk-core`'s cross-platform disk list without the forensic layer or
`forensicnomicon`.

## Scope

- Cross-platform enumeration of **whole physical (or macOS-synthesized) disks and
  their partitions**, with driver-reported geometry (size, logical/physical
  sector sizes).
- Unprivileged listing; raw device access gated behind `open_device`
  ([ADR 0005](decisions/0005-unprivileged-listing-fail-loud.md)).
- Acquisition-integrity findings derivable from the enumerated model.

## Non-goals

- **Not a filesystem or partition-table parser.** `livedisk` exposes the device
  and the kernel's partition list; interpreting NTFS/APFS/ext4 or carving the
  MBR/GPT is the job of the fleet's filesystem/container crates, which can be
  layered on the `Read + Seek` `open_device` returns.
- **Not an imager.** It reports acquisition risk; it does not acquire.
- **Not a verdict engine.** Findings are graded observations for an examiner to
  weigh, not conclusions.
- **No shelling out.** Backends read kernel structures directly, never
  `diskutil`/`lsblk`/`diskpart` output.

## Correctness & validation

- **Host-independent parser tests.** The Windows `DRIVE_LAYOUT_INFORMATION_EX`
  decoder and the `/proc/mounts` parser are always compiled and unit-tested on any
  host, decoding raw bytes at fixed offsets over the `safe-read` bounded reader
  ([ADR 0004](decisions/0004-host-independent-parsing-safe-read.md)).
- **Fuzzing.** A `fuzzing` feature exposes both parsers to the `fuzz/` harness;
  the committed corpus asserts they never panic on malformed input.
- **Analyzer behaviour is test-locked.** The overview-vs-target `LIVE-WRITABLE`
  split, the mounted/removable/sector-mismatch findings, and the clean-disk
  empty-result case are covered by unit tests in `forensic/src/lib.rs`.
- **Memory-safety posture.** `unsafe_code = "deny"` with `unsafe` confined to the
  macOS/Windows FFI backends behind scoped allows + `// SAFETY:` notes; Linux and
  all pure logic are unsafe-free
  ([ADR 0003](decisions/0003-unsafe-deny-scoped-ffi-allow.md)).
