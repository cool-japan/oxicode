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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RoomType {
    Single,
    Double,
    Suite,
    Penthouse,
    Family,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReservationStatus {
    Pending,
    Confirmed,
    CheckedIn,
    CheckedOut,
    Cancelled,
    NoShow,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AmenityCategory {
    Dining,
    Spa,
    Fitness,
    Pool,
    Business,
    Parking,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PaymentMethod {
    Cash,
    Card,
    Transfer,
    Loyalty,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Room {
    room_id: u32,
    hotel_id: u32,
    room_type: RoomType,
    floor: u8,
    capacity: u8,
    rate_cents: u32,
    available: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Guest {
    guest_id: u64,
    name: String,
    email: String,
    loyalty_points: u32,
    preferred_room: Option<RoomType>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Reservation {
    reservation_id: u64,
    guest_id: u64,
    hotel_id: u32,
    room_id: u32,
    check_in: u64,
    check_out: u64,
    status: ReservationStatus,
    total_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HotelAmenity {
    amenity_id: u32,
    hotel_id: u32,
    name: String,
    category: AmenityCategory,
    price_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Payment {
    payment_id: u64,
    reservation_id: u64,
    amount_cents: u32,
    method: PaymentMethod,
    processed_at: u64,
    confirmed: bool,
}

// Test 1: Room roundtrip with standard config
#[test]
fn test_room_roundtrip_standard() {
    let room = Room {
        room_id: 101,
        hotel_id: 1,
        room_type: RoomType::Single,
        floor: 1,
        capacity: 1,
        rate_cents: 8900,
        available: true,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&room, cfg).expect("encode room standard");
    let (decoded, consumed): (Room, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode room standard");
    assert_eq!(room, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 2: Room roundtrip with big endian config
#[test]
fn test_room_roundtrip_big_endian() {
    let room = Room {
        room_id: 202,
        hotel_id: 2,
        room_type: RoomType::Double,
        floor: 2,
        capacity: 2,
        rate_cents: 12500,
        available: false,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&room, cfg).expect("encode room big endian");
    let (decoded, consumed): (Room, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode room big endian");
    assert_eq!(room, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 3: Room roundtrip with fixed int encoding
#[test]
fn test_room_roundtrip_fixed_int() {
    let room = Room {
        room_id: 303,
        hotel_id: 3,
        room_type: RoomType::Suite,
        floor: 5,
        capacity: 3,
        rate_cents: 35000,
        available: true,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&room, cfg).expect("encode room fixed int");
    let (decoded, consumed): (Room, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode room fixed int");
    assert_eq!(room, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 4: Guest with preferred_room Some(RoomType::Suite)
#[test]
fn test_guest_preferred_room_some() {
    let guest = Guest {
        guest_id: 10001,
        name: "Alice Yamamoto".to_string(),
        email: "alice@example.com".to_string(),
        loyalty_points: 4200,
        preferred_room: Some(RoomType::Suite),
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&guest, cfg).expect("encode guest with preferred_room some");
    let (decoded, consumed): (Guest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode guest with preferred_room some");
    assert_eq!(guest, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 5: Guest with preferred_room None
#[test]
fn test_guest_preferred_room_none() {
    let guest = Guest {
        guest_id: 10002,
        name: "Bob Tanaka".to_string(),
        email: "bob@example.com".to_string(),
        loyalty_points: 0,
        preferred_room: None,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&guest, cfg).expect("encode guest with preferred_room none");
    let (decoded, consumed): (Guest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode guest with preferred_room none");
    assert_eq!(guest, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 6: Guest with Penthouse preference and big endian
#[test]
fn test_guest_penthouse_preference_big_endian() {
    let guest = Guest {
        guest_id: 99999,
        name: "Carlos Fernandez".to_string(),
        email: "carlos@luxury.com".to_string(),
        loyalty_points: 100000,
        preferred_room: Some(RoomType::Penthouse),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&guest, cfg).expect("encode guest penthouse big endian");
    let (decoded, consumed): (Guest, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode guest penthouse big endian");
    assert_eq!(guest, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 7: Reservation with Confirmed status
#[test]
fn test_reservation_confirmed_standard() {
    let reservation = Reservation {
        reservation_id: 500001,
        guest_id: 10001,
        hotel_id: 1,
        room_id: 303,
        check_in: 1_750_000_000,
        check_out: 1_750_259_200,
        status: ReservationStatus::Confirmed,
        total_cents: 70000,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&reservation, cfg).expect("encode reservation confirmed");
    let (decoded, consumed): (Reservation, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode reservation confirmed");
    assert_eq!(reservation, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 8: Reservation with CheckedIn status and fixed int
#[test]
fn test_reservation_checked_in_fixed_int() {
    let reservation = Reservation {
        reservation_id: 500002,
        guest_id: 10002,
        hotel_id: 2,
        room_id: 202,
        check_in: 1_751_000_000,
        check_out: 1_751_086_400,
        status: ReservationStatus::CheckedIn,
        total_cents: 12500,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&reservation, cfg).expect("encode reservation checked in fixed int");
    let (decoded, consumed): (Reservation, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode reservation checked in fixed int");
    assert_eq!(reservation, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 9: Reservation with Cancelled status and big endian
#[test]
fn test_reservation_cancelled_big_endian() {
    let reservation = Reservation {
        reservation_id: 500003,
        guest_id: 10003,
        hotel_id: 1,
        room_id: 101,
        check_in: 1_752_000_000,
        check_out: 1_752_172_800,
        status: ReservationStatus::Cancelled,
        total_cents: 0,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&reservation, cfg).expect("encode reservation cancelled big endian");
    let (decoded, consumed): (Reservation, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode reservation cancelled big endian");
    assert_eq!(reservation, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 10: HotelAmenity Dining category
#[test]
fn test_hotel_amenity_dining_standard() {
    let amenity = HotelAmenity {
        amenity_id: 1001,
        hotel_id: 1,
        name: "The Grand Restaurant".to_string(),
        category: AmenityCategory::Dining,
        price_cents: 0,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&amenity, cfg).expect("encode amenity dining");
    let (decoded, consumed): (HotelAmenity, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode amenity dining");
    assert_eq!(amenity, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 11: HotelAmenity Spa category with big endian
#[test]
fn test_hotel_amenity_spa_big_endian() {
    let amenity = HotelAmenity {
        amenity_id: 1002,
        hotel_id: 1,
        name: "Zen Wellness Spa".to_string(),
        category: AmenityCategory::Spa,
        price_cents: 15000,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&amenity, cfg).expect("encode amenity spa big endian");
    let (decoded, consumed): (HotelAmenity, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode amenity spa big endian");
    assert_eq!(amenity, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 12: HotelAmenity Pool category with fixed int
#[test]
fn test_hotel_amenity_pool_fixed_int() {
    let amenity = HotelAmenity {
        amenity_id: 1003,
        hotel_id: 2,
        name: "Infinity Pool".to_string(),
        category: AmenityCategory::Pool,
        price_cents: 3000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&amenity, cfg).expect("encode amenity pool fixed int");
    let (decoded, consumed): (HotelAmenity, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode amenity pool fixed int");
    assert_eq!(amenity, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 13: Payment with Card method standard config
#[test]
fn test_payment_card_standard() {
    let payment = Payment {
        payment_id: 900001,
        reservation_id: 500001,
        amount_cents: 70000,
        method: PaymentMethod::Card,
        processed_at: 1_750_010_000,
        confirmed: true,
    };
    let cfg = config::standard();
    let bytes = encode_to_vec(&payment, cfg).expect("encode payment card");
    let (decoded, consumed): (Payment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode payment card");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 14: Payment with Loyalty method big endian
#[test]
fn test_payment_loyalty_big_endian() {
    let payment = Payment {
        payment_id: 900002,
        reservation_id: 500002,
        amount_cents: 12500,
        method: PaymentMethod::Loyalty,
        processed_at: 1_751_010_000,
        confirmed: true,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&payment, cfg).expect("encode payment loyalty big endian");
    let (decoded, consumed): (Payment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode payment loyalty big endian");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 15: Payment with Transfer method fixed int, unconfirmed
#[test]
fn test_payment_transfer_unconfirmed_fixed_int() {
    let payment = Payment {
        payment_id: 900003,
        reservation_id: 500003,
        amount_cents: 50000,
        method: PaymentMethod::Transfer,
        processed_at: 1_752_010_000,
        confirmed: false,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&payment, cfg).expect("encode payment transfer fixed int");
    let (decoded, consumed): (Payment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode payment transfer fixed int");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 16: Vec<Room> roundtrip standard config
#[test]
fn test_vec_rooms_roundtrip_standard() {
    let rooms = vec![
        Room {
            room_id: 101,
            hotel_id: 1,
            room_type: RoomType::Single,
            floor: 1,
            capacity: 1,
            rate_cents: 8900,
            available: true,
        },
        Room {
            room_id: 201,
            hotel_id: 1,
            room_type: RoomType::Double,
            floor: 2,
            capacity: 2,
            rate_cents: 14000,
            available: true,
        },
        Room {
            room_id: 501,
            hotel_id: 1,
            room_type: RoomType::Suite,
            floor: 5,
            capacity: 3,
            rate_cents: 35000,
            available: false,
        },
        Room {
            room_id: 1001,
            hotel_id: 1,
            room_type: RoomType::Penthouse,
            floor: 10,
            capacity: 4,
            rate_cents: 120000,
            available: true,
        },
        Room {
            room_id: 301,
            hotel_id: 1,
            room_type: RoomType::Family,
            floor: 3,
            capacity: 5,
            rate_cents: 22000,
            available: true,
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&rooms, cfg).expect("encode vec rooms standard");
    let (decoded, consumed): (Vec<Room>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec rooms standard");
    assert_eq!(rooms, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 17: Vec<Guest> roundtrip big endian
#[test]
fn test_vec_guests_roundtrip_big_endian() {
    let guests = vec![
        Guest {
            guest_id: 1,
            name: "Yuki Sato".to_string(),
            email: "yuki@hotel.jp".to_string(),
            loyalty_points: 500,
            preferred_room: Some(RoomType::Single),
        },
        Guest {
            guest_id: 2,
            name: "Marco Rossi".to_string(),
            email: "marco@example.it".to_string(),
            loyalty_points: 12000,
            preferred_room: Some(RoomType::Double),
        },
        Guest {
            guest_id: 3,
            name: "Priya Patel".to_string(),
            email: "priya@example.in".to_string(),
            loyalty_points: 0,
            preferred_room: None,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&guests, cfg).expect("encode vec guests big endian");
    let (decoded, consumed): (Vec<Guest>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec guests big endian");
    assert_eq!(guests, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 18: Vec<Reservation> with all statuses
#[test]
fn test_vec_reservations_all_statuses() {
    let reservations = vec![
        Reservation {
            reservation_id: 1,
            guest_id: 1,
            hotel_id: 1,
            room_id: 101,
            check_in: 1_750_000_000,
            check_out: 1_750_086_400,
            status: ReservationStatus::Pending,
            total_cents: 8900,
        },
        Reservation {
            reservation_id: 2,
            guest_id: 2,
            hotel_id: 1,
            room_id: 201,
            check_in: 1_750_100_000,
            check_out: 1_750_272_800,
            status: ReservationStatus::Confirmed,
            total_cents: 28000,
        },
        Reservation {
            reservation_id: 3,
            guest_id: 3,
            hotel_id: 1,
            room_id: 501,
            check_in: 1_750_200_000,
            check_out: 1_750_459_200,
            status: ReservationStatus::CheckedIn,
            total_cents: 105000,
        },
        Reservation {
            reservation_id: 4,
            guest_id: 4,
            hotel_id: 2,
            room_id: 301,
            check_in: 1_749_000_000,
            check_out: 1_749_172_800,
            status: ReservationStatus::CheckedOut,
            total_cents: 44000,
        },
        Reservation {
            reservation_id: 5,
            guest_id: 5,
            hotel_id: 2,
            room_id: 202,
            check_in: 1_748_000_000,
            check_out: 1_748_086_400,
            status: ReservationStatus::Cancelled,
            total_cents: 0,
        },
        Reservation {
            reservation_id: 6,
            guest_id: 6,
            hotel_id: 1,
            room_id: 101,
            check_in: 1_747_000_000,
            check_out: 1_747_086_400,
            status: ReservationStatus::NoShow,
            total_cents: 8900,
        },
    ];
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&reservations, cfg).expect("encode vec reservations all statuses");
    let (decoded, consumed): (Vec<Reservation>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec reservations all statuses");
    assert_eq!(reservations, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 19: Vec<HotelAmenity> all categories roundtrip
#[test]
fn test_vec_amenities_all_categories() {
    let amenities = vec![
        HotelAmenity {
            amenity_id: 1,
            hotel_id: 1,
            name: "Bistro 88".to_string(),
            category: AmenityCategory::Dining,
            price_cents: 0,
        },
        HotelAmenity {
            amenity_id: 2,
            hotel_id: 1,
            name: "Serenity Spa".to_string(),
            category: AmenityCategory::Spa,
            price_cents: 20000,
        },
        HotelAmenity {
            amenity_id: 3,
            hotel_id: 1,
            name: "FitLife Gym".to_string(),
            category: AmenityCategory::Fitness,
            price_cents: 1000,
        },
        HotelAmenity {
            amenity_id: 4,
            hotel_id: 1,
            name: "Rooftop Pool".to_string(),
            category: AmenityCategory::Pool,
            price_cents: 5000,
        },
        HotelAmenity {
            amenity_id: 5,
            hotel_id: 1,
            name: "Executive Boardroom".to_string(),
            category: AmenityCategory::Business,
            price_cents: 50000,
        },
        HotelAmenity {
            amenity_id: 6,
            hotel_id: 1,
            name: "Underground Parking".to_string(),
            category: AmenityCategory::Parking,
            price_cents: 2500,
        },
    ];
    let cfg = config::standard();
    let bytes = encode_to_vec(&amenities, cfg).expect("encode vec amenities all categories");
    let (decoded, consumed): (Vec<HotelAmenity>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec amenities all categories");
    assert_eq!(amenities, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 20: Vec<Payment> all payment methods roundtrip
#[test]
fn test_vec_payments_all_methods() {
    let payments = vec![
        Payment {
            payment_id: 1,
            reservation_id: 100,
            amount_cents: 9000,
            method: PaymentMethod::Cash,
            processed_at: 1_750_000_100,
            confirmed: true,
        },
        Payment {
            payment_id: 2,
            reservation_id: 101,
            amount_cents: 14000,
            method: PaymentMethod::Card,
            processed_at: 1_750_000_200,
            confirmed: true,
        },
        Payment {
            payment_id: 3,
            reservation_id: 102,
            amount_cents: 35000,
            method: PaymentMethod::Transfer,
            processed_at: 1_750_000_300,
            confirmed: false,
        },
        Payment {
            payment_id: 4,
            reservation_id: 103,
            amount_cents: 5000,
            method: PaymentMethod::Loyalty,
            processed_at: 1_750_000_400,
            confirmed: true,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec(&payments, cfg).expect("encode vec payments all methods");
    let (decoded, consumed): (Vec<Payment>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode vec payments all methods");
    assert_eq!(payments, decoded);
    assert_eq!(consumed, bytes.len());
}

// Test 21: consumed bytes is non-zero and matches encoded length for complex struct
#[test]
fn test_consumed_bytes_matches_encoded_length() {
    let reservation = Reservation {
        reservation_id: 777777,
        guest_id: 333333,
        hotel_id: 5,
        room_id: 1001,
        check_in: 1_760_000_000,
        check_out: 1_760_604_800,
        status: ReservationStatus::CheckedOut,
        total_cents: 840000,
    };
    let cfg = config::standard();
    let bytes =
        encode_to_vec(&reservation, cfg).expect("encode reservation for consumed bytes check");
    assert!(!bytes.is_empty(), "encoded bytes should be non-empty");
    let (decoded, consumed): (Reservation, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode reservation for consumed bytes check");
    assert_eq!(reservation, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
    assert!(consumed > 0, "consumed bytes must be positive");
}

// Test 22: Family room with all three configs produce different bytes but identical decoded values
#[test]
fn test_family_room_three_configs_identical_decode() {
    let room = Room {
        room_id: 401,
        hotel_id: 4,
        room_type: RoomType::Family,
        floor: 4,
        capacity: 5,
        rate_cents: 22000,
        available: true,
    };

    let cfg_std = config::standard();
    let cfg_be = config::standard().with_big_endian();
    let cfg_fi = config::standard().with_fixed_int_encoding();

    let bytes_std = encode_to_vec(&room, cfg_std).expect("encode family room standard");
    let bytes_be = encode_to_vec(&room, cfg_be).expect("encode family room big endian");
    let bytes_fi = encode_to_vec(&room, cfg_fi).expect("encode family room fixed int");

    let (decoded_std, consumed_std): (Room, usize) =
        decode_owned_from_slice(&bytes_std, cfg_std).expect("decode family room standard");
    let (decoded_be, consumed_be): (Room, usize) =
        decode_owned_from_slice(&bytes_be, cfg_be).expect("decode family room big endian");
    let (decoded_fi, consumed_fi): (Room, usize) =
        decode_owned_from_slice(&bytes_fi, cfg_fi).expect("decode family room fixed int");

    assert_eq!(room, decoded_std);
    assert_eq!(room, decoded_be);
    assert_eq!(room, decoded_fi);

    assert_eq!(consumed_std, bytes_std.len());
    assert_eq!(consumed_be, bytes_be.len());
    assert_eq!(consumed_fi, bytes_fi.len());
}
