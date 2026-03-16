//! Tests for Gaming / RPG character system — advanced enum roundtrip coverage.

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
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Cleric,
    Ranger,
    Paladin,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Attribute {
    strength: u8,
    dexterity: u8,
    intelligence: u8,
    constitution: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Equipment {
    Weapon { name: String, damage: u32 },
    Armor { name: String, defense: u32 },
    Accessory { name: String, effect: String },
    None,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Character {
    id: u64,
    name: String,
    class: CharacterClass,
    level: u32,
    attributes: Attribute,
    equipment: Vec<Equipment>,
    experience: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Quest {
    id: u32,
    title: String,
    min_level: u32,
    reward_exp: u64,
    completed: bool,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn default_attributes() -> Attribute {
    Attribute {
        strength: 10,
        dexterity: 10,
        intelligence: 10,
        constitution: 10,
        wisdom: 10,
        charisma: 10,
    }
}

fn make_character(
    id: u64,
    name: &str,
    class: CharacterClass,
    level: u32,
    attributes: Attribute,
    equipment: Vec<Equipment>,
    experience: u64,
) -> Character {
    Character {
        id,
        name: name.to_string(),
        class,
        level,
        attributes,
        equipment,
        experience,
    }
}

fn make_quest(id: u32, title: &str, min_level: u32, reward_exp: u64, completed: bool) -> Quest {
    Quest {
        id,
        title: title.to_string(),
        min_level,
        reward_exp,
        completed,
    }
}

// ── test 1: all CharacterClass variants roundtrip ─────────────────────────────

#[test]
fn test_character_class_all_variants_roundtrip() {
    let variants = [
        CharacterClass::Warrior,
        CharacterClass::Mage,
        CharacterClass::Rogue,
        CharacterClass::Cleric,
        CharacterClass::Ranger,
        CharacterClass::Paladin,
    ];

    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode CharacterClass");
        let (decoded, consumed): (CharacterClass, usize) =
            decode_from_slice(&bytes).expect("decode CharacterClass");
        assert_eq!(variant, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for CharacterClass"
        );
    }
}

// ── test 2: CharacterClass discriminant uniqueness ────────────────────────────

#[test]
fn test_character_class_discriminant_uniqueness() {
    let variants = [
        CharacterClass::Warrior,
        CharacterClass::Mage,
        CharacterClass::Rogue,
        CharacterClass::Cleric,
        CharacterClass::Ranger,
        CharacterClass::Paladin,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode CharacterClass for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "CharacterClass variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ── test 3: Equipment::Weapon roundtrip ───────────────────────────────────────

#[test]
fn test_equipment_weapon_roundtrip() {
    let val = Equipment::Weapon {
        name: "Excalibur".to_string(),
        damage: 250,
    };
    let bytes = encode_to_vec(&val).expect("encode Equipment::Weapon");
    let (decoded, consumed): (Equipment, usize) =
        decode_from_slice(&bytes).expect("decode Equipment::Weapon");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Equipment::Weapon"
    );
}

// ── test 4: Equipment::Armor roundtrip ────────────────────────────────────────

#[test]
fn test_equipment_armor_roundtrip() {
    let val = Equipment::Armor {
        name: "Dragonscale Plate".to_string(),
        defense: 180,
    };
    let bytes = encode_to_vec(&val).expect("encode Equipment::Armor");
    let (decoded, consumed): (Equipment, usize) =
        decode_from_slice(&bytes).expect("decode Equipment::Armor");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Equipment::Armor"
    );
}

// ── test 5: Equipment::Accessory roundtrip ────────────────────────────────────

#[test]
fn test_equipment_accessory_roundtrip() {
    let val = Equipment::Accessory {
        name: "Ring of Arcane Might".to_string(),
        effect: "+20 intelligence, regenerate 5 mana per second".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Equipment::Accessory");
    let (decoded, consumed): (Equipment, usize) =
        decode_from_slice(&bytes).expect("decode Equipment::Accessory");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Equipment::Accessory"
    );
}

// ── test 6: Equipment::None roundtrip ─────────────────────────────────────────

#[test]
fn test_equipment_none_roundtrip() {
    let val = Equipment::None;
    let bytes = encode_to_vec(&val).expect("encode Equipment::None");
    let (decoded, consumed): (Equipment, usize) =
        decode_from_slice(&bytes).expect("decode Equipment::None");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Equipment::None"
    );
}

// ── test 7: Equipment all variants produce distinct encodings ─────────────────

#[test]
fn test_equipment_discriminant_uniqueness() {
    let variants: Vec<Equipment> = vec![
        Equipment::Weapon {
            name: "Sword".to_string(),
            damage: 100,
        },
        Equipment::Armor {
            name: "Shield".to_string(),
            defense: 50,
        },
        Equipment::Accessory {
            name: "Amulet".to_string(),
            effect: "Strength +5".to_string(),
        },
        Equipment::None,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode Equipment variant"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Equipment variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ── test 8: Vec<Equipment> roundtrip ──────────────────────────────────────────

#[test]
fn test_vec_equipment_roundtrip() {
    let equipment: Vec<Equipment> = vec![
        Equipment::Weapon {
            name: "Longsword".to_string(),
            damage: 120,
        },
        Equipment::Armor {
            name: "Chainmail".to_string(),
            defense: 75,
        },
        Equipment::Accessory {
            name: "Boots of Swiftness".to_string(),
            effect: "Move speed +30%".to_string(),
        },
        Equipment::None,
        Equipment::Weapon {
            name: "Dagger".to_string(),
            damage: 45,
        },
    ];

    let bytes = encode_to_vec(&equipment).expect("encode Vec<Equipment>");
    let (decoded, consumed): (Vec<Equipment>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Equipment>");
    assert_eq!(equipment, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Vec<Equipment>"
    );
}

// ── test 9: Attribute struct roundtrip with extreme values ────────────────────

#[test]
fn test_attribute_roundtrip_extreme_values() {
    let val = Attribute {
        strength: u8::MAX,
        dexterity: u8::MIN,
        intelligence: 127,
        constitution: 200,
        wisdom: 1,
        charisma: 255,
    };
    let bytes = encode_to_vec(&val).expect("encode Attribute extreme values");
    let (decoded, consumed): (Attribute, usize) =
        decode_from_slice(&bytes).expect("decode Attribute extreme values");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Attribute"
    );
}

// ── test 10: Character (Warrior) with full equipment roundtrip ────────────────

#[test]
fn test_character_warrior_full_equipment_roundtrip() {
    let attributes = Attribute {
        strength: 18,
        dexterity: 12,
        intelligence: 8,
        constitution: 16,
        wisdom: 10,
        charisma: 11,
    };
    let equipment = vec![
        Equipment::Weapon {
            name: "Greatsword of Slaying".to_string(),
            damage: 300,
        },
        Equipment::Armor {
            name: "Plate of the Titan".to_string(),
            defense: 250,
        },
        Equipment::Accessory {
            name: "Warrior's Signet".to_string(),
            effect: "Strength +10, critical hit +5%".to_string(),
        },
    ];
    let val = make_character(
        1001,
        "Aldric the Bold",
        CharacterClass::Warrior,
        50,
        attributes,
        equipment,
        1_500_000,
    );
    let bytes = encode_to_vec(&val).expect("encode warrior Character");
    let (decoded, consumed): (Character, usize) =
        decode_from_slice(&bytes).expect("decode warrior Character");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for warrior Character"
    );
}

// ── test 11: Character (Mage) with empty equipment roundtrip ──────────────────

#[test]
fn test_character_mage_empty_equipment_roundtrip() {
    let attributes = Attribute {
        strength: 6,
        dexterity: 10,
        intelligence: 20,
        constitution: 8,
        wisdom: 16,
        charisma: 14,
    };
    let val = make_character(
        1002,
        "Seraphina the Wise",
        CharacterClass::Mage,
        35,
        attributes,
        vec![],
        750_000,
    );
    let bytes = encode_to_vec(&val).expect("encode mage Character");
    let (decoded, consumed): (Character, usize) =
        decode_from_slice(&bytes).expect("decode mage Character");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for mage Character"
    );
}

// ── test 12: Character (Rogue) with mixed equipment including None ─────────────

#[test]
fn test_character_rogue_mixed_equipment_roundtrip() {
    let attributes = Attribute {
        strength: 12,
        dexterity: 20,
        intelligence: 13,
        constitution: 10,
        wisdom: 11,
        charisma: 15,
    };
    let equipment = vec![
        Equipment::Weapon {
            name: "Shadow Blade".to_string(),
            damage: 180,
        },
        Equipment::None,
        Equipment::Accessory {
            name: "Cloak of Shadows".to_string(),
            effect: "Stealth +50, dodge +20%".to_string(),
        },
    ];
    let val = make_character(
        1003,
        "Kira Nightfall",
        CharacterClass::Rogue,
        42,
        attributes,
        equipment,
        980_000,
    );
    let bytes = encode_to_vec(&val).expect("encode rogue Character");
    let (decoded, _): (Character, usize) =
        decode_from_slice(&bytes).expect("decode rogue Character");
    assert_eq!(val, decoded);
}

// ── test 13: Quest roundtrip — completed quest ────────────────────────────────

#[test]
fn test_quest_completed_roundtrip() {
    let val = make_quest(501, "Slay the Dragon of Ashenmoor", 40, 50_000, true);
    let bytes = encode_to_vec(&val).expect("encode completed Quest");
    let (decoded, consumed): (Quest, usize) =
        decode_from_slice(&bytes).expect("decode completed Quest");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for completed Quest"
    );
}

// ── test 14: Quest roundtrip — incomplete quest ───────────────────────────────

#[test]
fn test_quest_incomplete_roundtrip() {
    let val = make_quest(
        502,
        "Recover the Lost Artifacts of Sunken Keep",
        15,
        8_000,
        false,
    );
    let bytes = encode_to_vec(&val).expect("encode incomplete Quest");
    let (decoded, consumed): (Quest, usize) =
        decode_from_slice(&bytes).expect("decode incomplete Quest");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for incomplete Quest"
    );
}

// ── test 15: Vec<Character> roundtrip ─────────────────────────────────────────

#[test]
fn test_vec_character_roundtrip() {
    let characters: Vec<Character> = vec![
        make_character(
            2001,
            "Theron",
            CharacterClass::Paladin,
            60,
            Attribute {
                strength: 16,
                dexterity: 10,
                intelligence: 12,
                constitution: 18,
                wisdom: 14,
                charisma: 17,
            },
            vec![
                Equipment::Weapon {
                    name: "Holy Avenger".to_string(),
                    damage: 280,
                },
                Equipment::Armor {
                    name: "Sacred Platemail".to_string(),
                    defense: 220,
                },
            ],
            2_000_000,
        ),
        make_character(
            2002,
            "Sylvara",
            CharacterClass::Ranger,
            28,
            Attribute {
                strength: 13,
                dexterity: 19,
                intelligence: 12,
                constitution: 12,
                wisdom: 15,
                charisma: 10,
            },
            vec![Equipment::Weapon {
                name: "Elven Longbow".to_string(),
                damage: 160,
            }],
            450_000,
        ),
        make_character(
            2003,
            "Brother Callum",
            CharacterClass::Cleric,
            22,
            default_attributes(),
            vec![Equipment::None],
            200_000,
        ),
    ];

    let bytes = encode_to_vec(&characters).expect("encode Vec<Character>");
    let (decoded, consumed): (Vec<Character>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Character>");
    assert_eq!(characters, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Vec<Character>"
    );
}

// ── test 16: Character with big-endian config roundtrip ───────────────────────

#[test]
fn test_character_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = make_character(
        3001,
        "Voraxis the Archmage",
        CharacterClass::Mage,
        99,
        Attribute {
            strength: 5,
            dexterity: 9,
            intelligence: 25,
            constitution: 7,
            wisdom: 22,
            charisma: 18,
        },
        vec![
            Equipment::Weapon {
                name: "Staff of Eternity".to_string(),
                damage: 500,
            },
            Equipment::Accessory {
                name: "Tome of Ancient Power".to_string(),
                effect: "All spells +30%".to_string(),
            },
        ],
        9_999_999,
    );
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Character big-endian");
    let (decoded, _): (Character, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Character big-endian");
    assert_eq!(val, decoded);
}

// ── test 17: Quest with fixed-int config roundtrip ────────────────────────────

#[test]
fn test_quest_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = make_quest(600, "The Grand Tournament", 30, 25_000, false);
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Quest fixed-int");
    let (decoded, consumed): (Quest, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Quest fixed-int");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Quest fixed-int"
    );
}

// ── test 18: completed vs incomplete Quest encode differently ─────────────────

#[test]
fn test_quest_completed_vs_incomplete_encode_differently() {
    let completed = make_quest(700, "Defeat the Lich King", 50, 100_000, true);
    let incomplete = make_quest(700, "Defeat the Lich King", 50, 100_000, false);

    let bytes_completed = encode_to_vec(&completed).expect("encode completed Quest");
    let bytes_incomplete = encode_to_vec(&incomplete).expect("encode incomplete Quest");
    assert_ne!(
        bytes_completed, bytes_incomplete,
        "completed and incomplete Quest must yield different encodings"
    );
}

// ── test 19: Character consumed bytes equals encoded length ───────────────────

#[test]
fn test_character_consumed_bytes_equals_encoded_length() {
    let val = make_character(
        4001,
        "Zephyrus the Swift — an exceptionally named ranger of the northern realms",
        CharacterClass::Ranger,
        77,
        Attribute {
            strength: 11,
            dexterity: 21,
            intelligence: 13,
            constitution: 13,
            wisdom: 12,
            charisma: 14,
        },
        vec![
            Equipment::Weapon {
                name: "Gale-Force Shortbow".to_string(),
                damage: 195,
            },
            Equipment::Armor {
                name: "Leather Vest of the Wind".to_string(),
                defense: 85,
            },
            Equipment::Accessory {
                name: "Quiver of Endless Arrows".to_string(),
                effect: "Never run out of arrows".to_string(),
            },
        ],
        3_141_592,
    );
    let bytes = encode_to_vec(&val).expect("encode Character for length check");
    let (decoded, consumed): (Character, usize) =
        decode_from_slice(&bytes).expect("decode Character for length check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must exactly equal total encoded length"
    );
}

// ── test 20: nested struct roundtrip — Attribute field-by-field verification ──

#[test]
fn test_character_attribute_nested_struct_field_by_field() {
    let attributes = Attribute {
        strength: 14,
        dexterity: 16,
        intelligence: 18,
        constitution: 12,
        wisdom: 20,
        charisma: 9,
    };
    let val = make_character(
        5001,
        "Mirenna the Devoted",
        CharacterClass::Cleric,
        45,
        attributes,
        vec![Equipment::Weapon {
            name: "Mace of Holy Light".to_string(),
            damage: 140,
        }],
        1_200_000,
    );
    let bytes = encode_to_vec(&val).expect("encode Character for nested field check");
    let (decoded, _): (Character, usize) =
        decode_from_slice(&bytes).expect("decode Character for nested field check");

    assert_eq!(
        decoded.attributes.strength, 14,
        "strength must be preserved"
    );
    assert_eq!(
        decoded.attributes.dexterity, 16,
        "dexterity must be preserved"
    );
    assert_eq!(
        decoded.attributes.intelligence, 18,
        "intelligence must be preserved"
    );
    assert_eq!(
        decoded.attributes.constitution, 12,
        "constitution must be preserved"
    );
    assert_eq!(decoded.attributes.wisdom, 20, "wisdom must be preserved");
    assert_eq!(decoded.attributes.charisma, 9, "charisma must be preserved");
    assert_eq!(decoded.id, 5001, "id must be preserved");
    assert_eq!(decoded.level, 45, "level must be preserved");
    assert_eq!(
        decoded.experience, 1_200_000,
        "experience must be preserved"
    );
}

// ── test 21: Characters with different classes but identical other fields differ

#[test]
fn test_different_character_classes_encode_differently() {
    let make = |class: CharacterClass| {
        make_character(
            6001,
            "Generic Hero",
            class,
            20,
            default_attributes(),
            vec![Equipment::None],
            100_000,
        )
    };

    let bytes_warrior = encode_to_vec(&make(CharacterClass::Warrior)).expect("encode Warrior");
    let bytes_mage = encode_to_vec(&make(CharacterClass::Mage)).expect("encode Mage");
    let bytes_rogue = encode_to_vec(&make(CharacterClass::Rogue)).expect("encode Rogue");
    let bytes_cleric = encode_to_vec(&make(CharacterClass::Cleric)).expect("encode Cleric");
    let bytes_ranger = encode_to_vec(&make(CharacterClass::Ranger)).expect("encode Ranger");
    let bytes_paladin = encode_to_vec(&make(CharacterClass::Paladin)).expect("encode Paladin");

    assert_ne!(
        bytes_warrior, bytes_mage,
        "Warrior and Mage must yield different encodings"
    );
    assert_ne!(
        bytes_warrior, bytes_rogue,
        "Warrior and Rogue must yield different encodings"
    );
    assert_ne!(
        bytes_warrior, bytes_cleric,
        "Warrior and Cleric must yield different encodings"
    );
    assert_ne!(
        bytes_warrior, bytes_ranger,
        "Warrior and Ranger must yield different encodings"
    );
    assert_ne!(
        bytes_warrior, bytes_paladin,
        "Warrior and Paladin must yield different encodings"
    );
    assert_ne!(
        bytes_mage, bytes_rogue,
        "Mage and Rogue must yield different encodings"
    );
    assert_ne!(
        bytes_mage, bytes_cleric,
        "Mage and Cleric must yield different encodings"
    );
    assert_ne!(
        bytes_mage, bytes_ranger,
        "Mage and Ranger must yield different encodings"
    );
    assert_ne!(
        bytes_mage, bytes_paladin,
        "Mage and Paladin must yield different encodings"
    );
    assert_ne!(
        bytes_rogue, bytes_cleric,
        "Rogue and Cleric must yield different encodings"
    );
    assert_ne!(
        bytes_rogue, bytes_ranger,
        "Rogue and Ranger must yield different encodings"
    );
    assert_ne!(
        bytes_rogue, bytes_paladin,
        "Rogue and Paladin must yield different encodings"
    );
    assert_ne!(
        bytes_cleric, bytes_ranger,
        "Cleric and Ranger must yield different encodings"
    );
    assert_ne!(
        bytes_cleric, bytes_paladin,
        "Cleric and Paladin must yield different encodings"
    );
    assert_ne!(
        bytes_ranger, bytes_paladin,
        "Ranger and Paladin must yield different encodings"
    );
}

// ── test 22: Vec<Quest> roundtrip with mixed completed flags ───────────────────

#[test]
fn test_vec_quest_mixed_completion_roundtrip() {
    let quests: Vec<Quest> = vec![
        make_quest(801, "Prologue: The Village in Flames", 1, 500, true),
        make_quest(802, "Find the Missing Scout", 5, 1_200, true),
        make_quest(803, "Infiltrate the Bandit Fortress", 12, 4_500, false),
        make_quest(804, "Retrieve the Crystalline Heart", 20, 12_000, false),
        make_quest(805, "Storm the Necromancer's Tower", 35, 35_000, false),
        make_quest(806, "The Final Reckoning", 60, 200_000, false),
    ];

    let bytes = encode_to_vec(&quests).expect("encode Vec<Quest>");
    let (decoded, consumed): (Vec<Quest>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Quest>");
    assert_eq!(quests, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Vec<Quest>"
    );

    assert_eq!(decoded[0].completed, true, "first quest must be completed");
    assert_eq!(decoded[1].completed, true, "second quest must be completed");
    assert_eq!(
        decoded[2].completed, false,
        "third quest must not be completed"
    );
    assert_eq!(
        decoded[5].reward_exp, 200_000,
        "final quest reward_exp must be preserved"
    );
    assert_eq!(
        decoded[5].title, "The Final Reckoning",
        "final quest title must be preserved"
    );
}
