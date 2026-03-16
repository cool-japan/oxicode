#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DriveMode {
    Manual,
    Assisted,
    SemiAutonomous,
    FullyAutonomous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ObjectClass {
    Vehicle,
    Pedestrian,
    Cyclist,
    Animal,
    StaticObstacle,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LaneType {
    Driving,
    Shoulder,
    BicycleLane,
    Sidewalk,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WeatherCondition {
    Clear,
    Rain,
    Fog,
    Snow,
    Night,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LidarPoint {
    x_mm: i32,
    y_mm: i32,
    z_mm: i32,
    intensity: u8,
    ring: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DetectedObject {
    obj_id: u32,
    class: ObjectClass,
    confidence_pct: u8,
    distance_mm: u32,
    velocity_mm_s: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LaneMark {
    lane_id: u16,
    lane_type: LaneType,
    left_offset_mm: i32,
    right_offset_mm: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleState {
    vehicle_id: u32,
    speed_mm_s: u32,
    heading_deg_x10: u16,
    mode: DriveMode,
    weather: WeatherCondition,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerceptionFrame {
    frame_id: u64,
    vehicle_id: u32,
    objects: Vec<DetectedObject>,
    lanes: Vec<LaneMark>,
    timestamp: u64,
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
}

// ── Test 1: async write/read single LidarPoint ────────────────────────────────

#[test]
fn test_async_single_lidar_point() {
    rt().block_on(async {
        let point = LidarPoint {
            x_mm: 1200,
            y_mm: -340,
            z_mm: 80,
            intensity: 200,
            ring: 3,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&point).await.expect("write lidar point");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<LidarPoint> = decoder
            .read_item::<LidarPoint>()
            .await
            .expect("read lidar point");
        assert_eq!(result, Some(point));
    });
}

// ── Test 2: async write/read single VehicleState ──────────────────────────────

#[test]
fn test_async_single_vehicle_state() {
    rt().block_on(async {
        let state = VehicleState {
            vehicle_id: 42,
            speed_mm_s: 13_889,
            heading_deg_x10: 2700,
            mode: DriveMode::SemiAutonomous,
            weather: WeatherCondition::Clear,
            timestamp: 1_700_000_000_000,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&state)
            .await
            .expect("write vehicle state");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read vehicle state");
        assert_eq!(result, Some(state));
    });
}

// ── Test 3: DriveMode variants – Manual ───────────────────────────────────────

#[test]
fn test_drive_mode_manual() {
    rt().block_on(async {
        let mode = DriveMode::Manual;
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&mode).await.expect("write Manual");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder.read_item::<DriveMode>().await.expect("read Manual");
        assert_eq!(result, Some(DriveMode::Manual));
    });
}

// ── Test 4: DriveMode variants – FullyAutonomous ─────────────────────────────

#[test]
fn test_drive_mode_fully_autonomous() {
    rt().block_on(async {
        let mode = DriveMode::FullyAutonomous;
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&mode)
            .await
            .expect("write FullyAutonomous");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<DriveMode>()
            .await
            .expect("read FullyAutonomous");
        assert_eq!(result, Some(DriveMode::FullyAutonomous));
    });
}

// ── Test 5: All ObjectClass variants round-trip ───────────────────────────────

#[test]
fn test_all_object_class_variants() {
    rt().block_on(async {
        let variants = vec![
            ObjectClass::Vehicle,
            ObjectClass::Pedestrian,
            ObjectClass::Cyclist,
            ObjectClass::Animal,
            ObjectClass::StaticObstacle,
            ObjectClass::Unknown,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("write ObjectClass variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<ObjectClass>()
                .await
                .expect("read ObjectClass variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 6: All WeatherCondition variants round-trip ─────────────────────────

#[test]
fn test_all_weather_condition_variants() {
    rt().block_on(async {
        let variants = vec![
            WeatherCondition::Clear,
            WeatherCondition::Rain,
            WeatherCondition::Fog,
            WeatherCondition::Snow,
            WeatherCondition::Night,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("write WeatherCondition variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<WeatherCondition>()
                .await
                .expect("read WeatherCondition variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 7: All LaneType variants round-trip ─────────────────────────────────

#[test]
fn test_all_lane_type_variants() {
    rt().block_on(async {
        let variants = vec![
            LaneType::Driving,
            LaneType::Shoulder,
            LaneType::BicycleLane,
            LaneType::Sidewalk,
        ];
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder.write_item(v).await.expect("write LaneType variant");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &variants {
            let result = decoder
                .read_item::<LaneType>()
                .await
                .expect("read LaneType variant");
            assert_eq!(result.as_ref(), Some(expected));
        }
    });
}

// ── Test 8: Nested struct – DetectedObject with ObjectClass::Pedestrian ───────

#[test]
fn test_detected_object_pedestrian() {
    rt().block_on(async {
        let obj = DetectedObject {
            obj_id: 7,
            class: ObjectClass::Pedestrian,
            confidence_pct: 93,
            distance_mm: 8_500,
            velocity_mm_s: -200,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&obj)
            .await
            .expect("write DetectedObject");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<DetectedObject>()
            .await
            .expect("read DetectedObject");
        assert_eq!(result, Some(obj));
    });
}

// ── Test 9: LaneMark with BicycleLane ────────────────────────────────────────

#[test]
fn test_lane_mark_bicycle_lane() {
    rt().block_on(async {
        let lane = LaneMark {
            lane_id: 3,
            lane_type: LaneType::BicycleLane,
            left_offset_mm: 1_500,
            right_offset_mm: -1_500,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&lane).await.expect("write LaneMark");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<LaneMark>()
            .await
            .expect("read LaneMark");
        assert_eq!(result, Some(lane));
    });
}

// ── Test 10: PerceptionFrame with empty Vec fields ────────────────────────────

#[test]
fn test_perception_frame_empty_vecs() {
    rt().block_on(async {
        let frame = PerceptionFrame {
            frame_id: 1000,
            vehicle_id: 1,
            objects: vec![],
            lanes: vec![],
            timestamp: 1_700_000_001_000,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&frame)
            .await
            .expect("write empty PerceptionFrame");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<PerceptionFrame>()
            .await
            .expect("read empty PerceptionFrame");
        assert_eq!(result, Some(frame));
    });
}

// ── Test 11: PerceptionFrame with populated Vecs ─────────────────────────────

#[test]
fn test_perception_frame_with_objects_and_lanes() {
    rt().block_on(async {
        let frame = PerceptionFrame {
            frame_id: 2001,
            vehicle_id: 5,
            objects: vec![
                DetectedObject {
                    obj_id: 1,
                    class: ObjectClass::Vehicle,
                    confidence_pct: 98,
                    distance_mm: 20_000,
                    velocity_mm_s: 8_000,
                },
                DetectedObject {
                    obj_id: 2,
                    class: ObjectClass::Cyclist,
                    confidence_pct: 85,
                    distance_mm: 5_000,
                    velocity_mm_s: 3_000,
                },
                DetectedObject {
                    obj_id: 3,
                    class: ObjectClass::Animal,
                    confidence_pct: 70,
                    distance_mm: 3_200,
                    velocity_mm_s: 0,
                },
            ],
            lanes: vec![
                LaneMark {
                    lane_id: 0,
                    lane_type: LaneType::Driving,
                    left_offset_mm: 1_800,
                    right_offset_mm: -1_800,
                },
                LaneMark {
                    lane_id: 1,
                    lane_type: LaneType::Shoulder,
                    left_offset_mm: 3_600,
                    right_offset_mm: -500,
                },
            ],
            timestamp: 1_700_000_002_000,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&frame)
            .await
            .expect("write populated PerceptionFrame");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<PerceptionFrame>()
            .await
            .expect("read populated PerceptionFrame");
        assert_eq!(result, Some(frame));
    });
}

// ── Test 12: Batch write of 15 LidarPoints ────────────────────────────────────

#[test]
fn test_batch_write_15_lidar_points() {
    rt().block_on(async {
        let points: Vec<LidarPoint> = (0..15)
            .map(|i| LidarPoint {
                x_mm: i * 100,
                y_mm: i * -50,
                z_mm: i * 10,
                intensity: (i as u8).wrapping_mul(17),
                ring: (i % 32) as u8,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for p in &points {
            encoder
                .write_item(p)
                .await
                .expect("write LidarPoint in batch");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &points {
            let result = decoder
                .read_item::<LidarPoint>()
                .await
                .expect("read LidarPoint in batch");
            assert_eq!(result.as_ref(), Some(expected));
        }

        // Stream should be exhausted
        let tail = decoder
            .read_item::<LidarPoint>()
            .await
            .expect("read after batch end");
        assert_eq!(tail, None);
    });
}

// ── Test 13: Empty stream returns None ────────────────────────────────────────

#[test]
fn test_empty_stream_returns_none() {
    rt().block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::new(writer);
        encoder.finish().await.expect("finish empty encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read from empty stream");
        assert_eq!(result, None);
    });
}

// ── Test 14: Sync encode_to_vec / decode_from_slice consistency ───────────────

#[test]
fn test_sync_encode_decode_vehicle_state() {
    let state = VehicleState {
        vehicle_id: 99,
        speed_mm_s: 27_778,
        heading_deg_x10: 900,
        mode: DriveMode::Assisted,
        weather: WeatherCondition::Rain,
        timestamp: 1_700_000_010_000,
    };
    let bytes = encode_to_vec(&state).expect("sync encode VehicleState");
    let (decoded, consumed): (VehicleState, usize) =
        decode_from_slice(&bytes).expect("sync decode VehicleState");
    assert_eq!(decoded, state);
    assert_eq!(consumed, bytes.len());
}

// ── Test 15: Sync vs async consistency for DetectedObject ────────────────────

#[test]
fn test_sync_async_consistency_detected_object() {
    let obj = DetectedObject {
        obj_id: 55,
        class: ObjectClass::StaticObstacle,
        confidence_pct: 100,
        distance_mm: 1_000,
        velocity_mm_s: 0,
    };

    // Sync path
    let bytes = encode_to_vec(&obj).expect("sync encode DetectedObject");
    let (sync_decoded, _): (DetectedObject, usize) =
        decode_from_slice(&bytes).expect("sync decode DetectedObject");
    assert_eq!(sync_decoded, obj);

    // Async path
    rt().block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&obj)
            .await
            .expect("async write DetectedObject");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let async_decoded = decoder
            .read_item::<DetectedObject>()
            .await
            .expect("async read DetectedObject");
        assert_eq!(async_decoded, Some(obj.clone()));

        // Both decodings must agree
        assert_eq!(sync_decoded, async_decoded.expect("unwrap async decoded"));
    });
}

// ── Test 16: Fog + Night weather VehicleState ─────────────────────────────────

#[test]
fn test_vehicle_state_fog_night() {
    rt().block_on(async {
        let fog_state = VehicleState {
            vehicle_id: 10,
            speed_mm_s: 5_000,
            heading_deg_x10: 1_800,
            mode: DriveMode::FullyAutonomous,
            weather: WeatherCondition::Fog,
            timestamp: 1_700_000_020_000,
        };
        let night_state = VehicleState {
            vehicle_id: 11,
            speed_mm_s: 11_111,
            heading_deg_x10: 0,
            mode: DriveMode::SemiAutonomous,
            weather: WeatherCondition::Night,
            timestamp: 1_700_000_021_000,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&fog_state)
            .await
            .expect("write fog state");
        encoder
            .write_item(&night_state)
            .await
            .expect("write night state");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let r1 = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read fog state");
        let r2 = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read night state");
        assert_eq!(r1, Some(fog_state));
        assert_eq!(r2, Some(night_state));
    });
}

// ── Test 17: Negative velocity and extreme coordinates in LidarPoint ──────────

#[test]
fn test_lidar_point_extreme_values() {
    rt().block_on(async {
        let point = LidarPoint {
            x_mm: i32::MIN,
            y_mm: i32::MAX,
            z_mm: -999_999,
            intensity: 255,
            ring: 127,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&point)
            .await
            .expect("write extreme LidarPoint");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<LidarPoint>()
            .await
            .expect("read extreme LidarPoint");
        assert_eq!(result, Some(point));
    });
}

// ── Test 18: Large PerceptionFrame with 4 objects and 3 lanes ────────────────

#[test]
fn test_large_perception_frame() {
    rt().block_on(async {
        let frame = PerceptionFrame {
            frame_id: u64::MAX,
            vehicle_id: u32::MAX,
            objects: vec![
                DetectedObject {
                    obj_id: 100,
                    class: ObjectClass::Vehicle,
                    confidence_pct: 99,
                    distance_mm: 50_000,
                    velocity_mm_s: 15_000,
                },
                DetectedObject {
                    obj_id: 101,
                    class: ObjectClass::Pedestrian,
                    confidence_pct: 88,
                    distance_mm: 7_000,
                    velocity_mm_s: 500,
                },
                DetectedObject {
                    obj_id: 102,
                    class: ObjectClass::Unknown,
                    confidence_pct: 40,
                    distance_mm: 120_000,
                    velocity_mm_s: -300,
                },
                DetectedObject {
                    obj_id: 103,
                    class: ObjectClass::Cyclist,
                    confidence_pct: 76,
                    distance_mm: 15_000,
                    velocity_mm_s: 4_000,
                },
            ],
            lanes: vec![
                LaneMark {
                    lane_id: 10,
                    lane_type: LaneType::Driving,
                    left_offset_mm: 2_000,
                    right_offset_mm: -2_000,
                },
                LaneMark {
                    lane_id: 11,
                    lane_type: LaneType::Sidewalk,
                    left_offset_mm: 5_000,
                    right_offset_mm: -800,
                },
                LaneMark {
                    lane_id: 12,
                    lane_type: LaneType::Shoulder,
                    left_offset_mm: 4_000,
                    right_offset_mm: -200,
                },
            ],
            timestamp: 9_999_999_999_999,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&frame)
            .await
            .expect("write large PerceptionFrame");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<PerceptionFrame>()
            .await
            .expect("read large PerceptionFrame");
        assert_eq!(result, Some(frame));
    });
}

// ── Test 19: Mixed types written sequentially ─────────────────────────────────

#[test]
fn test_mixed_types_sequential() {
    rt().block_on(async {
        let state = VehicleState {
            vehicle_id: 77,
            speed_mm_s: 0,
            heading_deg_x10: 3_590,
            mode: DriveMode::Manual,
            weather: WeatherCondition::Snow,
            timestamp: 1_700_000_030_000,
        };
        let obj = DetectedObject {
            obj_id: 200,
            class: ObjectClass::Vehicle,
            confidence_pct: 95,
            distance_mm: 30_000,
            velocity_mm_s: 10_000,
        };
        let lane = LaneMark {
            lane_id: 5,
            lane_type: LaneType::Driving,
            left_offset_mm: 1_700,
            right_offset_mm: -1_700,
        };

        // Encode each to its own channel to avoid type confusion
        let (w1, r1) = tokio::io::duplex(65536);
        let (w2, r2) = tokio::io::duplex(65536);
        let (w3, r3) = tokio::io::duplex(65536);

        let mut enc1 = AsyncEncoder::new(w1);
        enc1.write_item(&state).await.expect("write state");
        enc1.finish().await.expect("finish enc1");

        let mut enc2 = AsyncEncoder::new(w2);
        enc2.write_item(&obj).await.expect("write obj");
        enc2.finish().await.expect("finish enc2");

        let mut enc3 = AsyncEncoder::new(w3);
        enc3.write_item(&lane).await.expect("write lane");
        enc3.finish().await.expect("finish enc3");

        let r_state = AsyncDecoder::new(r1)
            .read_item::<VehicleState>()
            .await
            .expect("read state");
        let r_obj = AsyncDecoder::new(r2)
            .read_item::<DetectedObject>()
            .await
            .expect("read obj");
        let r_lane = AsyncDecoder::new(r3)
            .read_item::<LaneMark>()
            .await
            .expect("read lane");

        assert_eq!(r_state, Some(state));
        assert_eq!(r_obj, Some(obj));
        assert_eq!(r_lane, Some(lane));
    });
}

// ── Test 20: Batch write of 20 VehicleState items ────────────────────────────

#[test]
fn test_batch_write_20_vehicle_states() {
    rt().block_on(async {
        let states: Vec<VehicleState> = (0..20u32)
            .map(|i| VehicleState {
                vehicle_id: i,
                speed_mm_s: i * 500,
                heading_deg_x10: (i * 180) as u16,
                mode: match i % 4 {
                    0 => DriveMode::Manual,
                    1 => DriveMode::Assisted,
                    2 => DriveMode::SemiAutonomous,
                    _ => DriveMode::FullyAutonomous,
                },
                weather: match i % 5 {
                    0 => WeatherCondition::Clear,
                    1 => WeatherCondition::Rain,
                    2 => WeatherCondition::Fog,
                    3 => WeatherCondition::Snow,
                    _ => WeatherCondition::Night,
                },
                timestamp: 1_700_000_000_000 + (i as u64) * 33,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for s in &states {
            encoder
                .write_item(s)
                .await
                .expect("write VehicleState in batch");
        }
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        for expected in &states {
            let result = decoder
                .read_item::<VehicleState>()
                .await
                .expect("read VehicleState in batch");
            assert_eq!(result.as_ref(), Some(expected));
        }

        let tail = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read after batch");
        assert_eq!(tail, None);
    });
}

// ── Test 21: Sync encode_to_vec / decode_from_slice for PerceptionFrame ───────

#[test]
fn test_sync_encode_decode_perception_frame() {
    let frame = PerceptionFrame {
        frame_id: 5000,
        vehicle_id: 8,
        objects: vec![DetectedObject {
            obj_id: 301,
            class: ObjectClass::Animal,
            confidence_pct: 60,
            distance_mm: 4_000,
            velocity_mm_s: -100,
        }],
        lanes: vec![LaneMark {
            lane_id: 7,
            lane_type: LaneType::Sidewalk,
            left_offset_mm: 3_000,
            right_offset_mm: -300,
        }],
        timestamp: 1_700_000_050_000,
    };
    let bytes = encode_to_vec(&frame).expect("sync encode PerceptionFrame");
    let (decoded, consumed): (PerceptionFrame, usize) =
        decode_from_slice(&bytes).expect("sync decode PerceptionFrame");
    assert_eq!(decoded, frame);
    assert_eq!(consumed, bytes.len());
}

// ── Test 22: VehicleState zero-speed stopped in Snow + Manual mode ────────────

#[test]
fn test_vehicle_state_stopped_snow_manual() {
    rt().block_on(async {
        let state = VehicleState {
            vehicle_id: 0,
            speed_mm_s: 0,
            heading_deg_x10: 0,
            mode: DriveMode::Manual,
            weather: WeatherCondition::Snow,
            timestamp: 0,
        };
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_item(&state)
            .await
            .expect("write stopped snow state");
        encoder.finish().await.expect("finish encoder");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read stopped snow state");
        assert_eq!(result, Some(state));

        // Confirm stream is exhausted after single item
        let tail = decoder
            .read_item::<VehicleState>()
            .await
            .expect("read tail after single item");
        assert_eq!(tail, None);
    });
}
