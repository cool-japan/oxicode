#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Feed arbitrary bytes to streaming decoder — must never panic
    let mut decoder = oxicode::streaming::BufferStreamingDecoder::new(data);
    // Try reading up to 100 items; stop on error
    for _ in 0..100 {
        match decoder.read_item::<u32>() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => break,
        }
    }
});
