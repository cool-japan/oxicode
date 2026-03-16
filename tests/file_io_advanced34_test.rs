//! Advanced file I/O tests for OxiCode — domain: mining operations / mineral processing / geological surveying

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
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MineralType {
    Gold,
    Silver,
    Copper,
    Iron,
    Coal,
    Lithium,
    Nickel,
    Zinc,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExcavationMethod {
    OpenPit,
    Underground,
    Quarrying,
    Dredging,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RockClass {
    Igneous,
    Sedimentary,
    Metamorphic,
    Ore,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentType {
    Excavator,
    Drill,
    Crusher,
    Conveyor,
    HaulTruck,
    Blaster,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrillHole {
    hole_id: u32,
    collar_x_m: f32,
    collar_y_m: f32,
    collar_z_m: f32,
    depth_m: f32,
    azimuth_deg: f32,
    dip_deg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssaySample {
    sample_id: u64,
    hole_id: u32,
    depth_from_m: f32,
    depth_to_m: f32,
    mineral: MineralType,
    grade_ppm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BlastDesign {
    blast_id: u64,
    bench_id: u32,
    hole_count: u16,
    explosive_kg: u32,
    delay_ms: u32,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OreBlock {
    block_id: u64,
    x: f32,
    y: f32,
    z: f32,
    rock_class: RockClass,
    tonnage_x100: u32,
    grade_ppm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentShift {
    shift_id: u64,
    equipment_id: u32,
    equip_type: EquipmentType,
    operator_id: u32,
    hours_worked_x100: u16,
    fuel_liters: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeologicalMap {
    survey_id: u32,
    blocks: Vec<OreBlock>,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn unique_tmp(name: &str) -> std::path::PathBuf {
    temp_dir().join(name)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. DrillHole roundtrip via Vec<u8>
#[test]
fn test_drill_hole_vec_roundtrip() {
    let hole = DrillHole {
        hole_id: 1001,
        collar_x_m: 512.75,
        collar_y_m: 3087.0,
        collar_z_m: 210.5,
        depth_m: 350.0,
        azimuth_deg: 045.0,
        dip_deg: -70.0,
    };
    let bytes = encode_to_vec(&hole).expect("encode DrillHole");
    let (decoded, consumed): (DrillHole, usize) =
        decode_from_slice(&bytes).expect("decode DrillHole");
    assert_eq!(hole, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 2. DrillHole roundtrip via file I/O
#[test]
fn test_drill_hole_file_roundtrip() {
    let path = unique_tmp("drill_hole_34.bin");
    let hole = DrillHole {
        hole_id: 2002,
        collar_x_m: 1024.0,
        collar_y_m: 2048.0,
        collar_z_m: 305.25,
        depth_m: 500.0,
        azimuth_deg: 180.0,
        dip_deg: -60.0,
    };
    encode_to_file(&hole, &path).expect("encode_to_file DrillHole");
    let decoded: DrillHole = decode_from_file(&path).expect("decode_from_file DrillHole");
    assert_eq!(hole, decoded);
    std::fs::remove_file(&path).expect("cleanup drill_hole_34.bin");
}

/// 3. AssaySample with Gold mineral — vec roundtrip
#[test]
fn test_assay_sample_gold_vec_roundtrip() {
    let sample = AssaySample {
        sample_id: 100_000,
        hole_id: 1001,
        depth_from_m: 120.0,
        depth_to_m: 121.0,
        mineral: MineralType::Gold,
        grade_ppm: 3_450,
    };
    let bytes = encode_to_vec(&sample).expect("encode AssaySample gold");
    let (decoded, _): (AssaySample, usize) =
        decode_from_slice(&bytes).expect("decode AssaySample gold");
    assert_eq!(sample, decoded);
}

/// 4. AssaySample with Lithium — file roundtrip
#[test]
fn test_assay_sample_lithium_file_roundtrip() {
    let path = unique_tmp("assay_lithium_34.bin");
    let sample = AssaySample {
        sample_id: 200_001,
        hole_id: 3003,
        depth_from_m: 55.5,
        depth_to_m: 56.5,
        mineral: MineralType::Lithium,
        grade_ppm: 12_000,
    };
    encode_to_file(&sample, &path).expect("encode_to_file Lithium");
    let decoded: AssaySample = decode_from_file(&path).expect("decode_from_file Lithium");
    assert_eq!(sample, decoded);
    std::fs::remove_file(&path).expect("cleanup assay_lithium_34.bin");
}

/// 5. Large assay dataset — 500+ samples roundtrip
#[test]
fn test_large_assay_dataset_500_samples() {
    let minerals = [
        MineralType::Gold,
        MineralType::Silver,
        MineralType::Copper,
        MineralType::Iron,
        MineralType::Coal,
        MineralType::Lithium,
        MineralType::Nickel,
        MineralType::Zinc,
    ];
    let samples: Vec<AssaySample> = (0u64..512)
        .map(|i| AssaySample {
            sample_id: i,
            hole_id: (i % 16) as u32,
            depth_from_m: (i as f32) * 1.5,
            depth_to_m: (i as f32) * 1.5 + 1.0,
            mineral: minerals[(i as usize) % minerals.len()].clone(),
            grade_ppm: ((i * 7 + 13) % 50_000) as u32,
        })
        .collect();
    assert_eq!(samples.len(), 512);
    let bytes = encode_to_vec(&samples).expect("encode large assay dataset");
    let (decoded, _): (Vec<AssaySample>, usize) =
        decode_from_slice(&bytes).expect("decode large assay dataset");
    assert_eq!(samples, decoded);
}

/// 6. BlastDesign vec roundtrip via file
#[test]
fn test_blast_design_vec_file_roundtrip() {
    let path = unique_tmp("blast_designs_34.bin");
    let blasts: Vec<BlastDesign> = (0u64..20)
        .map(|i| BlastDesign {
            blast_id: 5000 + i,
            bench_id: (i % 5) as u32,
            hole_count: (30 + i * 2) as u16,
            explosive_kg: 2000 + (i * 150) as u32,
            delay_ms: 500 + (i * 42) as u32,
            timestamp: 1_700_000_000 + i * 3600,
        })
        .collect();
    encode_to_file(&blasts, &path).expect("encode_to_file blast designs");
    let decoded: Vec<BlastDesign> =
        decode_from_file(&path).expect("decode_from_file blast designs");
    assert_eq!(blasts, decoded);
    std::fs::remove_file(&path).expect("cleanup blast_designs_34.bin");
}

/// 7. OreBlock all RockClass variants roundtrip
#[test]
fn test_ore_block_all_rock_classes() {
    let classes = [
        RockClass::Igneous,
        RockClass::Sedimentary,
        RockClass::Metamorphic,
        RockClass::Ore,
    ];
    for (i, rock_class) in classes.into_iter().enumerate() {
        let block = OreBlock {
            block_id: i as u64,
            x: i as f32 * 10.0,
            y: i as f32 * 20.0,
            z: i as f32 * 5.0,
            rock_class,
            tonnage_x100: 50_000 + i as u32 * 1000,
            grade_ppm: 800 + i as u32 * 200,
        };
        let bytes = encode_to_vec(&block).expect("encode OreBlock");
        let (decoded, _): (OreBlock, usize) = decode_from_slice(&bytes).expect("decode OreBlock");
        assert_eq!(block, decoded);
    }
}

/// 8. GeologicalMap with multiple OreBlocks — vec roundtrip
#[test]
fn test_geological_map_vec_roundtrip() {
    let blocks: Vec<OreBlock> = (0u64..10)
        .map(|i| OreBlock {
            block_id: i,
            x: i as f32 * 25.0,
            y: i as f32 * 25.0,
            z: 0.0,
            rock_class: if i % 2 == 0 {
                RockClass::Ore
            } else {
                RockClass::Igneous
            },
            tonnage_x100: 100_000,
            grade_ppm: 500,
        })
        .collect();
    let map = GeologicalMap {
        survey_id: 9001,
        blocks,
    };
    let bytes = encode_to_vec(&map).expect("encode GeologicalMap");
    let (decoded, _): (GeologicalMap, usize) =
        decode_from_slice(&bytes).expect("decode GeologicalMap");
    assert_eq!(map, decoded);
}

/// 9. GeologicalMap file roundtrip
#[test]
fn test_geological_map_file_roundtrip() {
    let path = unique_tmp("geo_map_34.bin");
    let blocks: Vec<OreBlock> = (0u64..5)
        .map(|i| OreBlock {
            block_id: i + 100,
            x: i as f32 * 50.0,
            y: 300.0,
            z: -10.0,
            rock_class: RockClass::Metamorphic,
            tonnage_x100: 75_000,
            grade_ppm: 2_200,
        })
        .collect();
    let map = GeologicalMap {
        survey_id: 7777,
        blocks,
    };
    encode_to_file(&map, &path).expect("encode_to_file GeologicalMap");
    let decoded: GeologicalMap = decode_from_file(&path).expect("decode_from_file GeologicalMap");
    assert_eq!(map, decoded);
    std::fs::remove_file(&path).expect("cleanup geo_map_34.bin");
}

/// 10. EquipmentShift all EquipmentType variants roundtrip
#[test]
fn test_equipment_shift_all_types() {
    let types = [
        EquipmentType::Excavator,
        EquipmentType::Drill,
        EquipmentType::Crusher,
        EquipmentType::Conveyor,
        EquipmentType::HaulTruck,
        EquipmentType::Blaster,
    ];
    for (i, equip_type) in types.into_iter().enumerate() {
        let shift = EquipmentShift {
            shift_id: 3000 + i as u64,
            equipment_id: 10 + i as u32,
            equip_type,
            operator_id: 200 + i as u32,
            hours_worked_x100: 1200 + i as u16 * 50,
            fuel_liters: 800 + i as u32 * 100,
        };
        let bytes = encode_to_vec(&shift).expect("encode EquipmentShift");
        let (decoded, _): (EquipmentShift, usize) =
            decode_from_slice(&bytes).expect("decode EquipmentShift");
        assert_eq!(shift, decoded);
    }
}

/// 11. EquipmentShift file roundtrip — HaulTruck long shift
#[test]
fn test_equipment_shift_haul_truck_file_roundtrip() {
    let path = unique_tmp("haul_truck_shift_34.bin");
    let shift = EquipmentShift {
        shift_id: 99_999,
        equipment_id: 42,
        equip_type: EquipmentType::HaulTruck,
        operator_id: 1337,
        hours_worked_x100: 1200,
        fuel_liters: 3_500,
    };
    encode_to_file(&shift, &path).expect("encode_to_file HaulTruck shift");
    let decoded: EquipmentShift =
        decode_from_file(&path).expect("decode_from_file HaulTruck shift");
    assert_eq!(shift, decoded);
    std::fs::remove_file(&path).expect("cleanup haul_truck_shift_34.bin");
}

/// 12. All ExcavationMethod variants — vec roundtrip
#[test]
fn test_excavation_method_all_variants() {
    let methods = [
        ExcavationMethod::OpenPit,
        ExcavationMethod::Underground,
        ExcavationMethod::Quarrying,
        ExcavationMethod::Dredging,
    ];
    for method in &methods {
        let bytes = encode_to_vec(method).expect("encode ExcavationMethod");
        let (decoded, _): (ExcavationMethod, usize) =
            decode_from_slice(&bytes).expect("decode ExcavationMethod");
        assert_eq!(method, &decoded);
    }
}

/// 13. Bytes match — encode twice must produce identical output
#[test]
fn test_encode_determinism_bytes_match() {
    let blast = BlastDesign {
        blast_id: 123_456,
        bench_id: 7,
        hole_count: 64,
        explosive_kg: 4_800,
        delay_ms: 250,
        timestamp: 1_720_000_000,
    };
    let bytes_a = encode_to_vec(&blast).expect("encode blast first");
    let bytes_b = encode_to_vec(&blast).expect("encode blast second");
    assert_eq!(bytes_a, bytes_b, "encoding must be deterministic");
}

/// 14. File overwrite — second write replaces first
#[test]
fn test_file_overwrite_replaces_previous_content() {
    let path = unique_tmp("overwrite_34.bin");
    let first = AssaySample {
        sample_id: 1,
        hole_id: 10,
        depth_from_m: 0.0,
        depth_to_m: 1.0,
        mineral: MineralType::Coal,
        grade_ppm: 100,
    };
    let second = AssaySample {
        sample_id: 2,
        hole_id: 20,
        depth_from_m: 5.0,
        depth_to_m: 6.0,
        mineral: MineralType::Nickel,
        grade_ppm: 99_000,
    };
    encode_to_file(&first, &path).expect("first write");
    encode_to_file(&second, &path).expect("second write (overwrite)");
    let decoded: AssaySample = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).expect("cleanup overwrite_34.bin");
}

/// 15. Error on missing file
#[test]
fn test_error_on_missing_file() {
    let path = unique_tmp("nonexistent_mining_34.bin");
    // Ensure the file truly does not exist
    let _ = std::fs::remove_file(&path);
    let result: Result<DrillHole, _> = decode_from_file(&path);
    assert!(
        result.is_err(),
        "decode_from_file must fail for missing file"
    );
}

/// 16. Option<MineralType> Some and None roundtrip
#[test]
fn test_option_mineral_type_some_and_none() {
    let some_val: Option<MineralType> = Some(MineralType::Silver);
    let none_val: Option<MineralType> = None;

    let bytes_some = encode_to_vec(&some_val).expect("encode Some mineral");
    let (decoded_some, _): (Option<MineralType>, usize) =
        decode_from_slice(&bytes_some).expect("decode Some mineral");
    assert_eq!(some_val, decoded_some);

    let bytes_none = encode_to_vec(&none_val).expect("encode None mineral");
    let (decoded_none, _): (Option<MineralType>, usize) =
        decode_from_slice(&bytes_none).expect("decode None mineral");
    assert_eq!(none_val, decoded_none);
}

/// 17. Vec of DrillHoles roundtrip via file
#[test]
fn test_vec_drill_holes_file_roundtrip() {
    let path = unique_tmp("drill_holes_vec_34.bin");
    let holes: Vec<DrillHole> = (0u32..30)
        .map(|i| DrillHole {
            hole_id: i,
            collar_x_m: i as f32 * 15.0,
            collar_y_m: i as f32 * 8.5,
            collar_z_m: 400.0 - i as f32 * 2.0,
            depth_m: 200.0 + i as f32 * 5.0,
            azimuth_deg: (i % 360) as f32,
            dip_deg: -45.0 - (i % 30) as f32,
        })
        .collect();
    encode_to_file(&holes, &path).expect("encode_to_file vec DrillHoles");
    let decoded: Vec<DrillHole> = decode_from_file(&path).expect("decode_from_file vec DrillHoles");
    assert_eq!(holes, decoded);
    std::fs::remove_file(&path).expect("cleanup drill_holes_vec_34.bin");
}

/// 18. OreBlock zero-grade boundary values
#[test]
fn test_ore_block_zero_grade_boundary() {
    let block = OreBlock {
        block_id: u64::MAX,
        x: f32::MAX,
        y: f32::MIN_POSITIVE,
        z: 0.0,
        rock_class: RockClass::Sedimentary,
        tonnage_x100: u32::MAX,
        grade_ppm: 0,
    };
    let bytes = encode_to_vec(&block).expect("encode boundary OreBlock");
    let (decoded, _): (OreBlock, usize) =
        decode_from_slice(&bytes).expect("decode boundary OreBlock");
    assert_eq!(block, decoded);
}

/// 19. BlastDesign with maximum field values
#[test]
fn test_blast_design_max_values() {
    let blast = BlastDesign {
        blast_id: u64::MAX,
        bench_id: u32::MAX,
        hole_count: u16::MAX,
        explosive_kg: u32::MAX,
        delay_ms: u32::MAX,
        timestamp: u64::MAX,
    };
    let bytes = encode_to_vec(&blast).expect("encode max BlastDesign");
    let (decoded, _): (BlastDesign, usize) =
        decode_from_slice(&bytes).expect("decode max BlastDesign");
    assert_eq!(blast, decoded);
}

/// 20. Vec<EquipmentShift> large batch file roundtrip
#[test]
fn test_vec_equipment_shifts_large_batch_file() {
    let path = unique_tmp("equipment_shifts_batch_34.bin");
    let equip_types = [
        EquipmentType::Excavator,
        EquipmentType::Drill,
        EquipmentType::Crusher,
        EquipmentType::Conveyor,
        EquipmentType::HaulTruck,
        EquipmentType::Blaster,
    ];
    let shifts: Vec<EquipmentShift> = (0u64..120)
        .map(|i| EquipmentShift {
            shift_id: i,
            equipment_id: (i % 20) as u32,
            equip_type: equip_types[(i as usize) % equip_types.len()].clone(),
            operator_id: (i % 50) as u32 + 1000,
            hours_worked_x100: (800 + i % 400) as u16,
            fuel_liters: (500 + i * 10) as u32 % 10_000,
        })
        .collect();
    encode_to_file(&shifts, &path).expect("encode_to_file EquipmentShift batch");
    let decoded: Vec<EquipmentShift> =
        decode_from_file(&path).expect("decode_from_file EquipmentShift batch");
    assert_eq!(shifts, decoded);
    std::fs::remove_file(&path).expect("cleanup equipment_shifts_batch_34.bin");
}

/// 21. Nested GeologicalMap with mixed RockClass — bytes identity check
#[test]
fn test_geological_map_bytes_identity() {
    let blocks: Vec<OreBlock> = vec![
        OreBlock {
            block_id: 1,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rock_class: RockClass::Ore,
            tonnage_x100: 200_000,
            grade_ppm: 5_000,
        },
        OreBlock {
            block_id: 2,
            x: 50.0,
            y: 50.0,
            z: -20.0,
            rock_class: RockClass::Igneous,
            tonnage_x100: 150_000,
            grade_ppm: 300,
        },
    ];
    let map = GeologicalMap {
        survey_id: 4242,
        blocks,
    };
    let bytes_first = encode_to_vec(&map).expect("encode GeologicalMap first");
    let bytes_second = encode_to_vec(&map).expect("encode GeologicalMap second");
    assert_eq!(
        bytes_first, bytes_second,
        "GeologicalMap encoding must be deterministic"
    );
    let (decoded, consumed): (GeologicalMap, usize) =
        decode_from_slice(&bytes_first).expect("decode GeologicalMap bytes identity");
    assert_eq!(map, decoded);
    assert_eq!(consumed, bytes_first.len());
}

/// 22. Full pipeline — encode struct to file, read raw bytes, decode from slice, verify
#[test]
fn test_full_pipeline_file_bytes_slice_decode() {
    let path = unique_tmp("full_pipeline_34.bin");
    let sample = AssaySample {
        sample_id: 9_999_999,
        hole_id: 888,
        depth_from_m: 300.0,
        depth_to_m: 301.5,
        mineral: MineralType::Zinc,
        grade_ppm: 55_432,
    };
    encode_to_file(&sample, &path).expect("pipeline: encode_to_file");
    let raw_bytes = std::fs::read(&path).expect("pipeline: read raw bytes");
    let (decoded_from_slice, consumed): (AssaySample, usize) =
        decode_from_slice(&raw_bytes).expect("pipeline: decode_from_slice");
    let decoded_from_file: AssaySample =
        decode_from_file(&path).expect("pipeline: decode_from_file");
    assert_eq!(sample, decoded_from_slice);
    assert_eq!(sample, decoded_from_file);
    assert_eq!(consumed, raw_bytes.len());
    std::fs::remove_file(&path).expect("cleanup full_pipeline_34.bin");
}
