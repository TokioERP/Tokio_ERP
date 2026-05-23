use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bank_transaction::auto_match_party::{
    get_parties_in_order, AccountIbanFilters, AutoMatchByAccountIban, AutoMatchParty,
    AutoMatchResult, FuzzyExtractResult, FuzzyMatchDecision,
};

#[test]
fn get_parties_in_order_matches_erpnext_deposit_branching() {
    assert_eq!(
        get_parties_in_order(100.0),
        ["Customer", "Supplier", "Employee"]
    );
    assert_eq!(
        get_parties_in_order(0.0),
        ["Supplier", "Employee", "Customer"]
    );
    assert_eq!(
        get_parties_in_order(-10.0),
        ["Supplier", "Employee", "Customer"]
    );
}

#[test]
fn account_iban_filters_match_erpnext_bank_account_and_employee_fields() {
    let matcher = AutoMatchByAccountIban {
        bank_party_account_number: Some("000003716541159".to_string()),
        bank_party_iban: Some("DE02000000003716541159".to_string()),
    };

    assert_eq!(
        matcher.get_or_filters(None),
        AccountIbanFilters(BTreeMap::from([
            ("bank_account_no".to_string(), "000003716541159".to_string()),
            ("iban".to_string(), "DE02000000003716541159".to_string()),
        ]))
    );
    assert_eq!(
        matcher.get_or_filters(Some("Employee")),
        AccountIbanFilters(BTreeMap::from([
            ("bank_ac_no".to_string(), "000003716541159".to_string()),
            ("iban".to_string(), "DE02000000003716541159".to_string()),
        ]))
    );

    let empty = AutoMatchByAccountIban::default();
    assert_eq!(empty.get_or_filters(None), AccountIbanFilters::default());
}

#[test]
fn account_iban_match_uses_bank_account_before_employee_like_erpnext() {
    let matcher = AutoMatchByAccountIban {
        bank_party_account_number: Some("A-1".to_string()),
        bank_party_iban: None,
    };

    assert_eq!(
        matcher.match_account_in_party(
            Some(AutoMatchResult::new("Supplier", "John Doe & Co.")),
            Some("EMP-0001")
        ),
        Some(AutoMatchResult::new("Supplier", "John Doe & Co."))
    );
    assert_eq!(
        matcher.match_account_in_party(None, Some("EMP-0001")),
        Some(AutoMatchResult::new("Employee", "EMP-0001"))
    );
    assert_eq!(matcher.match_account_in_party(None, None), None);
}

#[test]
fn fuzzy_result_processing_matches_erpnext_cutoff_and_duplicate_skip_rules() {
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[]),
        FuzzyMatchDecision {
            party_name: None,
            skip: false,
        }
    );
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[FuzzyExtractResult::new(
            "Microsoft",
            80,
            "SUP-MICROSOFT",
        )]),
        FuzzyMatchDecision {
            party_name: None,
            skip: true,
        }
    );
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[FuzzyExtractResult::new(
            "Microsoft",
            81,
            "SUP-MICROSOFT",
        )]),
        FuzzyMatchDecision {
            party_name: Some("SUP-MICROSOFT".to_string()),
            skip: true,
        }
    );
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[
            FuzzyExtractResult::new("Adithya Medical & General Stores", 95, "SUP-1"),
            FuzzyExtractResult::new("Adithya Medical And General Stores", 95, "SUP-2"),
        ]),
        FuzzyMatchDecision {
            party_name: None,
            skip: true,
        }
    );
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[
            FuzzyExtractResult::new("Jackson Ella W.", 93, "SUP-JACKSON"),
            FuzzyExtractResult::new("Jackson LLC", 85, "SUP-JACKSON-LLC"),
        ]),
        FuzzyMatchDecision {
            party_name: Some("SUP-JACKSON".to_string()),
            skip: true,
        }
    );
    assert_eq!(
        FuzzyMatchDecision::from_extract_results(&[
            FuzzyExtractResult::new("Microsoft", 75, "SUP-MICROSOFT"),
            FuzzyExtractResult::new("Micro Systems", 74, "SUP-MICRO"),
        ]),
        FuzzyMatchDecision {
            party_name: None,
            skip: false,
        }
    );
}

#[test]
fn auto_match_party_sequence_prefers_account_iban_then_fuzzy_when_enabled() {
    let matcher = AutoMatchParty {
        bank_party_account_number: Some("A-1".to_string()),
        bank_party_iban: Some("IBAN-1".to_string()),
        bank_party_name: Some("Ella Jackson".to_string()),
        description: Some("payment".to_string()),
        deposit: 0.0,
    };

    assert_eq!(
        matcher.match_with_context(
            true,
            Some(AutoMatchResult::new("Supplier", "John Doe & Co.")),
            Some(AutoMatchResult::new("Supplier", "Jackson Ella W.")),
        ),
        Some(AutoMatchResult::new("Supplier", "John Doe & Co."))
    );
    assert_eq!(
        matcher.match_with_context(
            true,
            None,
            Some(AutoMatchResult::new("Supplier", "Jackson Ella W.")),
        ),
        Some(AutoMatchResult::new("Supplier", "Jackson Ella W."))
    );
    assert_eq!(
        matcher.match_with_context(
            false,
            None,
            Some(AutoMatchResult::new("Supplier", "Jackson Ella W.")),
        ),
        None
    );
}
