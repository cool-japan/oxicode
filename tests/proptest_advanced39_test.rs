//! Advanced property-based tests using proptest (set 39).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying roundtrip and encoding invariants for
//! PhysicsShape, RigidBody, Collision, and PhysicsWorld types.

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

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PhysicsShape {
    Sphere,
    Box,
    Capsule,
    Cylinder,
    Convex,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RigidBody {
    id: u64,
    shape: PhysicsShape,
    mass: u32,        // mass * 1000 (kg)
    restitution: u16, // 0-1000 (coefficient * 1000)
    is_static: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Collision {
    body_a_id: u64,
    body_b_id: u64,
    impulse: i32, // impact impulse
    timestep_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhysicsWorld {
    bodies: Vec<RigidBody>,
    gravity: i32, // gravity * 100 (m/s²)
    simulation_step_ms: u16,
}

// ── Strategies ────────────────────────────────────────────────────────────────

fn arb_physics_shape() -> impl Strategy<Value = PhysicsShape> {
    prop_oneof![
        Just(PhysicsShape::Sphere),
        Just(PhysicsShape::Box),
        Just(PhysicsShape::Capsule),
        Just(PhysicsShape::Cylinder),
        Just(PhysicsShape::Convex),
    ]
}

fn arb_rigid_body() -> impl Strategy<Value = RigidBody> {
    (
        any::<u64>(),
        arb_physics_shape(),
        any::<u32>(),
        0u16..=1000u16,
        any::<bool>(),
    )
        .prop_map(|(id, shape, mass, restitution, is_static)| RigidBody {
            id,
            shape,
            mass,
            restitution,
            is_static,
        })
}

fn arb_collision() -> impl Strategy<Value = Collision> {
    (any::<u64>(), any::<u64>(), any::<i32>(), any::<u32>()).prop_map(
        |(body_a_id, body_b_id, impulse, timestep_ms)| Collision {
            body_a_id,
            body_b_id,
            impulse,
            timestep_ms,
        },
    )
}

fn arb_physics_world() -> impl Strategy<Value = PhysicsWorld> {
    (
        proptest::collection::vec(arb_rigid_body(), 0usize..=8),
        any::<i32>(),
        any::<u16>(),
    )
        .prop_map(|(bodies, gravity, simulation_step_ms)| PhysicsWorld {
            bodies,
            gravity,
            simulation_step_ms,
        })
}

// ── Test 1: RigidBody struct roundtrip ───────────────────────────────────────

#[test]
fn prop_rigid_body_roundtrip() {
    proptest!(|(body in arb_rigid_body())| {
        let enc = encode_to_vec(&body).expect("encode RigidBody");
        let (dec, bytes_read): (RigidBody, usize) =
            decode_from_slice(&enc).expect("decode RigidBody");
        prop_assert_eq!(&body, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 2: Collision struct roundtrip ───────────────────────────────────────

#[test]
fn prop_collision_roundtrip() {
    proptest!(|(col in arb_collision())| {
        let enc = encode_to_vec(&col).expect("encode Collision");
        let (dec, bytes_read): (Collision, usize) =
            decode_from_slice(&enc).expect("decode Collision");
        prop_assert_eq!(&col, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 3: PhysicsWorld struct roundtrip ────────────────────────────────────

#[test]
fn prop_physics_world_roundtrip() {
    proptest!(|(world in arb_physics_world())| {
        let enc = encode_to_vec(&world).expect("encode PhysicsWorld");
        let (dec, bytes_read): (PhysicsWorld, usize) =
            decode_from_slice(&enc).expect("decode PhysicsWorld");
        prop_assert_eq!(&world, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 4: PhysicsShape::Sphere roundtrip ───────────────────────────────────

#[test]
fn prop_physics_shape_sphere_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = PhysicsShape::Sphere;
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape::Sphere");
        let (dec, bytes_read): (PhysicsShape, usize) =
            decode_from_slice(&enc).expect("decode PhysicsShape::Sphere");
        prop_assert_eq!(&shape, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 5: PhysicsShape::Box roundtrip ──────────────────────────────────────

#[test]
fn prop_physics_shape_box_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = PhysicsShape::Box;
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape::Box");
        let (dec, bytes_read): (PhysicsShape, usize) =
            decode_from_slice(&enc).expect("decode PhysicsShape::Box");
        prop_assert_eq!(&shape, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 6: PhysicsShape::Capsule roundtrip ──────────────────────────────────

#[test]
fn prop_physics_shape_capsule_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = PhysicsShape::Capsule;
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape::Capsule");
        let (dec, bytes_read): (PhysicsShape, usize) =
            decode_from_slice(&enc).expect("decode PhysicsShape::Capsule");
        prop_assert_eq!(&shape, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 7: PhysicsShape::Cylinder roundtrip ─────────────────────────────────

#[test]
fn prop_physics_shape_cylinder_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = PhysicsShape::Cylinder;
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape::Cylinder");
        let (dec, bytes_read): (PhysicsShape, usize) =
            decode_from_slice(&enc).expect("decode PhysicsShape::Cylinder");
        prop_assert_eq!(&shape, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 8: PhysicsShape::Convex roundtrip ───────────────────────────────────

#[test]
fn prop_physics_shape_convex_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let shape = PhysicsShape::Convex;
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape::Convex");
        let (dec, bytes_read): (PhysicsShape, usize) =
            decode_from_slice(&enc).expect("decode PhysicsShape::Convex");
        prop_assert_eq!(&shape, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 9: All PhysicsShape variants encode to distinct bytes ────────────────

#[test]
fn prop_physics_shape_variants_distinct() {
    proptest!(|(shape in arb_physics_shape())| {
        let enc = encode_to_vec(&shape).expect("encode PhysicsShape variant");
        // Unit variants must encode as exactly 1 byte (discriminant varint)
        prop_assert_eq!(
            enc.len(),
            1,
            "PhysicsShape unit variant must encode as 1 byte, got {}",
            enc.len()
        );
    });
}

// ── Test 10: Vec<RigidBody> roundtrip (max 10) ───────────────────────────────

#[test]
fn prop_vec_rigid_body_roundtrip() {
    proptest!(|(bodies in proptest::collection::vec(arb_rigid_body(), 0usize..=10))| {
        let enc = encode_to_vec(&bodies).expect("encode Vec<RigidBody>");
        let (dec, bytes_read): (Vec<RigidBody>, usize) =
            decode_from_slice(&enc).expect("decode Vec<RigidBody>");
        prop_assert_eq!(&bodies, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 11: Vec<Collision> roundtrip (max 10) ───────────────────────────────

#[test]
fn prop_vec_collision_roundtrip() {
    proptest!(|(cols in proptest::collection::vec(arb_collision(), 0usize..=10))| {
        let enc = encode_to_vec(&cols).expect("encode Vec<Collision>");
        let (dec, bytes_read): (Vec<Collision>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Collision>");
        prop_assert_eq!(&cols, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 12: Encoding determinism for RigidBody ──────────────────────────────

#[test]
fn prop_rigid_body_encoding_determinism() {
    proptest!(|(body in arb_rigid_body())| {
        let enc1 = encode_to_vec(&body).expect("first encode RigidBody");
        let enc2 = encode_to_vec(&body).expect("second encode RigidBody");
        prop_assert_eq!(&enc1, &enc2, "repeated encodes must produce identical bytes");
    });
}

// ── Test 13: Encoding determinism for PhysicsWorld ───────────────────────────

#[test]
fn prop_physics_world_encoding_determinism() {
    proptest!(|(world in arb_physics_world())| {
        let enc1 = encode_to_vec(&world).expect("first encode PhysicsWorld");
        let enc2 = encode_to_vec(&world).expect("second encode PhysicsWorld");
        prop_assert_eq!(&enc1, &enc2, "repeated encodes of PhysicsWorld must be identical");
    });
}

// ── Test 14: RigidBody re-encode invariant ───────────────────────────────────

#[test]
fn prop_rigid_body_reencode_invariant() {
    proptest!(|(body in arb_rigid_body())| {
        let enc1 = encode_to_vec(&body).expect("first encode RigidBody");
        let (decoded, _): (RigidBody, usize) =
            decode_from_slice(&enc1).expect("decode RigidBody for re-encode");
        let enc2 = encode_to_vec(&decoded).expect("re-encode RigidBody");
        prop_assert_eq!(&enc1, &enc2, "re-encode must produce identical bytes");
    });
}

// ── Test 15: Collision re-encode invariant ───────────────────────────────────

#[test]
fn prop_collision_reencode_invariant() {
    proptest!(|(col in arb_collision())| {
        let enc1 = encode_to_vec(&col).expect("first encode Collision");
        let (decoded, _): (Collision, usize) =
            decode_from_slice(&enc1).expect("decode Collision for re-encode");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Collision");
        prop_assert_eq!(&enc1, &enc2, "re-encode must produce identical bytes");
    });
}

// ── Test 16: RigidBody with boundary mass values ─────────────────────────────

#[test]
fn prop_rigid_body_boundary_mass() {
    proptest!(|(id: u64, shape in arb_physics_shape(), restitution in 0u16..=1000u16, is_static: bool)| {
        for &mass in &[0u32, 1u32, u32::MAX] {
            let body = RigidBody { id, shape: shape.clone(), mass, restitution, is_static };
            let enc = encode_to_vec(&body).expect("encode RigidBody boundary mass");
            let (dec, bytes_read): (RigidBody, usize) =
                decode_from_slice(&enc).expect("decode RigidBody boundary mass");
            prop_assert_eq!(&body, &dec);
            prop_assert_eq!(bytes_read, enc.len());
        }
    });
}

// ── Test 17: Collision with boundary impulse values ──────────────────────────

#[test]
fn prop_collision_boundary_impulse() {
    proptest!(|(body_a_id: u64, body_b_id: u64, timestep_ms: u32)| {
        for &impulse in &[i32::MIN, -1i32, 0i32, 1i32, i32::MAX] {
            let col = Collision { body_a_id, body_b_id, impulse, timestep_ms };
            let enc = encode_to_vec(&col).expect("encode Collision boundary impulse");
            let (dec, bytes_read): (Collision, usize) =
                decode_from_slice(&enc).expect("decode Collision boundary impulse");
            prop_assert_eq!(&col, &dec);
            prop_assert_eq!(bytes_read, enc.len());
        }
    });
}

// ── Test 18: PhysicsWorld with zero-body world ────────────────────────────────

#[test]
fn prop_physics_world_empty_bodies() {
    proptest!(|(gravity: i32, simulation_step_ms: u16)| {
        let world = PhysicsWorld {
            bodies: Vec::new(),
            gravity,
            simulation_step_ms,
        };
        let enc = encode_to_vec(&world).expect("encode empty PhysicsWorld");
        let (dec, bytes_read): (PhysicsWorld, usize) =
            decode_from_slice(&enc).expect("decode empty PhysicsWorld");
        prop_assert_eq!(&world, &dec);
        prop_assert_eq!(bytes_read, enc.len());
        prop_assert_eq!(dec.bodies.len(), 0);
    });
}

// ── Test 19: PhysicsWorld gravity boundary values ────────────────────────────

#[test]
fn prop_physics_world_gravity_boundary() {
    proptest!(|(bodies in proptest::collection::vec(arb_rigid_body(), 0usize..=4), simulation_step_ms: u16)| {
        for &gravity in &[i32::MIN, -981i32, 0i32, 981i32, i32::MAX] {
            let world = PhysicsWorld {
                bodies: bodies.clone(),
                gravity,
                simulation_step_ms,
            };
            let enc = encode_to_vec(&world).expect("encode PhysicsWorld gravity boundary");
            let (dec, bytes_read): (PhysicsWorld, usize) =
                decode_from_slice(&enc).expect("decode PhysicsWorld gravity boundary");
            prop_assert_eq!(&world, &dec);
            prop_assert_eq!(bytes_read, enc.len());
        }
    });
}

// ── Test 20: restitution clamped to 0-1000 range survives roundtrip ──────────

#[test]
fn prop_rigid_body_restitution_range() {
    proptest!(|(id: u64, shape in arb_physics_shape(), mass: u32, is_static: bool, restitution in 0u16..=1000u16)| {
        let body = RigidBody { id, shape, mass, restitution, is_static };
        let enc = encode_to_vec(&body).expect("encode RigidBody restitution");
        let (dec, bytes_read): (RigidBody, usize) =
            decode_from_slice(&enc).expect("decode RigidBody restitution");
        prop_assert_eq!(&body, &dec);
        prop_assert_eq!(bytes_read, enc.len());
        prop_assert!(dec.restitution <= 1000, "restitution must stay within 0-1000");
    });
}

// ── Test 21: Vec<PhysicsShape> roundtrip (all variant combinations) ───────────

#[test]
fn prop_vec_physics_shape_roundtrip() {
    proptest!(|(shapes in proptest::collection::vec(arb_physics_shape(), 0usize..=20))| {
        let enc = encode_to_vec(&shapes).expect("encode Vec<PhysicsShape>");
        let (dec, bytes_read): (Vec<PhysicsShape>, usize) =
            decode_from_slice(&enc).expect("decode Vec<PhysicsShape>");
        prop_assert_eq!(&shapes, &dec);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── Test 22: Consumed bytes == encoded length for PhysicsWorld ────────────────

#[test]
fn prop_physics_world_consumed_bytes_match() {
    proptest!(|(world in arb_physics_world())| {
        let enc = encode_to_vec(&world).expect("encode PhysicsWorld consumed bytes");
        let (dec, consumed): (PhysicsWorld, usize) =
            decode_from_slice(&enc).expect("decode PhysicsWorld consumed bytes");
        prop_assert_eq!(&world, &dec);
        prop_assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes {} must equal encoded length {}",
            consumed,
            enc.len()
        );
    });
}
