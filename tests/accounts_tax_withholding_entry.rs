use tokio_erp::erpnext::accounts::doctype::tax_withholding_entry::tax_withholding_entry::{
    compute_withheld_amount, TaxWithholdingEntry, TaxWithholdingEntryError,
    TaxWithholdingUpdateValues,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn tax_withholding_entry_matches_erpnext_core_metadata() {
    assert_eq!(TaxWithholdingEntry::DOCTYPE, "Tax Withholding Entry");
    assert_eq!(TaxWithholdingEntry::MODULE, "Accounts");
    assert_eq!(TaxWithholdingEntry::FIELD_ORDER.len(), 29);
    assert_eq!(TaxWithholdingEntry::FIELD_ORDER[0], "section_break_krko");
    assert_eq!(TaxWithholdingEntry::FIELD_ORDER[28], "created_by_migration");
    assert!(TaxWithholdingEntry::ALLOW_RENAME);
    assert!(TaxWithholdingEntry::IS_TABLE);
    assert!(TaxWithholdingEntry::INDEX_WEB_PAGES_FOR_SEARCH);
    let doc = TaxWithholdingEntry::default();
    assert_eq!(doc.doctype(), "Tax Withholding Entry");
    assert_eq!(doc.module(), "Accounts");

    let fields = TaxWithholdingEntry::fields();
    assert_eq!(fields.len(), 29);
    assert!(fields.contains(
        &FieldSpec::dynamic_link("party")
            .label("Party")
            .options("party_type")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::link("tax_withholding_category", "Tax Withholding Category")
            .options("Tax Withholding Category")
            .read_only()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("withholding_amount", "Base Tax Withheld")
            .options("Company:company:default_currency")
            .read_only()
            .in_list_view()
            .columns(1)
    ));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("\nSettled\nUnder Withheld\nOver Withheld\nDuplicate\nCancelled")
            .read_only()
    ));
}

#[test]
fn tax_withholding_entry_status_and_link_difference_match_erpnext() {
    let mut entry = TaxWithholdingEntry {
        parenttype: "Purchase Invoice".to_string(),
        parent: "PINV-0001".to_string(),
        taxable_doctype: Some("Purchase Invoice".to_string()),
        taxable_name: Some("PINV-0001".to_string()),
        withholding_doctype: Some("Purchase Invoice".to_string()),
        withholding_name: Some("PINV-0001".to_string()),
        ..TaxWithholdingEntry::default()
    };
    assert_eq!(entry.get_status(), "Settled");
    entry.set_status(None);
    assert_eq!(entry.status.as_deref(), Some("Settled"));
    assert!(!entry.is_taxable_different());
    assert!(!entry.is_withholding_different());

    entry.withholding_name = None;
    entry.under_withheld_reason = None;
    assert_eq!(entry.get_status(), "Under Withheld");

    entry.withholding_name = Some("PE-0001".to_string());
    entry.taxable_name = None;
    assert_eq!(entry.get_status(), "Over Withheld");

    entry.docstatus = 2;
    assert_eq!(entry.get_status(), "Cancelled");
}

#[test]
fn tax_withholding_entry_validations_match_erpnext() {
    let entry = TaxWithholdingEntry {
        idx: 7,
        parenttype: "Purchase Invoice".to_string(),
        parent: "PINV-0001".to_string(),
        taxable_doctype: Some("Purchase Invoice".to_string()),
        taxable_name: Some("PINV-OLD".to_string()),
        withholding_doctype: Some("Payment Entry".to_string()),
        withholding_name: Some("PE-0001".to_string()),
        ..TaxWithholdingEntry::default()
    };
    assert_eq!(
        entry.validate_adjustments().unwrap_err(),
        TaxWithholdingEntryError::DifferentTaxableAndWithholdingLinks(
            "Row #7: Cannot create entry with different taxable AND withholding document links."
                .to_string()
        )
    );

    let invalid_amount = TaxWithholdingEntry {
        idx: 3,
        taxable_amount: 1000.0,
        tax_rate: 10.0,
        withholding_amount: 80.0,
        withholding_name: Some("PINV-0001".to_string()),
        ..TaxWithholdingEntry::default()
    };
    assert_eq!(
        invalid_amount.validate_tax_withheld_amount(2).unwrap_err(),
        TaxWithholdingEntryError::MismatchedWithholdingAmount(
            "Row #3: Withholding Amount 80 does not match calculated amount 100.".to_string()
        )
    );

    let skipped = TaxWithholdingEntry {
        withholding_name: None,
        taxable_amount: 1000.0,
        tax_rate: 10.0,
        withholding_amount: 0.0,
        ..TaxWithholdingEntry::default()
    };
    assert!(skipped.validate_tax_withheld_amount(2).is_ok());
}

#[test]
fn tax_withholding_entry_adjustment_value_helpers_match_erpnext() {
    let entry = TaxWithholdingEntry {
        tax_rate: 5.0,
        taxable_amount: 400.0,
        taxable_doctype: Some("Purchase Invoice".to_string()),
        taxable_name: Some("PINV-0001".to_string()),
        taxable_date: Some("2026-05-23".to_string()),
        withholding_amount: 40.0,
        withholding_doctype: Some("Payment Entry".to_string()),
        withholding_name: Some("PE-0001".to_string()),
        withholding_date: Some("2026-05-24".to_string()),
        currency: Some("USD".to_string()),
        conversion_rate: 1.25,
        under_withheld_reason: Some("Lower Deduction Certificate".to_string()),
        lower_deduction_certificate: Some("LDC-1".to_string()),
        ..TaxWithholdingEntry::default()
    };

    assert_eq!(
        entry.values_to_update(0.5, "taxable"),
        TaxWithholdingUpdateValues {
            withholding_amount: Some(20.0),
            withholding_doctype: Some("Payment Entry".to_string()),
            withholding_name: Some("PE-0001".to_string()),
            withholding_date: Some("2026-05-24".to_string()),
            tax_rate: Some(5.0),
            status: Some("Duplicate".to_string()),
            under_withheld_reason: None,
            ..TaxWithholdingUpdateValues::default()
        }
    );
    assert_eq!(
        entry.values_to_update(0.5, "withholding"),
        TaxWithholdingUpdateValues {
            taxable_amount: Some(200.0),
            taxable_doctype: Some("Purchase Invoice".to_string()),
            taxable_name: Some("PINV-0001".to_string()),
            taxable_date: Some("2026-05-23".to_string()),
            tax_rate: Some(5.0),
            status: Some("Duplicate".to_string()),
            currency: Some("USD".to_string()),
            conversion_rate: Some(1.25),
            under_withheld_reason: Some("Lower Deduction Certificate".to_string()),
            lower_deduction_certificate: Some("LDC-1".to_string()),
            ..TaxWithholdingUpdateValues::default()
        }
    );
    assert_eq!(
        entry.balance_values_to_update(0.25, "taxable", 2),
        TaxWithholdingUpdateValues {
            withholding_amount: Some(30.0),
            ..TaxWithholdingUpdateValues::default()
        }
    );
}

#[test]
fn tax_withholding_compute_amount_matches_erpnext_rounding() {
    assert_eq!(compute_withheld_amount(1234.56, 10.0, false, 2), 123.46);
    assert_eq!(compute_withheld_amount(1234.56, 10.0, true, 2), 123.0);
}
