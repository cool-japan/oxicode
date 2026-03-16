#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkNode {
    node_id: u32,
    address: [u8; 4],
    port: u16,
    weight: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NodeRole {
    Leader,
    Follower,
    Candidate { term: u32 },
    Observer,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClusterConfig {
    cluster_id: u64,
    nodes: Vec<NetworkNode>,
    quorum: u8,
}

fn arb_network_node() -> impl Strategy<Value = NetworkNode> {
    (
        any::<u32>(),
        [any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>()],
        any::<u16>(),
        any::<u8>(),
    )
        .prop_map(|(node_id, address, port, weight)| NetworkNode {
            node_id,
            address,
            port,
            weight,
        })
}

fn arb_node_role() -> impl Strategy<Value = NodeRole> {
    prop_oneof![
        Just(NodeRole::Leader),
        Just(NodeRole::Follower),
        any::<u32>().prop_map(|term| NodeRole::Candidate { term }),
        Just(NodeRole::Observer),
    ]
}

fn arb_cluster_config() -> impl Strategy<Value = ClusterConfig> {
    (
        any::<u64>(),
        prop::collection::vec(arb_network_node(), 0..8),
        any::<u8>(),
    )
        .prop_map(|(cluster_id, nodes, quorum)| ClusterConfig {
            cluster_id,
            nodes,
            quorum,
        })
}

#[test]
fn test_network_node_roundtrip() {
    let rt = proptest::test_runner::TestRunner::default();
    let _ = rt;
    proptest!(|(node in arb_network_node())| {
        let encoded = encode_to_vec(&node).expect("encode NetworkNode");
        let (decoded, _): (NetworkNode, usize) =
            decode_from_slice(&encoded).expect("decode NetworkNode");
        prop_assert_eq!(node, decoded);
    });
}

#[test]
fn test_node_role_leader_roundtrip() {
    let role = NodeRole::Leader;
    let encoded = encode_to_vec(&role).expect("encode Leader");
    let (decoded, _): (NodeRole, usize) = decode_from_slice(&encoded).expect("decode Leader");
    assert_eq!(role, decoded);
}

#[test]
fn test_node_role_follower_roundtrip() {
    let role = NodeRole::Follower;
    let encoded = encode_to_vec(&role).expect("encode Follower");
    let (decoded, _): (NodeRole, usize) = decode_from_slice(&encoded).expect("decode Follower");
    assert_eq!(role, decoded);
}

#[test]
fn test_node_role_candidate_roundtrip() {
    proptest!(|(term in any::<u32>())| {
        let role = NodeRole::Candidate { term };
        let encoded = encode_to_vec(&role).expect("encode Candidate");
        let (decoded, _): (NodeRole, usize) = decode_from_slice(&encoded).expect("decode Candidate");
        prop_assert_eq!(role, decoded);
    });
}

#[test]
fn test_node_role_observer_roundtrip() {
    let role = NodeRole::Observer;
    let encoded = encode_to_vec(&role).expect("encode Observer");
    let (decoded, _): (NodeRole, usize) = decode_from_slice(&encoded).expect("decode Observer");
    assert_eq!(role, decoded);
}

#[test]
fn test_cluster_config_roundtrip() {
    proptest!(|(cfg in arb_cluster_config())| {
        let encoded = encode_to_vec(&cfg).expect("encode ClusterConfig");
        let (decoded, _): (ClusterConfig, usize) =
            decode_from_slice(&encoded).expect("decode ClusterConfig");
        prop_assert_eq!(cfg, decoded);
    });
}

#[test]
fn test_ipv4_array_roundtrip() {
    proptest!(|(addr in [any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>()])| {
        let encoded = encode_to_vec(&addr).expect("encode [u8;4]");
        let (decoded, _): ([u8; 4], usize) = decode_from_slice(&encoded).expect("decode [u8;4]");
        prop_assert_eq!(addr, decoded);
    });
}

#[test]
fn test_u16_roundtrip() {
    proptest!(|(v in any::<u16>())| {
        let encoded = encode_to_vec(&v).expect("encode u16");
        let (decoded, _): (u16, usize) = decode_from_slice(&encoded).expect("decode u16");
        prop_assert_eq!(v, decoded);
    });
}

#[test]
fn test_u32_roundtrip() {
    proptest!(|(v in any::<u32>())| {
        let encoded = encode_to_vec(&v).expect("encode u32");
        let (decoded, _): (u32, usize) = decode_from_slice(&encoded).expect("decode u32");
        prop_assert_eq!(v, decoded);
    });
}

#[test]
fn test_u64_roundtrip() {
    proptest!(|(v in any::<u64>())| {
        let encoded = encode_to_vec(&v).expect("encode u64");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("decode u64");
        prop_assert_eq!(v, decoded);
    });
}

#[test]
fn test_u8_roundtrip() {
    proptest!(|(v in any::<u8>())| {
        let encoded = encode_to_vec(&v).expect("encode u8");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("decode u8");
        prop_assert_eq!(v, decoded);
    });
}

#[test]
fn test_consumed_bytes_equals_encoded_length_network_node() {
    proptest!(|(node in arb_network_node())| {
        let encoded = encode_to_vec(&node).expect("encode NetworkNode for length check");
        let (_, consumed): (NetworkNode, usize) =
            decode_from_slice(&encoded).expect("decode NetworkNode for length check");
        prop_assert_eq!(consumed, encoded.len());
    });
}

#[test]
fn test_consumed_bytes_equals_encoded_length_cluster_config() {
    proptest!(|(cfg in arb_cluster_config())| {
        let encoded = encode_to_vec(&cfg).expect("encode ClusterConfig for length check");
        let (_, consumed): (ClusterConfig, usize) =
            decode_from_slice(&encoded).expect("decode ClusterConfig for length check");
        prop_assert_eq!(consumed, encoded.len());
    });
}

#[test]
fn test_encoding_determinism_network_node() {
    proptest!(|(node in arb_network_node())| {
        let enc1 = encode_to_vec(&node).expect("encode NetworkNode first");
        let enc2 = encode_to_vec(&node).expect("encode NetworkNode second");
        prop_assert_eq!(enc1, enc2);
    });
}

#[test]
fn test_encoding_determinism_node_role() {
    proptest!(|(role in arb_node_role())| {
        let enc1 = encode_to_vec(&role).expect("encode NodeRole first");
        let enc2 = encode_to_vec(&role).expect("encode NodeRole second");
        prop_assert_eq!(enc1, enc2);
    });
}

#[test]
fn test_vec_network_node_roundtrip() {
    proptest!(|(nodes in prop::collection::vec(arb_network_node(), 0..16))| {
        let encoded = encode_to_vec(&nodes).expect("encode Vec<NetworkNode>");
        let (decoded, _): (Vec<NetworkNode>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<NetworkNode>");
        prop_assert_eq!(nodes, decoded);
    });
}

#[test]
fn test_option_cluster_config_some_roundtrip() {
    proptest!(|(cfg in arb_cluster_config())| {
        let opt: Option<ClusterConfig> = Some(cfg);
        let encoded = encode_to_vec(&opt).expect("encode Option<ClusterConfig> Some");
        let (decoded, _): (Option<ClusterConfig>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ClusterConfig> Some");
        prop_assert_eq!(opt, decoded);
    });
}

#[test]
fn test_option_cluster_config_none_roundtrip() {
    let opt: Option<ClusterConfig> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<ClusterConfig> None");
    let (decoded, _): (Option<ClusterConfig>, usize) =
        decode_from_slice(&encoded).expect("decode Option<ClusterConfig> None");
    assert_eq!(opt, decoded);
}

#[test]
fn test_vec_node_role_roundtrip() {
    proptest!(|(roles in prop::collection::vec(arb_node_role(), 0..16))| {
        let encoded = encode_to_vec(&roles).expect("encode Vec<NodeRole>");
        let (decoded, _): (Vec<NodeRole>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<NodeRole>");
        prop_assert_eq!(roles, decoded);
    });
}

#[test]
fn test_nested_cluster_config_in_vec_roundtrip() {
    proptest!(|(cfgs in prop::collection::vec(arb_cluster_config(), 0..4))| {
        let encoded = encode_to_vec(&cfgs).expect("encode Vec<ClusterConfig>");
        let (decoded, _): (Vec<ClusterConfig>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ClusterConfig>");
        prop_assert_eq!(cfgs, decoded);
    });
}

#[test]
fn test_node_with_zero_address_roundtrip() {
    let node = NetworkNode {
        node_id: 0,
        address: [0u8; 4],
        port: 0,
        weight: 0,
    };
    let encoded = encode_to_vec(&node).expect("encode zero NetworkNode");
    let (decoded, consumed): (NetworkNode, usize) =
        decode_from_slice(&encoded).expect("decode zero NetworkNode");
    assert_eq!(node, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_node_with_max_values_roundtrip() {
    let node = NetworkNode {
        node_id: u32::MAX,
        address: [255u8; 4],
        port: u16::MAX,
        weight: u8::MAX,
    };
    let encoded = encode_to_vec(&node).expect("encode max NetworkNode");
    let (decoded, consumed): (NetworkNode, usize) =
        decode_from_slice(&encoded).expect("decode max NetworkNode");
    assert_eq!(node, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cluster_config_empty_nodes_roundtrip() {
    proptest!(|(cluster_id in any::<u64>(), quorum in any::<u8>())| {
        let cfg = ClusterConfig {
            cluster_id,
            nodes: vec![],
            quorum,
        };
        let encoded = encode_to_vec(&cfg).expect("encode empty ClusterConfig");
        let (decoded, consumed): (ClusterConfig, usize) =
            decode_from_slice(&encoded).expect("decode empty ClusterConfig");
        prop_assert_eq!(cfg, decoded);
        prop_assert_eq!(consumed, encoded.len());
    });
}
