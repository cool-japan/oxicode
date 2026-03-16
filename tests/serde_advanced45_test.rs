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
enum ContentType {
    Text,
    Image,
    Video,
    Audio,
    Link,
    Poll,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ModerationAction {
    Approved,
    Flagged,
    Removed,
    Escalated,
    AutoHidden,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReactionType {
    Like,
    Love,
    Laugh,
    Wow,
    Sad,
    Angry,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum NotificationType {
    Mention,
    Reply,
    Follow,
    Reaction,
    Direct,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UserProfile {
    user_id: u64,
    username: String,
    follower_count: u64,
    following_count: u64,
    verified: bool,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Post {
    post_id: u64,
    author_id: u64,
    content_type: ContentType,
    text: Option<String>,
    media_url: Option<String>,
    likes: u64,
    shares: u32,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ModerationEvent {
    event_id: u64,
    content_id: u64,
    moderator_id: Option<u64>,
    action: ModerationAction,
    reason: String,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Notification {
    notif_id: u64,
    user_id: u64,
    sender_id: u64,
    notif_type: NotificationType,
    read: bool,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReactionCount {
    reaction_type: u8,
    count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EngagementMetric {
    content_id: u64,
    period_start: u64,
    period_end: u64,
    impressions: u64,
    clicks: u32,
    reactions: Vec<ReactionCount>,
}

// --- Test 1: UserProfile basic roundtrip with standard config ---
#[test]
fn test_user_profile_standard_roundtrip() {
    let profile = UserProfile {
        user_id: 100001,
        username: "alice_wonder".to_string(),
        follower_count: 4820,
        following_count: 312,
        verified: true,
        created_at: 1700000000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&profile, cfg).expect("encode UserProfile standard");
    let (decoded, consumed): (UserProfile, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode UserProfile standard");
    assert_eq!(profile, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 2: UserProfile with big endian config ---
#[test]
fn test_user_profile_big_endian_roundtrip() {
    let profile = UserProfile {
        user_id: 999888777,
        username: "bob_the_builder".to_string(),
        follower_count: 0,
        following_count: 0,
        verified: false,
        created_at: 1710000000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&profile, cfg).expect("encode UserProfile big endian");
    let (decoded, consumed): (UserProfile, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode UserProfile big endian");
    assert_eq!(profile, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 3: UserProfile with fixed int encoding ---
#[test]
fn test_user_profile_fixed_int_roundtrip() {
    let profile = UserProfile {
        user_id: 555444333,
        username: "charlie_x".to_string(),
        follower_count: 1_000_000,
        following_count: 500,
        verified: true,
        created_at: 1720000000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&profile, cfg).expect("encode UserProfile fixed int");
    let (decoded, consumed): (UserProfile, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode UserProfile fixed int");
    assert_eq!(profile, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 4: Post with text content, Some text, None media_url ---
#[test]
fn test_post_text_some_none_standard() {
    let post = Post {
        post_id: 201,
        author_id: 100001,
        content_type: ContentType::Text,
        text: Some("Hello, world! This is my first post.".to_string()),
        media_url: None,
        likes: 42,
        shares: 7,
        created_at: 1700100000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&post, cfg).expect("encode Post text/some/none");
    let (decoded, consumed): (Post, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Post text/some/none");
    assert_eq!(post, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 5: Post with image content, None text, Some media_url ---
#[test]
fn test_post_image_none_some_big_endian() {
    let post = Post {
        post_id: 202,
        author_id: 100002,
        content_type: ContentType::Image,
        text: None,
        media_url: Some("https://cdn.example.com/images/photo123.jpg".to_string()),
        likes: 1500,
        shares: 200,
        created_at: 1700200000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&post, cfg).expect("encode Post image big endian");
    let (decoded, consumed): (Post, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Post image big endian");
    assert_eq!(post, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 6: Post with video content, both Some fields ---
#[test]
fn test_post_video_both_some_fixed_int() {
    let post = Post {
        post_id: 203,
        author_id: 100003,
        content_type: ContentType::Video,
        text: Some("Watch this amazing video!".to_string()),
        media_url: Some("https://cdn.example.com/videos/clip456.mp4".to_string()),
        likes: 9900,
        shares: 3200,
        created_at: 1700300000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&post, cfg).expect("encode Post video fixed int");
    let (decoded, consumed): (Post, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Post video fixed int");
    assert_eq!(post, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 7: Post with Poll content type ---
#[test]
fn test_post_poll_content_type_standard() {
    let post = Post {
        post_id: 204,
        author_id: 100004,
        content_type: ContentType::Poll,
        text: Some("What is your favorite programming language?".to_string()),
        media_url: None,
        likes: 250,
        shares: 40,
        created_at: 1700400000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&post, cfg).expect("encode Post poll");
    let (decoded, consumed): (Post, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Post poll");
    assert_eq!(post, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 8: ModerationEvent with human moderator (Some moderator_id), Approved ---
#[test]
fn test_moderation_event_approved_human_standard() {
    let event = ModerationEvent {
        event_id: 3001,
        content_id: 201,
        moderator_id: Some(9001),
        action: ModerationAction::Approved,
        reason: "Content complies with community guidelines.".to_string(),
        timestamp: 1700500000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&event, cfg).expect("encode ModerationEvent approved");
    let (decoded, consumed): (ModerationEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ModerationEvent approved");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 9: ModerationEvent with AutoHidden (None moderator_id = automated) ---
#[test]
fn test_moderation_event_auto_hidden_none_moderator_big_endian() {
    let event = ModerationEvent {
        event_id: 3002,
        content_id: 202,
        moderator_id: None,
        action: ModerationAction::AutoHidden,
        reason: "Automated policy violation: spam detection triggered.".to_string(),
        timestamp: 1700600000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&event, cfg).expect("encode ModerationEvent auto hidden");
    let (decoded, consumed): (ModerationEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ModerationEvent auto hidden");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 10: ModerationEvent Escalated with fixed int ---
#[test]
fn test_moderation_event_escalated_fixed_int() {
    let event = ModerationEvent {
        event_id: 3003,
        content_id: 203,
        moderator_id: Some(9002),
        action: ModerationAction::Escalated,
        reason: "Requires senior review: potential doxxing content.".to_string(),
        timestamp: 1700700000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&event, cfg).expect("encode ModerationEvent escalated");
    let (decoded, consumed): (ModerationEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ModerationEvent escalated");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 11: Notification for Mention, unread ---
#[test]
fn test_notification_mention_unread_standard() {
    let notif = Notification {
        notif_id: 5001,
        user_id: 100001,
        sender_id: 100002,
        notif_type: NotificationType::Mention,
        read: false,
        created_at: 1700800000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&notif, cfg).expect("encode Notification mention");
    let (decoded, consumed): (Notification, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Notification mention");
    assert_eq!(notif, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 12: Notification for Follow, read ---
#[test]
fn test_notification_follow_read_big_endian() {
    let notif = Notification {
        notif_id: 5002,
        user_id: 100003,
        sender_id: 100004,
        notif_type: NotificationType::Follow,
        read: true,
        created_at: 1700900000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&notif, cfg).expect("encode Notification follow");
    let (decoded, consumed): (Notification, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Notification follow");
    assert_eq!(notif, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 13: Notification for Direct message with fixed int ---
#[test]
fn test_notification_direct_fixed_int() {
    let notif = Notification {
        notif_id: 5003,
        user_id: 100005,
        sender_id: 100006,
        notif_type: NotificationType::Direct,
        read: false,
        created_at: 1701000000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&notif, cfg).expect("encode Notification direct");
    let (decoded, consumed): (Notification, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Notification direct");
    assert_eq!(notif, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 14: EngagementMetric with multiple reaction counts ---
#[test]
fn test_engagement_metric_multi_reactions_standard() {
    let metric = EngagementMetric {
        content_id: 201,
        period_start: 1700000000,
        period_end: 1700086400,
        impressions: 50000,
        clicks: 3200,
        reactions: vec![
            ReactionCount {
                reaction_type: 0,
                count: 1200,
            }, // Like
            ReactionCount {
                reaction_type: 1,
                count: 430,
            }, // Love
            ReactionCount {
                reaction_type: 2,
                count: 80,
            }, // Laugh
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&metric, cfg).expect("encode EngagementMetric multi reactions");
    let (decoded, consumed): (EngagementMetric, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EngagementMetric multi reactions");
    assert_eq!(metric, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 15: EngagementMetric with empty reactions vec ---
#[test]
fn test_engagement_metric_empty_reactions_big_endian() {
    let metric = EngagementMetric {
        content_id: 202,
        period_start: 1701000000,
        period_end: 1701086400,
        impressions: 12345,
        clicks: 678,
        reactions: vec![],
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&metric, cfg).expect("encode EngagementMetric empty reactions");
    let (decoded, consumed): (EngagementMetric, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EngagementMetric empty reactions");
    assert_eq!(metric, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 16: EngagementMetric with all reaction types and fixed int ---
#[test]
fn test_engagement_metric_all_reaction_types_fixed_int() {
    let metric = EngagementMetric {
        content_id: 203,
        period_start: 1702000000,
        period_end: 1702172800,
        impressions: 1_000_000,
        clicks: 45000,
        reactions: vec![
            ReactionCount {
                reaction_type: 0,
                count: 20000,
            }, // Like
            ReactionCount {
                reaction_type: 1,
                count: 8000,
            }, // Love
            ReactionCount {
                reaction_type: 2,
                count: 3000,
            }, // Laugh
            ReactionCount {
                reaction_type: 3,
                count: 500,
            }, // Wow
            ReactionCount {
                reaction_type: 4,
                count: 150,
            }, // Sad
            ReactionCount {
                reaction_type: 5,
                count: 75,
            }, // Angry
        ],
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&metric, cfg).expect("encode EngagementMetric all types");
    let (decoded, consumed): (EngagementMetric, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EngagementMetric all types");
    assert_eq!(metric, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Test 17: Vec<Post> roundtrip with mixed content types ---
#[test]
fn test_vec_posts_mixed_content_standard() {
    let posts = vec![
        Post {
            post_id: 301,
            author_id: 100010,
            content_type: ContentType::Audio,
            text: Some("Listen to my new track!".to_string()),
            media_url: Some("https://cdn.example.com/audio/track789.mp3".to_string()),
            likes: 88,
            shares: 12,
            created_at: 1703000000,
        },
        Post {
            post_id: 302,
            author_id: 100011,
            content_type: ContentType::Link,
            text: Some("Check out this article".to_string()),
            media_url: Some("https://news.example.com/article/42".to_string()),
            likes: 35,
            shares: 5,
            created_at: 1703010000,
        },
        Post {
            post_id: 303,
            author_id: 100012,
            content_type: ContentType::Text,
            text: Some("Just vibing today #monday".to_string()),
            media_url: None,
            likes: 7,
            shares: 0,
            created_at: 1703020000,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&posts, cfg).expect("encode Vec<Post> mixed");
    let (decoded, consumed): (Vec<Post>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<Post> mixed");
    assert_eq!(posts, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
}

// --- Test 18: Vec<ModerationEvent> with mixed actions ---
#[test]
fn test_vec_moderation_events_mixed_actions_big_endian() {
    let events = vec![
        ModerationEvent {
            event_id: 4001,
            content_id: 301,
            moderator_id: None,
            action: ModerationAction::Flagged,
            reason: "User report threshold reached.".to_string(),
            timestamp: 1703100000,
        },
        ModerationEvent {
            event_id: 4002,
            content_id: 301,
            moderator_id: Some(9003),
            action: ModerationAction::Removed,
            reason: "Confirmed policy violation: hate speech.".to_string(),
            timestamp: 1703110000,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&events, cfg).expect("encode Vec<ModerationEvent>");
    let (decoded, consumed): (Vec<ModerationEvent>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<ModerationEvent>");
    assert_eq!(events, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 2);
}

// --- Test 19: Vec<Notification> with all notification types ---
#[test]
fn test_vec_notifications_all_types_fixed_int() {
    let notifs = vec![
        Notification {
            notif_id: 6001,
            user_id: 100020,
            sender_id: 100021,
            notif_type: NotificationType::Mention,
            read: false,
            created_at: 1704000000,
        },
        Notification {
            notif_id: 6002,
            user_id: 100020,
            sender_id: 100022,
            notif_type: NotificationType::Reply,
            read: true,
            created_at: 1704010000,
        },
        Notification {
            notif_id: 6003,
            user_id: 100020,
            sender_id: 100023,
            notif_type: NotificationType::Follow,
            read: false,
            created_at: 1704020000,
        },
        Notification {
            notif_id: 6004,
            user_id: 100020,
            sender_id: 100024,
            notif_type: NotificationType::Reaction,
            read: true,
            created_at: 1704030000,
        },
        Notification {
            notif_id: 6005,
            user_id: 100020,
            sender_id: 100025,
            notif_type: NotificationType::Direct,
            read: false,
            created_at: 1704040000,
        },
    ];
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&notifs, cfg).expect("encode Vec<Notification> all types");
    let (decoded, consumed): (Vec<Notification>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<Notification> all types");
    assert_eq!(notifs, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 5);
}

// --- Test 20: Encoding size comparison: standard vs fixed int for UserProfile ---
#[test]
fn test_encoding_size_comparison_user_profile() {
    let profile = UserProfile {
        user_id: 1,
        username: "tiny".to_string(),
        follower_count: 1,
        following_count: 1,
        verified: false,
        created_at: 1,
    };
    let cfg_std = config::standard();
    let cfg_fixed = config::standard().with_fixed_int_encoding();
    let encoded_std =
        encode_to_vec(&profile, cfg_std).expect("encode standard for size comparison");
    let encoded_fixed =
        encode_to_vec(&profile, cfg_fixed).expect("encode fixed int for size comparison");
    // fixed int encoding uses more bytes for small values (always encodes full width)
    assert!(
        encoded_fixed.len() >= encoded_std.len(),
        "fixed int encoding should be >= standard for small values; fixed={}, std={}",
        encoded_fixed.len(),
        encoded_std.len()
    );
    // Both should decode correctly
    let (decoded_std, _): (UserProfile, usize) =
        decode_owned_from_slice(&encoded_std, cfg_std).expect("decode standard size comparison");
    let (decoded_fixed, _): (UserProfile, usize) =
        decode_owned_from_slice(&encoded_fixed, cfg_fixed)
            .expect("decode fixed int size comparison");
    assert_eq!(profile, decoded_std);
    assert_eq!(profile, decoded_fixed);
}

// --- Test 21: ModerationEvent all action variants roundtrip ---
#[test]
fn test_moderation_all_action_variants_standard() {
    let actions = vec![
        ModerationAction::Approved,
        ModerationAction::Flagged,
        ModerationAction::Removed,
        ModerationAction::Escalated,
        ModerationAction::AutoHidden,
    ];
    let cfg = config::standard();
    for (idx, action) in actions.iter().enumerate() {
        let event = ModerationEvent {
            event_id: (7000 + idx) as u64,
            content_id: (400 + idx) as u64,
            moderator_id: if idx % 2 == 0 {
                Some(9000 + idx as u64)
            } else {
                None
            },
            action: action.clone(),
            reason: format!("Test reason for action variant {}", idx),
            timestamp: 1705000000 + idx as u64,
        };
        let encoded = encode_to_vec(&event, cfg).expect("encode all action variants");
        let (decoded, consumed): (ModerationEvent, usize) =
            decode_owned_from_slice(&encoded, cfg).expect("decode all action variants");
        assert_eq!(event, decoded, "mismatch at variant index {}", idx);
        assert_eq!(consumed, encoded.len());
    }
}

// --- Test 22: ReactionCount vec preserves order and count integrity ---
#[test]
fn test_reaction_count_order_integrity_big_endian() {
    let reactions: Vec<ReactionCount> = (0u8..6)
        .map(|i| ReactionCount {
            reaction_type: i,
            count: (i as u32 + 1) * 100,
        })
        .collect();
    let metric = EngagementMetric {
        content_id: 999,
        period_start: 1706000000,
        period_end: 1706259200,
        impressions: 250_000,
        clicks: 18_500,
        reactions: reactions.clone(),
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&metric, cfg).expect("encode reaction order integrity");
    let (decoded, consumed): (EngagementMetric, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reaction order integrity");
    assert_eq!(metric, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.reactions.len(), 6);
    for (i, rc) in decoded.reactions.iter().enumerate() {
        assert_eq!(
            rc.reaction_type, i as u8,
            "reaction_type mismatch at index {}",
            i
        );
        assert_eq!(
            rc.count,
            (i as u32 + 1) * 100,
            "count mismatch at index {}",
            i
        );
    }
}
