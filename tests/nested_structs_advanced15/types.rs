// Shared domain types for nested_structs_advanced15 tests
use oxicode::{Decode, Encode};

// ─── Ride configuration with vehicle specs and track segments ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct TrackSegment {
    pub segment_id: u32,
    pub segment_type: String,
    pub length_meters: f64,
    pub incline_degrees: f32,
    pub max_speed_kmh: f32,
    pub has_inversion: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RideVehicle {
    pub vehicle_id: u32,
    pub name: String,
    pub capacity: u16,
    pub weight_kg: f64,
    pub safety_harness_type: String,
    pub last_inspection_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RideConfiguration {
    pub ride_id: u64,
    pub ride_name: String,
    pub category: String,
    pub height_requirement_cm: u16,
    pub vehicles: Vec<RideVehicle>,
    pub track: Vec<TrackSegment>,
    pub total_track_length_meters: f64,
    pub max_g_force: f32,
    pub ride_duration_seconds: u32,
    pub opened_year: u16,
}

// ─── Queue management with virtual queue slots ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct VirtualQueueSlot {
    pub slot_id: u64,
    pub guest_id: u64,
    pub estimated_wait_minutes: u16,
    pub window_start_epoch: u64,
    pub window_end_epoch: u64,
    pub redeemed: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct QueueLane {
    pub lane_id: u32,
    pub lane_type: String,
    pub current_wait_minutes: u16,
    pub capacity: u32,
    pub slots: Vec<VirtualQueueSlot>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct QueueManagement {
    pub ride_id: u64,
    pub ride_name: String,
    pub lanes: Vec<QueueLane>,
    pub total_guests_in_queue: u32,
    pub is_operational: bool,
}

// ─── Show schedule hierarchies ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Performer {
    pub performer_id: u32,
    pub name: String,
    pub role: String,
    pub is_understudy: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Performance {
    pub performance_id: u64,
    pub start_epoch: u64,
    pub duration_minutes: u16,
    pub venue: String,
    pub performers: Vec<Performer>,
    pub sold_out: bool,
    pub attendance: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DaySchedule {
    pub date_str: String,
    pub performances: Vec<Performance>,
    pub park_open_epoch: u64,
    pub park_close_epoch: u64,
    pub weather_forecast: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SeasonSchedule {
    pub season_name: String,
    pub start_date: String,
    pub end_date: String,
    pub days: Vec<DaySchedule>,
    pub total_performances: u32,
}

// ─── Food & beverage outlet menus with allergen data ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Allergen {
    pub code: String,
    pub name: String,
    pub severity: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct NutritionInfo {
    pub calories: u32,
    pub protein_grams: f32,
    pub carbs_grams: f32,
    pub fat_grams: f32,
    pub sodium_mg: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MenuItem {
    pub item_id: u32,
    pub name: String,
    pub description: String,
    pub price_cents: u32,
    pub allergens: Vec<Allergen>,
    pub nutrition: NutritionInfo,
    pub available: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MenuCategory {
    pub category_name: String,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FoodOutlet {
    pub outlet_id: u32,
    pub name: String,
    pub location_zone: String,
    pub categories: Vec<MenuCategory>,
    pub open_epoch: u64,
    pub close_epoch: u64,
}

// ─── Hotel room inventories with rate calendars ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct NightlyRate {
    pub date_str: String,
    pub base_rate_cents: u32,
    pub tax_cents: u32,
    pub resort_fee_cents: u32,
    pub available_rooms: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RoomAmenity {
    pub amenity_name: String,
    pub included: bool,
    pub surcharge_cents: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RoomType {
    pub type_id: u32,
    pub name: String,
    pub max_occupancy: u8,
    pub bed_config: String,
    pub square_meters: f32,
    pub amenities: Vec<RoomAmenity>,
    pub rate_calendar: Vec<NightlyRate>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct HotelInventory {
    pub hotel_id: u32,
    pub hotel_name: String,
    pub star_rating: u8,
    pub room_types: Vec<RoomType>,
    pub total_rooms: u16,
}

// ─── Guest experience scores with ride-by-ride breakdowns ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct RideScore {
    pub ride_id: u64,
    pub ride_name: String,
    pub overall_score: f32,
    pub thrill_score: f32,
    pub queue_experience_score: f32,
    pub cleanliness_score: f32,
    pub staff_friendliness_score: f32,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DiningScore {
    pub outlet_id: u32,
    pub outlet_name: String,
    pub food_quality_score: f32,
    pub service_score: f32,
    pub value_score: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct GuestExperienceReport {
    pub guest_id: u64,
    pub visit_date: String,
    pub overall_satisfaction: f32,
    pub ride_scores: Vec<RideScore>,
    pub dining_scores: Vec<DiningScore>,
    pub would_recommend: bool,
    pub net_promoter_score: i8,
}

// ─── Maintenance work orders with parts lists ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SparePart {
    pub part_number: String,
    pub description: String,
    pub quantity_needed: u16,
    pub unit_cost_cents: u32,
    pub lead_time_days: u16,
    pub in_stock: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MaintenanceTask {
    pub task_id: u32,
    pub description: String,
    pub estimated_hours: f32,
    pub priority: u8,
    pub parts: Vec<SparePart>,
    pub requires_ride_shutdown: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct WorkOrder {
    pub order_id: u64,
    pub ride_id: u64,
    pub ride_name: String,
    pub created_epoch: u64,
    pub due_epoch: u64,
    pub tasks: Vec<MaintenanceTask>,
    pub assigned_technician: String,
    pub status: String,
    pub total_estimated_cost_cents: u64,
}

// ─── Fireworks choreography sequences ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct PyroEffect {
    pub effect_type: String,
    pub color_primary: String,
    pub color_secondary: Option<String>,
    pub altitude_meters: f32,
    pub spread_degrees: f32,
    pub duration_ms: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LaunchCue {
    pub cue_id: u32,
    pub timestamp_ms: u64,
    pub launcher_id: u16,
    pub angle_degrees: f32,
    pub effects: Vec<PyroEffect>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MusicSync {
    pub track_name: String,
    pub bpm: f32,
    pub beat_markers_ms: Vec<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FireworksShow {
    pub show_id: u64,
    pub show_name: String,
    pub total_duration_ms: u64,
    pub cues: Vec<LaunchCue>,
    pub music: MusicSync,
    pub total_shells: u32,
    pub safety_radius_meters: f32,
}

// ─── Character meet-and-greet schedules ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct CharacterCostume {
    pub costume_id: u32,
    pub variant: String,
    pub seasonal: bool,
    pub condition_rating: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MeetGreetSession {
    pub session_id: u64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub max_guests: u16,
    pub photo_pass_available: bool,
    pub autograph_available: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct CharacterSchedule {
    pub character_name: String,
    pub zone: String,
    pub costumes: Vec<CharacterCostume>,
    pub sessions: Vec<MeetGreetSession>,
    pub popularity_rank: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct MeetGreetPlan {
    pub date_str: String,
    pub park_section: String,
    pub characters: Vec<CharacterSchedule>,
    pub total_sessions: u32,
}

// ─── Ticket pricing tiers with date-based dynamic pricing ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DatePricePoint {
    pub date_str: String,
    pub demand_tier: String,
    pub adult_price_cents: u32,
    pub child_price_cents: u32,
    pub senior_price_cents: u32,
    pub capacity_percentage: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct AddOn {
    pub addon_id: u32,
    pub name: String,
    pub price_cents: u32,
    pub description: String,
    pub limited_quantity: Option<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct TicketTier {
    pub tier_id: u32,
    pub tier_name: String,
    pub includes_parks: Vec<String>,
    pub date_prices: Vec<DatePricePoint>,
    pub available_addons: Vec<AddOn>,
    pub max_days: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct PricingPlan {
    pub plan_id: u64,
    pub season: String,
    pub tiers: Vec<TicketTier>,
    pub currency: String,
    pub tax_rate_percent: f32,
}

// ─── Parade float configurations ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FloatLighting {
    pub zone_name: String,
    pub led_count: u32,
    pub color_sequence: Vec<String>,
    pub animation_pattern: String,
    pub power_watts: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct FloatAudio {
    pub speaker_count: u8,
    pub track_name: String,
    pub volume_db: f32,
    pub sync_offset_ms: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ParadeFloat {
    pub float_id: u32,
    pub theme: String,
    pub length_meters: f32,
    pub weight_kg: f64,
    pub performer_count: u8,
    pub lighting: Vec<FloatLighting>,
    pub audio: FloatAudio,
    pub max_speed_kmh: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ParadeConfiguration {
    pub parade_id: u64,
    pub parade_name: String,
    pub route_name: String,
    pub floats: Vec<ParadeFloat>,
    pub total_duration_minutes: u16,
    pub start_epoch: u64,
}

// ─── Water ride splash zone modeling ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SplashZone {
    pub zone_id: u32,
    pub distance_from_ride_meters: f32,
    pub splash_probability: f32,
    pub avg_water_volume_liters: f32,
    pub spectator_area: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct WaterChannel {
    pub channel_id: u32,
    pub width_meters: f32,
    pub depth_meters: f32,
    pub flow_rate_lps: f32,
    pub temperature_celsius: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct DropProfile {
    pub drop_id: u32,
    pub height_meters: f32,
    pub angle_degrees: f32,
    pub entry_speed_kmh: f32,
    pub splash_height_meters: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct WaterRide {
    pub ride_id: u64,
    pub ride_name: String,
    pub channels: Vec<WaterChannel>,
    pub drops: Vec<DropProfile>,
    pub splash_zones: Vec<SplashZone>,
    pub total_water_volume_liters: f64,
    pub recirculation_rate_percent: f32,
}

// ─── Park-wide energy management ───

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct PowerConsumer {
    pub asset_id: u64,
    pub asset_name: String,
    pub asset_type: String,
    pub peak_watts: f64,
    pub avg_watts: f64,
    pub zone: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct EnergyReading {
    pub timestamp_epoch: u64,
    pub kwh_consumed: f64,
    pub peak_demand_kw: f64,
    pub solar_generated_kwh: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct EnergyZone {
    pub zone_name: String,
    pub consumers: Vec<PowerConsumer>,
    pub readings: Vec<EnergyReading>,
    pub transformer_capacity_kva: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ParkEnergyPlan {
    pub plan_id: u64,
    pub date_str: String,
    pub zones: Vec<EnergyZone>,
    pub total_budget_kwh: f64,
    pub renewable_target_percent: f32,
}

// ═══════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════

// Test 1: Ride configuration with track segments and vehicles

// Test 2: Virtual queue management with multiple lanes

// Test 3: Season → day → performance show schedule

// Test 4: Food outlet menu with allergens and nutrition

// Test 5: Hotel room inventory with rate calendar

// Test 6: Guest experience report with ride-by-ride scores

// Test 7: Maintenance work order with parts lists

// Test 8: Fireworks choreography with music sync

// Test 9: Character meet-and-greet daily plan

// Test 10: Dynamic ticket pricing plan

// Test 11: Parade float configuration with lighting and audio

// Test 12: Water ride with splash zones and drop profiles

// Test 13: Park energy management zones

// Test 14: Empty ride configuration (no vehicles, no track)

// Test 15: Queue management with all lanes empty

// Test 16: Multi-outlet food court with allergen-heavy menus

// Test 17: Fireworks show with single cue (minimal)

// Test 18: Hotel inventory with no availability (sold out)

// Test 19: Guest with only dining scores, no rides

// Test 20: Work order with zero-parts inspection-only tasks

// Test 21: Massive character meet-greet plan for holiday event

// Test 22: Ride configuration with extreme track (many inversions, long track)
