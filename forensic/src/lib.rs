//! # livedisk-forensic
//!
//! Acquisition-integrity analysis of a live block device enumerated by
//! [`livedisk`]. Given a [`PhysicalDisk`], [`analyse`] returns graded
//! [`forensicnomicon`] findings flagging conditions that bear on a *forensically
//! sound acquisition* of the running system — never a verdict, always an
//! observation:
//!
//! - `LIVE-MOUNTED` — a volume is mounted during acquisition (live writes may
//!   alter the image).
//! - `LIVE-WRITABLE` — the device is writable; no hardware write-blocker
//!   detected.
//! - `LIVE-REMOVABLE` — removable media.
//! - `LIVE-SECTOR-4KN` — logical/physical sector sizes differ (512e/4Kn).
//! - `LIVE-SYNTHESIZED` — a synthesized container overlay, not a backing
//!   physical store.
//!
//! ```no_run
//! for disk in livedisk::enumerate()? {
//!     for finding in livedisk_forensic::analyse(&disk) {
//!         println!("{}: {}", finding.code, finding.note);
//!     }
//! }
//! # Ok::<(), livedisk::Error>(())
//! ```

use forensicnomicon::report::{Category, Finding, Severity, Source};
use livedisk::PhysicalDisk;

/// Analyzer name recorded on every finding's [`Source`].
const ANALYZER: &str = "livedisk-forensic";

fn source(disk: &PhysicalDisk) -> Source {
    Source {
        analyzer: ANALYZER.to_string(),
        scope: disk.name.clone(),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
    }
}

/// Analyse a live disk for acquisition-integrity conditions, returning graded
/// findings (empty for a write-protected, unmounted, fixed, non-synthesized
/// disk with matching sector sizes — the ideal acquisition target).
#[must_use]
pub fn analyse(disk: &PhysicalDisk) -> Vec<Finding> {
    let mut findings = Vec::new();

    // LIVE-MOUNTED — mounted volumes during acquisition risk altering the image.
    let mounted = disk
        .partitions
        .iter()
        .filter(|p| p.mount_point.is_some())
        .count();
    if mounted > 0 {
        let mut builder = Finding::observation(Severity::High, Category::Integrity, "LIVE-MOUNTED")
            .source(source(disk))
            .note(
                "device has mounted volume(s) during acquisition; live writes may alter the \
                 image — consistent with imaging a running system",
            );
        for p in &disk.partitions {
            if let Some(mount) = &p.mount_point {
                builder = builder.evidence(p.name.clone(), mount.clone());
            }
        }
        findings.push(builder.build());
    }

    // LIVE-WRITABLE — no write-blocker means acquisition can alter the evidence.
    if !disk.read_only {
        findings.push(
            Finding::observation(Severity::Medium, Category::Integrity, "LIVE-WRITABLE")
                .source(source(disk))
                .note(
                    "device is writable (no hardware write-blocker detected); acquisition can \
                     alter the evidence",
                )
                .build(),
        );
    }

    // LIVE-REMOVABLE — removable media (provenance/chain-of-custody context).
    if disk.removable {
        findings.push(
            Finding::observation(Severity::Info, Category::Provenance, "LIVE-REMOVABLE")
                .source(source(disk))
                .note("removable media")
                .build(),
        );
    }

    // LIVE-SECTOR-4KN — 512e/4Kn mismatch; image aligned to the physical sector.
    if disk.logical_sector_size > 0 && disk.physical_sector_size != disk.logical_sector_size {
        findings.push(
            Finding::observation(Severity::Info, Category::Structure, "LIVE-SECTOR-4KN")
                .source(source(disk))
                .note(
                    "logical and physical sector sizes differ (512e/4Kn); align imaging to the \
                     physical sector size",
                )
                .evidence("logical_sector_size", disk.logical_sector_size.to_string())
                .evidence(
                    "physical_sector_size",
                    disk.physical_sector_size.to_string(),
                )
                .build(),
        );
    }

    // LIVE-SYNTHESIZED — overlay (APFS container, device-mapper), not a store.
    if disk.synthesized {
        findings.push(
            Finding::observation(Severity::Info, Category::Provenance, "LIVE-SYNTHESIZED")
                .source(source(disk))
                .note(
                    "synthesized device — a container overlay (e.g. APFS container, \
                     device-mapper/LVM) over one or more physical stores, not itself a backing \
                     physical disk",
                )
                .build(),
        );
    }

    findings
}

/// Analyse a disk you intend to **acquire** (image). Returns everything
/// [`analyse`] reports for the host overview, plus the acquisition-target-only
/// `LIVE-WRITABLE` warning when the device is writable — i.e. no hardware
/// write-blocker is engaged, so imaging could alter the evidence. On a live
/// host every internal disk is writable, so that condition is omitted from the
/// overview [`analyse`] (it would fire on every device); it is signal only for
/// the specific device under acquisition.
#[must_use]
pub fn analyse_target(disk: &PhysicalDisk) -> Vec<Finding> {
    let mut findings = analyse(disk);
    if !disk.read_only {
        findings.push(
            Finding::observation(Severity::High, Category::Integrity, "LIVE-WRITABLE")
                .source(source(disk))
                .note(
                    "acquisition target is writable — no hardware write-blocker is engaged; \
                     imaging can alter the evidence",
                )
                .build(),
        );
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use livedisk::Partition;

    /// A pristine acquisition target: write-protected, unmounted, fixed,
    /// physical, matching sector sizes.
    fn clean_disk() -> PhysicalDisk {
        PhysicalDisk {
            device_path: "/dev/disk0".into(),
            name: "disk0".into(),
            size_bytes: 1_000_000_000_000,
            logical_sector_size: 512,
            physical_sector_size: 512,
            model: Some("WRITE BLOCKED".into()),
            serial: None,
            removable: false,
            read_only: true,
            synthesized: false,
            partitions: vec![],
        }
    }

    fn codes(findings: &[Finding]) -> Vec<&str> {
        findings.iter().map(|f| f.code.as_ref()).collect()
    }

    #[test]
    fn clean_write_protected_disk_has_no_findings() {
        assert!(analyse(&clean_disk()).is_empty());
    }

    #[test]
    fn overview_analyse_omits_live_writable() {
        // Writable is the baseline on a live host — flagging it on every disk is
        // noise, so the overview analyser must not emit LIVE-WRITABLE.
        let mut d = clean_disk();
        d.read_only = false;
        assert!(!codes(&analyse(&d)).contains(&"LIVE-WRITABLE"));
    }

    #[test]
    fn target_analyse_flags_writable_high() {
        // Imaging a writable target means no write-blocker is engaged — a real,
        // high-severity acquisition risk.
        let mut d = clean_disk();
        d.read_only = false;
        let findings = analyse_target(&d);
        let f = findings.iter().find(|f| f.code == "LIVE-WRITABLE").unwrap();
        assert_eq!(f.severity, Some(Severity::High));
        assert_eq!(f.source.analyzer, "livedisk-forensic");
        assert_eq!(f.source.scope, "disk0");
    }

    #[test]
    fn target_analyse_write_blocked_has_no_writable() {
        // A read-only target = write-blocker engaged → reassuring silence.
        assert!(!codes(&analyse_target(&clean_disk())).contains(&"LIVE-WRITABLE"));
    }

    #[test]
    fn mounted_disk_flags_live_mounted_high_with_evidence() {
        let mut d = clean_disk();
        d.partitions = vec![Partition {
            device_path: "/dev/disk0s1".into(),
            name: "disk0s1".into(),
            start_offset: 0,
            size_bytes: 1,
            partition_type: None,
            mount_point: Some("/Volumes/Data".into()),
            filesystem: None,
            label: None,
        }];
        let findings = analyse(&d);
        let f = findings.iter().find(|f| f.code == "LIVE-MOUNTED").unwrap();
        assert_eq!(f.severity, Some(Severity::High));
        assert!(f.evidence.iter().any(|e| e.value == "/Volumes/Data"));
    }

    #[test]
    fn removable_disk_flags_live_removable_info() {
        let mut d = clean_disk();
        d.removable = true;
        assert!(codes(&analyse(&d)).contains(&"LIVE-REMOVABLE"));
    }

    #[test]
    fn sector_mismatch_flags_4kn_with_both_sizes() {
        let mut d = clean_disk();
        d.logical_sector_size = 512;
        d.physical_sector_size = 4096;
        let findings = analyse(&d);
        let f = findings
            .iter()
            .find(|f| f.code == "LIVE-SECTOR-4KN")
            .unwrap();
        assert!(f.evidence.iter().any(|e| e.value == "4096"));
        assert!(f.evidence.iter().any(|e| e.value == "512"));
    }

    #[test]
    fn synthesized_disk_flags_live_synthesized() {
        let mut d = clean_disk();
        d.synthesized = true;
        assert!(codes(&analyse(&d)).contains(&"LIVE-SYNTHESIZED"));
    }
}
