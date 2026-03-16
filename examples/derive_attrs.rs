//! Demonstrates all derive macro attributes in OxiCode
//!
//! Run with: cargo run --example derive_attrs

use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Custom encode/decode functions for the `with` attribute
// ---------------------------------------------------------------------------

mod duration_secs {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };
    use std::time::Duration;

    pub fn encode<E: Encoder>(val: &Duration, encoder: &mut E) -> Result<(), Error> {
        val.as_secs().encode(encoder)
    }

    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Duration, Error> {
        Ok(Duration::from_secs(u64::decode(decoder)?))
    }
}

// ---------------------------------------------------------------------------
// rename_all = "camelCase" — attribute is parsed and accepted; wire format is
// positional (binary), so renaming affects only documentation/schema tooling.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct Config {
    server_host: String,  // logically "serverHost"
    max_connections: u32, // logically "maxConnections"
}

// ---------------------------------------------------------------------------
// Enum with custom tag type (u8 saves space vs default usize)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Status {
    Active,
    Inactive,
    Pending(String),
}

// ---------------------------------------------------------------------------
// Struct with skip + default_value
// ---------------------------------------------------------------------------

fn default_version() -> u32 {
    1
}

#[derive(Debug, Encode, Decode)]
struct Request {
    endpoint: String,
    /// Not in the binary stream; restored to `default_version()` on decode.
    #[oxicode(default = "default_version")]
    version: u32,
    #[oxicode(with = "duration_secs")]
    timeout: std::time::Duration,
}

// ---------------------------------------------------------------------------
// Transparent newtype — encodes identically to the inner type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct UserId(u64);

// ---------------------------------------------------------------------------
// Compact vec lengths with seq_len
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tags {
    #[oxicode(seq_len = "u8")] // at most 255 tags
    items: Vec<String>,
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Derive Attributes Example\n");

    // --- rename_all ---
    println!("1. rename_all = \"camelCase\":");
    let cfg = Config {
        server_host: "localhost".into(),
        max_connections: 100,
    };
    let bytes = oxicode::encode_to_vec(&cfg).expect("encode Config");
    let (decoded, _): (Config, _) = oxicode::decode_from_slice(&bytes).expect("decode Config");
    assert_eq!(cfg, decoded);
    println!("   Config encoded to {} bytes, round-trip OK", bytes.len());

    // --- tag_type ---
    println!("\n2. tag_type = \"u8\" (enum discriminant width):");
    let variants = [
        Status::Active,
        Status::Inactive,
        Status::Pending("waiting for approval".into()),
    ];
    for status in &variants {
        let bytes = oxicode::encode_to_vec(status).expect("encode Status");
        let (dec, _): (Status, _) = oxicode::decode_from_slice(&bytes).expect("decode Status");
        assert_eq!(status, &dec);
        println!("   {:?} -> {} bytes, round-trip OK", status, bytes.len());
    }

    // Compare tag sizes: u8 vs default (varint usize)
    let u8_bytes = oxicode::encode_to_vec_with_config(&Status::Active, oxicode::config::legacy())
        .expect("encode u8 tag legacy");
    println!(
        "   Active with u8 tag (legacy/fixed): {} byte(s) for discriminant region",
        u8_bytes.len()
    );

    // --- skip + default ---
    println!("\n3. skip + default (field excluded from wire format):");
    let req = Request {
        endpoint: "/api/health".into(),
        version: 42, // will be skipped during encode
        timeout: std::time::Duration::from_secs(30),
    };
    let bytes = oxicode::encode_to_vec(&req).expect("encode Request");
    let (dec, _): (Request, _) = oxicode::decode_from_slice(&bytes).expect("decode Request");
    println!(
        "   Request encoded to {} bytes (version field skipped)",
        bytes.len()
    );
    assert_eq!(dec.version, 1, "skipped field must use default value");
    assert_eq!(dec.endpoint, req.endpoint);
    println!(
        "   Decoded version = {} (default_version()), endpoint = {}",
        dec.version, dec.endpoint
    );

    // --- with (custom encode/decode) ---
    println!("\n4. with = \"duration_secs\" (custom encode/decode):");
    let dur = std::time::Duration::from_secs(3600);
    let req2 = Request {
        endpoint: "/api/long-op".into(),
        version: 0,
        timeout: dur,
    };
    let bytes2 = oxicode::encode_to_vec(&req2).expect("encode Request2");
    let (dec2, _): (Request, _) = oxicode::decode_from_slice(&bytes2).expect("decode Request2");
    assert_eq!(dec2.timeout, dur);
    println!(
        "   Duration {:?} round-tripped correctly via duration_secs module",
        dur
    );

    // --- transparent ---
    println!("\n5. transparent (newtype encodes as inner type):");
    let uid = UserId(99_999);
    let uid_bytes = oxicode::encode_to_vec(&uid).expect("encode UserId");
    let raw_bytes = oxicode::encode_to_vec(&99_999u64).expect("encode u64");
    assert_eq!(
        uid_bytes, raw_bytes,
        "transparent newtype must produce identical bytes"
    );
    let (dec_uid, _): (UserId, _) = oxicode::decode_from_slice(&uid_bytes).expect("decode UserId");
    assert_eq!(uid, dec_uid);
    println!(
        "   UserId(99999) == raw u64(99999) encoding: {} bytes, identical: true",
        uid_bytes.len()
    );

    // --- seq_len ---
    println!("\n6. seq_len = \"u8\" (compact sequence length prefix):");
    let tags = Tags {
        items: vec!["rust".into(), "binary".into(), "fast".into()],
    };
    let tags_bytes = oxicode::encode_to_vec(&tags).expect("encode Tags");
    let (dec_tags, _): (Tags, _) = oxicode::decode_from_slice(&tags_bytes).expect("decode Tags");
    assert_eq!(tags, dec_tags);

    // Compare seq_len = u8 vs default (varint usize for length prefix)
    #[derive(Encode, Decode)]
    struct TagsDefault {
        items: Vec<String>,
    }
    let tags_default = TagsDefault {
        items: vec!["rust".into(), "binary".into(), "fast".into()],
    };
    let default_bytes = oxicode::encode_to_vec(&tags_default).expect("encode TagsDefault");
    println!(
        "   Tags with seq_len=u8: {} bytes, default (varint usize): {} bytes",
        tags_bytes.len(),
        default_bytes.len()
    );
    println!("   Round-trip OK for {} items", dec_tags.items.len());

    println!("\nAll derive attribute examples passed!");
    Ok(())
}
