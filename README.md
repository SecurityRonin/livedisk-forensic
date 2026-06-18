# livedisk

[![Crates.io](https://img.shields.io/crates/v/livedisk-core.svg)](https://crates.io/crates/livedisk-core)
[![Docs.rs](https://docs.rs/livedisk-core/badge.svg)](https://docs.rs/livedisk-core)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/SecurityRonin/livedisk-forensic/actions/workflows/ci.yml/badge.svg)](https://github.com/SecurityRonin/livedisk-forensic/actions/workflows/ci.yml)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-db61a2)](https://github.com/sponsors/h4x0r)

**List every physical disk and partition on the running machine — macOS, Linux, and Windows — through one unified Rust API.**

`diskutil list`, `lsblk`, and `diskpart` each speak a different dialect. `livedisk` gives you their answer as one set of structs, on every platform, with no daemon and no shelling out — plus a partition-manager-style visual and acquisition-integrity triage.

```rust
for disk in livedisk::enumerate()? {
    println!("{}  {}", disk.name, livedisk::human_size(disk.size_bytes));
    for part in &disk.partitions {
        println!("  {}  {}", part.name, livedisk::human_size(part.size_bytes));
    }
}
# Ok::<(), livedisk::Error>(())
```

```console
$ cargo add livedisk-core      # the reader (imported as `livedisk`)
```

## At a glance

`render_overview` draws a horizontal bar chart scaled to the largest physical disk; `render_disk_bar` draws each disk's partition layout proportionally (ANSI colour on a TTY, ASCII when piped):

```text
All storage (3 physical disks, 14.1 TB total):
 disk0  [############################                            ]   4.0 TB  28.5%
 disk4  [==============                                          ]   2.0 TB  14.6%
 disk5  [++++++++++++++++++++++++++++++++++++++++++++++++++++++++]   8.0 TB  56.9%
```

## Unified model

Every backend — IOKit `IOMedia` on macOS, `/sys/block` on Linux, `DeviceIoControl` on Windows — fills the same struct:

```rust
pub struct PhysicalDisk {
    pub device_path: String,        // /dev/disk0, /dev/sda, \\.\PhysicalDrive0
    pub name: String,
    pub size_bytes: u64,
    pub logical_sector_size: u32,
    pub physical_sector_size: u32,  // 4Kn/512e aware
    pub model: Option<String>,
    pub serial: Option<String>,
    pub removable: bool,
    pub read_only: bool,
    pub synthesized: bool,          // APFS container / device-mapper overlay
    pub partitions: Vec<Partition>,
}
```

Listing works **unprivileged** (it reads the kernel's device registry, not raw sectors). [`open_device`] hands you a sized `Read + Seek` so a partition or filesystem analyzer can run on a live disk exactly as it would on an image file.

## Acquisition-integrity triage

`livedisk-forensic` turns a live disk into graded [`forensicnomicon`](https://crates.io/crates/forensicnomicon) findings — never a verdict, always an observation:

| Code | Meaning |
|---|---|
| `LIVE-MOUNTED` | a volume is mounted during acquisition (live writes may alter the image) |
| `LIVE-WRITABLE` | the device is writable; no hardware write-blocker detected |
| `LIVE-REMOVABLE` | removable media |
| `LIVE-SECTOR-4KN` | logical/physical sector sizes differ (512e/4Kn) |
| `LIVE-SYNTHESIZED` | a synthesized container overlay, not a backing physical store |

```rust
for finding in livedisk_forensic::analyse(&disk) {
    println!("{}: {}", finding.code, finding.note);
}
```

## Platform support

| OS | Backend | Notes |
|---|---|---|
| macOS | IOKit `IOMedia` registry | physical + APFS-synthesized disks |
| Linux | `/sys/block` sysfs + `/proc/mounts` | zero C dependencies |
| Windows | `DeviceIoControl` (`IOCTL_DISK_GET_DRIVE_LAYOUT_EX`) | layout query needs Administrator |

Two crates, mirroring the forensic-fleet split: **`livedisk-core`** (the reader, imported as `livedisk`) and **`livedisk-forensic`** (the analyzer).

---

[Privacy Policy](https://securityronin.github.io/livedisk-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/livedisk-forensic/terms/) · © 2026 Security Ronin Ltd
