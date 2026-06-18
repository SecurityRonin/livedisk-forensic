# livedisk-forensic

[![Crates.io](https://img.shields.io/crates/v/livedisk-forensic.svg)](https://crates.io/crates/livedisk-forensic)
[![Docs.rs](https://docs.rs/livedisk-forensic/badge.svg)](https://docs.rs/livedisk-forensic)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/SecurityRonin/livedisk-forensic/actions/workflows/ci.yml/badge.svg)](https://github.com/SecurityRonin/livedisk-forensic/actions/workflows/ci.yml)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-db61a2)](https://github.com/sponsors/h4x0r)

**Acquisition-integrity findings for live block devices — graded `forensicnomicon` findings, built on [`livedisk-core`](https://crates.io/crates/livedisk-core).**

Given a [`livedisk::PhysicalDisk`](https://docs.rs/livedisk-core), `analyse` returns graded findings that bear on a forensically sound acquisition of the running system — observations, never verdicts.

```toml
[dependencies]
livedisk-forensic = "0.1"
```

```rust
for disk in livedisk::enumerate()? {
    for finding in livedisk_forensic::analyse(&disk) {
        println!("{}: {}", finding.code, finding.note);
    }
}
# Ok::<(), livedisk::Error>(())
```

| Code | Meaning |
|---|---|
| `LIVE-MOUNTED` | a volume is mounted during acquisition (live writes may alter the image) |
| `LIVE-WRITABLE` | the device being **acquired** is writable — no write-blocker engaged (emitted only by `analyse_target`, not the host overview, since every live disk is writable) |
| `LIVE-REMOVABLE` | removable media |
| `LIVE-SECTOR-4KN` | logical/physical sector sizes differ (512e/4Kn) |
| `LIVE-SYNTHESIZED` | a synthesized container overlay, not a backing physical store |

---

[Privacy Policy](https://securityronin.github.io/livedisk-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/livedisk-forensic/terms/) · © 2026 Security Ronin Ltd
