#![cfg(feature = "compression-lz4")]

//! Advanced LZ4 compression tests for the computational fluid dynamics (CFD) domain.
//!
//! Covers mesh cell data, velocity fields, pressure snapshots, turbulence models,
//! boundary conditions, Reynolds number regimes, convergence residuals, Navier-Stokes
//! solver states, vortex shedding, drag/lift coefficients, adaptive mesh refinement,
//! and multiphase flow volume fractions.

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MeshVertex {
    id: u64,
    position: Vec3,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FaceNormal {
    face_id: u64,
    normal: Vec3,
    area: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MeshCell {
    cell_id: u64,
    vertices: Vec<MeshVertex>,
    face_normals: Vec<FaceNormal>,
    volume: f64,
    centroid: Vec3,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VelocityFieldPoint {
    cell_id: u64,
    velocity: Vec3,
    magnitude: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VelocityFieldSnapshot {
    time_step: u64,
    time_seconds: f64,
    points: Vec<VelocityFieldPoint>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PressureFieldSnapshot {
    time_step: u64,
    time_seconds: f64,
    cell_pressures: Vec<f64>,
    min_pressure: f64,
    max_pressure: f64,
    mean_pressure: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KEpsilonParams {
    k: f64,
    epsilon: f64,
    c_mu: f64,
    c_1: f64,
    c_2: f64,
    sigma_k: f64,
    sigma_epsilon: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KOmegaSstParams {
    k: f64,
    omega: f64,
    alpha_1: f64,
    alpha_2: f64,
    beta_1: f64,
    beta_2: f64,
    beta_star: f64,
    sigma_k1: f64,
    sigma_k2: f64,
    sigma_omega1: f64,
    sigma_omega2: f64,
    a1: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TurbulenceModel {
    Laminar,
    KEpsilon(KEpsilonParams),
    KOmegaSst(KOmegaSstParams),
    SpalartAllmaras {
        nu_tilde: f64,
        cb1: f64,
        cb2: f64,
        sigma: f64,
    },
    LargeEddySimulation {
        smagorinsky_constant: f64,
        filter_width: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BoundaryConditionType {
    VelocityInlet {
        velocity: Vec3,
    },
    PressureOutlet {
        gauge_pressure: f64,
    },
    Wall {
        no_slip: bool,
        roughness_height: f64,
    },
    Symmetry,
    Periodic {
        translation: Vec3,
    },
    FarField {
        mach_number: f64,
        angle_of_attack: f64,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundaryCondition {
    zone_name: String,
    zone_id: u32,
    face_count: u64,
    condition: BoundaryConditionType,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum FlowRegime {
    Creeping,
    Laminar,
    Transitional { critical_re: f64 },
    TurbulentSmooth,
    TurbulentRough { relative_roughness: f64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReynoldsNumberAnalysis {
    characteristic_length: f64,
    reference_velocity: f64,
    kinematic_viscosity: f64,
    reynolds_number: f64,
    regime: FlowRegime,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConvergenceResidual {
    iteration: u64,
    continuity: f64,
    x_momentum: f64,
    y_momentum: f64,
    z_momentum: f64,
    energy: f64,
    turbulence_k: f64,
    turbulence_secondary: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConvergenceHistory {
    solver_name: String,
    target_residual: f64,
    converged: bool,
    residuals: Vec<ConvergenceResidual>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SolverScheme {
    Simple,
    Simplec,
    Piso { corrector_steps: u8 },
    Coupled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DiscretizationScheme {
    FirstOrderUpwind,
    SecondOrderUpwind,
    CentralDifference,
    Quick,
    Muscl,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NavierStokesSolverState {
    iteration: u64,
    time_seconds: f64,
    dt: f64,
    cfl_number: f64,
    solver_scheme: SolverScheme,
    momentum_discretization: DiscretizationScheme,
    pressure_discretization: DiscretizationScheme,
    under_relaxation_pressure: f64,
    under_relaxation_momentum: f64,
    converged: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VortexSheddingData {
    strouhal_number: f64,
    shedding_frequency_hz: f64,
    cylinder_diameter: f64,
    freestream_velocity: f64,
    amplitude_history: Vec<f64>,
    phase_angles: Vec<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AerodynamicCoefficient {
    time_seconds: f64,
    cd: f64,
    cl: f64,
    cm: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DragLiftTimeSeries {
    body_name: String,
    reference_area: f64,
    reference_length: f64,
    dynamic_pressure: f64,
    coefficients: Vec<AerodynamicCoefficient>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AmrCell {
    cell_id: u64,
    refinement_level: u8,
    parent_id: Option<u64>,
    children_ids: Vec<u64>,
    error_indicator: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdaptiveMeshRefinement {
    max_level: u8,
    refinement_threshold: f64,
    coarsening_threshold: f64,
    total_cells: u64,
    cells: Vec<AmrCell>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiphaseVof {
    cell_id: u64,
    volume_fraction_liquid: f64,
    volume_fraction_gas: f64,
    interface_normal: Option<Vec3>,
    curvature: Option<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiphaseFlowSnapshot {
    time_step: u64,
    time_seconds: f64,
    surface_tension_coefficient: f64,
    gravity: Vec3,
    cells: Vec<MultiphaseVof>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TemperatureFieldPoint {
    cell_id: u64,
    temperature_k: f64,
    heat_flux: Vec3,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConjugateHeatTransfer {
    fluid_zone: String,
    solid_zone: String,
    interface_area: f64,
    fluid_points: Vec<TemperatureFieldPoint>,
    solid_points: Vec<TemperatureFieldPoint>,
    nusselt_number: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WallShearStress {
    face_id: u64,
    tau_wall: Vec3,
    y_plus: f64,
    friction_velocity: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundaryLayerProfile {
    station_name: String,
    wall_distance: Vec<f64>,
    velocity_parallel: Vec<f64>,
    velocity_normal: Vec<f64>,
    turbulent_kinetic_energy: Vec<f64>,
    displacement_thickness: f64,
    momentum_thickness: f64,
    shape_factor: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SpeciesTransport {
    species_name: String,
    molecular_weight: f64,
    diffusion_coefficient: f64,
    mass_fractions: Vec<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CombustionState {
    mixture_fraction: f64,
    scalar_dissipation_rate: f64,
    temperature_k: f64,
    species: Vec<SpeciesTransport>,
}

// ---------------------------------------------------------------------------
// Test 1 – Mesh cell with vertices and face normals
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_mesh_cell_roundtrip() {
    let cell = MeshCell {
        cell_id: 42_000,
        vertices: vec![
            MeshVertex {
                id: 1,
                position: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            MeshVertex {
                id: 2,
                position: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            MeshVertex {
                id: 3,
                position: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            MeshVertex {
                id: 4,
                position: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            MeshVertex {
                id: 5,
                position: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            MeshVertex {
                id: 6,
                position: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            MeshVertex {
                id: 7,
                position: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            MeshVertex {
                id: 8,
                position: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
        ],
        face_normals: vec![
            FaceNormal {
                face_id: 100,
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                area: 1.0,
            },
            FaceNormal {
                face_id: 101,
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                area: 1.0,
            },
            FaceNormal {
                face_id: 102,
                normal: Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                area: 1.0,
            },
            FaceNormal {
                face_id: 103,
                normal: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                area: 1.0,
            },
            FaceNormal {
                face_id: 104,
                normal: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                area: 1.0,
            },
            FaceNormal {
                face_id: 105,
                normal: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                area: 1.0,
            },
        ],
        volume: 1.0,
        centroid: Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    };

    let encoded = encode_to_vec(&cell).expect("encode MeshCell failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress MeshCell failed");
    let decompressed = decompress(&compressed).expect("decompress MeshCell failed");
    let (decoded, _): (MeshCell, usize) =
        decode_from_slice(&decompressed).expect("decode MeshCell failed");

    assert_eq!(cell, decoded);
}

// ---------------------------------------------------------------------------
// Test 2 – Velocity field snapshot
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_velocity_field_snapshot_roundtrip() {
    let snapshot = VelocityFieldSnapshot {
        time_step: 5000,
        time_seconds: 0.025,
        points: (0..50)
            .map(|i| {
                let u = 10.0 + 0.1 * (i as f64);
                let v = 0.05 * (i as f64);
                let w = -0.01 * (i as f64);
                VelocityFieldPoint {
                    cell_id: i as u64,
                    velocity: Vec3 { x: u, y: v, z: w },
                    magnitude: (u * u + v * v + w * w).sqrt(),
                }
            })
            .collect(),
    };

    let encoded = encode_to_vec(&snapshot).expect("encode VelocityFieldSnapshot failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress VelocityFieldSnapshot failed");
    let decompressed = decompress(&compressed).expect("decompress VelocityFieldSnapshot failed");
    let (decoded, _): (VelocityFieldSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode VelocityFieldSnapshot failed");

    assert_eq!(snapshot, decoded);
}

// ---------------------------------------------------------------------------
// Test 3 – Pressure field snapshot with statistics
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_pressure_field_snapshot_roundtrip() {
    let pressures: Vec<f64> = (0..100).map(|i| 101325.0 + 50.0 * (i as f64)).collect();
    let min_p = pressures.iter().copied().fold(f64::INFINITY, f64::min);
    let max_p = pressures.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean_p = pressures.iter().sum::<f64>() / pressures.len() as f64;

    let snapshot = PressureFieldSnapshot {
        time_step: 12_000,
        time_seconds: 0.06,
        cell_pressures: pressures,
        min_pressure: min_p,
        max_pressure: max_p,
        mean_pressure: mean_p,
    };

    let encoded = encode_to_vec(&snapshot).expect("encode PressureFieldSnapshot failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress PressureFieldSnapshot failed");
    let decompressed = decompress(&compressed).expect("decompress PressureFieldSnapshot failed");
    let (decoded, _): (PressureFieldSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode PressureFieldSnapshot failed");

    assert_eq!(snapshot, decoded);
}

// ---------------------------------------------------------------------------
// Test 4 – k-epsilon turbulence model
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_k_epsilon_turbulence_roundtrip() {
    let model = TurbulenceModel::KEpsilon(KEpsilonParams {
        k: 3.75,
        epsilon: 0.41,
        c_mu: 0.09,
        c_1: 1.44,
        c_2: 1.92,
        sigma_k: 1.0,
        sigma_epsilon: 1.3,
    });

    let encoded = encode_to_vec(&model).expect("encode k-epsilon TurbulenceModel failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress k-epsilon TurbulenceModel failed");
    let decompressed =
        decompress(&compressed).expect("decompress k-epsilon TurbulenceModel failed");
    let (decoded, _): (TurbulenceModel, usize) =
        decode_from_slice(&decompressed).expect("decode k-epsilon TurbulenceModel failed");

    assert_eq!(model, decoded);
}

// ---------------------------------------------------------------------------
// Test 5 – k-omega SST turbulence model
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_k_omega_sst_turbulence_roundtrip() {
    let model = TurbulenceModel::KOmegaSst(KOmegaSstParams {
        k: 1.5,
        omega: 300.0,
        alpha_1: 0.5532,
        alpha_2: 0.4403,
        beta_1: 0.075,
        beta_2: 0.0828,
        beta_star: 0.09,
        sigma_k1: 0.85,
        sigma_k2: 1.0,
        sigma_omega1: 0.5,
        sigma_omega2: 0.856,
        a1: 0.31,
    });

    let encoded = encode_to_vec(&model).expect("encode k-omega SST TurbulenceModel failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress k-omega SST failed");
    let decompressed = decompress(&compressed).expect("decompress k-omega SST failed");
    let (decoded, _): (TurbulenceModel, usize) =
        decode_from_slice(&decompressed).expect("decode k-omega SST TurbulenceModel failed");

    assert_eq!(model, decoded);
}

// ---------------------------------------------------------------------------
// Test 6 – Boundary conditions (multiple types)
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_boundary_conditions_roundtrip() {
    let bcs = vec![
        BoundaryCondition {
            zone_name: "inlet".to_string(),
            zone_id: 1,
            face_count: 2400,
            condition: BoundaryConditionType::VelocityInlet {
                velocity: Vec3 {
                    x: 15.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        },
        BoundaryCondition {
            zone_name: "outlet".to_string(),
            zone_id: 2,
            face_count: 2400,
            condition: BoundaryConditionType::PressureOutlet {
                gauge_pressure: 0.0,
            },
        },
        BoundaryCondition {
            zone_name: "airfoil-surface".to_string(),
            zone_id: 3,
            face_count: 18_000,
            condition: BoundaryConditionType::Wall {
                no_slip: true,
                roughness_height: 0.0,
            },
        },
        BoundaryCondition {
            zone_name: "midplane".to_string(),
            zone_id: 4,
            face_count: 5000,
            condition: BoundaryConditionType::Symmetry,
        },
        BoundaryCondition {
            zone_name: "far-field".to_string(),
            zone_id: 5,
            face_count: 8000,
            condition: BoundaryConditionType::FarField {
                mach_number: 0.72,
                angle_of_attack: 2.5,
            },
        },
    ];

    let encoded = encode_to_vec(&bcs).expect("encode BoundaryConditions failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BoundaryConditions failed");
    let decompressed = decompress(&compressed).expect("decompress BoundaryConditions failed");
    let (decoded, _): (Vec<BoundaryCondition>, usize) =
        decode_from_slice(&decompressed).expect("decode BoundaryConditions failed");

    assert_eq!(bcs, decoded);
}

// ---------------------------------------------------------------------------
// Test 7 – Reynolds number regimes
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_reynolds_number_analysis_roundtrip() {
    let analyses = vec![
        ReynoldsNumberAnalysis {
            characteristic_length: 0.001,
            reference_velocity: 0.01,
            kinematic_viscosity: 1.004e-6,
            reynolds_number: 9.96,
            regime: FlowRegime::Creeping,
        },
        ReynoldsNumberAnalysis {
            characteristic_length: 0.05,
            reference_velocity: 0.5,
            kinematic_viscosity: 1.004e-6,
            reynolds_number: 24_900.0,
            regime: FlowRegime::Transitional { critical_re: 5e5 },
        },
        ReynoldsNumberAnalysis {
            characteristic_length: 1.0,
            reference_velocity: 30.0,
            kinematic_viscosity: 1.51e-5,
            reynolds_number: 1.987e6,
            regime: FlowRegime::TurbulentSmooth,
        },
        ReynoldsNumberAnalysis {
            characteristic_length: 2.0,
            reference_velocity: 50.0,
            kinematic_viscosity: 1.51e-5,
            reynolds_number: 6.623e6,
            regime: FlowRegime::TurbulentRough {
                relative_roughness: 0.002,
            },
        },
    ];

    let encoded = encode_to_vec(&analyses).expect("encode ReynoldsNumberAnalysis failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ReynoldsNumberAnalysis failed");
    let decompressed = decompress(&compressed).expect("decompress ReynoldsNumberAnalysis failed");
    let (decoded, _): (Vec<ReynoldsNumberAnalysis>, usize) =
        decode_from_slice(&decompressed).expect("decode ReynoldsNumberAnalysis failed");

    assert_eq!(analyses, decoded);
}

// ---------------------------------------------------------------------------
// Test 8 – Convergence residual history
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_convergence_history_roundtrip() {
    let residuals: Vec<ConvergenceResidual> = (0..200)
        .map(|i| {
            let decay = (-0.015 * i as f64).exp();
            ConvergenceResidual {
                iteration: i as u64,
                continuity: 1e-1 * decay,
                x_momentum: 5e-2 * decay,
                y_momentum: 3e-2 * decay,
                z_momentum: 2e-2 * decay,
                energy: 1e-3 * decay,
                turbulence_k: 8e-2 * decay,
                turbulence_secondary: 6e-2 * decay,
            }
        })
        .collect();

    let history = ConvergenceHistory {
        solver_name: "SIMPLE pressure-velocity coupling".to_string(),
        target_residual: 1e-6,
        converged: false,
        residuals,
    };

    let encoded = encode_to_vec(&history).expect("encode ConvergenceHistory failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ConvergenceHistory failed");
    let decompressed = decompress(&compressed).expect("decompress ConvergenceHistory failed");
    let (decoded, _): (ConvergenceHistory, usize) =
        decode_from_slice(&decompressed).expect("decode ConvergenceHistory failed");

    assert_eq!(history, decoded);
}

// ---------------------------------------------------------------------------
// Test 9 – Navier-Stokes solver state
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_navier_stokes_solver_state_roundtrip() {
    let state = NavierStokesSolverState {
        iteration: 150_000,
        time_seconds: 0.75,
        dt: 5e-6,
        cfl_number: 0.85,
        solver_scheme: SolverScheme::Piso { corrector_steps: 2 },
        momentum_discretization: DiscretizationScheme::SecondOrderUpwind,
        pressure_discretization: DiscretizationScheme::SecondOrderUpwind,
        under_relaxation_pressure: 0.3,
        under_relaxation_momentum: 0.7,
        converged: false,
    };

    let encoded = encode_to_vec(&state).expect("encode NavierStokesSolverState failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress NavierStokesSolverState failed");
    let decompressed = decompress(&compressed).expect("decompress NavierStokesSolverState failed");
    let (decoded, _): (NavierStokesSolverState, usize) =
        decode_from_slice(&decompressed).expect("decode NavierStokesSolverState failed");

    assert_eq!(state, decoded);
}

// ---------------------------------------------------------------------------
// Test 10 – Vortex shedding frequency data
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_vortex_shedding_roundtrip() {
    let shedding = VortexSheddingData {
        strouhal_number: 0.198,
        shedding_frequency_hz: 14.85,
        cylinder_diameter: 0.02,
        freestream_velocity: 1.5,
        amplitude_history: (0..80)
            .map(|i| 0.5 * (2.0 * std::f64::consts::PI * 14.85 * i as f64 * 0.001).sin())
            .collect(),
        phase_angles: (0..80)
            .map(|i| {
                (2.0 * std::f64::consts::PI * 14.85 * i as f64 * 0.001)
                    % (2.0 * std::f64::consts::PI)
            })
            .collect(),
    };

    let encoded = encode_to_vec(&shedding).expect("encode VortexSheddingData failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress VortexSheddingData failed");
    let decompressed = decompress(&compressed).expect("decompress VortexSheddingData failed");
    let (decoded, _): (VortexSheddingData, usize) =
        decode_from_slice(&decompressed).expect("decode VortexSheddingData failed");

    assert_eq!(shedding, decoded);
}

// ---------------------------------------------------------------------------
// Test 11 – Drag/lift coefficient time series
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_drag_lift_time_series_roundtrip() {
    let series = DragLiftTimeSeries {
        body_name: "NACA-0012 airfoil".to_string(),
        reference_area: 0.3048,
        reference_length: 0.3048,
        dynamic_pressure: 612.5,
        coefficients: (0..120)
            .map(|i| {
                let t = i as f64 * 0.001;
                AerodynamicCoefficient {
                    time_seconds: t,
                    cd: 0.012 + 0.0005 * (50.0 * t).sin(),
                    cl: 0.45 + 0.02 * (50.0 * t).sin(),
                    cm: -0.05 + 0.003 * (50.0 * t).cos(),
                }
            })
            .collect(),
    };

    let encoded = encode_to_vec(&series).expect("encode DragLiftTimeSeries failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress DragLiftTimeSeries failed");
    let decompressed = decompress(&compressed).expect("decompress DragLiftTimeSeries failed");
    let (decoded, _): (DragLiftTimeSeries, usize) =
        decode_from_slice(&decompressed).expect("decode DragLiftTimeSeries failed");

    assert_eq!(series, decoded);
}

// ---------------------------------------------------------------------------
// Test 12 – Adaptive mesh refinement
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_adaptive_mesh_refinement_roundtrip() {
    let amr = AdaptiveMeshRefinement {
        max_level: 5,
        refinement_threshold: 0.01,
        coarsening_threshold: 0.001,
        total_cells: 12,
        cells: vec![
            AmrCell {
                cell_id: 0,
                refinement_level: 0,
                parent_id: None,
                children_ids: vec![1, 2, 3, 4],
                error_indicator: 0.05,
            },
            AmrCell {
                cell_id: 1,
                refinement_level: 1,
                parent_id: Some(0),
                children_ids: vec![5, 6, 7, 8],
                error_indicator: 0.03,
            },
            AmrCell {
                cell_id: 2,
                refinement_level: 1,
                parent_id: Some(0),
                children_ids: vec![],
                error_indicator: 0.008,
            },
            AmrCell {
                cell_id: 3,
                refinement_level: 1,
                parent_id: Some(0),
                children_ids: vec![],
                error_indicator: 0.005,
            },
            AmrCell {
                cell_id: 4,
                refinement_level: 1,
                parent_id: Some(0),
                children_ids: vec![],
                error_indicator: 0.007,
            },
            AmrCell {
                cell_id: 5,
                refinement_level: 2,
                parent_id: Some(1),
                children_ids: vec![],
                error_indicator: 0.015,
            },
            AmrCell {
                cell_id: 6,
                refinement_level: 2,
                parent_id: Some(1),
                children_ids: vec![],
                error_indicator: 0.012,
            },
            AmrCell {
                cell_id: 7,
                refinement_level: 2,
                parent_id: Some(1),
                children_ids: vec![],
                error_indicator: 0.009,
            },
            AmrCell {
                cell_id: 8,
                refinement_level: 2,
                parent_id: Some(1),
                children_ids: vec![9, 10, 11],
                error_indicator: 0.025,
            },
            AmrCell {
                cell_id: 9,
                refinement_level: 3,
                parent_id: Some(8),
                children_ids: vec![],
                error_indicator: 0.004,
            },
            AmrCell {
                cell_id: 10,
                refinement_level: 3,
                parent_id: Some(8),
                children_ids: vec![],
                error_indicator: 0.006,
            },
            AmrCell {
                cell_id: 11,
                refinement_level: 3,
                parent_id: Some(8),
                children_ids: vec![],
                error_indicator: 0.003,
            },
        ],
    };

    let encoded = encode_to_vec(&amr).expect("encode AdaptiveMeshRefinement failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress AdaptiveMeshRefinement failed");
    let decompressed = decompress(&compressed).expect("decompress AdaptiveMeshRefinement failed");
    let (decoded, _): (AdaptiveMeshRefinement, usize) =
        decode_from_slice(&decompressed).expect("decode AdaptiveMeshRefinement failed");

    assert_eq!(amr, decoded);
}

// ---------------------------------------------------------------------------
// Test 13 – Multiphase flow volume fractions (VOF method)
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_multiphase_vof_snapshot_roundtrip() {
    let snapshot = MultiphaseFlowSnapshot {
        time_step: 8000,
        time_seconds: 0.04,
        surface_tension_coefficient: 0.0728,
        gravity: Vec3 {
            x: 0.0,
            y: -9.81,
            z: 0.0,
        },
        cells: vec![
            MultiphaseVof {
                cell_id: 0,
                volume_fraction_liquid: 1.0,
                volume_fraction_gas: 0.0,
                interface_normal: None,
                curvature: None,
            },
            MultiphaseVof {
                cell_id: 1,
                volume_fraction_liquid: 0.65,
                volume_fraction_gas: 0.35,
                interface_normal: Some(Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }),
                curvature: Some(50.0),
            },
            MultiphaseVof {
                cell_id: 2,
                volume_fraction_liquid: 0.0,
                volume_fraction_gas: 1.0,
                interface_normal: None,
                curvature: None,
            },
            MultiphaseVof {
                cell_id: 3,
                volume_fraction_liquid: 0.12,
                volume_fraction_gas: 0.88,
                interface_normal: Some(Vec3 {
                    x: 0.1,
                    y: 0.99,
                    z: 0.0,
                }),
                curvature: Some(120.0),
            },
        ],
    };

    let encoded = encode_to_vec(&snapshot).expect("encode MultiphaseFlowSnapshot failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress MultiphaseFlowSnapshot failed");
    let decompressed = decompress(&compressed).expect("decompress MultiphaseFlowSnapshot failed");
    let (decoded, _): (MultiphaseFlowSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode MultiphaseFlowSnapshot failed");

    assert_eq!(snapshot, decoded);
}

// ---------------------------------------------------------------------------
// Test 14 – Spalart-Allmaras turbulence model
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_spalart_allmaras_roundtrip() {
    let model = TurbulenceModel::SpalartAllmaras {
        nu_tilde: 3.0e-4,
        cb1: 0.1355,
        cb2: 0.622,
        sigma: 2.0 / 3.0,
    };

    let encoded = encode_to_vec(&model).expect("encode Spalart-Allmaras failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress Spalart-Allmaras failed");
    let decompressed = decompress(&compressed).expect("decompress Spalart-Allmaras failed");
    let (decoded, _): (TurbulenceModel, usize) =
        decode_from_slice(&decompressed).expect("decode Spalart-Allmaras failed");

    assert_eq!(model, decoded);
}

// ---------------------------------------------------------------------------
// Test 15 – Conjugate heat transfer
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_conjugate_heat_transfer_roundtrip() {
    let cht = ConjugateHeatTransfer {
        fluid_zone: "coolant-channel".to_string(),
        solid_zone: "heat-sink-aluminum".to_string(),
        interface_area: 0.0025,
        fluid_points: (0..20)
            .map(|i| TemperatureFieldPoint {
                cell_id: i as u64,
                temperature_k: 300.0 + 2.0 * i as f64,
                heat_flux: Vec3 {
                    x: 0.0,
                    y: 5000.0 + 100.0 * i as f64,
                    z: 0.0,
                },
            })
            .collect(),
        solid_points: (0..20)
            .map(|i| TemperatureFieldPoint {
                cell_id: (i + 1000) as u64,
                temperature_k: 350.0 - 1.5 * i as f64,
                heat_flux: Vec3 {
                    x: 0.0,
                    y: -(5000.0 + 100.0 * i as f64),
                    z: 0.0,
                },
            })
            .collect(),
        nusselt_number: 48.3,
    };

    let encoded = encode_to_vec(&cht).expect("encode ConjugateHeatTransfer failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ConjugateHeatTransfer failed");
    let decompressed = decompress(&compressed).expect("decompress ConjugateHeatTransfer failed");
    let (decoded, _): (ConjugateHeatTransfer, usize) =
        decode_from_slice(&decompressed).expect("decode ConjugateHeatTransfer failed");

    assert_eq!(cht, decoded);
}

// ---------------------------------------------------------------------------
// Test 16 – Wall shear stress distribution
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_wall_shear_stress_roundtrip() {
    let wall_data: Vec<WallShearStress> = (0..40)
        .map(|i| {
            let tau_x = 0.3 + 0.01 * i as f64;
            let u_tau = (tau_x / 1.225).sqrt();
            WallShearStress {
                face_id: 5000 + i as u64,
                tau_wall: Vec3 {
                    x: tau_x,
                    y: 0.001,
                    z: 0.0,
                },
                y_plus: 0.8 + 0.1 * i as f64,
                friction_velocity: u_tau,
            }
        })
        .collect();

    let encoded = encode_to_vec(&wall_data).expect("encode WallShearStress vec failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress WallShearStress failed");
    let decompressed = decompress(&compressed).expect("decompress WallShearStress failed");
    let (decoded, _): (Vec<WallShearStress>, usize) =
        decode_from_slice(&decompressed).expect("decode WallShearStress vec failed");

    assert_eq!(wall_data, decoded);
}

// ---------------------------------------------------------------------------
// Test 17 – Boundary layer profile
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_boundary_layer_profile_roundtrip() {
    let n_points = 60;
    let delta = 0.01; // boundary layer thickness in meters
    let wall_distance: Vec<f64> = (0..n_points)
        .map(|i| delta * (i as f64 / n_points as f64))
        .collect();
    let velocity_parallel: Vec<f64> = wall_distance
        .iter()
        .map(|y| 20.0 * (y / delta).powf(1.0 / 7.0))
        .collect();
    let velocity_normal: Vec<f64> = wall_distance.iter().map(|y| 0.05 * (y / delta)).collect();
    let tke: Vec<f64> = wall_distance
        .iter()
        .map(|y| {
            let eta = y / delta;
            4.0 * eta * (1.0 - eta) * (1.0 - eta)
        })
        .collect();

    let profile = BoundaryLayerProfile {
        station_name: "x/c=0.5 suction side".to_string(),
        wall_distance,
        velocity_parallel,
        velocity_normal,
        turbulent_kinetic_energy: tke,
        displacement_thickness: 0.0013,
        momentum_thickness: 0.001,
        shape_factor: 1.3,
    };

    let encoded = encode_to_vec(&profile).expect("encode BoundaryLayerProfile failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BoundaryLayerProfile failed");
    let decompressed = decompress(&compressed).expect("decompress BoundaryLayerProfile failed");
    let (decoded, _): (BoundaryLayerProfile, usize) =
        decode_from_slice(&decompressed).expect("decode BoundaryLayerProfile failed");

    assert_eq!(profile, decoded);
}

// ---------------------------------------------------------------------------
// Test 18 – Large eddy simulation turbulence model
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_les_model_roundtrip() {
    let model = TurbulenceModel::LargeEddySimulation {
        smagorinsky_constant: 0.17,
        filter_width: 2.5e-4,
    };

    let encoded = encode_to_vec(&model).expect("encode LES TurbulenceModel failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress LES TurbulenceModel failed");
    let decompressed = decompress(&compressed).expect("decompress LES TurbulenceModel failed");
    let (decoded, _): (TurbulenceModel, usize) =
        decode_from_slice(&decompressed).expect("decode LES TurbulenceModel failed");

    assert_eq!(model, decoded);
}

// ---------------------------------------------------------------------------
// Test 19 – Combustion state with species transport
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_combustion_species_transport_roundtrip() {
    let state = CombustionState {
        mixture_fraction: 0.055,
        scalar_dissipation_rate: 12.5,
        temperature_k: 1850.0,
        species: vec![
            SpeciesTransport {
                species_name: "CH4".to_string(),
                molecular_weight: 16.04,
                diffusion_coefficient: 2.1e-5,
                mass_fractions: vec![0.04, 0.02, 0.005, 0.001, 0.0],
            },
            SpeciesTransport {
                species_name: "O2".to_string(),
                molecular_weight: 32.0,
                diffusion_coefficient: 2.0e-5,
                mass_fractions: vec![0.23, 0.18, 0.10, 0.05, 0.02],
            },
            SpeciesTransport {
                species_name: "CO2".to_string(),
                molecular_weight: 44.01,
                diffusion_coefficient: 1.6e-5,
                mass_fractions: vec![0.0, 0.03, 0.08, 0.11, 0.12],
            },
            SpeciesTransport {
                species_name: "H2O".to_string(),
                molecular_weight: 18.015,
                diffusion_coefficient: 2.5e-5,
                mass_fractions: vec![0.0, 0.02, 0.06, 0.09, 0.10],
            },
            SpeciesTransport {
                species_name: "N2".to_string(),
                molecular_weight: 28.014,
                diffusion_coefficient: 1.9e-5,
                mass_fractions: vec![0.73, 0.75, 0.755, 0.749, 0.76],
            },
        ],
    };

    let encoded = encode_to_vec(&state).expect("encode CombustionState failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress CombustionState failed");
    let decompressed = decompress(&compressed).expect("decompress CombustionState failed");
    let (decoded, _): (CombustionState, usize) =
        decode_from_slice(&decompressed).expect("decode CombustionState failed");

    assert_eq!(state, decoded);
}

// ---------------------------------------------------------------------------
// Test 20 – Periodic boundary condition with translation vector
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_periodic_boundary_roundtrip() {
    let bc = BoundaryCondition {
        zone_name: "periodic-pair-left".to_string(),
        zone_id: 10,
        face_count: 3200,
        condition: BoundaryConditionType::Periodic {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.5,
            },
        },
    };

    let encoded = encode_to_vec(&bc).expect("encode Periodic BoundaryCondition failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Periodic BC failed");
    let decompressed = decompress(&compressed).expect("decompress Periodic BC failed");
    let (decoded, _): (BoundaryCondition, usize) =
        decode_from_slice(&decompressed).expect("decode Periodic BoundaryCondition failed");

    assert_eq!(bc, decoded);
}

// ---------------------------------------------------------------------------
// Test 21 – Solver state with COUPLED scheme
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_coupled_solver_state_roundtrip() {
    let state = NavierStokesSolverState {
        iteration: 500_000,
        time_seconds: 2.5,
        dt: 1e-5,
        cfl_number: 1.2,
        solver_scheme: SolverScheme::Coupled,
        momentum_discretization: DiscretizationScheme::Quick,
        pressure_discretization: DiscretizationScheme::CentralDifference,
        under_relaxation_pressure: 0.5,
        under_relaxation_momentum: 0.5,
        converged: true,
    };

    let encoded = encode_to_vec(&state).expect("encode Coupled SolverState failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress Coupled SolverState failed");
    let decompressed = decompress(&compressed).expect("decompress Coupled SolverState failed");
    let (decoded, _): (NavierStokesSolverState, usize) =
        decode_from_slice(&decompressed).expect("decode Coupled SolverState failed");

    assert_eq!(state, decoded);
}

// ---------------------------------------------------------------------------
// Test 22 – Rough wall boundary with elevated roughness
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_cfd_rough_wall_boundary_roundtrip() {
    let bc = BoundaryCondition {
        zone_name: "turbine-blade-leading-edge".to_string(),
        zone_id: 42,
        face_count: 45_000,
        condition: BoundaryConditionType::Wall {
            no_slip: true,
            roughness_height: 5.0e-5,
        },
    };

    let encoded = encode_to_vec(&bc).expect("encode rough wall BoundaryCondition failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress rough wall BC failed");
    let decompressed = decompress(&compressed).expect("decompress rough wall BC failed");
    let (decoded, _): (BoundaryCondition, usize) =
        decode_from_slice(&decompressed).expect("decode rough wall BoundaryCondition failed");

    assert_eq!(bc, decoded);
}
