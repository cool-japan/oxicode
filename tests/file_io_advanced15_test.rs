//! Advanced file I/O tests — computational physics / simulation domain.
//!
//! 22 `#[test]` functions exercising `encode_to_file` / `decode_from_file`
//! and the slice-based helpers with realistic scientific simulation types.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SimulationType {
    MolecularDynamics,
    FiniteElement,
    MonteCarlo,
    LatticeBoltzmann,
    Dft,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoundaryCondition {
    Periodic,
    Absorbing,
    Reflecting,
    Fixed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Particle {
    id: u64,
    mass: f64,
    charge: f64,
    position: [f64; 3],
    velocity: [f64; 3],
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimulationConfig {
    sim_type: SimulationType,
    dt: f64,
    steps: u64,
    boundary: BoundaryCondition,
    temperature: f64,
    pressure: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SimulationSnapshot {
    step: u64,
    time: f64,
    particles: Vec<Particle>,
    total_energy: f64,
    config: SimulationConfig,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_particle(id: u64) -> Particle {
    Particle {
        id,
        mass: 1.008 * id as f64,
        charge: -1.602e-19 * (id as f64 % 3.0 - 1.0),
        position: [id as f64 * 0.1, id as f64 * 0.2, id as f64 * 0.3],
        velocity: [0.01 * id as f64, -0.02 * id as f64, 0.005 * id as f64],
    }
}

fn make_md_config() -> SimulationConfig {
    SimulationConfig {
        sim_type: SimulationType::MolecularDynamics,
        dt: 1.0e-15,
        steps: 100_000,
        boundary: BoundaryCondition::Periodic,
        temperature: 300.0,
        pressure: 101_325.0,
    }
}

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("oxicode_phys_{}", name))
}

// ---------------------------------------------------------------------------
// 1. Particle struct — write and read back from file
// ---------------------------------------------------------------------------

#[test]
fn test_particle_file_roundtrip() {
    let path = temp_path("particle_roundtrip.bin");
    let particle = make_particle(42);

    encode_to_file(&particle, &path).expect("encode Particle to file");
    let decoded: Particle = decode_from_file(&path).expect("decode Particle from file");

    assert_eq!(particle, decoded);
    std::fs::remove_file(&path).expect("cleanup particle_roundtrip.bin");
}

// ---------------------------------------------------------------------------
// 2. SimulationConfig — standard config roundtrip via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_config_file_roundtrip() {
    let path = temp_path("sim_config.bin");
    let config = make_md_config();

    encode_to_file(&config, &path).expect("encode SimulationConfig");
    let decoded: SimulationConfig = decode_from_file(&path).expect("decode SimulationConfig");

    assert_eq!(config, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_config.bin");
}

// ---------------------------------------------------------------------------
// 3. SimulationType::MolecularDynamics variant via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_type_molecular_dynamics_file() {
    let path = temp_path("sim_type_md.bin");
    let sim_type = SimulationType::MolecularDynamics;

    encode_to_file(&sim_type, &path).expect("encode SimulationType::MolecularDynamics");
    let decoded: SimulationType =
        decode_from_file(&path).expect("decode SimulationType::MolecularDynamics");

    assert_eq!(sim_type, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_type_md.bin");
}

// ---------------------------------------------------------------------------
// 4. SimulationType::FiniteElement variant via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_type_finite_element_file() {
    let path = temp_path("sim_type_fe.bin");
    let sim_type = SimulationType::FiniteElement;

    encode_to_file(&sim_type, &path).expect("encode SimulationType::FiniteElement");
    let decoded: SimulationType =
        decode_from_file(&path).expect("decode SimulationType::FiniteElement");

    assert_eq!(sim_type, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_type_fe.bin");
}

// ---------------------------------------------------------------------------
// 5. SimulationType::MonteCarlo variant via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_type_monte_carlo_file() {
    let path = temp_path("sim_type_mc.bin");
    let sim_type = SimulationType::MonteCarlo;

    encode_to_file(&sim_type, &path).expect("encode SimulationType::MonteCarlo");
    let decoded: SimulationType =
        decode_from_file(&path).expect("decode SimulationType::MonteCarlo");

    assert_eq!(sim_type, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_type_mc.bin");
}

// ---------------------------------------------------------------------------
// 6. SimulationType::LatticeBoltzmann via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_type_lattice_boltzmann_file() {
    let path = temp_path("sim_type_lb.bin");
    let sim_type = SimulationType::LatticeBoltzmann;

    encode_to_file(&sim_type, &path).expect("encode SimulationType::LatticeBoltzmann");
    let decoded: SimulationType =
        decode_from_file(&path).expect("decode SimulationType::LatticeBoltzmann");

    assert_eq!(sim_type, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_type_lb.bin");
}

// ---------------------------------------------------------------------------
// 7. SimulationType::Dft via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_type_dft_file() {
    let path = temp_path("sim_type_dft.bin");
    let sim_type = SimulationType::Dft;

    encode_to_file(&sim_type, &path).expect("encode SimulationType::Dft");
    let decoded: SimulationType = decode_from_file(&path).expect("decode SimulationType::Dft");

    assert_eq!(sim_type, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_type_dft.bin");
}

// ---------------------------------------------------------------------------
// 8. BoundaryCondition — all four variants via file
// ---------------------------------------------------------------------------

#[test]
fn test_boundary_conditions_all_variants_file() {
    let variants = [
        (BoundaryCondition::Periodic, "bc_periodic.bin"),
        (BoundaryCondition::Absorbing, "bc_absorbing.bin"),
        (BoundaryCondition::Reflecting, "bc_reflecting.bin"),
        (BoundaryCondition::Fixed, "bc_fixed.bin"),
    ];

    for (bc, name) in &variants {
        let path = temp_path(name);
        encode_to_file(bc, &path).expect("encode BoundaryCondition");
        let decoded: BoundaryCondition = decode_from_file(&path).expect("decode BoundaryCondition");
        assert_eq!(*bc, decoded, "BoundaryCondition mismatch for {}", name);
        std::fs::remove_file(&path).expect("cleanup boundary condition file");
    }
}

// ---------------------------------------------------------------------------
// 9. SimulationConfig with DFT + Absorbing boundary via file
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_config_dft_absorbing_file() {
    let path = temp_path("sim_config_dft.bin");
    let config = SimulationConfig {
        sim_type: SimulationType::Dft,
        dt: 5.0e-18,
        steps: 500,
        boundary: BoundaryCondition::Absorbing,
        temperature: 0.0,
        pressure: 0.0,
    };

    encode_to_file(&config, &path).expect("encode DFT config");
    let decoded: SimulationConfig = decode_from_file(&path).expect("decode DFT config");

    assert_eq!(config, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_config_dft.bin");
}

// ---------------------------------------------------------------------------
// 10. SimulationSnapshot with a small particle array via file
// ---------------------------------------------------------------------------

#[test]
fn test_snapshot_small_particle_array_file() {
    let path = temp_path("snapshot_small.bin");
    let config = make_md_config();
    let particles: Vec<Particle> = (0..10).map(make_particle).collect();
    let snapshot = SimulationSnapshot {
        step: 1000,
        time: 1.0e-12,
        particles,
        total_energy: -4523.7,
        config,
    };

    encode_to_file(&snapshot, &path).expect("encode small snapshot");
    let decoded: SimulationSnapshot = decode_from_file(&path).expect("decode small snapshot");

    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).expect("cleanup snapshot_small.bin");
}

// ---------------------------------------------------------------------------
// 11. SimulationSnapshot with a large particle array (1 000 particles)
// ---------------------------------------------------------------------------

#[test]
fn test_snapshot_large_particle_array_file() {
    let path = temp_path("snapshot_large.bin");
    let config = SimulationConfig {
        sim_type: SimulationType::MonteCarlo,
        dt: 1.0e-14,
        steps: 1_000_000,
        boundary: BoundaryCondition::Reflecting,
        temperature: 77.0,
        pressure: 50_000.0,
    };
    let particles: Vec<Particle> = (0..1000).map(make_particle).collect();
    let snapshot = SimulationSnapshot {
        step: 500_000,
        time: 5.0e-9,
        particles,
        total_energy: -1_234_567.89,
        config,
    };

    encode_to_file(&snapshot, &path).expect("encode large snapshot");
    let decoded: SimulationSnapshot = decode_from_file(&path).expect("decode large snapshot");

    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).expect("cleanup snapshot_large.bin");
}

// ---------------------------------------------------------------------------
// 12. Vec<Particle> serialised to a file
// ---------------------------------------------------------------------------

#[test]
fn test_vec_particles_file_roundtrip() {
    let path = temp_path("vec_particles.bin");
    let particles: Vec<Particle> = (0..256).map(make_particle).collect();

    encode_to_file(&particles, &path).expect("encode Vec<Particle>");
    let decoded: Vec<Particle> = decode_from_file(&path).expect("decode Vec<Particle>");

    assert_eq!(particles, decoded);
    std::fs::remove_file(&path).expect("cleanup vec_particles.bin");
}

// ---------------------------------------------------------------------------
// 13. Particle roundtrip via encode_to_vec / decode_from_slice
// ---------------------------------------------------------------------------

#[test]
fn test_particle_slice_roundtrip() {
    let particle = make_particle(7);

    let bytes = encode_to_vec(&particle).expect("encode Particle to vec");
    let (decoded, consumed): (Particle, usize) =
        decode_from_slice(&bytes).expect("decode Particle from slice");

    assert_eq!(particle, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// 14. SimulationConfig roundtrip via encode_to_vec / decode_from_slice
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_config_slice_roundtrip() {
    let config = SimulationConfig {
        sim_type: SimulationType::LatticeBoltzmann,
        dt: 1.0e-6,
        steps: 2048,
        boundary: BoundaryCondition::Periodic,
        temperature: 293.15,
        pressure: 101_325.0,
    };

    let bytes = encode_to_vec(&config).expect("encode SimulationConfig to vec");
    let (decoded, consumed): (SimulationConfig, usize) =
        decode_from_slice(&bytes).expect("decode SimulationConfig from slice");

    assert_eq!(config, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 15. SimulationSnapshot with zero particles (edge case)
// ---------------------------------------------------------------------------

#[test]
fn test_snapshot_empty_particles_file() {
    let path = temp_path("snapshot_empty.bin");
    let config = SimulationConfig {
        sim_type: SimulationType::FiniteElement,
        dt: 1.0e-3,
        steps: 0,
        boundary: BoundaryCondition::Fixed,
        temperature: 0.0,
        pressure: 0.0,
    };
    let snapshot = SimulationSnapshot {
        step: 0,
        time: 0.0,
        particles: vec![],
        total_energy: 0.0,
        config,
    };

    encode_to_file(&snapshot, &path).expect("encode empty snapshot");
    let decoded: SimulationSnapshot = decode_from_file(&path).expect("decode empty snapshot");

    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).expect("cleanup snapshot_empty.bin");
}

// ---------------------------------------------------------------------------
// 16. Multiple snapshots written to separate files and read back
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_snapshots_separate_files() {
    let configs = [
        SimulationConfig {
            sim_type: SimulationType::MolecularDynamics,
            dt: 1.0e-15,
            steps: 10_000,
            boundary: BoundaryCondition::Periodic,
            temperature: 300.0,
            pressure: 101_325.0,
        },
        SimulationConfig {
            sim_type: SimulationType::FiniteElement,
            dt: 1.0e-4,
            steps: 50,
            boundary: BoundaryCondition::Fixed,
            temperature: 800.0,
            pressure: 200_000.0,
        },
    ];

    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    let mut originals: Vec<SimulationSnapshot> = Vec::new();

    for (idx, cfg) in configs.iter().enumerate() {
        let path = temp_path(&format!("multi_snap_{}.bin", idx));
        let snap = SimulationSnapshot {
            step: idx as u64 * 500,
            time: idx as f64 * 0.5e-12,
            particles: (0..5).map(make_particle).collect(),
            total_energy: -(idx as f64 + 1.0) * 100.0,
            config: cfg.clone(),
        };
        encode_to_file(&snap, &path).expect("encode snapshot");
        paths.push(path);
        originals.push(snap);
    }

    for (original, path) in originals.iter().zip(paths.iter()) {
        let decoded: SimulationSnapshot = decode_from_file(path).expect("decode snapshot");
        assert_eq!(*original, decoded);
        std::fs::remove_file(path).expect("cleanup multi_snap file");
    }
}

// ---------------------------------------------------------------------------
// 17. Particle with extreme float values (NaN-free, inf-free boundary values)
// ---------------------------------------------------------------------------

#[test]
fn test_particle_extreme_float_values_file() {
    let path = temp_path("particle_extreme.bin");
    let particle = Particle {
        id: u64::MAX,
        mass: f64::MAX,
        charge: f64::MIN_POSITIVE,
        position: [f64::MAX, f64::MIN_POSITIVE, 0.0],
        velocity: [-f64::MAX / 2.0, f64::MIN_POSITIVE, 1.0e-300],
    };

    encode_to_file(&particle, &path).expect("encode extreme Particle");
    let decoded: Particle = decode_from_file(&path).expect("decode extreme Particle");

    assert_eq!(particle, decoded);
    std::fs::remove_file(&path).expect("cleanup particle_extreme.bin");
}

// ---------------------------------------------------------------------------
// 18. SimulationConfig with maximum step count and minimum dt
// ---------------------------------------------------------------------------

#[test]
fn test_simulation_config_extreme_parameters_file() {
    let path = temp_path("sim_config_extreme.bin");
    let config = SimulationConfig {
        sim_type: SimulationType::MonteCarlo,
        dt: f64::MIN_POSITIVE,
        steps: u64::MAX,
        boundary: BoundaryCondition::Absorbing,
        temperature: f64::MAX,
        pressure: f64::MIN_POSITIVE,
    };

    encode_to_file(&config, &path).expect("encode extreme SimulationConfig");
    let decoded: SimulationConfig =
        decode_from_file(&path).expect("decode extreme SimulationConfig");

    assert_eq!(config, decoded);
    std::fs::remove_file(&path).expect("cleanup sim_config_extreme.bin");
}

// ---------------------------------------------------------------------------
// 19. Snapshot with many particles — encoded size consistency check
// ---------------------------------------------------------------------------

#[test]
fn test_snapshot_encoded_size_consistency() {
    let config = make_md_config();
    let particles: Vec<Particle> = (0..200).map(make_particle).collect();
    let snapshot = SimulationSnapshot {
        step: 200,
        time: 2.0e-13,
        particles,
        total_energy: -9876.54,
        config,
    };

    let bytes = encode_to_vec(&snapshot).expect("encode snapshot to vec");
    let predicted_size = oxicode::encoded_size(&snapshot).expect("encoded_size snapshot");

    assert_eq!(
        predicted_size,
        bytes.len(),
        "encoded_size must match actual byte length"
    );

    let (decoded, consumed): (SimulationSnapshot, usize) =
        decode_from_slice(&bytes).expect("decode snapshot from slice");
    assert_eq!(snapshot, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes consumed");
}

// ---------------------------------------------------------------------------
// 20. Re-encode decoded snapshot yields identical bytes (idempotency)
// ---------------------------------------------------------------------------

#[test]
fn test_snapshot_encode_decode_encode_idempotency() {
    let config = SimulationConfig {
        sim_type: SimulationType::Dft,
        dt: 2.5e-17,
        steps: 64,
        boundary: BoundaryCondition::Reflecting,
        temperature: 10.0,
        pressure: 1.0,
    };
    let original = SimulationSnapshot {
        step: 32,
        time: 8.0e-16,
        particles: (0..8).map(make_particle).collect(),
        total_energy: -42.0,
        config,
    };

    let bytes1 = encode_to_vec(&original).expect("encode pass 1");
    let (decoded, _): (SimulationSnapshot, usize) =
        decode_from_slice(&bytes1).expect("decode pass 1");
    let bytes2 = encode_to_vec(&decoded).expect("encode pass 2");

    assert_eq!(
        bytes1, bytes2,
        "re-encoding a decoded snapshot must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 21. decode_from_file on a nonexistent path must return Err, not panic
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_nonexistent_file_returns_error() {
    let path = temp_path("does_not_exist_xyz_99999.bin");
    // Ensure the file really does not exist
    let _ = std::fs::remove_file(&path);

    let result: Result<Particle, _> = decode_from_file(&path);
    assert!(
        result.is_err(),
        "decode_from_file on a nonexistent path must return Err"
    );
}

// ---------------------------------------------------------------------------
// 22. Temp file cleanup — verify the file is gone after manual removal
// ---------------------------------------------------------------------------

#[test]
fn test_temp_file_cleanup_verified() {
    let path = temp_path("cleanup_check.bin");
    let particle = make_particle(1);

    encode_to_file(&particle, &path).expect("encode for cleanup test");
    assert!(path.exists(), "file must exist after encode_to_file");

    std::fs::remove_file(&path).expect("explicit cleanup");
    assert!(
        !path.exists(),
        "file must not exist after std::fs::remove_file"
    );

    // Confirm decode on the removed file fails
    let result: Result<Particle, _> = decode_from_file(&path);
    assert!(result.is_err(), "decode after file removal must return Err");
}
