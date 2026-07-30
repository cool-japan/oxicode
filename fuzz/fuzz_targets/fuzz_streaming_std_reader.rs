#![no_main]
use libfuzzer_sys::fuzz_target;
use oxicode::streaming::StreamingDecoder;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // Exercises the `std::io::Read`-generic streaming decoder path
    // (`StreamingDecoder<R: Read>`), as opposed to the buffer-only
    // `BufferStreamingDecoder` already covered by fuzz_streaming.rs. Driving
    // it over a `Cursor<&[u8]>` takes the codepath through `std::io::Read`
    // rather than direct slice indexing, which is a materially different
    // implementation (chunk assembly buffers, `Read::read` short-read
    // handling, EOF detection) and must never panic or misbehave on
    // arbitrary untrusted framing.
    let mut decoder = StreamingDecoder::<_>::new(Cursor::new(data));
    for _ in 0..100 {
        match decoder.read_item::<u32>() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => break,
        }
    }

    // Same stream, different item type: strings exercise the allocating
    // decode path (length-prefixed bytes) rather than a fixed-size integer.
    let mut decoder = StreamingDecoder::<_>::new(Cursor::new(data));
    for _ in 0..100 {
        match decoder.read_item::<String>() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => break,
        }
    }

    // Fixed-int + big-endian config, to exercise the alternate codec
    // configuration path through the same std::io::Read-backed decoder.
    let fixed_be_config = oxicode::config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let mut decoder =
        StreamingDecoder::new_with_config(Cursor::new(data), fixed_be_config);
    for _ in 0..100 {
        match decoder.read_item::<u64>() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => break,
        }
    }
});
