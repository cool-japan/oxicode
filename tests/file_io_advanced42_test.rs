//! Advanced file I/O tests for OxiCode — domain: e-commerce payment processing and fraud prevention

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types — Payment Methods
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PaymentMethod {
    CreditCard,
    DebitCard,
    Ach,
    Wire,
    Crypto,
    DigitalWallet,
    BuyNowPayLater,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CardNetwork {
    Visa,
    Mastercard,
    Amex,
    Discover,
    UnionPay,
    Jcb,
    DinersClub,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransactionStatus {
    Pending,
    Authorized,
    Captured,
    Settled,
    Declined,
    Voided,
    Refunded,
    PartialRefund,
    Chargeback,
    Disputed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CurrencyCode {
    Usd,
    Eur,
    Gbp,
    Jpy,
    Cny,
    Krw,
    Inr,
    Brl,
    Aud,
    Cad,
    Chf,
    Sgd,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThreeDsStatus {
    NotEnrolled,
    ChallengeRequired,
    ChallengeCompleted,
    Authenticated,
    AuthenticationFailed,
    AttemptedAuthentication,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargebackReason {
    FraudulentTransaction,
    ProductNotReceived,
    ProductNotAsDescribed,
    DuplicateCharge,
    SubscriptionCancelled,
    CreditNotProcessed,
    Unauthorized,
    ProcessingError,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DisputeStage {
    FirstChargeback,
    PreArbitration,
    Arbitration,
    ComplianceFiling,
    Resolved,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FraudRiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MerchantCategory {
    RetailGeneral,
    GroceryStore,
    Restaurant,
    Hotel,
    Airline,
    CarRental,
    GasStation,
    DigitalGoods,
    GamingOnline,
    Subscription,
    Marketplace,
    Pharmacy,
    FinancialService,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum KycAmlStatus {
    NotScreened,
    Pending,
    Cleared,
    FlaggedForReview,
    SanctionsHit,
    PepMatch,
    AdverseMedia,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GatewayProvider {
    Primary,
    SecondaryFallback,
    RegionalProcessor,
    CryptoProcessor,
    HighRiskProcessor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SubscriptionStatus {
    Active,
    PastDue,
    Paused,
    Cancelled,
    Expired,
    TrialPeriod,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RefundReason {
    CustomerRequest,
    MerchantError,
    FraudConfirmed,
    DuplicateTransaction,
    ServiceNotProvided,
    PolicyViolation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PciComplianceLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AbandonmentStage {
    BrowsingProducts,
    AddedToCart,
    StartedCheckout,
    EnteredShipping,
    EnteredPayment,
    ReviewOrder,
}

// ---------------------------------------------------------------------------
// Domain types — Structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PaymentTransaction {
    transaction_id: u64,
    merchant_id: u32,
    customer_id: u64,
    method: PaymentMethod,
    status: TransactionStatus,
    amount_minor_units: u64,
    currency: CurrencyCode,
    authorization_code: String,
    created_at_unix: u64,
    settled_at_unix: u64,
    gateway: GatewayProvider,
    is_recurring: bool,
    metadata_tags: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TokenizedCard {
    token_id: String,
    card_network: CardNetwork,
    last_four_digits: u16,
    expiry_month: u8,
    expiry_year: u16,
    cardholder_name_hash: Vec<u8>,
    bin_number: u32,
    issuing_country_code: u16,
    is_corporate: bool,
    token_created_unix: u64,
    token_last_used_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThreeDsAuthentication {
    authentication_id: u64,
    transaction_id: u64,
    protocol_version: String,
    status: ThreeDsStatus,
    eci_indicator: u8,
    cavv: String,
    ds_transaction_id: String,
    challenge_requested: bool,
    challenge_completed: bool,
    authentication_timestamp: u64,
    risk_score_from_issuer: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChargebackDispute {
    dispute_id: u64,
    transaction_id: u64,
    reason: ChargebackReason,
    stage: DisputeStage,
    amount_minor_units: u64,
    currency: CurrencyCode,
    filed_date_unix: u64,
    response_due_date_unix: u64,
    merchant_responded: bool,
    evidence_submitted: Vec<String>,
    outcome_favorable: bool,
    representment_amount_minor: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MerchantProfile {
    merchant_id: u32,
    merchant_name: String,
    category: MerchantCategory,
    mcc_code: u16,
    pci_level: PciComplianceLevel,
    country_code: u16,
    monthly_volume_limit_minor: u64,
    chargeback_rate_bps: u16,
    kyc_status: KycAmlStatus,
    onboarded_date_unix: u64,
    risk_tier: FraudRiskLevel,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PciAuditResult {
    audit_id: u64,
    merchant_id: u32,
    compliance_level: PciComplianceLevel,
    audit_date_unix: u64,
    next_audit_due_unix: u64,
    passed: bool,
    findings_count: u16,
    critical_findings: u16,
    vulnerability_scan_passed: bool,
    penetration_test_passed: bool,
    encryption_at_rest_verified: bool,
    network_segmentation_verified: bool,
    auditor_name: String,
    remediation_items: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RecurringSubscription {
    subscription_id: u64,
    customer_id: u64,
    merchant_id: u32,
    status: SubscriptionStatus,
    plan_name: String,
    amount_minor_units: u64,
    currency: CurrencyCode,
    billing_interval_days: u16,
    next_billing_date_unix: u64,
    total_cycles_completed: u32,
    failed_payment_count: u8,
    token_id: String,
    created_at_unix: u64,
    cancelled_at_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MultiCurrencyConversion {
    conversion_id: u64,
    source_currency: CurrencyCode,
    target_currency: CurrencyCode,
    source_amount_minor: u64,
    target_amount_minor: u64,
    exchange_rate_x1e6: u64,
    markup_bps: u16,
    conversion_timestamp: u64,
    rate_source: String,
    settlement_date_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FraudScoreResult {
    scoring_id: u64,
    transaction_id: u64,
    overall_score: u16,
    risk_level: FraudRiskLevel,
    velocity_check_score: u16,
    device_fingerprint_score: u16,
    geo_anomaly_score: u16,
    behavioral_score: u16,
    card_testing_score: u16,
    ip_address_hash: Vec<u8>,
    device_id_hash: Vec<u8>,
    rules_triggered: Vec<String>,
    manual_review_required: bool,
    scored_at_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VelocityCheckResult {
    check_id: u64,
    customer_id: u64,
    card_token: String,
    txn_count_last_1h: u16,
    txn_count_last_24h: u16,
    txn_count_last_7d: u32,
    total_amount_last_1h_minor: u64,
    total_amount_last_24h_minor: u64,
    distinct_merchants_24h: u16,
    distinct_countries_24h: u8,
    velocity_exceeded: bool,
    threshold_txn_per_hour: u16,
    threshold_amount_per_hour_minor: u64,
    checked_at_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeviceFingerprint {
    fingerprint_id: String,
    customer_id: u64,
    browser_hash: Vec<u8>,
    os_family: String,
    screen_resolution_w: u16,
    screen_resolution_h: u16,
    timezone_offset_min: i16,
    language_code: String,
    plugins_hash: Vec<u8>,
    canvas_hash: Vec<u8>,
    webgl_hash: Vec<u8>,
    first_seen_unix: u64,
    last_seen_unix: u64,
    trust_score: u16,
    anomaly_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SettlementBatch {
    batch_id: u64,
    merchant_id: u32,
    gateway: GatewayProvider,
    currency: CurrencyCode,
    transaction_count: u32,
    gross_amount_minor: u64,
    fees_amount_minor: u64,
    net_amount_minor: u64,
    chargebacks_minor: u64,
    refunds_minor: u64,
    batch_opened_unix: u64,
    batch_closed_unix: u64,
    settled: bool,
    settlement_reference: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RefundRecord {
    refund_id: u64,
    original_transaction_id: u64,
    reason: RefundReason,
    amount_minor_units: u64,
    currency: CurrencyCode,
    partial_refund: bool,
    initiated_by_merchant: bool,
    refund_status: TransactionStatus,
    created_at_unix: u64,
    processed_at_unix: u64,
    arn: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GatewayRoutingDecision {
    decision_id: u64,
    transaction_id: u64,
    selected_gateway: GatewayProvider,
    fallback_gateway: GatewayProvider,
    routing_reason: String,
    latency_ms: u16,
    primary_available: bool,
    cost_bps: u16,
    success_rate_bps: u16,
    decided_at_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InterchangeFeeCalc {
    calc_id: u64,
    transaction_id: u64,
    card_network: CardNetwork,
    merchant_category: MerchantCategory,
    interchange_rate_bps: u16,
    assessment_fee_bps: u16,
    acquirer_markup_bps: u16,
    total_fee_minor_units: u64,
    transaction_amount_minor: u64,
    is_debit_regulated: bool,
    cross_border: bool,
    downgrade_applied: bool,
    qualification_tier: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KycAmlScreeningResult {
    screening_id: u64,
    entity_name: String,
    entity_type: String,
    status: KycAmlStatus,
    sanctions_lists_checked: Vec<String>,
    pep_databases_checked: u8,
    adverse_media_hits: u16,
    match_score_percent: u8,
    screened_at_unix: u64,
    next_review_date_unix: u64,
    reviewer_id: u32,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CartAbandonmentEvent {
    event_id: u64,
    session_id: String,
    customer_id: u64,
    stage: AbandonmentStage,
    cart_value_minor: u64,
    currency: CurrencyCode,
    item_count: u16,
    time_spent_seconds: u32,
    device_type: String,
    referral_source: String,
    abandoned_at_unix: u64,
    recovery_email_sent: bool,
    recovered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PaymentLedgerEntry {
    entry_id: u64,
    merchant_id: u32,
    transaction_id: u64,
    debit_minor: u64,
    credit_minor: u64,
    balance_after_minor: u64,
    currency: CurrencyCode,
    entry_type: String,
    posted_at_unix: u64,
    description: String,
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn sample_payment_transaction(id: u64) -> PaymentTransaction {
    PaymentTransaction {
        transaction_id: id,
        merchant_id: 5001,
        customer_id: 880000 + id,
        method: PaymentMethod::CreditCard,
        status: TransactionStatus::Captured,
        amount_minor_units: 125_99,
        currency: CurrencyCode::Usd,
        authorization_code: format!("AUTH{:06}", id),
        created_at_unix: 1_700_000_000 + id * 60,
        settled_at_unix: 1_700_086_400 + id * 60,
        gateway: GatewayProvider::Primary,
        is_recurring: false,
        metadata_tags: vec!["web".into(), "desktop".into()],
    }
}

fn sample_tokenized_card(suffix: u16) -> TokenizedCard {
    TokenizedCard {
        token_id: format!("tok_live_{}", suffix),
        card_network: CardNetwork::Visa,
        last_four_digits: suffix,
        expiry_month: 12,
        expiry_year: 2028,
        cardholder_name_hash: vec![0xAB, 0xCD, 0xEF, 0x01, 0x23],
        bin_number: 411111,
        issuing_country_code: 840,
        is_corporate: false,
        token_created_unix: 1_690_000_000,
        token_last_used_unix: 1_700_500_000,
    }
}

fn sample_three_ds(id: u64) -> ThreeDsAuthentication {
    ThreeDsAuthentication {
        authentication_id: id,
        transaction_id: id + 10000,
        protocol_version: "2.2.0".into(),
        status: ThreeDsStatus::Authenticated,
        eci_indicator: 5,
        cavv: "AAABBBBCCCCDDDD1234567890".into(),
        ds_transaction_id: format!("ds-txn-{}", id),
        challenge_requested: true,
        challenge_completed: true,
        authentication_timestamp: 1_700_100_000 + id,
        risk_score_from_issuer: 15,
    }
}

fn sample_chargeback(id: u64) -> ChargebackDispute {
    ChargebackDispute {
        dispute_id: id,
        transaction_id: id + 50000,
        reason: ChargebackReason::FraudulentTransaction,
        stage: DisputeStage::FirstChargeback,
        amount_minor_units: 299_99,
        currency: CurrencyCode::Usd,
        filed_date_unix: 1_702_000_000,
        response_due_date_unix: 1_703_000_000,
        merchant_responded: false,
        evidence_submitted: vec![],
        outcome_favorable: false,
        representment_amount_minor: 0,
    }
}

fn sample_merchant(id: u32) -> MerchantProfile {
    MerchantProfile {
        merchant_id: id,
        merchant_name: format!("Merchant_{}", id),
        category: MerchantCategory::RetailGeneral,
        mcc_code: 5411,
        pci_level: PciComplianceLevel::Level1,
        country_code: 840,
        monthly_volume_limit_minor: 10_000_000_00,
        chargeback_rate_bps: 45,
        kyc_status: KycAmlStatus::Cleared,
        onboarded_date_unix: 1_680_000_000,
        risk_tier: FraudRiskLevel::Low,
        active: true,
    }
}

fn sample_pci_audit(id: u64) -> PciAuditResult {
    PciAuditResult {
        audit_id: id,
        merchant_id: 5001,
        compliance_level: PciComplianceLevel::Level1,
        audit_date_unix: 1_700_000_000,
        next_audit_due_unix: 1_731_536_000,
        passed: true,
        findings_count: 3,
        critical_findings: 0,
        vulnerability_scan_passed: true,
        penetration_test_passed: true,
        encryption_at_rest_verified: true,
        network_segmentation_verified: true,
        auditor_name: "SecureScan Ltd.".into(),
        remediation_items: vec!["Update TLS config".into(), "Rotate API keys".into()],
    }
}

fn sample_subscription(id: u64) -> RecurringSubscription {
    RecurringSubscription {
        subscription_id: id,
        customer_id: 900_000 + id,
        merchant_id: 5001,
        status: SubscriptionStatus::Active,
        plan_name: "Premium Monthly".into(),
        amount_minor_units: 19_99,
        currency: CurrencyCode::Usd,
        billing_interval_days: 30,
        next_billing_date_unix: 1_703_000_000,
        total_cycles_completed: 6,
        failed_payment_count: 0,
        token_id: format!("tok_sub_{}", id),
        created_at_unix: 1_685_000_000,
        cancelled_at_unix: 0,
    }
}

fn sample_currency_conversion(id: u64) -> MultiCurrencyConversion {
    MultiCurrencyConversion {
        conversion_id: id,
        source_currency: CurrencyCode::Eur,
        target_currency: CurrencyCode::Usd,
        source_amount_minor: 100_00,
        target_amount_minor: 108_50,
        exchange_rate_x1e6: 1_085_000,
        markup_bps: 150,
        conversion_timestamp: 1_700_200_000 + id,
        rate_source: "ECB reference rate".into(),
        settlement_date_unix: 1_700_300_000 + id,
    }
}

fn sample_fraud_score(id: u64) -> FraudScoreResult {
    FraudScoreResult {
        scoring_id: id,
        transaction_id: id + 20000,
        overall_score: 230,
        risk_level: FraudRiskLevel::Low,
        velocity_check_score: 50,
        device_fingerprint_score: 40,
        geo_anomaly_score: 30,
        behavioral_score: 60,
        card_testing_score: 50,
        ip_address_hash: vec![0x11, 0x22, 0x33, 0x44],
        device_id_hash: vec![0xAA, 0xBB, 0xCC, 0xDD],
        rules_triggered: vec!["RULE_LOW_VALUE".into()],
        manual_review_required: false,
        scored_at_unix: 1_700_100_000 + id,
    }
}

fn sample_velocity_check(id: u64) -> VelocityCheckResult {
    VelocityCheckResult {
        check_id: id,
        customer_id: 880_000 + id,
        card_token: format!("tok_vel_{}", id),
        txn_count_last_1h: 2,
        txn_count_last_24h: 5,
        txn_count_last_7d: 12,
        total_amount_last_1h_minor: 45_00,
        total_amount_last_24h_minor: 230_50,
        distinct_merchants_24h: 3,
        distinct_countries_24h: 1,
        velocity_exceeded: false,
        threshold_txn_per_hour: 10,
        threshold_amount_per_hour_minor: 500_00,
        checked_at_unix: 1_700_300_000 + id,
    }
}

fn sample_device_fingerprint(id: u64) -> DeviceFingerprint {
    DeviceFingerprint {
        fingerprint_id: format!("fp_{:016x}", id),
        customer_id: 880_000 + id,
        browser_hash: vec![0xDE, 0xAD, 0xBE, 0xEF],
        os_family: "macOS".into(),
        screen_resolution_w: 2560,
        screen_resolution_h: 1440,
        timezone_offset_min: -480,
        language_code: "en-US".into(),
        plugins_hash: vec![0x01, 0x02, 0x03],
        canvas_hash: vec![0x10, 0x20, 0x30, 0x40],
        webgl_hash: vec![0xA0, 0xB0, 0xC0],
        first_seen_unix: 1_690_000_000,
        last_seen_unix: 1_700_500_000 + id,
        trust_score: 850,
        anomaly_detected: false,
    }
}

fn sample_settlement_batch(id: u64) -> SettlementBatch {
    SettlementBatch {
        batch_id: id,
        merchant_id: 5001,
        gateway: GatewayProvider::Primary,
        currency: CurrencyCode::Usd,
        transaction_count: 142,
        gross_amount_minor: 1_250_000_00,
        fees_amount_minor: 31_250_00,
        net_amount_minor: 1_218_750_00,
        chargebacks_minor: 599_98,
        refunds_minor: 2_400_00,
        batch_opened_unix: 1_700_000_000,
        batch_closed_unix: 1_700_086_400,
        settled: true,
        settlement_reference: format!("STL-{:08}", id),
    }
}

fn sample_refund(id: u64) -> RefundRecord {
    RefundRecord {
        refund_id: id,
        original_transaction_id: id + 40000,
        reason: RefundReason::CustomerRequest,
        amount_minor_units: 49_99,
        currency: CurrencyCode::Usd,
        partial_refund: false,
        initiated_by_merchant: true,
        refund_status: TransactionStatus::Refunded,
        created_at_unix: 1_701_000_000,
        processed_at_unix: 1_701_100_000,
        arn: format!("ARN{:012}", id),
    }
}

fn sample_gateway_routing(id: u64) -> GatewayRoutingDecision {
    GatewayRoutingDecision {
        decision_id: id,
        transaction_id: id + 30000,
        selected_gateway: GatewayProvider::Primary,
        fallback_gateway: GatewayProvider::SecondaryFallback,
        routing_reason: "lowest_cost".into(),
        latency_ms: 45,
        primary_available: true,
        cost_bps: 195,
        success_rate_bps: 9850,
        decided_at_unix: 1_700_050_000 + id,
    }
}

fn sample_interchange_fee(id: u64) -> InterchangeFeeCalc {
    InterchangeFeeCalc {
        calc_id: id,
        transaction_id: id + 60000,
        card_network: CardNetwork::Visa,
        merchant_category: MerchantCategory::RetailGeneral,
        interchange_rate_bps: 165,
        assessment_fee_bps: 14,
        acquirer_markup_bps: 50,
        total_fee_minor_units: 2_87,
        transaction_amount_minor: 125_99,
        is_debit_regulated: false,
        cross_border: false,
        downgrade_applied: false,
        qualification_tier: "CPS/Retail".into(),
    }
}

fn sample_kyc_screening(id: u64) -> KycAmlScreeningResult {
    KycAmlScreeningResult {
        screening_id: id,
        entity_name: format!("Entity_{}", id),
        entity_type: "Individual".into(),
        status: KycAmlStatus::Cleared,
        sanctions_lists_checked: vec![
            "OFAC SDN".into(),
            "EU Consolidated".into(),
            "UN Security Council".into(),
        ],
        pep_databases_checked: 4,
        adverse_media_hits: 0,
        match_score_percent: 0,
        screened_at_unix: 1_700_400_000 + id,
        next_review_date_unix: 1_731_936_000,
        reviewer_id: 7001,
        notes: "No adverse findings".into(),
    }
}

fn sample_cart_abandonment(id: u64) -> CartAbandonmentEvent {
    CartAbandonmentEvent {
        event_id: id,
        session_id: format!("sess_{:016x}", id),
        customer_id: 880_000 + id,
        stage: AbandonmentStage::EnteredPayment,
        cart_value_minor: 89_99,
        currency: CurrencyCode::Usd,
        item_count: 3,
        time_spent_seconds: 240,
        device_type: "mobile".into(),
        referral_source: "google_ads".into(),
        abandoned_at_unix: 1_700_600_000 + id,
        recovery_email_sent: false,
        recovered: false,
    }
}

fn sample_ledger_entry(id: u64) -> PaymentLedgerEntry {
    PaymentLedgerEntry {
        entry_id: id,
        merchant_id: 5001,
        transaction_id: id + 70000,
        debit_minor: 0,
        credit_minor: 125_99,
        balance_after_minor: 50_000_00 + 125_99 * id,
        currency: CurrencyCode::Usd,
        entry_type: "CREDIT_SALE".into(),
        posted_at_unix: 1_700_100_000 + id * 60,
        description: format!("Sale transaction #{}", id + 70000),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_payment_transaction_roundtrip_memory() {
    let txn = sample_payment_transaction(1);
    let encoded = encode_to_vec(&txn).expect("encode payment transaction");
    let (decoded, _len): (PaymentTransaction, usize) =
        decode_from_slice(&encoded).expect("decode payment transaction");
    assert_eq!(txn, decoded);
}

#[test]
fn test_tokenized_card_file_io() {
    let card = sample_tokenized_card(4242);
    let path = temp_dir().join("oxicode_test_tokenized_card_42.bin");
    encode_to_file(&card, &path).expect("write tokenized card");
    let decoded: TokenizedCard = decode_from_file(&path).expect("read tokenized card");
    assert_eq!(card, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_three_ds_authentication_batch() {
    let records: Vec<ThreeDsAuthentication> = (1..=10).map(sample_three_ds).collect();
    let encoded = encode_to_vec(&records).expect("encode 3DS batch");
    let (decoded, _len): (Vec<ThreeDsAuthentication>, usize) =
        decode_from_slice(&encoded).expect("decode 3DS batch");
    assert_eq!(records, decoded);
}

#[test]
fn test_chargeback_dispute_file_io() {
    let disputes: Vec<ChargebackDispute> = (100..=107).map(sample_chargeback).collect();
    let path = temp_dir().join("oxicode_test_chargebacks_42.bin");
    encode_to_file(&disputes, &path).expect("write chargebacks");
    let decoded: Vec<ChargebackDispute> = decode_from_file(&path).expect("read chargebacks");
    assert_eq!(disputes, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_merchant_profile_all_categories() {
    let categories = vec![
        MerchantCategory::RetailGeneral,
        MerchantCategory::GroceryStore,
        MerchantCategory::Restaurant,
        MerchantCategory::Hotel,
        MerchantCategory::Airline,
        MerchantCategory::CarRental,
        MerchantCategory::GasStation,
        MerchantCategory::DigitalGoods,
        MerchantCategory::GamingOnline,
        MerchantCategory::Subscription,
        MerchantCategory::Marketplace,
        MerchantCategory::Pharmacy,
        MerchantCategory::FinancialService,
    ];
    for (i, cat) in categories.into_iter().enumerate() {
        let mut merchant = sample_merchant(i as u32 + 1000);
        merchant.category = cat;
        let encoded = encode_to_vec(&merchant).expect("encode merchant");
        let (decoded, _len): (MerchantProfile, usize) =
            decode_from_slice(&encoded).expect("decode merchant");
        assert_eq!(merchant, decoded);
    }
}

#[test]
fn test_pci_audit_result_file_io() {
    let audit = sample_pci_audit(9001);
    let path = temp_dir().join("oxicode_test_pci_audit_42.bin");
    encode_to_file(&audit, &path).expect("write PCI audit");
    let decoded: PciAuditResult = decode_from_file(&path).expect("read PCI audit");
    assert_eq!(audit, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_recurring_subscription_lifecycle() {
    let statuses = vec![
        SubscriptionStatus::TrialPeriod,
        SubscriptionStatus::Active,
        SubscriptionStatus::PastDue,
        SubscriptionStatus::Paused,
        SubscriptionStatus::Cancelled,
        SubscriptionStatus::Expired,
    ];
    let subs: Vec<RecurringSubscription> = statuses
        .into_iter()
        .enumerate()
        .map(|(i, st)| {
            let mut sub = sample_subscription(i as u64 + 1);
            sub.status = st;
            sub
        })
        .collect();
    let path = temp_dir().join("oxicode_test_subs_lifecycle_42.bin");
    encode_to_file(&subs, &path).expect("write subscriptions");
    let decoded: Vec<RecurringSubscription> = decode_from_file(&path).expect("read subscriptions");
    assert_eq!(subs, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_multi_currency_conversion_pairs() {
    let conversions: Vec<MultiCurrencyConversion> = (1..=8)
        .map(|i| {
            let mut conv = sample_currency_conversion(i);
            conv.source_currency = match i % 4 {
                0 => CurrencyCode::Gbp,
                1 => CurrencyCode::Eur,
                2 => CurrencyCode::Jpy,
                _ => CurrencyCode::Cny,
            };
            conv.target_currency = CurrencyCode::Usd;
            conv
        })
        .collect();
    let encoded = encode_to_vec(&conversions).expect("encode conversions");
    let (decoded, _len): (Vec<MultiCurrencyConversion>, usize) =
        decode_from_slice(&encoded).expect("decode conversions");
    assert_eq!(conversions, decoded);
}

#[test]
fn test_fraud_score_risk_levels() {
    let levels = vec![
        FraudRiskLevel::VeryLow,
        FraudRiskLevel::Low,
        FraudRiskLevel::Medium,
        FraudRiskLevel::High,
        FraudRiskLevel::Critical,
    ];
    for (i, level) in levels.into_iter().enumerate() {
        let mut score = sample_fraud_score(i as u64 + 1);
        score.risk_level = level;
        score.overall_score = (i as u16 + 1) * 200;
        let encoded = encode_to_vec(&score).expect("encode fraud score");
        let (decoded, _len): (FraudScoreResult, usize) =
            decode_from_slice(&encoded).expect("decode fraud score");
        assert_eq!(score, decoded);
    }
}

#[test]
fn test_velocity_check_exceeded_threshold() {
    let mut check = sample_velocity_check(500);
    check.txn_count_last_1h = 15;
    check.total_amount_last_1h_minor = 750_00;
    check.velocity_exceeded = true;
    check.distinct_merchants_24h = 8;
    check.distinct_countries_24h = 4;
    let path = temp_dir().join("oxicode_test_velocity_42.bin");
    encode_to_file(&check, &path).expect("write velocity check");
    let decoded: VelocityCheckResult = decode_from_file(&path).expect("read velocity check");
    assert_eq!(check, decoded);
    assert!(decoded.velocity_exceeded);
    assert!(decoded.txn_count_last_1h > decoded.threshold_txn_per_hour);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_device_fingerprint_anomaly_detection() {
    let mut normal_fp = sample_device_fingerprint(1);
    normal_fp.anomaly_detected = false;
    normal_fp.trust_score = 900;

    let mut suspicious_fp = sample_device_fingerprint(2);
    suspicious_fp.anomaly_detected = true;
    suspicious_fp.trust_score = 120;
    suspicious_fp.timezone_offset_min = 330;
    suspicious_fp.language_code = "ru-RU".into();

    let fingerprints = vec![normal_fp, suspicious_fp];
    let encoded = encode_to_vec(&fingerprints).expect("encode fingerprints");
    let (decoded, _len): (Vec<DeviceFingerprint>, usize) =
        decode_from_slice(&encoded).expect("decode fingerprints");
    assert_eq!(fingerprints, decoded);
    assert!(!decoded[0].anomaly_detected);
    assert!(decoded[1].anomaly_detected);
}

#[test]
fn test_settlement_batch_file_io() {
    let batches: Vec<SettlementBatch> = (1..=5).map(sample_settlement_batch).collect();
    let path = temp_dir().join("oxicode_test_settlement_42.bin");
    encode_to_file(&batches, &path).expect("write settlement batches");
    let decoded: Vec<SettlementBatch> = decode_from_file(&path).expect("read settlement batches");
    assert_eq!(batches, decoded);
    for batch in &decoded {
        assert_eq!(
            batch.net_amount_minor,
            batch.gross_amount_minor - batch.fees_amount_minor
        );
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_refund_workflow_partial_and_full() {
    let full_refund = sample_refund(1);
    let mut partial_refund = sample_refund(2);
    partial_refund.partial_refund = true;
    partial_refund.amount_minor_units = 20_00;
    partial_refund.reason = RefundReason::MerchantError;

    let refunds = vec![full_refund, partial_refund];
    let encoded = encode_to_vec(&refunds).expect("encode refunds");
    let (decoded, _len): (Vec<RefundRecord>, usize) =
        decode_from_slice(&encoded).expect("decode refunds");
    assert_eq!(refunds, decoded);
    assert!(!decoded[0].partial_refund);
    assert!(decoded[1].partial_refund);
}

#[test]
fn test_gateway_routing_fallback_scenario() {
    let mut primary_route = sample_gateway_routing(1);
    primary_route.primary_available = true;
    primary_route.selected_gateway = GatewayProvider::Primary;

    let mut fallback_route = sample_gateway_routing(2);
    fallback_route.primary_available = false;
    fallback_route.selected_gateway = GatewayProvider::SecondaryFallback;
    fallback_route.routing_reason = "primary_unavailable".into();
    fallback_route.latency_ms = 120;

    let mut regional_route = sample_gateway_routing(3);
    regional_route.selected_gateway = GatewayProvider::RegionalProcessor;
    regional_route.routing_reason = "geo_optimization".into();
    regional_route.cost_bps = 140;

    let decisions = vec![primary_route, fallback_route, regional_route];
    let path = temp_dir().join("oxicode_test_routing_42.bin");
    encode_to_file(&decisions, &path).expect("write routing decisions");
    let decoded: Vec<GatewayRoutingDecision> =
        decode_from_file(&path).expect("read routing decisions");
    assert_eq!(decisions, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_interchange_fee_cross_border() {
    let mut domestic = sample_interchange_fee(1);
    domestic.cross_border = false;
    domestic.interchange_rate_bps = 165;

    let mut cross_border = sample_interchange_fee(2);
    cross_border.cross_border = true;
    cross_border.interchange_rate_bps = 250;
    cross_border.total_fee_minor_units = 4_25;

    let mut downgraded = sample_interchange_fee(3);
    downgraded.downgrade_applied = true;
    downgraded.interchange_rate_bps = 295;
    downgraded.qualification_tier = "EIRF".into();

    let calcs = vec![domestic, cross_border, downgraded];
    let encoded = encode_to_vec(&calcs).expect("encode interchange fees");
    let (decoded, _len): (Vec<InterchangeFeeCalc>, usize) =
        decode_from_slice(&encoded).expect("decode interchange fees");
    assert_eq!(calcs, decoded);
    assert!(decoded[1].interchange_rate_bps > decoded[0].interchange_rate_bps);
}

#[test]
fn test_kyc_aml_screening_all_statuses() {
    let statuses = vec![
        KycAmlStatus::NotScreened,
        KycAmlStatus::Pending,
        KycAmlStatus::Cleared,
        KycAmlStatus::FlaggedForReview,
        KycAmlStatus::SanctionsHit,
        KycAmlStatus::PepMatch,
        KycAmlStatus::AdverseMedia,
        KycAmlStatus::Rejected,
    ];
    let screenings: Vec<KycAmlScreeningResult> = statuses
        .into_iter()
        .enumerate()
        .map(|(i, st)| {
            let mut s = sample_kyc_screening(i as u64 + 1);
            s.status = st;
            s
        })
        .collect();
    let path = temp_dir().join("oxicode_test_kyc_42.bin");
    encode_to_file(&screenings, &path).expect("write KYC screenings");
    let decoded: Vec<KycAmlScreeningResult> = decode_from_file(&path).expect("read KYC screenings");
    assert_eq!(screenings, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_cart_abandonment_analytics_stages() {
    let stages = vec![
        AbandonmentStage::BrowsingProducts,
        AbandonmentStage::AddedToCart,
        AbandonmentStage::StartedCheckout,
        AbandonmentStage::EnteredShipping,
        AbandonmentStage::EnteredPayment,
        AbandonmentStage::ReviewOrder,
    ];
    let events: Vec<CartAbandonmentEvent> = stages
        .into_iter()
        .enumerate()
        .map(|(i, stage)| {
            let mut evt = sample_cart_abandonment(i as u64 + 1);
            evt.stage = stage;
            evt.cart_value_minor = (i as u64 + 1) * 50_00;
            evt
        })
        .collect();
    let encoded = encode_to_vec(&events).expect("encode cart abandonments");
    let (decoded, _len): (Vec<CartAbandonmentEvent>, usize) =
        decode_from_slice(&encoded).expect("decode cart abandonments");
    assert_eq!(events, decoded);
}

#[test]
fn test_payment_ledger_entries_file_io() {
    let entries: Vec<PaymentLedgerEntry> = (1..=20).map(sample_ledger_entry).collect();
    let path = temp_dir().join("oxicode_test_ledger_42.bin");
    encode_to_file(&entries, &path).expect("write ledger entries");
    let decoded: Vec<PaymentLedgerEntry> = decode_from_file(&path).expect("read ledger entries");
    assert_eq!(entries, decoded);
    assert_eq!(decoded.len(), 20);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_full_transaction_pipeline_file_io() {
    let txn = sample_payment_transaction(777);
    let three_ds = sample_three_ds(777);
    let fraud = sample_fraud_score(777);
    let velocity = sample_velocity_check(777);
    let routing = sample_gateway_routing(777);
    let interchange = sample_interchange_fee(777);

    let pipeline = (
        txn.clone(),
        three_ds.clone(),
        fraud.clone(),
        velocity.clone(),
        routing.clone(),
        interchange.clone(),
    );
    let path = temp_dir().join("oxicode_test_pipeline_42.bin");
    encode_to_file(&pipeline, &path).expect("write pipeline");
    let decoded: (
        PaymentTransaction,
        ThreeDsAuthentication,
        FraudScoreResult,
        VelocityCheckResult,
        GatewayRoutingDecision,
        InterchangeFeeCalc,
    ) = decode_from_file(&path).expect("read pipeline");
    assert_eq!(pipeline, decoded);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_chargeback_dispute_progression() {
    let stages = vec![
        DisputeStage::FirstChargeback,
        DisputeStage::PreArbitration,
        DisputeStage::Arbitration,
        DisputeStage::ComplianceFiling,
        DisputeStage::Resolved,
    ];
    let disputes: Vec<ChargebackDispute> = stages
        .into_iter()
        .enumerate()
        .map(|(i, stage)| {
            let mut d = sample_chargeback(200 + i as u64);
            d.stage = stage;
            d.merchant_responded = i > 0;
            d.evidence_submitted = if i > 0 {
                vec![format!("receipt_{}.pdf", i), format!("logs_{}.txt", i)]
            } else {
                vec![]
            };
            d.outcome_favorable = i >= 3;
            d.representment_amount_minor = if i >= 1 { 299_99 } else { 0 };
            d
        })
        .collect();
    let encoded = encode_to_vec(&disputes).expect("encode dispute progression");
    let (decoded, _len): (Vec<ChargebackDispute>, usize) =
        decode_from_slice(&encoded).expect("decode dispute progression");
    assert_eq!(disputes, decoded);
    assert!(!decoded[0].merchant_responded);
    assert!(decoded[4].outcome_favorable);
}

#[test]
fn test_high_risk_fraud_scenario_combined() {
    let mut high_risk_score = sample_fraud_score(999);
    high_risk_score.risk_level = FraudRiskLevel::Critical;
    high_risk_score.overall_score = 950;
    high_risk_score.velocity_check_score = 200;
    high_risk_score.device_fingerprint_score = 190;
    high_risk_score.geo_anomaly_score = 180;
    high_risk_score.behavioral_score = 200;
    high_risk_score.card_testing_score = 180;
    high_risk_score.manual_review_required = true;
    high_risk_score.rules_triggered = vec![
        "VELOCITY_BURST".into(),
        "NEW_DEVICE".into(),
        "GEO_MISMATCH".into(),
        "CARD_TESTING_PATTERN".into(),
        "HIGH_VALUE_NEW_CUSTOMER".into(),
    ];

    let mut suspicious_velocity = sample_velocity_check(999);
    suspicious_velocity.txn_count_last_1h = 25;
    suspicious_velocity.total_amount_last_1h_minor = 2_500_00;
    suspicious_velocity.distinct_merchants_24h = 12;
    suspicious_velocity.distinct_countries_24h = 5;
    suspicious_velocity.velocity_exceeded = true;

    let mut suspicious_device = sample_device_fingerprint(999);
    suspicious_device.anomaly_detected = true;
    suspicious_device.trust_score = 50;
    suspicious_device.timezone_offset_min = 180;

    let scenario = (
        high_risk_score.clone(),
        suspicious_velocity.clone(),
        suspicious_device.clone(),
    );
    let path = temp_dir().join("oxicode_test_high_risk_42.bin");
    encode_to_file(&scenario, &path).expect("write high risk scenario");
    let decoded: (FraudScoreResult, VelocityCheckResult, DeviceFingerprint) =
        decode_from_file(&path).expect("read high risk scenario");
    assert_eq!(scenario, decoded);
    assert!(decoded.0.manual_review_required);
    assert!(decoded.1.velocity_exceeded);
    assert!(decoded.2.anomaly_detected);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_merchant_settlement_with_refunds_and_chargebacks() {
    let merchant = sample_merchant(6001);
    let batch = sample_settlement_batch(8001);
    let refunds: Vec<RefundRecord> = (1..=3)
        .map(|i| {
            let mut r = sample_refund(i);
            r.reason = match i {
                1 => RefundReason::CustomerRequest,
                2 => RefundReason::FraudConfirmed,
                _ => RefundReason::DuplicateTransaction,
            };
            r
        })
        .collect();
    let disputes: Vec<ChargebackDispute> = (1..=2).map(sample_chargeback).collect();

    let full_record = (
        merchant.clone(),
        batch.clone(),
        refunds.clone(),
        disputes.clone(),
    );
    let path = temp_dir().join("oxicode_test_merchant_settlement_42.bin");
    encode_to_file(&full_record, &path).expect("write merchant settlement");
    let decoded: (
        MerchantProfile,
        SettlementBatch,
        Vec<RefundRecord>,
        Vec<ChargebackDispute>,
    ) = decode_from_file(&path).expect("read merchant settlement");
    assert_eq!(full_record, decoded);
    assert_eq!(decoded.2.len(), 3);
    assert_eq!(decoded.3.len(), 2);
    let _ = std::fs::remove_file(&path);
}
