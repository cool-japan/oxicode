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

#[derive(Debug, PartialEq, Encode, Decode)]
enum TaskStatus {
    Planned,
    InProgress,
    Blocked,
    Completed,
    OnHold,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MaterialType {
    Concrete,
    Steel,
    Wood,
    Glass,
    Brick,
    Insulation,
    Wiring,
    Plumbing,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BuildingSystem {
    Structural,
    Hvac,
    Electrical,
    Plumbing,
    Fire,
    Elevator,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MaterialOrder {
    order_id: u32,
    material: MaterialType,
    quantity_kg: u32,
    unit_cost_cents: u32,
    delivery_date: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConstructionTask {
    task_id: u32,
    name: String,
    status: TaskStatus,
    system: BuildingSystem,
    planned_start: u64,
    planned_end: u64,
    materials: Vec<MaterialOrder>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProjectPhase {
    phase_id: u16,
    name: String,
    tasks: Vec<ConstructionTask>,
    floor_level: i8,
}

#[test]
fn test_task_status_planned() {
    let val = TaskStatus::Planned;
    let bytes = encode_to_vec(&val).expect("encode TaskStatus::Planned");
    let (decoded, _) = decode_from_slice::<TaskStatus>(&bytes).expect("decode TaskStatus::Planned");
    assert_eq!(val, decoded);
}

#[test]
fn test_task_status_in_progress() {
    let val = TaskStatus::InProgress;
    let bytes = encode_to_vec(&val).expect("encode TaskStatus::InProgress");
    let (decoded, _) =
        decode_from_slice::<TaskStatus>(&bytes).expect("decode TaskStatus::InProgress");
    assert_eq!(val, decoded);
}

#[test]
fn test_task_status_blocked() {
    let val = TaskStatus::Blocked;
    let bytes = encode_to_vec(&val).expect("encode TaskStatus::Blocked");
    let (decoded, _) = decode_from_slice::<TaskStatus>(&bytes).expect("decode TaskStatus::Blocked");
    assert_eq!(val, decoded);
}

#[test]
fn test_task_status_completed() {
    let val = TaskStatus::Completed;
    let bytes = encode_to_vec(&val).expect("encode TaskStatus::Completed");
    let (decoded, _) =
        decode_from_slice::<TaskStatus>(&bytes).expect("decode TaskStatus::Completed");
    assert_eq!(val, decoded);
}

#[test]
fn test_task_status_on_hold() {
    let val = TaskStatus::OnHold;
    let bytes = encode_to_vec(&val).expect("encode TaskStatus::OnHold");
    let (decoded, _) = decode_from_slice::<TaskStatus>(&bytes).expect("decode TaskStatus::OnHold");
    assert_eq!(val, decoded);
}

#[test]
fn test_material_type_all_variants() {
    let variants = [
        MaterialType::Concrete,
        MaterialType::Steel,
        MaterialType::Wood,
        MaterialType::Glass,
        MaterialType::Brick,
        MaterialType::Insulation,
        MaterialType::Wiring,
        MaterialType::Plumbing,
    ];
    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode MaterialType variant");
        let (decoded, _) =
            decode_from_slice::<MaterialType>(&bytes).expect("decode MaterialType variant");
        assert_eq!(variant, decoded);
    }
}

#[test]
fn test_building_system_all_variants() {
    let variants = [
        BuildingSystem::Structural,
        BuildingSystem::Hvac,
        BuildingSystem::Electrical,
        BuildingSystem::Plumbing,
        BuildingSystem::Fire,
        BuildingSystem::Elevator,
    ];
    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode BuildingSystem variant");
        let (decoded, _) =
            decode_from_slice::<BuildingSystem>(&bytes).expect("decode BuildingSystem variant");
        assert_eq!(variant, decoded);
    }
}

#[test]
fn test_material_order_roundtrip() {
    let order = MaterialOrder {
        order_id: 1001,
        material: MaterialType::Concrete,
        quantity_kg: 5000,
        unit_cost_cents: 1250,
        delivery_date: 1_700_000_000,
    };
    let bytes = encode_to_vec(&order).expect("encode MaterialOrder");
    let (decoded, _) = decode_from_slice::<MaterialOrder>(&bytes).expect("decode MaterialOrder");
    assert_eq!(order, decoded);
}

#[test]
fn test_construction_task_empty_materials() {
    let task = ConstructionTask {
        task_id: 42,
        name: "Foundation Inspection".to_string(),
        status: TaskStatus::Planned,
        system: BuildingSystem::Structural,
        planned_start: 1_710_000_000,
        planned_end: 1_710_086_400,
        materials: vec![],
    };
    let bytes = encode_to_vec(&task).expect("encode ConstructionTask empty materials");
    let (decoded, _) = decode_from_slice::<ConstructionTask>(&bytes)
        .expect("decode ConstructionTask empty materials");
    assert_eq!(task, decoded);
}

#[test]
fn test_construction_task_with_three_materials() {
    let task = ConstructionTask {
        task_id: 100,
        name: "Pour Concrete Slab".to_string(),
        status: TaskStatus::InProgress,
        system: BuildingSystem::Structural,
        planned_start: 1_711_000_000,
        planned_end: 1_711_172_800,
        materials: vec![
            MaterialOrder {
                order_id: 201,
                material: MaterialType::Concrete,
                quantity_kg: 10_000,
                unit_cost_cents: 900,
                delivery_date: 1_710_950_000,
            },
            MaterialOrder {
                order_id: 202,
                material: MaterialType::Steel,
                quantity_kg: 2_000,
                unit_cost_cents: 5_500,
                delivery_date: 1_710_960_000,
            },
            MaterialOrder {
                order_id: 203,
                material: MaterialType::Wood,
                quantity_kg: 500,
                unit_cost_cents: 1_800,
                delivery_date: 1_710_970_000,
            },
        ],
    };
    let bytes = encode_to_vec(&task).expect("encode ConstructionTask 3 materials");
    let (decoded, _) =
        decode_from_slice::<ConstructionTask>(&bytes).expect("decode ConstructionTask 3 materials");
    assert_eq!(task, decoded);
}

#[test]
fn test_project_phase_roundtrip() {
    let phase = ProjectPhase {
        phase_id: 1,
        name: "Ground Floor".to_string(),
        tasks: vec![ConstructionTask {
            task_id: 10,
            name: "Install Flooring".to_string(),
            status: TaskStatus::Planned,
            system: BuildingSystem::Structural,
            planned_start: 1_712_000_000,
            planned_end: 1_712_259_200,
            materials: vec![],
        }],
        floor_level: 0,
    };
    let bytes = encode_to_vec(&phase).expect("encode ProjectPhase");
    let (decoded, _) = decode_from_slice::<ProjectPhase>(&bytes).expect("decode ProjectPhase");
    assert_eq!(phase, decoded);
}

#[test]
fn test_phase_with_five_tasks() {
    let make_task =
        |id: u32, name: &str, status: TaskStatus, system: BuildingSystem| ConstructionTask {
            task_id: id,
            name: name.to_string(),
            status,
            system,
            planned_start: 1_713_000_000 + id as u64 * 86_400,
            planned_end: 1_713_000_000 + id as u64 * 86_400 + 28_800,
            materials: vec![],
        };
    let phase = ProjectPhase {
        phase_id: 2,
        name: "Second Floor MEP Rough-In".to_string(),
        tasks: vec![
            make_task(
                1,
                "Electrical Conduit",
                TaskStatus::Planned,
                BuildingSystem::Electrical,
            ),
            make_task(
                2,
                "HVAC Ductwork",
                TaskStatus::InProgress,
                BuildingSystem::Hvac,
            ),
            make_task(
                3,
                "Sprinkler Heads",
                TaskStatus::Blocked,
                BuildingSystem::Fire,
            ),
            make_task(
                4,
                "Plumbing Rough",
                TaskStatus::Planned,
                BuildingSystem::Plumbing,
            ),
            make_task(
                5,
                "Elevator Shaft Prep",
                TaskStatus::OnHold,
                BuildingSystem::Elevator,
            ),
        ],
        floor_level: 2,
    };
    let bytes = encode_to_vec(&phase).expect("encode phase with 5 tasks");
    let (decoded, _) =
        decode_from_slice::<ProjectPhase>(&bytes).expect("decode phase with 5 tasks");
    assert_eq!(phase, decoded);
}

#[test]
fn test_big_endian_config() {
    let order = MaterialOrder {
        order_id: 9999,
        material: MaterialType::Glass,
        quantity_kg: 750,
        unit_cost_cents: 8_000,
        delivery_date: 1_720_000_000,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&order, cfg).expect("encode big endian");
    let (decoded, _) =
        decode_from_slice_with_config::<MaterialOrder, _>(&bytes, cfg).expect("decode big endian");
    assert_eq!(order, decoded);
}

#[test]
fn test_fixed_int_config() {
    let task = ConstructionTask {
        task_id: 77,
        name: "Install Fire Suppression".to_string(),
        status: TaskStatus::InProgress,
        system: BuildingSystem::Fire,
        planned_start: 1_714_000_000,
        planned_end: 1_714_100_000,
        materials: vec![],
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&task, cfg).expect("encode fixed int");
    let (decoded, _) = decode_from_slice_with_config::<ConstructionTask, _>(&bytes, cfg)
        .expect("decode fixed int");
    assert_eq!(task, decoded);
}

#[test]
fn test_consumed_bytes_check() {
    let order = MaterialOrder {
        order_id: 500,
        material: MaterialType::Brick,
        quantity_kg: 3_000,
        unit_cost_cents: 200,
        delivery_date: 1_715_000_000,
    };
    let bytes = encode_to_vec(&order).expect("encode for consumed bytes check");
    let (decoded, consumed) =
        decode_from_slice::<MaterialOrder>(&bytes).expect("decode for consumed bytes check");
    assert_eq!(order, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes should equal encoded length"
    );
}

#[test]
fn test_vec_material_order_roundtrip() {
    let orders = vec![
        MaterialOrder {
            order_id: 1,
            material: MaterialType::Insulation,
            quantity_kg: 800,
            unit_cost_cents: 3_200,
            delivery_date: 1_716_000_000,
        },
        MaterialOrder {
            order_id: 2,
            material: MaterialType::Wiring,
            quantity_kg: 150,
            unit_cost_cents: 12_000,
            delivery_date: 1_716_100_000,
        },
        MaterialOrder {
            order_id: 3,
            material: MaterialType::Plumbing,
            quantity_kg: 400,
            unit_cost_cents: 6_500,
            delivery_date: 1_716_200_000,
        },
    ];
    let bytes = encode_to_vec(&orders).expect("encode Vec<MaterialOrder>");
    let (decoded, _) =
        decode_from_slice::<Vec<MaterialOrder>>(&bytes).expect("decode Vec<MaterialOrder>");
    assert_eq!(orders, decoded);
}

#[test]
fn test_vec_construction_task_roundtrip() {
    let tasks = vec![
        ConstructionTask {
            task_id: 1,
            name: "Framing".to_string(),
            status: TaskStatus::Completed,
            system: BuildingSystem::Structural,
            planned_start: 1_700_000_000,
            planned_end: 1_700_259_200,
            materials: vec![],
        },
        ConstructionTask {
            task_id: 2,
            name: "Drywall".to_string(),
            status: TaskStatus::InProgress,
            system: BuildingSystem::Structural,
            planned_start: 1_700_300_000,
            planned_end: 1_700_600_000,
            materials: vec![],
        },
    ];
    let bytes = encode_to_vec(&tasks).expect("encode Vec<ConstructionTask>");
    let (decoded, _) =
        decode_from_slice::<Vec<ConstructionTask>>(&bytes).expect("decode Vec<ConstructionTask>");
    assert_eq!(tasks, decoded);
}

#[test]
fn test_concrete_order_large_quantity() {
    let order = MaterialOrder {
        order_id: 8888,
        material: MaterialType::Concrete,
        quantity_kg: 500_000,
        unit_cost_cents: 850,
        delivery_date: 1_717_000_000,
    };
    let bytes = encode_to_vec(&order).expect("encode large concrete order");
    let (decoded, _) =
        decode_from_slice::<MaterialOrder>(&bytes).expect("decode large concrete order");
    assert_eq!(order, decoded);
    assert_eq!(decoded.quantity_kg, 500_000);
}

#[test]
fn test_steel_structural_task() {
    let task = ConstructionTask {
        task_id: 301,
        name: "Steel Beam Installation".to_string(),
        status: TaskStatus::InProgress,
        system: BuildingSystem::Structural,
        planned_start: 1_718_000_000,
        planned_end: 1_718_432_000,
        materials: vec![MaterialOrder {
            order_id: 401,
            material: MaterialType::Steel,
            quantity_kg: 15_000,
            unit_cost_cents: 6_200,
            delivery_date: 1_717_900_000,
        }],
    };
    let bytes = encode_to_vec(&task).expect("encode steel structural task");
    let (decoded, _) =
        decode_from_slice::<ConstructionTask>(&bytes).expect("decode steel structural task");
    assert_eq!(task, decoded);
    assert_eq!(decoded.system, BuildingSystem::Structural);
    assert_eq!(decoded.materials[0].material, MaterialType::Steel);
}

#[test]
fn test_hvac_blocked_task() {
    let task = ConstructionTask {
        task_id: 555,
        name: "Air Handling Unit Install".to_string(),
        status: TaskStatus::Blocked,
        system: BuildingSystem::Hvac,
        planned_start: 1_719_000_000,
        planned_end: 1_719_172_800,
        materials: vec![],
    };
    let bytes = encode_to_vec(&task).expect("encode HVAC blocked task");
    let (decoded, _) =
        decode_from_slice::<ConstructionTask>(&bytes).expect("decode HVAC blocked task");
    assert_eq!(task, decoded);
    assert_eq!(decoded.status, TaskStatus::Blocked);
    assert_eq!(decoded.system, BuildingSystem::Hvac);
}

#[test]
fn test_electrical_wiring_completed() {
    let task = ConstructionTask {
        task_id: 666,
        name: "Panel Board Wiring".to_string(),
        status: TaskStatus::Completed,
        system: BuildingSystem::Electrical,
        planned_start: 1_720_000_000,
        planned_end: 1_720_086_400,
        materials: vec![MaterialOrder {
            order_id: 701,
            material: MaterialType::Wiring,
            quantity_kg: 300,
            unit_cost_cents: 15_000,
            delivery_date: 1_719_950_000,
        }],
    };
    let bytes = encode_to_vec(&task).expect("encode electrical wiring completed");
    let (decoded, _) =
        decode_from_slice::<ConstructionTask>(&bytes).expect("decode electrical wiring completed");
    assert_eq!(task, decoded);
    assert_eq!(decoded.status, TaskStatus::Completed);
    assert_eq!(decoded.system, BuildingSystem::Electrical);
}

#[test]
fn test_underground_phase_negative_floor() {
    let phase = ProjectPhase {
        phase_id: 10,
        name: "Basement Parking".to_string(),
        tasks: vec![ConstructionTask {
            task_id: 800,
            name: "Underground Waterproofing".to_string(),
            status: TaskStatus::Planned,
            system: BuildingSystem::Structural,
            planned_start: 1_721_000_000,
            planned_end: 1_721_259_200,
            materials: vec![],
        }],
        floor_level: -3,
    };
    let bytes = encode_to_vec(&phase).expect("encode underground phase");
    let (decoded, _) = decode_from_slice::<ProjectPhase>(&bytes).expect("decode underground phase");
    assert_eq!(phase, decoded);
    assert_eq!(decoded.floor_level, -3);
}

#[test]
fn test_rooftop_phase_high_floor() {
    let phase = ProjectPhase {
        phase_id: 99,
        name: "Rooftop Mechanical".to_string(),
        tasks: vec![ConstructionTask {
            task_id: 900,
            name: "Rooftop HVAC Units".to_string(),
            status: TaskStatus::Planned,
            system: BuildingSystem::Hvac,
            planned_start: 1_722_000_000,
            planned_end: 1_722_172_800,
            materials: vec![],
        }],
        floor_level: 50,
    };
    let bytes = encode_to_vec(&phase).expect("encode rooftop phase");
    let (decoded, _) = decode_from_slice::<ProjectPhase>(&bytes).expect("decode rooftop phase");
    assert_eq!(phase, decoded);
    assert_eq!(decoded.floor_level, 50);
}

#[test]
fn test_multi_phase_project_three_phases() {
    let phases = vec![
        ProjectPhase {
            phase_id: 1,
            name: "Foundation".to_string(),
            tasks: vec![ConstructionTask {
                task_id: 1,
                name: "Excavation".to_string(),
                status: TaskStatus::Completed,
                system: BuildingSystem::Structural,
                planned_start: 1_700_000_000,
                planned_end: 1_700_604_800,
                materials: vec![],
            }],
            floor_level: -1,
        },
        ProjectPhase {
            phase_id: 2,
            name: "Structure".to_string(),
            tasks: vec![ConstructionTask {
                task_id: 2,
                name: "Column Erection".to_string(),
                status: TaskStatus::InProgress,
                system: BuildingSystem::Structural,
                planned_start: 1_701_000_000,
                planned_end: 1_701_864_000,
                materials: vec![],
            }],
            floor_level: 1,
        },
        ProjectPhase {
            phase_id: 3,
            name: "Fit-Out".to_string(),
            tasks: vec![ConstructionTask {
                task_id: 3,
                name: "Interior Finishes".to_string(),
                status: TaskStatus::Planned,
                system: BuildingSystem::Electrical,
                planned_start: 1_702_000_000,
                planned_end: 1_703_296_000,
                materials: vec![],
            }],
            floor_level: 5,
        },
    ];
    let bytes = encode_to_vec(&phases).expect("encode multi-phase project");
    let (decoded, _) =
        decode_from_slice::<Vec<ProjectPhase>>(&bytes).expect("decode multi-phase project");
    assert_eq!(phases, decoded);
    assert_eq!(decoded.len(), 3);
}

#[test]
fn test_distinct_discriminants_task_status() {
    let statuses = [
        TaskStatus::Planned,
        TaskStatus::InProgress,
        TaskStatus::Blocked,
        TaskStatus::Completed,
        TaskStatus::OnHold,
    ];
    let mut encoded_variants: Vec<Vec<u8>> = Vec::new();
    for status in &statuses {
        let bytes = encode_to_vec(status).expect("encode TaskStatus for distinct check");
        encoded_variants.push(bytes);
    }
    // All encoded forms must be distinct
    for i in 0..encoded_variants.len() {
        for j in (i + 1)..encoded_variants.len() {
            assert_ne!(
                encoded_variants[i], encoded_variants[j],
                "TaskStatus variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

#[test]
fn test_building_system_all_variants_in_one_phase() {
    let systems = [
        BuildingSystem::Structural,
        BuildingSystem::Hvac,
        BuildingSystem::Electrical,
        BuildingSystem::Plumbing,
        BuildingSystem::Fire,
        BuildingSystem::Elevator,
    ];
    let tasks: Vec<ConstructionTask> = systems
        .into_iter()
        .enumerate()
        .map(|(i, system)| ConstructionTask {
            task_id: i as u32 + 1,
            name: format!("System Task {}", i + 1),
            status: TaskStatus::Planned,
            system,
            planned_start: 1_725_000_000 + i as u64 * 86_400,
            planned_end: 1_725_000_000 + i as u64 * 86_400 + 28_800,
            materials: vec![],
        })
        .collect();
    let phase = ProjectPhase {
        phase_id: 50,
        name: "All Systems Integration".to_string(),
        tasks,
        floor_level: 3,
    };
    let bytes = encode_to_vec(&phase).expect("encode all building systems phase");
    let (decoded, consumed) =
        decode_from_slice::<ProjectPhase>(&bytes).expect("decode all building systems phase");
    assert_eq!(phase, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.tasks.len(), 6);
    assert_eq!(decoded.tasks[0].system, BuildingSystem::Structural);
    assert_eq!(decoded.tasks[1].system, BuildingSystem::Hvac);
    assert_eq!(decoded.tasks[2].system, BuildingSystem::Electrical);
    assert_eq!(decoded.tasks[3].system, BuildingSystem::Plumbing);
    assert_eq!(decoded.tasks[4].system, BuildingSystem::Fire);
    assert_eq!(decoded.tasks[5].system, BuildingSystem::Elevator);
}
