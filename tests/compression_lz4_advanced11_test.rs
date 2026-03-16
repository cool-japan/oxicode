//! Advanced LZ4 compression tests with a 3D graphics / game engine domain theme.
//!
//! Exercises OxiCode's LZ4 compression API over a rich set of game-engine data
//! structures: vertices, meshes, scene nodes, transforms, and more.

#![cfg(feature = "compression-lz4")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum PrimitiveType {
    Triangle,
    Quad,
    Point,
    Line,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: (f32, f32),
    color: [u8; 4],
}

impl Vertex {
    fn new(position: Vec3, normal: Vec3, uv: (f32, f32), color: [u8; 4]) -> Self {
        Self {
            position,
            normal,
            uv,
            color,
        }
    }

    fn default_white() -> Self {
        Self::new(
            Vec3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            (0.0, 0.0),
            [255, 255, 255, 255],
        )
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Mesh {
    name: String,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    primitive: PrimitiveType,
}

impl Mesh {
    fn empty(name: &str, primitive: PrimitiveType) -> Self {
        Self {
            name: name.to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            primitive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Transform {
    fn identity() -> Self {
        Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }

    fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SceneNode {
    id: u32,
    name: String,
    transform: Transform,
    mesh: Option<Mesh>,
}

impl SceneNode {
    fn new(id: u32, name: &str, transform: Transform, mesh: Option<Mesh>) -> Self {
        Self {
            id,
            name: name.to_string(),
            transform,
            mesh,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_triangle_mesh(name: &str, vertex_count: usize) -> Mesh {
    let vertices: Vec<Vertex> = (0..vertex_count)
        .map(|i| {
            let fi = i as f32;
            Vertex::new(
                Vec3::new(fi, fi * 2.0, fi * 3.0),
                Vec3::new(0.0, 1.0, 0.0),
                (fi / vertex_count as f32, fi / vertex_count as f32),
                [255, 128, 64, 255],
            )
        })
        .collect();
    let indices: Vec<u32> = (0..vertex_count as u32).collect();
    Mesh {
        name: name.to_string(),
        vertices,
        indices,
        primitive: PrimitiveType::Triangle,
    }
}

fn make_repetitive_mesh(name: &str, vertex_count: usize) -> Mesh {
    // All vertices identical — highly compressible.
    let vertex = Vertex::default_white();
    let vertices = vec![vertex; vertex_count];
    let indices: Vec<u32> = (0..vertex_count as u32)
        .flat_map(|i| {
            if i + 2 < vertex_count as u32 {
                vec![i, i + 1, i + 2]
            } else {
                vec![]
            }
        })
        .collect();
    Mesh {
        name: name.to_string(),
        vertices,
        indices,
        primitive: PrimitiveType::Triangle,
    }
}

fn roundtrip_compress<T>(value: &T) -> T
where
    T: Encode + for<'de> Decode<()>,
    T: PartialEq + std::fmt::Debug,
{
    let encoded = encode_to_vec(value).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (T, usize) =
        decode_from_slice(&decompressed).expect("decode_from_slice failed");
    decoded
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

#[test]
fn test_vec3_zero_roundtrip() {
    let v = Vec3::zero();
    let recovered = roundtrip_compress(&v);
    assert_eq!(v, recovered);
}

#[test]
fn test_vec3_arbitrary_values_roundtrip() {
    let v = Vec3::new(1.23456789, -9.87654321, std::f32::consts::PI);
    let recovered = roundtrip_compress(&v);
    assert_eq!(v, recovered);
}

#[test]
fn test_vertex_default_white_roundtrip() {
    let vtx = Vertex::default_white();
    let recovered = roundtrip_compress(&vtx);
    assert_eq!(vtx, recovered);
}

#[test]
fn test_vertex_full_fields_roundtrip() {
    let vtx = Vertex::new(
        Vec3::new(10.5, -3.14, 0.001),
        Vec3::new(0.0, 0.0, 1.0),
        (0.75, 0.25),
        [100, 150, 200, 255],
    );
    let recovered = roundtrip_compress(&vtx);
    assert_eq!(vtx, recovered);
}

#[test]
fn test_mesh_triangle_small_roundtrip() {
    let mesh = make_triangle_mesh("small_tri", 30);
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_large_500_vertices_roundtrip() {
    let mesh = make_triangle_mesh("large_mesh", 500);
    assert_eq!(mesh.vertices.len(), 500);
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_large_1000_vertices_roundtrip() {
    let mesh = make_triangle_mesh("huge_mesh", 1000);
    assert_eq!(mesh.vertices.len(), 1000);
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_empty_roundtrip() {
    let mesh = Mesh::empty("empty", PrimitiveType::Triangle);
    assert!(mesh.vertices.is_empty());
    assert!(mesh.indices.is_empty());
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_primitive_quad_roundtrip() {
    let vertices = vec![
        Vertex::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            (0.0, 0.0),
            [255, 0, 0, 255],
        ),
        Vertex::new(
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            (1.0, 0.0),
            [0, 255, 0, 255],
        ),
        Vertex::new(
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            (1.0, 1.0),
            [0, 0, 255, 255],
        ),
        Vertex::new(
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            (0.0, 1.0),
            [255, 255, 0, 255],
        ),
    ];
    let mesh = Mesh {
        name: "unit_quad".to_string(),
        vertices,
        indices: vec![0, 1, 2, 0, 2, 3],
        primitive: PrimitiveType::Quad,
    };
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_primitive_point_cloud_roundtrip() {
    let vertices: Vec<Vertex> = (0..50)
        .map(|i| {
            let fi = i as f32 * 0.1;
            Vertex::new(
                Vec3::new(fi, fi, fi),
                Vec3::zero(),
                (0.0, 0.0),
                [255, 255, 255, 255],
            )
        })
        .collect();
    let mesh = Mesh {
        name: "point_cloud".to_string(),
        indices: (0..50u32).collect(),
        vertices,
        primitive: PrimitiveType::Point,
    };
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_mesh_primitive_line_strip_roundtrip() {
    let vertices: Vec<Vertex> = (0..20)
        .map(|i| {
            let fi = i as f32;
            Vertex::new(
                Vec3::new(fi * 0.5, (fi * 0.5).sin(), 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                (fi / 20.0, 0.0),
                [200, 100, 50, 255],
            )
        })
        .collect();
    let mesh = Mesh {
        name: "line_strip".to_string(),
        indices: (0..20u32).collect(),
        vertices,
        primitive: PrimitiveType::Line,
    };
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh, recovered);
}

#[test]
fn test_transform_identity_roundtrip() {
    let t = Transform::identity();
    let recovered = roundtrip_compress(&t);
    assert_eq!(t, recovered);
}

#[test]
fn test_transform_arbitrary_roundtrip() {
    let t = Transform::new(
        Vec3::new(100.0, -50.0, 25.5),
        Vec3::new(0.0, std::f32::consts::FRAC_PI_4, 0.0),
        Vec3::new(2.0, 2.0, 2.0),
    );
    let recovered = roundtrip_compress(&t);
    assert_eq!(t, recovered);
}

#[test]
fn test_scene_node_with_mesh_roundtrip() {
    let mesh = make_triangle_mesh("hero_mesh", 60);
    let node = SceneNode::new(
        42,
        "hero",
        Transform::new(Vec3::new(0.0, 1.8, 0.0), Vec3::zero(), Vec3::one()),
        Some(mesh),
    );
    let recovered = roundtrip_compress(&node);
    assert_eq!(node, recovered);
}

#[test]
fn test_scene_node_without_mesh_roundtrip() {
    let node = SceneNode::new(99, "empty_pivot", Transform::identity(), None);
    assert!(node.mesh.is_none());
    let recovered = roundtrip_compress(&node);
    assert_eq!(node, recovered);
}

#[test]
fn test_scene_node_vec_roundtrip() {
    let nodes: Vec<SceneNode> = (0..20)
        .map(|i| {
            let mesh = if i % 3 == 0 {
                Some(make_triangle_mesh(&format!("mesh_{i}"), 12))
            } else {
                None
            };
            SceneNode::new(
                i,
                &format!("node_{i}"),
                Transform::new(Vec3::new(i as f32, 0.0, 0.0), Vec3::zero(), Vec3::one()),
                mesh,
            )
        })
        .collect();
    let recovered = roundtrip_compress(&nodes);
    assert_eq!(nodes, recovered);
}

#[test]
fn test_repetitive_mesh_compresses_smaller_than_encoded() {
    // Identical vertices should compress very well with LZ4.
    let mesh = make_repetitive_mesh("flat_plane", 600);
    let encoded = encode_to_vec(&mesh).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive geometry",
        compressed.len(),
        encoded.len(),
    );
}

#[test]
fn test_large_mesh_compresses_smaller_than_encoded() {
    // Sequential positions produce runs that LZ4 can exploit.
    let mesh = make_repetitive_mesh("lod0_terrain", 800);
    let encoded = encode_to_vec(&mesh).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be < encoded ({} bytes)",
        compressed.len(),
        encoded.len(),
    );
}

#[test]
fn test_all_primitive_types_roundtrip() {
    for prim in [
        PrimitiveType::Triangle,
        PrimitiveType::Quad,
        PrimitiveType::Point,
        PrimitiveType::Line,
    ] {
        let mesh = Mesh {
            name: format!("prim_{prim:?}"),
            vertices: vec![Vertex::default_white()],
            indices: vec![0],
            primitive: prim.clone(),
        };
        let recovered = roundtrip_compress(&mesh);
        assert_eq!(mesh, recovered, "roundtrip failed for primitive {prim:?}");
    }
}

#[test]
fn test_corruption_detection_truncated_payload() {
    let mesh = make_triangle_mesh("truncated_mesh", 40);
    let encoded = encode_to_vec(&mesh).expect("encode_to_vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // Truncate to 60 % of original length — should produce a decompression error.
    let truncated = &compressed[..compressed.len() * 3 / 5];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress should fail on truncated compressed data"
    );
}

#[test]
fn test_corruption_detection_random_bytes() {
    // A buffer with no valid OxiCode/LZ4 header must be rejected.
    let garbage: Vec<u8> = (0u8..=63).cycle().take(128).collect();
    let result = decompress(&garbage);
    assert!(
        result.is_err(),
        "decompress should fail on random bytes with no valid header"
    );
}

#[test]
fn test_mesh_uv_coordinates_preserved() {
    // Verify that floating-point UV coordinates survive the full encode→compress
    // →decompress→decode pipeline without bit-level corruption.
    let uvs = [(0.0f32, 0.0f32), (1.0, 0.0), (0.5, 1.0), (0.25, 0.75)];
    let vertices: Vec<Vertex> = uvs
        .iter()
        .map(|&uv| Vertex::new(Vec3::zero(), Vec3::new(0.0, 1.0, 0.0), uv, [255; 4]))
        .collect();
    let mesh = Mesh {
        name: "uv_test_quad".to_string(),
        vertices,
        indices: vec![0, 1, 2, 0, 2, 3],
        primitive: PrimitiveType::Triangle,
    };
    let recovered = roundtrip_compress(&mesh);
    assert_eq!(mesh.vertices.len(), recovered.vertices.len());
    for (orig, rec) in mesh.vertices.iter().zip(recovered.vertices.iter()) {
        assert_eq!(orig.uv, rec.uv, "UV coordinate mismatch after roundtrip");
    }
    assert_eq!(mesh, recovered);
}

#[test]
fn test_scene_node_color_channels_preserved() {
    // Verify all four RGBA channels survive the compression roundtrip intact.
    let color_samples: [[u8; 4]; 5] = [
        [255, 0, 0, 255],
        [0, 255, 0, 128],
        [0, 0, 255, 64],
        [255, 255, 255, 255],
        [0, 0, 0, 0],
    ];
    let vertices: Vec<Vertex> = color_samples
        .iter()
        .map(|&c| Vertex::new(Vec3::zero(), Vec3::new(0.0, 1.0, 0.0), (0.0, 0.0), c))
        .collect();
    let mesh = Mesh {
        name: "color_test".to_string(),
        indices: (0..5u32).collect(),
        vertices,
        primitive: PrimitiveType::Point,
    };
    let node = SceneNode::new(7, "color_node", Transform::identity(), Some(mesh.clone()));
    let recovered = roundtrip_compress(&node);
    let rec_mesh = recovered.mesh.expect("recovered mesh should be Some");
    for (orig_vtx, rec_vtx) in mesh.vertices.iter().zip(rec_mesh.vertices.iter()) {
        assert_eq!(
            orig_vtx.color, rec_vtx.color,
            "color channel mismatch after roundtrip"
        );
    }
}
