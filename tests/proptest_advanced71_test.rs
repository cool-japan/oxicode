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

// ── Domain types ─────────────────────────────────────────────────────────────

/// 3-component velocity vector (stored as integer micrometres/s × 1000)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VelocityVector {
    node_id: u64,
    u_x1000: i64,
    v_x1000: i64,
    w_x1000: i64,
}

/// Scalar pressure field at a mesh node (Pascals × 1000)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PressureNode {
    node_id: u64,
    pressure_x1000: i64,
    time_step: u32,
}

/// Reynolds-number descriptor (dimensionless × 100)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReynoldsDescriptor {
    domain_id: u32,
    reynolds_x100: u64,
    characteristic_length_um: u32,
    kinematic_viscosity_x1e9: u32,
}

/// Navier-Stokes solver parameters
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NavierStokesParams {
    solver_id: u32,
    max_iterations: u32,
    convergence_tol_x1e12: u64,
    relaxation_factor_x1000: u32,
    density_kg_m3_x1000: u32,
    dynamic_viscosity_x1e6: u32,
}

/// Unstructured mesh node with neighbour count
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeshNode {
    node_id: u64,
    x_um: i64,
    y_um: i64,
    z_um: i64,
    neighbour_count: u16,
    layer_id: u16,
}

/// Turbulence model selector
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbulenceModel {
    LaminarFlow,
    KEpsilonStandard,
    KOmegaSST,
    SpalartAllmaras,
    ReynoldsStressMdl,
    LargeEddySim,
    DetachedEddySim,
}

/// Boundary condition type for a CFD face patch
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoundaryConditionKind {
    NoSlipWall,
    SlipWall,
    VelocityInlet,
    PressureOutlet,
    SymmetryPlane,
    PeriodicPair,
    FarField,
}

/// A single CFD boundary patch
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoundaryPatch {
    patch_id: u32,
    kind: BoundaryConditionKind,
    face_count: u32,
    area_um2_x1000: u64,
}

/// Flow coefficient (Cd, Cl, Cm, etc.) record
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowCoefficient {
    run_id: u64,
    coefficient_x1e6: i64,
    reference_area_um2: u64,
    angle_of_attack_x1000: i32,
}

/// 3×3 vorticity tensor (stored as integer rad/s × 1e6)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VorticityTensor {
    cell_id: u64,
    omega_xx: i64,
    omega_xy: i64,
    omega_xz: i64,
    omega_yx: i64,
    omega_yy: i64,
    omega_yz: i64,
    omega_zx: i64,
    omega_zy: i64,
    omega_zz: i64,
}

/// Stream function value at a 2-D mesh node
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StreamFunction {
    node_id: u64,
    psi_x1e9: i64,
    iter_number: u32,
}

/// Mach number descriptor (free-stream and local)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachDescriptor {
    cell_id: u64,
    mach_freestream_x10000: u32,
    mach_local_x10000: u32,
    speed_of_sound_mm_s_x1000: u32,
}

/// Convective heat transfer coefficient (W m⁻² K⁻¹ × 1000)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeatTransferCoeff {
    surface_id: u32,
    htc_x1000: u64,
    wall_temp_mk: u32,
    fluid_temp_mk: u32,
    nusselt_x1000: u32,
}

// ── Strategies ───────────────────────────────────────────────────────────────

fn arb_velocity_vector() -> impl Strategy<Value = VelocityVector> {
    (any::<u64>(), any::<i64>(), any::<i64>(), any::<i64>()).prop_map(
        |(node_id, u_x1000, v_x1000, w_x1000)| VelocityVector {
            node_id,
            u_x1000,
            v_x1000,
            w_x1000,
        },
    )
}

fn arb_pressure_node() -> impl Strategy<Value = PressureNode> {
    (any::<u64>(), any::<i64>(), any::<u32>()).prop_map(|(node_id, pressure_x1000, time_step)| {
        PressureNode {
            node_id,
            pressure_x1000,
            time_step,
        }
    })
}

fn arb_reynolds_descriptor() -> impl Strategy<Value = ReynoldsDescriptor> {
    (any::<u32>(), any::<u64>(), any::<u32>(), any::<u32>()).prop_map(
        |(domain_id, reynolds_x100, characteristic_length_um, kinematic_viscosity_x1e9)| {
            ReynoldsDescriptor {
                domain_id,
                reynolds_x100,
                characteristic_length_um,
                kinematic_viscosity_x1e9,
            }
        },
    )
}

fn arb_navier_stokes_params() -> impl Strategy<Value = NavierStokesParams> {
    (
        any::<u32>(),
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(
                solver_id,
                max_iterations,
                convergence_tol_x1e12,
                relaxation_factor_x1000,
                density_kg_m3_x1000,
                dynamic_viscosity_x1e6,
            )| NavierStokesParams {
                solver_id,
                max_iterations,
                convergence_tol_x1e12,
                relaxation_factor_x1000,
                density_kg_m3_x1000,
                dynamic_viscosity_x1e6,
            },
        )
}

fn arb_mesh_node() -> impl Strategy<Value = MeshNode> {
    (
        any::<u64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<u16>(),
        any::<u16>(),
    )
        .prop_map(
            |(node_id, x_um, y_um, z_um, neighbour_count, layer_id)| MeshNode {
                node_id,
                x_um,
                y_um,
                z_um,
                neighbour_count,
                layer_id,
            },
        )
}

fn arb_turbulence_model() -> impl Strategy<Value = TurbulenceModel> {
    prop_oneof![
        Just(TurbulenceModel::LaminarFlow),
        Just(TurbulenceModel::KEpsilonStandard),
        Just(TurbulenceModel::KOmegaSST),
        Just(TurbulenceModel::SpalartAllmaras),
        Just(TurbulenceModel::ReynoldsStressMdl),
        Just(TurbulenceModel::LargeEddySim),
        Just(TurbulenceModel::DetachedEddySim),
    ]
}

fn arb_boundary_condition_kind() -> impl Strategy<Value = BoundaryConditionKind> {
    prop_oneof![
        Just(BoundaryConditionKind::NoSlipWall),
        Just(BoundaryConditionKind::SlipWall),
        Just(BoundaryConditionKind::VelocityInlet),
        Just(BoundaryConditionKind::PressureOutlet),
        Just(BoundaryConditionKind::SymmetryPlane),
        Just(BoundaryConditionKind::PeriodicPair),
        Just(BoundaryConditionKind::FarField),
    ]
}

fn arb_boundary_patch() -> impl Strategy<Value = BoundaryPatch> {
    (
        any::<u32>(),
        arb_boundary_condition_kind(),
        any::<u32>(),
        any::<u64>(),
    )
        .prop_map(
            |(patch_id, kind, face_count, area_um2_x1000)| BoundaryPatch {
                patch_id,
                kind,
                face_count,
                area_um2_x1000,
            },
        )
}

fn arb_flow_coefficient() -> impl Strategy<Value = FlowCoefficient> {
    (any::<u64>(), any::<i64>(), any::<u64>(), any::<i32>()).prop_map(
        |(run_id, coefficient_x1e6, reference_area_um2, angle_of_attack_x1000)| FlowCoefficient {
            run_id,
            coefficient_x1e6,
            reference_area_um2,
            angle_of_attack_x1000,
        },
    )
}

fn arb_vorticity_tensor() -> impl Strategy<Value = VorticityTensor> {
    (
        any::<u64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
        any::<i64>(),
    )
        .prop_map(
            |(
                cell_id,
                omega_xx,
                omega_xy,
                omega_xz,
                omega_yx,
                omega_yy,
                omega_yz,
                omega_zx,
                omega_zy,
                omega_zz,
            )| VorticityTensor {
                cell_id,
                omega_xx,
                omega_xy,
                omega_xz,
                omega_yx,
                omega_yy,
                omega_yz,
                omega_zx,
                omega_zy,
                omega_zz,
            },
        )
}

fn arb_stream_function() -> impl Strategy<Value = StreamFunction> {
    (any::<u64>(), any::<i64>(), any::<u32>()).prop_map(|(node_id, psi_x1e9, iter_number)| {
        StreamFunction {
            node_id,
            psi_x1e9,
            iter_number,
        }
    })
}

fn arb_mach_descriptor() -> impl Strategy<Value = MachDescriptor> {
    (any::<u64>(), any::<u32>(), any::<u32>(), any::<u32>()).prop_map(
        |(cell_id, mach_freestream_x10000, mach_local_x10000, speed_of_sound_mm_s_x1000)| {
            MachDescriptor {
                cell_id,
                mach_freestream_x10000,
                mach_local_x10000,
                speed_of_sound_mm_s_x1000,
            }
        },
    )
}

fn arb_heat_transfer_coeff() -> impl Strategy<Value = HeatTransferCoeff> {
    (
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(surface_id, htc_x1000, wall_temp_mk, fluid_temp_mk, nusselt_x1000)| {
                HeatTransferCoeff {
                    surface_id,
                    htc_x1000,
                    wall_temp_mk,
                    fluid_temp_mk,
                    nusselt_x1000,
                }
            },
        )
}

// ── Tests ─────────────────────────────────────────────────────────────────────

proptest! {
    // 1. VelocityVector roundtrip
    #[test]
    fn test_velocity_vector_roundtrip(vel in arb_velocity_vector()) {
        let bytes = encode_to_vec(&vel).expect("encode VelocityVector");
        let (decoded, _): (VelocityVector, usize) =
            decode_from_slice(&bytes).expect("decode VelocityVector");
        prop_assert_eq!(vel, decoded);
    }

    // 2. VelocityVector deterministic encoding
    #[test]
    fn test_velocity_vector_deterministic(vel in arb_velocity_vector()) {
        let bytes_a = encode_to_vec(&vel).expect("encode VelocityVector first");
        let bytes_b = encode_to_vec(&vel).expect("encode VelocityVector second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 3. VelocityVector consumed bytes equals total encoded length
    #[test]
    fn test_velocity_vector_consumed_bytes(vel in arb_velocity_vector()) {
        let bytes = encode_to_vec(&vel).expect("encode VelocityVector");
        let len = bytes.len();
        let (_, consumed): (VelocityVector, usize) =
            decode_from_slice(&bytes).expect("decode VelocityVector");
        prop_assert_eq!(consumed, len);
    }

    // 4. PressureNode roundtrip
    #[test]
    fn test_pressure_node_roundtrip(pn in arb_pressure_node()) {
        let bytes = encode_to_vec(&pn).expect("encode PressureNode");
        let (decoded, _): (PressureNode, usize) =
            decode_from_slice(&bytes).expect("decode PressureNode");
        prop_assert_eq!(pn, decoded);
    }

    // 5. PressureNode consumed bytes equals total encoded length
    #[test]
    fn test_pressure_node_consumed_bytes(pn in arb_pressure_node()) {
        let bytes = encode_to_vec(&pn).expect("encode PressureNode");
        let len = bytes.len();
        let (_, consumed): (PressureNode, usize) =
            decode_from_slice(&bytes).expect("decode PressureNode");
        prop_assert_eq!(consumed, len);
    }

    // 6. ReynoldsDescriptor roundtrip
    #[test]
    fn test_reynolds_descriptor_roundtrip(re in arb_reynolds_descriptor()) {
        let bytes = encode_to_vec(&re).expect("encode ReynoldsDescriptor");
        let (decoded, _): (ReynoldsDescriptor, usize) =
            decode_from_slice(&bytes).expect("decode ReynoldsDescriptor");
        prop_assert_eq!(re, decoded);
    }

    // 7. ReynoldsDescriptor deterministic encoding
    #[test]
    fn test_reynolds_descriptor_deterministic(re in arb_reynolds_descriptor()) {
        let bytes_a = encode_to_vec(&re).expect("encode ReynoldsDescriptor first");
        let bytes_b = encode_to_vec(&re).expect("encode ReynoldsDescriptor second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 8. NavierStokesParams roundtrip
    #[test]
    fn test_navier_stokes_params_roundtrip(ns in arb_navier_stokes_params()) {
        let bytes = encode_to_vec(&ns).expect("encode NavierStokesParams");
        let (decoded, _): (NavierStokesParams, usize) =
            decode_from_slice(&bytes).expect("decode NavierStokesParams");
        prop_assert_eq!(ns, decoded);
    }

    // 9. NavierStokesParams consumed bytes equals total encoded length
    #[test]
    fn test_navier_stokes_params_consumed_bytes(ns in arb_navier_stokes_params()) {
        let bytes = encode_to_vec(&ns).expect("encode NavierStokesParams");
        let len = bytes.len();
        let (_, consumed): (NavierStokesParams, usize) =
            decode_from_slice(&bytes).expect("decode NavierStokesParams");
        prop_assert_eq!(consumed, len);
    }

    // 10. MeshNode roundtrip
    #[test]
    fn test_mesh_node_roundtrip(mn in arb_mesh_node()) {
        let bytes = encode_to_vec(&mn).expect("encode MeshNode");
        let (decoded, _): (MeshNode, usize) =
            decode_from_slice(&bytes).expect("decode MeshNode");
        prop_assert_eq!(mn, decoded);
    }

    // 11. TurbulenceModel enum roundtrip
    #[test]
    fn test_turbulence_model_roundtrip(tm in arb_turbulence_model()) {
        let bytes = encode_to_vec(&tm).expect("encode TurbulenceModel");
        let (decoded, _): (TurbulenceModel, usize) =
            decode_from_slice(&bytes).expect("decode TurbulenceModel");
        prop_assert_eq!(tm, decoded);
    }

    // 12. TurbulenceModel deterministic encoding
    #[test]
    fn test_turbulence_model_deterministic(tm in arb_turbulence_model()) {
        let bytes_a = encode_to_vec(&tm).expect("encode TurbulenceModel first");
        let bytes_b = encode_to_vec(&tm).expect("encode TurbulenceModel second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 13. BoundaryPatch roundtrip
    #[test]
    fn test_boundary_patch_roundtrip(bp in arb_boundary_patch()) {
        let bytes = encode_to_vec(&bp).expect("encode BoundaryPatch");
        let (decoded, _): (BoundaryPatch, usize) =
            decode_from_slice(&bytes).expect("decode BoundaryPatch");
        prop_assert_eq!(bp, decoded);
    }

    // 14. BoundaryPatch consumed bytes equals total encoded length
    #[test]
    fn test_boundary_patch_consumed_bytes(bp in arb_boundary_patch()) {
        let bytes = encode_to_vec(&bp).expect("encode BoundaryPatch");
        let len = bytes.len();
        let (_, consumed): (BoundaryPatch, usize) =
            decode_from_slice(&bytes).expect("decode BoundaryPatch");
        prop_assert_eq!(consumed, len);
    }

    // 15. FlowCoefficient roundtrip
    #[test]
    fn test_flow_coefficient_roundtrip(fc in arb_flow_coefficient()) {
        let bytes = encode_to_vec(&fc).expect("encode FlowCoefficient");
        let (decoded, _): (FlowCoefficient, usize) =
            decode_from_slice(&bytes).expect("decode FlowCoefficient");
        prop_assert_eq!(fc, decoded);
    }

    // 16. VorticityTensor roundtrip
    #[test]
    fn test_vorticity_tensor_roundtrip(vt in arb_vorticity_tensor()) {
        let bytes = encode_to_vec(&vt).expect("encode VorticityTensor");
        let (decoded, _): (VorticityTensor, usize) =
            decode_from_slice(&bytes).expect("decode VorticityTensor");
        prop_assert_eq!(vt, decoded);
    }

    // 17. VorticityTensor deterministic encoding
    #[test]
    fn test_vorticity_tensor_deterministic(vt in arb_vorticity_tensor()) {
        let bytes_a = encode_to_vec(&vt).expect("encode VorticityTensor first");
        let bytes_b = encode_to_vec(&vt).expect("encode VorticityTensor second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 18. StreamFunction roundtrip
    #[test]
    fn test_stream_function_roundtrip(sf in arb_stream_function()) {
        let bytes = encode_to_vec(&sf).expect("encode StreamFunction");
        let (decoded, _): (StreamFunction, usize) =
            decode_from_slice(&bytes).expect("decode StreamFunction");
        prop_assert_eq!(sf, decoded);
    }

    // 19. MachDescriptor roundtrip
    #[test]
    fn test_mach_descriptor_roundtrip(md in arb_mach_descriptor()) {
        let bytes = encode_to_vec(&md).expect("encode MachDescriptor");
        let (decoded, _): (MachDescriptor, usize) =
            decode_from_slice(&bytes).expect("decode MachDescriptor");
        prop_assert_eq!(md, decoded);
    }

    // 20. HeatTransferCoeff roundtrip
    #[test]
    fn test_heat_transfer_coeff_roundtrip(htc in arb_heat_transfer_coeff()) {
        let bytes = encode_to_vec(&htc).expect("encode HeatTransferCoeff");
        let (decoded, _): (HeatTransferCoeff, usize) =
            decode_from_slice(&bytes).expect("decode HeatTransferCoeff");
        prop_assert_eq!(htc, decoded);
    }

    // 21. Vec<MeshNode> roundtrip (0..6 elements)
    #[test]
    fn test_vec_mesh_node_roundtrip(
        nodes in proptest::collection::vec(arb_mesh_node(), 0..6)
    ) {
        let bytes = encode_to_vec(&nodes).expect("encode Vec<MeshNode>");
        let (decoded, consumed): (Vec<MeshNode>, usize) =
            decode_from_slice(&bytes).expect("decode Vec<MeshNode>");
        prop_assert_eq!(nodes, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 22. Option<VelocityVector> roundtrip covering both Some and None
    #[test]
    fn test_option_velocity_vector_roundtrip(
        opt in proptest::option::of(arb_velocity_vector())
    ) {
        let bytes = encode_to_vec(&opt).expect("encode Option<VelocityVector>");
        let (decoded, consumed): (Option<VelocityVector>, usize) =
            decode_from_slice(&bytes).expect("decode Option<VelocityVector>");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}
