//! Advanced collection encoding tests: Graph/network data structures

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Encode, Decode)]
struct NodeData {
    label: String,
    weight: f64,
    tags: BTreeSet<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Graph {
    nodes: BTreeMap<u64, NodeData>,
    edges: Vec<(u64, u64, f64)>,
    metadata: HashMap<String, String>,
}

// Test 1: Empty BTreeMap<u64, NodeData> roundtrip
#[test]
fn test_empty_btreemap_node_data_roundtrip() {
    let original: BTreeMap<u64, NodeData> = BTreeMap::new();
    let bytes = encode_to_vec(&original).expect("Failed to encode empty BTreeMap<u64, NodeData>");
    let (decoded, _): (BTreeMap<u64, NodeData>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty BTreeMap<u64, NodeData>");
    assert_eq!(original, decoded);
}

// Test 2: BTreeMap<u64, NodeData> with 3 entries roundtrip
#[test]
fn test_btreemap_node_data_three_entries_roundtrip() {
    let mut original: BTreeMap<u64, NodeData> = BTreeMap::new();
    original.insert(
        1u64,
        NodeData {
            label: "alpha".to_string(),
            weight: 1.0,
            tags: {
                let mut s = BTreeSet::new();
                s.insert("tag_a".to_string());
                s
            },
        },
    );
    original.insert(
        2u64,
        NodeData {
            label: "beta".to_string(),
            weight: 2.5,
            tags: {
                let mut s = BTreeSet::new();
                s.insert("tag_b".to_string());
                s.insert("tag_c".to_string());
                s
            },
        },
    );
    original.insert(
        3u64,
        NodeData {
            label: "gamma".to_string(),
            weight: 0.75,
            tags: BTreeSet::new(),
        },
    );
    let bytes =
        encode_to_vec(&original).expect("Failed to encode BTreeMap<u64, NodeData> with 3 entries");
    let (decoded, _): (BTreeMap<u64, NodeData>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<u64, NodeData> with 3 entries");
    assert_eq!(original, decoded);
}

// Test 3: BTreeSet<String> roundtrip
#[test]
fn test_btreeset_string_roundtrip() {
    let mut original: BTreeSet<String> = BTreeSet::new();
    original.insert("apple".to_string());
    original.insert("banana".to_string());
    original.insert("cherry".to_string());
    let bytes = encode_to_vec(&original).expect("Failed to encode BTreeSet<String>");
    let (decoded, _): (BTreeSet<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet<String>");
    assert_eq!(original, decoded);
}

// Test 4: BTreeSet<u32> roundtrip
#[test]
fn test_btreeset_u32_roundtrip() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(10u32);
    original.insert(20u32);
    original.insert(5u32);
    original.insert(15u32);
    let bytes = encode_to_vec(&original).expect("Failed to encode BTreeSet<u32>");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet<u32>");
    assert_eq!(original, decoded);
}

// Test 5: NodeData with empty tags roundtrip
#[test]
fn test_node_data_empty_tags_roundtrip() {
    let original = NodeData {
        label: "solo_node".to_string(),
        weight: 3.14,
        tags: BTreeSet::new(),
    };
    let bytes = encode_to_vec(&original).expect("Failed to encode NodeData with empty tags");
    let (decoded, _): (NodeData, usize) =
        decode_from_slice(&bytes).expect("Failed to decode NodeData with empty tags");
    assert_eq!(original, decoded);
}

// Test 6: NodeData with multiple tags roundtrip
#[test]
fn test_node_data_multiple_tags_roundtrip() {
    let mut tags = BTreeSet::new();
    tags.insert("critical".to_string());
    tags.insert("infrastructure".to_string());
    tags.insert("network".to_string());
    tags.insert("router".to_string());
    let original = NodeData {
        label: "gateway_node".to_string(),
        weight: 9.99,
        tags,
    };
    let bytes = encode_to_vec(&original).expect("Failed to encode NodeData with multiple tags");
    let (decoded, _): (NodeData, usize) =
        decode_from_slice(&bytes).expect("Failed to decode NodeData with multiple tags");
    assert_eq!(original, decoded);
}

// Test 7: Graph empty roundtrip
#[test]
fn test_graph_empty_roundtrip() {
    let original = Graph {
        nodes: BTreeMap::new(),
        edges: Vec::new(),
        metadata: HashMap::new(),
    };
    let bytes = encode_to_vec(&original).expect("Failed to encode empty Graph");
    let (decoded, _): (Graph, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty Graph");
    assert_eq!(original, decoded);
}

// Test 8: Graph with 3 nodes and 2 edges roundtrip
#[test]
fn test_graph_three_nodes_two_edges_roundtrip() {
    let mut nodes = BTreeMap::new();
    nodes.insert(
        10u64,
        NodeData {
            label: "node_10".to_string(),
            weight: 1.0,
            tags: {
                let mut s = BTreeSet::new();
                s.insert("start".to_string());
                s
            },
        },
    );
    nodes.insert(
        20u64,
        NodeData {
            label: "node_20".to_string(),
            weight: 2.0,
            tags: BTreeSet::new(),
        },
    );
    nodes.insert(
        30u64,
        NodeData {
            label: "node_30".to_string(),
            weight: 3.0,
            tags: {
                let mut s = BTreeSet::new();
                s.insert("end".to_string());
                s
            },
        },
    );
    let edges = vec![(10u64, 20u64, 0.5f64), (20u64, 30u64, 1.5f64)];
    let original = Graph {
        nodes,
        edges,
        metadata: HashMap::new(),
    };
    let bytes = encode_to_vec(&original).expect("Failed to encode Graph with 3 nodes and 2 edges");
    let (decoded, _): (Graph, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Graph with 3 nodes and 2 edges");
    assert_eq!(original, decoded);
}

// Test 9: Graph with metadata HashMap roundtrip
#[test]
fn test_graph_with_metadata_roundtrip() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata.insert("created_by".to_string(), "oxicode_test".to_string());
    metadata.insert("description".to_string(), "test graph".to_string());
    let original = Graph {
        nodes: BTreeMap::new(),
        edges: Vec::new(),
        metadata,
    };
    let bytes = encode_to_vec(&original).expect("Failed to encode Graph with metadata");
    let (decoded, _): (Graph, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Graph with metadata");
    assert_eq!(original, decoded);
}

// Test 10: Vec<(u64, u64, f64)> (edges) roundtrip
#[test]
fn test_vec_edge_tuples_roundtrip() {
    let original: Vec<(u64, u64, f64)> = vec![
        (1u64, 2u64, 0.1f64),
        (2u64, 3u64, 0.2f64),
        (3u64, 4u64, 0.3f64),
        (1u64, 4u64, 0.9f64),
    ];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<(u64, u64, f64)>");
    let (decoded, _): (Vec<(u64, u64, f64)>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<(u64, u64, f64)>");
    assert_eq!(original, decoded);
}

// Test 11: VecDeque<u32> roundtrip
#[test]
fn test_vecdeque_u32_roundtrip() {
    let mut original: VecDeque<u32> = VecDeque::new();
    original.push_back(100u32);
    original.push_back(200u32);
    original.push_back(300u32);
    original.push_front(50u32);
    let bytes = encode_to_vec(&original).expect("Failed to encode VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode VecDeque<u32>");
    assert_eq!(original, decoded);
}

// Test 12: VecDeque<String> roundtrip
#[test]
fn test_vecdeque_string_roundtrip() {
    let mut original: VecDeque<String> = VecDeque::new();
    original.push_back("first".to_string());
    original.push_back("second".to_string());
    original.push_front("zeroth".to_string());
    original.push_back("third".to_string());
    let bytes = encode_to_vec(&original).expect("Failed to encode VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode VecDeque<String>");
    assert_eq!(original, decoded);
}

// Test 13: HashMap<String, String> roundtrip (use BTreeMap for determinism in assertions)
#[test]
fn test_hashmap_string_string_roundtrip() {
    let mut original: HashMap<String, String> = HashMap::new();
    original.insert("host".to_string(), "localhost".to_string());
    original.insert("port".to_string(), "8080".to_string());
    original.insert("protocol".to_string(), "tcp".to_string());
    let bytes = encode_to_vec(&original).expect("Failed to encode HashMap<String, String>");
    let (decoded, _): (HashMap<String, String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<String, String>");
    // Compare via BTreeMap for determinism
    let original_btree: BTreeMap<String, String> = original.into_iter().collect();
    let decoded_btree: BTreeMap<String, String> = decoded.into_iter().collect();
    assert_eq!(original_btree, decoded_btree);
}

// Test 14: HashSet<u32> roundtrip
#[test]
fn test_hashset_u32_roundtrip() {
    let mut original: HashSet<u32> = HashSet::new();
    original.insert(1u32);
    original.insert(2u32);
    original.insert(3u32);
    original.insert(5u32);
    original.insert(8u32);
    let bytes = encode_to_vec(&original).expect("Failed to encode HashSet<u32>");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashSet<u32>");
    assert_eq!(original, decoded);
}

// Test 15: Nested BTreeMap<String, Vec<u32>> roundtrip
#[test]
fn test_nested_btreemap_string_vec_u32_roundtrip() {
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert("layer0".to_string(), vec![1u32, 2u32, 3u32]);
    original.insert("layer1".to_string(), vec![10u32, 20u32]);
    original.insert("layer2".to_string(), vec![]);
    original.insert("layer3".to_string(), vec![100u32]);
    let bytes = encode_to_vec(&original).expect("Failed to encode BTreeMap<String, Vec<u32>>");
    let (decoded, _): (BTreeMap<String, Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<String, Vec<u32>>");
    assert_eq!(original, decoded);
}

// Test 16: Option<BTreeMap<u64, NodeData>> Some roundtrip
#[test]
fn test_option_btreemap_node_data_some_roundtrip() {
    let mut inner: BTreeMap<u64, NodeData> = BTreeMap::new();
    inner.insert(
        42u64,
        NodeData {
            label: "optional_node".to_string(),
            weight: 7.77,
            tags: {
                let mut s = BTreeSet::new();
                s.insert("optional".to_string());
                s
            },
        },
    );
    let original: Option<BTreeMap<u64, NodeData>> = Some(inner);
    let bytes =
        encode_to_vec(&original).expect("Failed to encode Option<BTreeMap<u64, NodeData>> Some");
    let (decoded, _): (Option<BTreeMap<u64, NodeData>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<BTreeMap<u64, NodeData>> Some");
    assert_eq!(original, decoded);
}

// Test 17: Option<BTreeMap<u64, NodeData>> None roundtrip
#[test]
fn test_option_btreemap_node_data_none_roundtrip() {
    let original: Option<BTreeMap<u64, NodeData>> = None;
    let bytes =
        encode_to_vec(&original).expect("Failed to encode Option<BTreeMap<u64, NodeData>> None");
    let (decoded, _): (Option<BTreeMap<u64, NodeData>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<BTreeMap<u64, NodeData>> None");
    assert_eq!(original, decoded);
}

// Test 18: Vec<Graph> with 2 graphs roundtrip
#[test]
fn test_vec_graph_two_elements_roundtrip() {
    let graph1 = Graph {
        nodes: {
            let mut m = BTreeMap::new();
            m.insert(
                1u64,
                NodeData {
                    label: "g1n1".to_string(),
                    weight: 1.1,
                    tags: BTreeSet::new(),
                },
            );
            m
        },
        edges: vec![(1u64, 1u64, 0.0f64)],
        metadata: HashMap::new(),
    };
    let graph2 = Graph {
        nodes: BTreeMap::new(),
        edges: Vec::new(),
        metadata: {
            let mut m = HashMap::new();
            m.insert("graph".to_string(), "second".to_string());
            m
        },
    };
    let original: Vec<Graph> = vec![graph1, graph2];
    let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Graph>");
    let (decoded, _): (Vec<Graph>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Graph>");
    assert_eq!(original, decoded);
}

// Test 19: Consumed bytes == encoded length for Graph
#[test]
fn test_consumed_bytes_equals_encoded_length_for_graph() {
    let graph = Graph {
        nodes: {
            let mut m = BTreeMap::new();
            m.insert(
                99u64,
                NodeData {
                    label: "measure_node".to_string(),
                    weight: 2.718,
                    tags: {
                        let mut s = BTreeSet::new();
                        s.insert("euler".to_string());
                        s
                    },
                },
            );
            m
        },
        edges: vec![(99u64, 99u64, 1.0f64)],
        metadata: HashMap::new(),
    };
    let bytes = encode_to_vec(&graph).expect("Failed to encode Graph for byte-length check");
    let (_decoded, consumed): (Graph, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Graph for byte-length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "Consumed bytes must equal encoded length"
    );
}

// Test 20: Big-endian config Graph roundtrip
#[test]
fn test_big_endian_config_graph_roundtrip() {
    let graph = Graph {
        nodes: {
            let mut m = BTreeMap::new();
            m.insert(
                7u64,
                NodeData {
                    label: "be_node".to_string(),
                    weight: 0.5,
                    tags: {
                        let mut s = BTreeSet::new();
                        s.insert("big_endian".to_string());
                        s
                    },
                },
            );
            m
        },
        edges: vec![(7u64, 7u64, 0.25f64)],
        metadata: {
            let mut m = HashMap::new();
            m.insert("encoding".to_string(), "big_endian".to_string());
            m
        },
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&graph, cfg)
        .expect("Failed to encode Graph with big-endian config");
    let (decoded, _): (Graph, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("Failed to decode Graph with big-endian config");
    assert_eq!(graph, decoded);
}

// Test 21: Fixed-int config NodeData roundtrip
#[test]
fn test_fixed_int_config_node_data_roundtrip() {
    let node = NodeData {
        label: "fixed_int_node".to_string(),
        weight: 1.23456789,
        tags: {
            let mut s = BTreeSet::new();
            s.insert("fixed".to_string());
            s.insert("integer".to_string());
            s
        },
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&node, cfg)
        .expect("Failed to encode NodeData with fixed-int config");
    let (decoded, _): (NodeData, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("Failed to decode NodeData with fixed-int config");
    assert_eq!(node, decoded);
}

// Test 22: Encoding determinism for Graph (BTreeMap guarantees stable order)
#[test]
fn test_graph_encoding_determinism() {
    let make_graph = || {
        let mut nodes = BTreeMap::new();
        nodes.insert(
            3u64,
            NodeData {
                label: "node_c".to_string(),
                weight: 3.0,
                tags: {
                    let mut s = BTreeSet::new();
                    s.insert("c_tag".to_string());
                    s
                },
            },
        );
        nodes.insert(
            1u64,
            NodeData {
                label: "node_a".to_string(),
                weight: 1.0,
                tags: {
                    let mut s = BTreeSet::new();
                    s.insert("a_tag".to_string());
                    s
                },
            },
        );
        nodes.insert(
            2u64,
            NodeData {
                label: "node_b".to_string(),
                weight: 2.0,
                tags: BTreeSet::new(),
            },
        );
        Graph {
            nodes,
            edges: vec![(1u64, 2u64, 0.5f64), (2u64, 3u64, 1.5f64)],
            metadata: HashMap::new(),
        }
    };
    let graph_a = make_graph();
    let graph_b = make_graph();
    let bytes_a = encode_to_vec(&graph_a).expect("Failed to encode Graph (determinism, first)");
    let bytes_b = encode_to_vec(&graph_b).expect("Failed to encode Graph (determinism, second)");
    assert_eq!(
        bytes_a, bytes_b,
        "BTreeMap-based Graph encoding must be deterministic across calls"
    );
}
