# 5. Listing is unprivileged; permission failures fail loud, never empty

Date: 2026-07-24
Status: Accepted

## Context

Two capabilities have very different privilege needs. **Listing** disk
layout/metadata reads the kernel's device registry (IOKit registry, sysfs,
`IOCTL_DISK_GET_DRIVE_LAYOUT_EX`) and generally works for an ordinary user.
**Reading raw sectors** (`open_device`) needs root/Administrator.

The dangerous failure mode here is the one the fleet Robustness discipline calls
out (`CLAUDE.core.md` → "Bootstrap failure ≠ artifact-not-found"): a backend that
hits a permission error and returns an *empty* disk list. On a forensic host that
is indistinguishable from "this machine has no disks" — a silent wrong answer that
hides the real cause from the examiner.

## Decision

1. **Keep listing unprivileged.** All three backends read device-registry
   metadata, not raw sectors, so `enumerate()` succeeds without elevation on a
   normal host (Windows layout query still needs Administrator, documented as
   such).
2. **Surface permission problems as `Error`, never as an empty `Vec`.** A backend
   that cannot read the registry returns the typed `Error` (`core/src/lib.rs` doc:
   "Backends therefore never silently return an empty list on a permission
   problem — they surface [`Error`]"). Empty is reserved for a host that genuinely
   has no matching devices.
3. **Separate the two privilege tiers in the API.** `enumerate()` (unprivileged
   listing) is distinct from `open_device()` (privileged raw access returning a
   sized `Read + Seek`), so a caller that only needs the layout never triggers an
   elevation requirement.

## Consequences

- "No disks found" means no disks, not "run me as root" — the examiner gets a
  loud, diagnosable error instead of a plausible-looking empty result.
- The common path (list the disks) needs no privileges; elevation is required only
  when the caller actually opens a device for raw reads.
