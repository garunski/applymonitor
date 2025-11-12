//! Tests for system email detection

use api_main::services::db::system_email_domains::matches_pattern;

#[test]
fn test_noreply_pattern_matches_local_part() {
    // Test that noreply.* pattern matches local part starting with noreply
    assert!(matches_pattern("noreply.*", "noreply"));
    assert!(matches_pattern("noreply.*", "noreply-test"));
    assert!(matches_pattern("noreply.*", "noreply123"));
    assert!(!matches_pattern("noreply.*", "test-noreply"));
    assert!(!matches_pattern("noreply.*", "reply"));
}

#[test]
fn test_domain_patterns() {
    // Test domain wildcard patterns
    assert!(matches_pattern("*.greenhouse.io", "greenhouse.io"));
    assert!(matches_pattern("*.greenhouse.io", "app.greenhouse.io"));
    assert!(matches_pattern("*.greenhouse.io", "anything.greenhouse.io"));
    assert!(!matches_pattern("*.greenhouse.io", "greenhouse.com"));
    assert!(!matches_pattern("*.greenhouse.io", "other.io"));
}

#[test]
fn test_exact_domain_match() {
    // Test exact domain matches
    assert!(matches_pattern("mailchimp.com", "mailchimp.com"));
    assert!(!matches_pattern("mailchimp.com", "mailchimp.org"));
}

#[test]
fn test_prefix_wildcard() {
    // Test prefix* patterns
    assert!(matches_pattern("noreply*", "noreply"));
    assert!(matches_pattern("noreply*", "noreply-test"));
    assert!(matches_pattern("noreply*", "noreply123"));
    assert!(!matches_pattern("noreply*", "test-noreply"));
}

#[test]
fn test_suffix_wildcard() {
    // Test *suffix patterns
    assert!(matches_pattern("*.io", "greenhouse.io"));
    assert!(matches_pattern("*.io", "test.io"));
    assert!(!matches_pattern("*.io", "greenhouse.com"));
}

#[test]
fn test_url_decoding_logic() {
    // Test URL decoding (simulating the logic)
    let email1 = "noreply%40steampowered.com";
    let decoded1 = email1.replace("%40", "@").replace("%2E", ".");
    assert_eq!(decoded1, "noreply@steampowered.com");

    let email2 = "test%2Eexample%40domain.com";
    let decoded2 = email2.replace("%40", "@").replace("%2E", ".");
    assert_eq!(decoded2, "test.example@domain.com");

    // Extract local part and domain
    let (local_part, domain) = if let Some(at_pos) = decoded1.find('@') {
        (&decoded1[..at_pos], &decoded1[at_pos + 1..])
    } else {
        ("", "")
    };
    assert_eq!(local_part, "noreply");
    assert_eq!(domain, "steampowered.com");

    // Verify pattern matching works on decoded email
    assert!(matches_pattern("noreply.*", local_part));
}

#[test]
fn test_full_email_scenarios() {
    // Test full email scenarios (simulating the full flow)
    let test_cases = vec![
        ("noreply%40steampowered.com", "noreply.*", true),
        ("noreply@steampowered.com", "noreply.*", true),
        ("noreply-test@domain.com", "noreply.*", true),
        ("test@noreply.com", "noreply.*", true), // domain "noreply.com" matches pattern (current behavior checks both)
        ("anything@greenhouse.io", "*.greenhouse.io", true),
        ("test@mailchimp.com", "mailchimp.com", true),
    ];

    for (email, pattern, should_match) in test_cases {
        // Decode email
        let decoded = email.replace("%40", "@").replace("%2E", ".");

        // Extract parts
        if let Some(at_pos) = decoded.find('@') {
            let local_part = &decoded[..at_pos];
            let domain = &decoded[at_pos + 1..];

            // Check pattern matching
            let matches = matches_pattern(pattern, local_part) || matches_pattern(pattern, domain);

            assert_eq!(
                matches, should_match,
                "Email: {}, Pattern: {}, Expected: {}, Got: {}",
                email, pattern, should_match, matches
            );
        }
    }
}
