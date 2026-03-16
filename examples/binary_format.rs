//! Demonstrates OxiCode's binary wire format
//!
//! Run with: cargo run --example binary_format

use oxicode::{
    config, display::EncodedBytes, encode_to_vec, encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Helper: print hex representation
// ---------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    format!("{:x}", EncodedBytes(bytes))
}

// ---------------------------------------------------------------------------
// Types used for struct/enum wire format demos
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum SmallEnum {
    A,
    B,
    C(u32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Nested {
    label: String,
    point: Point,
    active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Wire Format Examples\n");
    println!("Format: value -> N bytes: hex\n");

    // -----------------------------------------------------------------------
    // 1. Varint encoding for unsigned integers
    // -----------------------------------------------------------------------
    println!("--- Varint encoding (standard config) ---");
    println!("Values <= 127 fit in a single byte (MSB = 0 = 'final byte'):");
    for n in [0u64, 1, 63, 127] {
        let bytes = encode_to_vec(&n).expect("encode u64");
        println!(
            "  u64 {:>20} -> {} byte(s):  {}",
            n,
            bytes.len(),
            hex(&bytes)
        );
    }
    println!("\nValues 128+ use multi-byte varint (7 bits per byte, MSB=1 = 'more bytes follow'):");
    for n in [
        128u64,
        255,
        256,
        16383,
        16384,
        65535,
        u32::MAX as u64,
        u64::MAX,
    ] {
        let bytes = encode_to_vec(&n).expect("encode u64");
        println!(
            "  u64 {:>20} -> {} byte(s):  {}",
            n,
            bytes.len(),
            hex(&bytes)
        );
    }

    // -----------------------------------------------------------------------
    // 2. Signed integers (zigzag encoding)
    // -----------------------------------------------------------------------
    println!("\n--- Signed integer encoding (zigzag varint) ---");
    println!("Zigzag maps small absolute values to small varints regardless of sign:");
    for n in [
        0i64,
        -1,
        1,
        -64,
        63,
        -128,
        127,
        i32::MIN as i64,
        i32::MAX as i64,
    ] {
        let bytes = encode_to_vec(&n).expect("encode i64");
        println!(
            "  i64 {:>20} -> {} byte(s):  {}",
            n,
            bytes.len(),
            hex(&bytes)
        );
    }

    // -----------------------------------------------------------------------
    // 3. Fixed-width encoding (legacy config — bincode 1.x compatible)
    // -----------------------------------------------------------------------
    println!("\n--- Fixed-int encoding (legacy config, bincode 1.x compatible) ---");
    println!("Every integer is written at its natural width (1/2/4/8 bytes):");
    for n in [0u32, 1, 255, 65535, u32::MAX] {
        let bytes = encode_to_vec_with_config(&n, config::legacy()).expect("encode u32 legacy");
        println!("  u32 {:>10} -> {} bytes: {}", n, bytes.len(), hex(&bytes));
    }
    for n in [0u64, 1, u64::MAX] {
        let bytes = encode_to_vec_with_config(&n, config::legacy()).expect("encode u64 legacy");
        println!("  u64 {:>20} -> {} bytes: {}", n, bytes.len(), hex(&bytes));
    }

    println!("\nStandard vs legacy size comparison for u32::MAX:");
    let std_bytes = encode_to_vec(&u32::MAX).expect("encode");
    let leg_bytes = encode_to_vec_with_config(&u32::MAX, config::legacy()).expect("encode legacy");
    println!(
        "  standard (varint): {} bytes: {}",
        std_bytes.len(),
        hex(&std_bytes)
    );
    println!(
        "  legacy  (fixed32): {} bytes: {}",
        leg_bytes.len(),
        hex(&leg_bytes)
    );

    // -----------------------------------------------------------------------
    // 4. Floating point
    // -----------------------------------------------------------------------
    println!("\n--- Floating-point encoding (always 4 or 8 bytes, IEEE 754) ---");
    for f in [0.0f32, 1.0, -1.0, f32::NAN, f32::INFINITY] {
        let bytes = encode_to_vec(&f).expect("encode f32");
        println!("  f32 {:>12} -> {} bytes: {}", f, bytes.len(), hex(&bytes));
    }
    for f in [0.0f64, 1.0, std::f64::consts::PI, f64::NEG_INFINITY] {
        let bytes = encode_to_vec(&f).expect("encode f64");
        println!("  f64 {:>20} -> {} bytes: {}", f, bytes.len(), hex(&bytes));
    }

    // -----------------------------------------------------------------------
    // 5. Boolean
    // -----------------------------------------------------------------------
    println!("\n--- Boolean encoding (1 byte: 0x00=false, 0x01=true) ---");
    for b in [false, true] {
        let bytes = encode_to_vec(&b).expect("encode bool");
        println!("  bool {:>5} -> {} byte:  {}", b, bytes.len(), hex(&bytes));
    }

    // -----------------------------------------------------------------------
    // 6. Strings
    // -----------------------------------------------------------------------
    println!("\n--- String encoding (varint length prefix + UTF-8 bytes) ---");
    for s in ["", "hi", "hello", "こんにちは"] {
        let bytes = encode_to_vec(&s.to_string()).expect("encode String");
        println!(
            "  String {:>12} -> {} bytes: {}",
            format!("{:?}", s),
            bytes.len(),
            hex(&bytes)
        );
    }

    // -----------------------------------------------------------------------
    // 7. Options
    // -----------------------------------------------------------------------
    println!("\n--- Option encoding (0x00=None, 0x01+payload=Some) ---");
    let none: Option<u32> = None;
    let some: Option<u32> = Some(42);
    let some_large: Option<u32> = Some(u32::MAX);
    for (label, opt) in [
        ("None", &none),
        ("Some(42)", &some),
        ("Some(u32::MAX)", &some_large),
    ] {
        let bytes = encode_to_vec(opt).expect("encode Option");
        println!(
            "  Option<u32> {:>15} -> {} bytes: {}",
            label,
            bytes.len(),
            hex(&bytes)
        );
    }

    // -----------------------------------------------------------------------
    // 8. Collections (Vec)
    // -----------------------------------------------------------------------
    println!("\n--- Vec encoding (varint length prefix + elements) ---");
    let empty: Vec<u32> = vec![];
    let small = vec![1u32, 2, 3];
    let varied = vec![0u32, 127, 128, 255, 65535];
    for (label, v) in [
        ("[]", &empty),
        ("[1,2,3]", &small),
        ("[0,127,128,255,65535]", &varied),
    ] {
        let bytes = encode_to_vec(v).expect("encode Vec");
        println!(
            "  Vec<u32> {:>25} -> {} bytes: {}",
            label,
            bytes.len(),
            hex(&bytes)
        );
    }

    // -----------------------------------------------------------------------
    // 9. Struct encoding (fields concatenated, no field names on wire)
    // -----------------------------------------------------------------------
    println!("\n--- Struct encoding (fields concatenated, positional, no names) ---");
    let p = Point { x: 1, y: -1 };
    let bytes = encode_to_vec(&p).expect("encode Point");
    println!(
        "  Point {{ x: 1, y: -1 }} -> {} bytes: {}",
        bytes.len(),
        hex(&bytes)
    );
    println!("  (x=1 zigzag=0x02, y=-1 zigzag=0x01)");

    let nested = Nested {
        label: "origin".into(),
        point: Point { x: 0, y: 0 },
        active: true,
    };
    let bytes = encode_to_vec(&nested).expect("encode Nested");
    println!(
        "  Nested {{ label: \"origin\", point: (0,0), active: true }} -> {} bytes: {}",
        bytes.len(),
        hex(&bytes)
    );

    // -----------------------------------------------------------------------
    // 10. Enum encoding (varint discriminant + optional payload)
    // -----------------------------------------------------------------------
    println!("\n--- Enum encoding (varint discriminant + optional payload) ---");
    for color in [
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Custom(255, 128, 0),
    ] {
        let bytes = encode_to_vec(&color).expect("encode Color");
        println!(
            "  Color::{:?} -> {} bytes: {}",
            color,
            bytes.len(),
            hex(&bytes)
        );
    }

    println!("\n  With tag_type = \"u8\" (1-byte discriminant regardless of varint):");
    for variant in [SmallEnum::A, SmallEnum::B, SmallEnum::C(1000)] {
        let std_bytes = encode_to_vec(&variant).expect("encode SmallEnum std");
        let leg_bytes =
            encode_to_vec_with_config(&variant, config::legacy()).expect("encode SmallEnum legacy");
        println!(
            "  SmallEnum::{:?} -> std:{} bytes, legacy:{} bytes",
            variant,
            std_bytes.len(),
            leg_bytes.len()
        );
    }

    // -----------------------------------------------------------------------
    // 11. Encoded size consistency check
    // -----------------------------------------------------------------------
    println!("\n--- encoded_size consistency ---");
    println!("encoded_size() must equal encode_to_vec().len() for all types:");
    let check_point = Point { x: 100, y: -50 };
    let size = oxicode::encoded_size(&check_point).expect("encoded_size Point");
    let bytes = encode_to_vec(&check_point).expect("encode Point");
    assert_eq!(size, bytes.len(), "encoded_size mismatch for Point");
    println!("  Point size={}, actual={}, match=true", size, bytes.len());

    let check_color = Color::Custom(10, 20, 30);
    let size = oxicode::encoded_size(&check_color).expect("encoded_size Color");
    let bytes = encode_to_vec(&check_color).expect("encode Color");
    assert_eq!(size, bytes.len(), "encoded_size mismatch for Color");
    println!(
        "  Color::Custom size={}, actual={}, match=true",
        size,
        bytes.len()
    );

    // -----------------------------------------------------------------------
    // 12. Wire format round-trip verification
    // -----------------------------------------------------------------------
    println!("\n--- Round-trip verification ---");
    let original_point = Point { x: 42, y: -7 };
    let encoded = encode_to_vec(&original_point).expect("encode");
    let (decoded, bytes_read): (Point, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(original_point, decoded);
    assert_eq!(bytes_read, encoded.len());
    println!("  Point round-trip: OK ({} bytes consumed)", bytes_read);

    let original_nested = Nested {
        label: "test".into(),
        point: Point { x: 10, y: 20 },
        active: false,
    };
    let encoded = encode_to_vec(&original_nested).expect("encode Nested");
    let (decoded, _): (Nested, _) = oxicode::decode_from_slice(&encoded).expect("decode Nested");
    assert_eq!(original_nested, decoded);
    println!("  Nested round-trip: OK ({} bytes)", encoded.len());

    println!("\nAll wire format examples passed!");
    Ok(())
}
