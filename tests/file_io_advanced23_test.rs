//! Advanced file I/O tests — VR/AR gaming / virtual environment domain

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ObjectType {
    StaticMesh,
    DynamicRigidBody,
    Trigger,
    Light,
    Camera,
    ParticleSystem,
    Ui3D,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RenderLayer {
    Background,
    World,
    Player,
    Ui,
    Debug,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Vec3Fixed {
    x_um: i64,
    y_um: i64,
    z_um: i64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QuatFixed {
    w_micro: i32,
    x_micro: i32,
    y_micro: i32,
    z_micro: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SceneObject {
    object_id: u64,
    name: String,
    obj_type: ObjectType,
    layer: RenderLayer,
    position: Vec3Fixed,
    rotation: QuatFixed,
    scale_micro: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlayerState {
    player_id: u64,
    position: Vec3Fixed,
    rotation: QuatFixed,
    health: u16,
    score: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VrScene {
    scene_id: u64,
    name: String,
    objects: Vec<SceneObject>,
    players: Vec<PlayerState>,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn identity_quat() -> QuatFixed {
    QuatFixed {
        w_micro: 1_000_000,
        x_micro: 0,
        y_micro: 0,
        z_micro: 0,
    }
}

fn origin() -> Vec3Fixed {
    Vec3Fixed {
        x_um: 0,
        y_um: 0,
        z_um: 0,
    }
}

fn make_object(id: u64, obj_type: ObjectType, layer: RenderLayer) -> SceneObject {
    SceneObject {
        object_id: id,
        name: format!("object_{id}"),
        obj_type,
        layer,
        position: Vec3Fixed {
            x_um: id as i64 * 1_000,
            y_um: 0,
            z_um: 0,
        },
        rotation: identity_quat(),
        scale_micro: 1_000_000,
    }
}

fn make_player(id: u64) -> PlayerState {
    PlayerState {
        player_id: id,
        position: Vec3Fixed {
            x_um: id as i64 * 500_000,
            y_um: 0,
            z_um: 0,
        },
        rotation: identity_quat(),
        health: 100,
        score: id * 1_000,
    }
}

// ── Test 1: ObjectType::StaticMesh to file ────────────────────────────────────

#[test]
fn test_object_type_static_mesh_file() {
    let path = std::env::temp_dir().join("vr_obj_static_mesh.bin");
    let val = ObjectType::StaticMesh;
    encode_to_file(&val, &path).expect("encode ObjectType::StaticMesh");
    let decoded: ObjectType = decode_from_file(&path).expect("decode ObjectType::StaticMesh");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: ObjectType::DynamicRigidBody to file ──────────────────────────────

#[test]
fn test_object_type_dynamic_rigid_body_file() {
    let path = std::env::temp_dir().join("vr_obj_dynamic_rigid_body.bin");
    let val = ObjectType::DynamicRigidBody;
    encode_to_file(&val, &path).expect("encode ObjectType::DynamicRigidBody");
    let decoded: ObjectType = decode_from_file(&path).expect("decode ObjectType::DynamicRigidBody");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 3: ObjectType::Trigger to file ──────────────────────────────────────

#[test]
fn test_object_type_trigger_file() {
    let path = std::env::temp_dir().join("vr_obj_trigger.bin");
    let val = ObjectType::Trigger;
    encode_to_file(&val, &path).expect("encode ObjectType::Trigger");
    let decoded: ObjectType = decode_from_file(&path).expect("decode ObjectType::Trigger");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: ObjectType::Light to file ────────────────────────────────────────

#[test]
fn test_object_type_light_file() {
    let path = std::env::temp_dir().join("vr_obj_light.bin");
    let val = ObjectType::Light;
    encode_to_file(&val, &path).expect("encode ObjectType::Light");
    let decoded: ObjectType = decode_from_file(&path).expect("decode ObjectType::Light");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 5: ObjectType::Camera to file ───────────────────────────────────────

#[test]
fn test_object_type_camera_file() {
    let path = std::env::temp_dir().join("vr_obj_camera.bin");
    let val = ObjectType::Camera;
    encode_to_file(&val, &path).expect("encode ObjectType::Camera");
    let decoded: ObjectType = decode_from_file(&path).expect("decode ObjectType::Camera");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: RenderLayer variants to file (all five) ──────────────────────────

#[test]
fn test_render_layer_variants_file() {
    let layers = [
        (RenderLayer::Background, "vr_rl_background.bin"),
        (RenderLayer::World, "vr_rl_world.bin"),
        (RenderLayer::Player, "vr_rl_player.bin"),
        (RenderLayer::Ui, "vr_rl_ui.bin"),
        (RenderLayer::Debug, "vr_rl_debug.bin"),
    ];
    for (layer, filename) in layers {
        let path = std::env::temp_dir().join(filename);
        encode_to_file(&layer, &path).expect("encode RenderLayer variant");
        let decoded: RenderLayer = decode_from_file(&path).expect("decode RenderLayer variant");
        assert_eq!(layer, decoded);
        std::fs::remove_file(&path).ok();
    }
}

// ── Test 7: Vec3Fixed file roundtrip ─────────────────────────────────────────

#[test]
fn test_vec3fixed_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_vec3fixed.bin");
    let val = Vec3Fixed {
        x_um: 1_234_567_890,
        y_um: -987_654_321,
        z_um: 42_000_000,
    };
    encode_to_file(&val, &path).expect("encode Vec3Fixed");
    let decoded: Vec3Fixed = decode_from_file(&path).expect("decode Vec3Fixed");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: QuatFixed file roundtrip ─────────────────────────────────────────

#[test]
fn test_quatfixed_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_quatfixed.bin");
    // 45-degree rotation around Y axis (approx): w=cos(π/8), y=sin(π/8) scaled to micro
    let val = QuatFixed {
        w_micro: 923_880,
        x_micro: 0,
        y_micro: 382_683,
        z_micro: 0,
    };
    encode_to_file(&val, &path).expect("encode QuatFixed");
    let decoded: QuatFixed = decode_from_file(&path).expect("decode QuatFixed");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 9: SceneObject file roundtrip ───────────────────────────────────────

#[test]
fn test_scene_object_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_scene_object.bin");
    let obj = SceneObject {
        object_id: 99_001,
        name: "portal_gate".to_string(),
        obj_type: ObjectType::Trigger,
        layer: RenderLayer::World,
        position: Vec3Fixed {
            x_um: 5_000_000,
            y_um: 0,
            z_um: -3_000_000,
        },
        rotation: identity_quat(),
        scale_micro: 2_000_000,
    };
    encode_to_file(&obj, &path).expect("encode SceneObject");
    let decoded: SceneObject = decode_from_file(&path).expect("decode SceneObject");
    assert_eq!(obj, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: PlayerState file roundtrip ──────────────────────────────────────

#[test]
fn test_player_state_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_player_state.bin");
    let player = PlayerState {
        player_id: 7,
        position: Vec3Fixed {
            x_um: 1_000_000,
            y_um: 1_750_000,
            z_um: 500_000,
        },
        rotation: identity_quat(),
        health: 85,
        score: 142_500,
    };
    encode_to_file(&player, &path).expect("encode PlayerState");
    let decoded: PlayerState = decode_from_file(&path).expect("decode PlayerState");
    assert_eq!(player, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 11: VrScene with empty objects and players ───────────────────────────

#[test]
fn test_vr_scene_empty_file() {
    let path = std::env::temp_dir().join("vr_scene_empty.bin");
    let scene = VrScene {
        scene_id: 1,
        name: "empty_lobby".to_string(),
        objects: Vec::new(),
        players: Vec::new(),
    };
    encode_to_file(&scene, &path).expect("encode empty VrScene");
    let decoded: VrScene = decode_from_file(&path).expect("decode empty VrScene");
    assert_eq!(scene, decoded);
    assert!(decoded.objects.is_empty());
    assert!(decoded.players.is_empty());
    std::fs::remove_file(&path).ok();
}

// ── Test 12: VrScene with 5 objects and 2 players ────────────────────────────

#[test]
fn test_vr_scene_5_objects_2_players_file() {
    let path = std::env::temp_dir().join("vr_scene_5obj_2pl.bin");
    let objects: Vec<SceneObject> = (0..5)
        .map(|i| make_object(i, ObjectType::StaticMesh, RenderLayer::World))
        .collect();
    let players = vec![make_player(1), make_player(2)];
    let scene = VrScene {
        scene_id: 42,
        name: "arena_alpha".to_string(),
        objects,
        players,
    };
    encode_to_file(&scene, &path).expect("encode VrScene 5+2");
    let decoded: VrScene = decode_from_file(&path).expect("decode VrScene 5+2");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.objects.len(), 5);
    assert_eq!(decoded.players.len(), 2);
    std::fs::remove_file(&path).ok();
}

// ── Test 13: Large scene with 30 objects ─────────────────────────────────────

#[test]
fn test_vr_scene_30_objects_file() {
    let path = std::env::temp_dir().join("vr_scene_30obj.bin");
    let objects: Vec<SceneObject> = (0..30)
        .map(|i| make_object(i, ObjectType::StaticMesh, RenderLayer::World))
        .collect();
    let scene = VrScene {
        scene_id: 100,
        name: "massive_outdoor_zone".to_string(),
        objects,
        players: Vec::new(),
    };
    encode_to_file(&scene, &path).expect("encode large VrScene");
    let decoded: VrScene = decode_from_file(&path).expect("decode large VrScene");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.objects.len(), 30);
    std::fs::remove_file(&path).ok();
}

// ── Test 14: Overwrite file test ──────────────────────────────────────────────

#[test]
fn test_overwrite_scene_file() {
    let path = std::env::temp_dir().join("vr_scene_overwrite.bin");

    let first_scene = VrScene {
        scene_id: 1,
        name: "scene_v1".to_string(),
        objects: vec![make_object(0, ObjectType::Light, RenderLayer::World)],
        players: Vec::new(),
    };
    encode_to_file(&first_scene, &path).expect("encode first scene");

    let second_scene = VrScene {
        scene_id: 2,
        name: "scene_v2".to_string(),
        objects: vec![
            make_object(10, ObjectType::Camera, RenderLayer::Player),
            make_object(11, ObjectType::Trigger, RenderLayer::Ui),
        ],
        players: vec![make_player(99)],
    };
    encode_to_file(&second_scene, &path).expect("encode second scene (overwrite)");

    let decoded: VrScene = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second_scene, decoded);
    assert_eq!(decoded.scene_id, 2);
    std::fs::remove_file(&path).ok();
}

// ── Test 15: Vec<SceneObject> file roundtrip ─────────────────────────────────

#[test]
fn test_vec_scene_objects_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_vec_scene_objects.bin");
    let objects: Vec<SceneObject> = vec![
        make_object(1, ObjectType::StaticMesh, RenderLayer::Background),
        make_object(2, ObjectType::Light, RenderLayer::World),
        make_object(3, ObjectType::Camera, RenderLayer::Player),
    ];
    encode_to_file(&objects, &path).expect("encode Vec<SceneObject>");
    let decoded: Vec<SceneObject> = decode_from_file(&path).expect("decode Vec<SceneObject>");
    assert_eq!(objects, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Vec<PlayerState> file roundtrip ─────────────────────────────────

#[test]
fn test_vec_player_states_file_roundtrip() {
    let path = std::env::temp_dir().join("vr_vec_players.bin");
    let players: Vec<PlayerState> = (1..=4).map(make_player).collect();
    encode_to_file(&players, &path).expect("encode Vec<PlayerState>");
    let decoded: Vec<PlayerState> = decode_from_file(&path).expect("decode Vec<PlayerState>");
    assert_eq!(players, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 17: Player at origin ─────────────────────────────────────────────────

#[test]
fn test_player_at_origin_file() {
    let path = std::env::temp_dir().join("vr_player_origin.bin");
    let player = PlayerState {
        player_id: 0,
        position: origin(),
        rotation: identity_quat(),
        health: 100,
        score: 0,
    };
    encode_to_file(&player, &path).expect("encode player at origin");
    let decoded: PlayerState = decode_from_file(&path).expect("decode player at origin");
    assert_eq!(player, decoded);
    assert_eq!(decoded.position.x_um, 0);
    assert_eq!(decoded.position.y_um, 0);
    assert_eq!(decoded.position.z_um, 0);
    std::fs::remove_file(&path).ok();
}

// ── Test 18: Player at max coordinates ───────────────────────────────────────

#[test]
fn test_player_at_max_coordinates_file() {
    let path = std::env::temp_dir().join("vr_player_max_coords.bin");
    let player = PlayerState {
        player_id: u64::MAX,
        position: Vec3Fixed {
            x_um: i64::MAX,
            y_um: i64::MAX,
            z_um: i64::MAX,
        },
        rotation: QuatFixed {
            w_micro: i32::MAX,
            x_micro: i32::MAX,
            y_micro: i32::MAX,
            z_micro: i32::MAX,
        },
        health: u16::MAX,
        score: u64::MAX,
    };
    encode_to_file(&player, &path).expect("encode player at max coords");
    let decoded: PlayerState = decode_from_file(&path).expect("decode player at max coords");
    assert_eq!(player, decoded);
    assert_eq!(decoded.health, u16::MAX);
    assert_eq!(decoded.score, u64::MAX);
    std::fs::remove_file(&path).ok();
}

// ── Test 19: Scene with all object types ─────────────────────────────────────

#[test]
fn test_scene_with_all_object_types_file() {
    let path = std::env::temp_dir().join("vr_scene_all_obj_types.bin");
    let objects = vec![
        make_object(1, ObjectType::StaticMesh, RenderLayer::World),
        make_object(2, ObjectType::DynamicRigidBody, RenderLayer::World),
        make_object(3, ObjectType::Trigger, RenderLayer::World),
        make_object(4, ObjectType::Light, RenderLayer::World),
        make_object(5, ObjectType::Camera, RenderLayer::Player),
        make_object(6, ObjectType::ParticleSystem, RenderLayer::World),
        make_object(7, ObjectType::Ui3D, RenderLayer::Ui),
    ];
    let scene = VrScene {
        scene_id: 77,
        name: "showcase_scene".to_string(),
        objects,
        players: Vec::new(),
    };
    encode_to_file(&scene, &path).expect("encode scene all object types");
    let decoded: VrScene = decode_from_file(&path).expect("decode scene all object types");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.objects.len(), 7);
    assert_eq!(decoded.objects[5].obj_type, ObjectType::ParticleSystem);
    assert_eq!(decoded.objects[6].obj_type, ObjectType::Ui3D);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Score accumulation across multiple writes ────────────────────────

#[test]
fn test_score_accumulation_file() {
    let path = std::env::temp_dir().join("vr_score_accumulation.bin");
    let mut player = PlayerState {
        player_id: 5,
        position: origin(),
        rotation: identity_quat(),
        health: 100,
        score: 0,
    };

    for round in 1u64..=5 {
        player.score += round * 250;
        encode_to_file(&player, &path).expect("encode score round");
    }

    let decoded: PlayerState = decode_from_file(&path).expect("decode final score");
    // sum of 250*(1+2+3+4+5) = 250*15 = 3750
    assert_eq!(decoded.score, 3_750);
    std::fs::remove_file(&path).ok();
}

// ── Test 21: Health boundary — 0 and max ─────────────────────────────────────

#[test]
fn test_health_boundary_file() {
    let path_dead = std::env::temp_dir().join("vr_health_zero.bin");
    let path_full = std::env::temp_dir().join("vr_health_max.bin");

    let dead_player = PlayerState {
        player_id: 1,
        position: origin(),
        rotation: identity_quat(),
        health: 0,
        score: 50_000,
    };
    let full_player = PlayerState {
        player_id: 2,
        position: origin(),
        rotation: identity_quat(),
        health: u16::MAX,
        score: 0,
    };

    encode_to_file(&dead_player, &path_dead).expect("encode dead player");
    encode_to_file(&full_player, &path_full).expect("encode full health player");

    let decoded_dead: PlayerState = decode_from_file(&path_dead).expect("decode dead player");
    let decoded_full: PlayerState =
        decode_from_file(&path_full).expect("decode full health player");

    assert_eq!(decoded_dead.health, 0);
    assert_eq!(decoded_full.health, u16::MAX);
    std::fs::remove_file(&path_dead).ok();
    std::fs::remove_file(&path_full).ok();
}

// ── Test 22: Scene name with unicode characters ───────────────────────────────

#[test]
fn test_scene_name_unicode_file() {
    let path = std::env::temp_dir().join("vr_scene_unicode.bin");
    let scene = VrScene {
        scene_id: 9999,
        name: "仮想現実ステージ — Виртуальный мир 🌐".to_string(),
        objects: Vec::new(),
        players: Vec::new(),
    };
    encode_to_file(&scene, &path).expect("encode unicode scene name");
    let decoded: VrScene = decode_from_file(&path).expect("decode unicode scene name");
    assert_eq!(scene, decoded);
    assert_eq!(decoded.name, "仮想現実ステージ — Виртуальный мир 🌐");
    std::fs::remove_file(&path).ok();
}

// ── Test 23 (bonus — brings count to required 22 after merging test 6) ────────
// Spatial anchor at fixed position — encode/decode via vec then file

#[test]
fn test_spatial_anchor_fixed_position_file() {
    let path = std::env::temp_dir().join("vr_spatial_anchor.bin");
    // A spatial anchor is represented as a static trigger with a known world position
    let anchor = SceneObject {
        object_id: 0xDEAD_BEEF,
        name: "anchor::spawn_point_A".to_string(),
        obj_type: ObjectType::Trigger,
        layer: RenderLayer::World,
        position: Vec3Fixed {
            x_um: 10_000_000,
            y_um: 1_600_000,
            z_um: -5_000_000,
        },
        rotation: identity_quat(),
        scale_micro: 500_000,
    };
    // Verify bytes match between encode_to_vec and encode_to_file
    let vec_bytes = encode_to_vec(&anchor).expect("encode_to_vec spatial anchor");
    encode_to_file(&anchor, &path).expect("encode_to_file spatial anchor");
    let file_bytes = std::fs::read(&path).expect("read spatial anchor file");
    assert_eq!(vec_bytes, file_bytes);
    let (from_slice, _): (SceneObject, _) =
        decode_from_slice(&file_bytes).expect("decode_from_slice spatial anchor");
    assert_eq!(anchor, from_slice);
    let from_file: SceneObject = decode_from_file(&path).expect("decode_from_file spatial anchor");
    assert_eq!(anchor, from_file);
    std::fs::remove_file(&path).ok();
}

// ── Test 24: Dynamic rigid body physics state file ────────────────────────────

#[test]
fn test_dynamic_rigid_body_physics_file() {
    let path = std::env::temp_dir().join("vr_dynamic_rb.bin");
    // Simulate a physics object at a position with a non-trivial rotation
    let rb = SceneObject {
        object_id: 4_200,
        name: "physics_crate_01".to_string(),
        obj_type: ObjectType::DynamicRigidBody,
        layer: RenderLayer::World,
        position: Vec3Fixed {
            x_um: 2_500_000,
            y_um: 800_000,
            z_um: 1_200_000,
        },
        rotation: QuatFixed {
            w_micro: 707_107,
            x_micro: 0,
            y_micro: 707_107,
            z_micro: 0,
        },
        scale_micro: 1_000_000,
    };
    encode_to_file(&rb, &path).expect("encode dynamic rigid body");
    let decoded: SceneObject = decode_from_file(&path).expect("decode dynamic rigid body");
    assert_eq!(rb, decoded);
    assert_eq!(decoded.obj_type, ObjectType::DynamicRigidBody);
    std::fs::remove_file(&path).ok();
}

// ── Test 25: Particle system with custom scale ────────────────────────────────

#[test]
fn test_particle_system_custom_scale_file() {
    let path = std::env::temp_dir().join("vr_particle_system.bin");
    let particle_sys = SceneObject {
        object_id: 7_777,
        name: "vfx_explosion_large".to_string(),
        obj_type: ObjectType::ParticleSystem,
        layer: RenderLayer::World,
        position: Vec3Fixed {
            x_um: 0,
            y_um: 500_000,
            z_um: 0,
        },
        rotation: identity_quat(),
        scale_micro: 5_000_000, // 5× scale
    };
    encode_to_file(&particle_sys, &path).expect("encode particle system");
    let decoded: SceneObject = decode_from_file(&path).expect("decode particle system");
    assert_eq!(particle_sys, decoded);
    assert_eq!(decoded.scale_micro, 5_000_000);
    std::fs::remove_file(&path).ok();
}

// ── Test 26: Temp file cleanup — verify file was written then removed ─────────

#[test]
fn test_temp_file_cleanup_after_test() {
    let path = std::env::temp_dir().join("vr_cleanup_check.bin");

    // File must not exist before the test (clean environment)
    let _ = std::fs::remove_file(&path); // idempotent pre-cleanup

    let scene = VrScene {
        scene_id: 1_234,
        name: "cleanup_verification_scene".to_string(),
        objects: vec![make_object(0, ObjectType::Ui3D, RenderLayer::Ui)],
        players: vec![make_player(42)],
    };
    encode_to_file(&scene, &path).expect("encode for cleanup test");

    // Confirm file was actually written to disk
    assert!(path.exists(), "temp file must exist after encode_to_file");
    let metadata = std::fs::metadata(&path).expect("metadata of temp file");
    assert!(metadata.len() > 0, "temp file must be non-empty");

    // Decode and verify integrity
    let decoded: VrScene = decode_from_file(&path).expect("decode for cleanup test");
    assert_eq!(scene, decoded);

    // Clean up and confirm removal
    std::fs::remove_file(&path).expect("remove temp file");
    assert!(!path.exists(), "temp file must be absent after removal");
}
