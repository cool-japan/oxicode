//! Advanced file I/O tests for robotics / robot arm control data structures.

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

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("{}_{}", name, std::process::id()))
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JointAngle {
    joint_id: u8,
    angle_deg: f32,
    velocity_dps: f32,
    torque_nm: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RobotPose {
    pose_id: u64,
    joints: Vec<JointAngle>,
    timestamp_ms: u64,
    is_calibrated: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrajectoryPoint {
    sequence: u32,
    pose: RobotPose,
    duration_ms: u32,
}

// --- Test 1: JointAngle basic write and read ---
#[test]
fn test_joint_angle_write_read_roundtrip() {
    let path = tmp("oxicode_robot_001");
    let joint = JointAngle {
        joint_id: 1,
        angle_deg: 45.0,
        velocity_dps: 10.5,
        torque_nm: 3.2,
    };
    encode_to_file(&joint, &path).expect("encode JointAngle to file");
    let decoded: JointAngle = decode_from_file(&path).expect("decode JointAngle from file");
    assert_eq!(joint, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 2: JointAngle zero values ---
#[test]
fn test_joint_angle_zero_values() {
    let path = tmp("oxicode_robot_002");
    let joint = JointAngle {
        joint_id: 0,
        angle_deg: 0.0,
        velocity_dps: 0.0,
        torque_nm: 0.0,
    };
    encode_to_file(&joint, &path).expect("encode zero JointAngle");
    let decoded: JointAngle = decode_from_file(&path).expect("decode zero JointAngle");
    assert_eq!(joint, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 3: JointAngle negative angle and torque ---
#[test]
fn test_joint_angle_negative_values() {
    let path = tmp("oxicode_robot_003");
    let joint = JointAngle {
        joint_id: 5,
        angle_deg: -135.75,
        velocity_dps: -22.3,
        torque_nm: -8.91,
    };
    encode_to_file(&joint, &path).expect("encode negative JointAngle");
    let decoded: JointAngle = decode_from_file(&path).expect("decode negative JointAngle");
    assert_eq!(joint, decoded);
    assert!(decoded.angle_deg < 0.0, "angle_deg should be negative");
    assert!(decoded.torque_nm < 0.0, "torque_nm should be negative");
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 4: RobotPose basic write and read ---
#[test]
fn test_robot_pose_write_read_roundtrip() {
    let path = tmp("oxicode_robot_004");
    let pose = RobotPose {
        pose_id: 100,
        joints: vec![
            JointAngle {
                joint_id: 0,
                angle_deg: 30.0,
                velocity_dps: 5.0,
                torque_nm: 1.5,
            },
            JointAngle {
                joint_id: 1,
                angle_deg: -15.0,
                velocity_dps: 3.0,
                torque_nm: 2.1,
            },
        ],
        timestamp_ms: 1_000_000,
        is_calibrated: true,
    };
    encode_to_file(&pose, &path).expect("encode RobotPose");
    let decoded: RobotPose = decode_from_file(&path).expect("decode RobotPose");
    assert_eq!(pose, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 5: RobotPose is_calibrated = false ---
#[test]
fn test_robot_pose_not_calibrated() {
    let path = tmp("oxicode_robot_005");
    let pose = RobotPose {
        pose_id: 200,
        joints: vec![JointAngle {
            joint_id: 2,
            angle_deg: 90.0,
            velocity_dps: 0.0,
            torque_nm: 0.5,
        }],
        timestamp_ms: 2_000_000,
        is_calibrated: false,
    };
    encode_to_file(&pose, &path).expect("encode uncalibrated pose");
    let decoded: RobotPose = decode_from_file(&path).expect("decode uncalibrated pose");
    assert_eq!(pose, decoded);
    assert!(!decoded.is_calibrated, "pose should not be calibrated");
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 6: RobotPose is_calibrated = true ---
#[test]
fn test_robot_pose_is_calibrated_true() {
    let path = tmp("oxicode_robot_006");
    let pose = RobotPose {
        pose_id: 300,
        joints: vec![JointAngle {
            joint_id: 3,
            angle_deg: -45.0,
            velocity_dps: 12.0,
            torque_nm: 4.0,
        }],
        timestamp_ms: 3_500_000,
        is_calibrated: true,
    };
    encode_to_file(&pose, &path).expect("encode calibrated pose");
    let decoded: RobotPose = decode_from_file(&path).expect("decode calibrated pose");
    assert_eq!(pose, decoded);
    assert!(decoded.is_calibrated, "pose should be calibrated");
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 7: TrajectoryPoint basic write and read ---
#[test]
fn test_trajectory_point_write_read_roundtrip() {
    let path = tmp("oxicode_robot_007");
    let tp = TrajectoryPoint {
        sequence: 1,
        pose: RobotPose {
            pose_id: 10,
            joints: vec![JointAngle {
                joint_id: 0,
                angle_deg: 60.0,
                velocity_dps: 8.0,
                torque_nm: 2.5,
            }],
            timestamp_ms: 500,
            is_calibrated: true,
        },
        duration_ms: 250,
    };
    encode_to_file(&tp, &path).expect("encode TrajectoryPoint");
    let decoded: TrajectoryPoint = decode_from_file(&path).expect("decode TrajectoryPoint");
    assert_eq!(tp, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 8: TrajectoryPoint with sequence=0 and duration=0 ---
#[test]
fn test_trajectory_point_zero_sequence_duration() {
    let path = tmp("oxicode_robot_008");
    let tp = TrajectoryPoint {
        sequence: 0,
        pose: RobotPose {
            pose_id: 0,
            joints: vec![],
            timestamp_ms: 0,
            is_calibrated: false,
        },
        duration_ms: 0,
    };
    encode_to_file(&tp, &path).expect("encode zero TrajectoryPoint");
    let decoded: TrajectoryPoint = decode_from_file(&path).expect("decode zero TrajectoryPoint");
    assert_eq!(tp, decoded);
    assert_eq!(decoded.sequence, 0);
    assert_eq!(decoded.duration_ms, 0);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 9: Large RobotPose with 20 joints ---
#[test]
fn test_robot_pose_twenty_joints() {
    let path = tmp("oxicode_robot_009");
    let joints: Vec<JointAngle> = (0u8..20)
        .map(|i| JointAngle {
            joint_id: i,
            angle_deg: f32::from(i) * 9.0 - 90.0,
            velocity_dps: f32::from(i) * 2.5,
            torque_nm: f32::from(i) * 0.75,
        })
        .collect();
    let pose = RobotPose {
        pose_id: 9999,
        joints,
        timestamp_ms: 10_000_000,
        is_calibrated: true,
    };
    encode_to_file(&pose, &path).expect("encode 20-joint pose");
    let decoded: RobotPose = decode_from_file(&path).expect("decode 20-joint pose");
    assert_eq!(pose, decoded);
    assert_eq!(decoded.joints.len(), 20);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 10: File bytes match encode_to_vec for JointAngle ---
#[test]
fn test_file_bytes_match_encode_to_vec_joint_angle() {
    let path = tmp("oxicode_robot_010");
    let joint = JointAngle {
        joint_id: 7,
        angle_deg: 123.456,
        velocity_dps: 99.9,
        torque_nm: 15.3,
    };
    encode_to_file(&joint, &path).expect("encode JointAngle to file for byte comparison");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&joint).expect("encode JointAngle to vec");
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes should match encode_to_vec output"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 11: File bytes match encode_to_vec for RobotPose ---
#[test]
fn test_file_bytes_match_encode_to_vec_robot_pose() {
    let path = tmp("oxicode_robot_011");
    let pose = RobotPose {
        pose_id: 42,
        joints: vec![
            JointAngle {
                joint_id: 0,
                angle_deg: 10.0,
                velocity_dps: 1.0,
                torque_nm: 0.1,
            },
            JointAngle {
                joint_id: 1,
                angle_deg: 20.0,
                velocity_dps: 2.0,
                torque_nm: 0.2,
            },
        ],
        timestamp_ms: 77777,
        is_calibrated: false,
    };
    encode_to_file(&pose, &path).expect("encode RobotPose for byte comparison");
    let file_bytes = std::fs::read(&path).expect("read file bytes for pose");
    let vec_bytes = encode_to_vec(&pose).expect("encode RobotPose to vec");
    assert_eq!(
        file_bytes, vec_bytes,
        "file and vec bytes should be identical for RobotPose"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 12: File bytes match encode_to_vec for TrajectoryPoint ---
#[test]
fn test_file_bytes_match_encode_to_vec_trajectory_point() {
    let path = tmp("oxicode_robot_012");
    let tp = TrajectoryPoint {
        sequence: 7,
        pose: RobotPose {
            pose_id: 77,
            joints: vec![JointAngle {
                joint_id: 4,
                angle_deg: -30.0,
                velocity_dps: 6.6,
                torque_nm: 1.1,
            }],
            timestamp_ms: 88888,
            is_calibrated: true,
        },
        duration_ms: 100,
    };
    encode_to_file(&tp, &path).expect("encode TrajectoryPoint for byte comparison");
    let file_bytes = std::fs::read(&path).expect("read file bytes for tp");
    let vec_bytes = encode_to_vec(&tp).expect("encode TrajectoryPoint to vec");
    assert_eq!(
        file_bytes, vec_bytes,
        "file and vec bytes should match for TrajectoryPoint"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 13: decode_from_slice matches decode_from_file ---
#[test]
fn test_decode_from_slice_matches_decode_from_file() {
    let path = tmp("oxicode_robot_013");
    let joint = JointAngle {
        joint_id: 3,
        angle_deg: -180.0,
        velocity_dps: 50.0,
        torque_nm: 12.0,
    };
    encode_to_file(&joint, &path).expect("encode for slice comparison");
    let file_bytes = std::fs::read(&path).expect("read bytes for slice test");
    let from_file: JointAngle = decode_from_file(&path).expect("decode from file");
    let (from_slice, _): (JointAngle, usize) =
        decode_from_slice(&file_bytes).expect("decode from slice");
    assert_eq!(
        from_file, from_slice,
        "decode_from_file and decode_from_slice must agree"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 14: Overwrite existing file with different JointAngle ---
#[test]
fn test_overwrite_existing_file_joint_angle() {
    let path = tmp("oxicode_robot_014");
    let first = JointAngle {
        joint_id: 0,
        angle_deg: 10.0,
        velocity_dps: 1.0,
        torque_nm: 0.5,
    };
    let second = JointAngle {
        joint_id: 9,
        angle_deg: 175.0,
        velocity_dps: 30.0,
        torque_nm: 9.9,
    };
    encode_to_file(&first, &path).expect("first write");
    encode_to_file(&second, &path).expect("overwrite with second");
    let decoded: JointAngle = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded, "file should contain the overwritten value");
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 15: Overwrite existing file with different RobotPose ---
#[test]
fn test_overwrite_existing_file_robot_pose() {
    let path = tmp("oxicode_robot_015");
    let first_pose = RobotPose {
        pose_id: 1,
        joints: vec![JointAngle {
            joint_id: 0,
            angle_deg: 0.0,
            velocity_dps: 0.0,
            torque_nm: 0.0,
        }],
        timestamp_ms: 100,
        is_calibrated: false,
    };
    let second_pose = RobotPose {
        pose_id: 2,
        joints: vec![
            JointAngle {
                joint_id: 0,
                angle_deg: 45.0,
                velocity_dps: 10.0,
                torque_nm: 2.0,
            },
            JointAngle {
                joint_id: 1,
                angle_deg: -45.0,
                velocity_dps: 10.0,
                torque_nm: 2.0,
            },
        ],
        timestamp_ms: 200,
        is_calibrated: true,
    };
    encode_to_file(&first_pose, &path).expect("first pose write");
    encode_to_file(&second_pose, &path).expect("overwrite with second pose");
    let decoded: RobotPose = decode_from_file(&path).expect("decode overwritten pose");
    assert_eq!(
        second_pose, decoded,
        "second pose should be present after overwrite"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 16: Error on missing file ---
#[test]
fn test_error_on_missing_file() {
    let path = tmp("oxicode_robot_016_nonexistent");
    // Ensure file does not exist
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
    let result = decode_from_file::<JointAngle>(&path);
    assert!(
        result.is_err(),
        "decode_from_file should fail for missing file"
    );
}

// --- Test 17: Multiple separate files, each with a different joint ---
#[test]
fn test_multiple_files_different_joints() {
    let joints: Vec<JointAngle> = (0u8..5)
        .map(|i| JointAngle {
            joint_id: i,
            angle_deg: f32::from(i) * 36.0,
            velocity_dps: f32::from(i) * 5.0,
            torque_nm: f32::from(i) * 1.5,
        })
        .collect();
    let paths: Vec<_> = (0u8..5)
        .map(|i| tmp(&format!("oxicode_robot_017_{i}")))
        .collect();
    for (joint, path) in joints.iter().zip(paths.iter()) {
        encode_to_file(joint, path).expect("encode joint to separate file");
    }
    for (joint, path) in joints.iter().zip(paths.iter()) {
        let decoded: JointAngle = decode_from_file(path).expect("decode joint from separate file");
        assert_eq!(
            *joint, decoded,
            "joint in separate file should match original"
        );
        if path.exists() {
            std::fs::remove_file(path).ok();
        }
    }
}

// --- Test 18: Multiple trajectory points stored in separate files ---
#[test]
fn test_multiple_trajectory_points_separate_files() {
    let tps: Vec<TrajectoryPoint> = (0u32..4)
        .map(|i| TrajectoryPoint {
            sequence: i,
            pose: RobotPose {
                pose_id: u64::from(i) * 10,
                joints: vec![JointAngle {
                    joint_id: (i % 6) as u8,
                    angle_deg: f32::from(i as u16) * 22.5,
                    velocity_dps: 5.0,
                    torque_nm: 1.0,
                }],
                timestamp_ms: u64::from(i) * 1000,
                is_calibrated: i % 2 == 0,
            },
            duration_ms: 100 + i * 50,
        })
        .collect();
    let paths: Vec<_> = (0u32..4)
        .map(|i| tmp(&format!("oxicode_robot_018_{i}")))
        .collect();
    for (tp, path) in tps.iter().zip(paths.iter()) {
        encode_to_file(tp, path).expect("encode trajectory point");
    }
    for (tp, path) in tps.iter().zip(paths.iter()) {
        let decoded: TrajectoryPoint =
            decode_from_file(path).expect("decode trajectory point from file");
        assert_eq!(*tp, decoded, "trajectory point should round-trip correctly");
        if path.exists() {
            std::fs::remove_file(path).ok();
        }
    }
}

// --- Test 19: Max angle values (JointAngle at ±360 degrees) ---
#[test]
fn test_joint_angle_max_range_values() {
    let path = tmp("oxicode_robot_019");
    let joint = JointAngle {
        joint_id: 255,
        angle_deg: 360.0,
        velocity_dps: f32::MAX / 2.0,
        torque_nm: -360.0,
    };
    encode_to_file(&joint, &path).expect("encode max-range JointAngle");
    let decoded: JointAngle = decode_from_file(&path).expect("decode max-range JointAngle");
    assert_eq!(joint.joint_id, decoded.joint_id);
    assert!((decoded.angle_deg - 360.0_f32).abs() < f32::EPSILON * 1000.0);
    assert!((decoded.torque_nm - (-360.0_f32)).abs() < f32::EPSILON * 1000.0);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 20: RobotPose empty joints list ---
#[test]
fn test_robot_pose_empty_joints() {
    let path = tmp("oxicode_robot_020");
    let pose = RobotPose {
        pose_id: 555,
        joints: vec![],
        timestamp_ms: 123_456_789,
        is_calibrated: false,
    };
    encode_to_file(&pose, &path).expect("encode pose with empty joints");
    let decoded: RobotPose = decode_from_file(&path).expect("decode pose with empty joints");
    assert_eq!(pose, decoded);
    assert!(
        decoded.joints.is_empty(),
        "joints should remain empty after roundtrip"
    );
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 21: TrajectoryPoint with maximum sequence and duration values ---
#[test]
fn test_trajectory_point_max_sequence_duration() {
    let path = tmp("oxicode_robot_021");
    let tp = TrajectoryPoint {
        sequence: u32::MAX,
        pose: RobotPose {
            pose_id: u64::MAX,
            joints: vec![JointAngle {
                joint_id: 255,
                angle_deg: -90.0,
                velocity_dps: 180.0,
                torque_nm: -50.0,
            }],
            timestamp_ms: u64::MAX,
            is_calibrated: true,
        },
        duration_ms: u32::MAX,
    };
    encode_to_file(&tp, &path).expect("encode max TrajectoryPoint");
    let decoded: TrajectoryPoint = decode_from_file(&path).expect("decode max TrajectoryPoint");
    assert_eq!(tp, decoded);
    assert_eq!(decoded.sequence, u32::MAX);
    assert_eq!(decoded.duration_ms, u32::MAX);
    assert_eq!(decoded.pose.pose_id, u64::MAX);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// --- Test 22: Sequential overwrite of TrajectoryPoint, final state verified ---
#[test]
fn test_trajectory_point_sequential_overwrite() {
    let path = tmp("oxicode_robot_022");
    let make_tp = |seq: u32| TrajectoryPoint {
        sequence: seq,
        pose: RobotPose {
            pose_id: u64::from(seq),
            joints: vec![JointAngle {
                joint_id: (seq % 6) as u8,
                angle_deg: f32::from(seq as u16) * -1.5,
                velocity_dps: f32::from(seq as u16) * 3.0,
                torque_nm: f32::from(seq as u16) * 0.2,
            }],
            timestamp_ms: u64::from(seq) * 333,
            is_calibrated: seq % 2 != 0,
        },
        duration_ms: seq * 10,
    };
    for seq in 0u32..5 {
        encode_to_file(&make_tp(seq), &path).expect("encode in sequential overwrite loop");
    }
    let final_tp = make_tp(4);
    let decoded: TrajectoryPoint =
        decode_from_file(&path).expect("decode final state after sequential overwrites");
    assert_eq!(
        final_tp, decoded,
        "file should contain the last written TrajectoryPoint"
    );
    assert_eq!(decoded.sequence, 4);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}
