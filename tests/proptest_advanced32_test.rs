//! Advanced proptest-based tests for oxicode — E-commerce / shopping data theme

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Product {
    id: u64,
    price_cents: u64,
    quantity: u32,
    weight_g: u32,
    is_available: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShippingMethod {
    Standard,
    Express,
    Overnight { extra_cost_cents: u32 },
    Pickup,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CartItem {
    product_id: u64,
    quantity: u32,
    discount_pct: u8,
}

// --- 1. Product roundtrip with various field ranges ---
proptest! {
    #[test]
    fn prop_product_roundtrip(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let val = Product { id, price_cents, quantity, weight_g, is_available };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (Product, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 2. ShippingMethod::Standard roundtrip ---
proptest! {
    #[test]
    fn prop_shipping_standard_roundtrip(_dummy in 0u8..=1u8) {
        let val = ShippingMethod::Standard;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (ShippingMethod, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 3. ShippingMethod::Express roundtrip ---
proptest! {
    #[test]
    fn prop_shipping_express_roundtrip(_dummy in 0u8..=1u8) {
        let val = ShippingMethod::Express;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (ShippingMethod, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 4. ShippingMethod::Overnight roundtrip with u32 extra_cost_cents ---
proptest! {
    #[test]
    fn prop_shipping_overnight_roundtrip(extra_cost_cents in 0u32..=500_00u32) {
        let val = ShippingMethod::Overnight { extra_cost_cents };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (ShippingMethod, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 5. ShippingMethod::Pickup roundtrip ---
proptest! {
    #[test]
    fn prop_shipping_pickup_roundtrip(_dummy in 0u8..=1u8) {
        let val = ShippingMethod::Pickup;
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (ShippingMethod, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 6. CartItem roundtrip ---
proptest! {
    #[test]
    fn prop_cart_item_roundtrip(
        product_id in 0u64..=u64::MAX,
        quantity in 0u32..=1000u32,
        discount_pct in 0u8..=100u8,
    ) {
        let val = CartItem { product_id, quantity, discount_pct };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (CartItem, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 7. Vec<Product> roundtrip (0..8 items) ---
proptest! {
    #[test]
    fn prop_vec_product_roundtrip(
        items in prop::collection::vec(
            (0u64..=u64::MAX, 0u64..=1_000_000_00u64, 0u32..=10_000u32, 0u32..=50_000u32, any::<bool>()),
            0..8
        )
    ) {
        let products: Vec<Product> = items
            .into_iter()
            .map(|(id, price_cents, quantity, weight_g, is_available)| Product {
                id,
                price_cents,
                quantity,
                weight_g,
                is_available,
            })
            .collect();
        let bytes = encode_to_vec(&products).expect("encode failed");
        let (decoded, _): (Vec<Product>, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(products, decoded);
    }
}

// --- 8. Vec<CartItem> roundtrip (0..8 items) ---
proptest! {
    #[test]
    fn prop_vec_cart_item_roundtrip(
        items in prop::collection::vec(
            (0u64..=u64::MAX, 0u32..=1000u32, 0u8..=100u8),
            0..8
        )
    ) {
        let cart_items: Vec<CartItem> = items
            .into_iter()
            .map(|(product_id, quantity, discount_pct)| CartItem { product_id, quantity, discount_pct })
            .collect();
        let bytes = encode_to_vec(&cart_items).expect("encode failed");
        let (decoded, _): (Vec<CartItem>, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(cart_items, decoded);
    }
}

// --- 9. Option<Product> roundtrip ---
proptest! {
    #[test]
    fn prop_option_product_roundtrip(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
        is_some in any::<bool>(),
    ) {
        let val: Option<Product> = if is_some {
            Some(Product { id, price_cents, quantity, weight_g, is_available })
        } else {
            None
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (Option<Product>, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 10. Deterministic encoding for Product ---
proptest! {
    #[test]
    fn prop_product_encoding_deterministic(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let val = Product { id, price_cents, quantity, weight_g, is_available };
        let bytes1 = encode_to_vec(&val).expect("encode failed first");
        let bytes2 = encode_to_vec(&val).expect("encode failed second");
        prop_assert_eq!(bytes1, bytes2);
    }
}

// --- 11. Deterministic encoding for ShippingMethod ---
proptest! {
    #[test]
    fn prop_shipping_encoding_deterministic(extra_cost_cents in 0u32..=500_00u32) {
        let variants = [
            ShippingMethod::Standard,
            ShippingMethod::Express,
            ShippingMethod::Overnight { extra_cost_cents },
            ShippingMethod::Pickup,
        ];
        for val in &variants {
            let bytes1 = encode_to_vec(val).expect("encode failed first");
            let bytes2 = encode_to_vec(val).expect("encode failed second");
            prop_assert_eq!(bytes1, bytes2);
        }
    }
}

// --- 12. Consumed bytes equals encoded length for Product ---
proptest! {
    #[test]
    fn prop_product_consumed_bytes(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let val = Product { id, price_cents, quantity, weight_g, is_available };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (_, consumed): (Product, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, bytes.len());
    }
}

// --- 13. Consumed bytes equals encoded length for CartItem ---
proptest! {
    #[test]
    fn prop_cart_item_consumed_bytes(
        product_id in 0u64..=u64::MAX,
        quantity in 0u32..=1000u32,
        discount_pct in 0u8..=100u8,
    ) {
        let val = CartItem { product_id, quantity, discount_pct };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (_, consumed): (CartItem, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, bytes.len());
    }
}

// --- 14. u8 roundtrip (discount_pct field range) ---
proptest! {
    #[test]
    fn prop_u8_discount_pct_roundtrip(discount_pct in 0u8..=100u8) {
        let bytes = encode_to_vec(&discount_pct).expect("encode failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(discount_pct, decoded);
    }
}

// --- 15. u32 roundtrip (quantity field range) ---
proptest! {
    #[test]
    fn prop_u32_quantity_roundtrip(quantity in 0u32..=10_000u32) {
        let bytes = encode_to_vec(&quantity).expect("encode failed");
        let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(quantity, decoded);
    }
}

// --- 16. u64 roundtrip (price_cents range) ---
proptest! {
    #[test]
    fn prop_u64_price_cents_roundtrip(price_cents in 0u64..=1_000_000_00u64) {
        let bytes = encode_to_vec(&price_cents).expect("encode failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(price_cents, decoded);
    }
}

// --- 17. bool roundtrip (is_available) ---
proptest! {
    #[test]
    fn prop_bool_is_available_roundtrip(is_available in any::<bool>()) {
        let bytes = encode_to_vec(&is_available).expect("encode failed");
        let (decoded, _): (bool, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(is_available, decoded);
    }
}

// --- 18. Distinct Products with same price but different quantity → different bytes ---
proptest! {
    #[test]
    fn prop_products_same_price_diff_quantity_differ(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity_a in 0u32..=4999u32,
        quantity_b in 5000u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let product_a = Product { id, price_cents, quantity: quantity_a, weight_g, is_available };
        let product_b = Product { id, price_cents, quantity: quantity_b, weight_g, is_available };
        let bytes_a = encode_to_vec(&product_a).expect("encode failed a");
        let bytes_b = encode_to_vec(&product_b).expect("encode failed b");
        prop_assert_ne!(bytes_a, bytes_b);
    }
}

// --- 19. CartItem with discount_pct = 0 and 100 roundtrip ---
proptest! {
    #[test]
    fn prop_cart_item_boundary_discounts(
        product_id in 0u64..=u64::MAX,
        quantity in 0u32..=1000u32,
    ) {
        for discount_pct in [0u8, 100u8] {
            let val = CartItem { product_id, quantity, discount_pct };
            let bytes = encode_to_vec(&val).expect("encode failed");
            let (decoded, _): (CartItem, usize) = decode_from_slice(&bytes).expect("decode failed");
            prop_assert_eq!(val, decoded);
        }
    }
}

// --- 20. Vec<ShippingMethod> with all variants roundtrip ---
proptest! {
    #[test]
    fn prop_vec_all_shipping_methods_roundtrip(extra_cost_cents in 0u32..=500_00u32) {
        let val = vec![
            ShippingMethod::Standard,
            ShippingMethod::Express,
            ShippingMethod::Overnight { extra_cost_cents },
            ShippingMethod::Pickup,
        ];
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (Vec<ShippingMethod>, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 21. Nested option: Option<CartItem> Some/None ---
proptest! {
    #[test]
    fn prop_option_cart_item_roundtrip(
        product_id in 0u64..=u64::MAX,
        quantity in 0u32..=1000u32,
        discount_pct in 0u8..=100u8,
        is_some in any::<bool>(),
    ) {
        let val: Option<CartItem> = if is_some {
            Some(CartItem { product_id, quantity, discount_pct })
        } else {
            None
        };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (Option<CartItem>, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 22. Double encode/decode identity ---
proptest! {
    #[test]
    fn prop_double_encode_decode_identity(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let original = Product { id, price_cents, quantity, weight_g, is_available };
        // First encode/decode cycle
        let bytes1 = encode_to_vec(&original).expect("first encode failed");
        let (intermediate, _): (Product, usize) = decode_from_slice(&bytes1).expect("first decode failed");
        // Second encode/decode cycle
        let bytes2 = encode_to_vec(&intermediate).expect("second encode failed");
        let (final_val, _): (Product, usize) = decode_from_slice(&bytes2).expect("second decode failed");
        prop_assert_eq!(original, final_val);
        prop_assert_eq!(bytes1, bytes2);
    }
}

// --- 23. Large Product id (u64::MAX range) roundtrip ---
proptest! {
    #[test]
    fn prop_large_product_id_roundtrip(
        id in (u64::MAX - 1_000_000)..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
    ) {
        let val = Product { id, price_cents, quantity, weight_g, is_available };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (Product, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}

// --- 24. Encoded bytes non-empty for all types ---
proptest! {
    #[test]
    fn prop_encoded_bytes_non_empty(
        id in 0u64..=u64::MAX,
        price_cents in 0u64..=1_000_000_00u64,
        quantity in 0u32..=10_000u32,
        weight_g in 0u32..=50_000u32,
        is_available in any::<bool>(),
        product_id in 0u64..=u64::MAX,
        discount_pct in 0u8..=100u8,
        extra_cost_cents in 0u32..=500_00u32,
    ) {
        let product = Product { id, price_cents, quantity, weight_g, is_available };
        let cart_item = CartItem { product_id, quantity, discount_pct };
        let shipping = ShippingMethod::Overnight { extra_cost_cents };

        let product_bytes = encode_to_vec(&product).expect("product encode failed");
        let cart_bytes = encode_to_vec(&cart_item).expect("cart encode failed");
        let shipping_bytes = encode_to_vec(&shipping).expect("shipping encode failed");

        prop_assert!(!product_bytes.is_empty());
        prop_assert!(!cart_bytes.is_empty());
        prop_assert!(!shipping_bytes.is_empty());
    }
}

// --- 25. Overnight extra_cost_cents roundtrip ---
proptest! {
    #[test]
    fn prop_overnight_extra_cost_roundtrip(extra_cost_cents in 0u32..=u32::MAX) {
        let val = ShippingMethod::Overnight { extra_cost_cents };
        let bytes = encode_to_vec(&val).expect("encode failed");
        let (decoded, _): (ShippingMethod, usize) = decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(val, decoded);
    }
}
