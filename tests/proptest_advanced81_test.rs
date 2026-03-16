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

// ── Domain types: Pet Care Services & Animal Wellness ──────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Species {
    Dog,
    Cat,
    Bird,
    Rabbit,
    Reptile,
    Fish,
    SmallMammal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PetProfile {
    pet_id: u64,
    species: Species,
    breed_hash: u32,
    name_len: u8,
    age_months: u16,
    weight_grams: u32,
    microchip_id: u64,
    is_spayed_neutered: bool,
    color_code: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GroomingService {
    FullGroom,
    BathOnly,
    NailTrim,
    EarCleaning,
    TeethBrushing,
    DeShedding,
    FleaTreatment,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroomingAppointment {
    appointment_id: u64,
    pet_id: u64,
    service: GroomingService,
    scheduled_epoch: u64,
    duration_minutes: u16,
    groomer_id: u32,
    price_cents: u32,
    notes_hash: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoardingRoomType {
    StandardKennel,
    LuxurySuite,
    CatCondo,
    ExoticEnclosure,
    GroupPlay,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoardingReservation {
    reservation_id: u64,
    pet_id: u64,
    room_type: BoardingRoomType,
    check_in_epoch: u64,
    check_out_epoch: u64,
    daily_rate_cents: u32,
    special_needs_flags: u16,
    emergency_contact_hash: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DaycareAttendanceLog {
    log_id: u64,
    pet_id: u64,
    date_epoch: u64,
    drop_off_minutes: u16,
    pick_up_minutes: u16,
    play_group_id: u8,
    temperament_score: u8,
    incident_flags: u16,
    staff_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrainingSkill {
    Sit,
    Stay,
    Come,
    Heel,
    Down,
    LeaveIt,
    ShakeHands,
    Rollover,
    CrateTraining,
    Socialization,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingSessionProgress {
    session_id: u64,
    pet_id: u64,
    trainer_id: u32,
    skill: TrainingSkill,
    proficiency_pct: u8,
    repetitions: u16,
    treat_count: u16,
    session_epoch: u64,
    duration_minutes: u16,
    notes_hash: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DietType {
    Kibble,
    WetFood,
    RawDiet,
    Homemade,
    Prescription,
    GrainFree,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutritionPlan {
    plan_id: u64,
    pet_id: u64,
    diet_type: DietType,
    calories_per_day: u16,
    protein_pct_x10: u16,
    fat_pct_x10: u16,
    fiber_pct_x10: u16,
    servings_per_day: u8,
    serving_grams: u16,
    supplement_flags: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClaimStatus {
    Submitted,
    UnderReview,
    Approved,
    Denied,
    Paid,
    Appealed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceClaim {
    claim_id: u64,
    policy_id: u64,
    pet_id: u64,
    status: ClaimStatus,
    claim_amount_cents: u32,
    deductible_cents: u32,
    approved_amount_cents: u32,
    submitted_epoch: u64,
    condition_code: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdoptionApplication {
    application_id: u64,
    applicant_hash: u64,
    pet_id: u64,
    household_size: u8,
    has_yard: bool,
    has_other_pets: bool,
    experience_years: u8,
    income_bracket: u8,
    submitted_epoch: u64,
    score: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsWaypoint {
    latitude_x1e7: i32,
    longitude_x1e7: i32,
    altitude_cm: i32,
    timestamp_offset_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DogWalkingTrack {
    walk_id: u64,
    pet_id: u64,
    walker_id: u32,
    start_epoch: u64,
    total_distance_meters: u32,
    duration_seconds: u16,
    avg_pace_sec_per_km: u16,
    waypoint_count: u16,
    poop_bag_used: u8,
    weather_code: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BehavioralAssessment {
    assessment_id: u64,
    pet_id: u64,
    assessor_id: u32,
    aggression_score: u8,
    anxiety_score: u8,
    sociability_score: u8,
    trainability_score: u8,
    energy_level: u8,
    noise_sensitivity: u8,
    resource_guarding: u8,
    overall_rating: u16,
    assessed_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VisitTaskType {
    Feeding,
    WaterRefresh,
    PottyBreak,
    Medication,
    Playtime,
    Brushing,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PetSittingVisit {
    visit_id: u64,
    pet_id: u64,
    sitter_id: u32,
    arrival_epoch: u64,
    departure_epoch: u64,
    task: VisitTaskType,
    completed: bool,
    photo_count: u8,
    notes_hash: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VaccineType {
    Rabies,
    Distemper,
    Parvovirus,
    Bordetella,
    Leptospirosis,
    Feline3Way,
    Feline4Way,
    InfluenzaH3N2,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccinationReminder {
    reminder_id: u64,
    pet_id: u64,
    vaccine: VaccineType,
    last_given_epoch: u64,
    next_due_epoch: u64,
    interval_days: u16,
    vet_clinic_id: u32,
    is_overdue: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ScreeningType {
    HipDysplasia,
    ElbowDysplasia,
    CardiacExam,
    EyeCertification,
    ThyroidPanel,
    DegenerativeMyelopathy,
    VonWillebrand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BreedHealthScreening {
    screening_id: u64,
    pet_id: u64,
    screening_type: ScreeningType,
    result_code: u8,
    severity_scale: u8,
    performed_epoch: u64,
    vet_id: u32,
    cost_cents: u32,
    follow_up_days: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PetPhotoSession {
    session_id: u64,
    pet_id: u64,
    photographer_id: u32,
    photo_count: u16,
    best_photo_hash: u64,
    backdrop_code: u8,
    lighting_code: u8,
    session_epoch: u64,
    duration_minutes: u16,
    package_price_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RewardTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoyaltyRewards {
    member_id: u64,
    pet_id: u64,
    tier: RewardTier,
    points_balance: u32,
    lifetime_points: u64,
    visits_this_year: u16,
    referral_count: u8,
    joined_epoch: u64,
    last_visit_epoch: u64,
}

// ── Strategies ──────────────────────────────────────────────────────────────

fn arb_species() -> impl Strategy<Value = Species> {
    prop_oneof![
        Just(Species::Dog),
        Just(Species::Cat),
        Just(Species::Bird),
        Just(Species::Rabbit),
        Just(Species::Reptile),
        Just(Species::Fish),
        Just(Species::SmallMammal),
    ]
}

prop_compose! {
    fn arb_pet_profile()(
        pet_id in any::<u64>(),
        species in arb_species(),
        breed_hash in any::<u32>(),
        name_len in any::<u8>(),
        age_months in any::<u16>(),
        weight_grams in any::<u32>(),
        microchip_id in any::<u64>(),
        is_spayed_neutered in any::<bool>(),
        color_code in any::<u16>(),
    ) -> PetProfile {
        PetProfile {
            pet_id, species, breed_hash, name_len, age_months,
            weight_grams, microchip_id, is_spayed_neutered, color_code,
        }
    }
}

fn arb_grooming_service() -> impl Strategy<Value = GroomingService> {
    prop_oneof![
        Just(GroomingService::FullGroom),
        Just(GroomingService::BathOnly),
        Just(GroomingService::NailTrim),
        Just(GroomingService::EarCleaning),
        Just(GroomingService::TeethBrushing),
        Just(GroomingService::DeShedding),
        Just(GroomingService::FleaTreatment),
    ]
}

prop_compose! {
    fn arb_grooming_appointment()(
        appointment_id in any::<u64>(),
        pet_id in any::<u64>(),
        service in arb_grooming_service(),
        scheduled_epoch in any::<u64>(),
        duration_minutes in any::<u16>(),
        groomer_id in any::<u32>(),
        price_cents in any::<u32>(),
        notes_hash in any::<u64>(),
    ) -> GroomingAppointment {
        GroomingAppointment {
            appointment_id, pet_id, service, scheduled_epoch,
            duration_minutes, groomer_id, price_cents, notes_hash,
        }
    }
}

fn arb_room_type() -> impl Strategy<Value = BoardingRoomType> {
    prop_oneof![
        Just(BoardingRoomType::StandardKennel),
        Just(BoardingRoomType::LuxurySuite),
        Just(BoardingRoomType::CatCondo),
        Just(BoardingRoomType::ExoticEnclosure),
        Just(BoardingRoomType::GroupPlay),
    ]
}

prop_compose! {
    fn arb_boarding_reservation()(
        reservation_id in any::<u64>(),
        pet_id in any::<u64>(),
        room_type in arb_room_type(),
        check_in_epoch in any::<u64>(),
        check_out_epoch in any::<u64>(),
        daily_rate_cents in any::<u32>(),
        special_needs_flags in any::<u16>(),
        emergency_contact_hash in any::<u64>(),
    ) -> BoardingReservation {
        BoardingReservation {
            reservation_id, pet_id, room_type, check_in_epoch,
            check_out_epoch, daily_rate_cents, special_needs_flags,
            emergency_contact_hash,
        }
    }
}

prop_compose! {
    fn arb_daycare_log()(
        log_id in any::<u64>(),
        pet_id in any::<u64>(),
        date_epoch in any::<u64>(),
        drop_off_minutes in any::<u16>(),
        pick_up_minutes in any::<u16>(),
        play_group_id in any::<u8>(),
        temperament_score in any::<u8>(),
        incident_flags in any::<u16>(),
        staff_id in any::<u32>(),
    ) -> DaycareAttendanceLog {
        DaycareAttendanceLog {
            log_id, pet_id, date_epoch, drop_off_minutes, pick_up_minutes,
            play_group_id, temperament_score, incident_flags, staff_id,
        }
    }
}

fn arb_training_skill() -> impl Strategy<Value = TrainingSkill> {
    prop_oneof![
        Just(TrainingSkill::Sit),
        Just(TrainingSkill::Stay),
        Just(TrainingSkill::Come),
        Just(TrainingSkill::Heel),
        Just(TrainingSkill::Down),
        Just(TrainingSkill::LeaveIt),
        Just(TrainingSkill::ShakeHands),
        Just(TrainingSkill::Rollover),
        Just(TrainingSkill::CrateTraining),
        Just(TrainingSkill::Socialization),
    ]
}

prop_compose! {
    fn arb_training_session()(
        session_id in any::<u64>(),
        pet_id in any::<u64>(),
        trainer_id in any::<u32>(),
        skill in arb_training_skill(),
        proficiency_pct in any::<u8>(),
        repetitions in any::<u16>(),
        treat_count in any::<u16>(),
        session_epoch in any::<u64>(),
        duration_minutes in any::<u16>(),
        notes_hash in any::<u64>(),
    ) -> TrainingSessionProgress {
        TrainingSessionProgress {
            session_id, pet_id, trainer_id, skill, proficiency_pct,
            repetitions, treat_count, session_epoch, duration_minutes,
            notes_hash,
        }
    }
}

fn arb_diet_type() -> impl Strategy<Value = DietType> {
    prop_oneof![
        Just(DietType::Kibble),
        Just(DietType::WetFood),
        Just(DietType::RawDiet),
        Just(DietType::Homemade),
        Just(DietType::Prescription),
        Just(DietType::GrainFree),
    ]
}

prop_compose! {
    fn arb_nutrition_plan()(
        plan_id in any::<u64>(),
        pet_id in any::<u64>(),
        diet_type in arb_diet_type(),
        calories_per_day in any::<u16>(),
        protein_pct_x10 in any::<u16>(),
        fat_pct_x10 in any::<u16>(),
        fiber_pct_x10 in any::<u16>(),
        servings_per_day in any::<u8>(),
        serving_grams in any::<u16>(),
        supplement_flags in any::<u32>(),
    ) -> NutritionPlan {
        NutritionPlan {
            plan_id, pet_id, diet_type, calories_per_day, protein_pct_x10,
            fat_pct_x10, fiber_pct_x10, servings_per_day, serving_grams,
            supplement_flags,
        }
    }
}

fn arb_claim_status() -> impl Strategy<Value = ClaimStatus> {
    prop_oneof![
        Just(ClaimStatus::Submitted),
        Just(ClaimStatus::UnderReview),
        Just(ClaimStatus::Approved),
        Just(ClaimStatus::Denied),
        Just(ClaimStatus::Paid),
        Just(ClaimStatus::Appealed),
    ]
}

prop_compose! {
    fn arb_insurance_claim()(
        claim_id in any::<u64>(),
        policy_id in any::<u64>(),
        pet_id in any::<u64>(),
        status in arb_claim_status(),
        claim_amount_cents in any::<u32>(),
        deductible_cents in any::<u32>(),
        approved_amount_cents in any::<u32>(),
        submitted_epoch in any::<u64>(),
        condition_code in any::<u16>(),
    ) -> InsuranceClaim {
        InsuranceClaim {
            claim_id, policy_id, pet_id, status, claim_amount_cents,
            deductible_cents, approved_amount_cents, submitted_epoch,
            condition_code,
        }
    }
}

prop_compose! {
    fn arb_adoption_application()(
        application_id in any::<u64>(),
        applicant_hash in any::<u64>(),
        pet_id in any::<u64>(),
        household_size in any::<u8>(),
        has_yard in any::<bool>(),
        has_other_pets in any::<bool>(),
        experience_years in any::<u8>(),
        income_bracket in any::<u8>(),
        submitted_epoch in any::<u64>(),
        score in any::<u16>(),
    ) -> AdoptionApplication {
        AdoptionApplication {
            application_id, applicant_hash, pet_id, household_size,
            has_yard, has_other_pets, experience_years, income_bracket,
            submitted_epoch, score,
        }
    }
}

prop_compose! {
    fn arb_gps_waypoint()(
        latitude_x1e7 in any::<i32>(),
        longitude_x1e7 in any::<i32>(),
        altitude_cm in any::<i32>(),
        timestamp_offset_ms in any::<u32>(),
    ) -> GpsWaypoint {
        GpsWaypoint {
            latitude_x1e7, longitude_x1e7, altitude_cm, timestamp_offset_ms,
        }
    }
}

prop_compose! {
    fn arb_dog_walking_track()(
        walk_id in any::<u64>(),
        pet_id in any::<u64>(),
        walker_id in any::<u32>(),
        start_epoch in any::<u64>(),
        total_distance_meters in any::<u32>(),
        duration_seconds in any::<u16>(),
        avg_pace_sec_per_km in any::<u16>(),
        waypoint_count in any::<u16>(),
        poop_bag_used in any::<u8>(),
        weather_code in any::<u8>(),
    ) -> DogWalkingTrack {
        DogWalkingTrack {
            walk_id, pet_id, walker_id, start_epoch, total_distance_meters,
            duration_seconds, avg_pace_sec_per_km, waypoint_count,
            poop_bag_used, weather_code,
        }
    }
}

prop_compose! {
    fn arb_behavioral_assessment()(
        assessment_id in any::<u64>(),
        pet_id in any::<u64>(),
        assessor_id in any::<u32>(),
        aggression_score in any::<u8>(),
        anxiety_score in any::<u8>(),
        sociability_score in any::<u8>(),
        trainability_score in any::<u8>(),
        energy_level in any::<u8>(),
        noise_sensitivity in any::<u8>(),
        resource_guarding in any::<u8>(),
        overall_rating in any::<u16>(),
        assessed_epoch in any::<u64>(),
    ) -> BehavioralAssessment {
        BehavioralAssessment {
            assessment_id, pet_id, assessor_id, aggression_score,
            anxiety_score, sociability_score, trainability_score,
            energy_level, noise_sensitivity, resource_guarding,
            overall_rating, assessed_epoch,
        }
    }
}

fn arb_visit_task() -> impl Strategy<Value = VisitTaskType> {
    prop_oneof![
        Just(VisitTaskType::Feeding),
        Just(VisitTaskType::WaterRefresh),
        Just(VisitTaskType::PottyBreak),
        Just(VisitTaskType::Medication),
        Just(VisitTaskType::Playtime),
        Just(VisitTaskType::Brushing),
    ]
}

prop_compose! {
    fn arb_pet_sitting_visit()(
        visit_id in any::<u64>(),
        pet_id in any::<u64>(),
        sitter_id in any::<u32>(),
        arrival_epoch in any::<u64>(),
        departure_epoch in any::<u64>(),
        task in arb_visit_task(),
        completed in any::<bool>(),
        photo_count in any::<u8>(),
        notes_hash in any::<u64>(),
    ) -> PetSittingVisit {
        PetSittingVisit {
            visit_id, pet_id, sitter_id, arrival_epoch, departure_epoch,
            task, completed, photo_count, notes_hash,
        }
    }
}

fn arb_vaccine_type() -> impl Strategy<Value = VaccineType> {
    prop_oneof![
        Just(VaccineType::Rabies),
        Just(VaccineType::Distemper),
        Just(VaccineType::Parvovirus),
        Just(VaccineType::Bordetella),
        Just(VaccineType::Leptospirosis),
        Just(VaccineType::Feline3Way),
        Just(VaccineType::Feline4Way),
        Just(VaccineType::InfluenzaH3N2),
    ]
}

prop_compose! {
    fn arb_vaccination_reminder()(
        reminder_id in any::<u64>(),
        pet_id in any::<u64>(),
        vaccine in arb_vaccine_type(),
        last_given_epoch in any::<u64>(),
        next_due_epoch in any::<u64>(),
        interval_days in any::<u16>(),
        vet_clinic_id in any::<u32>(),
        is_overdue in any::<bool>(),
    ) -> VaccinationReminder {
        VaccinationReminder {
            reminder_id, pet_id, vaccine, last_given_epoch, next_due_epoch,
            interval_days, vet_clinic_id, is_overdue,
        }
    }
}

fn arb_screening_type() -> impl Strategy<Value = ScreeningType> {
    prop_oneof![
        Just(ScreeningType::HipDysplasia),
        Just(ScreeningType::ElbowDysplasia),
        Just(ScreeningType::CardiacExam),
        Just(ScreeningType::EyeCertification),
        Just(ScreeningType::ThyroidPanel),
        Just(ScreeningType::DegenerativeMyelopathy),
        Just(ScreeningType::VonWillebrand),
    ]
}

prop_compose! {
    fn arb_breed_health_screening()(
        screening_id in any::<u64>(),
        pet_id in any::<u64>(),
        screening_type in arb_screening_type(),
        result_code in any::<u8>(),
        severity_scale in any::<u8>(),
        performed_epoch in any::<u64>(),
        vet_id in any::<u32>(),
        cost_cents in any::<u32>(),
        follow_up_days in any::<u16>(),
    ) -> BreedHealthScreening {
        BreedHealthScreening {
            screening_id, pet_id, screening_type, result_code,
            severity_scale, performed_epoch, vet_id, cost_cents,
            follow_up_days,
        }
    }
}

prop_compose! {
    fn arb_pet_photo_session()(
        session_id in any::<u64>(),
        pet_id in any::<u64>(),
        photographer_id in any::<u32>(),
        photo_count in any::<u16>(),
        best_photo_hash in any::<u64>(),
        backdrop_code in any::<u8>(),
        lighting_code in any::<u8>(),
        session_epoch in any::<u64>(),
        duration_minutes in any::<u16>(),
        package_price_cents in any::<u32>(),
    ) -> PetPhotoSession {
        PetPhotoSession {
            session_id, pet_id, photographer_id, photo_count,
            best_photo_hash, backdrop_code, lighting_code,
            session_epoch, duration_minutes, package_price_cents,
        }
    }
}

fn arb_reward_tier() -> impl Strategy<Value = RewardTier> {
    prop_oneof![
        Just(RewardTier::Bronze),
        Just(RewardTier::Silver),
        Just(RewardTier::Gold),
        Just(RewardTier::Platinum),
        Just(RewardTier::Diamond),
    ]
}

prop_compose! {
    fn arb_loyalty_rewards()(
        member_id in any::<u64>(),
        pet_id in any::<u64>(),
        tier in arb_reward_tier(),
        points_balance in any::<u32>(),
        lifetime_points in any::<u64>(),
        visits_this_year in any::<u16>(),
        referral_count in any::<u8>(),
        joined_epoch in any::<u64>(),
        last_visit_epoch in any::<u64>(),
    ) -> LoyaltyRewards {
        LoyaltyRewards {
            member_id, pet_id, tier, points_balance, lifetime_points,
            visits_this_year, referral_count, joined_epoch, last_visit_epoch,
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

// 1. Pet profile roundtrip
#[test]
fn test_pet_profile_roundtrip() {
    proptest!(|(profile in arb_pet_profile())| {
        let bytes = encode_to_vec(&profile).expect("encode PetProfile");
        let (decoded, _): (PetProfile, usize) =
            decode_from_slice(&bytes).expect("decode PetProfile");
        prop_assert_eq!(profile, decoded);
    });
}

// 2. Grooming appointment roundtrip
#[test]
fn test_grooming_appointment_roundtrip() {
    proptest!(|(appt in arb_grooming_appointment())| {
        let bytes = encode_to_vec(&appt).expect("encode GroomingAppointment");
        let (decoded, _): (GroomingAppointment, usize) =
            decode_from_slice(&bytes).expect("decode GroomingAppointment");
        prop_assert_eq!(appt, decoded);
    });
}

// 3. Boarding reservation roundtrip
#[test]
fn test_boarding_reservation_roundtrip() {
    proptest!(|(res in arb_boarding_reservation())| {
        let bytes = encode_to_vec(&res).expect("encode BoardingReservation");
        let (decoded, _): (BoardingReservation, usize) =
            decode_from_slice(&bytes).expect("decode BoardingReservation");
        prop_assert_eq!(res, decoded);
    });
}

// 4. Daycare attendance log roundtrip
#[test]
fn test_daycare_attendance_roundtrip() {
    proptest!(|(log in arb_daycare_log())| {
        let bytes = encode_to_vec(&log).expect("encode DaycareAttendanceLog");
        let (decoded, _): (DaycareAttendanceLog, usize) =
            decode_from_slice(&bytes).expect("decode DaycareAttendanceLog");
        prop_assert_eq!(log, decoded);
    });
}

// 5. Training session progress roundtrip
#[test]
fn test_training_session_roundtrip() {
    proptest!(|(session in arb_training_session())| {
        let bytes = encode_to_vec(&session).expect("encode TrainingSessionProgress");
        let (decoded, _): (TrainingSessionProgress, usize) =
            decode_from_slice(&bytes).expect("decode TrainingSessionProgress");
        prop_assert_eq!(session, decoded);
    });
}

// 6. Nutrition plan roundtrip
#[test]
fn test_nutrition_plan_roundtrip() {
    proptest!(|(plan in arb_nutrition_plan())| {
        let bytes = encode_to_vec(&plan).expect("encode NutritionPlan");
        let (decoded, _): (NutritionPlan, usize) =
            decode_from_slice(&bytes).expect("decode NutritionPlan");
        prop_assert_eq!(plan, decoded);
    });
}

// 7. Insurance claim roundtrip
#[test]
fn test_insurance_claim_roundtrip() {
    proptest!(|(claim in arb_insurance_claim())| {
        let bytes = encode_to_vec(&claim).expect("encode InsuranceClaim");
        let (decoded, _): (InsuranceClaim, usize) =
            decode_from_slice(&bytes).expect("decode InsuranceClaim");
        prop_assert_eq!(claim, decoded);
    });
}

// 8. Adoption application roundtrip
#[test]
fn test_adoption_application_roundtrip() {
    proptest!(|(app in arb_adoption_application())| {
        let bytes = encode_to_vec(&app).expect("encode AdoptionApplication");
        let (decoded, _): (AdoptionApplication, usize) =
            decode_from_slice(&bytes).expect("decode AdoptionApplication");
        prop_assert_eq!(app, decoded);
    });
}

// 9. Dog walking GPS track roundtrip
#[test]
fn test_dog_walking_track_roundtrip() {
    proptest!(|(track in arb_dog_walking_track())| {
        let bytes = encode_to_vec(&track).expect("encode DogWalkingTrack");
        let (decoded, _): (DogWalkingTrack, usize) =
            decode_from_slice(&bytes).expect("decode DogWalkingTrack");
        prop_assert_eq!(track, decoded);
    });
}

// 10. Behavioral assessment roundtrip
#[test]
fn test_behavioral_assessment_roundtrip() {
    proptest!(|(assessment in arb_behavioral_assessment())| {
        let bytes = encode_to_vec(&assessment).expect("encode BehavioralAssessment");
        let (decoded, _): (BehavioralAssessment, usize) =
            decode_from_slice(&bytes).expect("decode BehavioralAssessment");
        prop_assert_eq!(assessment, decoded);
    });
}

// 11. Pet sitting visit roundtrip
#[test]
fn test_pet_sitting_visit_roundtrip() {
    proptest!(|(visit in arb_pet_sitting_visit())| {
        let bytes = encode_to_vec(&visit).expect("encode PetSittingVisit");
        let (decoded, _): (PetSittingVisit, usize) =
            decode_from_slice(&bytes).expect("decode PetSittingVisit");
        prop_assert_eq!(visit, decoded);
    });
}

// 12. Vaccination reminder roundtrip
#[test]
fn test_vaccination_reminder_roundtrip() {
    proptest!(|(reminder in arb_vaccination_reminder())| {
        let bytes = encode_to_vec(&reminder).expect("encode VaccinationReminder");
        let (decoded, _): (VaccinationReminder, usize) =
            decode_from_slice(&bytes).expect("decode VaccinationReminder");
        prop_assert_eq!(reminder, decoded);
    });
}

// 13. Breed health screening roundtrip
#[test]
fn test_breed_health_screening_roundtrip() {
    proptest!(|(screening in arb_breed_health_screening())| {
        let bytes = encode_to_vec(&screening).expect("encode BreedHealthScreening");
        let (decoded, _): (BreedHealthScreening, usize) =
            decode_from_slice(&bytes).expect("decode BreedHealthScreening");
        prop_assert_eq!(screening, decoded);
    });
}

// 14. Pet photo session roundtrip
#[test]
fn test_pet_photo_session_roundtrip() {
    proptest!(|(session in arb_pet_photo_session())| {
        let bytes = encode_to_vec(&session).expect("encode PetPhotoSession");
        let (decoded, _): (PetPhotoSession, usize) =
            decode_from_slice(&bytes).expect("decode PetPhotoSession");
        prop_assert_eq!(session, decoded);
    });
}

// 15. Loyalty rewards roundtrip
#[test]
fn test_loyalty_rewards_roundtrip() {
    proptest!(|(rewards in arb_loyalty_rewards())| {
        let bytes = encode_to_vec(&rewards).expect("encode LoyaltyRewards");
        let (decoded, _): (LoyaltyRewards, usize) =
            decode_from_slice(&bytes).expect("decode LoyaltyRewards");
        prop_assert_eq!(rewards, decoded);
    });
}

// 16. GPS waypoint roundtrip
#[test]
fn test_gps_waypoint_roundtrip() {
    proptest!(|(wp in arb_gps_waypoint())| {
        let bytes = encode_to_vec(&wp).expect("encode GpsWaypoint");
        let (decoded, _): (GpsWaypoint, usize) =
            decode_from_slice(&bytes).expect("decode GpsWaypoint");
        prop_assert_eq!(wp, decoded);
    });
}

// 17. Pet profile deterministic encoding
#[test]
fn test_pet_profile_deterministic() {
    proptest!(|(profile in arb_pet_profile())| {
        let bytes_a = encode_to_vec(&profile).expect("encode PetProfile first");
        let bytes_b = encode_to_vec(&profile).expect("encode PetProfile second");
        prop_assert_eq!(bytes_a, bytes_b);
    });
}

// 18. Training session consumed bytes equals total length
#[test]
fn test_training_session_consumed_bytes() {
    proptest!(|(session in arb_training_session())| {
        let bytes = encode_to_vec(&session).expect("encode TrainingSessionProgress");
        let (_, consumed): (TrainingSessionProgress, usize) =
            decode_from_slice(&bytes).expect("decode TrainingSessionProgress");
        prop_assert_eq!(consumed, bytes.len());
    });
}

// 19. Insurance claim double roundtrip
#[test]
fn test_insurance_claim_double_roundtrip() {
    proptest!(|(claim in arb_insurance_claim())| {
        let bytes1 = encode_to_vec(&claim).expect("encode InsuranceClaim pass 1");
        let (mid, _): (InsuranceClaim, usize) =
            decode_from_slice(&bytes1).expect("decode InsuranceClaim pass 1");
        let bytes2 = encode_to_vec(&mid).expect("encode InsuranceClaim pass 2");
        let (final_val, _): (InsuranceClaim, usize) =
            decode_from_slice(&bytes2).expect("decode InsuranceClaim pass 2");
        prop_assert_eq!(claim, final_val);
        prop_assert_eq!(bytes1, bytes2);
    });
}

// 20. Behavioral assessment encoded size is non-zero
#[test]
fn test_behavioral_assessment_nonzero_size() {
    proptest!(|(assessment in arb_behavioral_assessment())| {
        let bytes = encode_to_vec(&assessment).expect("encode BehavioralAssessment");
        prop_assert!(bytes.len() > 0);
    });
}

// 21. Vaccination reminder clone encodes identically
#[test]
fn test_vaccination_reminder_clone_identity() {
    proptest!(|(reminder in arb_vaccination_reminder())| {
        let cloned = reminder.clone();
        let bytes_orig = encode_to_vec(&reminder).expect("encode original");
        let bytes_clone = encode_to_vec(&cloned).expect("encode clone");
        prop_assert_eq!(bytes_orig, bytes_clone);
    });
}

// 22. Loyalty rewards double roundtrip preserves bytes
#[test]
fn test_loyalty_rewards_double_roundtrip() {
    proptest!(|(rewards in arb_loyalty_rewards())| {
        let bytes1 = encode_to_vec(&rewards).expect("encode LoyaltyRewards pass 1");
        let (mid, consumed1): (LoyaltyRewards, usize) =
            decode_from_slice(&bytes1).expect("decode LoyaltyRewards pass 1");
        prop_assert_eq!(consumed1, bytes1.len());
        let bytes2 = encode_to_vec(&mid).expect("encode LoyaltyRewards pass 2");
        let (final_val, consumed2): (LoyaltyRewards, usize) =
            decode_from_slice(&bytes2).expect("decode LoyaltyRewards pass 2");
        prop_assert_eq!(consumed2, bytes2.len());
        prop_assert_eq!(rewards, final_val);
        prop_assert_eq!(bytes1, bytes2);
    });
}
