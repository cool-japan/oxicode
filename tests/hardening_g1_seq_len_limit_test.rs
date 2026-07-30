//! WP-G1 hardening: the derive `seq_len` decode path must claim the container against the
//! configured decode memory limit before allocating, so an attacker-controlled length cannot
//! drive an unbounded pre-allocation / OOM (finding derive-audit#0, TODO line 419).

#![cfg(all(feature = "alloc", feature = "derive"))]
use oxicode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct WithSeq {
    #[oxicode(seq_len = "u64")]
    data: Vec<u8>,
}

#[test]
fn seq_len_roundtrips_within_limit() {
    let value = WithSeq {
        data: vec![1u8; 100],
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (WithSeq, usize) =
        oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(decoded, value);
}

#[test]
fn seq_len_huge_length_is_rejected_by_limit_not_allocated() {
    // Craft a payload whose `seq_len = "u64"` length prefix claims 2^40 elements but supplies
    // no element bytes. Before the fix this drove `Vec::with_capacity(1 << 40)` (a ~1 TiB
    // reservation → abort). With the container claim in place, the configured byte limit rejects
    // it up front with a graceful error and no large allocation is attempted.
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_limit::<1024>();

    let bytes = (1u64 << 40).to_le_bytes().to_vec();
    let result: Result<(WithSeq, usize), _> = oxicode::decode_from_slice_with_config(&bytes, cfg);
    assert!(
        result.is_err(),
        "an oversized seq_len must be rejected by the decode limit"
    );
}

#[test]
fn seq_len_container_of_large_elements_is_claimed() {
    // With `Vec<u64>` and a per-element size of 8, a length just past the limit / 8 must be
    // rejected: the claim uses claim_container_read::<u64> so the element size is accounted for.
    #[derive(Encode, Decode, Debug)]
    struct BigElems {
        #[oxicode(seq_len = "u64")]
        data: Vec<u64>,
    }
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_limit::<64>();
    // 100 elements * 8 bytes = 800 bytes > 64-byte limit.
    let bytes = 100u64.to_le_bytes().to_vec();
    let result: Result<(BigElems, usize), _> = oxicode::decode_from_slice_with_config(&bytes, cfg);
    assert!(
        result.is_err(),
        "container claim must account for element size"
    );
}
