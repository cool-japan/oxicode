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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ItemCategory {
    Weapon,
    Armor,
    Consumable,
    Mount,
    Pet,
    Cosmetic,
    Material,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TradeStatus {
    Open,
    Pending,
    Completed,
    Cancelled,
    Expired,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GameItem {
    item_id: u64,
    name: String,
    rarity: ItemRarity,
    category: ItemCategory,
    base_price: u64,
    stack_size: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct InventorySlot {
    slot_id: u16,
    item: Option<GameItem>,
    quantity: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MarketListing {
    listing_id: u64,
    seller_id: u64,
    item: GameItem,
    quantity: u32,
    ask_price: u64,
    status: TradeStatus,
    listed_at: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlayerInventory {
    player_id: u64,
    slots: Vec<InventorySlot>,
    gold: u64,
}

// Test 1: each ItemRarity variant
#[test]
fn test_each_item_rarity_variant() {
    let variants = [
        ItemRarity::Common,
        ItemRarity::Uncommon,
        ItemRarity::Rare,
        ItemRarity::Epic,
        ItemRarity::Legendary,
        ItemRarity::Mythic,
    ];
    for variant in variants {
        let label = format!("{:?}", variant);
        let bytes =
            encode_to_vec(&variant).unwrap_or_else(|_| panic!("encode ItemRarity::{}", label));
        let (decoded, _) = decode_from_slice::<ItemRarity>(&bytes)
            .unwrap_or_else(|_| panic!("decode ItemRarity::{}", label));
        assert_eq!(variant, decoded);
    }
}

// Test 2: each ItemCategory variant
#[test]
fn test_each_item_category_variant() {
    let variants = [
        ItemCategory::Weapon,
        ItemCategory::Armor,
        ItemCategory::Consumable,
        ItemCategory::Mount,
        ItemCategory::Pet,
        ItemCategory::Cosmetic,
        ItemCategory::Material,
    ];
    for variant in variants {
        let label = format!("{:?}", variant);
        let bytes =
            encode_to_vec(&variant).unwrap_or_else(|_| panic!("encode ItemCategory::{}", label));
        let (decoded, _) = decode_from_slice::<ItemCategory>(&bytes)
            .unwrap_or_else(|_| panic!("decode ItemCategory::{}", label));
        assert_eq!(variant, decoded);
    }
}

// Test 3: each TradeStatus variant
#[test]
fn test_each_trade_status_variant() {
    let variants = [
        TradeStatus::Open,
        TradeStatus::Pending,
        TradeStatus::Completed,
        TradeStatus::Cancelled,
        TradeStatus::Expired,
    ];
    for variant in variants {
        let label = format!("{:?}", variant);
        let bytes =
            encode_to_vec(&variant).unwrap_or_else(|_| panic!("encode TradeStatus::{}", label));
        let (decoded, _) = decode_from_slice::<TradeStatus>(&bytes)
            .unwrap_or_else(|_| panic!("decode TradeStatus::{}", label));
        assert_eq!(variant, decoded);
    }
}

// Test 4: GameItem roundtrip
#[test]
fn test_game_item_roundtrip() {
    let val = GameItem {
        item_id: 10001,
        name: "Dragon Slayer Sword".to_string(),
        rarity: ItemRarity::Epic,
        category: ItemCategory::Weapon,
        base_price: 50000,
        stack_size: 1,
    };
    let bytes = encode_to_vec(&val).expect("encode GameItem");
    let (decoded, _) = decode_from_slice::<GameItem>(&bytes).expect("decode GameItem");
    assert_eq!(val, decoded);
}

// Test 5: InventorySlot with None item
#[test]
fn test_inventory_slot_none_item() {
    let val = InventorySlot {
        slot_id: 7,
        item: None,
        quantity: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode InventorySlot (None)");
    let (decoded, _) =
        decode_from_slice::<InventorySlot>(&bytes).expect("decode InventorySlot (None)");
    assert_eq!(val, decoded);
}

// Test 6: InventorySlot with Some item
#[test]
fn test_inventory_slot_some_item() {
    let val = InventorySlot {
        slot_id: 3,
        item: Some(GameItem {
            item_id: 20002,
            name: "Health Potion".to_string(),
            rarity: ItemRarity::Common,
            category: ItemCategory::Consumable,
            base_price: 50,
            stack_size: 99,
        }),
        quantity: 42,
    };
    let bytes = encode_to_vec(&val).expect("encode InventorySlot (Some)");
    let (decoded, _) =
        decode_from_slice::<InventorySlot>(&bytes).expect("decode InventorySlot (Some)");
    assert_eq!(val, decoded);
}

// Test 7: MarketListing roundtrip
#[test]
fn test_market_listing_roundtrip() {
    let val = MarketListing {
        listing_id: 999001,
        seller_id: 12345,
        item: GameItem {
            item_id: 30003,
            name: "Shadowcloak Armor".to_string(),
            rarity: ItemRarity::Rare,
            category: ItemCategory::Armor,
            base_price: 12000,
            stack_size: 1,
        },
        quantity: 1,
        ask_price: 18500,
        status: TradeStatus::Open,
        listed_at: 1700000000,
    };
    let bytes = encode_to_vec(&val).expect("encode MarketListing");
    let (decoded, _) = decode_from_slice::<MarketListing>(&bytes).expect("decode MarketListing");
    assert_eq!(val, decoded);
}

// Test 8: PlayerInventory empty slots
#[test]
fn test_player_inventory_empty_slots() {
    let val = PlayerInventory {
        player_id: 555,
        slots: vec![],
        gold: 1000,
    };
    let bytes = encode_to_vec(&val).expect("encode PlayerInventory (empty)");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode PlayerInventory (empty)");
    assert_eq!(val, decoded);
}

// Test 9: PlayerInventory with 10 slots
#[test]
fn test_player_inventory_with_10_slots() {
    let slots: Vec<InventorySlot> = (0..10)
        .map(|i| InventorySlot {
            slot_id: i as u16,
            item: Some(GameItem {
                item_id: 40000 + i as u64,
                name: format!("Item_{}", i),
                rarity: ItemRarity::Common,
                category: ItemCategory::Material,
                base_price: 10 * (i as u64 + 1),
                stack_size: 50,
            }),
            quantity: i as u32 * 5 + 1,
        })
        .collect();
    let val = PlayerInventory {
        player_id: 777,
        slots,
        gold: 250000,
    };
    let bytes = encode_to_vec(&val).expect("encode PlayerInventory (10 slots)");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode PlayerInventory (10 slots)");
    assert_eq!(val, decoded);
}

// Test 10: big_endian config
#[test]
fn test_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let val = GameItem {
        item_id: 88888,
        name: "Storm Bow".to_string(),
        rarity: ItemRarity::Uncommon,
        category: ItemCategory::Weapon,
        base_price: 3200,
        stack_size: 1,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big-endian GameItem");
    let (decoded, _) = decode_from_slice_with_config::<GameItem, _>(&bytes, cfg)
        .expect("decode big-endian GameItem");
    assert_eq!(val, decoded);
}

// Test 11: fixed_int config
#[test]
fn test_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = MarketListing {
        listing_id: 123456,
        seller_id: 654321,
        item: GameItem {
            item_id: 50001,
            name: "Iron Shield".to_string(),
            rarity: ItemRarity::Common,
            category: ItemCategory::Armor,
            base_price: 800,
            stack_size: 1,
        },
        quantity: 3,
        ask_price: 1200,
        status: TradeStatus::Pending,
        listed_at: 1710000000,
    };
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode fixed-int MarketListing");
    let (decoded, _) = decode_from_slice_with_config::<MarketListing, _>(&bytes, cfg)
        .expect("decode fixed-int MarketListing");
    assert_eq!(val, decoded);
}

// Test 12: consumed bytes check
#[test]
fn test_consumed_bytes_check() {
    let val = TradeStatus::Completed;
    let bytes = encode_to_vec(&val).expect("encode TradeStatus for bytes check");
    let (decoded, consumed) =
        decode_from_slice::<TradeStatus>(&bytes).expect("decode TradeStatus for bytes check");
    assert_eq!(val, decoded);
    assert!(consumed > 0, "consumed bytes must be greater than zero");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal total encoded length"
    );
}

// Test 13: Vec<MarketListing> roundtrip
#[test]
fn test_vec_market_listing_roundtrip() {
    let listings: Vec<MarketListing> = vec![
        MarketListing {
            listing_id: 1,
            seller_id: 100,
            item: GameItem {
                item_id: 1001,
                name: "Mana Crystal".to_string(),
                rarity: ItemRarity::Uncommon,
                category: ItemCategory::Material,
                base_price: 500,
                stack_size: 20,
            },
            quantity: 10,
            ask_price: 750,
            status: TradeStatus::Open,
            listed_at: 1700100000,
        },
        MarketListing {
            listing_id: 2,
            seller_id: 200,
            item: GameItem {
                item_id: 1002,
                name: "Fire Staff".to_string(),
                rarity: ItemRarity::Rare,
                category: ItemCategory::Weapon,
                base_price: 9000,
                stack_size: 1,
            },
            quantity: 1,
            ask_price: 14000,
            status: TradeStatus::Completed,
            listed_at: 1700200000,
        },
        MarketListing {
            listing_id: 3,
            seller_id: 300,
            item: GameItem {
                item_id: 1003,
                name: "Swift Mount Token".to_string(),
                rarity: ItemRarity::Epic,
                category: ItemCategory::Mount,
                base_price: 30000,
                stack_size: 1,
            },
            quantity: 1,
            ask_price: 45000,
            status: TradeStatus::Expired,
            listed_at: 1700300000,
        },
    ];
    let bytes = encode_to_vec(&listings).expect("encode Vec<MarketListing>");
    let (decoded, _) =
        decode_from_slice::<Vec<MarketListing>>(&bytes).expect("decode Vec<MarketListing>");
    assert_eq!(listings, decoded);
}

// Test 14: legendary item price
#[test]
fn test_legendary_item_price() {
    let val = GameItem {
        item_id: 99001,
        name: "Excalibur Reforged".to_string(),
        rarity: ItemRarity::Legendary,
        category: ItemCategory::Weapon,
        base_price: 999_999_999,
        stack_size: 1,
    };
    let bytes = encode_to_vec(&val).expect("encode legendary item");
    let (decoded, _) = decode_from_slice::<GameItem>(&bytes).expect("decode legendary item");
    assert_eq!(val, decoded);
    assert_eq!(decoded.base_price, 999_999_999);
    assert!(matches!(decoded.rarity, ItemRarity::Legendary));
}

// Test 15: mythic item stack
#[test]
fn test_mythic_item_stack() {
    let val = GameItem {
        item_id: 99999,
        name: "Void Essence".to_string(),
        rarity: ItemRarity::Mythic,
        category: ItemCategory::Material,
        base_price: 1_000_000,
        stack_size: u32::MAX,
    };
    let bytes = encode_to_vec(&val).expect("encode mythic item");
    let (decoded, _) = decode_from_slice::<GameItem>(&bytes).expect("decode mythic item");
    assert_eq!(val, decoded);
    assert_eq!(decoded.stack_size, u32::MAX);
    assert!(matches!(decoded.rarity, ItemRarity::Mythic));
}

// Test 16: open vs completed listing distinct bytes
#[test]
fn test_open_vs_completed_listing_distinct_bytes() {
    let make_item = || GameItem {
        item_id: 77001,
        name: "Silver Ring".to_string(),
        rarity: ItemRarity::Uncommon,
        category: ItemCategory::Cosmetic,
        base_price: 2000,
        stack_size: 1,
    };
    let open_listing = MarketListing {
        listing_id: 500,
        seller_id: 111,
        item: make_item(),
        quantity: 1,
        ask_price: 3000,
        status: TradeStatus::Open,
        listed_at: 1705000000,
    };
    let completed_listing = MarketListing {
        listing_id: 500,
        seller_id: 111,
        item: make_item(),
        quantity: 1,
        ask_price: 3000,
        status: TradeStatus::Completed,
        listed_at: 1705000000,
    };
    let open_bytes = encode_to_vec(&open_listing).expect("encode open listing");
    let completed_bytes = encode_to_vec(&completed_listing).expect("encode completed listing");
    assert_ne!(
        open_bytes, completed_bytes,
        "Open and Completed listings must produce distinct bytes"
    );
}

// Test 17: inventory with all rarities
#[test]
fn test_inventory_with_all_rarities() {
    let rarities = [
        ItemRarity::Common,
        ItemRarity::Uncommon,
        ItemRarity::Rare,
        ItemRarity::Epic,
        ItemRarity::Legendary,
        ItemRarity::Mythic,
    ];
    let slots: Vec<InventorySlot> = rarities
        .into_iter()
        .enumerate()
        .map(|(i, rarity)| InventorySlot {
            slot_id: i as u16,
            item: Some(GameItem {
                item_id: 60000 + i as u64,
                name: format!("Rarity_Item_{}", i),
                rarity,
                category: ItemCategory::Armor,
                base_price: (i as u64 + 1) * 1000,
                stack_size: 1,
            }),
            quantity: 1,
        })
        .collect();
    let val = PlayerInventory {
        player_id: 888,
        slots,
        gold: 5000,
    };
    let bytes = encode_to_vec(&val).expect("encode inventory all rarities");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode inventory all rarities");
    assert_eq!(val, decoded);
}

// Test 18: marketplace with mixed statuses
#[test]
fn test_marketplace_with_mixed_statuses() {
    let statuses = [
        TradeStatus::Open,
        TradeStatus::Pending,
        TradeStatus::Completed,
        TradeStatus::Cancelled,
        TradeStatus::Expired,
    ];
    let listings: Vec<MarketListing> = statuses
        .into_iter()
        .enumerate()
        .map(|(i, status)| MarketListing {
            listing_id: 700 + i as u64,
            seller_id: 9000 + i as u64,
            item: GameItem {
                item_id: 70000 + i as u64,
                name: format!("Trade_Item_{}", i),
                rarity: ItemRarity::Common,
                category: ItemCategory::Material,
                base_price: 100 * (i as u64 + 1),
                stack_size: 10,
            },
            quantity: 5,
            ask_price: 150 * (i as u64 + 1),
            status,
            listed_at: 1706000000 + i as u64 * 3600,
        })
        .collect();
    let bytes = encode_to_vec(&listings).expect("encode mixed statuses listings");
    let (decoded, _) =
        decode_from_slice::<Vec<MarketListing>>(&bytes).expect("decode mixed statuses listings");
    assert_eq!(listings, decoded);
}

// Test 19: full inventory (20 slots)
#[test]
fn test_full_inventory_20_slots() {
    let slots: Vec<InventorySlot> = (0..20)
        .map(|i| {
            let has_item = i % 3 != 0;
            InventorySlot {
                slot_id: i as u16,
                item: if has_item {
                    Some(GameItem {
                        item_id: 80000 + i as u64,
                        name: format!("FullInv_Item_{}", i),
                        rarity: if i % 6 == 0 {
                            ItemRarity::Epic
                        } else if i % 4 == 0 {
                            ItemRarity::Rare
                        } else {
                            ItemRarity::Common
                        },
                        category: ItemCategory::Consumable,
                        base_price: 200 + i as u64 * 50,
                        stack_size: 20,
                    })
                } else {
                    None
                },
                quantity: if has_item { 10 + i as u32 } else { 0 },
            }
        })
        .collect();
    let val = PlayerInventory {
        player_id: 101010,
        slots,
        gold: 123456,
    };
    let bytes = encode_to_vec(&val).expect("encode full inventory 20 slots");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode full inventory 20 slots");
    assert_eq!(val, decoded);
}

// Test 20: gold boundary u64::MAX
#[test]
fn test_gold_boundary_u64_max() {
    let val = PlayerInventory {
        player_id: 999999,
        slots: vec![],
        gold: u64::MAX,
    };
    let bytes = encode_to_vec(&val).expect("encode PlayerInventory gold=u64::MAX");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode PlayerInventory gold=u64::MAX");
    assert_eq!(val, decoded);
    assert_eq!(decoded.gold, u64::MAX);
}

// Test 21: item name unicode
#[test]
fn test_item_name_unicode() {
    let val = GameItem {
        item_id: 90001,
        name: "龍の剣 🐉⚔️ Épée du Dragon".to_string(),
        rarity: ItemRarity::Legendary,
        category: ItemCategory::Weapon,
        base_price: 888888,
        stack_size: 1,
    };
    let bytes = encode_to_vec(&val).expect("encode unicode item name");
    let (decoded, _) = decode_from_slice::<GameItem>(&bytes).expect("decode unicode item name");
    assert_eq!(val, decoded);
    assert_eq!(decoded.name, "龍の剣 🐉⚔️ Épée du Dragon");
}

// Test 22: bulk material stacking
#[test]
fn test_bulk_material_stacking() {
    let materials: Vec<InventorySlot> = (0..5)
        .map(|i| InventorySlot {
            slot_id: 100 + i as u16,
            item: Some(GameItem {
                item_id: 90100 + i as u64,
                name: format!("Bulk_Ore_{}", i),
                rarity: ItemRarity::Common,
                category: ItemCategory::Material,
                base_price: 1,
                stack_size: 9999,
            }),
            quantity: 9999,
        })
        .collect();
    let val = PlayerInventory {
        player_id: 202020,
        slots: materials,
        gold: 77777,
    };
    let bytes = encode_to_vec(&val).expect("encode bulk material inventory");
    let (decoded, _) =
        decode_from_slice::<PlayerInventory>(&bytes).expect("decode bulk material inventory");
    assert_eq!(val, decoded);
    for slot in &decoded.slots {
        assert_eq!(slot.quantity, 9999);
        if let Some(ref item) = slot.item {
            assert_eq!(item.stack_size, 9999);
            assert!(matches!(item.category, ItemCategory::Material));
        }
    }
}
