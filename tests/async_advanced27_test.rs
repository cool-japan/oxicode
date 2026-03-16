//! Advanced async streaming tests (27th set) for OxiCode.
//!
//! Theme: Email / messaging system.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types: `EmailPriority`, `Email`, `EmailFolder`.
//!
//! Coverage matrix:
//!   1:  EmailPriority::Low single roundtrip via duplex
//!   2:  EmailPriority::Urgent single roundtrip via duplex
//!   3:  Email with Normal priority roundtrip via duplex
//!   4:  Email with Urgent priority and attachments roundtrip via duplex
//!   5:  Email with empty body and empty recipient list
//!   6:  EmailFolder roundtrip via duplex
//!   7:  EmailFolder with zero unread_count roundtrip
//!   8:  Five Emails in order via write_item / read_item
//!   9:  write_all / read_all for Vec<Email> (8 items)
//!  10:  Large batch of 100 Emails via write_all, verify read_all
//!  11:  Vec<EmailFolder> stream of 10 folders roundtrip
//!  12:  progress().items_processed > 0 after reading emails
//!  13:  StreamingConfig with chunk_size(512) forces multiple chunks
//!  14:  flush_per_item produces correct items_processed per Email
//!  15:  Empty stream returns None on first read_item
//!  16:  is_finished() true after email stream exhausted
//!  17:  bytes_processed grows after reading more emails
//!  18:  Sync encode / async decode interop for Email
//!  19:  Async encode / sync decode interop for EmailFolder
//!  20:  Vec<EmailPriority> all variants roundtrip
//!  21:  Email with multiple recipients roundtrip
//!  22:  tokio::join! concurrent encode/decode for email feed replay

#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder, StreamingConfig};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmailPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Email {
    id: u64,
    from: String,
    to: Vec<String>,
    subject: String,
    body: String,
    priority: EmailPriority,
    has_attachments: bool,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmailFolder {
    folder_id: u32,
    name: String,
    message_count: u32,
    unread_count: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_email(id: u64, from: &str, to: &[&str], priority: EmailPriority) -> Email {
    Email {
        id,
        from: from.to_string(),
        to: to.iter().map(|s| s.to_string()).collect(),
        subject: format!("Subject of email {id}"),
        body: format!("Body text for email {id}. This is the message content."),
        priority,
        has_attachments: id % 3 == 0,
        timestamp_s: 1_700_000_000 + id * 60,
    }
}

fn make_folder(folder_id: u32, name: &str, message_count: u32, unread_count: u32) -> EmailFolder {
    EmailFolder {
        folder_id,
        name: name.to_string(),
        message_count,
        unread_count,
    }
}

fn make_email_batch(count: usize) -> Vec<Email> {
    let senders = ["alice@example.com", "bob@example.com", "carol@example.com"];
    let recipients = ["dave@example.com", "eve@example.com", "frank@example.com"];
    (0..count)
        .map(|i| {
            let priority = match i % 4 {
                0 => EmailPriority::Low,
                1 => EmailPriority::Normal,
                2 => EmailPriority::High,
                _ => EmailPriority::Urgent,
            };
            let sender = senders[i % senders.len()];
            let recipient = recipients[i % recipients.len()];
            make_email(i as u64, sender, &[recipient], priority)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: EmailPriority::Low single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_priority_low_roundtrip() {
    let priority = EmailPriority::Low;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&priority)
        .await
        .expect("write_item EmailPriority::Low failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EmailPriority = dec
        .read_item()
        .await
        .expect("read_item EmailPriority::Low failed")
        .expect("expected Some(EmailPriority::Low)");
    assert_eq!(priority, got, "EmailPriority::Low roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: EmailPriority::Urgent single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_priority_urgent_roundtrip() {
    let priority = EmailPriority::Urgent;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&priority)
        .await
        .expect("write_item EmailPriority::Urgent failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EmailPriority = dec
        .read_item()
        .await
        .expect("read_item EmailPriority::Urgent failed")
        .expect("expected Some(EmailPriority::Urgent)");
    assert_eq!(priority, got, "EmailPriority::Urgent roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: Email with Normal priority roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_normal_priority_roundtrip() {
    let email = Email {
        id: 1001,
        from: "alice@example.com".to_string(),
        to: vec!["bob@example.com".to_string()],
        subject: "Hello from Alice".to_string(),
        body: "Hi Bob, hope you are well.".to_string(),
        priority: EmailPriority::Normal,
        has_attachments: false,
        timestamp_s: 1_700_000_100,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&email)
        .await
        .expect("write_item Email(Normal) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Email = dec
        .read_item()
        .await
        .expect("read_item Email(Normal) failed")
        .expect("expected Some(Email)");
    assert_eq!(email, got, "Email with Normal priority roundtrip mismatch");
    assert_eq!(
        got.priority,
        EmailPriority::Normal,
        "priority must be Normal"
    );
    assert!(!got.has_attachments, "has_attachments must be false");
}

// ---------------------------------------------------------------------------
// Test 4: Email with Urgent priority and attachments roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_urgent_with_attachments_roundtrip() {
    let email = Email {
        id: 2002,
        from: "carol@example.com".to_string(),
        to: vec![
            "dave@example.com".to_string(),
            "eve@example.com".to_string(),
        ],
        subject: "URGENT: Action required immediately".to_string(),
        body: "Please review the attached documents before the meeting.".to_string(),
        priority: EmailPriority::Urgent,
        has_attachments: true,
        timestamp_s: 1_700_000_200,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&email)
        .await
        .expect("write_item Email(Urgent) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Email = dec
        .read_item()
        .await
        .expect("read_item Email(Urgent) failed")
        .expect("expected Some(Email)");
    assert_eq!(email, got, "Email with Urgent priority roundtrip mismatch");
    assert_eq!(
        got.priority,
        EmailPriority::Urgent,
        "priority must be Urgent"
    );
    assert!(got.has_attachments, "has_attachments must be true");
    assert_eq!(got.to.len(), 2, "must have exactly 2 recipients");
}

// ---------------------------------------------------------------------------
// Test 5: Email with empty body and empty recipient list
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_empty_body_and_recipients_roundtrip() {
    let email = Email {
        id: 3003,
        from: "system@example.com".to_string(),
        to: vec![],
        subject: "Draft".to_string(),
        body: String::new(),
        priority: EmailPriority::Low,
        has_attachments: false,
        timestamp_s: 0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&email)
        .await
        .expect("write_item Email(empty body) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Email = dec
        .read_item()
        .await
        .expect("read_item Email(empty body) failed")
        .expect("expected Some(Email) with empty body");
    assert_eq!(email, got, "Email with empty body roundtrip mismatch");
    assert!(got.body.is_empty(), "body must be empty");
    assert!(got.to.is_empty(), "recipient list must be empty");
    assert_eq!(got.timestamp_s, 0, "timestamp_s must be 0");
}

// ---------------------------------------------------------------------------
// Test 6: EmailFolder roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_folder_roundtrip() {
    let folder = EmailFolder {
        folder_id: 1,
        name: "Inbox".to_string(),
        message_count: 250,
        unread_count: 17,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&folder)
        .await
        .expect("write_item EmailFolder failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EmailFolder = dec
        .read_item()
        .await
        .expect("read_item EmailFolder failed")
        .expect("expected Some(EmailFolder)");
    assert_eq!(folder, got, "EmailFolder roundtrip mismatch");
    assert_eq!(got.name, "Inbox", "folder name must be Inbox");
    assert!(
        got.unread_count <= got.message_count,
        "unread_count must not exceed message_count"
    );
}

// ---------------------------------------------------------------------------
// Test 7: EmailFolder with zero unread_count roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_folder_zero_unread_roundtrip() {
    let folder = EmailFolder {
        folder_id: 42,
        name: "Sent".to_string(),
        message_count: 1_000,
        unread_count: 0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&folder)
        .await
        .expect("write_item EmailFolder(zero unread) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EmailFolder = dec
        .read_item()
        .await
        .expect("read_item EmailFolder(zero unread) failed")
        .expect("expected Some(EmailFolder) with zero unread");
    assert_eq!(
        folder, got,
        "EmailFolder with zero unread roundtrip mismatch"
    );
    assert_eq!(got.unread_count, 0, "unread_count must be zero");
    assert_eq!(got.message_count, 1_000, "message_count must be 1000");
}

// ---------------------------------------------------------------------------
// Test 8: Five Emails in order via write_item / read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_five_emails_in_order() {
    let emails = vec![
        make_email(
            10,
            "alice@example.com",
            &["bob@example.com"],
            EmailPriority::Normal,
        ),
        make_email(
            11,
            "bob@example.com",
            &["carol@example.com"],
            EmailPriority::High,
        ),
        make_email(
            12,
            "carol@example.com",
            &["dave@example.com"],
            EmailPriority::Low,
        ),
        make_email(
            13,
            "dave@example.com",
            &["eve@example.com"],
            EmailPriority::Urgent,
        ),
        make_email(
            14,
            "eve@example.com",
            &["alice@example.com"],
            EmailPriority::Normal,
        ),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for email in &emails {
        enc.write_item(email)
            .await
            .expect("write_item in 5-email sequence failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    for expected in &emails {
        let got: Email = dec
            .read_item()
            .await
            .expect("read_item in 5-email sequence failed")
            .expect("expected Some(Email)");
        assert_eq!(*expected, got, "Email mismatch at email id {}", expected.id);
    }

    let eof: Option<Email> = dec.read_item().await.expect("eof read_item failed");
    assert_eq!(eof, None, "expected None after all five emails");
}

// ---------------------------------------------------------------------------
// Test 9: write_all / read_all for Vec<Email> (8 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_write_all_read_all_8_emails() {
    let emails: Vec<Email> = (0u64..8)
        .map(|i| {
            let priority = match i % 4 {
                0 => EmailPriority::Low,
                1 => EmailPriority::Normal,
                2 => EmailPriority::High,
                _ => EmailPriority::Urgent,
            };
            make_email(
                i,
                "sender@example.com",
                &["recipient@example.com"],
                priority,
            )
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(emails.clone().into_iter())
        .await
        .expect("write_all 8 Emails failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<Email> = dec.read_all().await.expect("read_all 8 Emails failed");
    assert_eq!(emails, got, "write_all/read_all 8-email roundtrip mismatch");
    assert_eq!(got.len(), 8, "must decode exactly 8 Emails");
}

// ---------------------------------------------------------------------------
// Test 10: Large batch of 100 Emails via write_all, verify read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_large_batch_100_emails_write_all_read_all() {
    let emails = make_email_batch(100);
    assert_eq!(emails.len(), 100, "must generate exactly 100 emails");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(emails.clone().into_iter())
        .await
        .expect("write_all 100 Emails failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<Email> = dec.read_all().await.expect("read_all 100 Emails failed");
    assert_eq!(got.len(), 100, "expected 100 decoded Emails");
    assert_eq!(emails, got, "large batch 100-email roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 11: Vec<EmailFolder> stream of 10 folders roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_folder_stream_10_folders() {
    let folder_names = [
        "Inbox",
        "Sent",
        "Drafts",
        "Spam",
        "Trash",
        "Archive",
        "Work",
        "Personal",
        "Promotions",
        "Social",
    ];
    let folders: Vec<EmailFolder> = folder_names
        .iter()
        .enumerate()
        .map(|(i, name)| make_folder(i as u32, name, (i as u32 + 1) * 10, i as u32 % 5))
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(folders.clone().into_iter())
        .await
        .expect("write_all EmailFolders failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<EmailFolder> = dec.read_all().await.expect("read_all EmailFolders failed");
    assert_eq!(got.len(), 10, "must decode exactly 10 EmailFolders");
    assert_eq!(folders, got, "EmailFolder stream roundtrip mismatch");

    for folder in &got {
        assert!(
            folder.unread_count <= folder.message_count,
            "unread_count must not exceed message_count for folder {}",
            folder.name
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: progress().items_processed > 0 after reading emails
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_progress_items_processed_after_reading_emails() {
    const N: u64 = 9;
    let emails = make_email_batch(N as usize);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(emails.clone().into_iter())
        .await
        .expect("write_all for progress test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<Email> = dec
        .read_all()
        .await
        .expect("read_all for progress test failed");

    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading emails"
    );
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal N={N} after reading all emails"
    );
}

// ---------------------------------------------------------------------------
// Test 13: StreamingConfig with chunk_size(512) forces multiple chunks
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_streaming_config_small_chunk_forces_multiple_chunks() {
    let config = StreamingConfig::new().with_chunk_size(512);
    // Each Email with subject/body/to fields is ~80-150 bytes; 40 emails ~4000+ bytes → multiple 512-byte chunks
    let emails = make_email_batch(40);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for email in &emails {
        enc.write_item(email)
            .await
            .expect("write_item with chunk_size 512 failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<Email> = dec
        .read_all()
        .await
        .expect("read_all with chunk_size 512 failed");

    assert_eq!(got.len(), 40, "must decode 40 Emails");
    assert_eq!(emails, got, "small-chunk roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading with small chunk size"
    );
}

// ---------------------------------------------------------------------------
// Test 14: flush_per_item produces correct items_processed per Email
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_flush_per_item_correct_items_processed() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let emails: Vec<Email> = (0u64..7)
        .map(|i| {
            make_email(
                i,
                "flush@example.com",
                &["recv@example.com"],
                EmailPriority::High,
            )
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for email in &emails {
        enc.write_item(email)
            .await
            .expect("write_item flush_per_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<Email> = dec
        .read_all()
        .await
        .expect("read_all flush_per_item failed");

    assert_eq!(got, emails, "flush_per_item roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after flush_per_item read"
    );
    assert_eq!(
        dec.progress().items_processed,
        7,
        "items_processed must equal 7 after reading 7 flush_per_item emails"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Empty stream returns None on first read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_empty_stream_returns_none() {
    let (client, server) = tokio::io::duplex(65536);

    let enc = AsyncEncoder::new(client);
    enc.finish()
        .await
        .expect("finish empty email stream failed");

    let mut dec = AsyncDecoder::new(server);
    let item: Option<Email> = dec
        .read_item()
        .await
        .expect("read_item from empty email stream failed");
    assert_eq!(
        item, None,
        "empty email stream must return None on first read_item"
    );
}

// ---------------------------------------------------------------------------
// Test 16: is_finished() true after email stream exhausted
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_is_finished_after_email_stream_exhausted() {
    let emails = vec![
        make_email(
            1,
            "alice@example.com",
            &["bob@example.com"],
            EmailPriority::Normal,
        ),
        make_email(
            2,
            "carol@example.com",
            &["dave@example.com"],
            EmailPriority::Low,
        ),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for email in &emails {
        enc.write_item(email).await.expect("write_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    assert!(
        !dec.is_finished(),
        "decoder must not be finished before reading"
    );

    let _: Option<Email> = dec.read_item().await.expect("read email 1 failed");
    let _: Option<Email> = dec.read_item().await.expect("read email 2 failed");

    let eof: Option<Email> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at end of email stream");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() after email stream is exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 17: bytes_processed grows after reading more emails
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_bytes_processed_grows_with_more_emails() {
    let emails = make_email_batch(12);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(emails.clone().into_iter())
        .await
        .expect("write_all for bytes_processed test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    let first: Email = dec
        .read_item()
        .await
        .expect("read first Email failed")
        .expect("expected Some(Email) for first email");
    assert_eq!(first, emails[0], "first decoded Email mismatch");

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first email"
    );

    let rest: Vec<Email> = dec
        .read_all()
        .await
        .expect("read_all remaining emails failed");
    assert_eq!(rest.len(), 11, "must decode 11 remaining emails");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow: was {bytes_after_one}, now {bytes_after_all}"
    );
    assert!(
        dec.progress().items_processed >= 12,
        "items_processed must be >= 12 after reading all emails"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Sync encode / async decode interop for Email
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_sync_encode_async_decode_interop_email() {
    let email = Email {
        id: 9_999_999,
        from: "system@example.com".to_string(),
        to: vec!["admin@example.com".to_string()],
        subject: "Boundary test email".to_string(),
        body: "x".repeat(2048),
        priority: EmailPriority::Urgent,
        has_attachments: true,
        timestamp_s: u64::MAX / 8,
    };

    // Sync encode for consistency baseline
    let sync_bytes = encode_to_vec(&email).expect("sync encode Email failed");
    let (sync_decoded, _): (Email, _) =
        decode_from_slice(&sync_bytes).expect("sync decode Email failed");
    assert_eq!(email, sync_decoded, "sync Email roundtrip mismatch");

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&email)
        .await
        .expect("async write_item for interop test failed");
    enc.finish().await.expect("finish for interop test failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: Email = dec
        .read_item()
        .await
        .expect("async read_item for interop test failed")
        .expect("expected Some(Email) in interop test");
    assert_eq!(
        email, async_decoded,
        "async encode/decode Email interop mismatch"
    );
    assert_eq!(
        async_decoded.priority,
        EmailPriority::Urgent,
        "priority must be Urgent after async decode"
    );
    assert!(
        async_decoded.has_attachments,
        "has_attachments must be true after async decode"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Async encode / sync decode interop for EmailFolder
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_async_encode_sync_decode_interop_email_folder() {
    let folder = EmailFolder {
        folder_id: 7,
        name: "Work Projects".to_string(),
        message_count: 5_000,
        unread_count: 42,
    };

    // Sync encode then sync decode for consistency baseline
    let sync_bytes = encode_to_vec(&folder).expect("sync encode EmailFolder failed");
    let (sync_decoded, _): (EmailFolder, _) =
        decode_from_slice(&sync_bytes).expect("sync decode EmailFolder failed");
    assert_eq!(folder, sync_decoded, "sync EmailFolder roundtrip mismatch");

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&folder)
        .await
        .expect("async write_item EmailFolder failed");
    enc.finish().await.expect("finish EmailFolder failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: EmailFolder = dec
        .read_item()
        .await
        .expect("async read_item EmailFolder failed")
        .expect("expected Some(EmailFolder)");
    assert_eq!(
        folder, async_decoded,
        "async encode/decode EmailFolder interop mismatch"
    );
    assert_eq!(
        async_decoded.unread_count, 42,
        "decoded unread_count must be 42"
    );
    assert_eq!(
        async_decoded.message_count, 5_000,
        "decoded message_count must be 5000"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Vec<EmailPriority> all variants roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_vec_email_priority_all_variants_roundtrip() {
    let variants = vec![
        EmailPriority::Low,
        EmailPriority::Normal,
        EmailPriority::High,
        EmailPriority::Urgent,
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&variants)
        .await
        .expect("write_item Vec<EmailPriority> all variants failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<EmailPriority> = dec
        .read_item()
        .await
        .expect("read_item Vec<EmailPriority> all variants failed")
        .expect("expected Some(Vec<EmailPriority>)");
    assert_eq!(
        variants, got,
        "Vec<EmailPriority> all-variants roundtrip mismatch"
    );
    assert_eq!(
        got.len(),
        4,
        "decoded Vec<EmailPriority> must have 4 variants"
    );
    assert_eq!(got[0], EmailPriority::Low, "first variant must be Low");
    assert_eq!(got[3], EmailPriority::Urgent, "last variant must be Urgent");
}

// ---------------------------------------------------------------------------
// Test 21: Email with multiple recipients roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_email_multiple_recipients_roundtrip() {
    let recipients: Vec<String> = (0..10)
        .map(|i| format!("recipient{i}@example.com"))
        .collect();
    let email = Email {
        id: 5_555,
        from: "broadcast@example.com".to_string(),
        to: recipients.clone(),
        subject: "Team announcement".to_string(),
        body: "Please join us for the team meeting next Monday.".to_string(),
        priority: EmailPriority::High,
        has_attachments: false,
        timestamp_s: 1_700_500_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&email)
        .await
        .expect("write_item Email(multiple recipients) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Email = dec
        .read_item()
        .await
        .expect("read_item Email(multiple recipients) failed")
        .expect("expected Some(Email) with multiple recipients");
    assert_eq!(
        email, got,
        "Email with multiple recipients roundtrip mismatch"
    );
    assert_eq!(
        got.to.len(),
        10,
        "decoded Email must have exactly 10 recipients"
    );
    assert_eq!(
        got.to[0], "recipient0@example.com",
        "first recipient must be recipient0@example.com"
    );
    assert_eq!(
        got.to[9], "recipient9@example.com",
        "last recipient must be recipient9@example.com"
    );
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode for email feed replay
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_email27_concurrent_encode_decode_email_feed_replay() {
    let emails = make_email_batch(22);
    let emails_for_enc = emails.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(emails_for_enc.into_iter())
                .await
                .expect("concurrent write_all email feed failed");
            enc.finish().await.expect("concurrent finish failed");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            let decoded: Vec<Email> = dec
                .read_all()
                .await
                .expect("concurrent read_all email feed failed");
            decoded
        }
    );

    assert_eq!(
        got.len(),
        22,
        "must decode 22 emails from concurrent stream"
    );
    assert_eq!(
        emails, got,
        "concurrent email feed replay roundtrip mismatch"
    );
}
