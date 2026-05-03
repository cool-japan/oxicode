// Tests for nested_structs_advanced13 — part A (tests 1-11)
use super::types::*;
use oxicode::{decode_from_slice, encode_to_vec};

#[test]
fn test_patient_record_with_weight_history() {
    let record = PatientRecord {
        patient_id: 100234,
        name: "Bella".into(),
        species: Species::Canine,
        sex: Sex::FemaleSpayed,
        breed: BreedInfo {
            primary_breed: "Labrador Retriever".into(),
            secondary_breed: Some("Golden Retriever".into()),
            breed_percentage: Some(75),
            genetic_test_id: Some("EMB-2025-44821".into()),
        },
        date_of_birth_epoch: Some(19450),
        microchip_id: Some("985121033456789".into()),
        weight_history: vec![
            WeightEntry {
                date_epoch_days: 20100,
                weight_grams: 28500,
                body_condition_score: 5,
                notes: Some("Ideal weight".into()),
            },
            WeightEntry {
                date_epoch_days: 20200,
                weight_grams: 30200,
                body_condition_score: 6,
                notes: Some("Slight weight gain, adjust diet".into()),
            },
            WeightEntry {
                date_epoch_days: 20300,
                weight_grams: 29100,
                body_condition_score: 5,
                notes: None,
            },
        ],
        owners: vec![OwnerContact {
            name: "Tanaka Yuki".into(),
            phone_primary: "090-1234-5678".into(),
            phone_emergency: Some("080-8765-4321".into()),
            email: Some("yuki.tanaka@example.jp".into()),
            address_lines: vec!["Minato-ku Roppongi 1-2-3".into(), "Tokyo 106-0032".into()],
        }],
        allergies: vec!["Chicken protein".into(), "Amoxicillin".into()],
        chronic_conditions: vec!["Hip dysplasia".into()],
    };

    let encoded = encode_to_vec(&record).expect("encode patient record");
    let (decoded, _): (PatientRecord, _) =
        decode_from_slice(&encoded).expect("decode patient record");
    assert_eq!(record, decoded);
}

#[test]
fn test_vaccination_schedule_with_boosters() {
    let record = VaccinationRecord {
        patient_id: 100234,
        series_list: vec![
            VaccinationSeries {
                vaccine_type: VaccinationType::Core,
                disease_target: "Rabies".into(),
                administrations: vec![VaccineAdministration {
                    date_epoch: 20050,
                    vaccine_name: "Imrab 3TF".into(),
                    lot: VaccineLot {
                        manufacturer: "Boehringer Ingelheim".into(),
                        lot_number: "BI-RAB-20250101".into(),
                        expiration_epoch: 20780,
                        storage_temp_c_x10: 40,
                    },
                    route: VaccineRoute::Subcutaneous,
                    site: "Right rear leg".into(),
                    administered_by: "Dr. Sato".into(),
                    reaction_observed: None,
                }],
                booster: Some(BoosterSchedule {
                    next_due_epoch: 21145,
                    interval_days: 1095,
                    is_overdue: false,
                    reminder_sent: false,
                }),
                series_complete: true,
            },
            VaccinationSeries {
                vaccine_type: VaccinationType::Core,
                disease_target: "DHPP".into(),
                administrations: vec![
                    VaccineAdministration {
                        date_epoch: 19500,
                        vaccine_name: "Nobivac DHP".into(),
                        lot: VaccineLot {
                            manufacturer: "MSD Animal Health".into(),
                            lot_number: "MSD-DHP-001".into(),
                            expiration_epoch: 20200,
                            storage_temp_c_x10: 25,
                        },
                        route: VaccineRoute::Subcutaneous,
                        site: "Left shoulder".into(),
                        administered_by: "Dr. Kimura".into(),
                        reaction_observed: Some("Mild swelling at site".into()),
                    },
                    VaccineAdministration {
                        date_epoch: 19521,
                        vaccine_name: "Nobivac DHP".into(),
                        lot: VaccineLot {
                            manufacturer: "MSD Animal Health".into(),
                            lot_number: "MSD-DHP-002".into(),
                            expiration_epoch: 20250,
                            storage_temp_c_x10: 25,
                        },
                        route: VaccineRoute::Subcutaneous,
                        site: "Right shoulder".into(),
                        administered_by: "Dr. Kimura".into(),
                        reaction_observed: None,
                    },
                ],
                booster: Some(BoosterSchedule {
                    next_due_epoch: 20615,
                    interval_days: 365,
                    is_overdue: true,
                    reminder_sent: true,
                }),
                series_complete: true,
            },
        ],
        exemptions: vec!["Leptospirosis - prior adverse reaction".into()],
        titer_tests: vec![TiterResult {
            disease: "Distemper".into(),
            date_epoch: 20300,
            result_value: 256,
            adequate: true,
            lab_name: "VetPath Diagnostics".into(),
        }],
    };

    let encoded = encode_to_vec(&record).expect("encode vaccination record");
    let (decoded, _): (VaccinationRecord, _) =
        decode_from_slice(&encoded).expect("decode vaccination record");
    assert_eq!(record, decoded);
}

#[test]
fn test_cbc_panel_with_reference_ranges() {
    let submission = LabSubmission {
        accession_number: "LAB-2025-08812".into(),
        patient_id: 100234,
        collected_epoch: 20300,
        received_epoch: 20300,
        reported_epoch: Some(20301),
        collected_by: "Tech Nakamura".into(),
        specimen_type: "EDTA whole blood".into(),
        cbc: Some(CbcPanel {
            analytes: vec![
                LabAnalyte {
                    name: "WBC".into(),
                    code: "CBC-WBC".into(),
                    value_x100: 1250,
                    reference: ReferenceRange {
                        low_x100: 550,
                        high_x100: 1680,
                        unit: "x10^9/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "RBC".into(),
                    code: "CBC-RBC".into(),
                    value_x100: 720,
                    reference: ReferenceRange {
                        low_x100: 550,
                        high_x100: 850,
                        unit: "x10^12/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "HCT".into(),
                    code: "CBC-HCT".into(),
                    value_x100: 5200,
                    reference: ReferenceRange {
                        low_x100: 3700,
                        high_x100: 5500,
                        unit: "%".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "PLT".into(),
                    code: "CBC-PLT".into(),
                    value_x100: 8500,
                    reference: ReferenceRange {
                        low_x100: 17500,
                        high_x100: 50000,
                        unit: "x10^9/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Critical,
                },
            ],
            morphology_notes: Some(
                "Occasional target cells noted. Large platelets present.".into(),
            ),
            platelet_estimate: Some("Decreased".into()),
        }),
    };

    let encoded = encode_to_vec(&submission).expect("encode lab submission");
    let (decoded, _): (LabSubmission, _) =
        decode_from_slice(&encoded).expect("decode lab submission");
    assert_eq!(submission, decoded);
}

#[test]
fn test_chemistry_panel_organ_grouped() {
    let report = ChemistryReport {
        accession: "LAB-2025-08813".into(),
        fasting: true,
        hemolysis_index: 0,
        lipemia_index: 1,
        panels: vec![
            OrganPanel {
                organ_system: "Hepatic".into(),
                analytes: vec![
                    LabAnalyte {
                        name: "ALT".into(),
                        code: "CHEM-ALT".into(),
                        value_x100: 4500,
                        reference: ReferenceRange {
                            low_x100: 1000,
                            high_x100: 12500,
                            unit: "U/L".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                    LabAnalyte {
                        name: "ALP".into(),
                        code: "CHEM-ALP".into(),
                        value_x100: 28000,
                        reference: ReferenceRange {
                            low_x100: 2300,
                            high_x100: 21200,
                            unit: "U/L".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::High,
                    },
                ],
                clinical_significance: Some("Elevated ALP may indicate cholestasis".into()),
            },
            OrganPanel {
                organ_system: "Renal".into(),
                analytes: vec![
                    LabAnalyte {
                        name: "BUN".into(),
                        code: "CHEM-BUN".into(),
                        value_x100: 1800,
                        reference: ReferenceRange {
                            low_x100: 700,
                            high_x100: 2700,
                            unit: "mg/dL".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                    LabAnalyte {
                        name: "Creatinine".into(),
                        code: "CHEM-CREA".into(),
                        value_x100: 120,
                        reference: ReferenceRange {
                            low_x100: 50,
                            high_x100: 180,
                            unit: "mg/dL".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                ],
                clinical_significance: None,
            },
        ],
        pathologist_comment: Some("Elevated ALP warrants follow-up bile acids test.".into()),
    };

    let encoded = encode_to_vec(&report).expect("encode chemistry report");
    let (decoded, _): (ChemistryReport, _) =
        decode_from_slice(&encoded).expect("decode chemistry report");
    assert_eq!(report, decoded);
}

#[test]
fn test_urinalysis_with_sediment() {
    let report = UrinalysisReport {
        collection_method: "Cystocentesis".into(),
        color: "Yellow".into(),
        clarity: "Slightly turbid".into(),
        dipstick: UrinalysisDipstick {
            ph_x10: 65,
            specific_gravity_x1000: 1035,
            protein: 1,
            glucose: 0,
            ketones: 0,
            bilirubin: 0,
            blood: 2,
        },
        sediment: vec![
            SedimentFinding {
                element: "RBC".into(),
                quantity_per_hpf: "5-10".into(),
                significance: Some("Mild hematuria".into()),
            },
            SedimentFinding {
                element: "WBC".into(),
                quantity_per_hpf: "0-2".into(),
                significance: None,
            },
            SedimentFinding {
                element: "Struvite crystals".into(),
                quantity_per_hpf: "Moderate".into(),
                significance: Some("Consistent with alkaline urine".into()),
            },
        ],
        culture_submitted: true,
    };

    let encoded = encode_to_vec(&report).expect("encode urinalysis");
    let (decoded, _): (UrinalysisReport, _) =
        decode_from_slice(&encoded).expect("decode urinalysis");
    assert_eq!(report, decoded);
}

#[test]
fn test_surgical_procedure_with_anesthesia() {
    let procedure = SurgicalProcedure {
        procedure_id: 5001,
        patient_id: 100234,
        date_epoch: 20310,
        category: SurgeryCategory::Orthopedic,
        procedure_name: "Tibial Plateau Leveling Osteotomy (TPLO)".into(),
        surgeon: "Dr. Watanabe".into(),
        assistant: Some("Dr. Ito".into()),
        duration_minutes: 95,
        anesthesia: AnesthesiaLog {
            protocol: "Balanced anesthesia".into(),
            induction_agent: "Propofol".into(),
            maintenance_agent: "Isoflurane".into(),
            drugs: vec![
                AnesthesiaDrug {
                    drug_name: "Acepromazine".into(),
                    dose_mg_per_kg_x100: 2,
                    route: "IM".into(),
                    time_elapsed_min: 0,
                },
                AnesthesiaDrug {
                    drug_name: "Hydromorphone".into(),
                    dose_mg_per_kg_x100: 10,
                    route: "IM".into(),
                    time_elapsed_min: 0,
                },
                AnesthesiaDrug {
                    drug_name: "Propofol".into(),
                    dose_mg_per_kg_x100: 400,
                    route: "IV".into(),
                    time_elapsed_min: 20,
                },
            ],
            vitals: vec![
                VitalReading {
                    elapsed_minutes: 25,
                    heart_rate_bpm: 88,
                    resp_rate: 12,
                    spo2_percent: 98,
                    etco2_mmhg: Some(38),
                    systolic_bp: Some(110),
                    diastolic_bp: Some(70),
                    temp_c_x10: 384,
                    anesthesia_stage: AnesthesiaStage::Maintenance,
                },
                VitalReading {
                    elapsed_minutes: 55,
                    heart_rate_bpm: 92,
                    resp_rate: 14,
                    spo2_percent: 97,
                    etco2_mmhg: Some(40),
                    systolic_bp: Some(105),
                    diastolic_bp: Some(65),
                    temp_c_x10: 378,
                    anesthesia_stage: AnesthesiaStage::Maintenance,
                },
                VitalReading {
                    elapsed_minutes: 100,
                    heart_rate_bpm: 110,
                    resp_rate: 20,
                    spo2_percent: 99,
                    etco2_mmhg: None,
                    systolic_bp: None,
                    diastolic_bp: None,
                    temp_c_x10: 374,
                    anesthesia_stage: AnesthesiaStage::Recovery,
                },
            ],
            complications: vec![],
        },
        findings: "Complete cranial cruciate ligament rupture confirmed. Meniscal release performed. Plate angle 5 degrees.".into(),
        complications: vec![],
        post_op_pain_score: PainScore::Moderate,
    };

    let encoded = encode_to_vec(&procedure).expect("encode surgical procedure");
    let (decoded, _): (SurgicalProcedure, _) =
        decode_from_slice(&encoded).expect("decode surgical procedure");
    assert_eq!(procedure, decoded);
}

#[test]
fn test_prescription_records_with_dosing() {
    let history = PrescriptionHistory {
        patient_id: 100234,
        active_prescriptions: vec![
            Prescription {
                rx_number: 900100,
                drug_name: "Carprofen".into(),
                strength: "75mg".into(),
                dose: DoseInstruction {
                    amount_x100: 220,
                    unit: DosageUnit::MgPerKg,
                    frequency: Frequency::TwiceDaily,
                    with_food: true,
                    special_instructions: Some("Monitor for GI upset".into()),
                },
                duration_days: Some(14),
                start_epoch: 20310,
                prescriber: "Dr. Watanabe".into(),
                refill: RefillInfo {
                    refills_authorized: 1,
                    refills_used: 0,
                    last_refill_epoch: None,
                    pharmacy: None,
                },
                warnings: vec!["NSAID - do not combine with corticosteroids".into()],
                drug_interactions: vec!["Aspirin".into(), "Meloxicam".into()],
            },
            Prescription {
                rx_number: 900101,
                drug_name: "Gabapentin".into(),
                strength: "100mg capsules".into(),
                dose: DoseInstruction {
                    amount_x100: 500,
                    unit: DosageUnit::MgPerKg,
                    frequency: Frequency::ThreeTimesDaily,
                    with_food: false,
                    special_instructions: Some("May cause sedation".into()),
                },
                duration_days: Some(30),
                start_epoch: 20310,
                prescriber: "Dr. Watanabe".into(),
                refill: RefillInfo {
                    refills_authorized: 2,
                    refills_used: 0,
                    last_refill_epoch: None,
                    pharmacy: Some("Pet Pharmacy Central".into()),
                },
                warnings: vec![],
                drug_interactions: vec![],
            },
        ],
        past_prescriptions: vec![Prescription {
            rx_number: 800050,
            drug_name: "Amoxicillin-Clavulanate".into(),
            strength: "250mg".into(),
            dose: DoseInstruction {
                amount_x100: 1375,
                unit: DosageUnit::MgPerKg,
                frequency: Frequency::TwiceDaily,
                with_food: true,
                special_instructions: None,
            },
            duration_days: Some(10),
            start_epoch: 20100,
            prescriber: "Dr. Sato".into(),
            refill: RefillInfo {
                refills_authorized: 0,
                refills_used: 0,
                last_refill_epoch: None,
                pharmacy: None,
            },
            warnings: vec![
                "Patient has recorded allergy to Amoxicillin - OVERRIDE by prescriber".into(),
            ],
            drug_interactions: vec![],
        }],
    };

    let encoded = encode_to_vec(&history).expect("encode prescription history");
    let (decoded, _): (PrescriptionHistory, _) =
        decode_from_slice(&encoded).expect("decode prescription history");
    assert_eq!(history, decoded);
}

#[test]
fn test_kennel_boarding_reservation() {
    let reservation = BoardingReservation {
        reservation_id: 3001,
        patient_id: 100234,
        check_in_epoch: 20400,
        check_out_epoch: 20407,
        kennel_size: BoardingSize::Large,
        kennel_number: Some("L-14".into()),
        feeding: vec![
            FeedingSchedule {
                time_of_day: "07:00".into(),
                food_type: "Royal Canin GI Low Fat".into(),
                amount: "1.5 cups".into(),
                special_prep: Some("Add warm water, wait 5 min".into()),
            },
            FeedingSchedule {
                time_of_day: "17:00".into(),
                food_type: "Royal Canin GI Low Fat".into(),
                amount: "1.5 cups".into(),
                special_prep: Some("Add warm water, wait 5 min".into()),
            },
        ],
        medications: vec![MedicationDuringBoarding {
            drug_name: "Gabapentin 100mg".into(),
            dose_instructions: "1 capsule by mouth".into(),
            frequency: Frequency::ThreeTimesDaily,
            supplied_by_owner: true,
        }],
        exercise_notes: Some(
            "Leash walks only - post-surgical recovery. No running or jumping.".into(),
        ),
        behavioral_notes: Some("Anxious around cats. Friendly with other dogs.".into()),
        emergency_contact: OwnerContact {
            name: "Tanaka Yuki".into(),
            phone_primary: "090-1234-5678".into(),
            phone_emergency: Some("080-8765-4321".into()),
            email: Some("yuki.tanaka@example.jp".into()),
            address_lines: vec!["Minato-ku Roppongi 1-2-3".into()],
        },
        vaccination_verified: true,
        belongings: vec![
            "Blue blanket".into(),
            "Tennis ball".into(),
            "Medication bag".into(),
        ],
    };

    let encoded = encode_to_vec(&reservation).expect("encode boarding reservation");
    let (decoded, _): (BoardingReservation, _) =
        decode_from_slice(&encoded).expect("decode boarding reservation");
    assert_eq!(reservation, decoded);
}

#[test]
fn test_livestock_herd_health_program() {
    let program = HerdHealthProgram {
        herd_id: "HERD-JP-2025-0042".into(),
        species: Species::Bovine,
        total_head: 85,
        location: "Hokkaido Dairy Farm #7".into(),
        animals: vec![
            HerdAnimal {
                ear_tag: "JP-0042-0001".into(),
                rfid: Some("840003148812345".into()),
                sex: Sex::Female,
                birth_date_epoch: Some(18900),
                dam_tag: Some("JP-0042-0089".into()),
                sire_tag: Some("AI-HOLST-9921".into()),
            },
            HerdAnimal {
                ear_tag: "JP-0042-0002".into(),
                rfid: Some("840003148812346".into()),
                sex: Sex::Female,
                birth_date_epoch: Some(19100),
                dam_tag: None,
                sire_tag: None,
            },
        ],
        test_history: vec![
            HerdTest {
                test_type: HerdTestType::Tuberculin,
                date_epoch: 20200,
                animals_tested: 85,
                positives: 0,
                inconclusive: 1,
                lab_report_id: Some("TB-20250301-042".into()),
            },
            HerdTest {
                test_type: HerdTestType::BVD,
                date_epoch: 20200,
                animals_tested: 85,
                positives: 2,
                inconclusive: 0,
                lab_report_id: Some("BVD-20250301-042".into()),
            },
        ],
        protocols: vec![TreatmentProtocol {
            condition: "Clinical mastitis".into(),
            drug: "Ceftiofur".into(),
            withdrawal_days_meat: 13,
            withdrawal_days_milk: 4,
            dosage_instructions: "1 mg/kg IM SID x 5 days".into(),
        }],
        next_vet_visit_epoch: Some(20500),
        certifications: vec!["TB-Free Status".into(), "Brucellosis-Free Status".into()],
    };

    let encoded = encode_to_vec(&program).expect("encode herd health program");
    let (decoded, _): (HerdHealthProgram, _) =
        decode_from_slice(&encoded).expect("decode herd health program");
    assert_eq!(program, decoded);
}

#[test]
fn test_radiology_study_hierarchy() {
    let study = RadiologyStudy {
        study_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234".into(),
        patient_id: 100234,
        date_epoch: 20280,
        referring_vet: "Dr. Sato".into(),
        radiologist: Some("Dr. Yamamoto (DACVR)".into()),
        clinical_indication: "Right pelvic limb lameness, suspected CCL rupture".into(),
        series: vec![
            RadiologySeries {
                series_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.1".into(),
                series_number: 1,
                modality: ImagingModality::Radiograph,
                body_part: "Right stifle".into(),
                description: "Lateral view".into(),
                images: vec![RadiologyImage {
                    image_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.1.1".into(),
                    instance_number: 1,
                    laterality: Laterality::Right,
                    position: "Lateral".into(),
                    kvp: 55,
                    mas: 8,
                    exposure_time_ms: 20,
                    file_size_bytes: 4_200_000,
                }],
            },
            RadiologySeries {
                series_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.2".into(),
                series_number: 2,
                modality: ImagingModality::Radiograph,
                body_part: "Right stifle".into(),
                description: "CrCd view".into(),
                images: vec![RadiologyImage {
                    image_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.2.1".into(),
                    instance_number: 1,
                    laterality: Laterality::Right,
                    position: "CrCd".into(),
                    kvp: 60,
                    mas: 10,
                    exposure_time_ms: 25,
                    file_size_bytes: 4_500_000,
                }],
            },
        ],
        findings: vec![
            RadiologyFinding {
                location: "Right stifle joint".into(),
                description: "Moderate joint effusion with cranial tibial thrust".into(),
                severity: "Moderate".into(),
                differential_diagnoses: vec![
                    "Cranial cruciate ligament rupture".into(),
                    "Meniscal injury".into(),
                ],
            },
            RadiologyFinding {
                location: "Right femoral trochlea".into(),
                description: "Mild osteophyte formation on trochlear ridges".into(),
                severity: "Mild".into(),
                differential_diagnoses: vec!["Degenerative joint disease".into()],
            },
        ],
        impression: Some(
            "Findings consistent with cranial cruciate ligament rupture with secondary DJD.".into(),
        ),
        recommendations: vec![
            "Surgical consultation for TPLO".into(),
            "Follow-up radiographs in 8 weeks post-surgery".into(),
        ],
    };

    let encoded = encode_to_vec(&study).expect("encode radiology study");
    let (decoded, _): (RadiologyStudy, _) =
        decode_from_slice(&encoded).expect("decode radiology study");
    assert_eq!(study, decoded);
}

#[test]
fn test_dental_charting() {
    let chart = DentalChart {
        patient_id: 200100,
        date_epoch: 20350,
        veterinarian: "Dr. Kobayashi".into(),
        dental_grade: 3,
        teeth: vec![
            ToothRecord {
                tooth_number: 108,
                quadrant: 1,
                condition: ToothCondition::Periodontitis,
                pocket_depths_mm: vec![4, 5, 6, 4],
                mobility_grade: 2,
                notes: Some("Stage 3 periodontal disease, recommend extraction".into()),
            },
            ToothRecord {
                tooth_number: 204,
                quadrant: 2,
                condition: ToothCondition::Fractured,
                pocket_depths_mm: vec![2, 2, 3, 2],
                mobility_grade: 0,
                notes: Some("Complicated crown fracture with pulp exposure".into()),
            },
            ToothRecord {
                tooth_number: 309,
                quadrant: 3,
                condition: ToothCondition::Resorptive,
                pocket_depths_mm: vec![3, 4, 5, 3],
                mobility_grade: 1,
                notes: Some("Type 2 tooth resorption on radiograph".into()),
            },
            ToothRecord {
                tooth_number: 404,
                quadrant: 4,
                condition: ToothCondition::Healthy,
                pocket_depths_mm: vec![1, 1, 2, 1],
                mobility_grade: 0,
                notes: None,
            },
        ],
        procedures_performed: vec![
            DentalProcedureEntry {
                tooth_number: 108,
                procedure: "Surgical extraction".into(),
                material_used: Some("Absorbable suture 4-0".into()),
            },
            DentalProcedureEntry {
                tooth_number: 204,
                procedure: "Vital pulp therapy".into(),
                material_used: Some("MTA cement".into()),
            },
            DentalProcedureEntry {
                tooth_number: 309,
                procedure: "Crown amputation".into(),
                material_used: None,
            },
        ],
        scaling_performed: true,
        polishing_performed: true,
        fluoride_applied: true,
        dental_radiographs_taken: 6,
        home_care_recommendations: vec![
            "Daily tooth brushing with enzymatic paste".into(),
            "Dental chews (VOHC accepted)".into(),
            "Recheck in 6 months".into(),
        ],
    };

    let encoded = encode_to_vec(&chart).expect("encode dental chart");
    let (decoded, _): (DentalChart, _) = decode_from_slice(&encoded).expect("decode dental chart");
    assert_eq!(chart, decoded);
}
