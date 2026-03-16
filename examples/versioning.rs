//! Schema versioning example for OxiCode
//!
//! This example demonstrates how to use OxiCode's schema evolution features
//! to maintain backward compatibility when your data format changes.
//!
//! Run with: cargo run --example versioning

use oxicode::versioning::{
    can_migrate, check_compatibility, decode_versioned, encode_versioned, migration_path,
    CompatibilityLevel, Version,
};
use oxicode::{Decode, Encode};

/// Version 1.0.0 data format
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserV1 {
    id: u64,
    name: String,
    email: String,
}

/// Version 2.0.0 data format (added role field)
#[allow(dead_code)]
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserV2 {
    id: u64,
    name: String,
    email: String,
    role: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OxiCode Schema Versioning Example\n");

    let v1_0_0 = Version::new(1, 0, 0);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    // Example 1: Encode data with version header
    println!("1. Encoding with version header:");

    let user_v1 = UserV1 {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let raw_data = oxicode::encode_to_vec(&user_v1)?;
    let versioned = encode_versioned(&raw_data, v1_0_0)?;

    println!("   Raw data: {} bytes", raw_data.len());
    println!(
        "   Versioned: {} bytes (+ {} byte header)",
        versioned.len(),
        versioned.len() - raw_data.len()
    );

    // Decode with version info — decode_versioned returns (payload, version)
    let (decoded_data, found_version) = decode_versioned(&versioned)?;
    println!("   Decoded version: {}", found_version);
    assert_eq!(found_version, v1_0_0);

    let (user, _): (UserV1, _) = oxicode::decode_from_slice(&decoded_data)?;
    assert_eq!(user_v1, user);
    println!("   Version 1.0.0 roundtrip verified");

    // Example 2: Compatibility checking
    println!("\n2. Compatibility checking:");

    let compat_same = check_compatibility(v1_0_0, v1_0_0, None);
    println!("   v1.0.0 vs v1.0.0: {:?}", compat_same);
    assert_eq!(compat_same, CompatibilityLevel::Compatible);

    // data v1.0.0 with reader at v1.1.0 — older minor => CompatibleWithWarnings
    let compat_minor = check_compatibility(v1_0_0, v1_1_0, None);
    println!("   v1.0.0 data with v1.1.0 reader: {:?}", compat_minor);
    assert_eq!(compat_minor, CompatibilityLevel::CompatibleWithWarnings);

    // data v1.0.0 with reader at v2.0.0 — different major => Incompatible
    let compat_major = check_compatibility(v1_0_0, v2_0_0, None);
    println!("   v1.0.0 data with v2.0.0 reader: {:?}", compat_major);
    assert_eq!(compat_major, CompatibilityLevel::Incompatible);

    // Example 3: Migration path
    println!("\n3. Migration paths:");

    let can_1_to_2 = can_migrate(v1_0_0, v2_0_0);
    let path_1_to_2 = migration_path(v1_0_0, v2_0_0);
    println!(
        "   v1.0.0 → v2.0.0: can_migrate={}, steps={}",
        can_1_to_2,
        path_1_to_2.len()
    );

    let v3_0_0 = Version::new(3, 0, 0);
    let path_1_to_3 = migration_path(v1_0_0, v3_0_0);
    println!(
        "   v1.0.0 → v3.0.0: {} intermediate step(s)",
        path_1_to_3.len()
    );
    if !path_1_to_3.is_empty() {
        println!("   Steps: {:?}", path_1_to_3);
    }

    // Example 4: Minimum version enforcement
    println!("\n4. Minimum version enforcement:");

    let min_required = Version::new(1, 2, 0);
    let old_data_version = Version::new(1, 1, 0);
    let current_version = Version::new(2, 0, 0);

    let compat = check_compatibility(old_data_version, current_version, Some(min_required));
    println!(
        "   Old data v{} with min_required v{}: {:?}",
        old_data_version, min_required, compat
    );
    assert_eq!(compat, CompatibilityLevel::Incompatible);
    println!("   Minimum version enforcement works");

    // Example 5: is_versioned and extract_version helpers
    println!("\n5. Version detection helpers:");

    let raw_unversioned = oxicode::encode_to_vec(&user_v1)?;
    println!(
        "   Raw data is_versioned: {}",
        oxicode::versioning::is_versioned(&raw_unversioned)
    );
    assert!(!oxicode::versioning::is_versioned(&raw_unversioned));

    let with_header = encode_versioned(&raw_unversioned, v1_1_0)?;
    println!(
        "   Wrapped data is_versioned: {}",
        oxicode::versioning::is_versioned(&with_header)
    );
    assert!(oxicode::versioning::is_versioned(&with_header));

    let extracted = oxicode::versioning::extract_version(&with_header)?;
    println!("   Extracted version: {}", extracted);
    assert_eq!(extracted, v1_1_0);
    println!("   Version detection verified");

    // Example 6: Best practices summary
    println!("\n6. Best practices for schema evolution:");
    println!("   Use semantic versioning: major.minor.patch");
    println!("   Increment major for breaking changes");
    println!("   Increment minor for backward-compatible additions");
    println!("   Increment patch for bug fixes");
    println!("   Always check compatibility before decoding");
    println!("   Provide migration paths between major versions");

    println!("\nVersioning example completed successfully!");
    Ok(())
}
