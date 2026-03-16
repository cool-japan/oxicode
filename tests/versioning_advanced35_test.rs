#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NodeStatus {
    Online,
    Offline,
    Degraded,
    Updating,
    Overloaded,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WorkloadType {
    Inference,
    Analytics,
    Stream,
    Batch,
    Control,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NetworkTier {
    Local,
    Fog,
    Cloud,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ComputeResource {
    CPU,
    GPU,
    FPGA,
    NPU,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeNode {
    node_id: u64,
    name: String,
    status: NodeStatus,
    tier: NetworkTier,
    cpu_cores: u8,
    memory_mb: u32,
    storage_gb: u32,
    lat_x1e6: i32,
    lon_x1e6: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WorkloadDeployment {
    deployment_id: u64,
    node_id: u64,
    workload_type: WorkloadType,
    resource: ComputeResource,
    cpu_pct: u8,
    memory_mb: u32,
    latency_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InferenceRequest {
    request_id: u64,
    node_id: u64,
    model_id: u32,
    input_size_kb: u32,
    priority: u8,
    deadline_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdgeMetrics {
    node_id: u64,
    timestamp: u64,
    cpu_util_pct: u8,
    mem_util_pct: u8,
    net_rx_kbps: u32,
    net_tx_kbps: u32,
    active_workloads: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NodeUpdate {
    update_id: u64,
    node_id: u64,
    from_version: String,
    to_version: String,
    status: NodeStatus,
    started_at: u64,
}

#[test]
fn test_edge_node_online_fog_tier_roundtrip() {
    let node = EdgeNode {
        node_id: 1001,
        name: "fog-node-alpha".to_string(),
        status: NodeStatus::Online,
        tier: NetworkTier::Fog,
        cpu_cores: 8,
        memory_mb: 16384,
        storage_gb: 512,
        lat_x1e6: 37_774_929,
        lon_x1e6: -122_419_416,
    };
    let bytes = encode_to_vec(&node).expect("encode EdgeNode Online Fog failed");
    let (decoded, _consumed) =
        decode_from_slice::<EdgeNode>(&bytes).expect("decode EdgeNode Online Fog failed");
    assert_eq!(node, decoded);
}

#[test]
fn test_edge_node_degraded_cloud_tier_roundtrip() {
    let node = EdgeNode {
        node_id: 2002,
        name: "cloud-edge-beta".to_string(),
        status: NodeStatus::Degraded,
        tier: NetworkTier::Cloud,
        cpu_cores: 16,
        memory_mb: 65536,
        storage_gb: 2048,
        lat_x1e6: 51_507_351,
        lon_x1e6: -127_136,
    };
    let bytes = encode_to_vec(&node).expect("encode EdgeNode Degraded Cloud failed");
    let (decoded, _consumed) =
        decode_from_slice::<EdgeNode>(&bytes).expect("decode EdgeNode Degraded Cloud failed");
    assert_eq!(node, decoded);
}

#[test]
fn test_edge_node_versioned_v1_0_0() {
    let node = EdgeNode {
        node_id: 3003,
        name: "local-node-gamma".to_string(),
        status: NodeStatus::Offline,
        tier: NetworkTier::Local,
        cpu_cores: 4,
        memory_mb: 4096,
        storage_gb: 128,
        lat_x1e6: 35_689_487,
        lon_x1e6: 139_691_711,
    };
    let version = Version::new(1, 0, 0);
    let bytes =
        encode_versioned_value(&node, version).expect("encode versioned EdgeNode v1.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<EdgeNode>(&bytes)
        .expect("decode versioned EdgeNode v1.0.0 failed");
    assert_eq!(node, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_edge_node_versioned_v2_0_0() {
    let node = EdgeNode {
        node_id: 4004,
        name: "fog-node-delta".to_string(),
        status: NodeStatus::Overloaded,
        tier: NetworkTier::Fog,
        cpu_cores: 32,
        memory_mb: 131072,
        storage_gb: 4096,
        lat_x1e6: 48_856_614,
        lon_x1e6: 2_352_222,
    };
    let version = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&node, version).expect("encode versioned EdgeNode v2.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<EdgeNode>(&bytes)
        .expect("decode versioned EdgeNode v2.0.0 failed");
    assert_eq!(node, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_workload_deployment_inference_gpu_roundtrip() {
    let deployment = WorkloadDeployment {
        deployment_id: 5005,
        node_id: 1001,
        workload_type: WorkloadType::Inference,
        resource: ComputeResource::GPU,
        cpu_pct: 45,
        memory_mb: 8192,
        latency_ms: 10,
    };
    let bytes = encode_to_vec(&deployment).expect("encode WorkloadDeployment Inference GPU failed");
    let (decoded, consumed) = decode_from_slice::<WorkloadDeployment>(&bytes)
        .expect("decode WorkloadDeployment Inference GPU failed");
    assert_eq!(deployment, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_workload_deployment_analytics_fpga_versioned_v1_3_5() {
    let deployment = WorkloadDeployment {
        deployment_id: 6006,
        node_id: 2002,
        workload_type: WorkloadType::Analytics,
        resource: ComputeResource::FPGA,
        cpu_pct: 70,
        memory_mb: 4096,
        latency_ms: 50,
    };
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&deployment, version)
        .expect("encode versioned WorkloadDeployment v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<WorkloadDeployment>(&bytes)
        .expect("decode versioned WorkloadDeployment v1.3.5 failed");
    assert_eq!(deployment, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert!(consumed > 0);
}

#[test]
fn test_workload_deployment_stream_npu_roundtrip() {
    let deployment = WorkloadDeployment {
        deployment_id: 7007,
        node_id: 3003,
        workload_type: WorkloadType::Stream,
        resource: ComputeResource::NPU,
        cpu_pct: 30,
        memory_mb: 2048,
        latency_ms: 5,
    };
    let bytes = encode_to_vec(&deployment).expect("encode WorkloadDeployment Stream NPU failed");
    let (decoded, _consumed) = decode_from_slice::<WorkloadDeployment>(&bytes)
        .expect("decode WorkloadDeployment Stream NPU failed");
    assert_eq!(deployment, decoded);
}

#[test]
fn test_workload_deployment_batch_cpu_versioned_v2_0_0() {
    let deployment = WorkloadDeployment {
        deployment_id: 8008,
        node_id: 4004,
        workload_type: WorkloadType::Batch,
        resource: ComputeResource::CPU,
        cpu_pct: 90,
        memory_mb: 16384,
        latency_ms: 1000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&deployment, version)
        .expect("encode versioned WorkloadDeployment Batch CPU v2.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<WorkloadDeployment>(&bytes)
        .expect("decode versioned WorkloadDeployment Batch CPU v2.0.0 failed");
    assert_eq!(deployment, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_inference_request_high_priority_roundtrip() {
    let request = InferenceRequest {
        request_id: 9009,
        node_id: 1001,
        model_id: 42,
        input_size_kb: 256,
        priority: 10,
        deadline_ms: 20,
    };
    let bytes = encode_to_vec(&request).expect("encode InferenceRequest high priority failed");
    let (decoded, consumed) = decode_from_slice::<InferenceRequest>(&bytes)
        .expect("decode InferenceRequest high priority failed");
    assert_eq!(request, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_inference_request_versioned_v1_3_5() {
    let request = InferenceRequest {
        request_id: 10010,
        node_id: 2002,
        model_id: 99,
        input_size_kb: 1024,
        priority: 5,
        deadline_ms: 100,
    };
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&request, version)
        .expect("encode versioned InferenceRequest v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<InferenceRequest>(&bytes)
        .expect("decode versioned InferenceRequest v1.3.5 failed");
    assert_eq!(request, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert!(consumed > 0);
}

#[test]
fn test_edge_metrics_normal_load_roundtrip() {
    let metrics = EdgeMetrics {
        node_id: 1001,
        timestamp: 1_700_000_000,
        cpu_util_pct: 55,
        mem_util_pct: 62,
        net_rx_kbps: 10240,
        net_tx_kbps: 5120,
        active_workloads: 7,
    };
    let bytes = encode_to_vec(&metrics).expect("encode EdgeMetrics normal load failed");
    let (decoded, consumed) =
        decode_from_slice::<EdgeMetrics>(&bytes).expect("decode EdgeMetrics normal load failed");
    assert_eq!(metrics, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_edge_metrics_peak_load_versioned_v1_0_0() {
    let metrics = EdgeMetrics {
        node_id: 4004,
        timestamp: 1_700_001_000,
        cpu_util_pct: 98,
        mem_util_pct: 95,
        net_rx_kbps: 102400,
        net_tx_kbps: 51200,
        active_workloads: 50,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&metrics, version)
        .expect("encode versioned EdgeMetrics peak load v1.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<EdgeMetrics>(&bytes)
        .expect("decode versioned EdgeMetrics peak load v1.0.0 failed");
    assert_eq!(metrics, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_node_update_in_progress_versioned_v2_0_0() {
    let update = NodeUpdate {
        update_id: 11011,
        node_id: 3003,
        from_version: "1.5.2".to_string(),
        to_version: "2.0.0".to_string(),
        status: NodeStatus::Updating,
        started_at: 1_700_002_000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&update, version)
        .expect("encode versioned NodeUpdate Updating v2.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<NodeUpdate>(&bytes)
        .expect("decode versioned NodeUpdate Updating v2.0.0 failed");
    assert_eq!(update, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_node_update_completed_roundtrip() {
    let update = NodeUpdate {
        update_id: 12012,
        node_id: 2002,
        from_version: "0.9.0".to_string(),
        to_version: "1.0.0".to_string(),
        status: NodeStatus::Online,
        started_at: 1_700_003_000,
    };
    let bytes = encode_to_vec(&update).expect("encode NodeUpdate completed failed");
    let (decoded, _consumed) =
        decode_from_slice::<NodeUpdate>(&bytes).expect("decode NodeUpdate completed failed");
    assert_eq!(update, decoded);
}

#[test]
fn test_vec_of_edge_nodes_versioned_v1_3_5() {
    let nodes = vec![
        EdgeNode {
            node_id: 13001,
            name: "cluster-node-01".to_string(),
            status: NodeStatus::Online,
            tier: NetworkTier::Local,
            cpu_cores: 4,
            memory_mb: 8192,
            storage_gb: 256,
            lat_x1e6: 40_712_776,
            lon_x1e6: -74_005_974,
        },
        EdgeNode {
            node_id: 13002,
            name: "cluster-node-02".to_string(),
            status: NodeStatus::Degraded,
            tier: NetworkTier::Fog,
            cpu_cores: 8,
            memory_mb: 16384,
            storage_gb: 512,
            lat_x1e6: 34_052_235,
            lon_x1e6: -118_243_683,
        },
        EdgeNode {
            node_id: 13003,
            name: "cluster-node-03".to_string(),
            status: NodeStatus::Overloaded,
            tier: NetworkTier::Cloud,
            cpu_cores: 64,
            memory_mb: 262144,
            storage_gb: 8192,
            lat_x1e6: 41_878_114,
            lon_x1e6: -87_629_798,
        },
    ];
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&nodes, version)
        .expect("encode versioned Vec<EdgeNode> v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<EdgeNode>>(&bytes)
        .expect("decode versioned Vec<EdgeNode> v1.3.5 failed");
    assert_eq!(nodes, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_vec_of_workload_deployments_versioned_v1_0_0() {
    let deployments = vec![
        WorkloadDeployment {
            deployment_id: 14001,
            node_id: 1001,
            workload_type: WorkloadType::Inference,
            resource: ComputeResource::NPU,
            cpu_pct: 40,
            memory_mb: 4096,
            latency_ms: 8,
        },
        WorkloadDeployment {
            deployment_id: 14002,
            node_id: 2002,
            workload_type: WorkloadType::Control,
            resource: ComputeResource::CPU,
            cpu_pct: 15,
            memory_mb: 512,
            latency_ms: 2,
        },
    ];
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&deployments, version)
        .expect("encode versioned Vec<WorkloadDeployment> v1.0.0 failed");
    let (decoded, ver, _consumed) = decode_versioned_value::<Vec<WorkloadDeployment>>(&bytes)
        .expect("decode versioned Vec<WorkloadDeployment> v1.0.0 failed");
    assert_eq!(deployments, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_vec_of_inference_requests_versioned_v2_0_0() {
    let requests = vec![
        InferenceRequest {
            request_id: 15001,
            node_id: 3003,
            model_id: 7,
            input_size_kb: 128,
            priority: 9,
            deadline_ms: 15,
        },
        InferenceRequest {
            request_id: 15002,
            node_id: 3003,
            model_id: 7,
            input_size_kb: 512,
            priority: 3,
            deadline_ms: 200,
        },
        InferenceRequest {
            request_id: 15003,
            node_id: 4004,
            model_id: 21,
            input_size_kb: 2048,
            priority: 1,
            deadline_ms: 5000,
        },
    ];
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&requests, version)
        .expect("encode versioned Vec<InferenceRequest> v2.0.0 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<InferenceRequest>>(&bytes)
        .expect("decode versioned Vec<InferenceRequest> v2.0.0 failed");
    assert_eq!(requests, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_vec_of_edge_metrics_roundtrip() {
    let metrics_batch = vec![
        EdgeMetrics {
            node_id: 1001,
            timestamp: 1_700_010_000,
            cpu_util_pct: 20,
            mem_util_pct: 35,
            net_rx_kbps: 2048,
            net_tx_kbps: 1024,
            active_workloads: 2,
        },
        EdgeMetrics {
            node_id: 2002,
            timestamp: 1_700_010_000,
            cpu_util_pct: 75,
            mem_util_pct: 80,
            net_rx_kbps: 51200,
            net_tx_kbps: 25600,
            active_workloads: 15,
        },
    ];
    let bytes = encode_to_vec(&metrics_batch).expect("encode Vec<EdgeMetrics> failed");
    let (decoded, consumed) =
        decode_from_slice::<Vec<EdgeMetrics>>(&bytes).expect("decode Vec<EdgeMetrics> failed");
    assert_eq!(metrics_batch, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_node_status_all_variants_versioned_v1_3_5() {
    let statuses = vec![
        NodeStatus::Online,
        NodeStatus::Offline,
        NodeStatus::Degraded,
        NodeStatus::Updating,
        NodeStatus::Overloaded,
    ];
    let version = Version::new(1, 3, 5);
    let bytes = encode_versioned_value(&statuses, version)
        .expect("encode versioned Vec<NodeStatus> v1.3.5 failed");
    let (decoded, ver, consumed) = decode_versioned_value::<Vec<NodeStatus>>(&bytes)
        .expect("decode versioned Vec<NodeStatus> v1.3.5 failed");
    assert_eq!(statuses, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 5);
    assert!(consumed > 0);
}

#[test]
fn test_compute_resource_and_workload_type_all_variants_roundtrip() {
    let resources = vec![
        ComputeResource::CPU,
        ComputeResource::GPU,
        ComputeResource::FPGA,
        ComputeResource::NPU,
    ];
    let workloads = vec![
        WorkloadType::Inference,
        WorkloadType::Analytics,
        WorkloadType::Stream,
        WorkloadType::Batch,
        WorkloadType::Control,
    ];
    let res_bytes = encode_to_vec(&resources).expect("encode Vec<ComputeResource> failed");
    let (decoded_res, consumed_res) = decode_from_slice::<Vec<ComputeResource>>(&res_bytes)
        .expect("decode Vec<ComputeResource> failed");
    assert_eq!(resources, decoded_res);
    assert_eq!(consumed_res, res_bytes.len());

    let wl_bytes = encode_to_vec(&workloads).expect("encode Vec<WorkloadType> failed");
    let (decoded_wl, consumed_wl) =
        decode_from_slice::<Vec<WorkloadType>>(&wl_bytes).expect("decode Vec<WorkloadType> failed");
    assert_eq!(workloads, decoded_wl);
    assert_eq!(consumed_wl, wl_bytes.len());
}

#[test]
fn test_consumed_bytes_verification_edge_node_v1_0_0() {
    let node = EdgeNode {
        node_id: 16016,
        name: "bytes-check-node".to_string(),
        status: NodeStatus::Online,
        tier: NetworkTier::Fog,
        cpu_cores: 12,
        memory_mb: 32768,
        storage_gb: 1024,
        lat_x1e6: -33_868_820,
        lon_x1e6: 151_209_290,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&node, version)
        .expect("encode versioned EdgeNode for bytes check failed");
    let total_len = bytes.len();
    let (decoded, ver, consumed) = decode_versioned_value::<EdgeNode>(&bytes)
        .expect("decode versioned EdgeNode for bytes check failed");
    assert_eq!(node, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, total_len);
}

#[test]
fn test_edge_metrics_zero_load_and_network_tier_local_roundtrip() {
    let metrics = EdgeMetrics {
        node_id: 17017,
        timestamp: 0,
        cpu_util_pct: 0,
        mem_util_pct: 0,
        net_rx_kbps: 0,
        net_tx_kbps: 0,
        active_workloads: 0,
    };
    let tier = NetworkTier::Local;
    let version = Version::new(2, 0, 0);
    let metrics_bytes = encode_versioned_value(&metrics, version)
        .expect("encode versioned EdgeMetrics zero load failed");
    let (decoded_metrics, ver, consumed) = decode_versioned_value::<EdgeMetrics>(&metrics_bytes)
        .expect("decode versioned EdgeMetrics zero load failed");
    assert_eq!(metrics, decoded_metrics);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert_eq!(consumed, metrics_bytes.len());

    let tier_bytes = encode_to_vec(&tier).expect("encode NetworkTier::Local failed");
    let (decoded_tier, _consumed_tier) =
        decode_from_slice::<NetworkTier>(&tier_bytes).expect("decode NetworkTier::Local failed");
    assert_eq!(tier, decoded_tier);
}
