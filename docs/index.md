# livedisk

**List every physical disk and partition on the running machine — macOS, Linux, and Windows — through one unified Rust API.**

```rust
for disk in livedisk::enumerate()? {
    println!("{}  {}", disk.name, livedisk::human_size(disk.size_bytes));
    for part in &disk.partitions {
        println!("  {}  {}", part.name, livedisk::human_size(part.size_bytes));
    }
}
# Ok::<(), livedisk::Error>(())
```

**[GitHub Repository →](https://github.com/SecurityRonin/livedisk-forensic)**

---

## What it does

`diskutil list`, `lsblk`, and `diskpart` each speak a different dialect. `livedisk` returns their answer as one set of structs, on every platform, with no daemon and no shelling out.

- **Unified model** — IOKit `IOMedia` on macOS, `/sys/block` on Linux, and `DeviceIoControl` on Windows all fill the same `PhysicalDisk` struct (device path, size, logical/physical sector sizes, model, serial, removable/read-only flags, synthesized-container flag, and partitions). Listing works **unprivileged** — it reads the kernel's device registry, not raw sectors. `open_device` hands you a sized `Read + Seek` so a partition or filesystem analyzer runs on a live disk exactly as it would on an image file.
- **Visual overview** — `render_overview` draws a horizontal bar chart scaled to the largest disk; `render_disk_bar` draws each disk's partition layout proportionally (ANSI colour on a TTY, ASCII when piped).
- **Acquisition-integrity triage** — `livedisk-forensic` turns a live disk into graded [`forensicnomicon`](https://crates.io/crates/forensicnomicon) findings — never a verdict, always an observation.

## Acquisition-integrity findings

| Code | Meaning |
|---|---|
| `LIVE-MOUNTED` | a volume is mounted during acquisition (live writes may alter the image) |
| `LIVE-WRITABLE` | the device is writable; no hardware write-blocker detected |
| `LIVE-REMOVABLE` | removable media |
| `LIVE-SECTOR-4KN` | logical/physical sector sizes differ (512e/4Kn) |
| `LIVE-SYNTHESIZED` | a synthesized container overlay, not a backing physical store |

## The two crates

Mirroring the forensic-fleet split: **`livedisk-core`** (the reader, imported as `livedisk`) and **`livedisk-forensic`** (the analyzer).

---

[Privacy Policy](privacy.md) · [Terms of Service](terms.md) · [GitHub](https://github.com/SecurityRonin/livedisk-forensic) · © 2026 Security Ronin Ltd.
