// Tests for nested_structs_advanced13 — part B (tests 12-22)
use super::types::*;
use oxicode::{decode_from_slice, encode_to_vec};

#[test]
fn test_feline_multi_disease_monitoring() {
    let profile = FelineMultiDiseaseProfile {
        patient_id: 200200,
        name: "Mochi".into(),
        thyroid: ThyroidMonitoring {
            t4_values: vec![(20100, 680), (20200, 420), (20300, 380)],
            medication: Some("Methimazole".into()),
            current_dose_mg_x100: Some(250),
        },
        renal: RenalMonitoring {
            sdma_values: vec![(20100, 18), (20200, 22), (20300, 25)],
            iris_stage: 2,
            on_renal_diet: true,
            sub_q_fluids: true,
            fluid_volume_ml: Some(150),
            fluid_frequency: Some(Frequency::EveryOtherDay),
        },
        blood_pressure_history: vec![
            BloodPressureReading {
                date_epoch: 20200,
                systolic: 165,
                diastolic: 95,
                method: "Doppler".into(),
                limb_used: "Right forelimb".into(),
                readings_averaged: 5,
            },
            BloodPressureReading {
                date_epoch: 20300,
                systolic: 148,
                diastolic: 88,
                method: "Doppler".into(),
                limb_used: "Right forelimb".into(),
                readings_averaged: 5,
            },
        ],
        current_medications: vec![
            "Methimazole 2.5mg PO BID".into(),
            "Amlodipine 0.625mg PO SID".into(),
            "Aluminum hydroxide with meals".into(),
        ],
    };

    let encoded = encode_to_vec(&profile).expect("encode feline profile");
    let (decoded, _): (FelineMultiDiseaseProfile, _) =
        decode_from_slice(&encoded).expect("decode feline profile");
    assert_eq!(profile, decoded);
}

#[test]
fn test_equine_lameness_evaluation() {
    let exam = LamenessExam {
        patient_id: 300100,
        date_epoch: 20310,
        examiner: "Dr. Hashimoto (DACVS)".into(),
        primary_complaint: "Intermittent right forelimb lameness at trot".into(),
        grade_aaep: 2,
        affected_limb: Laterality::Right,
        gait_observations: vec![
            "Head bob positive at trot on hard surface".into(),
            "No obvious lameness at walk".into(),
            "Worsens on right circle".into(),
        ],
        flexion_tests: vec![
            FlexionTest {
                joint: "Distal interphalangeal".into(),
                duration_seconds: 60,
                lameness_before: 2,
                lameness_after: 3,
                limb: Laterality::Right,
            },
            FlexionTest {
                joint: "Metacarpophalangeal".into(),
                duration_seconds: 60,
                lameness_before: 2,
                lameness_after: 2,
                limb: Laterality::Right,
            },
        ],
        nerve_blocks: vec![NerveBlock {
            block_name: "Palmar digital nerve block".into(),
            agent: "Mepivacaine 2%".into(),
            volume_ml_x10: 30,
            response: "80% improvement at trot".into(),
            improved_percent: 80,
        }],
        imaging_recommended: vec![ImagingModality::Radiograph, ImagingModality::MRI],
        diagnosis: Some("Navicular syndrome, right forelimb".into()),
        treatment_plan: vec![
            "Corrective shoeing with egg bar shoes".into(),
            "Isoxsuprine 0.6 mg/kg PO BID x 60 days".into(),
            "Follow-up lameness exam in 60 days".into(),
        ],
    };

    let encoded = encode_to_vec(&exam).expect("encode lameness exam");
    let (decoded, _): (LamenessExam, _) =
        decode_from_slice(&encoded).expect("decode lameness exam");
    assert_eq!(exam, decoded);
}

#[test]
fn test_nutrition_plan_with_analysis() {
    let plan = NutritionPlan {
        patient_id: 100234,
        target_weight_grams: 29000,
        daily_calorie_target: 1100,
        rer_calories: 786,
        activity_factor_x100: 140,
        diet_components: vec![
            DietComponent {
                food_name: "Royal Canin Gastrointestinal Low Fat".into(),
                manufacturer: "Royal Canin".into(),
                daily_amount: "3 cups divided into 2 meals".into(),
                calories_per_serving: 266,
                nutrients: vec![
                    NutrientContent {
                        category: NutrientCategory::Protein,
                        name: "Crude Protein".into(),
                        amount_per_kg_x100: 2200,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                    NutrientContent {
                        category: NutrientCategory::Fat,
                        name: "Crude Fat".into(),
                        amount_per_kg_x100: 700,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                    NutrientContent {
                        category: NutrientCategory::Fiber,
                        name: "Crude Fiber".into(),
                        amount_per_kg_x100: 500,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                ],
            },
            DietComponent {
                food_name: "Green beans (canned, no salt)".into(),
                manufacturer: "Generic".into(),
                daily_amount: "0.5 cup per meal".into(),
                calories_per_serving: 18,
                nutrients: vec![NutrientContent {
                    category: NutrientCategory::Fiber,
                    name: "Dietary Fiber".into(),
                    amount_per_kg_x100: 2700,
                    unit: "g/kg".into(),
                    meets_aafco: true,
                }],
            },
        ],
        supplements: vec![
            "Omega-3 fish oil 1000mg EPA+DHA daily".into(),
            "Glucosamine/Chondroitin joint supplement".into(),
        ],
        feeding_guidelines: vec![
            "Feed measured amounts only".into(),
            "No table scraps".into(),
            "Weigh weekly and record".into(),
        ],
        review_date_epoch: 20400,
    };

    let encoded = encode_to_vec(&plan).expect("encode nutrition plan");
    let (decoded, _): (NutritionPlan, _) =
        decode_from_slice(&encoded).expect("decode nutrition plan");
    assert_eq!(plan, decoded);
}

#[test]
fn test_exotic_pet_reptile_husbandry() {
    let record = ExoticPetRecord {
        patient_id: 400100,
        common_name: "Ball Python".into(),
        scientific_name: "Python regius".into(),
        species: Species::Reptile,
        sex: Sex::Female,
        length_cm: Some(130),
        weight_grams: 1800,
        enclosure: EnclosureParams {
            length_cm: 120,
            width_cm: 60,
            height_cm: 45,
            substrate: "Coconut fiber".into(),
            basking_temp_c_x10: 330,
            cool_side_temp_c_x10: 265,
            humidity_percent: 60,
            uv_index: 2,
            light_cycle_hours: 12,
        },
        diet_items: vec!["Medium rat every 14 days".into(), "Occasional chick".into()],
        supplement_schedule: vec!["Calcium dusting every other feeding".into()],
        shedding_history: vec![
            SheddingRecord {
                date_epoch: 20200,
                complete: true,
                issues: None,
            },
            SheddingRecord {
                date_epoch: 20260,
                complete: false,
                issues: Some("Retained eye caps - humidity was low".into()),
            },
            SheddingRecord {
                date_epoch: 20320,
                complete: true,
                issues: None,
            },
        ],
        parasite_history: vec!["Fecal: Snake mites treated with fipronil spray 2025-01".into()],
        husbandry_notes: vec![
            "Increase humidity to 70% during shed cycles".into(),
            "Provide two hides minimum".into(),
        ],
    };

    let encoded = encode_to_vec(&record).expect("encode exotic pet record");
    let (decoded, _): (ExoticPetRecord, _) =
        decode_from_slice(&encoded).expect("decode exotic pet record");
    assert_eq!(record, decoded);
}

#[test]
fn test_emergency_triage_timeline() {
    let case = EmergencyCase {
        case_id: 7001,
        patient_id: 100500,
        triage: TriageAssessment {
            arrival_epoch: 20315,
            presenting_complaint: "Ingested dark chocolate 2 hours ago".into(),
            triage_color: "Red - Immediate".into(),
            temp_c_x10: 395,
            heart_rate: 160,
            resp_rate: 36,
            mucous_membrane_color: "Pink, tacky".into(),
            crt_seconds_x10: 15,
            pain_score: PainScore::Mild,
            mentation: "Hyperexcitable, tremoring".into(),
        },
        working_diagnoses: vec!["Theobromine toxicosis".into(), "Caffeine toxicosis".into()],
        treatment_timeline: vec![
            TreatmentAction {
                time_offset_minutes: 0,
                action: "IV catheter placed".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("20G cephalic vein, left forelimb".into()),
                outcome: Some("Successful first attempt".into()),
            },
            TreatmentAction {
                time_offset_minutes: 5,
                action: "Apomorphine administered".into(),
                performed_by: "Dr. Honda".into(),
                details: Some("0.03 mg/kg IV".into()),
                outcome: Some("Productive emesis, chocolate material recovered".into()),
            },
            TreatmentAction {
                time_offset_minutes: 15,
                action: "Activated charcoal administered".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("2 g/kg PO via syringe".into()),
                outcome: Some("Patient accepted without aspiration".into()),
            },
            TreatmentAction {
                time_offset_minutes: 20,
                action: "IV fluid therapy initiated".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("LRS at 2x maintenance rate".into()),
                outcome: None,
            },
        ],
        diagnostics_ordered: vec![
            "Stat CBC/Chemistry".into(),
            "ECG monitoring".into(),
            "Blood pressure monitoring q30min".into(),
        ],
        disposition: "Hospitalized for 24-hour observation and IV fluids".into(),
        estimated_cost_yen: Some(180000),
    };

    let encoded = encode_to_vec(&case).expect("encode emergency case");
    let (decoded, _): (EmergencyCase, _) =
        decode_from_slice(&encoded).expect("decode emergency case");
    assert_eq!(case, decoded);
}

#[test]
fn test_ultrasound_abdominal_study() {
    let study = AbdominalUltrasound {
        study_id: "US-2025-4421".into(),
        patient_id: 200200,
        date_epoch: 20310,
        sonographer: "Dr. Morita (DACVR)".into(),
        interpreter: "Dr. Morita (DACVR)".into(),
        prep_notes: Some("12-hour fast. Patient mildly sedated with butorphanol.".into()),
        organ_findings: vec![
            UltrasoundOrganFinding {
                organ: "Liver".into(),
                echogenicity: "Mildly hyperechoic compared to falciform fat".into(),
                architecture: "Diffusely homogeneous".into(),
                measurements: vec![OrganMeasurement {
                    organ: "Liver".into(),
                    dimension: "Hepatic length".into(),
                    value_mm_x10: 520,
                    normal_range_low: 300,
                    normal_range_high: 450,
                }],
                abnormalities: vec![
                    "Hepatomegaly".into(),
                    "Diffuse hyperechogenicity suggestive of hepatic lipidosis".into(),
                ],
                doppler_findings: Some("Normal hepatic and portal venous flow".into()),
            },
            UltrasoundOrganFinding {
                organ: "Left kidney".into(),
                echogenicity: "Cortex hyperechoic".into(),
                architecture: "Preserved corticomedullary distinction".into(),
                measurements: vec![
                    OrganMeasurement {
                        organ: "Left kidney".into(),
                        dimension: "Length".into(),
                        value_mm_x10: 380,
                        normal_range_low: 300,
                        normal_range_high: 440,
                    },
                    OrganMeasurement {
                        organ: "Left kidney".into(),
                        dimension: "Cortical thickness".into(),
                        value_mm_x10: 45,
                        normal_range_low: 30,
                        normal_range_high: 60,
                    },
                ],
                abnormalities: vec![],
                doppler_findings: Some("Normal renal arterial flow, RI 0.62".into()),
            },
        ],
        free_fluid: false,
        fluid_description: None,
        impression: "Feline hepatic lipidosis. Kidneys within normal limits.".into(),
        recommendations: vec![
            "Hepatic fine needle aspirate for cytology".into(),
            "Initiate aggressive nutritional support".into(),
            "Recheck ultrasound in 4 weeks".into(),
        ],
    };

    let encoded = encode_to_vec(&study).expect("encode ultrasound study");
    let (decoded, _): (AbdominalUltrasound, _) =
        decode_from_slice(&encoded).expect("decode ultrasound study");
    assert_eq!(study, decoded);
}

#[test]
fn test_avian_patient_breeding_records() {
    let patient = AvianPatient {
        patient_id: 500100,
        common_name: "African Grey Parrot".into(),
        scientific_name: "Psittacus erithacus".into(),
        band_number: Some("AGP-2020-0451".into()),
        sex: Sex::Female,
        weight_grams: 420,
        wing_chord_mm: Some(245),
        clutch_history: vec![
            ClutchRecord {
                clutch_number: 1,
                start_date_epoch: 20050,
                nest_box_id: "NB-A12".into(),
                eggs: vec![
                    EggRecord {
                        laid_date_epoch: 20050,
                        fertile: Some(true),
                        hatched: true,
                        hatch_date_epoch: Some(20078),
                        chick_band_id: Some("AGP-2025-0102".into()),
                        weight_grams_x10: Some(145),
                    },
                    EggRecord {
                        laid_date_epoch: 20053,
                        fertile: Some(true),
                        hatched: true,
                        hatch_date_epoch: Some(20081),
                        chick_band_id: Some("AGP-2025-0103".into()),
                        weight_grams_x10: Some(138),
                    },
                    EggRecord {
                        laid_date_epoch: 20056,
                        fertile: Some(false),
                        hatched: false,
                        hatch_date_epoch: None,
                        chick_band_id: None,
                        weight_grams_x10: None,
                    },
                ],
                incubation_method: "Natural - parent reared".into(),
                notes: Some("Successful clutch, both chicks healthy".into()),
            },
            ClutchRecord {
                clutch_number: 2,
                start_date_epoch: 20250,
                nest_box_id: "NB-A12".into(),
                eggs: vec![EggRecord {
                    laid_date_epoch: 20250,
                    fertile: None,
                    hatched: false,
                    hatch_date_epoch: None,
                    chick_band_id: None,
                    weight_grams_x10: Some(150),
                }],
                incubation_method: "Artificial incubator".into(),
                notes: Some("Egg candled at day 10, fertility TBD".into()),
            },
        ],
        feather_condition: "Good overall, mild barbering on chest feathers".into(),
        behavioral_notes: vec![
            "Talks extensively, vocabulary ~50 words".into(),
            "Prefers female handlers".into(),
        ],
    };

    let encoded = encode_to_vec(&patient).expect("encode avian patient");
    let (decoded, _): (AvianPatient, _) =
        decode_from_slice(&encoded).expect("decode avian patient");
    assert_eq!(patient, decoded);
}

#[test]
fn test_rehabilitation_physiotherapy_program() {
    let program = RehabProgram {
        patient_id: 100234,
        diagnosis: "Post-TPLO right stifle".into(),
        surgery_date_epoch: Some(20310),
        program_start_epoch: 20317,
        target_goals: vec![
            "Restore full range of motion".into(),
            "Rebuild quadriceps mass".into(),
            "Return to normal activity by 12 weeks".into(),
        ],
        sessions: vec![RehabSession {
            session_number: 1,
            date_epoch: 20317,
            therapist: "Rehab Tech Fujita (CCRP)".into(),
            rom_measurements: vec![RangeOfMotion {
                joint: "Right stifle".into(),
                flexion_degrees: 70,
                extension_degrees: 140,
                normal_flexion: 42,
                normal_extension: 162,
            }],
            exercises_performed: vec![
                ExerciseProtocol {
                    exercise_name: "Underwater treadmill".into(),
                    sets: 1,
                    reps: 1,
                    duration_seconds: Some(600),
                    resistance_level: Some("Water at stifle level".into()),
                    instructions: "Slow walk, 0.8 km/h".into(),
                },
                ExerciseProtocol {
                    exercise_name: "Passive range of motion".into(),
                    sets: 3,
                    reps: 15,
                    duration_seconds: None,
                    resistance_level: None,
                    instructions: "Gentle flexion/extension of stifle".into(),
                },
            ],
            modalities_used: vec![
                "Therapeutic laser Class IV, 8 J/cm2".into(),
                "Cryotherapy 10 min post-session".into(),
            ],
            girth_measurements_mm: vec![
                ("Right thigh (mid)".into(), 320),
                ("Left thigh (mid)".into(), 365),
            ],
            subjective_improvement: "Bearing more weight on right hind today".into(),
            next_session_plan: vec![
                "Increase UWTM to 12 minutes".into(),
                "Add cavaletti rails if comfortable".into(),
            ],
        }],
        home_exercises: vec![
            ExerciseProtocol {
                exercise_name: "Leash walk".into(),
                sets: 3,
                reps: 1,
                duration_seconds: Some(300),
                resistance_level: None,
                instructions: "Slow controlled walk on flat surface, 3x daily".into(),
            },
            ExerciseProtocol {
                exercise_name: "Sit-to-stand".into(),
                sets: 2,
                reps: 10,
                duration_seconds: None,
                resistance_level: None,
                instructions: "On non-slip surface, ensure symmetrical sit".into(),
            },
        ],
        weight_bearing_status: "Partial weight bearing, toe-touching at rest".into(),
    };

    let encoded = encode_to_vec(&program).expect("encode rehab program");
    let (decoded, _): (RehabProgram, _) =
        decode_from_slice(&encoded).expect("decode rehab program");
    assert_eq!(program, decoded);
}

#[test]
fn test_parasite_screening_prevention() {
    let record = ParasiteScreeningRecord {
        patient_id: 100234,
        heartworm_status: "Negative".into(),
        last_heartworm_test_epoch: Some(20300),
        fecal_results: vec![FecalResult {
            date_epoch: 20300,
            method: "Centrifugal flotation with zinc sulfate".into(),
            parasites_found: vec![ParasiteIdentification {
                organism: "Giardia spp.".into(),
                life_stage: "Cysts".into(),
                quantity: "Moderate".into(),
                zoonotic_risk: true,
            }],
            eggs_per_gram: None,
            lab_name: "IDEXX Reference Lab".into(),
        }],
        preventives: vec![
            PreventiveProduct {
                product_name: "NexGard Spectra".into(),
                active_ingredient: "Afoxolaner + Milbemycin oxime".into(),
                spectrum: vec![
                    "Fleas".into(),
                    "Ticks".into(),
                    "Heartworm".into(),
                    "Roundworm".into(),
                    "Hookworm".into(),
                    "Whipworm".into(),
                ],
                dose_for_weight_range: "25.1-50 kg".into(),
                administration_route: VaccineRoute::Oral,
                frequency: Frequency::OnceDaily,
                last_given_epoch: Some(20290),
                next_due_epoch: Some(20320),
            },
            PreventiveProduct {
                product_name: "Fenbendazole".into(),
                active_ingredient: "Fenbendazole".into(),
                spectrum: vec![
                    "Giardia".into(),
                    "Roundworm".into(),
                    "Hookworm".into(),
                    "Whipworm".into(),
                    "Tapeworm (Taenia)".into(),
                ],
                dose_for_weight_range: "50 mg/kg".into(),
                administration_route: VaccineRoute::Oral,
                frequency: Frequency::OnceDaily,
                last_given_epoch: Some(20300),
                next_due_epoch: Some(20305),
            },
        ],
        environmental_risks: vec![
            "Dog park visits weekly".into(),
            "Endemic heartworm area".into(),
        ],
    };

    let encoded = encode_to_vec(&record).expect("encode parasite record");
    let (decoded, _): (ParasiteScreeningRecord, _) =
        decode_from_slice(&encoded).expect("decode parasite record");
    assert_eq!(record, decoded);
}

#[test]
fn test_multi_species_clinic_schedule() {
    let schedule = DailySchedule {
        date_epoch: 20320,
        clinic_name: "Sakura Animal Hospital".into(),
        appointments: vec![
            Appointment {
                appointment_id: 10001,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 30,
                    veterinarian: "Dr. Sato".into(),
                    room: "Exam 1".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 100234,
                    patient_name: "Bella".into(),
                    species: Species::Canine,
                    owner_name: "Tanaka Yuki".into(),
                    phone: "090-1234-5678".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Post-surgical TPLO recheck".into(),
                    secondary_reasons: vec!["Weight check".into()],
                    requires_sedation: false,
                    requires_fasting: false,
                    special_handling: vec!["Allergy to amoxicillin".into()],
                },
                confirmed: true,
                checked_in: false,
                no_show: false,
                notes: Some("Bring recent radiographs from referral".into()),
            },
            Appointment {
                appointment_id: 10002,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 20,
                    veterinarian: "Dr. Kobayashi".into(),
                    room: "Exam 2".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 200200,
                    patient_name: "Mochi".into(),
                    species: Species::Feline,
                    owner_name: "Suzuki Aiko".into(),
                    phone: "090-9876-5432".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Renal recheck + blood pressure".into(),
                    secondary_reasons: vec!["Thyroid level check".into()],
                    requires_sedation: false,
                    requires_fasting: true,
                    special_handling: vec!["Fractious - feliway in room 15 min prior".into()],
                },
                confirmed: true,
                checked_in: false,
                no_show: false,
                notes: None,
            },
            Appointment {
                appointment_id: 10003,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 45,
                    veterinarian: "Dr. Sato".into(),
                    room: "Exotic Suite".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 400100,
                    patient_name: "Nagini".into(),
                    species: Species::Reptile,
                    owner_name: "Yamada Ken".into(),
                    phone: "080-5555-1234".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Annual wellness exam".into(),
                    secondary_reasons: vec![
                        "Fecal parasite screen".into(),
                        "Husbandry review".into(),
                    ],
                    requires_sedation: false,
                    requires_fasting: false,
                    special_handling: vec![
                        "Handle with gloves - defensive striker".into(),
                        "Maintain warm room temperature".into(),
                    ],
                },
                confirmed: false,
                checked_in: false,
                no_show: false,
                notes: Some("Owner bringing enclosure photos for husbandry review".into()),
            },
        ],
        blocked_slots: vec![
            (20320, "Lunch break 12:00-13:00".into()),
            (20320, "Staff meeting 17:00-17:30".into()),
        ],
        on_call_vet: "Dr. Honda".into(),
    };

    let encoded = encode_to_vec(&schedule).expect("encode daily schedule");
    let (decoded, _): (DailySchedule, _) =
        decode_from_slice(&encoded).expect("decode daily schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_clinical_trial_enrollment_multi_visit() {
    let enrollment = ClinicalTrialEnrollment {
        enrollment_id: "TRIAL-OA-2025-0088".into(),
        patient_id: 100234,
        protocol: TrialProtocol {
            protocol_id: "PROTO-OA-001".into(),
            title: "Efficacy of Novel Anti-NGF Monoclonal Antibody for Canine Osteoarthritis"
                .into(),
            sponsor: "VetPharma Research Inc.".into(),
            phase: "Pivotal field study".into(),
            investigational_product: "caninumab (anti-NGF mAb)".into(),
            control_product: Some("Placebo (saline)".into()),
            total_visits: 6,
            duration_weeks: 24,
        },
        enrollment_date_epoch: 20300,
        randomization_group: "Treatment".into(),
        blinded: true,
        inclusion_criteria: vec![
            InclusionCriteria {
                criterion: "Radiographic evidence of OA in at least one joint".into(),
                met: true,
                verification_method: "Radiographs reviewed by DACVR".into(),
            },
            InclusionCriteria {
                criterion: "CBPI pain score >= 3".into(),
                met: true,
                verification_method: "Owner questionnaire".into(),
            },
            InclusionCriteria {
                criterion: "No NSAIDs for 14 days prior to enrollment".into(),
                met: true,
                verification_method: "Owner report and medical record review".into(),
            },
            InclusionCriteria {
                criterion: "Body weight 10-50 kg".into(),
                met: true,
                verification_method: "Clinic scale, 29.1 kg".into(),
            },
        ],
        consent_obtained: true,
        consent_date_epoch: 20298,
        visits: vec![
            TrialVisit {
                visit_number: 1,
                scheduled_epoch: 20300,
                actual_epoch: Some(20300),
                procedures: vec![
                    "Physical exam".into(),
                    "CBPI owner assessment".into(),
                    "Gait analysis (force plate)".into(),
                    "Blood draw (CBC, chemistry, urinalysis)".into(),
                ],
                measurements: vec![
                    ("CBPI pain score".into(), "5.2".into()),
                    ("Peak vertical force (N/kg)".into(), "4.8".into()),
                    ("Weight (kg)".into(), "29.1".into()),
                ],
                treatment_administered: Some("Study drug SC injection, 0.5 mg/kg".into()),
                adverse_events: vec![],
                investigator_notes: Some("Baseline visit. Patient cooperative.".into()),
            },
            TrialVisit {
                visit_number: 2,
                scheduled_epoch: 20328,
                actual_epoch: Some(20329),
                procedures: vec![
                    "Physical exam".into(),
                    "CBPI owner assessment".into(),
                    "Gait analysis (force plate)".into(),
                ],
                measurements: vec![
                    ("CBPI pain score".into(), "3.8".into()),
                    ("Peak vertical force (N/kg)".into(), "5.6".into()),
                    ("Weight (kg)".into(), "29.3".into()),
                ],
                treatment_administered: None,
                adverse_events: vec![AdverseEvent {
                    date_epoch: 20310,
                    description: "Mild injection site swelling".into(),
                    severity: "Mild".into(),
                    related_to_treatment: "Possibly related".into(),
                    action_taken: "Cold compress applied by owner".into(),
                    resolved: true,
                    resolution_epoch: Some(20313),
                }],
                investigator_notes: Some(
                    "Improvement in pain scores and gait. Owner reports increased activity.".into(),
                ),
            },
        ],
        withdrawn: false,
        withdrawal_reason: None,
    };

    let encoded = encode_to_vec(&enrollment).expect("encode trial enrollment");
    let (decoded, _): (ClinicalTrialEnrollment, _) =
        decode_from_slice(&encoded).expect("decode trial enrollment");
    assert_eq!(enrollment, decoded);
}
