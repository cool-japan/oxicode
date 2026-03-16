//! Advanced nested struct encoding tests for OxiCode (set 4)
//! Theme: Computational Fluid Dynamics (CFD) simulation

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

// ── Geometry / Mesh primitives ──────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CellTopology {
    Hexahedral,
    Tetrahedral,
    Polyhedral { face_count: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeshCell {
    id: u64,
    centroid: Vec3,
    volume: f64,
    topology: CellTopology,
    quality: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RefinementLevel {
    level: u32,
    cells: Vec<MeshCell>,
    total_volume: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridHierarchy {
    name: String,
    levels: Vec<RefinementLevel>,
    base_cell_count: u64,
}

// ── Boundary conditions ─────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InletCondition {
    velocity: Vec3,
    temperature: f64,
    turbulence_intensity: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OutletCondition {
    pressure: f64,
    backflow_temperature: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WallCondition {
    no_slip: bool,
    roughness: f64,
    heat_flux: Option<f64>,
    temperature: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SymmetryCondition {
    normal: Vec3,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoundaryCondition {
    Inlet(InletCondition),
    Outlet(OutletCondition),
    Wall(WallCondition),
    Symmetry(SymmetryCondition),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoundaryPatch {
    name: String,
    face_count: u64,
    condition: BoundaryCondition,
}

// ── Flow field variables ────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowState {
    velocity: Vec3,
    pressure: f64,
    temperature: f64,
    density: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellFlowData {
    cell_id: u64,
    state: FlowState,
    gradients: Option<FlowGradients>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowGradients {
    dp_dx: Vec3,
    du_dx: Vec3,
    dt_dx: Vec3,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowField {
    time_step: u64,
    physical_time: f64,
    cells: Vec<CellFlowData>,
}

// ── Turbulence models ───────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KEpsilonParams {
    k: f64,
    epsilon: f64,
    c_mu: f64,
    c1_epsilon: f64,
    c2_epsilon: f64,
    sigma_k: f64,
    sigma_epsilon: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KOmegaSstParams {
    k: f64,
    omega: f64,
    a1: f64,
    beta_star: f64,
    alpha_1: f64,
    alpha_2: f64,
    beta_1: f64,
    beta_2: f64,
    sigma_k1: f64,
    sigma_k2: f64,
    sigma_omega1: f64,
    sigma_omega2: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbulenceModel {
    Laminar,
    KEpsilon(KEpsilonParams),
    KOmegaSst(KOmegaSstParams),
}

// ── Discretization / solver ─────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConvectionScheme {
    Upwind,
    SecondOrderUpwind,
    Quick,
    CentralDifference,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GradientScheme {
    GreenGauss,
    LeastSquares,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiscretizationSchemes {
    convection: ConvectionScheme,
    gradient: GradientScheme,
    pressure_velocity_coupling: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConvergenceCriterion {
    variable: String,
    tolerance: f64,
    current_residual: f64,
    converged: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SolverSettings {
    max_iterations: u64,
    criteria: Vec<ConvergenceCriterion>,
    discretization: DiscretizationSchemes,
    under_relaxation_pressure: f64,
    under_relaxation_velocity: f64,
}

// ── Time stepping ───────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimeStepping {
    cfl_number: f64,
    dt: f64,
    current_time: f64,
    max_time: f64,
    adaptive: bool,
    cfl_ramp: Option<CflRamp>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CflRamp {
    initial_cfl: f64,
    final_cfl: f64,
    ramp_iterations: u64,
}

// ── Fluid properties ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FluidProperties {
    name: String,
    density: f64,
    dynamic_viscosity: f64,
    kinematic_viscosity: f64,
    thermal_conductivity: f64,
    specific_heat_cp: f64,
    prandtl_number: f64,
}

// ── Reynolds number ─────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReynoldsInfo {
    characteristic_length: f64,
    freestream_velocity: f64,
    reynolds_number: f64,
    fluid: FluidProperties,
}

// ── Navier-Stokes residuals ─────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NavierStokesResiduals {
    iteration: u64,
    continuity: f64,
    x_momentum: f64,
    y_momentum: f64,
    z_momentum: f64,
    energy: f64,
    turbulence_residuals: Option<TurbulenceResiduals>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TurbulenceResiduals {
    primary: f64,
    secondary: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ResidualHistory {
    residuals: Vec<NavierStokesResiduals>,
    best_continuity: f64,
}

// ── Aerodynamic coefficients ────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AeroCoefficients {
    cl: f64,
    cd: f64,
    cm: f64,
    reference_area: f64,
    reference_length: f64,
    moment_center: Vec3,
}

// ── Vortex shedding ─────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VortexShedding {
    strouhal_number: f64,
    shedding_frequency_hz: f64,
    amplitude: f64,
    monitor_point: Vec3,
    pressure_history: Vec<f64>,
}

// ── Top-level simulation ────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CfdSimulation {
    name: String,
    grid: GridHierarchy,
    boundaries: Vec<BoundaryPatch>,
    turbulence: TurbulenceModel,
    solver: SolverSettings,
    time_stepping: TimeStepping,
    fluid: FluidProperties,
    reynolds: ReynoldsInfo,
    residual_history: ResidualHistory,
    aero_coefficients: Option<AeroCoefficients>,
    vortex_shedding: Option<VortexShedding>,
}

// ── Helpers ─────────────────────────────────────────────────────────

fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

fn make_air() -> FluidProperties {
    FluidProperties {
        name: "Air".into(),
        density: 1.225,
        dynamic_viscosity: 1.81e-5,
        kinematic_viscosity: 1.48e-5,
        thermal_conductivity: 0.0262,
        specific_heat_cp: 1006.0,
        prandtl_number: 0.71,
    }
}

fn make_water() -> FluidProperties {
    FluidProperties {
        name: "Water".into(),
        density: 998.2,
        dynamic_viscosity: 1.003e-3,
        kinematic_viscosity: 1.004e-6,
        thermal_conductivity: 0.598,
        specific_heat_cp: 4182.0,
        prandtl_number: 7.01,
    }
}

fn make_hex_cell(id: u64, cx: f64, cy: f64, cz: f64, vol: f64) -> MeshCell {
    MeshCell {
        id,
        centroid: v3(cx, cy, cz),
        volume: vol,
        topology: CellTopology::Hexahedral,
        quality: 0.95,
    }
}

fn make_tet_cell(id: u64, cx: f64, cy: f64, cz: f64, vol: f64) -> MeshCell {
    MeshCell {
        id,
        centroid: v3(cx, cy, cz),
        volume: vol,
        topology: CellTopology::Tetrahedral,
        quality: 0.82,
    }
}

fn make_poly_cell(id: u64, cx: f64, cy: f64, cz: f64, vol: f64, faces: u32) -> MeshCell {
    MeshCell {
        id,
        centroid: v3(cx, cy, cz),
        volume: vol,
        topology: CellTopology::Polyhedral { face_count: faces },
        quality: 0.88,
    }
}

fn make_k_epsilon() -> TurbulenceModel {
    TurbulenceModel::KEpsilon(KEpsilonParams {
        k: 0.01,
        epsilon: 0.001,
        c_mu: 0.09,
        c1_epsilon: 1.44,
        c2_epsilon: 1.92,
        sigma_k: 1.0,
        sigma_epsilon: 1.3,
    })
}

fn make_k_omega_sst() -> TurbulenceModel {
    TurbulenceModel::KOmegaSst(KOmegaSstParams {
        k: 0.015,
        omega: 5.0,
        a1: 0.31,
        beta_star: 0.09,
        alpha_1: 5.0 / 9.0,
        alpha_2: 0.44,
        beta_1: 0.075,
        beta_2: 0.0828,
        sigma_k1: 0.85,
        sigma_k2: 1.0,
        sigma_omega1: 0.5,
        sigma_omega2: 0.856,
    })
}

fn make_solver() -> SolverSettings {
    SolverSettings {
        max_iterations: 5000,
        criteria: vec![
            ConvergenceCriterion {
                variable: "continuity".into(),
                tolerance: 1e-6,
                current_residual: 3.2e-4,
                converged: false,
            },
            ConvergenceCriterion {
                variable: "x-momentum".into(),
                tolerance: 1e-6,
                current_residual: 1.1e-5,
                converged: false,
            },
        ],
        discretization: DiscretizationSchemes {
            convection: ConvectionScheme::SecondOrderUpwind,
            gradient: GradientScheme::LeastSquares,
            pressure_velocity_coupling: "SIMPLE".into(),
        },
        under_relaxation_pressure: 0.3,
        under_relaxation_velocity: 0.7,
    }
}

fn make_reynolds(fluid: &FluidProperties, vel: f64, length: f64) -> ReynoldsInfo {
    ReynoldsInfo {
        characteristic_length: length,
        freestream_velocity: vel,
        reynolds_number: vel * length / fluid.kinematic_viscosity,
        fluid: fluid.clone(),
    }
}

// ── Tests ───────────────────────────────────────────────────────────

// Test 1: Hexahedral mesh cell roundtrip
#[test]
fn test_hexahedral_cell_roundtrip() {
    let cell = make_hex_cell(1, 0.5, 0.5, 0.5, 1e-6);
    let bytes = encode_to_vec(&cell).expect("encode hex cell");
    let (dec, _): (MeshCell, usize) = decode_from_slice(&bytes).expect("decode hex cell");
    assert_eq!(cell, dec);
}

// Test 2: Tetrahedral mesh cell roundtrip
#[test]
fn test_tetrahedral_cell_roundtrip() {
    let cell = make_tet_cell(42, 1.2, 3.4, 5.6, 2.5e-7);
    let bytes = encode_to_vec(&cell).expect("encode tet cell");
    let (dec, _): (MeshCell, usize) = decode_from_slice(&bytes).expect("decode tet cell");
    assert_eq!(cell, dec);
}

// Test 3: Polyhedral mesh cell roundtrip
#[test]
fn test_polyhedral_cell_roundtrip() {
    let cell = make_poly_cell(99, 0.1, 0.2, 0.3, 4.4e-8, 14);
    let bytes = encode_to_vec(&cell).expect("encode poly cell");
    let (dec, _): (MeshCell, usize) = decode_from_slice(&bytes).expect("decode poly cell");
    assert_eq!(cell, dec);
}

// Test 4: Grid hierarchy with multiple refinement levels
#[test]
fn test_grid_hierarchy_roundtrip() {
    let grid = GridHierarchy {
        name: "channel_flow_mesh".into(),
        base_cell_count: 100_000,
        levels: vec![
            RefinementLevel {
                level: 0,
                total_volume: 1.0,
                cells: vec![
                    make_hex_cell(0, 0.0, 0.0, 0.0, 1e-4),
                    make_hex_cell(1, 0.01, 0.0, 0.0, 1e-4),
                ],
            },
            RefinementLevel {
                level: 1,
                total_volume: 0.25,
                cells: vec![
                    make_tet_cell(100, 0.0, 0.0, 0.0, 1.25e-5),
                    make_poly_cell(101, 0.005, 0.0, 0.0, 1.1e-5, 12),
                ],
            },
        ],
    };
    let bytes = encode_to_vec(&grid).expect("encode grid hierarchy");
    let (dec, _): (GridHierarchy, usize) =
        decode_from_slice(&bytes).expect("decode grid hierarchy");
    assert_eq!(grid, dec);
}

// Test 5: Inlet boundary condition roundtrip
#[test]
fn test_inlet_boundary_roundtrip() {
    let patch = BoundaryPatch {
        name: "velocity_inlet".into(),
        face_count: 2400,
        condition: BoundaryCondition::Inlet(InletCondition {
            velocity: v3(10.0, 0.0, 0.0),
            temperature: 300.0,
            turbulence_intensity: 0.05,
        }),
    };
    let bytes = encode_to_vec(&patch).expect("encode inlet patch");
    let (dec, _): (BoundaryPatch, usize) = decode_from_slice(&bytes).expect("decode inlet patch");
    assert_eq!(patch, dec);
}

// Test 6: Wall boundary with heat flux roundtrip
#[test]
fn test_wall_boundary_heat_flux_roundtrip() {
    let patch = BoundaryPatch {
        name: "heated_wall".into(),
        face_count: 8000,
        condition: BoundaryCondition::Wall(WallCondition {
            no_slip: true,
            roughness: 0.001,
            heat_flux: Some(5000.0),
            temperature: None,
        }),
    };
    let bytes = encode_to_vec(&patch).expect("encode wall patch");
    let (dec, _): (BoundaryPatch, usize) = decode_from_slice(&bytes).expect("decode wall patch");
    assert_eq!(patch, dec);
}

// Test 7: Symmetry boundary roundtrip
#[test]
fn test_symmetry_boundary_roundtrip() {
    let patch = BoundaryPatch {
        name: "symmetry_plane".into(),
        face_count: 3200,
        condition: BoundaryCondition::Symmetry(SymmetryCondition {
            normal: v3(0.0, 1.0, 0.0),
        }),
    };
    let bytes = encode_to_vec(&patch).expect("encode symmetry patch");
    let (dec, _): (BoundaryPatch, usize) =
        decode_from_slice(&bytes).expect("decode symmetry patch");
    assert_eq!(patch, dec);
}

// Test 8: Flow state with gradients (deeply nested)
#[test]
fn test_cell_flow_data_with_gradients_roundtrip() {
    let data = CellFlowData {
        cell_id: 555,
        state: FlowState {
            velocity: v3(12.5, -0.3, 0.01),
            pressure: 101325.0,
            temperature: 293.15,
            density: 1.205,
        },
        gradients: Some(FlowGradients {
            dp_dx: v3(50.0, -10.0, 0.0),
            du_dx: v3(100.0, -20.0, 5.0),
            dt_dx: v3(0.5, 0.1, 0.0),
        }),
    };
    let bytes = encode_to_vec(&data).expect("encode cell flow data");
    let (dec, _): (CellFlowData, usize) = decode_from_slice(&bytes).expect("decode cell flow data");
    assert_eq!(data, dec);
}

// Test 9: Flow field with multiple cells
#[test]
fn test_flow_field_roundtrip() {
    let field = FlowField {
        time_step: 1200,
        physical_time: 0.024,
        cells: vec![
            CellFlowData {
                cell_id: 0,
                state: FlowState {
                    velocity: v3(15.0, 0.0, 0.0),
                    pressure: 101325.0,
                    temperature: 300.0,
                    density: 1.177,
                },
                gradients: None,
            },
            CellFlowData {
                cell_id: 1,
                state: FlowState {
                    velocity: v3(14.8, 0.2, -0.01),
                    pressure: 101310.0,
                    temperature: 300.5,
                    density: 1.176,
                },
                gradients: Some(FlowGradients {
                    dp_dx: v3(-15.0, 0.0, 0.0),
                    du_dx: v3(-20.0, 10.0, 0.0),
                    dt_dx: v3(0.5, 0.0, 0.0),
                }),
            },
        ],
    };
    let bytes = encode_to_vec(&field).expect("encode flow field");
    let (dec, _): (FlowField, usize) = decode_from_slice(&bytes).expect("decode flow field");
    assert_eq!(field, dec);
}

// Test 10: k-epsilon turbulence model roundtrip
#[test]
fn test_k_epsilon_model_roundtrip() {
    let model = make_k_epsilon();
    let bytes = encode_to_vec(&model).expect("encode k-epsilon");
    let (dec, _): (TurbulenceModel, usize) = decode_from_slice(&bytes).expect("decode k-epsilon");
    assert_eq!(model, dec);
}

// Test 11: k-omega SST turbulence model roundtrip
#[test]
fn test_k_omega_sst_model_roundtrip() {
    let model = make_k_omega_sst();
    let bytes = encode_to_vec(&model).expect("encode k-omega SST");
    let (dec, _): (TurbulenceModel, usize) = decode_from_slice(&bytes).expect("decode k-omega SST");
    assert_eq!(model, dec);
}

// Test 12: Solver settings with convergence criteria
#[test]
fn test_solver_settings_roundtrip() {
    let solver = make_solver();
    let bytes = encode_to_vec(&solver).expect("encode solver settings");
    let (dec, _): (SolverSettings, usize) =
        decode_from_slice(&bytes).expect("decode solver settings");
    assert_eq!(solver, dec);
}

// Test 13: Time stepping with CFL ramp
#[test]
fn test_time_stepping_with_cfl_ramp_roundtrip() {
    let ts = TimeStepping {
        cfl_number: 1.0,
        dt: 1e-5,
        current_time: 0.0,
        max_time: 1.0,
        adaptive: true,
        cfl_ramp: Some(CflRamp {
            initial_cfl: 0.5,
            final_cfl: 5.0,
            ramp_iterations: 200,
        }),
    };
    let bytes = encode_to_vec(&ts).expect("encode time stepping");
    let (dec, _): (TimeStepping, usize) = decode_from_slice(&bytes).expect("decode time stepping");
    assert_eq!(ts, dec);
}

// Test 14: Fluid properties roundtrip (air)
#[test]
fn test_fluid_properties_air_roundtrip() {
    let air = make_air();
    let bytes = encode_to_vec(&air).expect("encode air properties");
    let (dec, _): (FluidProperties, usize) =
        decode_from_slice(&bytes).expect("decode air properties");
    assert_eq!(air, dec);
}

// Test 15: Reynolds number info with nested fluid
#[test]
fn test_reynolds_info_roundtrip() {
    let water = make_water();
    let re = make_reynolds(&water, 2.0, 0.05);
    let bytes = encode_to_vec(&re).expect("encode Reynolds info");
    let (dec, _): (ReynoldsInfo, usize) = decode_from_slice(&bytes).expect("decode Reynolds info");
    assert_eq!(re, dec);
}

// Test 16: Navier-Stokes residuals with turbulence residuals
#[test]
fn test_navier_stokes_residuals_roundtrip() {
    let history = ResidualHistory {
        best_continuity: 1.2e-5,
        residuals: vec![
            NavierStokesResiduals {
                iteration: 100,
                continuity: 5.5e-3,
                x_momentum: 2.1e-3,
                y_momentum: 1.8e-3,
                z_momentum: 9.0e-4,
                energy: 4.4e-4,
                turbulence_residuals: Some(TurbulenceResiduals {
                    primary: 3.3e-3,
                    secondary: 7.7e-4,
                }),
            },
            NavierStokesResiduals {
                iteration: 200,
                continuity: 1.2e-5,
                x_momentum: 8.0e-6,
                y_momentum: 6.5e-6,
                z_momentum: 3.1e-6,
                energy: 1.0e-6,
                turbulence_residuals: Some(TurbulenceResiduals {
                    primary: 9.9e-6,
                    secondary: 2.2e-6,
                }),
            },
        ],
    };
    let bytes = encode_to_vec(&history).expect("encode residual history");
    let (dec, _): (ResidualHistory, usize) =
        decode_from_slice(&bytes).expect("decode residual history");
    assert_eq!(history, dec);
}

// Test 17: Aerodynamic coefficients roundtrip
#[test]
fn test_aero_coefficients_roundtrip() {
    let aero = AeroCoefficients {
        cl: 0.45,
        cd: 0.012,
        cm: -0.08,
        reference_area: 1.5,
        reference_length: 0.3,
        moment_center: v3(0.25, 0.0, 0.0),
    };
    let bytes = encode_to_vec(&aero).expect("encode aero coefficients");
    let (dec, _): (AeroCoefficients, usize) =
        decode_from_slice(&bytes).expect("decode aero coefficients");
    assert_eq!(aero, dec);
}

// Test 18: Vortex shedding data roundtrip
#[test]
fn test_vortex_shedding_roundtrip() {
    let vs = VortexShedding {
        strouhal_number: 0.21,
        shedding_frequency_hz: 42.0,
        amplitude: 0.15,
        monitor_point: v3(2.0, 0.0, 0.0),
        pressure_history: vec![
            101325.0, 101320.0, 101330.0, 101315.0, 101335.0, 101310.0, 101340.0, 101305.0,
            101345.0, 101300.0,
        ],
    };
    let bytes = encode_to_vec(&vs).expect("encode vortex shedding");
    let (dec, _): (VortexShedding, usize) =
        decode_from_slice(&bytes).expect("decode vortex shedding");
    assert_eq!(vs, dec);
}

// Test 19: Full CFD simulation (deeply nested, 4 levels)
#[test]
fn test_full_cfd_simulation_roundtrip() {
    let air = make_air();
    let sim = CfdSimulation {
        name: "naca0012_external_aero".into(),
        grid: GridHierarchy {
            name: "airfoil_mesh".into(),
            base_cell_count: 500_000,
            levels: vec![RefinementLevel {
                level: 0,
                total_volume: 100.0,
                cells: vec![make_hex_cell(0, 0.0, 0.0, 0.0, 1e-3)],
            }],
        },
        boundaries: vec![
            BoundaryPatch {
                name: "farfield".into(),
                face_count: 400,
                condition: BoundaryCondition::Inlet(InletCondition {
                    velocity: v3(50.0, 0.0, 0.0),
                    temperature: 288.15,
                    turbulence_intensity: 0.01,
                }),
            },
            BoundaryPatch {
                name: "airfoil_surface".into(),
                face_count: 12000,
                condition: BoundaryCondition::Wall(WallCondition {
                    no_slip: true,
                    roughness: 0.0,
                    heat_flux: None,
                    temperature: None,
                }),
            },
        ],
        turbulence: make_k_omega_sst(),
        solver: make_solver(),
        time_stepping: TimeStepping {
            cfl_number: 5.0,
            dt: 1e-4,
            current_time: 0.0,
            max_time: 0.5,
            adaptive: false,
            cfl_ramp: None,
        },
        fluid: air.clone(),
        reynolds: make_reynolds(&air, 50.0, 0.3),
        residual_history: ResidualHistory {
            best_continuity: 8.8e-7,
            residuals: vec![NavierStokesResiduals {
                iteration: 3000,
                continuity: 8.8e-7,
                x_momentum: 5.5e-7,
                y_momentum: 3.3e-7,
                z_momentum: 1.1e-7,
                energy: 9.0e-8,
                turbulence_residuals: Some(TurbulenceResiduals {
                    primary: 4.4e-7,
                    secondary: 2.2e-7,
                }),
            }],
        },
        aero_coefficients: Some(AeroCoefficients {
            cl: 0.35,
            cd: 0.008,
            cm: -0.05,
            reference_area: 0.3,
            reference_length: 0.3,
            moment_center: v3(0.075, 0.0, 0.0),
        }),
        vortex_shedding: None,
    };
    let bytes = encode_to_vec(&sim).expect("encode full CFD simulation");
    let (dec, _): (CfdSimulation, usize) =
        decode_from_slice(&bytes).expect("decode full CFD simulation");
    assert_eq!(sim, dec);
}

// Test 20: CFD simulation with vortex shedding (cylinder flow)
#[test]
fn test_cylinder_vortex_shedding_simulation_roundtrip() {
    let water = make_water();
    let sim = CfdSimulation {
        name: "cylinder_crossflow_vortex".into(),
        grid: GridHierarchy {
            name: "cylinder_mesh".into(),
            base_cell_count: 250_000,
            levels: vec![
                RefinementLevel {
                    level: 0,
                    total_volume: 4.0,
                    cells: vec![make_hex_cell(0, -1.0, 0.0, 0.0, 2e-4)],
                },
                RefinementLevel {
                    level: 1,
                    total_volume: 0.5,
                    cells: vec![
                        make_tet_cell(1000, 0.05, 0.0, 0.0, 5e-6),
                        make_poly_cell(1001, 0.06, 0.01, 0.0, 4.8e-6, 10),
                    ],
                },
            ],
        },
        boundaries: vec![
            BoundaryPatch {
                name: "inlet".into(),
                face_count: 500,
                condition: BoundaryCondition::Inlet(InletCondition {
                    velocity: v3(0.5, 0.0, 0.0),
                    temperature: 293.15,
                    turbulence_intensity: 0.02,
                }),
            },
            BoundaryPatch {
                name: "outlet".into(),
                face_count: 500,
                condition: BoundaryCondition::Outlet(OutletCondition {
                    pressure: 0.0,
                    backflow_temperature: 293.15,
                }),
            },
            BoundaryPatch {
                name: "cylinder_wall".into(),
                face_count: 6000,
                condition: BoundaryCondition::Wall(WallCondition {
                    no_slip: true,
                    roughness: 0.0,
                    heat_flux: None,
                    temperature: None,
                }),
            },
            BoundaryPatch {
                name: "top_sym".into(),
                face_count: 1000,
                condition: BoundaryCondition::Symmetry(SymmetryCondition {
                    normal: v3(0.0, 1.0, 0.0),
                }),
            },
        ],
        turbulence: TurbulenceModel::Laminar,
        solver: SolverSettings {
            max_iterations: 20,
            criteria: vec![ConvergenceCriterion {
                variable: "continuity".into(),
                tolerance: 1e-5,
                current_residual: 8.0e-6,
                converged: true,
            }],
            discretization: DiscretizationSchemes {
                convection: ConvectionScheme::CentralDifference,
                gradient: GradientScheme::GreenGauss,
                pressure_velocity_coupling: "PISO".into(),
            },
            under_relaxation_pressure: 1.0,
            under_relaxation_velocity: 1.0,
        },
        time_stepping: TimeStepping {
            cfl_number: 0.8,
            dt: 5e-4,
            current_time: 10.0,
            max_time: 20.0,
            adaptive: true,
            cfl_ramp: Some(CflRamp {
                initial_cfl: 0.2,
                final_cfl: 0.8,
                ramp_iterations: 100,
            }),
        },
        fluid: water.clone(),
        reynolds: make_reynolds(&water, 0.5, 0.02),
        residual_history: ResidualHistory {
            best_continuity: 3.0e-6,
            residuals: vec![
                NavierStokesResiduals {
                    iteration: 500,
                    continuity: 1.1e-4,
                    x_momentum: 5.5e-5,
                    y_momentum: 4.4e-5,
                    z_momentum: 1.1e-5,
                    energy: 3.3e-5,
                    turbulence_residuals: None,
                },
                NavierStokesResiduals {
                    iteration: 5000,
                    continuity: 3.0e-6,
                    x_momentum: 2.0e-6,
                    y_momentum: 1.5e-6,
                    z_momentum: 8.0e-7,
                    energy: 5.0e-7,
                    turbulence_residuals: None,
                },
            ],
        },
        aero_coefficients: Some(AeroCoefficients {
            cl: 0.0,
            cd: 1.18,
            cm: 0.0,
            reference_area: 0.02,
            reference_length: 0.02,
            moment_center: v3(0.0, 0.0, 0.0),
        }),
        vortex_shedding: Some(VortexShedding {
            strouhal_number: 0.198,
            shedding_frequency_hz: 4.95,
            amplitude: 0.22,
            monitor_point: v3(0.05, 0.0, 0.0),
            pressure_history: vec![0.0, 1.5, 0.0, -1.5, 0.0, 1.5],
        }),
    };
    let bytes = encode_to_vec(&sim).expect("encode cylinder simulation");
    let (dec, _): (CfdSimulation, usize) =
        decode_from_slice(&bytes).expect("decode cylinder simulation");
    assert_eq!(sim, dec);
}

// Test 21: Multiple boundary patches collection
#[test]
fn test_multiple_boundary_patches_roundtrip() {
    let patches = vec![
        BoundaryPatch {
            name: "inlet_main".into(),
            face_count: 1200,
            condition: BoundaryCondition::Inlet(InletCondition {
                velocity: v3(5.0, 0.0, 0.0),
                temperature: 350.0,
                turbulence_intensity: 0.10,
            }),
        },
        BoundaryPatch {
            name: "outlet_main".into(),
            face_count: 1200,
            condition: BoundaryCondition::Outlet(OutletCondition {
                pressure: 101325.0,
                backflow_temperature: 300.0,
            }),
        },
        BoundaryPatch {
            name: "casing_wall".into(),
            face_count: 9600,
            condition: BoundaryCondition::Wall(WallCondition {
                no_slip: true,
                roughness: 0.005,
                heat_flux: None,
                temperature: Some(400.0),
            }),
        },
        BoundaryPatch {
            name: "midplane_sym".into(),
            face_count: 2400,
            condition: BoundaryCondition::Symmetry(SymmetryCondition {
                normal: v3(0.0, 0.0, 1.0),
            }),
        },
    ];
    let bytes = encode_to_vec(&patches).expect("encode boundary patches");
    let (dec, _): (Vec<BoundaryPatch>, usize) =
        decode_from_slice(&bytes).expect("decode boundary patches");
    assert_eq!(patches, dec);
}

// Test 22: Discretization scheme variants roundtrip
#[test]
fn test_discretization_scheme_variants_roundtrip() {
    let schemes = vec![
        DiscretizationSchemes {
            convection: ConvectionScheme::Upwind,
            gradient: GradientScheme::GreenGauss,
            pressure_velocity_coupling: "SIMPLE".into(),
        },
        DiscretizationSchemes {
            convection: ConvectionScheme::Quick,
            gradient: GradientScheme::LeastSquares,
            pressure_velocity_coupling: "SIMPLEC".into(),
        },
        DiscretizationSchemes {
            convection: ConvectionScheme::CentralDifference,
            gradient: GradientScheme::GreenGauss,
            pressure_velocity_coupling: "PISO".into(),
        },
        DiscretizationSchemes {
            convection: ConvectionScheme::SecondOrderUpwind,
            gradient: GradientScheme::LeastSquares,
            pressure_velocity_coupling: "Coupled".into(),
        },
    ];
    let bytes = encode_to_vec(&schemes).expect("encode discretization schemes");
    let (dec, _): (Vec<DiscretizationSchemes>, usize) =
        decode_from_slice(&bytes).expect("decode discretization schemes");
    assert_eq!(schemes, dec);
}
