use tokio_erp::erpnext::accounts::doctype::pos_closing_entry_taxes::pos_closing_entry_taxes::PosClosingEntryTaxes;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_closing_entry_taxes_matches_erpnext_metadata() {
    assert_eq!(PosClosingEntryTaxes::DOCTYPE, "POS Closing Entry Taxes");
    assert_eq!(PosClosingEntryTaxes::MODULE, "Accounts");
    assert_eq!(
        PosClosingEntryTaxes::FIELD_ORDER,
        ["account_head", "amount"]
    );
    assert!(PosClosingEntryTaxes::IS_TABLE);
    assert!(PosClosingEntryTaxes::EDITABLE_GRID);

    assert_eq!(
        PosClosingEntryTaxes::fields(),
        vec![
            FieldSpec::link("account_head", "Account Head")
                .options("Account")
                .in_list_view()
                .read_only(),
            FieldSpec::currency("amount", "Amount")
                .in_list_view()
                .read_only(),
        ]
    );
}

#[test]
fn pos_closing_entry_taxes_preserves_pass_controller_behavior() {
    let blank = PosClosingEntryTaxes::default();
    assert_eq!(blank.account_head, None);
    assert_eq!(blank.amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosClosingEntryTaxes::new("VAT", "12.50");
    assert_eq!(row.account_head.as_deref(), Some("VAT"));
    assert_eq!(row.amount.as_deref(), Some("12.50"));
    assert_eq!(row.doctype(), "POS Closing Entry Taxes");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
