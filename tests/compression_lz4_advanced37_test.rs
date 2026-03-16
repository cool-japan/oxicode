#![cfg(feature = "compression-lz4")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// ── MARC Bibliographic Record ──────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarcSubfield {
    code: char,
    value: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarcDataField {
    tag: String,
    indicator1: char,
    indicator2: char,
    subfields: Vec<MarcSubfield>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarcControlField {
    tag: String,
    value: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarcRecord {
    leader: String,
    control_fields: Vec<MarcControlField>,
    data_fields: Vec<MarcDataField>,
}

#[test]
fn test_marc_bibliographic_record_lz4() {
    let record = MarcRecord {
        leader: String::from("00942nam a2200301 a 4500"),
        control_fields: vec![
            MarcControlField {
                tag: String::from("001"),
                value: String::from("ocm12345678"),
            },
            MarcControlField {
                tag: String::from("005"),
                value: String::from("20260315120000.0"),
            },
            MarcControlField {
                tag: String::from("008"),
                value: String::from("260101s2026    nyu           000 0 eng d"),
            },
        ],
        data_fields: vec![
            MarcDataField {
                tag: String::from("020"),
                indicator1: ' ',
                indicator2: ' ',
                subfields: vec![MarcSubfield {
                    code: 'a',
                    value: String::from("9780123456789"),
                }],
            },
            MarcDataField {
                tag: String::from("245"),
                indicator1: '1',
                indicator2: '0',
                subfields: vec![
                    MarcSubfield {
                        code: 'a',
                        value: String::from("Introduction to library science /"),
                    },
                    MarcSubfield {
                        code: 'c',
                        value: String::from("by Alice Cataloger."),
                    },
                ],
            },
            MarcDataField {
                tag: String::from("300"),
                indicator1: ' ',
                indicator2: ' ',
                subfields: vec![
                    MarcSubfield {
                        code: 'a',
                        value: String::from("xii, 450 pages :"),
                    },
                    MarcSubfield {
                        code: 'b',
                        value: String::from("illustrations ;"),
                    },
                    MarcSubfield {
                        code: 'c',
                        value: String::from("24 cm."),
                    },
                ],
            },
        ],
    };
    let enc = encode_to_vec(&record).expect("encode marc record");
    let compressed = compress_lz4(&enc).expect("compress marc record");
    let decompressed = decompress_lz4(&compressed).expect("decompress marc record");
    let (decoded, _): (MarcRecord, usize) =
        decode_from_slice(&decompressed).expect("decode marc record");
    assert_eq!(record, decoded);
}

// ── Patron Account ─────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum PatronType {
    Adult,
    Juvenile,
    YoungAdult,
    Faculty,
    GraduateStudent,
    CommunityBorrower,
    InterlibrarySponsor,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PatronAddress {
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PatronAccount {
    barcode: String,
    patron_type: PatronType,
    first_name: String,
    last_name: String,
    email: String,
    address: PatronAddress,
    phone: Option<String>,
    checkout_limit: u16,
    current_checkouts: u16,
    fines_owed_cents: u32,
    blocks: Vec<String>,
    expiration_date: String,
}

#[test]
fn test_patron_account_lz4() {
    let patron = PatronAccount {
        barcode: String::from("21154012345678"),
        patron_type: PatronType::Faculty,
        first_name: String::from("Melvil"),
        last_name: String::from("Dewey"),
        email: String::from("mdewey@library.edu"),
        address: PatronAddress {
            street: String::from("100 Main Library"),
            city: String::from("Albany"),
            state: String::from("NY"),
            zip: String::from("12201"),
            country: String::from("US"),
        },
        phone: Some(String::from("518-555-0100")),
        checkout_limit: 100,
        current_checkouts: 23,
        fines_owed_cents: 0,
        blocks: Vec::new(),
        expiration_date: String::from("2027-08-31"),
    };
    let enc = encode_to_vec(&patron).expect("encode patron account");
    let compressed = compress_lz4(&enc).expect("compress patron account");
    let decompressed = decompress_lz4(&compressed).expect("decompress patron account");
    let (decoded, _): (PatronAccount, usize) =
        decode_from_slice(&decompressed).expect("decode patron account");
    assert_eq!(patron, decoded);
}

// ── Circulation Transaction ────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum CirculationAction {
    Checkout,
    Return,
    Renew,
    ClaimReturned,
    DeclaredLost,
    MarkDamaged,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CirculationTransaction {
    transaction_id: u64,
    item_barcode: String,
    patron_barcode: String,
    action: CirculationAction,
    timestamp_epoch_secs: u64,
    due_date: Option<String>,
    fine_assessed_cents: u32,
    staff_initials: String,
    workstation_id: String,
    override_reason: Option<String>,
}

#[test]
fn test_circulation_checkout_lz4() {
    let txn = CirculationTransaction {
        transaction_id: 9900001,
        item_barcode: String::from("31203045678901"),
        patron_barcode: String::from("21154012345678"),
        action: CirculationAction::Checkout,
        timestamp_epoch_secs: 1773705600,
        due_date: Some(String::from("2026-04-14")),
        fine_assessed_cents: 0,
        staff_initials: String::from("JKL"),
        workstation_id: String::from("CIRC-DESK-01"),
        override_reason: None,
    };
    let enc = encode_to_vec(&txn).expect("encode circulation checkout");
    let compressed = compress_lz4(&enc).expect("compress circulation checkout");
    let decompressed = decompress_lz4(&compressed).expect("decompress circulation checkout");
    let (decoded, _): (CirculationTransaction, usize) =
        decode_from_slice(&decompressed).expect("decode circulation checkout");
    assert_eq!(txn, decoded);
}

#[test]
fn test_circulation_return_and_renew_batch_lz4() {
    let batch = vec![
        CirculationTransaction {
            transaction_id: 9900002,
            item_barcode: String::from("31203045678901"),
            patron_barcode: String::from("21154012345678"),
            action: CirculationAction::Return,
            timestamp_epoch_secs: 1774310400,
            due_date: None,
            fine_assessed_cents: 50,
            staff_initials: String::from("MNO"),
            workstation_id: String::from("SELF-CHECK-03"),
            override_reason: None,
        },
        CirculationTransaction {
            transaction_id: 9900003,
            item_barcode: String::from("31203099887766"),
            patron_barcode: String::from("21154012345678"),
            action: CirculationAction::Renew,
            timestamp_epoch_secs: 1774310500,
            due_date: Some(String::from("2026-05-12")),
            fine_assessed_cents: 0,
            staff_initials: String::from("MNO"),
            workstation_id: String::from("SELF-CHECK-03"),
            override_reason: Some(String::from("Patron in good standing")),
        },
    ];
    let enc = encode_to_vec(&batch).expect("encode circulation batch");
    let compressed = compress_lz4(&enc).expect("compress circulation batch");
    let decompressed = decompress_lz4(&compressed).expect("decompress circulation batch");
    let (decoded, _): (Vec<CirculationTransaction>, usize) =
        decode_from_slice(&decompressed).expect("decode circulation batch");
    assert_eq!(batch, decoded);
}

// ── Hold Queue Management ──────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum HoldStatus {
    Pending,
    InTransit,
    AvailableForPickup,
    Expired,
    Cancelled,
    Suspended,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct HoldRequest {
    hold_id: u64,
    bib_record_id: u64,
    patron_barcode: String,
    pickup_location: String,
    status: HoldStatus,
    queue_position: u16,
    placed_date: String,
    expiration_date: String,
    not_needed_after: Option<String>,
    freeze_until: Option<String>,
    notification_sent: bool,
}

#[test]
fn test_hold_queue_management_lz4() {
    let queue = vec![
        HoldRequest {
            hold_id: 500001,
            bib_record_id: 1234567,
            patron_barcode: String::from("21154000000001"),
            pickup_location: String::from("MAIN-CIRC"),
            status: HoldStatus::AvailableForPickup,
            queue_position: 1,
            placed_date: String::from("2026-02-10"),
            expiration_date: String::from("2026-03-20"),
            not_needed_after: None,
            freeze_until: None,
            notification_sent: true,
        },
        HoldRequest {
            hold_id: 500002,
            bib_record_id: 1234567,
            patron_barcode: String::from("21154000000002"),
            pickup_location: String::from("BRANCH-EAST"),
            status: HoldStatus::Suspended,
            queue_position: 2,
            placed_date: String::from("2026-02-15"),
            expiration_date: String::from("2026-06-15"),
            not_needed_after: Some(String::from("2026-05-01")),
            freeze_until: Some(String::from("2026-04-01")),
            notification_sent: false,
        },
        HoldRequest {
            hold_id: 500003,
            bib_record_id: 1234567,
            patron_barcode: String::from("21154000000003"),
            pickup_location: String::from("MAIN-CIRC"),
            status: HoldStatus::Pending,
            queue_position: 3,
            placed_date: String::from("2026-03-01"),
            expiration_date: String::from("2026-09-01"),
            not_needed_after: None,
            freeze_until: None,
            notification_sent: false,
        },
    ];
    let enc = encode_to_vec(&queue).expect("encode hold queue");
    let compressed = compress_lz4(&enc).expect("compress hold queue");
    let decompressed = decompress_lz4(&compressed).expect("decompress hold queue");
    let (decoded, _): (Vec<HoldRequest>, usize) =
        decode_from_slice(&decompressed).expect("decode hold queue");
    assert_eq!(queue, decoded);
}

// ── Interlibrary Loan Request ──────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum IllStatus {
    NewRequest,
    Searching,
    RequestSentToLender,
    ShippedByLender,
    ReceivedByBorrower,
    CheckedOutToPatron,
    ReturnedByPatron,
    ShippedBackToLender,
    Complete,
    Unfilled,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IllRequest {
    ill_number: String,
    patron_barcode: String,
    title: String,
    author: String,
    isbn: Option<String>,
    issn: Option<String>,
    oclc_number: Option<u64>,
    lending_library_symbol: Option<String>,
    borrowing_library_symbol: String,
    status: IllStatus,
    request_date: String,
    need_before_date: Option<String>,
    shipping_cost_cents: u32,
    copyright_compliance: bool,
    max_cost_cents: u32,
}

#[test]
fn test_ill_request_lz4() {
    let req = IllRequest {
        ill_number: String::from("ILL-2026-003421"),
        patron_barcode: String::from("21154000000010"),
        title: String::from("Rare manuscripts of the medieval period"),
        author: String::from("Thornton, Elizabeth M."),
        isbn: Some(String::from("978-0-555-12345-6")),
        issn: None,
        oclc_number: Some(987654321),
        lending_library_symbol: Some(String::from("NjP")),
        borrowing_library_symbol: String::from("NAlU"),
        status: IllStatus::ShippedByLender,
        request_date: String::from("2026-02-20"),
        need_before_date: Some(String::from("2026-04-15")),
        shipping_cost_cents: 1200,
        copyright_compliance: true,
        max_cost_cents: 5000,
    };
    let enc = encode_to_vec(&req).expect("encode ill request");
    let compressed = compress_lz4(&enc).expect("compress ill request");
    let decompressed = decompress_lz4(&compressed).expect("decompress ill request");
    let (decoded, _): (IllRequest, usize) =
        decode_from_slice(&decompressed).expect("decode ill request");
    assert_eq!(req, decoded);
}

// ── Collection Development Budget ──────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct FundAllocation {
    fund_code: String,
    fund_name: String,
    allocated_cents: u64,
    encumbered_cents: u64,
    expended_cents: u64,
    fiscal_year: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CollectionBudget {
    department: String,
    selector_name: String,
    fund_allocations: Vec<FundAllocation>,
    approval_plan_active: bool,
    standing_order_count: u32,
}

#[test]
fn test_collection_development_budget_lz4() {
    let budget = CollectionBudget {
        department: String::from("Humanities & Social Sciences"),
        selector_name: String::from("Dr. Chen, Wei-Lin"),
        fund_allocations: vec![
            FundAllocation {
                fund_code: String::from("HSS-MONO-26"),
                fund_name: String::from("HSS Monographs FY2026"),
                allocated_cents: 15000000,
                encumbered_cents: 3200000,
                expended_cents: 8750000,
                fiscal_year: String::from("FY2026"),
            },
            FundAllocation {
                fund_code: String::from("HSS-SER-26"),
                fund_name: String::from("HSS Serials FY2026"),
                allocated_cents: 42000000,
                encumbered_cents: 38000000,
                expended_cents: 38000000,
                fiscal_year: String::from("FY2026"),
            },
            FundAllocation {
                fund_code: String::from("HSS-ELEC-26"),
                fund_name: String::from("HSS Electronic Resources FY2026"),
                allocated_cents: 28500000,
                encumbered_cents: 22000000,
                expended_cents: 19800000,
                fiscal_year: String::from("FY2026"),
            },
        ],
        approval_plan_active: true,
        standing_order_count: 47,
    };
    let enc = encode_to_vec(&budget).expect("encode collection budget");
    let compressed = compress_lz4(&enc).expect("compress collection budget");
    let decompressed = decompress_lz4(&compressed).expect("decompress collection budget");
    let (decoded, _): (CollectionBudget, usize) =
        decode_from_slice(&decompressed).expect("decode collection budget");
    assert_eq!(budget, decoded);
}

// ── Cataloging Workflow ────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum CatalogingStage {
    ReceivingQueue,
    CopySearch,
    OriginalCataloging,
    SubjectAnalysis,
    ClassificationAssignment,
    AuthorityWork,
    QualityReview,
    PhysicalProcessing,
    Complete,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CatalogingWorkItem {
    work_item_id: u64,
    bib_record_id: Option<u64>,
    title_brief: String,
    stage: CatalogingStage,
    assigned_cataloger: Option<String>,
    cataloging_level: String,
    encoding_level: char,
    descriptive_standard: String,
    date_received: String,
    date_completed: Option<String>,
    notes: Vec<String>,
    rush: bool,
}

#[test]
fn test_cataloging_workflow_lz4() {
    let item = CatalogingWorkItem {
        work_item_id: 77001,
        bib_record_id: Some(5500123),
        title_brief: String::from("Machine learning for information retrieval"),
        stage: CatalogingStage::SubjectAnalysis,
        assigned_cataloger: Some(String::from("Garcia, Roberto")),
        cataloging_level: String::from("Full"),
        encoding_level: ' ',
        descriptive_standard: String::from("RDA"),
        date_received: String::from("2026-03-01"),
        date_completed: None,
        notes: vec![
            String::from("Accompanying CD-ROM requires separate record"),
            String::from("LCSH headings need verification against 45th edition"),
        ],
        rush: true,
    };
    let enc = encode_to_vec(&item).expect("encode cataloging work item");
    let compressed = compress_lz4(&enc).expect("compress cataloging work item");
    let decompressed = decompress_lz4(&compressed).expect("decompress cataloging work item");
    let (decoded, _): (CatalogingWorkItem, usize) =
        decode_from_slice(&decompressed).expect("decode cataloging work item");
    assert_eq!(item, decoded);
}

// ── Authority Control Record ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum AuthorityType {
    PersonalName,
    CorporateName,
    ConferenceName,
    UniformTitle,
    TopicalSubject,
    GeographicName,
    GenreForm,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AuthorityRecord {
    authority_id: u64,
    auth_type: AuthorityType,
    established_heading: String,
    cross_references_see_from: Vec<String>,
    cross_references_see_also: Vec<String>,
    source_citation: String,
    lccn: Option<String>,
    creation_date: String,
    last_modified: String,
    undifferentiated: bool,
}

#[test]
fn test_authority_control_record_lz4() {
    let authority = AuthorityRecord {
        authority_id: 8800001,
        auth_type: AuthorityType::PersonalName,
        established_heading: String::from("Twain, Mark, 1835-1910"),
        cross_references_see_from: vec![
            String::from("Clemens, Samuel Langhorne, 1835-1910"),
            String::from("Snodgrass, Quintus Curtius, 1835-1910"),
        ],
        cross_references_see_also: vec![],
        source_citation: String::from("His The adventures of Tom Sawyer, 1876"),
        lccn: Some(String::from("n 79021164")),
        creation_date: String::from("1977-06-15"),
        last_modified: String::from("2025-11-20"),
        undifferentiated: false,
    };
    let enc = encode_to_vec(&authority).expect("encode authority record");
    let compressed = compress_lz4(&enc).expect("compress authority record");
    let decompressed = decompress_lz4(&compressed).expect("decompress authority record");
    let (decoded, _): (AuthorityRecord, usize) =
        decode_from_slice(&decompressed).expect("decode authority record");
    assert_eq!(authority, decoded);
}

// ── Shelf Location Map ─────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShelfRange {
    start_call_number: String,
    end_call_number: String,
    floor: u8,
    aisle: String,
    section: String,
    shelf_start: u8,
    shelf_end: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShelfLocationMap {
    building_name: String,
    classification_scheme: String,
    last_shift_date: String,
    ranges: Vec<ShelfRange>,
    total_linear_feet: u32,
    occupancy_percent: u8,
}

#[test]
fn test_shelf_location_map_lz4() {
    let map = ShelfLocationMap {
        building_name: String::from("Central Library"),
        classification_scheme: String::from("Library of Congress"),
        last_shift_date: String::from("2025-09-15"),
        ranges: vec![
            ShelfRange {
                start_call_number: String::from("A1"),
                end_call_number: String::from("AZ999"),
                floor: 2,
                aisle: String::from("A"),
                section: String::from("01-03"),
                shelf_start: 1,
                shelf_end: 7,
            },
            ShelfRange {
                start_call_number: String::from("B1"),
                end_call_number: String::from("BJ999"),
                floor: 2,
                aisle: String::from("B"),
                section: String::from("01-08"),
                shelf_start: 1,
                shelf_end: 7,
            },
            ShelfRange {
                start_call_number: String::from("QA1"),
                end_call_number: String::from("QA999"),
                floor: 4,
                aisle: String::from("Q"),
                section: String::from("01-12"),
                shelf_start: 1,
                shelf_end: 7,
            },
        ],
        total_linear_feet: 42000,
        occupancy_percent: 87,
    };
    let enc = encode_to_vec(&map).expect("encode shelf location map");
    let compressed = compress_lz4(&enc).expect("compress shelf location map");
    let decompressed = decompress_lz4(&compressed).expect("decompress shelf location map");
    let (decoded, _): (ShelfLocationMap, usize) =
        decode_from_slice(&decompressed).expect("decode shelf location map");
    assert_eq!(map, decoded);
}

// ── Acquisition Order Record ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderStatus {
    OnOrder,
    Claimed,
    ReceivedPartially,
    ReceivedComplete,
    Invoiced,
    Paid,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderType {
    FirmOrder,
    ApprovalPlan,
    StandingOrder,
    Blanket,
    Gift,
    Exchange,
    DepositAccount,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AcquisitionOrder {
    order_number: String,
    vendor_code: String,
    order_type: OrderType,
    status: OrderStatus,
    title: String,
    isbn: Option<String>,
    quantity: u16,
    unit_price_cents: u32,
    currency: String,
    fund_code: String,
    order_date: String,
    expected_receipt_date: Option<String>,
    claiming_interval_days: u16,
    times_claimed: u8,
    requestor: String,
}

#[test]
fn test_acquisition_order_lz4() {
    let order = AcquisitionOrder {
        order_number: String::from("PO-2026-008812"),
        vendor_code: String::from("YANKEE"),
        order_type: OrderType::FirmOrder,
        status: OrderStatus::OnOrder,
        title: String::from("Advances in digital humanities research methods"),
        isbn: Some(String::from("978-3-030-99999-0")),
        quantity: 2,
        unit_price_cents: 8995,
        currency: String::from("USD"),
        fund_code: String::from("HSS-MONO-26"),
        order_date: String::from("2026-03-10"),
        expected_receipt_date: Some(String::from("2026-04-25")),
        claiming_interval_days: 60,
        times_claimed: 0,
        requestor: String::from("Dr. Patel, Anita"),
    };
    let enc = encode_to_vec(&order).expect("encode acquisition order");
    let compressed = compress_lz4(&enc).expect("compress acquisition order");
    let decompressed = decompress_lz4(&compressed).expect("decompress acquisition order");
    let (decoded, _): (AcquisitionOrder, usize) =
        decode_from_slice(&decompressed).expect("decode acquisition order");
    assert_eq!(order, decoded);
}

// ── Serial Subscription Tracking ───────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum Frequency {
    Weekly,
    Biweekly,
    Monthly,
    Bimonthly,
    Quarterly,
    SemiAnnual,
    Annual,
    Irregular,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PredictionPattern {
    enumeration_caption: String,
    chronology_caption: String,
    levels: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SerialSubscription {
    subscription_id: u64,
    issn: String,
    title: String,
    publisher: String,
    frequency: Frequency,
    prediction_pattern: PredictionPattern,
    start_date: String,
    renewal_date: String,
    annual_cost_cents: u64,
    vendor_code: String,
    last_received_volume: u16,
    last_received_issue: u16,
    expected_issues_per_year: u8,
    binding_required: bool,
    check_in_location: String,
}

#[test]
fn test_serial_subscription_lz4() {
    let sub = SerialSubscription {
        subscription_id: 330001,
        issn: String::from("0024-2519"),
        title: String::from("Library Journal"),
        publisher: String::from("Media Source Inc."),
        frequency: Frequency::Monthly,
        prediction_pattern: PredictionPattern {
            enumeration_caption: String::from("v.{vol} no.{iss}"),
            chronology_caption: String::from("{month} {year}"),
            levels: 2,
        },
        start_date: String::from("2024-01-01"),
        renewal_date: String::from("2027-01-01"),
        annual_cost_cents: 16995,
        vendor_code: String::from("EBSCO"),
        last_received_volume: 151,
        last_received_issue: 2,
        expected_issues_per_year: 12,
        binding_required: false,
        check_in_location: String::from("SERIALS-DESK"),
    };
    let enc = encode_to_vec(&sub).expect("encode serial subscription");
    let compressed = compress_lz4(&enc).expect("compress serial subscription");
    let decompressed = decompress_lz4(&compressed).expect("decompress serial subscription");
    let (decoded, _): (SerialSubscription, usize) =
        decode_from_slice(&decompressed).expect("decode serial subscription");
    assert_eq!(sub, decoded);
}

// ── Digital Repository Metadata ────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum DigitalFormat {
    Pdf,
    Tiff,
    Jpeg2000,
    Wav,
    Mp4,
    Epub,
    MarcXml,
    Mets,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AccessLevel {
    OpenAccess,
    Restricted,
    CampusOnly,
    Embargoed,
    DarkArchive,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DigitalObject {
    handle: String,
    title: String,
    creator: Vec<String>,
    date_created: String,
    format: DigitalFormat,
    file_size_bytes: u64,
    checksum_sha256: String,
    access_level: AccessLevel,
    collection_name: String,
    rights_statement: String,
    preservation_level: u8,
    doi: Option<String>,
}

#[test]
fn test_digital_repository_metadata_lz4() {
    let obj = DigitalObject {
        handle: String::from("hdl:2142/12345"),
        title: String::from("Oral history interview with Professor Emerita Johnson"),
        creator: vec![
            String::from("Johnson, Maria L."),
            String::from("Smith, David (interviewer)"),
        ],
        date_created: String::from("2025-06-15"),
        format: DigitalFormat::Wav,
        file_size_bytes: 1_073_741_824,
        checksum_sha256: String::from(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        access_level: AccessLevel::CampusOnly,
        collection_name: String::from("University Oral History Project"),
        rights_statement: String::from("In Copyright - Educational Use Permitted"),
        preservation_level: 3,
        doi: Some(String::from("10.12345/oral-hist.2025.0042")),
    };
    let enc = encode_to_vec(&obj).expect("encode digital object");
    let compressed = compress_lz4(&enc).expect("compress digital object");
    let decompressed = decompress_lz4(&compressed).expect("decompress digital object");
    let (decoded, _): (DigitalObject, usize) =
        decode_from_slice(&decompressed).expect("decode digital object");
    assert_eq!(obj, decoded);
}

// ── RDA Description ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ContentType {
    Text,
    PerformedMusic,
    StillImage,
    CartographicImage,
    ComputerProgram,
    SpokenWord,
    TwoDimensionalMovingImage,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MediaType {
    Unmediated,
    Computer,
    Audio,
    Video,
    Microform,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CarrierType {
    Volume,
    OnlineResource,
    AudioDisc,
    Videodisc,
    MicroficheUnit,
    ComputerDisc,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RdaDescription {
    work_identifier: String,
    preferred_title: String,
    variant_titles: Vec<String>,
    content_type: ContentType,
    media_type: MediaType,
    carrier_type: CarrierType,
    extent: String,
    dimensions: String,
    language_of_expression: String,
    subject_headings: Vec<String>,
    classification_lcc: Option<String>,
    classification_ddc: Option<String>,
}

#[test]
fn test_rda_description_lz4() {
    let desc = RdaDescription {
        work_identifier: String::from("http://id.loc.gov/resources/works/c000012345"),
        preferred_title: String::from("Don Quixote"),
        variant_titles: vec![
            String::from("El ingenioso hidalgo don Quixote de la Mancha"),
            String::from("Don Quijote"),
        ],
        content_type: ContentType::Text,
        media_type: MediaType::Unmediated,
        carrier_type: CarrierType::Volume,
        extent: String::from("2 volumes (xxiv, 478; xvi, 532 pages)"),
        dimensions: String::from("22 cm"),
        language_of_expression: String::from("spa"),
        subject_headings: vec![
            String::from("Knights and knighthood--Spain--Fiction"),
            String::from("Spain--Social life and customs--16th century--Fiction"),
        ],
        classification_lcc: Some(String::from("PQ6323")),
        classification_ddc: Some(String::from("863/.3")),
    };
    let enc = encode_to_vec(&desc).expect("encode rda description");
    let compressed = compress_lz4(&enc).expect("compress rda description");
    let decompressed = decompress_lz4(&compressed).expect("decompress rda description");
    let (decoded, _): (RdaDescription, usize) =
        decode_from_slice(&decompressed).expect("decode rda description");
    assert_eq!(desc, decoded);
}

// ── Course Reserve List ────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ReserveMaterialType {
    PhysicalBook,
    EBook,
    JournalArticle,
    BookChapter,
    VideoStreaming,
    AudioRecording,
    CoursePacket,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReserveItem {
    item_id: u64,
    material_type: ReserveMaterialType,
    title: String,
    author: String,
    pages_or_chapters: Option<String>,
    copyright_cleared: bool,
    electronic_link: Option<String>,
    loan_period_hours: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CourseReserve {
    course_code: String,
    course_title: String,
    instructor_name: String,
    department: String,
    semester: String,
    enrollment: u16,
    items: Vec<ReserveItem>,
}

#[test]
fn test_course_reserve_list_lz4() {
    let reserve = CourseReserve {
        course_code: String::from("LIS-501"),
        course_title: String::from("Introduction to Information Science"),
        instructor_name: String::from("Prof. Ranganathan, Shiyali"),
        department: String::from("Library and Information Science"),
        semester: String::from("Spring 2026"),
        enrollment: 35,
        items: vec![
            ReserveItem {
                item_id: 660001,
                material_type: ReserveMaterialType::PhysicalBook,
                title: String::from("The organization of information"),
                author: String::from("Taylor, Arlene G."),
                pages_or_chapters: None,
                copyright_cleared: true,
                electronic_link: None,
                loan_period_hours: 2,
            },
            ReserveItem {
                item_id: 660002,
                material_type: ReserveMaterialType::JournalArticle,
                title: String::from("Facets of information retrieval"),
                author: String::from("Lancaster, F. Wilfrid"),
                pages_or_chapters: Some(String::from("pp. 112-134")),
                copyright_cleared: true,
                electronic_link: Some(String::from("https://doi.org/10.1234/example")),
                loan_period_hours: 0,
            },
        ],
    };
    let enc = encode_to_vec(&reserve).expect("encode course reserve");
    let compressed = compress_lz4(&enc).expect("compress course reserve");
    let decompressed = decompress_lz4(&compressed).expect("decompress course reserve");
    let (decoded, _): (CourseReserve, usize) =
        decode_from_slice(&decompressed).expect("decode course reserve");
    assert_eq!(reserve, decoded);
}

// ── Weeding / Deselection Candidate ────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum WeedingCriterion {
    LowCirculation,
    PoorCondition,
    Superseded,
    DuplicateCopy,
    OutOfScope,
    DamagedBeyondRepair,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WeedingDisposition {
    Withdraw,
    Donate,
    Recycle,
    TransferToStorage,
    RetainAfterReview,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WeedingCandidate {
    item_barcode: String,
    call_number: String,
    title: String,
    publication_year: u16,
    last_checkout_date: Option<String>,
    total_checkouts: u32,
    criterion: WeedingCriterion,
    disposition: Option<WeedingDisposition>,
    reviewed_by: Option<String>,
    review_date: Option<String>,
    replacement_isbn: Option<String>,
}

#[test]
fn test_weeding_candidates_lz4() {
    let candidates = vec![
        WeedingCandidate {
            item_barcode: String::from("31203000111222"),
            call_number: String::from("QA76.73.J38 S55 2005"),
            title: String::from("Java programming fundamentals, 3rd ed."),
            publication_year: 2005,
            last_checkout_date: Some(String::from("2018-09-02")),
            total_checkouts: 12,
            criterion: WeedingCriterion::Superseded,
            disposition: Some(WeedingDisposition::Withdraw),
            reviewed_by: Some(String::from("Kim, Soo-Jin")),
            review_date: Some(String::from("2026-02-28")),
            replacement_isbn: Some(String::from("978-0-13-468599-1")),
        },
        WeedingCandidate {
            item_barcode: String::from("31203000333444"),
            call_number: String::from("PR6019.O9 U6 1965"),
            title: String::from("Ulysses"),
            publication_year: 1965,
            last_checkout_date: Some(String::from("2025-12-01")),
            total_checkouts: 89,
            criterion: WeedingCriterion::PoorCondition,
            disposition: Some(WeedingDisposition::RetainAfterReview),
            reviewed_by: Some(String::from("O'Brien, Patrick")),
            review_date: Some(String::from("2026-03-01")),
            replacement_isbn: None,
        },
    ];
    let enc = encode_to_vec(&candidates).expect("encode weeding candidates");
    let compressed = compress_lz4(&enc).expect("compress weeding candidates");
    let decompressed = decompress_lz4(&compressed).expect("decompress weeding candidates");
    let (decoded, _): (Vec<WeedingCandidate>, usize) =
        decode_from_slice(&decompressed).expect("decode weeding candidates");
    assert_eq!(candidates, decoded);
}

// ── Reference Transaction Log ──────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ReferenceMode {
    InPerson,
    Phone,
    Email,
    Chat,
    TextMessage,
    VideoConference,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum QuestionComplexity {
    Directional,
    ReadyReference,
    SpecificSearch,
    ResearchConsultation,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReferenceTransaction {
    transaction_id: u64,
    timestamp_epoch_secs: u64,
    mode: ReferenceMode,
    complexity: QuestionComplexity,
    duration_minutes: u16,
    patron_type_code: String,
    subject_area: String,
    databases_consulted: Vec<String>,
    referral_made: bool,
    follow_up_needed: bool,
    librarian_initials: String,
}

#[test]
fn test_reference_transaction_log_lz4() {
    let txn = ReferenceTransaction {
        transaction_id: 4400001,
        timestamp_epoch_secs: 1773790800,
        mode: ReferenceMode::Chat,
        complexity: QuestionComplexity::ResearchConsultation,
        duration_minutes: 45,
        patron_type_code: String::from("GRAD"),
        subject_area: String::from("Digital Humanities / Text Mining"),
        databases_consulted: vec![
            String::from("JSTOR"),
            String::from("MLA International Bibliography"),
            String::from("ProQuest Dissertations"),
        ],
        referral_made: false,
        follow_up_needed: true,
        librarian_initials: String::from("AKT"),
    };
    let enc = encode_to_vec(&txn).expect("encode reference transaction");
    let compressed = compress_lz4(&enc).expect("compress reference transaction");
    let decompressed = decompress_lz4(&compressed).expect("decompress reference transaction");
    let (decoded, _): (ReferenceTransaction, usize) =
        decode_from_slice(&decompressed).expect("decode reference transaction");
    assert_eq!(txn, decoded);
}

// ── Preservation Assessment ────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum BindingType {
    PerfectBound,
    CaseBound,
    SaddleStitched,
    SpiralBound,
    Pamphlet,
    Vellum,
    LeatherBound,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PaperCondition {
    Excellent,
    Good,
    Fair,
    Brittle,
    Fragmentary,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PreservationAction {
    None,
    Rebind,
    Encapsulate,
    Deacidify,
    Digitize,
    BoxForStorage,
    ConservationRepair,
    Microfilm,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PreservationAssessment {
    item_barcode: String,
    call_number: String,
    binding_type: BindingType,
    paper_condition: PaperCondition,
    ph_level: Option<u8>,
    mold_present: bool,
    pest_damage: bool,
    water_damage: bool,
    red_rot: bool,
    recommended_action: PreservationAction,
    estimated_cost_cents: u32,
    priority_score: u8,
    assessor: String,
    assessment_date: String,
}

#[test]
fn test_preservation_assessment_lz4() {
    let assessment = PreservationAssessment {
        item_barcode: String::from("31203000555666"),
        call_number: String::from("Z4 .B63 1542"),
        binding_type: BindingType::LeatherBound,
        paper_condition: PaperCondition::Brittle,
        ph_level: Some(4),
        mold_present: false,
        pest_damage: true,
        water_damage: false,
        red_rot: true,
        recommended_action: PreservationAction::ConservationRepair,
        estimated_cost_cents: 250000,
        priority_score: 9,
        assessor: String::from("Martinez, Isabella"),
        assessment_date: String::from("2026-01-22"),
    };
    let enc = encode_to_vec(&assessment).expect("encode preservation assessment");
    let compressed = compress_lz4(&enc).expect("compress preservation assessment");
    let decompressed = decompress_lz4(&compressed).expect("decompress preservation assessment");
    let (decoded, _): (PreservationAssessment, usize) =
        decode_from_slice(&decompressed).expect("decode preservation assessment");
    assert_eq!(assessment, decoded);
}

// ── Space Usage Statistics ─────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct HourlyCount {
    hour: u8,
    head_count: u16,
    workstations_in_use: u16,
    study_rooms_occupied: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DailySpaceUsage {
    date: String,
    building_code: String,
    floor: u8,
    gate_count_entries: u32,
    gate_count_exits: u32,
    hourly_snapshots: Vec<HourlyCount>,
    wifi_device_peak: u32,
    events_scheduled: u8,
}

#[test]
fn test_space_usage_statistics_lz4() {
    let usage = DailySpaceUsage {
        date: String::from("2026-03-14"),
        building_code: String::from("MAIN"),
        floor: 1,
        gate_count_entries: 3421,
        gate_count_exits: 3398,
        hourly_snapshots: vec![
            HourlyCount {
                hour: 8,
                head_count: 45,
                workstations_in_use: 12,
                study_rooms_occupied: 2,
            },
            HourlyCount {
                hour: 12,
                head_count: 287,
                workstations_in_use: 48,
                study_rooms_occupied: 10,
            },
            HourlyCount {
                hour: 18,
                head_count: 412,
                workstations_in_use: 50,
                study_rooms_occupied: 12,
            },
            HourlyCount {
                hour: 22,
                head_count: 156,
                workstations_in_use: 38,
                study_rooms_occupied: 8,
            },
        ],
        wifi_device_peak: 1820,
        events_scheduled: 3,
    };
    let enc = encode_to_vec(&usage).expect("encode space usage");
    let compressed = compress_lz4(&enc).expect("compress space usage");
    let decompressed = decompress_lz4(&compressed).expect("decompress space usage");
    let (decoded, _): (DailySpaceUsage, usize) =
        decode_from_slice(&decompressed).expect("decode space usage");
    assert_eq!(usage, decoded);
}

// ── ERM (Electronic Resource Management) License ───────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum LicensePermission {
    Permitted,
    Prohibited,
    Silent,
    Negotiable,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LicenseTerms {
    ill_print: LicensePermission,
    ill_electronic: LicensePermission,
    course_reserves: LicensePermission,
    text_mining: LicensePermission,
    scholarly_sharing: LicensePermission,
    walk_in_access: LicensePermission,
    perpetual_access: bool,
    archival_rights: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ElectronicResourceLicense {
    license_id: String,
    resource_name: String,
    vendor: String,
    start_date: String,
    end_date: String,
    annual_fee_cents: u64,
    fte_tier: String,
    simultaneous_users: Option<u32>,
    terms: LicenseTerms,
    sushi_counter_compliant: bool,
    openurl_resolver_enabled: bool,
    admin_contact_email: String,
}

#[test]
fn test_erm_license_lz4() {
    let license = ElectronicResourceLicense {
        license_id: String::from("LIC-2026-00042"),
        resource_name: String::from("Academic Search Ultimate"),
        vendor: String::from("EBSCO"),
        start_date: String::from("2026-01-01"),
        end_date: String::from("2026-12-31"),
        annual_fee_cents: 8500000,
        fte_tier: String::from("10001-20000"),
        simultaneous_users: None,
        terms: LicenseTerms {
            ill_print: LicensePermission::Permitted,
            ill_electronic: LicensePermission::Prohibited,
            course_reserves: LicensePermission::Permitted,
            text_mining: LicensePermission::Negotiable,
            scholarly_sharing: LicensePermission::Permitted,
            walk_in_access: LicensePermission::Permitted,
            perpetual_access: true,
            archival_rights: true,
        },
        sushi_counter_compliant: true,
        openurl_resolver_enabled: true,
        admin_contact_email: String::from("eresources@library.edu"),
    };
    let enc = encode_to_vec(&license).expect("encode erm license");
    let compressed = compress_lz4(&enc).expect("compress erm license");
    let decompressed = decompress_lz4(&compressed).expect("decompress erm license");
    let (decoded, _): (ElectronicResourceLicense, usize) =
        decode_from_slice(&decompressed).expect("decode erm license");
    assert_eq!(license, decoded);
}

// ── MARC Holdings Record ───────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct HoldingsStatement {
    field_tag: String,
    textual_holdings: String,
    note: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarcHoldings {
    holdings_id: u64,
    bib_record_id: u64,
    location_code: String,
    call_number_prefix: Option<String>,
    call_number: String,
    call_number_suffix: Option<String>,
    copy_number: u8,
    statements: Vec<HoldingsStatement>,
    gaps: Vec<String>,
    retention_policy: String,
}

#[test]
fn test_marc_holdings_record_lz4() {
    let holdings = MarcHoldings {
        holdings_id: 2200001,
        bib_record_id: 1100500,
        location_code: String::from("MAIN-SER"),
        call_number_prefix: Some(String::from("Per")),
        call_number: String::from("Z671 .L7"),
        call_number_suffix: None,
        copy_number: 1,
        statements: vec![
            HoldingsStatement {
                field_tag: String::from("866"),
                textual_holdings: String::from("v.1 (1876)-v.145 (2020)"),
                note: Some(String::from(
                    "Print subscription cancelled 2020; online access continues",
                )),
            },
            HoldingsStatement {
                field_tag: String::from("867"),
                textual_holdings: String::from("v.1-v.100 supplements"),
                note: None,
            },
        ],
        gaps: vec![
            String::from("v.42 no.3 (1917) - missing"),
            String::from("v.68 no.11 (1943) - missing"),
        ],
        retention_policy: String::from("Permanently retained"),
    };
    let enc = encode_to_vec(&holdings).expect("encode marc holdings");
    let compressed = compress_lz4(&enc).expect("compress marc holdings");
    let decompressed = decompress_lz4(&compressed).expect("decompress marc holdings");
    let (decoded, _): (MarcHoldings, usize) =
        decode_from_slice(&decompressed).expect("decode marc holdings");
    assert_eq!(holdings, decoded);
}

// ── Batch MARC Import Job ──────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum ImportOutcome {
    Created,
    Overlaid,
    Skipped,
    ErrorInvalidLeader,
    ErrorDuplicateDetected,
    ErrorEncodingMismatch,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ImportRecordResult {
    record_sequence: u32,
    oclc_number: Option<u64>,
    outcome: ImportOutcome,
    bib_id_assigned: Option<u64>,
    message: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BatchImportJob {
    job_id: String,
    file_name: String,
    total_records: u32,
    records_processed: u32,
    profile_name: String,
    match_point: String,
    overlay_action: String,
    results: Vec<ImportRecordResult>,
    start_time_epoch_secs: u64,
    end_time_epoch_secs: Option<u64>,
}

#[test]
fn test_batch_marc_import_job_lz4() {
    let job = BatchImportJob {
        job_id: String::from("IMPORT-20260315-001"),
        file_name: String::from("oclc_worldcat_extract_20260310.mrc"),
        total_records: 5,
        records_processed: 5,
        profile_name: String::from("OCLC-Overlay-Full"),
        match_point: String::from("OCLC Number (035$a)"),
        overlay_action: String::from("Replace except call number"),
        results: vec![
            ImportRecordResult {
                record_sequence: 1,
                oclc_number: Some(12345678),
                outcome: ImportOutcome::Overlaid,
                bib_id_assigned: Some(1100501),
                message: None,
            },
            ImportRecordResult {
                record_sequence: 2,
                oclc_number: Some(23456789),
                outcome: ImportOutcome::Created,
                bib_id_assigned: Some(1100502),
                message: None,
            },
            ImportRecordResult {
                record_sequence: 3,
                oclc_number: None,
                outcome: ImportOutcome::ErrorInvalidLeader,
                bib_id_assigned: None,
                message: Some(String::from("Leader byte 06 has invalid value 'x'")),
            },
            ImportRecordResult {
                record_sequence: 4,
                oclc_number: Some(34567890),
                outcome: ImportOutcome::Skipped,
                bib_id_assigned: None,
                message: Some(String::from("Record suppressed by profile filter")),
            },
            ImportRecordResult {
                record_sequence: 5,
                oclc_number: Some(45678901),
                outcome: ImportOutcome::Overlaid,
                bib_id_assigned: Some(1100503),
                message: None,
            },
        ],
        start_time_epoch_secs: 1773792000,
        end_time_epoch_secs: Some(1773792045),
    };
    let enc = encode_to_vec(&job).expect("encode batch import job");
    let compressed = compress_lz4(&enc).expect("compress batch import job");
    let decompressed = decompress_lz4(&compressed).expect("decompress batch import job");
    let (decoded, _): (BatchImportJob, usize) =
        decode_from_slice(&decompressed).expect("decode batch import job");
    assert_eq!(job, decoded);
}
