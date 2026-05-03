// Tests for nested_structs_advanced17 — part A (all tests)
use super::types::*;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[test]
fn test_recipe_formulation_roundtrip() {
    let recipe = RecipeFormulation {
        recipe_id: 101,
        name: "Country Sourdough".to_string(),
        flour_type: FlourType::BreadFlour,
        total_flour_grams: 10000,
        percentages: vec![
            BakersPercentage {
                ingredient_name: "Bread Flour".to_string(),
                category: IngredientCategory::Flour,
                percentage: 8000,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Whole Wheat".to_string(),
                category: IngredientCategory::Flour,
                percentage: 2000,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Water".to_string(),
                category: IngredientCategory::Liquid,
                percentage: 7500,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Salt".to_string(),
                category: IngredientCategory::Flavoring,
                percentage: 200,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Levain".to_string(),
                category: IngredientCategory::Leavening,
                percentage: 2000,
                is_preferment: true,
            },
        ],
        target_dough_temp_c: 26,
        notes: Some("Autolyse 30 min before salt".to_string()),
    };
    roundtrip(&recipe, "country sourdough recipe");
}

#[test]
fn test_production_batch_roundtrip() {
    let batch = ProductionBatch {
        batch_id: 2001,
        recipe: RecipeFormulation {
            recipe_id: 102,
            name: "Pain de Campagne".to_string(),
            flour_type: FlourType::AllPurpose,
            total_flour_grams: 25000,
            percentages: vec![
                BakersPercentage {
                    ingredient_name: "AP Flour".to_string(),
                    category: IngredientCategory::Flour,
                    percentage: 10000,
                    is_preferment: false,
                },
                BakersPercentage {
                    ingredient_name: "Water".to_string(),
                    category: IngredientCategory::Liquid,
                    percentage: 6800,
                    is_preferment: false,
                },
            ],
            target_dough_temp_c: 24,
            notes: None,
        },
        stages: vec![
            FermentationStage {
                stage_name: "Autolyse".to_string(),
                method: FermentationMethod::Autolyse,
                duration_minutes: 30,
                temperature_c: 24,
                humidity_pct: 75,
                fold_count: 0,
                notes: None,
            },
            FermentationStage {
                stage_name: "Bulk Fermentation".to_string(),
                method: FermentationMethod::BulkAmbient,
                duration_minutes: 240,
                temperature_c: 25,
                humidity_pct: 78,
                fold_count: 4,
                notes: Some("Fold every 45 min".to_string()),
            },
            FermentationStage {
                stage_name: "Cold Retard".to_string(),
                method: FermentationMethod::ColdRetard,
                duration_minutes: 720,
                temperature_c: 4,
                humidity_pct: 85,
                fold_count: 0,
                notes: Some("Overnight in retarder".to_string()),
            },
        ],
        operator_name: "Jean-Pierre".to_string(),
        total_yield_kg: 38,
        date_epoch_secs: 1710000000,
    };
    roundtrip(&batch, "production batch");
}

#[test]
fn test_oven_profile_roundtrip() {
    let profile = OvenProfile {
        profile_id: 301,
        name: "Artisan Hearth Bake".to_string(),
        oven_model: "Bongard Cervap".to_string(),
        phases: vec![
            OvenPhase {
                phase_name: "Initial Steam".to_string(),
                duration_seconds: 90,
                temp_top_c: 260,
                temp_bottom_c: 240,
                steam: SteamMode::High,
                damper_open: false,
                fan_speed_pct: 0,
            },
            OvenPhase {
                phase_name: "Oven Spring".to_string(),
                duration_seconds: 300,
                temp_top_c: 245,
                temp_bottom_c: 235,
                steam: SteamMode::Low,
                damper_open: false,
                fan_speed_pct: 20,
            },
            OvenPhase {
                phase_name: "Crust Development".to_string(),
                duration_seconds: 600,
                temp_top_c: 220,
                temp_bottom_c: 210,
                steam: SteamMode::Off,
                damper_open: true,
                fan_speed_pct: 50,
            },
            OvenPhase {
                phase_name: "Final Dry".to_string(),
                duration_seconds: 180,
                temp_top_c: 200,
                temp_bottom_c: 190,
                steam: SteamMode::Off,
                damper_open: true,
                fan_speed_pct: 70,
            },
        ],
        preheat_minutes: 60,
        total_bake_seconds: 1170,
    };
    roundtrip(&profile, "artisan oven profile");
}

#[test]
fn test_ingredient_inventory_roundtrip() {
    let inventory = IngredientInventory {
        warehouse_id: 1,
        items: vec![
            InventoryItem {
                item_id: 4001,
                ingredient_name: "King Arthur Bread Flour".to_string(),
                category: IngredientCategory::Flour,
                current_stock_grams: 500_000,
                reorder_threshold_grams: 100_000,
                supplier: make_supplier(10, "King Arthur"),
                lot_number: "KA-2024-0387".to_string(),
                expiry_epoch_secs: 1730000000,
            },
            InventoryItem {
                item_id: 4002,
                ingredient_name: "Plugra European Butter".to_string(),
                category: IngredientCategory::Fat,
                current_stock_grams: 50_000,
                reorder_threshold_grams: 20_000,
                supplier: make_supplier(11, "Plugra"),
                lot_number: "PL-2024-1122".to_string(),
                expiry_epoch_secs: 1715000000,
            },
        ],
        last_audit_epoch: 1709500000,
    };
    roundtrip(&inventory, "ingredient inventory");
}

#[test]
fn test_cake_specification_roundtrip() {
    let cake = CakeSpecification {
        order_id: 5001,
        customer_name: "Anderson Wedding".to_string(),
        tiers: vec![
            TierConfig {
                tier_number: 1,
                diameter_cm: 35,
                height_cm: 15,
                flavor: "Vanilla Bean".to_string(),
                icing: IcingType::Fondant,
                decorations: vec![
                    DecorationElement {
                        element_name: "Sugar Rose".to_string(),
                        color: ColorSpec {
                            name: "Blush Pink".to_string(),
                            red: 255,
                            green: 182,
                            blue: 193,
                        },
                        quantity: 12,
                        is_edible: true,
                    },
                    DecorationElement {
                        element_name: "Gold Leaf".to_string(),
                        color: ColorSpec {
                            name: "Gold".to_string(),
                            red: 255,
                            green: 215,
                            blue: 0,
                        },
                        quantity: 8,
                        is_edible: true,
                    },
                ],
                serves: 40,
            },
            TierConfig {
                tier_number: 2,
                diameter_cm: 25,
                height_cm: 12,
                flavor: "Dark Chocolate".to_string(),
                icing: IcingType::Ganache,
                decorations: vec![DecorationElement {
                    element_name: "Chocolate Curl".to_string(),
                    color: ColorSpec {
                        name: "Dark Brown".to_string(),
                        red: 59,
                        green: 31,
                        blue: 12,
                    },
                    quantity: 20,
                    is_edible: true,
                }],
                serves: 25,
            },
            TierConfig {
                tier_number: 3,
                diameter_cm: 18,
                height_cm: 10,
                flavor: "Lemon Elderflower".to_string(),
                icing: IcingType::Buttercream,
                decorations: vec![],
                serves: 15,
            },
        ],
        delivery_date_epoch: 1712000000,
        special_instructions: Some("Topper: gold monogram A&B".to_string()),
        allergen_notes: vec![
            "Contains wheat".to_string(),
            "Contains dairy".to_string(),
            "Contains eggs".to_string(),
        ],
    };
    roundtrip(&cake, "wedding cake specification");
}

#[test]
fn test_scoring_pattern_roundtrip() {
    let pattern = ScoringPattern {
        pattern_name: "Batard Leaf".to_string(),
        cuts: vec![
            ScoreCut {
                cut_index: 0,
                shape: ScoreShape::Leaf,
                depth_mm: 8,
                angle_degrees: 30,
                length_cm: 18,
            },
            ScoreCut {
                cut_index: 1,
                shape: ScoreShape::Leaf,
                depth_mm: 5,
                angle_degrees: 45,
                length_cm: 10,
            },
            ScoreCut {
                cut_index: 2,
                shape: ScoreShape::Leaf,
                depth_mm: 5,
                angle_degrees: 315,
                length_cm: 10,
            },
            ScoreCut {
                cut_index: 3,
                shape: ScoreShape::Leaf,
                depth_mm: 4,
                angle_degrees: 60,
                length_cm: 7,
            },
            ScoreCut {
                cut_index: 4,
                shape: ScoreShape::Leaf,
                depth_mm: 4,
                angle_degrees: 300,
                length_cm: 7,
            },
        ],
        lame_type: "Curved razor blade".to_string(),
        flour_dusting: true,
        stencil: None,
    };
    roundtrip(&pattern, "batard leaf scoring");
}

#[test]
fn test_lamination_schedule_roundtrip() {
    let schedule = LaminationSchedule {
        product_name: "Classic Croissant".to_string(),
        butter_block_grams: 500,
        dough_grams: 1000,
        fold_steps: vec![
            FoldStep {
                step_number: 1,
                fold: FoldType::LetterFold,
                rest_minutes: 30,
                rest_temp_c: 4,
                thickness_mm: 12,
                notes: Some("Enclose butter block".to_string()),
            },
            FoldStep {
                step_number: 2,
                fold: FoldType::SingleFold,
                rest_minutes: 30,
                rest_temp_c: 4,
                thickness_mm: 8,
                notes: None,
            },
            FoldStep {
                step_number: 3,
                fold: FoldType::SingleFold,
                rest_minutes: 30,
                rest_temp_c: 4,
                thickness_mm: 6,
                notes: None,
            },
            FoldStep {
                step_number: 4,
                fold: FoldType::BookFold,
                rest_minutes: 60,
                rest_temp_c: 4,
                thickness_mm: 4,
                notes: Some("Final rest before sheeting".to_string()),
            },
        ],
        total_layers: 81,
        final_proof_minutes: 120,
    };
    roundtrip(&schedule, "croissant lamination");
}

#[test]
fn test_starter_maintenance_roundtrip() {
    let log = StarterMaintenanceLog {
        starter_name: "Old Faithful".to_string(),
        origin_year: 2018,
        feedings: vec![
            FeedingRecord {
                timestamp_epoch: 1709900000,
                flour_grams: 100,
                water_grams: 100,
                flour_type: FlourType::BreadFlour,
                ambient_temp_c: 22,
                rise_factor: 25,
                notes: Some("Strong rise in 4h".to_string()),
            },
            FeedingRecord {
                timestamp_epoch: 1709986400,
                flour_grams: 50,
                water_grams: 50,
                flour_type: FlourType::WholeWheat,
                ambient_temp_c: 21,
                rise_factor: 22,
                notes: None,
            },
            FeedingRecord {
                timestamp_epoch: 1710072800,
                flour_grams: 100,
                water_grams: 100,
                flour_type: FlourType::Rye,
                ambient_temp_c: 23,
                rise_factor: 30,
                notes: Some("Very active after rye feed".to_string()),
            },
        ],
        current_hydration_pct: 100,
        is_active: true,
        backup_location: Some("Walk-in freezer, shelf 3".to_string()),
    };
    roundtrip(&log, "sourdough starter log");
}

#[test]
fn test_allergen_matrix_roundtrip() {
    let matrix = AllergenMatrix {
        facility_name: "Sunrise Bakery - Main Plant".to_string(),
        profiles: vec![
            ProductAllergenProfile {
                product_name: "Sourdough Boule".to_string(),
                entries: vec![
                    AllergenEntry {
                        allergen: Allergen::Wheat,
                        risk: ContaminationRisk::PrimaryIngredient,
                        source_ingredient: "Bread flour".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::Milk,
                        risk: ContaminationRisk::MayContain,
                        source_ingredient: "Shared mixer".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::Soy,
                        risk: ContaminationRisk::LowTrace,
                        source_ingredient: "Soy lecithin in release spray".to_string(),
                    },
                ],
                shared_line: true,
                cleaning_protocol: "Full CIP between allergen changeover".to_string(),
            },
            ProductAllergenProfile {
                product_name: "Almond Croissant".to_string(),
                entries: vec![
                    AllergenEntry {
                        allergen: Allergen::Wheat,
                        risk: ContaminationRisk::PrimaryIngredient,
                        source_ingredient: "Pastry flour".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::Milk,
                        risk: ContaminationRisk::PrimaryIngredient,
                        source_ingredient: "Butter".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::Eggs,
                        risk: ContaminationRisk::Contains,
                        source_ingredient: "Egg wash".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::TreeNuts,
                        risk: ContaminationRisk::PrimaryIngredient,
                        source_ingredient: "Almond frangipane".to_string(),
                    },
                ],
                shared_line: false,
                cleaning_protocol: "Dedicated pastry line".to_string(),
            },
        ],
        last_review_epoch: 1709000000,
    };
    roundtrip(&matrix, "allergen matrix");
}

#[test]
fn test_nutritional_label_roundtrip() {
    let label = NutritionalLabel {
        product_name: "Multigrain Sandwich Loaf".to_string(),
        serving_size_grams: 43,
        servings_per_package: 16,
        macros: MacroNutrients {
            calories_kcal: 120,
            total_fat_mg: 1500,
            saturated_fat_mg: 0,
            trans_fat_mg: 0,
            cholesterol_mg: 0,
            sodium_mg: 200,
            total_carbs_mg: 23000,
            dietary_fiber_mg: 3000,
            total_sugars_mg: 3000,
            added_sugars_mg: 2000,
            protein_mg: 5000,
        },
        micros: vec![
            MicroNutrient {
                name: "Iron".to_string(),
                amount_mcg: 1800,
                daily_value_pct: 10,
            },
            MicroNutrient {
                name: "Thiamin".to_string(),
                amount_mcg: 300,
                daily_value_pct: 25,
            },
            MicroNutrient {
                name: "Folate".to_string(),
                amount_mcg: 100,
                daily_value_pct: 25,
            },
            MicroNutrient {
                name: "Calcium".to_string(),
                amount_mcg: 30000,
                daily_value_pct: 2,
            },
        ],
        ingredients_list: "Enriched wheat flour, water, whole wheat flour, oats, flaxseed, \
            sunflower seeds, honey, yeast, salt, soybean oil"
            .to_string(),
    };
    roundtrip(&label, "nutritional label");
}

#[test]
fn test_wholesale_order_roundtrip() {
    let order = WholesaleOrder {
        order_id: 11001,
        client_name: "Metro Cafe Group".to_string(),
        lines: vec![
            OrderLine {
                product_name: "Sourdough Boule".to_string(),
                quantity: 50,
                unit_price_cents: 450,
                batch_id: 2001,
            },
            OrderLine {
                product_name: "Baguette".to_string(),
                quantity: 100,
                unit_price_cents: 250,
                batch_id: 2002,
            },
            OrderLine {
                product_name: "Croissant".to_string(),
                quantity: 200,
                unit_price_cents: 180,
                batch_id: 2003,
            },
        ],
        delivery_route: DeliveryRoute {
            route_id: 7001,
            driver_name: "Marco".to_string(),
            vehicle_plate: "BKR-4521".to_string(),
            stops: vec![
                DeliveryStop {
                    stop_index: 0,
                    client_name: "Metro Cafe Downtown".to_string(),
                    address: "123 Main St".to_string(),
                    arrival_epoch: 1710040000,
                    cases_count: 8,
                    requires_signature: true,
                },
                DeliveryStop {
                    stop_index: 1,
                    client_name: "Metro Cafe Midtown".to_string(),
                    address: "456 Oak Ave".to_string(),
                    arrival_epoch: 1710042000,
                    cases_count: 6,
                    requires_signature: true,
                },
            ],
            total_distance_km: 45,
            departure_epoch: 1710036000,
        },
        total_cents: 94_500,
        paid: false,
    };
    roundtrip(&order, "wholesale order");
}

#[test]
fn test_qc_report_roundtrip() {
    let report = QcReport {
        report_id: 12001,
        batch_id: 2001,
        product_name: "Sourdough Boule 800g".to_string(),
        checks: vec![
            QcCheck {
                check_name: "Loaf Weight".to_string(),
                result: QcResult::Pass,
                measured_value: Some("812g".to_string()),
                target_range: "790-830g".to_string(),
                inspector: "Yuki".to_string(),
            },
            QcCheck {
                check_name: "Internal Temperature".to_string(),
                result: QcResult::Pass,
                measured_value: Some("98C".to_string()),
                target_range: "96-100C".to_string(),
                inspector: "Yuki".to_string(),
            },
            QcCheck {
                check_name: "Crumb Structure".to_string(),
                result: QcResult::MinorDeviation,
                measured_value: Some("Slightly dense at base".to_string()),
                target_range: "Open, even".to_string(),
                inspector: "Yuki".to_string(),
            },
            QcCheck {
                check_name: "Crust Color".to_string(),
                result: QcResult::Pass,
                measured_value: Some("Deep mahogany".to_string()),
                target_range: "Golden to mahogany".to_string(),
                inspector: "Yuki".to_string(),
            },
            QcCheck {
                check_name: "Ear Formation".to_string(),
                result: QcResult::Pass,
                measured_value: None,
                target_range: "Visible, crisp".to_string(),
                inspector: "Yuki".to_string(),
            },
        ],
        overall: QcResult::Pass,
        timestamp_epoch: 1710050000,
    };
    roundtrip(&report, "qc report");
}

#[test]
fn test_equipment_maintenance_roundtrip() {
    let equipment = Equipment {
        equipment_id: 13001,
        name: "Spiral Mixer #2".to_string(),
        model: "Diosna SP160A".to_string(),
        serial_number: "DIO-2019-SP-04421".to_string(),
        install_date_epoch: 1560000000,
        maintenance_history: vec![
            MaintenanceRecord {
                record_id: 1,
                maintenance_type: MaintenanceType::Scheduled,
                description: "Annual bearing inspection and lubrication".to_string(),
                technician: "Maintenance Co. - Frank".to_string(),
                cost_cents: 35000,
                downtime_minutes: 120,
                date_epoch: 1680000000,
            },
            MaintenanceRecord {
                record_id: 2,
                maintenance_type: MaintenanceType::Emergency,
                description: "Bowl lock mechanism failure - replaced solenoid".to_string(),
                technician: "In-house - Tomas".to_string(),
                cost_cents: 18500,
                downtime_minutes: 240,
                date_epoch: 1695000000,
            },
            MaintenanceRecord {
                record_id: 3,
                maintenance_type: MaintenanceType::Calibration,
                description: "Speed controller calibration after motor service".to_string(),
                technician: "Diosna Service Tech".to_string(),
                cost_cents: 12000,
                downtime_minutes: 90,
                date_epoch: 1705000000,
            },
        ],
    };
    roundtrip(&equipment, "equipment maintenance");
}

#[test]
fn test_daily_schedule_roundtrip() {
    let schedule = DailySchedule {
        date_epoch: 1710028800,
        assignments: vec![
            ShiftAssignment {
                employee_name: "Carlos".to_string(),
                shift: ShiftType::EarlyMorning,
                station: "Mixing Room".to_string(),
                start_epoch: 1710028800,
                end_epoch: 1710057600,
            },
            ShiftAssignment {
                employee_name: "Aiko".to_string(),
                shift: ShiftType::EarlyMorning,
                station: "Oven Deck 1".to_string(),
                start_epoch: 1710028800,
                end_epoch: 1710057600,
            },
            ShiftAssignment {
                employee_name: "Marie".to_string(),
                shift: ShiftType::Morning,
                station: "Pastry Bench".to_string(),
                start_epoch: 1710043200,
                end_epoch: 1710072000,
            },
            ShiftAssignment {
                employee_name: "Samuel".to_string(),
                shift: ShiftType::Afternoon,
                station: "Packaging".to_string(),
                start_epoch: 1710057600,
                end_epoch: 1710086400,
            },
            ShiftAssignment {
                employee_name: "Li Wei".to_string(),
                shift: ShiftType::Night,
                station: "Dough Prep".to_string(),
                start_epoch: 1710086400,
                end_epoch: 1710115200,
            },
        ],
        production_targets: vec![
            "500 sourdough boules".to_string(),
            "300 baguettes".to_string(),
            "200 croissants".to_string(),
            "100 pain au chocolat".to_string(),
        ],
        notes: Some("Holiday weekend - double production".to_string()),
    };
    roundtrip(&schedule, "daily schedule");
}

#[test]
fn test_bakery_facility_roundtrip() {
    let facility = BakeryFacility {
        facility_id: 15001,
        name: "Sunrise Bakery - Production Hub".to_string(),
        address: "789 Industrial Pkwy, Portland OR 97201".to_string(),
        equipment: vec![Equipment {
            equipment_id: 1,
            name: "Deck Oven Alpha".to_string(),
            model: "Miwe Ideal T6/0604".to_string(),
            serial_number: "MIW-2020-T6-00312".to_string(),
            install_date_epoch: 1590000000,
            maintenance_history: vec![MaintenanceRecord {
                record_id: 100,
                maintenance_type: MaintenanceType::Cleaning,
                description: "Deep clean stone decks".to_string(),
                technician: "In-house".to_string(),
                cost_cents: 0,
                downtime_minutes: 180,
                date_epoch: 1709000000,
            }],
        }],
        allergen_matrix: AllergenMatrix {
            facility_name: "Sunrise Bakery".to_string(),
            profiles: vec![ProductAllergenProfile {
                product_name: "GF Brownie".to_string(),
                entries: vec![
                    AllergenEntry {
                        allergen: Allergen::Eggs,
                        risk: ContaminationRisk::PrimaryIngredient,
                        source_ingredient: "Eggs".to_string(),
                    },
                    AllergenEntry {
                        allergen: Allergen::Wheat,
                        risk: ContaminationRisk::MayContain,
                        source_ingredient: "Shared facility".to_string(),
                    },
                ],
                shared_line: true,
                cleaning_protocol: "Allergen wipe-down between runs".to_string(),
            }],
            last_review_epoch: 1709500000,
        },
        active_orders: vec![WholesaleOrder {
            order_id: 9001,
            client_name: "Deli Fresh".to_string(),
            lines: vec![OrderLine {
                product_name: "Rye Loaf".to_string(),
                quantity: 30,
                unit_price_cents: 520,
                batch_id: 3001,
            }],
            delivery_route: DeliveryRoute {
                route_id: 8001,
                driver_name: "Kai".to_string(),
                vehicle_plate: "BKR-9911".to_string(),
                stops: vec![DeliveryStop {
                    stop_index: 0,
                    client_name: "Deli Fresh HQ".to_string(),
                    address: "55 River Rd".to_string(),
                    arrival_epoch: 1710060000,
                    cases_count: 5,
                    requires_signature: false,
                }],
                total_distance_km: 22,
                departure_epoch: 1710055000,
            },
            total_cents: 15600,
            paid: true,
        }],
    };
    roundtrip(&facility, "full bakery facility");
}

#[test]
fn test_rye_poolish_recipe_roundtrip() {
    let recipe = RecipeFormulation {
        recipe_id: 160,
        name: "Scandinavian Rye".to_string(),
        flour_type: FlourType::Rye,
        total_flour_grams: 5000,
        percentages: vec![
            BakersPercentage {
                ingredient_name: "Dark Rye Flour".to_string(),
                category: IngredientCategory::Flour,
                percentage: 6000,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Bread Flour (poolish)".to_string(),
                category: IngredientCategory::Flour,
                percentage: 4000,
                is_preferment: true,
            },
            BakersPercentage {
                ingredient_name: "Water".to_string(),
                category: IngredientCategory::Liquid,
                percentage: 7000,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Poolish Water".to_string(),
                category: IngredientCategory::Liquid,
                percentage: 4000,
                is_preferment: true,
            },
            BakersPercentage {
                ingredient_name: "Caraway Seeds".to_string(),
                category: IngredientCategory::Inclusion,
                percentage: 150,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Salt".to_string(),
                category: IngredientCategory::Flavoring,
                percentage: 220,
                is_preferment: false,
            },
            BakersPercentage {
                ingredient_name: "Molasses".to_string(),
                category: IngredientCategory::Sweetener,
                percentage: 300,
                is_preferment: false,
            },
        ],
        target_dough_temp_c: 27,
        notes: Some("Mix poolish 12h before final dough".to_string()),
    };

    let batch = ProductionBatch {
        batch_id: 16001,
        recipe,
        stages: vec![
            FermentationStage {
                stage_name: "Poolish".to_string(),
                method: FermentationMethod::PoolishOvernight,
                duration_minutes: 720,
                temperature_c: 20,
                humidity_pct: 70,
                fold_count: 0,
                notes: Some("Room temp overnight".to_string()),
            },
            FermentationStage {
                stage_name: "Final Mix & Bulk".to_string(),
                method: FermentationMethod::BulkAmbient,
                duration_minutes: 90,
                temperature_c: 27,
                humidity_pct: 80,
                fold_count: 1,
                notes: None,
            },
        ],
        operator_name: "Sven".to_string(),
        total_yield_kg: 8,
        date_epoch_secs: 1710100000,
    };
    roundtrip(&batch, "rye poolish batch");
}

#[test]
fn test_danish_lamination_roundtrip() {
    let schedule = LaminationSchedule {
        product_name: "Danish Pastry Dough".to_string(),
        butter_block_grams: 750,
        dough_grams: 1500,
        fold_steps: vec![
            FoldStep {
                step_number: 1,
                fold: FoldType::LetterFold,
                rest_minutes: 20,
                rest_temp_c: 2,
                thickness_mm: 15,
                notes: Some("Lock-in fold".to_string()),
            },
            FoldStep {
                step_number: 2,
                fold: FoldType::BookFold,
                rest_minutes: 30,
                rest_temp_c: 2,
                thickness_mm: 10,
                notes: None,
            },
            FoldStep {
                step_number: 3,
                fold: FoldType::BookFold,
                rest_minutes: 30,
                rest_temp_c: 2,
                thickness_mm: 7,
                notes: None,
            },
            FoldStep {
                step_number: 4,
                fold: FoldType::DoubleFold,
                rest_minutes: 45,
                rest_temp_c: 2,
                thickness_mm: 5,
                notes: Some("Ready for shaping after final rest".to_string()),
            },
        ],
        total_layers: 144,
        final_proof_minutes: 90,
    };
    roundtrip(&schedule, "danish lamination");
}

#[test]
fn test_multi_product_nutrition_roundtrip() {
    let labels: Vec<NutritionalLabel> = vec![
        NutritionalLabel {
            product_name: "Whole Wheat Boule".to_string(),
            serving_size_grams: 50,
            servings_per_package: 12,
            macros: MacroNutrients {
                calories_kcal: 130,
                total_fat_mg: 1000,
                saturated_fat_mg: 0,
                trans_fat_mg: 0,
                cholesterol_mg: 0,
                sodium_mg: 280,
                total_carbs_mg: 26000,
                dietary_fiber_mg: 4000,
                total_sugars_mg: 2000,
                added_sugars_mg: 0,
                protein_mg: 5000,
            },
            micros: vec![MicroNutrient {
                name: "Iron".to_string(),
                amount_mcg: 2000,
                daily_value_pct: 11,
            }],
            ingredients_list: "Whole wheat flour, water, salt, sourdough culture".to_string(),
        },
        NutritionalLabel {
            product_name: "Butter Croissant".to_string(),
            serving_size_grams: 60,
            servings_per_package: 1,
            macros: MacroNutrients {
                calories_kcal: 270,
                total_fat_mg: 15000,
                saturated_fat_mg: 9000,
                trans_fat_mg: 500,
                cholesterol_mg: 45,
                sodium_mg: 310,
                total_carbs_mg: 28000,
                dietary_fiber_mg: 1000,
                total_sugars_mg: 5000,
                added_sugars_mg: 3000,
                protein_mg: 5000,
            },
            micros: vec![
                MicroNutrient {
                    name: "Vitamin A".to_string(),
                    amount_mcg: 120,
                    daily_value_pct: 10,
                },
                MicroNutrient {
                    name: "Calcium".to_string(),
                    amount_mcg: 20000,
                    daily_value_pct: 2,
                },
            ],
            ingredients_list: "Enriched flour, butter, sugar, yeast, milk, salt, eggs".to_string(),
        },
    ];
    let bytes = encode_to_vec(&labels).expect("encode nutrition vec");
    let (decoded, consumed): (Vec<NutritionalLabel>, usize) =
        decode_from_slice(&bytes).expect("decode nutrition vec");
    assert_eq!(labels, decoded);
    assert_eq!(decoded.len(), 2);
    assert!(consumed > 0, "consumed zero bytes");
}

#[test]
fn test_scoring_stencil_custom_roundtrip() {
    let pattern = ScoringPattern {
        pattern_name: "Holiday Snowflake Boule".to_string(),
        cuts: vec![
            ScoreCut {
                cut_index: 0,
                shape: ScoreShape::Custom,
                depth_mm: 3,
                angle_degrees: 0,
                length_cm: 20,
            },
            ScoreCut {
                cut_index: 1,
                shape: ScoreShape::Custom,
                depth_mm: 3,
                angle_degrees: 60,
                length_cm: 20,
            },
            ScoreCut {
                cut_index: 2,
                shape: ScoreShape::Custom,
                depth_mm: 3,
                angle_degrees: 120,
                length_cm: 20,
            },
            ScoreCut {
                cut_index: 3,
                shape: ScoreShape::Diamond,
                depth_mm: 2,
                angle_degrees: 30,
                length_cm: 5,
            },
            ScoreCut {
                cut_index: 4,
                shape: ScoreShape::Diamond,
                depth_mm: 2,
                angle_degrees: 90,
                length_cm: 5,
            },
            ScoreCut {
                cut_index: 5,
                shape: ScoreShape::Diamond,
                depth_mm: 2,
                angle_degrees: 150,
                length_cm: 5,
            },
        ],
        lame_type: "Straight blade".to_string(),
        flour_dusting: true,
        stencil: Some("Snowflake template v3".to_string()),
    };
    roundtrip(&pattern, "stencil scoring");
}

#[test]
fn test_inactive_starter_roundtrip() {
    let log = StarterMaintenanceLog {
        starter_name: "Backup Rye Culture".to_string(),
        origin_year: 2021,
        feedings: vec![],
        current_hydration_pct: 60,
        is_active: false,
        backup_location: None,
    };
    roundtrip(&log, "inactive starter");
    let bytes = encode_to_vec(&log).expect("encode inactive starter");
    let (decoded, _): (StarterMaintenanceLog, usize) =
        decode_from_slice(&bytes).expect("decode inactive starter");
    assert!(!decoded.is_active);
    assert!(decoded.feedings.is_empty());
    assert!(decoded.backup_location.is_none());
}

#[test]
fn test_large_delivery_route_roundtrip() {
    let stops: Vec<DeliveryStop> = (0..15)
        .map(|i| DeliveryStop {
            stop_index: i,
            client_name: format!("Client #{}", i + 1),
            address: format!("{} Commerce Blvd, Suite {}", 100 + i * 10, i + 1),
            arrival_epoch: 1710040000 + (i as u64) * 1200,
            cases_count: (3 + i * 2) as u16,
            requires_signature: i % 3 == 0,
        })
        .collect();

    let order = WholesaleOrder {
        order_id: 21001,
        client_name: "Regional Distribution".to_string(),
        lines: vec![
            OrderLine {
                product_name: "Sourdough Boule".to_string(),
                quantity: 200,
                unit_price_cents: 450,
                batch_id: 3010,
            },
            OrderLine {
                product_name: "Baguette".to_string(),
                quantity: 400,
                unit_price_cents: 250,
                batch_id: 3011,
            },
            OrderLine {
                product_name: "Ciabatta".to_string(),
                quantity: 150,
                unit_price_cents: 350,
                batch_id: 3012,
            },
            OrderLine {
                product_name: "Focaccia Sheet".to_string(),
                quantity: 80,
                unit_price_cents: 800,
                batch_id: 3013,
            },
            OrderLine {
                product_name: "Dinner Roll 12pk".to_string(),
                quantity: 100,
                unit_price_cents: 600,
                batch_id: 3014,
            },
        ],
        delivery_route: DeliveryRoute {
            route_id: 21500,
            driver_name: "Elena".to_string(),
            vehicle_plate: "BKR-7788".to_string(),
            stops,
            total_distance_km: 187,
            departure_epoch: 1710036000,
        },
        total_cents: 354_000,
        paid: false,
    };
    roundtrip(&order, "large delivery route order");

    let bytes = encode_to_vec(&order).expect("encode large route");
    let (decoded, _): (WholesaleOrder, usize) =
        decode_from_slice(&bytes).expect("decode large route");
    assert_eq!(decoded.delivery_route.stops.len(), 15);
    assert_eq!(decoded.lines.len(), 5);
}

#[test]
fn test_full_production_pipeline_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ProductionPipeline {
        pipeline_id: u64,
        batch: ProductionBatch,
        oven_profile: OvenProfile,
        scoring: ScoringPattern,
        qc_report: QcReport,
        label: NutritionalLabel,
        order: WholesaleOrder,
    }

    let pipeline = ProductionPipeline {
        pipeline_id: 22000,
        batch: ProductionBatch {
            batch_id: 22001,
            recipe: RecipeFormulation {
                recipe_id: 220,
                name: "Miche".to_string(),
                flour_type: FlourType::WholeWheat,
                total_flour_grams: 15000,
                percentages: vec![
                    BakersPercentage {
                        ingredient_name: "High-extraction wheat".to_string(),
                        category: IngredientCategory::Flour,
                        percentage: 7000,
                        is_preferment: false,
                    },
                    BakersPercentage {
                        ingredient_name: "Whole wheat".to_string(),
                        category: IngredientCategory::Flour,
                        percentage: 3000,
                        is_preferment: false,
                    },
                    BakersPercentage {
                        ingredient_name: "Water".to_string(),
                        category: IngredientCategory::Liquid,
                        percentage: 8000,
                        is_preferment: false,
                    },
                    BakersPercentage {
                        ingredient_name: "Levain".to_string(),
                        category: IngredientCategory::Leavening,
                        percentage: 1500,
                        is_preferment: true,
                    },
                    BakersPercentage {
                        ingredient_name: "Salt".to_string(),
                        category: IngredientCategory::Flavoring,
                        percentage: 220,
                        is_preferment: false,
                    },
                ],
                target_dough_temp_c: 25,
                notes: Some("Long cold ferment for flavor development".to_string()),
            },
            stages: vec![
                FermentationStage {
                    stage_name: "Autolyse".to_string(),
                    method: FermentationMethod::Autolyse,
                    duration_minutes: 60,
                    temperature_c: 24,
                    humidity_pct: 75,
                    fold_count: 0,
                    notes: None,
                },
                FermentationStage {
                    stage_name: "Bulk".to_string(),
                    method: FermentationMethod::SourdoughLevain,
                    duration_minutes: 300,
                    temperature_c: 26,
                    humidity_pct: 78,
                    fold_count: 3,
                    notes: Some("Stretch and fold at 50, 100, 150 min".to_string()),
                },
                FermentationStage {
                    stage_name: "Cold Retard".to_string(),
                    method: FermentationMethod::ColdRetard,
                    duration_minutes: 960,
                    temperature_c: 3,
                    humidity_pct: 85,
                    fold_count: 0,
                    notes: Some("16h retard".to_string()),
                },
            ],
            operator_name: "Nadia".to_string(),
            total_yield_kg: 24,
            date_epoch_secs: 1710200000,
        },
        oven_profile: make_oven_profile(220, "Miche High Hydration"),
        scoring: ScoringPattern {
            pattern_name: "Cross Hatch".to_string(),
            cuts: vec![
                ScoreCut {
                    cut_index: 0,
                    shape: ScoreShape::Cross,
                    depth_mm: 10,
                    angle_degrees: 0,
                    length_cm: 25,
                },
                ScoreCut {
                    cut_index: 1,
                    shape: ScoreShape::Cross,
                    depth_mm: 10,
                    angle_degrees: 90,
                    length_cm: 25,
                },
            ],
            lame_type: "Straight blade".to_string(),
            flour_dusting: true,
            stencil: None,
        },
        qc_report: QcReport {
            report_id: 22010,
            batch_id: 22001,
            product_name: "Miche 1.5kg".to_string(),
            checks: vec![
                QcCheck {
                    check_name: "Weight".to_string(),
                    result: QcResult::Pass,
                    measured_value: Some("1520g".to_string()),
                    target_range: "1450-1550g".to_string(),
                    inspector: "Priya".to_string(),
                },
                QcCheck {
                    check_name: "Crust Thickness".to_string(),
                    result: QcResult::Pass,
                    measured_value: Some("3mm".to_string()),
                    target_range: "2-4mm".to_string(),
                    inspector: "Priya".to_string(),
                },
            ],
            overall: QcResult::Pass,
            timestamp_epoch: 1710210000,
        },
        label: NutritionalLabel {
            product_name: "Miche 1.5kg".to_string(),
            serving_size_grams: 60,
            servings_per_package: 25,
            macros: MacroNutrients {
                calories_kcal: 150,
                total_fat_mg: 500,
                saturated_fat_mg: 0,
                trans_fat_mg: 0,
                cholesterol_mg: 0,
                sodium_mg: 350,
                total_carbs_mg: 31000,
                dietary_fiber_mg: 5000,
                total_sugars_mg: 1000,
                added_sugars_mg: 0,
                protein_mg: 6000,
            },
            micros: vec![
                MicroNutrient {
                    name: "Iron".to_string(),
                    amount_mcg: 2500,
                    daily_value_pct: 14,
                },
                MicroNutrient {
                    name: "Magnesium".to_string(),
                    amount_mcg: 50000,
                    daily_value_pct: 12,
                },
            ],
            ingredients_list: "Wheat flour, water, sourdough culture, salt".to_string(),
        },
        order: WholesaleOrder {
            order_id: 22100,
            client_name: "Artisan Table Restaurant".to_string(),
            lines: vec![OrderLine {
                product_name: "Miche 1.5kg".to_string(),
                quantity: 10,
                unit_price_cents: 1200,
                batch_id: 22001,
            }],
            delivery_route: DeliveryRoute {
                route_id: 22200,
                driver_name: "Dmitri".to_string(),
                vehicle_plate: "BKR-3344".to_string(),
                stops: vec![DeliveryStop {
                    stop_index: 0,
                    client_name: "Artisan Table".to_string(),
                    address: "22 Chef's Row".to_string(),
                    arrival_epoch: 1710220000,
                    cases_count: 2,
                    requires_signature: true,
                }],
                total_distance_km: 12,
                departure_epoch: 1710218000,
            },
            total_cents: 12000,
            paid: false,
        },
    };

    let bytes = encode_to_vec(&pipeline).expect("encode full pipeline");
    assert!(
        bytes.len() > 200,
        "pipeline should produce substantial output"
    );
    let (decoded, consumed): (ProductionPipeline, usize) =
        decode_from_slice(&bytes).expect("decode full pipeline");
    assert_eq!(pipeline, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.batch.stages.len(), 3);
    assert_eq!(decoded.qc_report.checks.len(), 2);
    assert_eq!(decoded.order.delivery_route.stops.len(), 1);
    assert_eq!(decoded.batch.recipe.percentages.len(), 5);
}
