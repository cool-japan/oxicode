#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Try to decode arbitrary bytes as various types — must never panic/UB
    let _ = oxicode::decode_from_slice::<u64>(data);
    let _ = oxicode::decode_from_slice::<String>(data);
    let _ = oxicode::decode_from_slice::<Vec<u8>>(data);
    let _ = oxicode::decode_from_slice::<(u32, String, bool)>(data);
    let _ = oxicode::decode_from_slice::<Option<Vec<u32>>>(data);
});
