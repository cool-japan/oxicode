// Shared domain types for nested_structs_advanced17 tests
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — Ingredients & Baker's Percentages
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum FlourType {
    BreadFlour,
    AllPurpose,
    WholeWheat,
    Rye,
    Spelt,
    Semolina,
    Einkorn,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum IngredientCategory {
    Flour,
    Liquid,
    Fat,
    Sweetener,
    Leavening,
    Dairy,
    Egg,
    Flavoring,
    Inclusion,
    Stabilizer,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct BakersPercentage {
    pub ingredient_name: String,
    pub category: IngredientCategory,
    pub percentage: u32, // basis-points (12.5% => 1250)
    pub is_preferment: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RecipeFormulation {
    pub recipe_id: u64,
    pub name: String,
    pub flour_type: FlourType,
    pub total_flour_grams: u32,
    pub percentages: Vec<BakersPercentage>,
    pub target_dough_temp_c: u16,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Fermentation & Production
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum FermentationMethod {
    BulkAmbient,
    ColdRetard,
    PoolishOvernight,
    BigaMethod,
    SourdoughLevain,
    Autolyse,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FermentationStage {
    pub stage_name: String,
    pub method: FermentationMethod,
    pub duration_minutes: u32,
    pub temperature_c: i16,
    pub humidity_pct: u8,
    pub fold_count: u8,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ProductionBatch {
    pub batch_id: u64,
    pub recipe: RecipeFormulation,
    pub stages: Vec<FermentationStage>,
    pub operator_name: String,
    pub total_yield_kg: u32,
    pub date_epoch_secs: u64,
}

// ---------------------------------------------------------------------------
// Domain types — Oven Profiles
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum SteamMode {
    Off,
    Low,
    Medium,
    High,
    Burst,
    Pulsed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OvenPhase {
    pub phase_name: String,
    pub duration_seconds: u32,
    pub temp_top_c: u16,
    pub temp_bottom_c: u16,
    pub steam: SteamMode,
    pub damper_open: bool,
    pub fan_speed_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OvenProfile {
    pub profile_id: u64,
    pub name: String,
    pub oven_model: String,
    pub phases: Vec<OvenPhase>,
    pub preheat_minutes: u16,
    pub total_bake_seconds: u32,
}

// ---------------------------------------------------------------------------
// Domain types — Ingredient Inventory & Suppliers
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SupplierContact {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub account_number: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SupplierDetails {
    pub supplier_id: u64,
    pub company_name: String,
    pub contact: SupplierContact,
    pub lead_time_days: u16,
    pub minimum_order_kg: u32,
    pub certifications: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct InventoryItem {
    pub item_id: u64,
    pub ingredient_name: String,
    pub category: IngredientCategory,
    pub current_stock_grams: u64,
    pub reorder_threshold_grams: u64,
    pub supplier: SupplierDetails,
    pub lot_number: String,
    pub expiry_epoch_secs: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct IngredientInventory {
    pub warehouse_id: u32,
    pub items: Vec<InventoryItem>,
    pub last_audit_epoch: u64,
}

// ---------------------------------------------------------------------------
// Domain types — Cake Decorations
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum IcingType {
    Buttercream,
    Fondant,
    RoyalIcing,
    Ganache,
    CreamCheese,
    MirrorGlaze,
    Naked,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ColorSpec {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DecorationElement {
    pub element_name: String,
    pub color: ColorSpec,
    pub quantity: u16,
    pub is_edible: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct TierConfig {
    pub tier_number: u8,
    pub diameter_cm: u16,
    pub height_cm: u16,
    pub flavor: String,
    pub icing: IcingType,
    pub decorations: Vec<DecorationElement>,
    pub serves: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct CakeSpecification {
    pub order_id: u64,
    pub customer_name: String,
    pub tiers: Vec<TierConfig>,
    pub delivery_date_epoch: u64,
    pub special_instructions: Option<String>,
    pub allergen_notes: Vec<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Bread Scoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum ScoreShape {
    SingleSlash,
    Cross,
    Leaf,
    Wheat,
    Spiral,
    Diamond,
    Custom,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ScoreCut {
    pub cut_index: u8,
    pub shape: ScoreShape,
    pub depth_mm: u8,
    pub angle_degrees: u16,
    pub length_cm: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ScoringPattern {
    pub pattern_name: String,
    pub cuts: Vec<ScoreCut>,
    pub lame_type: String,
    pub flour_dusting: bool,
    pub stencil: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Laminated Dough
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum FoldType {
    SingleFold,
    DoubleFold,
    BookFold,
    LetterFold,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FoldStep {
    pub step_number: u8,
    pub fold: FoldType,
    pub rest_minutes: u16,
    pub rest_temp_c: i16,
    pub thickness_mm: u16,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LaminationSchedule {
    pub product_name: String,
    pub butter_block_grams: u32,
    pub dough_grams: u32,
    pub fold_steps: Vec<FoldStep>,
    pub total_layers: u32,
    pub final_proof_minutes: u16,
}

// ---------------------------------------------------------------------------
// Domain types — Sourdough Starter
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FeedingRecord {
    pub timestamp_epoch: u64,
    pub flour_grams: u32,
    pub water_grams: u32,
    pub flour_type: FlourType,
    pub ambient_temp_c: i16,
    pub rise_factor: u8, // 1x=10, 2x=20, etc.
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct StarterMaintenanceLog {
    pub starter_name: String,
    pub origin_year: u16,
    pub feedings: Vec<FeedingRecord>,
    pub current_hydration_pct: u16,
    pub is_active: bool,
    pub backup_location: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Allergen Matrix
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum Allergen {
    Wheat,
    Milk,
    Eggs,
    TreeNuts,
    Peanuts,
    Soy,
    Sesame,
    Fish,
    Shellfish,
    Sulfites,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum ContaminationRisk {
    None,
    LowTrace,
    MayContain,
    Contains,
    PrimaryIngredient,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct AllergenEntry {
    pub allergen: Allergen,
    pub risk: ContaminationRisk,
    pub source_ingredient: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ProductAllergenProfile {
    pub product_name: String,
    pub entries: Vec<AllergenEntry>,
    pub shared_line: bool,
    pub cleaning_protocol: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct AllergenMatrix {
    pub facility_name: String,
    pub profiles: Vec<ProductAllergenProfile>,
    pub last_review_epoch: u64,
}

// ---------------------------------------------------------------------------
// Domain types — Nutritional Labels
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MacroNutrients {
    pub calories_kcal: u32,
    pub total_fat_mg: u32,
    pub saturated_fat_mg: u32,
    pub trans_fat_mg: u32,
    pub cholesterol_mg: u32,
    pub sodium_mg: u32,
    pub total_carbs_mg: u32,
    pub dietary_fiber_mg: u32,
    pub total_sugars_mg: u32,
    pub added_sugars_mg: u32,
    pub protein_mg: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MicroNutrient {
    pub name: String,
    pub amount_mcg: u32,
    pub daily_value_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct NutritionalLabel {
    pub product_name: String,
    pub serving_size_grams: u32,
    pub servings_per_package: u16,
    pub macros: MacroNutrients,
    pub micros: Vec<MicroNutrient>,
    pub ingredients_list: String,
}

// ---------------------------------------------------------------------------
// Domain types — Wholesale Orders & Delivery
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DeliveryStop {
    pub stop_index: u8,
    pub client_name: String,
    pub address: String,
    pub arrival_epoch: u64,
    pub cases_count: u16,
    pub requires_signature: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DeliveryRoute {
    pub route_id: u64,
    pub driver_name: String,
    pub vehicle_plate: String,
    pub stops: Vec<DeliveryStop>,
    pub total_distance_km: u32,
    pub departure_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OrderLine {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
    pub batch_id: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct WholesaleOrder {
    pub order_id: u64,
    pub client_name: String,
    pub lines: Vec<OrderLine>,
    pub delivery_route: DeliveryRoute,
    pub total_cents: u64,
    pub paid: bool,
}

// ---------------------------------------------------------------------------
// Domain types — Quality Control & Equipment
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum QcResult {
    Pass,
    MinorDeviation,
    MajorDeviation,
    Fail,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct QcCheck {
    pub check_name: String,
    pub result: QcResult,
    pub measured_value: Option<String>,
    pub target_range: String,
    pub inspector: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct QcReport {
    pub report_id: u64,
    pub batch_id: u64,
    pub product_name: String,
    pub checks: Vec<QcCheck>,
    pub overall: QcResult,
    pub timestamp_epoch: u64,
}

// ---------------------------------------------------------------------------
// Domain types — Equipment Maintenance
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum MaintenanceType {
    Scheduled,
    Emergency,
    Calibration,
    Cleaning,
    PartReplacement,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MaintenanceRecord {
    pub record_id: u64,
    pub maintenance_type: MaintenanceType,
    pub description: String,
    pub technician: String,
    pub cost_cents: u64,
    pub downtime_minutes: u32,
    pub date_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Equipment {
    pub equipment_id: u64,
    pub name: String,
    pub model: String,
    pub serial_number: String,
    pub install_date_epoch: u64,
    pub maintenance_history: Vec<MaintenanceRecord>,
}

// ---------------------------------------------------------------------------
// Domain types — Staff Scheduling
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum ShiftType {
    EarlyMorning,
    Morning,
    Afternoon,
    Night,
    Split,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ShiftAssignment {
    pub employee_name: String,
    pub shift: ShiftType,
    pub station: String,
    pub start_epoch: u64,
    pub end_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DailySchedule {
    pub date_epoch: u64,
    pub assignments: Vec<ShiftAssignment>,
    pub production_targets: Vec<String>,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain types — Composite facility
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct BakeryFacility {
    pub facility_id: u64,
    pub name: String,
    pub address: String,
    pub equipment: Vec<Equipment>,
    pub allergen_matrix: AllergenMatrix,
    pub active_orders: Vec<WholesaleOrder>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T, ctx: &str) {
    let bytes = encode_to_vec(val).unwrap_or_else(|_| panic!("encode {}", ctx));
    let (decoded, consumed): (T, usize) =
        decode_from_slice(&bytes).unwrap_or_else(|_| panic!("decode {}", ctx));
    assert_eq!(val, &decoded, "roundtrip mismatch for {}", ctx);
    assert!(consumed > 0, "consumed zero bytes for {}", ctx);
}

pub fn make_supplier(id: u64, name: &str) -> SupplierDetails {
    SupplierDetails {
        supplier_id: id,
        company_name: name.to_string(),
        contact: SupplierContact {
            name: format!("{} Rep", name),
            phone: "+1-555-0100".to_string(),
            email: format!("orders@{}.com", name.to_lowercase().replace(' ', "")),
            account_number: format!("ACCT-{:04}", id),
        },
        lead_time_days: 3,
        minimum_order_kg: 25,
        certifications: vec!["Organic".to_string(), "FSSC 22000".to_string()],
    }
}

pub fn make_oven_profile(id: u64, name: &str) -> OvenProfile {
    OvenProfile {
        profile_id: id,
        name: name.to_string(),
        oven_model: "Miwe Ideal T".to_string(),
        phases: vec![
            OvenPhase {
                phase_name: "Steam Blast".to_string(),
                duration_seconds: 120,
                temp_top_c: 250,
                temp_bottom_c: 230,
                steam: SteamMode::Burst,
                damper_open: false,
                fan_speed_pct: 0,
            },
            OvenPhase {
                phase_name: "Bake".to_string(),
                duration_seconds: 1200,
                temp_top_c: 220,
                temp_bottom_c: 210,
                steam: SteamMode::Off,
                damper_open: true,
                fan_speed_pct: 40,
            },
        ],
        preheat_minutes: 45,
        total_bake_seconds: 1320,
    }
}

// ===========================================================================
// Test  1: Recipe formulation with baker's percentages
// ===========================================================================

// ===========================================================================
// Test  2: Production batch with fermentation stages
// ===========================================================================

// ===========================================================================
// Test  3: Oven profile with multi-phase baking
// ===========================================================================

// ===========================================================================
// Test  4: Ingredient inventory with supplier details
// ===========================================================================

// ===========================================================================
// Test  5: Tiered cake specification with decorations
// ===========================================================================

// ===========================================================================
// Test  6: Bread scoring patterns
// ===========================================================================

// ===========================================================================
// Test  7: Laminated dough fold schedule (croissant)
// ===========================================================================

// ===========================================================================
// Test  8: Sourdough starter maintenance log
// ===========================================================================

// ===========================================================================
// Test  9: Allergen cross-contamination matrix
// ===========================================================================

// ===========================================================================
// Test 10: Nutritional label calculations
// ===========================================================================

// ===========================================================================
// Test 11: Wholesale order with delivery route
// ===========================================================================

// ===========================================================================
// Test 12: Quality control report with nested checks
// ===========================================================================

// ===========================================================================
// Test 13: Equipment maintenance history
// ===========================================================================

// ===========================================================================
// Test 14: Staff daily schedule with shift assignments
// ===========================================================================

// ===========================================================================
// Test 15: Full bakery facility composite
// ===========================================================================

// ===========================================================================
// Test 16: Rye bread recipe with poolish preferment
// ===========================================================================

// ===========================================================================
// Test 17: Danish pastry lamination (book folds)
// ===========================================================================

// ===========================================================================
// Test 18: Multi-product nutritional comparison
// ===========================================================================

// ===========================================================================
// Test 19: Scoring pattern with custom stencil
// ===========================================================================

// ===========================================================================
// Test 20: Inactive sourdough starter with empty feedings
// ===========================================================================

// ===========================================================================
// Test 21: Large delivery route with many stops
// ===========================================================================

// ===========================================================================
// Test 22: Full production pipeline (recipe -> batch -> oven -> QC -> delivery)
// ===========================================================================
