#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types: Telecom Billing ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CallDirection {
    Originating,
    Terminating,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CallType {
    VoiceLocal,
    VoiceLongDistance,
    VoiceInternational,
    VoiceTollFree,
    VideoCall,
    ConferenceCall,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CallDetailRecord {
    cdr_id: u64,
    calling_number: String,
    called_number: String,
    direction: CallDirection,
    call_type: CallType,
    start_epoch_ms: u64,
    duration_seconds: u32,
    answered: bool,
    release_cause_code: u16,
    trunk_group_in: String,
    trunk_group_out: String,
    switch_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MessageType {
    SmsOutbound,
    SmsInbound,
    MmsOutbound,
    MmsInbound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SmsMmsRecord {
    record_id: u64,
    subscriber_msisdn: String,
    remote_party: String,
    message_type: MessageType,
    timestamp_epoch_ms: u64,
    segment_count: u8,
    payload_size_bytes: u32,
    delivery_confirmed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DataSessionType {
    Lte4g,
    Nr5g,
    Wifi,
    Roaming,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DataUsageRecord {
    session_id: u64,
    imsi: String,
    apn: String,
    session_type: DataSessionType,
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    bytes_uplink: u64,
    bytes_downlink: u64,
    rat_type: String,
    cell_id: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RoamingCharge {
    charge_id: u64,
    subscriber_msisdn: String,
    home_plmn: String,
    visited_plmn: String,
    visited_country_iso: String,
    voice_minutes_used: u32,
    data_mb_used: u32,
    sms_count: u16,
    voice_rate_per_min_micros: u64,
    data_rate_per_mb_micros: u64,
    sms_rate_micros: u64,
    total_charge_micros: u64,
    currency_code: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PlanType {
    Prepaid,
    Postpaid,
    Hybrid,
    Enterprise,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RatePlan {
    plan_id: u32,
    plan_name: String,
    plan_type: PlanType,
    monthly_fee_micros: u64,
    included_voice_minutes: u32,
    included_sms: u32,
    included_data_mb: u64,
    overage_voice_per_min_micros: u64,
    overage_data_per_mb_micros: u64,
    overage_sms_micros: u64,
    international_included: bool,
    roaming_included: bool,
    contract_months: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SubscriberStatus {
    Active,
    Suspended,
    Terminated,
    PortingOut,
    PortingIn,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SubscriberProfile {
    subscriber_id: u64,
    msisdn: String,
    imsi: String,
    iccid: String,
    status: SubscriberStatus,
    plan_id: u32,
    activation_epoch_ms: u64,
    credit_limit_micros: u64,
    language_pref: String,
    notification_email: Option<String>,
    auto_topup_enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum LineItemCategory {
    MonthlyRecurring,
    UsageVoice,
    UsageData,
    UsageSms,
    Roaming,
    Equipment,
    Tax,
    Discount,
    OneTimeFee,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InvoiceLineItem {
    line_id: u32,
    invoice_id: u64,
    category: LineItemCategory,
    description: String,
    quantity: u32,
    unit_price_micros: i64,
    total_micros: i64,
    tax_applicable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TaxJurisdiction {
    Federal,
    State,
    Municipal,
    Universal,
    Regulatory,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TaxLineEntry {
    tax_id: u32,
    jurisdiction: TaxJurisdiction,
    jurisdiction_name: String,
    rate_bps: u32,
    taxable_amount_micros: i64,
    tax_amount_micros: i64,
    exempt: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DiscountType {
    PercentageOff,
    FixedAmount,
    BundleDiscount,
    LoyaltyCredit,
    ReferralBonus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PromotionalDiscount {
    promo_code: String,
    discount_type: DiscountType,
    description: String,
    value_micros: u64,
    percentage_bps: u32,
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    recurring: bool,
    max_applications: Option<u32>,
    applied_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PortingDirection {
    PortIn,
    PortOut,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PortingStatus {
    Requested,
    Confirmed,
    InProgress,
    Completed,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NumberPortabilityRecord {
    porting_id: u64,
    msisdn: String,
    direction: PortingDirection,
    donor_carrier: String,
    recipient_carrier: String,
    status: PortingStatus,
    requested_epoch_ms: u64,
    completed_epoch_ms: Option<u64>,
    lrn: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InterconnectFee {
    fee_id: u64,
    originating_carrier: String,
    terminating_carrier: String,
    traffic_type: String,
    rate_per_minute_micros: u64,
    minutes_billed: u32,
    total_fee_micros: u64,
    settlement_period: String,
    bilateral_agreement_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum QosTier {
    BestEffort,
    Standard,
    Premium,
    Critical,
    EmergencyServices,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QosProfile {
    profile_id: u32,
    tier: QosTier,
    max_downlink_kbps: u64,
    max_uplink_kbps: u64,
    latency_budget_ms: u32,
    packet_loss_rate_bps: u32,
    priority_level: u8,
    preemption_capable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PrepaidBalance {
    subscriber_id: u64,
    msisdn: String,
    main_balance_micros: i64,
    bonus_balance_micros: i64,
    data_balance_bytes: u64,
    voice_balance_seconds: u32,
    sms_balance: u32,
    last_recharge_epoch_ms: u64,
    expiry_epoch_ms: u64,
    currency_code: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ThresholdAction {
    SendSms,
    SendEmail,
    Throttle,
    Suspend,
    AutoTopup,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UsageThresholdAlert {
    alert_id: u64,
    subscriber_id: u64,
    threshold_type: String,
    threshold_pct: u8,
    action: ThresholdAction,
    triggered: bool,
    triggered_epoch_ms: Option<u64>,
    notification_sent: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BillingCycleMetadata {
    cycle_id: u64,
    subscriber_id: u64,
    cycle_start_epoch_ms: u64,
    cycle_end_epoch_ms: u64,
    statement_date_epoch_ms: u64,
    due_date_epoch_ms: u64,
    total_charges_micros: i64,
    total_credits_micros: i64,
    total_taxes_micros: i64,
    amount_due_micros: i64,
    payment_received: bool,
    currency_code: String,
}

// --- Tests ---

#[test]
fn test_cdr_voice_local_roundtrip() {
    let cfg = config::standard();
    let cdr = CallDetailRecord {
        cdr_id: 9_000_000_001,
        calling_number: "+14155551234".to_string(),
        called_number: "+14155559876".to_string(),
        direction: CallDirection::Originating,
        call_type: CallType::VoiceLocal,
        start_epoch_ms: 1_710_000_000_000,
        duration_seconds: 347,
        answered: true,
        release_cause_code: 16,
        trunk_group_in: "TG-PSTN-01".to_string(),
        trunk_group_out: "TG-LOCAL-05".to_string(),
        switch_id: "SW-SFO-A3".to_string(),
    };
    let bytes = encode_to_vec(&cdr, cfg).expect("encode CDR voice local");
    let (decoded, _): (CallDetailRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CDR voice local");
    assert_eq!(cdr, decoded);
}

#[test]
fn test_cdr_international_unanswered_roundtrip() {
    let cfg = config::standard();
    let cdr = CallDetailRecord {
        cdr_id: 9_000_000_002,
        calling_number: "+442071234567".to_string(),
        called_number: "+81345678901".to_string(),
        direction: CallDirection::Originating,
        call_type: CallType::VoiceInternational,
        start_epoch_ms: 1_710_000_120_000,
        duration_seconds: 0,
        answered: false,
        release_cause_code: 17,
        trunk_group_in: "TG-INT-UK-02".to_string(),
        trunk_group_out: "TG-INT-JP-01".to_string(),
        switch_id: "SW-LDN-B1".to_string(),
    };
    let bytes = encode_to_vec(&cdr, cfg).expect("encode CDR intl unanswered");
    let (decoded, _): (CallDetailRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CDR intl unanswered");
    assert_eq!(cdr, decoded);
}

#[test]
fn test_sms_mms_records_roundtrip() {
    let cfg = config::standard();
    let sms = SmsMmsRecord {
        record_id: 50_000_001,
        subscriber_msisdn: "+12125551000".to_string(),
        remote_party: "+12125552000".to_string(),
        message_type: MessageType::SmsOutbound,
        timestamp_epoch_ms: 1_710_001_000_000,
        segment_count: 1,
        payload_size_bytes: 142,
        delivery_confirmed: true,
    };
    let mms = SmsMmsRecord {
        record_id: 50_000_002,
        subscriber_msisdn: "+12125551000".to_string(),
        remote_party: "+447911123456".to_string(),
        message_type: MessageType::MmsOutbound,
        timestamp_epoch_ms: 1_710_001_060_000,
        segment_count: 3,
        payload_size_bytes: 1_245_000,
        delivery_confirmed: false,
    };
    let pair = (sms.clone(), mms.clone());
    let bytes = encode_to_vec(&pair, cfg).expect("encode SMS/MMS pair");
    let (decoded, _): ((SmsMmsRecord, SmsMmsRecord), _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SMS/MMS pair");
    assert_eq!(pair, decoded);
}

#[test]
fn test_data_usage_5g_session_roundtrip() {
    let cfg = config::standard();
    let record = DataUsageRecord {
        session_id: 7_700_000_001,
        imsi: "310260000000001".to_string(),
        apn: "fast.t-mobile.com".to_string(),
        session_type: DataSessionType::Nr5g,
        start_epoch_ms: 1_710_010_000_000,
        end_epoch_ms: 1_710_010_900_000,
        bytes_uplink: 52_428_800,
        bytes_downlink: 524_288_000,
        rat_type: "NR".to_string(),
        cell_id: 1_234_567,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode data usage 5G");
    let (decoded, _): (DataUsageRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode data usage 5G");
    assert_eq!(record, decoded);
}

#[test]
fn test_data_usage_roaming_with_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let record = DataUsageRecord {
        session_id: 7_700_000_002,
        imsi: "234150999888777".to_string(),
        apn: "roaming.partner.net".to_string(),
        session_type: DataSessionType::Roaming,
        start_epoch_ms: 1_710_020_000_000,
        end_epoch_ms: 1_710_020_600_000,
        bytes_uplink: 1_048_576,
        bytes_downlink: 10_485_760,
        rat_type: "LTE".to_string(),
        cell_id: 9_876_543,
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode data usage roaming fixed-int");
    let (decoded, _): (DataUsageRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode data usage roaming fixed-int");
    assert_eq!(record, decoded);
}

#[test]
fn test_roaming_charge_multi_service_roundtrip() {
    let cfg = config::standard();
    let charge = RoamingCharge {
        charge_id: 88_000_001,
        subscriber_msisdn: "+33612345678".to_string(),
        home_plmn: "20801".to_string(),
        visited_plmn: "31026".to_string(),
        visited_country_iso: "US".to_string(),
        voice_minutes_used: 45,
        data_mb_used: 320,
        sms_count: 12,
        voice_rate_per_min_micros: 350_000,
        data_rate_per_mb_micros: 120_000,
        sms_rate_micros: 50_000,
        total_charge_micros: 45 * 350_000 + 320 * 120_000 + 12 * 50_000,
        currency_code: "EUR".to_string(),
    };
    let bytes = encode_to_vec(&charge, cfg).expect("encode roaming charge");
    let (decoded, _): (RoamingCharge, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode roaming charge");
    assert_eq!(charge, decoded);
}

#[test]
fn test_rate_plan_postpaid_roundtrip() {
    let cfg = config::standard();
    let plan = RatePlan {
        plan_id: 1001,
        plan_name: "Unlimited Plus".to_string(),
        plan_type: PlanType::Postpaid,
        monthly_fee_micros: 75_000_000,
        included_voice_minutes: u32::MAX,
        included_sms: u32::MAX,
        included_data_mb: 50_000,
        overage_voice_per_min_micros: 0,
        overage_data_per_mb_micros: 15_000,
        overage_sms_micros: 0,
        international_included: false,
        roaming_included: false,
        contract_months: 24,
    };
    let bytes = encode_to_vec(&plan, cfg).expect("encode rate plan postpaid");
    let (decoded, _): (RatePlan, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode rate plan postpaid");
    assert_eq!(plan, decoded);
}

#[test]
fn test_rate_plan_prepaid_with_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let plan = RatePlan {
        plan_id: 2050,
        plan_name: "Pay As You Go Basic".to_string(),
        plan_type: PlanType::Prepaid,
        monthly_fee_micros: 0,
        included_voice_minutes: 0,
        included_sms: 0,
        included_data_mb: 0,
        overage_voice_per_min_micros: 100_000,
        overage_data_per_mb_micros: 50_000,
        overage_sms_micros: 10_000,
        international_included: false,
        roaming_included: false,
        contract_months: 0,
    };
    let bytes = encode_to_vec(&plan, cfg).expect("encode rate plan prepaid big-endian");
    let (decoded, _): (RatePlan, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode rate plan prepaid big-endian");
    assert_eq!(plan, decoded);
}

#[test]
fn test_subscriber_profile_active_roundtrip() {
    let cfg = config::standard();
    let profile = SubscriberProfile {
        subscriber_id: 100_000_001,
        msisdn: "+19175550123".to_string(),
        imsi: "310260100000001".to_string(),
        iccid: "8901260012345678901".to_string(),
        status: SubscriberStatus::Active,
        plan_id: 1001,
        activation_epoch_ms: 1_640_000_000_000,
        credit_limit_micros: 500_000_000,
        language_pref: "en-US".to_string(),
        notification_email: Some("user@example.com".to_string()),
        auto_topup_enabled: true,
    };
    let bytes = encode_to_vec(&profile, cfg).expect("encode subscriber active");
    let (decoded, _): (SubscriberProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode subscriber active");
    assert_eq!(profile, decoded);
}

#[test]
fn test_subscriber_profile_porting_out_no_email_roundtrip() {
    let cfg = config::standard();
    let profile = SubscriberProfile {
        subscriber_id: 100_000_002,
        msisdn: "+14085559999".to_string(),
        imsi: "310260100000002".to_string(),
        iccid: "8901260012345678902".to_string(),
        status: SubscriberStatus::PortingOut,
        plan_id: 2050,
        activation_epoch_ms: 1_600_000_000_000,
        credit_limit_micros: 0,
        language_pref: "es".to_string(),
        notification_email: None,
        auto_topup_enabled: false,
    };
    let bytes = encode_to_vec(&profile, cfg).expect("encode subscriber porting-out");
    let (decoded, _): (SubscriberProfile, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode subscriber porting-out");
    assert_eq!(profile, decoded);
}

#[test]
fn test_invoice_line_items_vec_roundtrip() {
    let cfg = config::standard();
    let items = vec![
        InvoiceLineItem {
            line_id: 1,
            invoice_id: 900_001,
            category: LineItemCategory::MonthlyRecurring,
            description: "Unlimited Plus Monthly Fee".to_string(),
            quantity: 1,
            unit_price_micros: 75_000_000,
            total_micros: 75_000_000,
            tax_applicable: true,
        },
        InvoiceLineItem {
            line_id: 2,
            invoice_id: 900_001,
            category: LineItemCategory::UsageData,
            description: "Data Overage - 2.5 GB".to_string(),
            quantity: 2560,
            unit_price_micros: 15_000,
            total_micros: 2560 * 15_000,
            tax_applicable: true,
        },
        InvoiceLineItem {
            line_id: 3,
            invoice_id: 900_001,
            category: LineItemCategory::Discount,
            description: "Loyalty Discount 10%".to_string(),
            quantity: 1,
            unit_price_micros: -7_500_000,
            total_micros: -7_500_000,
            tax_applicable: false,
        },
    ];
    let bytes = encode_to_vec(&items, cfg).expect("encode invoice line items");
    let (decoded, _): (Vec<InvoiceLineItem>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode invoice line items");
    assert_eq!(items, decoded);
}

#[test]
fn test_tax_calculation_multi_jurisdiction_roundtrip() {
    let cfg = config::standard();
    let taxes = vec![
        TaxLineEntry {
            tax_id: 1,
            jurisdiction: TaxJurisdiction::Federal,
            jurisdiction_name: "Federal USF".to_string(),
            rate_bps: 331,
            taxable_amount_micros: 75_000_000,
            tax_amount_micros: 2_482_500,
            exempt: false,
        },
        TaxLineEntry {
            tax_id: 2,
            jurisdiction: TaxJurisdiction::State,
            jurisdiction_name: "California PUC".to_string(),
            rate_bps: 550,
            taxable_amount_micros: 75_000_000,
            tax_amount_micros: 4_125_000,
            exempt: false,
        },
        TaxLineEntry {
            tax_id: 3,
            jurisdiction: TaxJurisdiction::Municipal,
            jurisdiction_name: "San Francisco Telecom Tax".to_string(),
            rate_bps: 750,
            taxable_amount_micros: 75_000_000,
            tax_amount_micros: 5_625_000,
            exempt: false,
        },
        TaxLineEntry {
            tax_id: 4,
            jurisdiction: TaxJurisdiction::Regulatory,
            jurisdiction_name: "E911 Fee".to_string(),
            rate_bps: 0,
            taxable_amount_micros: 0,
            tax_amount_micros: 1_250_000,
            exempt: false,
        },
    ];
    let bytes = encode_to_vec(&taxes, cfg).expect("encode tax entries");
    let (decoded, _): (Vec<TaxLineEntry>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode tax entries");
    assert_eq!(taxes, decoded);
}

#[test]
fn test_promotional_discount_percentage_roundtrip() {
    let cfg = config::standard();
    let promo = PromotionalDiscount {
        promo_code: "SWITCH2024".to_string(),
        discount_type: DiscountType::PercentageOff,
        description: "New customer 20% off first 6 months".to_string(),
        value_micros: 0,
        percentage_bps: 2000,
        start_epoch_ms: 1_704_067_200_000,
        end_epoch_ms: 1_719_792_000_000,
        recurring: true,
        max_applications: Some(6),
        applied_count: 3,
    };
    let bytes = encode_to_vec(&promo, cfg).expect("encode promo percentage");
    let (decoded, _): (PromotionalDiscount, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode promo percentage");
    assert_eq!(promo, decoded);
}

#[test]
fn test_promotional_discount_referral_unlimited_roundtrip() {
    let cfg = config::standard();
    let promo = PromotionalDiscount {
        promo_code: "REFER-A-FRIEND".to_string(),
        discount_type: DiscountType::ReferralBonus,
        description: "Referral credit per successful activation".to_string(),
        value_micros: 25_000_000,
        percentage_bps: 0,
        start_epoch_ms: 1_700_000_000_000,
        end_epoch_ms: u64::MAX,
        recurring: false,
        max_applications: None,
        applied_count: 0,
    };
    let bytes = encode_to_vec(&promo, cfg).expect("encode promo referral");
    let (decoded, _): (PromotionalDiscount, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode promo referral");
    assert_eq!(promo, decoded);
}

#[test]
fn test_number_portability_completed_roundtrip() {
    let cfg = config::standard();
    let record = NumberPortabilityRecord {
        porting_id: 330_000_001,
        msisdn: "+18005551234".to_string(),
        direction: PortingDirection::PortIn,
        donor_carrier: "Carrier-A".to_string(),
        recipient_carrier: "Carrier-B".to_string(),
        status: PortingStatus::Completed,
        requested_epoch_ms: 1_709_000_000_000,
        completed_epoch_ms: Some(1_709_259_200_000),
        lrn: "2125559000".to_string(),
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode porting completed");
    let (decoded, _): (NumberPortabilityRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode porting completed");
    assert_eq!(record, decoded);
}

#[test]
fn test_number_portability_rejected_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let record = NumberPortabilityRecord {
        porting_id: 330_000_002,
        msisdn: "+12025559876".to_string(),
        direction: PortingDirection::PortOut,
        donor_carrier: "Carrier-B".to_string(),
        recipient_carrier: "Carrier-C".to_string(),
        status: PortingStatus::Rejected,
        requested_epoch_ms: 1_709_100_000_000,
        completed_epoch_ms: None,
        lrn: "2025550000".to_string(),
    };
    let bytes = encode_to_vec(&record, cfg).expect("encode porting rejected fixed-int");
    let (decoded, _): (NumberPortabilityRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode porting rejected fixed-int");
    assert_eq!(record, decoded);
}

#[test]
fn test_interconnect_fee_settlement_roundtrip() {
    let cfg = config::standard();
    let fee = InterconnectFee {
        fee_id: 440_000_001,
        originating_carrier: "MNO-Alpha".to_string(),
        terminating_carrier: "MNO-Beta".to_string(),
        traffic_type: "voice-local".to_string(),
        rate_per_minute_micros: 800,
        minutes_billed: 125_000,
        total_fee_micros: 125_000 * 800,
        settlement_period: "2024-Q1".to_string(),
        bilateral_agreement_id: "BA-2023-0042".to_string(),
    };
    let bytes = encode_to_vec(&fee, cfg).expect("encode interconnect fee");
    let (decoded, _): (InterconnectFee, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode interconnect fee");
    assert_eq!(fee, decoded);
}

#[test]
fn test_qos_profiles_multiple_tiers_roundtrip() {
    let cfg = config::standard();
    let profiles = vec![
        QosProfile {
            profile_id: 1,
            tier: QosTier::BestEffort,
            max_downlink_kbps: 10_000,
            max_uplink_kbps: 5_000,
            latency_budget_ms: 300,
            packet_loss_rate_bps: 100,
            priority_level: 9,
            preemption_capable: false,
        },
        QosProfile {
            profile_id: 2,
            tier: QosTier::Premium,
            max_downlink_kbps: 1_000_000,
            max_uplink_kbps: 500_000,
            latency_budget_ms: 20,
            packet_loss_rate_bps: 1,
            priority_level: 2,
            preemption_capable: true,
        },
        QosProfile {
            profile_id: 3,
            tier: QosTier::EmergencyServices,
            max_downlink_kbps: u64::MAX,
            max_uplink_kbps: u64::MAX,
            latency_budget_ms: 5,
            packet_loss_rate_bps: 0,
            priority_level: 1,
            preemption_capable: true,
        },
    ];
    let bytes = encode_to_vec(&profiles, cfg).expect("encode QoS profiles");
    let (decoded, _): (Vec<QosProfile>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode QoS profiles");
    assert_eq!(profiles, decoded);
}

#[test]
fn test_prepaid_balance_tracking_roundtrip() {
    let cfg = config::standard();
    let balance = PrepaidBalance {
        subscriber_id: 200_000_001,
        msisdn: "+2348031234567".to_string(),
        main_balance_micros: 1_250_000,
        bonus_balance_micros: 500_000,
        data_balance_bytes: 536_870_912,
        voice_balance_seconds: 1800,
        sms_balance: 50,
        last_recharge_epoch_ms: 1_709_500_000_000,
        expiry_epoch_ms: 1_712_178_000_000,
        currency_code: "NGN".to_string(),
    };
    let bytes = encode_to_vec(&balance, cfg).expect("encode prepaid balance");
    let (decoded, _): (PrepaidBalance, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode prepaid balance");
    assert_eq!(balance, decoded);
}

#[test]
fn test_usage_threshold_alerts_roundtrip() {
    let cfg = config::standard();
    let alerts = vec![
        UsageThresholdAlert {
            alert_id: 60_001,
            subscriber_id: 100_000_001,
            threshold_type: "data".to_string(),
            threshold_pct: 80,
            action: ThresholdAction::SendSms,
            triggered: true,
            triggered_epoch_ms: Some(1_710_100_000_000),
            notification_sent: true,
        },
        UsageThresholdAlert {
            alert_id: 60_002,
            subscriber_id: 100_000_001,
            threshold_type: "data".to_string(),
            threshold_pct: 100,
            action: ThresholdAction::Throttle,
            triggered: false,
            triggered_epoch_ms: None,
            notification_sent: false,
        },
        UsageThresholdAlert {
            alert_id: 60_003,
            subscriber_id: 200_000_001,
            threshold_type: "voice".to_string(),
            threshold_pct: 90,
            action: ThresholdAction::AutoTopup,
            triggered: true,
            triggered_epoch_ms: Some(1_710_200_000_000),
            notification_sent: true,
        },
    ];
    let bytes = encode_to_vec(&alerts, cfg).expect("encode usage threshold alerts");
    let (decoded, _): (Vec<UsageThresholdAlert>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode usage threshold alerts");
    assert_eq!(alerts, decoded);
}

#[test]
fn test_billing_cycle_metadata_roundtrip() {
    let cfg = config::standard();
    let cycle = BillingCycleMetadata {
        cycle_id: 12_000_001,
        subscriber_id: 100_000_001,
        cycle_start_epoch_ms: 1_709_251_200_000,
        cycle_end_epoch_ms: 1_711_929_600_000,
        statement_date_epoch_ms: 1_711_929_600_000,
        due_date_epoch_ms: 1_713_139_200_000,
        total_charges_micros: 113_400_000,
        total_credits_micros: -7_500_000,
        total_taxes_micros: 13_482_500,
        amount_due_micros: 119_382_500,
        payment_received: false,
        currency_code: "USD".to_string(),
    };
    let bytes = encode_to_vec(&cycle, cfg).expect("encode billing cycle metadata");
    let (decoded, _): (BillingCycleMetadata, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode billing cycle metadata");
    assert_eq!(cycle, decoded);
}

#[test]
fn test_full_invoice_composition_roundtrip() {
    let cfg = config::standard().with_big_endian();

    let subscriber = SubscriberProfile {
        subscriber_id: 100_000_003,
        msisdn: "+81901234567".to_string(),
        imsi: "440100100000003".to_string(),
        iccid: "8981100012345678903".to_string(),
        status: SubscriberStatus::Active,
        plan_id: 3000,
        activation_epoch_ms: 1_680_000_000_000,
        credit_limit_micros: 300_000_000,
        language_pref: "ja".to_string(),
        notification_email: Some("user@example.jp".to_string()),
        auto_topup_enabled: false,
    };

    let plan = RatePlan {
        plan_id: 3000,
        plan_name: "Docomo Ahamo".to_string(),
        plan_type: PlanType::Postpaid,
        monthly_fee_micros: 2_970_000_000,
        included_voice_minutes: u32::MAX,
        included_sms: u32::MAX,
        included_data_mb: 20_000,
        overage_voice_per_min_micros: 0,
        overage_data_per_mb_micros: 550_000,
        overage_sms_micros: 3_300,
        international_included: false,
        roaming_included: true,
        contract_months: 0,
    };

    let cycle = BillingCycleMetadata {
        cycle_id: 12_100_001,
        subscriber_id: 100_000_003,
        cycle_start_epoch_ms: 1_709_251_200_000,
        cycle_end_epoch_ms: 1_711_929_600_000,
        statement_date_epoch_ms: 1_711_929_600_000,
        due_date_epoch_ms: 1_713_139_200_000,
        total_charges_micros: 2_970_000_000,
        total_credits_micros: 0,
        total_taxes_micros: 297_000_000,
        amount_due_micros: 3_267_000_000,
        payment_received: true,
        currency_code: "JPY".to_string(),
    };

    let composite = (subscriber.clone(), plan.clone(), cycle.clone());
    let bytes = encode_to_vec(&composite, cfg).expect("encode full invoice composition");
    let (decoded, _): ((SubscriberProfile, RatePlan, BillingCycleMetadata), _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode full invoice composition");
    assert_eq!(composite, decoded);
}
