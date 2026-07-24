# 8. Low, CI-verified MSRV floor decoupled from the pinned dev toolchain

Date: 2026-07-24
Status: Accepted

## Context

`livedisk-core` and `livedisk-forensic` are **published libraries** — external
code links them and may pin an MSRV against them. The fleet policy
(`CLAUDE.core.md` → "Rust MSRV & Toolchain Policy"; `CLAUDE.personal.md` → fleet
specifics) separates two distinct versions that a naive setup conflates: the
**dev/CI toolchain** (what we build, fmt, and clippy with) and the **declared
MSRV** (`rust-version`, a downstream-facing compatibility promise). A published
library keeps its MSRV low and CI-verified; raising it narrows the crates.io
audience and is treated as a near-breaking change.

## Decision

- **Dev/CI toolchain** is pinned to the current fleet stable in
  `rust-toolchain.toml` (`channel = "1.96.0"`, with `rustfmt` + `clippy`
  components declared in-toml so CI and local agree).
- **Declared MSRV** is set lower and separately in `[workspace.package]`
  (`rust-version = "1.85"`) and inherited by both members via
  `rust-version.workspace = true`. Both `Cargo.toml` and `rust-toolchain.toml`
  carry comments stating that the two numbers are intentionally different and why.
- The MSRV floor is a promise, so it is verified by a dedicated low-MSRV CI job
  (not merely asserted).

## Consequences

- Contributors and CI all build on one current stable — no fmt/clippy drift.
- Downstream consumers get a genuine `1.85` compatibility guarantee, independent
  of whatever stable the fleet develops on; a future toolchain bump does not
  silently raise the promised floor.
- Raising `rust-version` later is a deliberate, reasoned change (a newer-Rust
  feature genuinely needed), never an incidental side effect of bumping the
  toolchain pin.
