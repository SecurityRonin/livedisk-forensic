#![no_main]
//! Fuzz the /proc/mounts text parser (octal-escape decoding) against arbitrary
//! input — it must never panic.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    livedisk::fuzz_api::parse_mounts(&text);
});
