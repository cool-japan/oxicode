#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RelationshipType {
    Friend,
    Follower,
    Blocked,
    Colleague,
    Family,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UserProfile {
    user_id: u64,
    username: String,
    display_name: String,
    bio: String,
    follower_count: u64,
    following_count: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Edge {
    from_id: u64,
    to_id: u64,
    relationship: RelationshipType,
    created_at: u64,
    weight: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Post {
    post_id: u64,
    author_id: u64,
    content: String,
    likes: u64,
    reposts: u64,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Graph {
    nodes: Vec<UserProfile>,
    edges: Vec<Edge>,
    name: String,
}

fn sample_user(id: u64) -> UserProfile {
    UserProfile {
        user_id: id,
        username: format!("user_{}", id),
        display_name: format!("User {}", id),
        bio: format!("Bio for user {}", id),
        follower_count: id * 10,
        following_count: id * 5,
    }
}

fn sample_edge(from: u64, to: u64, rel: RelationshipType) -> Edge {
    Edge {
        from_id: from,
        to_id: to,
        relationship: rel,
        created_at: 1_700_000_000 + from + to,
        weight: 1.0,
    }
}

#[test]
fn test_user_profile_roundtrip_standard() {
    let user = sample_user(1);
    let cfg = config::standard();
    let bytes = encode_to_vec(&user, cfg).expect("encode user profile");
    let (decoded, _): (UserProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode user profile");
    assert_eq!(user, decoded);
}

#[test]
fn test_user_profile_roundtrip_big_endian() {
    let user = sample_user(42);
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&user, cfg).expect("encode user big endian");
    let (decoded, _): (UserProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode user big endian");
    assert_eq!(user, decoded);
}

#[test]
fn test_user_profile_roundtrip_fixed_int() {
    let user = sample_user(99);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&user, cfg).expect("encode user fixed int");
    let (decoded, _): (UserProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode user fixed int");
    assert_eq!(user, decoded);
}

#[test]
fn test_relationship_friend() {
    let rel = RelationshipType::Friend;
    let cfg = config::standard();
    let bytes = encode_to_vec(&rel, cfg).expect("encode Friend");
    let (decoded, _): (RelationshipType, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Friend");
    assert_eq!(rel, decoded);
}

#[test]
fn test_relationship_follower() {
    let rel = RelationshipType::Follower;
    let cfg = config::standard();
    let bytes = encode_to_vec(&rel, cfg).expect("encode Follower");
    let (decoded, _): (RelationshipType, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Follower");
    assert_eq!(rel, decoded);
}

#[test]
fn test_relationship_blocked() {
    let rel = RelationshipType::Blocked;
    let cfg = config::standard();
    let bytes = encode_to_vec(&rel, cfg).expect("encode Blocked");
    let (decoded, _): (RelationshipType, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Blocked");
    assert_eq!(rel, decoded);
}

#[test]
fn test_relationship_colleague() {
    let rel = RelationshipType::Colleague;
    let cfg = config::standard();
    let bytes = encode_to_vec(&rel, cfg).expect("encode Colleague");
    let (decoded, _): (RelationshipType, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Colleague");
    assert_eq!(rel, decoded);
}

#[test]
fn test_relationship_family() {
    let rel = RelationshipType::Family;
    let cfg = config::standard();
    let bytes = encode_to_vec(&rel, cfg).expect("encode Family");
    let (decoded, _): (RelationshipType, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Family");
    assert_eq!(rel, decoded);
}

#[test]
fn test_edge_roundtrip_standard() {
    let edge = sample_edge(1, 2, RelationshipType::Friend);
    let cfg = config::standard();
    let bytes = encode_to_vec(&edge, cfg).expect("encode edge");
    let (decoded, _): (Edge, _) = decode_owned_from_slice(&bytes, cfg).expect("decode edge");
    assert_eq!(edge, decoded);
}

#[test]
fn test_edge_roundtrip_big_endian() {
    let edge = sample_edge(10, 20, RelationshipType::Colleague);
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&edge, cfg).expect("encode edge big endian");
    let (decoded, _): (Edge, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode edge big endian");
    assert_eq!(edge, decoded);
}

#[test]
fn test_edge_blocked_relationship() {
    let edge = sample_edge(5, 6, RelationshipType::Blocked);
    let cfg = config::standard();
    let bytes = encode_to_vec(&edge, cfg).expect("encode blocked edge");
    let (decoded, _): (Edge, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode blocked edge");
    assert_eq!(decoded.relationship, RelationshipType::Blocked);
}

#[test]
fn test_post_roundtrip_standard() {
    let post = Post {
        post_id: 1001,
        author_id: 42,
        content: "Hello, social network!".to_string(),
        likes: 255,
        reposts: 13,
        created_at: 1_700_100_000,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&post, cfg).expect("encode post");
    let (decoded, _): (Post, _) = decode_owned_from_slice(&bytes, cfg).expect("decode post");
    assert_eq!(post, decoded);
}

#[test]
fn test_post_roundtrip_fixed_int() {
    let post = Post {
        post_id: 9999,
        author_id: 1,
        content: "Fixed int encoding post content.".to_string(),
        likes: 1024,
        reposts: 0,
        created_at: 1_700_200_000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&post, cfg).expect("encode post fixed int");
    let (decoded, _): (Post, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode post fixed int");
    assert_eq!(post, decoded);
}

#[test]
fn test_empty_graph_roundtrip() {
    let graph = Graph {
        nodes: vec![],
        edges: vec![],
        name: "empty_graph".to_string(),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&graph, cfg).expect("encode empty graph");
    let (decoded, _): (Graph, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode empty graph");
    assert_eq!(graph, decoded);
    assert_eq!(decoded.nodes.len(), 0);
    assert_eq!(decoded.edges.len(), 0);
}

#[test]
fn test_graph_with_single_node() {
    let graph = Graph {
        nodes: vec![sample_user(7)],
        edges: vec![],
        name: "single_node_graph".to_string(),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&graph, cfg).expect("encode single node graph");
    let (decoded, _): (Graph, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode single node graph");
    assert_eq!(graph, decoded);
    assert_eq!(decoded.nodes.len(), 1);
}

#[test]
fn test_graph_with_multiple_nodes_and_edges() {
    let nodes: Vec<UserProfile> = (1..=5).map(sample_user).collect();
    let edges = vec![
        sample_edge(1, 2, RelationshipType::Friend),
        sample_edge(2, 3, RelationshipType::Follower),
        sample_edge(3, 4, RelationshipType::Colleague),
        sample_edge(4, 5, RelationshipType::Family),
    ];
    let graph = Graph {
        nodes,
        edges,
        name: "social_cluster".to_string(),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&graph, cfg).expect("encode multi-node graph");
    let (decoded, _): (Graph, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode multi-node graph");
    assert_eq!(graph, decoded);
    assert_eq!(decoded.edges.len(), 4);
}

#[test]
fn test_large_graph_roundtrip() {
    let nodes: Vec<UserProfile> = (1..=100).map(sample_user).collect();
    let edges: Vec<Edge> = (1u64..=99)
        .map(|i| sample_edge(i, i + 1, RelationshipType::Follower))
        .collect();
    let graph = Graph {
        nodes,
        edges,
        name: "large_follower_chain".to_string(),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&graph, cfg).expect("encode large graph");
    let (decoded, _): (Graph, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large graph");
    assert_eq!(decoded.nodes.len(), 100);
    assert_eq!(decoded.edges.len(), 99);
    assert_eq!(decoded.name, "large_follower_chain");
}

#[test]
fn test_graph_big_endian_fixed_int() {
    let nodes: Vec<UserProfile> = (1..=3).map(sample_user).collect();
    let edges = vec![
        sample_edge(1, 2, RelationshipType::Friend),
        sample_edge(2, 3, RelationshipType::Family),
    ];
    let graph = Graph {
        nodes,
        edges,
        name: "be_fixed_graph".to_string(),
    };
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec(&graph, cfg).expect("encode big endian fixed int graph");
    let (decoded, _): (Graph, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode big endian fixed int graph");
    assert_eq!(graph, decoded);
}

#[test]
fn test_bytes_consumed_matches_encoded_length() {
    let user = sample_user(3);
    let cfg = config::standard();
    let bytes = encode_to_vec(&user, cfg).expect("encode user for size check");
    let (_, consumed): (UserProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode user for size check");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_post_with_long_content() {
    let long_content: String = "a".repeat(10_000);
    let post = Post {
        post_id: 2000,
        author_id: 777,
        content: long_content.clone(),
        likes: 0,
        reposts: 0,
        created_at: 1_700_300_000,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&post, cfg).expect("encode long content post");
    let (decoded, _): (Post, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode long content post");
    assert_eq!(decoded.content.len(), 10_000);
    assert_eq!(post, decoded);
}

#[test]
fn test_multiple_relationship_types_in_edges() {
    let all_rels = vec![
        RelationshipType::Friend,
        RelationshipType::Follower,
        RelationshipType::Blocked,
        RelationshipType::Colleague,
        RelationshipType::Family,
    ];
    let cfg = config::standard();
    for rel in &all_rels {
        let bytes = encode_to_vec(rel, cfg).expect("encode relationship variant");
        let (decoded, _): (RelationshipType, _) =
            decode_owned_from_slice(&bytes, cfg).expect("decode relationship variant");
        assert_eq!(rel, &decoded);
    }
}

#[test]
fn test_vec_of_posts_roundtrip() {
    let posts: Vec<Post> = (1u64..=20)
        .map(|i| Post {
            post_id: i,
            author_id: i % 5 + 1,
            content: format!("Post content number {}", i),
            likes: i * 3,
            reposts: i,
            created_at: 1_700_000_000 + i,
        })
        .collect();
    let cfg = config::standard();
    let bytes = encode_to_vec(&posts, cfg).expect("encode vec of posts");
    let (decoded, _): (Vec<Post>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec of posts");
    assert_eq!(posts, decoded);
    assert_eq!(decoded.len(), 20);
}

#[test]
fn test_nested_graphs_as_vec() {
    let subgraphs: Vec<Graph> = (1u64..=3)
        .map(|g| {
            let nodes: Vec<UserProfile> = (1u64..=2).map(|n| sample_user(g * 10 + n)).collect();
            let edges = vec![sample_edge(
                g * 10 + 1,
                g * 10 + 2,
                RelationshipType::Friend,
            )];
            Graph {
                nodes,
                edges,
                name: format!("subgraph_{}", g),
            }
        })
        .collect();
    let cfg = config::standard();
    let bytes = encode_to_vec(&subgraphs, cfg).expect("encode vec of graphs");
    let (decoded, _): (Vec<Graph>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec of graphs");
    assert_eq!(subgraphs, decoded);
    assert_eq!(decoded.len(), 3);
}
