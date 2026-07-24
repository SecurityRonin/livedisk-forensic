# 3. `unsafe_code = "deny"` with scoped FFI allows, not `forbid`

Date: 2026-07-24
Status: Accepted

## Context

The fleet default is `unsafe_code = "forbid"` — a provable, badge-able "zero
places a crafted input can corrupt memory" (`ronin-issen/CLAUDE.md` → "Rust Lint
Posture"; "`unsafe` Is an Avoidable Cost-Benefit Exception"). But two of the
three discovery backends are inherently FFI: the macOS IOKit / CoreFoundation
calls and the Windows `DeviceIoControl` path both cross into C ABIs that safe
Rust cannot express. `forbid` cannot be locally overridden, so a repo that needs
any `unsafe` at all must sit at `deny`.

## Decision

Set `unsafe_code = "deny"` at the workspace level (`Cargo.toml`
`[workspace.lints.rust]`) with a documented rationale, and confine `unsafe` to the
platform FFI backends behind scoped `#[allow(unsafe_code)]` + per-call `// SAFETY:`
notes. The Linux backend and all pure logic (rendering, sizing, parsing) stay
unsafe-free by construction. The critical parsers — the Windows
`DRIVE_LAYOUT_INFORMATION_EX` decode and `/proc/mounts` — read bytes/text rather
than transmuting foreign structs, so the *untrusted-input* surface is safe even
though the *syscall* surface is not (see ADR 0004).

## Consequences

- The FFI `unsafe` is minimal, pure-Rust-plus-system-headers (no `-sys`
  C-source compilation of our own), and auditable via `rg 'allow(unsafe_code)'`.
- The crate cannot wear an "unsafe-forbidden" badge; the README badge policy is
  honoured by omitting it rather than misrepresenting the posture (mirrors the
  `ewf`/`memory-forensic` mmap crates, which are also `deny` + bounded allow).
- Byte/text parsers being safe means the fuzz targets (ADR 0004) exercise safe
  code — a malformed layout buffer cannot reach the FFI at all.
