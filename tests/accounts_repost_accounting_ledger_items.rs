use tokio_erp::erpnext::accounts::doctype::repost_accounting_ledger_items::repost_accounting_ledger_items::RepostAccountingLedgerItems;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn repost_accounting_ledger_items_matches_erpnext_metadata() {
    assert_eq!(
        RepostAccountingLedgerItems::DOCTYPE,
        "Repost Accounting Ledger Items"
    );
    assert_eq!(RepostAccountingLedgerItems::MODULE, "Accounts");
    assert_eq!(
        RepostAccountingLedgerItems::FIELD_ORDER,
        ["voucher_type", "voucher_no"]
    );
    assert!(RepostAccountingLedgerItems::ALLOW_RENAME);
    assert!(RepostAccountingLedgerItems::IS_TABLE);
    assert!(RepostAccountingLedgerItems::EDITABLE_GRID);
    assert!(RepostAccountingLedgerItems::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        RepostAccountingLedgerItems::fields(),
        vec![
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .in_list_view(),
        ]
    );
}

#[test]
fn repost_accounting_ledger_items_controller_is_pass_through() {
    let row = RepostAccountingLedgerItems::new("Sales Invoice", "SINV-0001");
    assert_eq!(row.voucher_type.as_deref(), Some("Sales Invoice"));
    assert_eq!(row.voucher_no.as_deref(), Some("SINV-0001"));
    assert_eq!(row.doctype(), "Repost Accounting Ledger Items");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
