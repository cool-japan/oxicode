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

// --- Domain types: Library Management & Digital Collections ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MarcBibRecord {
    record_id: u64,
    leader: String,
    control_number: String,
    isbn: Option<String>,
    title: String,
    author: String,
    publisher: String,
    publication_year: u16,
    edition: Option<String>,
    physical_description: String,
    subject_headings: Vec<String>,
    language_code: String,
    encoding_level: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PatronAccount {
    patron_id: u64,
    name: String,
    email: String,
    patron_type: String,
    checkout_limit: u16,
    current_checkouts: u16,
    total_fines_cents: u64,
    outstanding_fines_cents: u64,
    holds_limit: u16,
    active_holds: u16,
    is_active: bool,
    barcode: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CirculationAction {
    Checkout,
    Checkin,
    Renewal,
    Hold,
    HoldPickup,
    HoldExpired,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CirculationTransaction {
    transaction_id: u64,
    patron_id: u64,
    item_barcode: String,
    action: CirculationAction,
    timestamp_epoch: u64,
    due_date_epoch: Option<u64>,
    branch_code: String,
    staff_id: Option<u32>,
    renewal_count: u8,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InterlibraryLoanRequest {
    request_id: u64,
    requesting_library_code: String,
    lending_library_code: Option<String>,
    patron_id: u64,
    title: String,
    author: String,
    isbn: Option<String>,
    issn: Option<String>,
    status: String,
    request_date_epoch: u64,
    fulfillment_date_epoch: Option<u64>,
    shipping_method: String,
    max_cost_cents: u32,
    notes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ClassificationScheme {
    DeweyDecimal {
        class_number: String,
        cutter_number: String,
    },
    LibraryOfCongress {
        class_letters: String,
        subclass_number: String,
        cutter: String,
    },
    Sudoc {
        stem: String,
        book_number: String,
    },
    Nlm {
        class_code: String,
        cutter: String,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClassificationAssignment {
    item_id: u64,
    scheme: ClassificationScheme,
    call_number: String,
    shelving_location: String,
    collection_code: String,
    assigned_by: String,
    assigned_date_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SerialSubscription {
    subscription_id: u64,
    title: String,
    issn: String,
    publisher: String,
    frequency: String,
    start_date_epoch: u64,
    end_date_epoch: Option<u64>,
    annual_cost_cents: u64,
    format: String,
    issues_received: u32,
    issues_expected: u32,
    claiming_threshold_days: u16,
    binding_preference: String,
    is_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DublinCoreMetadata {
    identifier: String,
    title: String,
    creator: Vec<String>,
    subject: Vec<String>,
    description: String,
    publisher: String,
    contributor: Vec<String>,
    date: String,
    resource_type: String,
    format: String,
    source: Option<String>,
    language: String,
    relation: Vec<String>,
    coverage: Option<String>,
    rights: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EbookLicense {
    license_id: u64,
    title: String,
    isbn: String,
    vendor: String,
    model: String,
    concurrent_users: u16,
    total_checkouts_allowed: Option<u32>,
    checkouts_used: u32,
    drm_type: String,
    annual_cost_cents: u64,
    start_date_epoch: u64,
    expiry_date_epoch: Option<u64>,
    formats_available: Vec<String>,
    metered_access: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CollectionBudgetLine {
    budget_id: u64,
    fiscal_year: u16,
    fund_code: String,
    fund_name: String,
    allocated_cents: u64,
    encumbered_cents: u64,
    expended_cents: u64,
    subject_area: String,
    material_type: String,
    responsible_librarian: String,
    notes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReadingProgramParticipant {
    participant_id: u64,
    patron_id: u64,
    program_name: String,
    program_year: u16,
    age_group: String,
    books_read: u32,
    pages_read: u64,
    minutes_read: u64,
    badges_earned: Vec<String>,
    prizes_claimed: Vec<String>,
    registration_date_epoch: u64,
    completed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReferenceDeskStats {
    stats_id: u64,
    branch_code: String,
    date_epoch: u64,
    hour_of_day: u8,
    questions_received: u32,
    directional: u32,
    ready_reference: u32,
    research_consultations: u32,
    technology_assistance: u32,
    referrals_made: u32,
    avg_duration_seconds: u32,
    staff_on_desk: u8,
    virtual_queries: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ArchiveFindingAid {
    finding_aid_id: u64,
    collection_title: String,
    collection_number: String,
    creator: String,
    date_range: String,
    extent: String,
    abstract_text: String,
    access_restrictions: Option<String>,
    use_restrictions: Option<String>,
    languages: Vec<String>,
    series: Vec<ArchiveSeries>,
    processing_status: String,
    ead_url: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ArchiveSeries {
    series_number: u16,
    title: String,
    date_range: String,
    extent: String,
    scope_note: String,
    box_count: u32,
    folder_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MakerspaceReservation {
    reservation_id: u64,
    patron_id: u64,
    equipment_name: String,
    equipment_category: String,
    branch_code: String,
    start_epoch: u64,
    end_epoch: u64,
    duration_minutes: u32,
    certification_required: bool,
    certification_verified: bool,
    materials_provided: Vec<String>,
    project_type: String,
    staff_assistance_needed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AccessibilityAccommodation {
    accommodation_id: u64,
    patron_id: u64,
    accommodation_type: String,
    description: String,
    assistive_tech: Vec<String>,
    preferred_format: String,
    large_print_size: Option<u8>,
    audio_speed_pct: Option<u8>,
    sign_language_interpreter: bool,
    service_animal_registered: bool,
    mobility_requirements: Vec<String>,
    effective_date_epoch: u64,
    review_date_epoch: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BranchPerformanceMetrics {
    branch_code: String,
    branch_name: String,
    reporting_period: String,
    total_circulation: u64,
    physical_visits: u64,
    program_attendance: u32,
    new_registrations: u32,
    computer_sessions: u32,
    wifi_sessions: u32,
    reference_transactions: u32,
    meeting_room_bookings: u32,
    volunteer_hours: u32,
    cost_per_circulation_cents: u32,
    collection_turnover_rate_x100: u32,
    square_footage: u32,
    staff_fte_x10: u16,
}

// --- Tests ---

#[test]
fn test_marc_bibliographic_record_roundtrip() {
    let record = MarcBibRecord {
        record_id: 100_000_001,
        leader: "00942cam a2200301 a 4500".to_string(),
        control_number: "ocm12345678".to_string(),
        isbn: Some("978-0-13-468599-1".to_string()),
        title: "The Rust Programming Language".to_string(),
        author: "Klabnik, Steve".to_string(),
        publisher: "No Starch Press".to_string(),
        publication_year: 2019,
        edition: Some("2nd ed.".to_string()),
        physical_description: "xxi, 526 pages ; 24 cm".to_string(),
        subject_headings: vec![
            "Rust (Computer program language)".to_string(),
            "Systems programming".to_string(),
        ],
        language_code: "eng".to_string(),
        encoding_level: 1,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("encode marc record");
    let (decoded, _): (MarcBibRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode marc record");
    assert_eq!(record, decoded);
}

#[test]
fn test_patron_account_roundtrip() {
    let patron = PatronAccount {
        patron_id: 2_000_000_042,
        name: "Ada Lovelace".to_string(),
        email: "ada@example.org".to_string(),
        patron_type: "Faculty".to_string(),
        checkout_limit: 50,
        current_checkouts: 12,
        total_fines_cents: 1500,
        outstanding_fines_cents: 350,
        holds_limit: 25,
        active_holds: 3,
        is_active: true,
        barcode: "21234567890123".to_string(),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&patron, cfg).expect("encode patron");
    let (decoded, _): (PatronAccount, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode patron");
    assert_eq!(patron, decoded);
}

#[test]
fn test_circulation_checkout_roundtrip() {
    let txn = CirculationTransaction {
        transaction_id: 500_000_001,
        patron_id: 2_000_000_042,
        item_barcode: "31234567890123".to_string(),
        action: CirculationAction::Checkout,
        timestamp_epoch: 1_710_500_000,
        due_date_epoch: Some(1_712_320_000),
        branch_code: "MAIN".to_string(),
        staff_id: Some(1001),
        renewal_count: 0,
        notes: None,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&txn, cfg).expect("encode checkout txn");
    let (decoded, _): (CirculationTransaction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode checkout txn");
    assert_eq!(txn, decoded);
}

#[test]
fn test_circulation_renewal_roundtrip() {
    let txn = CirculationTransaction {
        transaction_id: 500_000_099,
        patron_id: 2_000_000_042,
        item_barcode: "31234567890456".to_string(),
        action: CirculationAction::Renewal,
        timestamp_epoch: 1_711_200_000,
        due_date_epoch: Some(1_713_020_000),
        branch_code: "EAST".to_string(),
        staff_id: None,
        renewal_count: 2,
        notes: Some("Online self-renewal".to_string()),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&txn, cfg).expect("encode renewal txn");
    let (decoded, _): (CirculationTransaction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode renewal txn");
    assert_eq!(txn, decoded);
}

#[test]
fn test_circulation_hold_roundtrip() {
    let txn = CirculationTransaction {
        transaction_id: 500_000_200,
        patron_id: 2_000_000_100,
        item_barcode: "31234567891111".to_string(),
        action: CirculationAction::Hold,
        timestamp_epoch: 1_710_600_000,
        due_date_epoch: None,
        branch_code: "WEST".to_string(),
        staff_id: Some(1005),
        renewal_count: 0,
        notes: Some("Patron requested specific edition".to_string()),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&txn, cfg).expect("encode hold txn");
    let (decoded, _): (CirculationTransaction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode hold txn");
    assert_eq!(txn, decoded);
}

#[test]
fn test_interlibrary_loan_request_roundtrip() {
    let ill = InterlibraryLoanRequest {
        request_id: 80_000_001,
        requesting_library_code: "OCLC-12345".to_string(),
        lending_library_code: Some("OCLC-67890".to_string()),
        patron_id: 2_000_000_042,
        title: "Compilers: Principles, Techniques, and Tools".to_string(),
        author: "Aho, Alfred V.".to_string(),
        isbn: Some("978-0-321-48681-3".to_string()),
        issn: None,
        status: "Shipped".to_string(),
        request_date_epoch: 1_709_000_000,
        fulfillment_date_epoch: Some(1_709_500_000),
        shipping_method: "USPS Library Mail".to_string(),
        max_cost_cents: 2500,
        notes: vec![
            "Needed for graduate seminar".to_string(),
            "Second copy preferred if available".to_string(),
        ],
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&ill, cfg).expect("encode ILL request");
    let (decoded, _): (InterlibraryLoanRequest, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ILL request");
    assert_eq!(ill, decoded);
}

#[test]
fn test_dewey_classification_roundtrip() {
    let assignment = ClassificationAssignment {
        item_id: 900_000_001,
        scheme: ClassificationScheme::DeweyDecimal {
            class_number: "005.133".to_string(),
            cutter_number: "K53".to_string(),
        },
        call_number: "005.133 K53 2019".to_string(),
        shelving_location: "Stacks Floor 3".to_string(),
        collection_code: "CIRC".to_string(),
        assigned_by: "cataloger_mj".to_string(),
        assigned_date_epoch: 1_708_500_000,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&assignment, cfg).expect("encode dewey classification");
    let (decoded, _): (ClassificationAssignment, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode dewey classification");
    assert_eq!(assignment, decoded);
}

#[test]
fn test_lc_classification_roundtrip() {
    let assignment = ClassificationAssignment {
        item_id: 900_000_002,
        scheme: ClassificationScheme::LibraryOfCongress {
            class_letters: "QA".to_string(),
            subclass_number: "76.73".to_string(),
            cutter: ".R87 K53".to_string(),
        },
        call_number: "QA76.73.R87 K53 2019".to_string(),
        shelving_location: "Science Reading Room".to_string(),
        collection_code: "REF".to_string(),
        assigned_by: "cataloger_lp".to_string(),
        assigned_date_epoch: 1_708_600_000,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&assignment, cfg).expect("encode LC classification");
    let (decoded, _): (ClassificationAssignment, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LC classification");
    assert_eq!(assignment, decoded);
}

#[test]
fn test_serials_subscription_roundtrip() {
    let sub = SerialSubscription {
        subscription_id: 70_000_001,
        title: "Journal of the ACM".to_string(),
        issn: "0004-5411".to_string(),
        publisher: "Association for Computing Machinery".to_string(),
        frequency: "Bimonthly".to_string(),
        start_date_epoch: 1_672_531_200,
        end_date_epoch: Some(1_704_067_200),
        annual_cost_cents: 85_000,
        format: "Print+Electronic".to_string(),
        issues_received: 5,
        issues_expected: 6,
        claiming_threshold_days: 45,
        binding_preference: "Annual volume".to_string(),
        is_active: true,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&sub, cfg).expect("encode serial subscription");
    let (decoded, _): (SerialSubscription, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode serial subscription");
    assert_eq!(sub, decoded);
}

#[test]
fn test_dublin_core_metadata_roundtrip() {
    let dc = DublinCoreMetadata {
        identifier: "hdl:2142/12345".to_string(),
        title: "Analysis of Memory Safety Patterns in Systems Programming".to_string(),
        creator: vec!["Zhang, Wei".to_string(), "Patel, Priya".to_string()],
        subject: vec![
            "Memory safety".to_string(),
            "Systems programming".to_string(),
            "Static analysis".to_string(),
        ],
        description: "A comprehensive study of memory safety enforcement techniques \
            in modern systems programming languages."
            .to_string(),
        publisher: "University Digital Repository".to_string(),
        contributor: vec!["CS Department".to_string()],
        date: "2024-03-15".to_string(),
        resource_type: "Thesis".to_string(),
        format: "application/pdf".to_string(),
        source: Some("ETD Collection".to_string()),
        language: "en".to_string(),
        relation: vec!["doi:10.1234/example.5678".to_string()],
        coverage: Some("2020-2024".to_string()),
        rights: "CC BY-NC 4.0".to_string(),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&dc, cfg).expect("encode dublin core");
    let (decoded, _): (DublinCoreMetadata, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode dublin core");
    assert_eq!(dc, decoded);
}

#[test]
fn test_ebook_license_roundtrip() {
    let license = EbookLicense {
        license_id: 60_000_001,
        title: "Introduction to Algorithms".to_string(),
        isbn: "978-0-262-04630-5".to_string(),
        vendor: "OverDrive".to_string(),
        model: "OnePerUser".to_string(),
        concurrent_users: 3,
        total_checkouts_allowed: Some(52),
        checkouts_used: 31,
        drm_type: "Adobe DRM".to_string(),
        annual_cost_cents: 12_500,
        start_date_epoch: 1_704_067_200,
        expiry_date_epoch: Some(1_735_689_600),
        formats_available: vec!["EPUB".to_string(), "PDF".to_string()],
        metered_access: true,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&license, cfg).expect("encode ebook license");
    let (decoded, _): (EbookLicense, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ebook license");
    assert_eq!(license, decoded);
}

#[test]
fn test_ebook_license_unlimited_roundtrip() {
    let license = EbookLicense {
        license_id: 60_000_002,
        title: "Open Access Textbook: Data Structures".to_string(),
        isbn: "978-1-999-99999-0".to_string(),
        vendor: "Direct Publisher".to_string(),
        model: "Perpetual".to_string(),
        concurrent_users: 65535,
        total_checkouts_allowed: None,
        checkouts_used: 0,
        drm_type: "None".to_string(),
        annual_cost_cents: 0,
        start_date_epoch: 1_704_067_200,
        expiry_date_epoch: None,
        formats_available: vec!["EPUB".to_string(), "PDF".to_string(), "HTML".to_string()],
        metered_access: false,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&license, cfg).expect("encode perpetual license");
    let (decoded, _): (EbookLicense, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode perpetual license");
    assert_eq!(license, decoded);
}

#[test]
fn test_collection_development_budget_roundtrip() {
    let budget = CollectionBudgetLine {
        budget_id: 40_000_001,
        fiscal_year: 2025,
        fund_code: "STEM-MONO".to_string(),
        fund_name: "STEM Monographs".to_string(),
        allocated_cents: 15_000_000,
        encumbered_cents: 3_200_000,
        expended_cents: 9_800_000,
        subject_area: "Computer Science".to_string(),
        material_type: "Books".to_string(),
        responsible_librarian: "Dr. Sarah Chen".to_string(),
        notes: vec![
            "Increased allocation due to new CS program".to_string(),
            "Priority: AI and machine learning titles".to_string(),
        ],
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&budget, cfg).expect("encode budget line");
    let (decoded, _): (CollectionBudgetLine, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode budget line");
    assert_eq!(budget, decoded);
}

#[test]
fn test_reading_program_tracking_roundtrip() {
    let participant = ReadingProgramParticipant {
        participant_id: 30_000_001,
        patron_id: 2_000_000_200,
        program_name: "Summer Reading Challenge 2025".to_string(),
        program_year: 2025,
        age_group: "Teens (13-17)".to_string(),
        books_read: 14,
        pages_read: 4200,
        minutes_read: 12_600,
        badges_earned: vec![
            "Bookworm Bronze".to_string(),
            "Genre Explorer".to_string(),
            "Page Turner Silver".to_string(),
        ],
        prizes_claimed: vec!["Bookmark Set".to_string()],
        registration_date_epoch: 1_717_200_000,
        completed: false,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&participant, cfg).expect("encode reading program");
    let (decoded, _): (ReadingProgramParticipant, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reading program");
    assert_eq!(participant, decoded);
}

#[test]
fn test_reference_desk_statistics_roundtrip() {
    let stats = ReferenceDeskStats {
        stats_id: 20_000_001,
        branch_code: "MAIN".to_string(),
        date_epoch: 1_710_500_000,
        hour_of_day: 14,
        questions_received: 47,
        directional: 12,
        ready_reference: 18,
        research_consultations: 8,
        technology_assistance: 6,
        referrals_made: 3,
        avg_duration_seconds: 420,
        staff_on_desk: 2,
        virtual_queries: 11,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&stats, cfg).expect("encode reference stats");
    let (decoded, _): (ReferenceDeskStats, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reference stats");
    assert_eq!(stats, decoded);
}

#[test]
fn test_archive_finding_aid_roundtrip() {
    let finding_aid = ArchiveFindingAid {
        finding_aid_id: 10_000_001,
        collection_title: "Papers of Dr. Grace Hopper, 1944-1985".to_string(),
        collection_number: "MS-2024-042".to_string(),
        creator: "Hopper, Grace Murray, 1906-1992".to_string(),
        date_range: "1944-1985".to_string(),
        extent: "42 linear feet (84 boxes)".to_string(),
        abstract_text: "Professional papers documenting contributions to computer science, \
            including COBOL development, naval computing projects, and educational materials."
            .to_string(),
        access_restrictions: Some("Box 12 restricted until 2030".to_string()),
        use_restrictions: Some("Permission required for publication".to_string()),
        languages: vec!["English".to_string(), "French".to_string()],
        series: vec![
            ArchiveSeries {
                series_number: 1,
                title: "Correspondence".to_string(),
                date_range: "1944-1985".to_string(),
                extent: "10 linear feet".to_string(),
                scope_note: "Professional and personal correspondence".to_string(),
                box_count: 20,
                folder_count: 480,
            },
            ArchiveSeries {
                series_number: 2,
                title: "Technical Reports and Publications".to_string(),
                date_range: "1950-1982".to_string(),
                extent: "15 linear feet".to_string(),
                scope_note: "Published and unpublished technical reports".to_string(),
                box_count: 30,
                folder_count: 720,
            },
            ArchiveSeries {
                series_number: 3,
                title: "Photographs and Media".to_string(),
                date_range: "1946-1985".to_string(),
                extent: "5 linear feet".to_string(),
                scope_note: "Photographs, film reels, and audio tapes".to_string(),
                box_count: 10,
                folder_count: 150,
            },
        ],
        processing_status: "Fully Processed".to_string(),
        ead_url: Some("https://repository.example.org/ead/MS-2024-042.xml".to_string()),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&finding_aid, cfg).expect("encode finding aid");
    let (decoded, _): (ArchiveFindingAid, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode finding aid");
    assert_eq!(finding_aid, decoded);
}

#[test]
fn test_makerspace_equipment_reservation_roundtrip() {
    let reservation = MakerspaceReservation {
        reservation_id: 50_000_001,
        patron_id: 2_000_000_300,
        equipment_name: "Prusa i3 MK3S+ 3D Printer".to_string(),
        equipment_category: "3D Printing".to_string(),
        branch_code: "MAIN".to_string(),
        start_epoch: 1_710_600_000,
        end_epoch: 1_710_607_200,
        duration_minutes: 120,
        certification_required: true,
        certification_verified: true,
        materials_provided: vec![
            "PLA filament (white)".to_string(),
            "Build plate adhesive".to_string(),
        ],
        project_type: "Personal project".to_string(),
        staff_assistance_needed: false,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&reservation, cfg).expect("encode makerspace reservation");
    let (decoded, _): (MakerspaceReservation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode makerspace reservation");
    assert_eq!(reservation, decoded);
}

#[test]
fn test_accessibility_accommodation_roundtrip() {
    let accommodation = AccessibilityAccommodation {
        accommodation_id: 15_000_001,
        patron_id: 2_000_000_400,
        accommodation_type: "Visual Impairment".to_string(),
        description: "Legally blind patron requiring large print and audio materials".to_string(),
        assistive_tech: vec![
            "JAWS screen reader".to_string(),
            "Braille display".to_string(),
            "Magnification software".to_string(),
        ],
        preferred_format: "Audio".to_string(),
        large_print_size: Some(18),
        audio_speed_pct: Some(125),
        sign_language_interpreter: false,
        service_animal_registered: true,
        mobility_requirements: vec![],
        effective_date_epoch: 1_704_067_200,
        review_date_epoch: Some(1_735_689_600),
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&accommodation, cfg).expect("encode accommodation");
    let (decoded, _): (AccessibilityAccommodation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode accommodation");
    assert_eq!(accommodation, decoded);
}

#[test]
fn test_branch_performance_metrics_roundtrip() {
    let metrics = BranchPerformanceMetrics {
        branch_code: "MAIN".to_string(),
        branch_name: "Main Library".to_string(),
        reporting_period: "FY2025-Q1".to_string(),
        total_circulation: 145_320,
        physical_visits: 89_500,
        program_attendance: 4_200,
        new_registrations: 780,
        computer_sessions: 12_400,
        wifi_sessions: 35_600,
        reference_transactions: 6_800,
        meeting_room_bookings: 340,
        volunteer_hours: 1_200,
        cost_per_circulation_cents: 285,
        collection_turnover_rate_x100: 350,
        square_footage: 45_000,
        staff_fte_x10: 425,
    };

    let cfg = config::standard();
    let encoded = encode_to_vec(&metrics, cfg).expect("encode branch metrics");
    let (decoded, _): (BranchPerformanceMetrics, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode branch metrics");
    assert_eq!(metrics, decoded);
}

#[test]
fn test_multiple_marc_records_roundtrip() {
    let records = vec![
        MarcBibRecord {
            record_id: 100_000_010,
            leader: "01234nam a2200361 i 4500".to_string(),
            control_number: "ocm99887766".to_string(),
            isbn: Some("978-0-596-51774-8".to_string()),
            title: "Programming Rust".to_string(),
            author: "Blandy, Jim".to_string(),
            publisher: "O'Reilly Media".to_string(),
            publication_year: 2021,
            edition: Some("2nd ed.".to_string()),
            physical_description: "xxvi, 718 pages ; 24 cm".to_string(),
            subject_headings: vec![
                "Rust (Computer program language)".to_string(),
                "Computer programming".to_string(),
            ],
            language_code: "eng".to_string(),
            encoding_level: 1,
        },
        MarcBibRecord {
            record_id: 100_000_011,
            leader: "00890cam a2200265 a 4500".to_string(),
            control_number: "ocm11223344".to_string(),
            isbn: None,
            title: "Ancient Manuscript Collection Volume XII".to_string(),
            author: "Unknown".to_string(),
            publisher: "University Press".to_string(),
            publication_year: 1890,
            edition: None,
            physical_description: "iv, 312 pages : ill. ; 30 cm".to_string(),
            subject_headings: vec![
                "Manuscripts, Medieval".to_string(),
                "Paleography".to_string(),
                "Illumination of books and manuscripts".to_string(),
            ],
            language_code: "lat".to_string(),
            encoding_level: 3,
        },
    ];

    let cfg = config::standard();
    let encoded = encode_to_vec(&records, cfg).expect("encode marc records vec");
    let (decoded, _): (Vec<MarcBibRecord>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode marc records vec");
    assert_eq!(records, decoded);
}

#[test]
fn test_circulation_actions_all_variants_roundtrip() {
    let actions = vec![
        CirculationAction::Checkout,
        CirculationAction::Checkin,
        CirculationAction::Renewal,
        CirculationAction::Hold,
        CirculationAction::HoldPickup,
        CirculationAction::HoldExpired,
    ];

    let cfg = config::standard();
    let encoded = encode_to_vec(&actions, cfg).expect("encode all circulation actions");
    let (decoded, _): (Vec<CirculationAction>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode all circulation actions");
    assert_eq!(actions, decoded);
}

#[test]
fn test_sudoc_nlm_classification_roundtrip() {
    let assignments = vec![
        ClassificationAssignment {
            item_id: 900_000_010,
            scheme: ClassificationScheme::Sudoc {
                stem: "Y 4.SCI 2:".to_string(),
                book_number: "109-78".to_string(),
            },
            call_number: "Y 4.SCI 2:109-78".to_string(),
            shelving_location: "Government Documents".to_string(),
            collection_code: "GOVDOC".to_string(),
            assigned_by: "cataloger_rw".to_string(),
            assigned_date_epoch: 1_709_100_000,
        },
        ClassificationAssignment {
            item_id: 900_000_011,
            scheme: ClassificationScheme::Nlm {
                class_code: "QZ 200".to_string(),
                cutter: ".M45".to_string(),
            },
            call_number: "QZ 200 .M45 2023".to_string(),
            shelving_location: "Health Sciences Library".to_string(),
            collection_code: "MED".to_string(),
            assigned_by: "cataloger_tn".to_string(),
            assigned_date_epoch: 1_709_200_000,
        },
    ];

    let cfg = config::standard();
    let encoded = encode_to_vec(&assignments, cfg).expect("encode sudoc/nlm assignments");
    let (decoded, _): (Vec<ClassificationAssignment>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode sudoc/nlm assignments");
    assert_eq!(assignments, decoded);
}
