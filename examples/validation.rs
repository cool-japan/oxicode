//! Validation middleware example for oxicode
//!
//! This example demonstrates constraint-based validation for ensuring
//! data integrity and security during deserialization.
//!
//! Run with: cargo run --example validation

#[allow(unused_imports)]
use oxicode::validation::ValidationError;
use oxicode::validation::{Constraints, ValidationConfig, Validator};
use oxicode::{Decode, Encode};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct UserProfile {
    username: String,
    email: String,
    age: u8,
    bio: String,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ApiRequest {
    endpoint: String,
    payload: String,
    headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
    tags: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Validation Middleware Example\n");

    // Example 1: Basic validation with constraints
    println!("1. Basic validation with constraints:");

    // Create validator for UserProfile
    let mut user_validator: Validator<String> = Validator::new();
    user_validator.add_constraint("username", Constraints::max_len(50));
    user_validator.add_constraint("username", Constraints::min_len(3));
    user_validator.add_constraint("username", Constraints::ascii_only());

    // Valid username
    let valid_username = "alice_123".to_string();
    match user_validator.validate(&valid_username) {
        Ok(()) => println!("   ✓ Valid username: {}", valid_username),
        Err(errors) => println!("   ✗ Invalid username: {:?}", errors),
    }

    // Invalid username (too short)
    let invalid_username = "ab".to_string();
    match user_validator.validate(&invalid_username) {
        Ok(()) => println!("   ✓ Valid username: {}", invalid_username),
        Err(errors) => {
            println!("   ✗ Invalid username: {}", invalid_username);
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }

    // Invalid username (non-ASCII)
    let invalid_username = "user_日本語".to_string();
    match user_validator.validate(&invalid_username) {
        Ok(()) => println!("   ✓ Valid username: {}", invalid_username),
        Err(errors) => {
            println!("   ✗ Invalid username: {}", invalid_username);
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 2: Numeric range validation
    println!("2. Numeric range validation:");

    let mut age_validator: Validator<u8> = Validator::new();
    age_validator.add_constraint("age", Constraints::range(Some(0u8), Some(120u8)));

    // Valid age
    let valid_age = 25u8;
    match age_validator.validate(&valid_age) {
        Ok(()) => println!("   ✓ Valid age: {}", valid_age),
        Err(errors) => println!("   ✗ Invalid age: {:?}", errors),
    }

    // Invalid age (too high)
    let invalid_age = 150u8;
    match age_validator.validate(&invalid_age) {
        Ok(()) => println!("   ✓ Valid age: {}", invalid_age),
        Err(errors) => {
            println!("   ✗ Invalid age: {}", invalid_age);
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 3: Collection validation
    println!("3. Collection validation:");

    let mut tags_validator: Validator<Vec<String>> = Validator::new();
    tags_validator.add_constraint("tags", Constraints::max_len(10));
    tags_validator.add_constraint("tags", Constraints::min_len(1));
    tags_validator.add_constraint("tags", Constraints::non_empty());

    // Valid tags
    let valid_tags = vec!["rust".to_string(), "serialization".to_string()];
    match tags_validator.validate(&valid_tags) {
        Ok(()) => println!("   ✓ Valid tags: {:?}", valid_tags),
        Err(errors) => println!("   ✗ Invalid tags: {:?}", errors),
    }

    // Invalid tags (empty)
    let invalid_tags: Vec<String> = vec![];
    match tags_validator.validate(&invalid_tags) {
        Ok(()) => println!("   ✓ Valid tags: {:?}", invalid_tags),
        Err(errors) => {
            println!("   ✗ Invalid tags: {:?}", invalid_tags);
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 4: Fail-fast vs collect-all-errors
    println!("4. Fail-fast vs collect-all-errors:");

    let invalid_username = "ab".to_string(); // Too short AND will be non-empty check

    // Fail-fast mode (default)
    let config_fail_fast = ValidationConfig::new().with_fail_fast(true);
    let mut validator_fail_fast: Validator<String> = Validator::with_config(config_fail_fast);
    validator_fail_fast.add_constraint("username", Constraints::min_len(3));
    validator_fail_fast.add_constraint("username", Constraints::max_len(20));
    validator_fail_fast.add_constraint("username", Constraints::non_empty());

    match validator_fail_fast.validate(&invalid_username) {
        Ok(()) => println!("   ✓ Valid (fail-fast)"),
        Err(errors) => {
            println!("   ✗ Fail-fast mode: {} error(s)", errors.len());
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }

    // Collect-all-errors mode
    let config_collect_all = ValidationConfig::new().with_fail_fast(false);
    let mut validator_collect: Validator<String> = Validator::with_config(config_collect_all);
    validator_collect.add_constraint("username", Constraints::min_len(3));
    validator_collect.add_constraint("username", Constraints::max_len(20));

    let long_invalid = "this_is_a_very_long_username_that_exceeds_limit".to_string();
    match validator_collect.validate(&long_invalid) {
        Ok(()) => println!("   ✓ Valid (collect-all)"),
        Err(errors) => {
            println!("   ✗ Collect-all mode: {} error(s)", errors.len());
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 5: Chaining constraints with builder pattern
    println!("5. Chaining constraints with builder pattern:");

    let email_validator: Validator<String> = Validator::new()
        .constraint("email", Constraints::max_len(255))
        .constraint("email", Constraints::min_len(5))
        .constraint("email", Constraints::non_empty())
        .constraint("email", Constraints::ascii_only());

    let valid_email = "alice@example.com".to_string();
    match email_validator.validate(&valid_email) {
        Ok(()) => println!("   ✓ Valid email: {}", valid_email),
        Err(errors) => println!("   ✗ Invalid email: {:?}", errors),
    }

    let invalid_email = "ab".to_string();
    match email_validator.validate(&invalid_email) {
        Ok(()) => println!("   ✓ Valid email: {}", invalid_email),
        Err(errors) => {
            println!("   ✗ Invalid email: {}", invalid_email);
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 6: Validating decoded data
    println!("6. Validating decoded data:");

    let product = Product {
        name: "Laptop".to_string(),
        description: "High-performance laptop".to_string(),
        price: 999.99,
        quantity: 10,
        tags: vec!["electronics".to_string(), "computers".to_string()],
    };

    // Encode
    let bytes = oxicode::encode_to_vec(&product)?;
    println!("   Encoded product: {} bytes", bytes.len());

    // Decode and validate
    let (decoded, _): (Product, _) = oxicode::decode_from_slice(&bytes)?;

    // Validate product name
    let name_validator: Validator<String> = Validator::new()
        .constraint("name", Constraints::max_len(100))
        .constraint("name", Constraints::min_len(1))
        .constraint("name", Constraints::non_empty());

    match name_validator.validate(&decoded.name) {
        Ok(()) => println!("   ✓ Valid product name: {}", decoded.name),
        Err(errors) => {
            println!("   ✗ Invalid product name");
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }

    // Validate price range
    let price_validator: Validator<f64> =
        Validator::new().constraint("price", Constraints::range(Some(0.0), Some(1_000_000.0)));

    match price_validator.validate(&decoded.price) {
        Ok(()) => println!("   ✓ Valid price: ${:.2}", decoded.price),
        Err(errors) => {
            println!("   ✗ Invalid price");
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
        }
    }
    println!();

    // Example 7: Security use case - validating untrusted input
    println!("7. Security use case - validating untrusted input:");

    let api_request = ApiRequest {
        endpoint: "/api/users".to_string(),
        payload: "valid payload".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
    };

    let bytes = oxicode::encode_to_vec(&api_request)?;

    // Simulate receiving data from untrusted source
    let (decoded_request, _): (ApiRequest, _) = oxicode::decode_from_slice(&bytes)?;

    // Validate endpoint
    let endpoint_validator: Validator<String> = Validator::new()
        .constraint("endpoint", Constraints::max_len(1024))
        .constraint("endpoint", Constraints::ascii_only())
        .constraint("endpoint", Constraints::non_empty());

    match endpoint_validator.validate(&decoded_request.endpoint) {
        Ok(()) => println!("   ✓ Valid endpoint: {}", decoded_request.endpoint),
        Err(errors) => {
            println!("   ✗ Rejected untrusted endpoint");
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
            println!("   Security: Request rejected!");
        }
    }

    // Validate payload size
    let payload_validator: Validator<String> =
        Validator::new().constraint("payload", Constraints::max_len(10_485_760)); // 10MB limit

    match payload_validator.validate(&decoded_request.payload) {
        Ok(()) => println!(
            "   ✓ Valid payload size: {} bytes",
            decoded_request.payload.len()
        ),
        Err(errors) => {
            println!("   ✗ Rejected oversized payload");
            for error in errors {
                println!("      - {}: {}", error.field, error.message);
            }
            println!("   Security: DoS protection activated!");
        }
    }
    println!();

    // Example 8: Validation best practices
    println!("8. Validation best practices:");
    println!("   ✓ Always validate untrusted input before processing");
    println!("   ✓ Set reasonable size limits to prevent DoS attacks");
    println!("   ✓ Use fail-fast for performance, collect-all for UX");
    println!("   ✓ Combine multiple constraints for defense-in-depth");
    println!("   ✓ Validate at system boundaries (network, storage)");
    println!("   ✓ Use domain-specific validators for complex rules\n");

    println!("All validation examples completed successfully!");

    Ok(())
}
