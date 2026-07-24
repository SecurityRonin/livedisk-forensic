# 4. Host-independent byte parsing over `safe-read`

Date: 2026-07-24
Status: Accepted

## Context

The Windows backend receives a `DRIVE_LAYOUT_INFORMATION_EX` buffer from
`IOCTL_DISK_GET_DRIVE_LAYOUT_EX`, and the Linux backend receives `/proc/mounts`
text. Both are externally-supplied, potentially-malformed byte/character streams.
Two problems follow:

1. **Testability.** If the parser is `#[cfg(windows)]`, its tests only run on
   Windows CI — the platform we can least conveniently exercise. The offsets are
   fixed by the `#[repr(C)]` layout of the `windows-sys` structs, not by the host,
   so nothing about decoding them actually requires Windows.
2. **Robustness.** Hand-rolled `data[off..off+4]` reads can panic on a truncated
   buffer or overflow `usize` on a hostile length field. The fleet forbids
   re-deriving per-crate byte readers (`ronin-issen/CLAUDE.md` → Paranoid
   Gatekeeper: "route through the `safe-read` crate; NEVER hand-roll a per-crate
   `bytes.rs`").

## Decision

1. **Split pure parsing from I/O.** `drive_layout.rs` and `sysfs.rs` are
   *always compiled* (gated only with `#[cfg_attr(not(windows), allow(dead_code))]`
   / the Linux equivalent); only the `DeviceIoControl` / file-I/O that *fills* the
   buffer is platform-gated in `windows.rs` / `linux.rs`. So the parser tests run
   on any host.
2. **Parse raw bytes at fixed offsets, do not transmute the C struct.**
   `drive_layout.rs` decodes the layout by documented offset (partition array at
   byte 48, 144-byte `PARTITION_INFORMATION_EX`, `StartingOffset` +8,
   `PartitionLength` +16, GPT type GUID +32, GPT name `[u16;36]` +72, MBR type
   byte +32), little-endian — keeping the decode safe and host-independent.
3. **Fixed-width 16/32-bit fields go through `safe-read`** (`le_u16`, `le_u32`),
   the fleet's single audited, `forbid(unsafe)`, fuzzed bounded reader that returns
   0 out of range instead of panicking. The two signed 64-bit fields
   (`StartingOffset` +8, `PartitionLength` +16) are decoded by a small local
   `i64_le` helper (`i64::from_le_bytes`, `drive_layout.rs`), reached only after the
   caller has bounds-checked the whole 144-byte entry against the buffer length
   (the `e + ENTRY_SIZE > buf.len()` guard), so that read stays in bounds by
   construction.
4. **Fuzz the parsers.** A `fuzzing` feature exposes `fuzz_api::parse_drive_layout`
   and `fuzz_api::parse_mounts` (`core/src/lib.rs`) to the `fuzz/` harness, whose
   corpus asserts the parsers never panic on malformed input.

The dependency reached its current form through a rename: it began as
`forensic-bytes` (`2593bd0`), was renamed to `safe-read` (`386efe6`), and was
finally pinned to the published crate rather than a path dep (`82490fe`) — the
"prefer the published registry crate once it's on crates.io" rule.

## Consequences

- Windows/Linux parser correctness is proven on macOS/Linux CI, not only on the
  platform that produces the data.
- Malformed or truncated buffers degrade to empty/partial results, never panics —
  verified continuously by the fuzz corpus.
- `safe-read` handles fixed-width fields only; range-checking length/offset/count
  values from the buffer before use remains the parser's job, done in
  `drive_layout.rs`.
