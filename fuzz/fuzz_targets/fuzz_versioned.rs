#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Versioned decode must never panic on arbitrary input
    let _ = oxicode::versioning::decode_versioned(data);
    // Also try extract_version
    let _ = oxicode::versioning::extract_version(data);
    // Also try is_versioned (infallible)
    let _ = oxicode::versioning::is_versioned(data);
});
