#![no_main]
//! Fuzz the Windows DRIVE_LAYOUT_INFORMATION_EX byte parser against arbitrary
//! input — it must never panic, overflow, or over-allocate.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    livedisk::fuzz_api::parse_drive_layout(data);
});
