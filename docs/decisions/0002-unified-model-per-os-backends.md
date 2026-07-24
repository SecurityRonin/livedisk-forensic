# 2. One unified disk model, three OS-specific discovery backends

Date: 2026-07-24
Status: Accepted

## Context

`diskutil list` (macOS), `lsblk` (Linux), and `diskpart` (Windows) each report
the same physical reality in an incompatible dialect. A cross-platform forensic
tool that shells out to these parses three text formats, inherits their locale
and version quirks, and needs the binaries present on the evidence host. Only the
*discovery* of devices is genuinely OS-specific; everything downstream — sizing,
rendering, JSON, the forensic judgement — is platform-agnostic once the data is
in a common shape.

## Decision

Define one model — `PhysicalDisk` / `Partition` (`core/src/lib.rs`) — and fill it
from a native per-OS backend, with **no subprocess and no daemon**:

- **macOS** — the IOKit `IOMedia` registry (`core/src/macos.rs`), which also
  surfaces APFS-synthesized containers (`synthesized: true`).
- **Linux** — `/sys/block` sysfs + `/proc/mounts` (`core/src/linux.rs`,
  `core/src/sysfs.rs`), zero C dependencies.
- **Windows** — `DeviceIoControl(IOCTL_DISK_GET_DRIVE_LAYOUT_EX)`
  (`core/src/windows.rs`, `core/src/drive_layout.rs`).

`size_bytes` and both sector sizes come from the OS/driver layer, never from the
on-disk partition table — only the kernel knows the device's true geometry (doc
comment on `PhysicalDisk`). Everything above discovery (`render_overview`,
`render_disk_bar`, `render_listing`, the `serde` JSON form, and the whole
`livedisk-forensic` analyzer) is written once against the unified model.
`open_device` returns a sized `Read + Seek` so a partition/filesystem analyzer
runs on a live device exactly as it would on an image file.

## Consequences

- One data shape, one set of downstream code, three thin discovery modules — a
  new platform is a new backend, not a new pipeline.
- No dependency on `diskutil`/`lsblk`/`diskpart` being installed or on parsing
  their output; the tool reads kernel structures directly.
- The `synthesized` flag captures the macOS APFS-container / Linux device-mapper
  overlay distinction in the model, so the analyzer can flag it (`LIVE-SYNTHESIZED`)
  without re-discovering it.
