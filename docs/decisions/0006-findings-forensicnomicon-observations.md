# 6. Findings are `forensicnomicon::report` observations, never verdicts

Date: 2026-07-24
Status: Accepted

## Context

The fleet standardises every analyzer's output on one model —
`forensicnomicon::report::Finding` — so orchestration (Issen, disk4n6) and a
future GUI render findings uniformly instead of N bespoke `XxxAnalysis` types
(`ronin-issen/CLAUDE.md` → "The Reporting Model"). Acquisition-integrity
conditions are *epistemically observations*: a mounted volume or a writable target
is a fact about the acquisition context, not a legal or even a definitive forensic
conclusion.

## Decision

`livedisk-forensic::analyse` emits `Vec<Finding>` built through the
`Finding::observation(severity, category, code)` builder (`forensic/src/lib.rs`),
one graded finding per condition, each carrying a `Source { analyzer:
"livedisk-forensic", scope: <disk name>, version }`:

| Code | Severity | Category |
|---|---|---|
| `LIVE-MOUNTED` | High | Integrity |
| `LIVE-WRITABLE` | High | Integrity (target-only — ADR 0007) |
| `LIVE-REMOVABLE` | Info | Provenance |
| `LIVE-SECTOR-4KN` | Info | Structure |
| `LIVE-SYNTHESIZED` | Info | Provenance |

Notes are worded as observations ("consistent with imaging a running system"),
never verdicts; findings attach concrete `evidence` (mount points, both sector
sizes) so a reader can check the datum. A pristine target (write-protected,
unmounted, fixed, physical, matching sectors) returns **empty** — reassuring
silence, tested in `clean_write_protected_disk_has_no_findings`.

The reporting dependency tracked `forensicnomicon` as it matured:
`0.5 → 0.11` (`cd60850`) and the `1.0` cut (`594348a`, workspace pins
`forensicnomicon = "1"`).

## Consequences

- Issen / disk4n6 aggregate these findings into one `Report` alongside every other
  fleet analyzer with no bespoke adapter.
- `code` values are a published contract (scheme-prefixed SCREAMING-KEBAB); a
  shipped code is never repurposed, new conditions get new codes.
- The neutral, evidence-bearing wording keeps the analyzer on the "observation"
  side of the observed-fact / inference / legal-conclusion boundary — the
  examiner, not the tool, concludes.
