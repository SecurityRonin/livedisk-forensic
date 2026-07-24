# 7. `LIVE-WRITABLE` is acquisition-target-only, split from the host overview

Date: 2026-07-24
Status: Accepted

## Context

The first analyzer emitted `LIVE-WRITABLE` for every writable device. On a live
host that is *every internal disk* — writability is the baseline, not an anomaly —
so the finding fired on all of them and buried the discriminating findings
(mounted, removable, sector-mismatch) in noise. The signal "no write-blocker is
engaged" is only meaningful for the one device an examiner intends to **acquire**,
where it means imaging could alter the evidence.

This was a deliberate mid-development correction, recorded as a TDD RED/GREEN pair
in the git history: `9acef93` ("test(forensic): RED — context-aware LIVE-WRITABLE
(overview omits, target warns)") then `8c71588` ("feat(forensic): GREEN —
LIVE-WRITABLE is acquisition-target-only"), released as livedisk 0.1.3 (`ac1f61f`).

## Decision

Split the analyzer into two entry points (`forensic/src/lib.rs`):

- **`analyse(disk)`** — the host overview. Emits `LIVE-MOUNTED`, `LIVE-REMOVABLE`,
  `LIVE-SECTOR-4KN`, `LIVE-SYNTHESIZED`. It **deliberately does not** emit
  `LIVE-WRITABLE` (an in-code comment states why).
- **`analyse_target(disk)`** — for a disk you intend to acquire. Returns
  everything `analyse` does, **plus** `LIVE-WRITABLE` (High, Integrity) when the
  device is not read-only.

The distinction is locked down by tests: `overview_analyse_omits_live_writable`,
`target_analyse_flags_writable_high`, and
`target_analyse_write_blocked_has_no_writable`.

## Consequences

- The host-wide overview stays high signal-to-noise; the writability warning
  appears exactly where it is actionable — on the acquisition target.
- Callers must choose the right entry point: overview enumeration → `analyse`,
  pre-acquisition triage of a specific device → `analyse_target`.
- A read-only (write-blocked) target produces no `LIVE-WRITABLE` — reassuring
  silence rather than a redundant "all clear" finding.
