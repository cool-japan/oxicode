//! Advanced tests for geospatial analysis and GIS domain types.
//! 22 test functions covering complex enums, nested enums, and struct/enum compositions.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Coordinate {
    Wgs84 {
        lat_e7: i64,
        lon_e7: i64,
    },
    UtmZone {
        zone: u8,
        easting_mm: i64,
        northing_mm: i64,
        hemisphere: Hemisphere,
    },
    StatePlane {
        zone_code: u16,
        x_mm: i64,
        y_mm: i64,
        datum: Datum,
    },
    Ecef {
        x_mm: i64,
        y_mm: i64,
        z_mm: i64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Hemisphere {
    North,
    South,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Datum {
    Nad83,
    Nad27,
    Wgs84,
    Etrs89,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Crs {
    Geographic {
        epsg: u32,
        datum: Datum,
    },
    Projected {
        epsg: u32,
        zone_info: ProjectionZone,
    },
    Local {
        name_len: u8,
        origin: Coordinate,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProjectionZone {
    Utm {
        zone: u8,
        hemisphere: Hemisphere,
    },
    StatePlane {
        fips_code: u16,
    },
    LambertConformal {
        std_parallel1_e5: i32,
        std_parallel2_e5: i32,
    },
    TransverseMercator {
        central_meridian_e5: i32,
        scale_factor_e8: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Geometry {
    Point(Coordinate),
    LineString {
        coords: Vec<Coordinate>,
        is_closed: bool,
    },
    Polygon {
        exterior: Vec<Coordinate>,
        holes: Vec<Vec<Coordinate>>,
    },
    MultiPolygon {
        polygons: Vec<PolygonData>,
    },
    MultiPoint(Vec<Coordinate>),
    GeometryCollection(Vec<Geometry>),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PolygonData {
    exterior: Vec<Coordinate>,
    holes: Vec<Vec<Coordinate>>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RasterTile {
    Png {
        zoom: u8,
        x: u32,
        y: u32,
        size_bytes: u32,
    },
    Mvt {
        zoom: u8,
        x: u32,
        y: u32,
        layers: Vec<VectorLayer>,
    },
    Terrain {
        zoom: u8,
        x: u32,
        y: u32,
        elevation_model: ElevationModel,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VectorLayer {
    name_hash: u64,
    feature_count: u32,
    geom_type: GeomColumnType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GeomColumnType {
    PointColumn,
    LineColumn,
    PolygonColumn,
    Mixed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeatureAttribute {
    IntegerVal(i64),
    FloatVal(i64),
    StringHash(u64),
    BoolVal(bool),
    DateEpochDays(i32),
    NullVal,
    ListVal(Vec<FeatureAttribute>),
    Nested {
        key_hash: u64,
        value: Box<FeatureAttribute>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpatialIndexNode {
    RTreeLeaf {
        min_x_e7: i64,
        min_y_e7: i64,
        max_x_e7: i64,
        max_y_e7: i64,
        feature_id: u64,
    },
    RTreeBranch {
        min_x_e7: i64,
        min_y_e7: i64,
        max_x_e7: i64,
        max_y_e7: i64,
        children: Vec<SpatialIndexNode>,
    },
    QuadTreeLeaf {
        center_x_e7: i64,
        center_y_e7: i64,
        feature_ids: Vec<u64>,
    },
    QuadTreeBranch {
        center_x_e7: i64,
        center_y_e7: i64,
        half_extent_e7: i64,
        nw: Option<Box<SpatialIndexNode>>,
        ne: Option<Box<SpatialIndexNode>>,
        sw: Option<Box<SpatialIndexNode>>,
        se: Option<Box<SpatialIndexNode>>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GeocodingResult {
    ExactMatch {
        coord: Coordinate,
        confidence_pct: u8,
        address: ParsedAddress,
    },
    Interpolated {
        coord: Coordinate,
        side: StreetSide,
        range_low: u32,
        range_high: u32,
    },
    Centroid {
        coord: Coordinate,
        level: CentroidLevel,
    },
    NoMatch {
        reason: GeocodingFailure,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StreetSide {
    Left,
    Right,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CentroidLevel {
    Country,
    State,
    County,
    City,
    Zip,
    Neighborhood,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GeocodingFailure {
    InvalidAddress,
    Ambiguous { candidate_count: u16 },
    OutOfCoverage,
    ServiceUnavailable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParsedAddress {
    house_number: Option<u32>,
    street_hash: u64,
    city_hash: u64,
    state_code: u16,
    postal_code: u32,
    country_code: u16,
    unit: Option<AddressUnit>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AddressUnit {
    Apartment(u16),
    Suite(u16),
    Floor(u8),
    Building(u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoutingAlgorithm {
    Dijkstra {
        graph_nodes: u32,
        graph_edges: u32,
        result: RoutingResult,
    },
    AStar {
        graph_nodes: u32,
        heuristic: AStarHeuristic,
        result: RoutingResult,
    },
    ContractionHierarchy {
        level: u16,
        shortcuts_count: u32,
        result: RoutingResult,
    },
    BidirectionalDijkstra {
        forward_settled: u32,
        backward_settled: u32,
        meeting_node: u64,
        result: RoutingResult,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AStarHeuristic {
    Euclidean,
    Haversine,
    Manhattan,
    Chebyshev,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoutingResult {
    PathFound {
        distance_mm: u64,
        duration_ms: u64,
        waypoint_count: u32,
        maneuvers: Vec<Maneuver>,
    },
    NoPath {
        reason: NoPathReason,
    },
    PartialPath {
        distance_mm: u64,
        blocked_at: Coordinate,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Maneuver {
    TurnLeft { angle_deg: u16 },
    TurnRight { angle_deg: u16 },
    UTurn,
    Continue,
    MergeOnto { road_class: RoadClass },
    ExitRamp { exit_number: u16 },
    Roundabout { exit_index: u8 },
    Arrive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RoadClass {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Service,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NoPathReason {
    Disconnected,
    OneWayRestriction,
    TurnRestriction,
    TemporaryClosure,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ElevationModel {
    Dem {
        resolution_m: u16,
        vertical_datum: VerticalDatum,
        source: ElevationSource,
    },
    Dsm {
        resolution_m: u16,
        includes_vegetation: bool,
        source: ElevationSource,
    },
    Dtm {
        resolution_m: u16,
        filtering: TerrainFiltering,
        source: ElevationSource,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VerticalDatum {
    Egm96,
    Egm2008,
    Navd88,
    Msl,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ElevationSource {
    Lidar {
        point_density: u32,
        classification: LidarClassification,
    },
    Radar {
        band: RadarBand,
    },
    Photogrammetry {
        gsd_mm: u32,
    },
    Interpolated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LidarClassification {
    Ground,
    Vegetation,
    Building,
    Water,
    Unclassified,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RadarBand {
    CBand,
    XBand,
    LBand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TerrainFiltering {
    Progressive {
        window_size: u16,
        slope_threshold_e2: u16,
    },
    Cloth {
        cloth_resolution_cm: u16,
        iterations: u16,
    },
    Morphological {
        kernel_size: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LandUseClass {
    UrbanResidential {
        density: UrbanDensity,
        building_pct: u8,
    },
    UrbanCommercial {
        floor_area_ratio_e2: u16,
    },
    Agricultural {
        crop_type: CropType,
        irrigation: IrrigationType,
    },
    Forest {
        canopy_cover_pct: u8,
        species_group: ForestType,
    },
    Wetland {
        hydrology: WetlandHydrology,
    },
    Barren,
    Water {
        depth_class: WaterDepthClass,
    },
    Transportation {
        road_class: RoadClass,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum UrbanDensity {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CropType {
    Grain,
    Vegetable,
    Orchard,
    Pasture,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IrrigationType {
    Rainfed,
    SurfaceIrrigation,
    Sprinkler,
    Drip,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ForestType {
    Deciduous,
    Coniferous,
    Mixed,
    Tropical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WetlandHydrology {
    Permanent,
    Seasonal,
    Tidal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaterDepthClass {
    Shallow,
    Moderate,
    Deep,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CadastralParcel {
    Urban {
        parcel_id: u64,
        area_sq_m: u32,
        zoning: ZoningCode,
        boundary: Geometry,
        encumbrances: Vec<Encumbrance>,
    },
    Rural {
        parcel_id: u64,
        area_sq_m: u64,
        land_use: LandUseClass,
        boundary: Geometry,
        easements: Vec<Easement>,
    },
    Condominium {
        parcel_id: u64,
        unit_number: u16,
        floor: u8,
        strata_lot: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ZoningCode {
    R1SingleFamily,
    R2MultiFamily,
    C1LocalCommercial,
    C2GeneralCommercial,
    I1LightIndustrial,
    I2HeavyIndustrial,
    A1Agricultural,
    Os1OpenSpace,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Encumbrance {
    Mortgage { holder_id: u64, amount_cents: u64 },
    Lien { type_code: u16, amount_cents: u64 },
    Covenant { restriction_hash: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Easement {
    Utility { width_mm: u32, provider_id: u64 },
    Access { width_mm: u32, beneficiary_id: u64 },
    Conservation { area_sq_m: u32, authority_id: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RemoteSensingBand {
    Rgb {
        red_nm: u16,
        green_nm: u16,
        blue_nm: u16,
        bit_depth: u8,
    },
    Ndvi {
        nir_nm: u16,
        red_nm: u16,
        index_scale: i16,
    },
    Thermal {
        wavelength_nm: u32,
        temp_range: ThermalRange,
    },
    Sar {
        polarization: SarPolarization,
        frequency_mhz: u32,
    },
    Multispectral {
        bands: Vec<SpectralBand>,
    },
    Hyperspectral {
        start_nm: u16,
        end_nm: u16,
        band_count: u16,
        fwhm_nm: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThermalRange {
    min_kelvin_e2: i32,
    max_kelvin_e2: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SarPolarization {
    Hh,
    Vv,
    Hv,
    Vh,
    Dual,
    Quad,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectralBand {
    center_nm: u16,
    width_nm: u8,
    gain_e4: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpatialOperation {
    Intersection {
        left: Box<SpatialOperand>,
        right: Box<SpatialOperand>,
        tolerance_mm: u32,
    },
    Union {
        operands: Vec<SpatialOperand>,
    },
    Difference {
        base: Box<SpatialOperand>,
        subtract: Box<SpatialOperand>,
    },
    Buffer {
        source: Box<SpatialOperand>,
        distance_mm: i64,
        segments: u8,
    },
    Clip {
        input: Box<SpatialOperand>,
        clip_region: Geometry,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpatialOperand {
    FeatureRef(u64),
    Literal(Geometry),
    SubOperation(Box<SpatialOperation>),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MapProjectionParams {
    Mercator {
        central_meridian_e5: i32,
    },
    LambertConformalConic {
        std_parallel1_e5: i32,
        std_parallel2_e5: i32,
        lat_origin_e5: i32,
        lon_origin_e5: i32,
        false_easting_mm: i64,
        false_northing_mm: i64,
    },
    TransverseMercator {
        central_meridian_e5: i32,
        scale_factor_e8: u32,
        false_easting_mm: i64,
        false_northing_mm: i64,
    },
    AlbersEqualArea {
        std_parallel1_e5: i32,
        std_parallel2_e5: i32,
        lat_origin_e5: i32,
        lon_origin_e5: i32,
    },
    Stereographic {
        lat_origin_e5: i32,
        lon_origin_e5: i32,
        scale_factor_e8: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TopologyError {
    SelfIntersection {
        geometry_id: u64,
        at: Coordinate,
    },
    GapBetweenPolygons {
        poly_a: u64,
        poly_b: u64,
        gap_area_sq_mm: u64,
    },
    OverlappingPolygons {
        poly_a: u64,
        poly_b: u64,
        overlap_area_sq_mm: u64,
    },
    DanglingEdge {
        edge_id: u64,
        endpoint: Coordinate,
    },
    UnclosedRing {
        geometry_id: u64,
        start: Coordinate,
        end: Coordinate,
    },
    InvalidWinding {
        geometry_id: u64,
        expected: WindingOrder,
    },
    DuplicateVertex {
        geometry_id: u64,
        coord: Coordinate,
        count: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WindingOrder {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TilePyramid {
    Xyz {
        min_zoom: u8,
        max_zoom: u8,
        tile_size: u16,
        format: TileFormat,
        bounds: BoundingBox,
    },
    Tms {
        min_zoom: u8,
        max_zoom: u8,
        tile_size: u16,
        format: TileFormat,
        crs: Crs,
    },
    QuadKey {
        max_level: u8,
        tiles: Vec<QuadKeyTile>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TileFormat {
    PngRgba,
    Jpeg { quality: u8 },
    Webp { lossless: bool },
    Pbf,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoundingBox {
    min_x_e7: i64,
    min_y_e7: i64,
    max_x_e7: i64,
    max_y_e7: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuadKeyTile {
    key_value: u64,
    size_bytes: u32,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_wgs84_point_geometry() {
    let geom = Geometry::Point(Coordinate::Wgs84 {
        lat_e7: 408_500_000,
        lon_e7: -739_500_000,
    });
    let bytes = encode_to_vec(&geom).expect("encode wgs84 point");
    let (decoded, _) = decode_from_slice::<Geometry>(&bytes).expect("decode wgs84 point");
    assert_eq!(geom, decoded);
}

#[test]
fn test_utm_linestring() {
    let coords = vec![
        Coordinate::UtmZone {
            zone: 18,
            easting_mm: 583_960_000,
            northing_mm: 4_507_523_000,
            hemisphere: Hemisphere::North,
        },
        Coordinate::UtmZone {
            zone: 18,
            easting_mm: 584_100_000,
            northing_mm: 4_507_600_000,
            hemisphere: Hemisphere::North,
        },
        Coordinate::UtmZone {
            zone: 18,
            easting_mm: 584_250_000,
            northing_mm: 4_507_700_000,
            hemisphere: Hemisphere::North,
        },
    ];
    let geom = Geometry::LineString {
        coords,
        is_closed: false,
    };
    let bytes = encode_to_vec(&geom).expect("encode utm linestring");
    let (decoded, _) = decode_from_slice::<Geometry>(&bytes).expect("decode utm linestring");
    assert_eq!(geom, decoded);
}

#[test]
fn test_polygon_with_hole() {
    let exterior = vec![
        Coordinate::Wgs84 {
            lat_e7: 0,
            lon_e7: 0,
        },
        Coordinate::Wgs84 {
            lat_e7: 100_000_000,
            lon_e7: 0,
        },
        Coordinate::Wgs84 {
            lat_e7: 100_000_000,
            lon_e7: 100_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 0,
            lon_e7: 100_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 0,
            lon_e7: 0,
        },
    ];
    let hole = vec![
        Coordinate::Wgs84 {
            lat_e7: 25_000_000,
            lon_e7: 25_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 75_000_000,
            lon_e7: 25_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 75_000_000,
            lon_e7: 75_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 25_000_000,
            lon_e7: 75_000_000,
        },
        Coordinate::Wgs84 {
            lat_e7: 25_000_000,
            lon_e7: 25_000_000,
        },
    ];
    let geom = Geometry::Polygon {
        exterior,
        holes: vec![hole],
    };
    let bytes = encode_to_vec(&geom).expect("encode polygon with hole");
    let (decoded, _) = decode_from_slice::<Geometry>(&bytes).expect("decode polygon with hole");
    assert_eq!(geom, decoded);
}

#[test]
fn test_multipolygon_geometry() {
    let poly1 = PolygonData {
        exterior: vec![
            Coordinate::Wgs84 {
                lat_e7: 0,
                lon_e7: 0,
            },
            Coordinate::Wgs84 {
                lat_e7: 10_000_000,
                lon_e7: 0,
            },
            Coordinate::Wgs84 {
                lat_e7: 0,
                lon_e7: 10_000_000,
            },
            Coordinate::Wgs84 {
                lat_e7: 0,
                lon_e7: 0,
            },
        ],
        holes: vec![],
    };
    let poly2 = PolygonData {
        exterior: vec![
            Coordinate::Wgs84 {
                lat_e7: 50_000_000,
                lon_e7: 50_000_000,
            },
            Coordinate::Wgs84 {
                lat_e7: 60_000_000,
                lon_e7: 50_000_000,
            },
            Coordinate::Wgs84 {
                lat_e7: 60_000_000,
                lon_e7: 60_000_000,
            },
            Coordinate::Wgs84 {
                lat_e7: 50_000_000,
                lon_e7: 50_000_000,
            },
        ],
        holes: vec![],
    };
    let geom = Geometry::MultiPolygon {
        polygons: vec![poly1, poly2],
    };
    let bytes = encode_to_vec(&geom).expect("encode multipolygon");
    let (decoded, _) = decode_from_slice::<Geometry>(&bytes).expect("decode multipolygon");
    assert_eq!(geom, decoded);
}

#[test]
fn test_crs_projected_utm() {
    let crs = Crs::Projected {
        epsg: 32618,
        zone_info: ProjectionZone::Utm {
            zone: 18,
            hemisphere: Hemisphere::North,
        },
    };
    let bytes = encode_to_vec(&crs).expect("encode crs");
    let (decoded, _) = decode_from_slice::<Crs>(&bytes).expect("decode crs");
    assert_eq!(crs, decoded);
}

#[test]
fn test_raster_tile_mvt_with_layers() {
    let tile = RasterTile::Mvt {
        zoom: 14,
        x: 4823,
        y: 6160,
        layers: vec![
            VectorLayer {
                name_hash: 0xABCD_1234,
                feature_count: 512,
                geom_type: GeomColumnType::PolygonColumn,
            },
            VectorLayer {
                name_hash: 0xEF01_5678,
                feature_count: 1024,
                geom_type: GeomColumnType::LineColumn,
            },
            VectorLayer {
                name_hash: 0x9876_5432,
                feature_count: 256,
                geom_type: GeomColumnType::PointColumn,
            },
        ],
    };
    let bytes = encode_to_vec(&tile).expect("encode mvt tile");
    let (decoded, _) = decode_from_slice::<RasterTile>(&bytes).expect("decode mvt tile");
    assert_eq!(tile, decoded);
}

#[test]
fn test_nested_feature_attributes() {
    let attr = FeatureAttribute::Nested {
        key_hash: 0x1111_2222_3333_4444,
        value: Box::new(FeatureAttribute::ListVal(vec![
            FeatureAttribute::IntegerVal(42),
            FeatureAttribute::FloatVal(314_159),
            FeatureAttribute::Nested {
                key_hash: 0x5555_6666,
                value: Box::new(FeatureAttribute::BoolVal(true)),
            },
            FeatureAttribute::NullVal,
            FeatureAttribute::DateEpochDays(19432),
        ])),
    };
    let bytes = encode_to_vec(&attr).expect("encode nested attribute");
    let (decoded, _) =
        decode_from_slice::<FeatureAttribute>(&bytes).expect("decode nested attribute");
    assert_eq!(attr, decoded);
}

#[test]
fn test_rtree_spatial_index() {
    let tree = SpatialIndexNode::RTreeBranch {
        min_x_e7: -740_000_000,
        min_y_e7: 405_000_000,
        max_x_e7: -735_000_000,
        max_y_e7: 410_000_000,
        children: vec![
            SpatialIndexNode::RTreeLeaf {
                min_x_e7: -740_000_000,
                min_y_e7: 405_000_000,
                max_x_e7: -738_000_000,
                max_y_e7: 407_000_000,
                feature_id: 1001,
            },
            SpatialIndexNode::RTreeLeaf {
                min_x_e7: -737_000_000,
                min_y_e7: 407_000_000,
                max_x_e7: -735_000_000,
                max_y_e7: 410_000_000,
                feature_id: 1002,
            },
        ],
    };
    let bytes = encode_to_vec(&tree).expect("encode rtree");
    let (decoded, _) = decode_from_slice::<SpatialIndexNode>(&bytes).expect("decode rtree");
    assert_eq!(tree, decoded);
}

#[test]
fn test_quadtree_spatial_index() {
    let tree = SpatialIndexNode::QuadTreeBranch {
        center_x_e7: 0,
        center_y_e7: 0,
        half_extent_e7: 1_800_000_000,
        nw: Some(Box::new(SpatialIndexNode::QuadTreeLeaf {
            center_x_e7: -900_000_000,
            center_y_e7: 900_000_000,
            feature_ids: vec![10, 20, 30],
        })),
        ne: Some(Box::new(SpatialIndexNode::QuadTreeLeaf {
            center_x_e7: 900_000_000,
            center_y_e7: 900_000_000,
            feature_ids: vec![40, 50],
        })),
        sw: None,
        se: Some(Box::new(SpatialIndexNode::QuadTreeBranch {
            center_x_e7: 900_000_000,
            center_y_e7: -900_000_000,
            half_extent_e7: 450_000_000,
            nw: None,
            ne: None,
            sw: None,
            se: Some(Box::new(SpatialIndexNode::QuadTreeLeaf {
                center_x_e7: 1_350_000_000,
                center_y_e7: -1_350_000_000,
                feature_ids: vec![99],
            })),
        })),
    };
    let bytes = encode_to_vec(&tree).expect("encode quadtree");
    let (decoded, _) = decode_from_slice::<SpatialIndexNode>(&bytes).expect("decode quadtree");
    assert_eq!(tree, decoded);
}

#[test]
fn test_geocoding_exact_match() {
    let result = GeocodingResult::ExactMatch {
        coord: Coordinate::Wgs84 {
            lat_e7: 408_283_000,
            lon_e7: -740_585_000,
        },
        confidence_pct: 98,
        address: ParsedAddress {
            house_number: Some(350),
            street_hash: 0xDEAD_BEEF_CAFE,
            city_hash: 0x1234_5678_9ABC,
            state_code: 36,
            postal_code: 10001,
            country_code: 840,
            unit: Some(AddressUnit::Floor(12)),
        },
    };
    let bytes = encode_to_vec(&result).expect("encode geocoding exact");
    let (decoded, _) =
        decode_from_slice::<GeocodingResult>(&bytes).expect("decode geocoding exact");
    assert_eq!(result, decoded);
}

#[test]
fn test_dijkstra_routing_with_maneuvers() {
    let algo = RoutingAlgorithm::Dijkstra {
        graph_nodes: 150_000,
        graph_edges: 420_000,
        result: RoutingResult::PathFound {
            distance_mm: 12_345_678,
            duration_ms: 900_000,
            waypoint_count: 5,
            maneuvers: vec![
                Maneuver::Continue,
                Maneuver::TurnRight { angle_deg: 90 },
                Maneuver::MergeOnto {
                    road_class: RoadClass::Motorway,
                },
                Maneuver::ExitRamp { exit_number: 14 },
                Maneuver::TurnLeft { angle_deg: 45 },
                Maneuver::Roundabout { exit_index: 3 },
                Maneuver::Arrive,
            ],
        },
    };
    let bytes = encode_to_vec(&algo).expect("encode dijkstra");
    let (decoded, _) = decode_from_slice::<RoutingAlgorithm>(&bytes).expect("decode dijkstra");
    assert_eq!(algo, decoded);
}

#[test]
fn test_astar_routing_no_path() {
    let algo = RoutingAlgorithm::AStar {
        graph_nodes: 50_000,
        heuristic: AStarHeuristic::Haversine,
        result: RoutingResult::NoPath {
            reason: NoPathReason::Disconnected,
        },
    };
    let bytes = encode_to_vec(&algo).expect("encode astar no path");
    let (decoded, _) = decode_from_slice::<RoutingAlgorithm>(&bytes).expect("decode astar no path");
    assert_eq!(algo, decoded);
}

#[test]
fn test_contraction_hierarchy_partial_path() {
    let algo = RoutingAlgorithm::ContractionHierarchy {
        level: 12,
        shortcuts_count: 85_000,
        result: RoutingResult::PartialPath {
            distance_mm: 5_000_000,
            blocked_at: Coordinate::Wgs84 {
                lat_e7: 488_500_000,
                lon_e7: 23_500_000,
            },
        },
    };
    let bytes = encode_to_vec(&algo).expect("encode ch partial");
    let (decoded, _) = decode_from_slice::<RoutingAlgorithm>(&bytes).expect("decode ch partial");
    assert_eq!(algo, decoded);
}

#[test]
fn test_elevation_dem_lidar() {
    let model = ElevationModel::Dem {
        resolution_m: 1,
        vertical_datum: VerticalDatum::Navd88,
        source: ElevationSource::Lidar {
            point_density: 8,
            classification: LidarClassification::Ground,
        },
    };
    let bytes = encode_to_vec(&model).expect("encode dem lidar");
    let (decoded, _) = decode_from_slice::<ElevationModel>(&bytes).expect("decode dem lidar");
    assert_eq!(model, decoded);
}

#[test]
fn test_land_use_agricultural() {
    let lu = LandUseClass::Agricultural {
        crop_type: CropType::Orchard,
        irrigation: IrrigationType::Drip,
    };
    let bytes = encode_to_vec(&lu).expect("encode land use");
    let (decoded, _) = decode_from_slice::<LandUseClass>(&bytes).expect("decode land use");
    assert_eq!(lu, decoded);
}

#[test]
fn test_cadastral_urban_parcel() {
    let parcel = CadastralParcel::Urban {
        parcel_id: 123_456_789,
        area_sq_m: 850,
        zoning: ZoningCode::R2MultiFamily,
        boundary: Geometry::Polygon {
            exterior: vec![
                Coordinate::StatePlane {
                    zone_code: 3104,
                    x_mm: 100_000,
                    y_mm: 200_000,
                    datum: Datum::Nad83,
                },
                Coordinate::StatePlane {
                    zone_code: 3104,
                    x_mm: 130_000,
                    y_mm: 200_000,
                    datum: Datum::Nad83,
                },
                Coordinate::StatePlane {
                    zone_code: 3104,
                    x_mm: 130_000,
                    y_mm: 230_000,
                    datum: Datum::Nad83,
                },
                Coordinate::StatePlane {
                    zone_code: 3104,
                    x_mm: 100_000,
                    y_mm: 230_000,
                    datum: Datum::Nad83,
                },
                Coordinate::StatePlane {
                    zone_code: 3104,
                    x_mm: 100_000,
                    y_mm: 200_000,
                    datum: Datum::Nad83,
                },
            ],
            holes: vec![],
        },
        encumbrances: vec![
            Encumbrance::Mortgage {
                holder_id: 9001,
                amount_cents: 450_000_00,
            },
            Encumbrance::Covenant {
                restriction_hash: 0xFEDC_BA98,
            },
        ],
    };
    let bytes = encode_to_vec(&parcel).expect("encode urban parcel");
    let (decoded, _) = decode_from_slice::<CadastralParcel>(&bytes).expect("decode urban parcel");
    assert_eq!(parcel, decoded);
}

#[test]
fn test_remote_sensing_ndvi() {
    let band = RemoteSensingBand::Ndvi {
        nir_nm: 842,
        red_nm: 665,
        index_scale: 10000,
    };
    let bytes = encode_to_vec(&band).expect("encode ndvi");
    let (decoded, _) = decode_from_slice::<RemoteSensingBand>(&bytes).expect("decode ndvi");
    assert_eq!(band, decoded);
}

#[test]
fn test_spatial_intersection_operation() {
    let op = SpatialOperation::Intersection {
        left: Box::new(SpatialOperand::FeatureRef(42)),
        right: Box::new(SpatialOperand::Literal(Geometry::Point(
            Coordinate::Wgs84 {
                lat_e7: 350_000_000,
                lon_e7: -1_180_000_000,
            },
        ))),
        tolerance_mm: 10,
    };
    let bytes = encode_to_vec(&op).expect("encode spatial intersection");
    let (decoded, _) =
        decode_from_slice::<SpatialOperation>(&bytes).expect("decode spatial intersection");
    assert_eq!(op, decoded);
}

#[test]
fn test_nested_spatial_operations() {
    let op = SpatialOperation::Buffer {
        source: Box::new(SpatialOperand::SubOperation(Box::new(
            SpatialOperation::Difference {
                base: Box::new(SpatialOperand::FeatureRef(100)),
                subtract: Box::new(SpatialOperand::FeatureRef(200)),
            },
        ))),
        distance_mm: 5_000,
        segments: 16,
    };
    let bytes = encode_to_vec(&op).expect("encode nested spatial op");
    let (decoded, _) =
        decode_from_slice::<SpatialOperation>(&bytes).expect("decode nested spatial op");
    assert_eq!(op, decoded);
}

#[test]
fn test_topology_errors() {
    let errors: Vec<TopologyError> = vec![
        TopologyError::SelfIntersection {
            geometry_id: 501,
            at: Coordinate::Wgs84 {
                lat_e7: 400_000_000,
                lon_e7: -740_000_000,
            },
        },
        TopologyError::GapBetweenPolygons {
            poly_a: 10,
            poly_b: 11,
            gap_area_sq_mm: 250,
        },
        TopologyError::UnclosedRing {
            geometry_id: 77,
            start: Coordinate::Ecef {
                x_mm: 1_000,
                y_mm: 2_000,
                z_mm: 3_000,
            },
            end: Coordinate::Ecef {
                x_mm: 1_001,
                y_mm: 2_001,
                z_mm: 3_001,
            },
        },
        TopologyError::InvalidWinding {
            geometry_id: 88,
            expected: WindingOrder::CounterClockwise,
        },
        TopologyError::DuplicateVertex {
            geometry_id: 99,
            coord: Coordinate::Wgs84 {
                lat_e7: 0,
                lon_e7: 0,
            },
            count: 3,
        },
    ];
    let bytes = encode_to_vec(&errors).expect("encode topology errors");
    let (decoded, _) =
        decode_from_slice::<Vec<TopologyError>>(&bytes).expect("decode topology errors");
    assert_eq!(errors, decoded);
}

#[test]
fn test_tile_pyramid_xyz() {
    let pyramid = TilePyramid::Xyz {
        min_zoom: 0,
        max_zoom: 18,
        tile_size: 256,
        format: TileFormat::Webp { lossless: false },
        bounds: BoundingBox {
            min_x_e7: -1_800_000_000,
            min_y_e7: -900_000_000,
            max_x_e7: 1_800_000_000,
            max_y_e7: 900_000_000,
        },
    };
    let bytes = encode_to_vec(&pyramid).expect("encode tile pyramid");
    let (decoded, _) = decode_from_slice::<TilePyramid>(&bytes).expect("decode tile pyramid");
    assert_eq!(pyramid, decoded);
}

#[test]
fn test_lambert_conformal_conic_projection() {
    let proj = MapProjectionParams::LambertConformalConic {
        std_parallel1_e5: 33_00000,
        std_parallel2_e5: 45_00000,
        lat_origin_e5: 39_00000,
        lon_origin_e5: -96_00000,
        false_easting_mm: 0,
        false_northing_mm: 0,
    };
    let bytes = encode_to_vec(&proj).expect("encode lambert projection");
    let (decoded, _) =
        decode_from_slice::<MapProjectionParams>(&bytes).expect("decode lambert projection");
    assert_eq!(proj, decoded);
}
