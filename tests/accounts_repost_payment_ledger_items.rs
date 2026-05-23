use tokio_erp::erpnext::accounts::doctype::repost_payment_ledger_items::repost_payment_ledger_items::RepostPaymentLedgerItems;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn repost_payment_ledger_items_matches_erpnext_metadata() {
    assert_eq!(
        RepostPaymentLedgerItems::DOCTYPE,
        "Repost Payment Ledger Items"
    );
    assert_eq!(RepostPaymentLedgerItems::MODULE, "Accounts");
    assert_eq!(
        RepostPaymentLedgerItems::FIELD_ORDER,
        ["voucher_type", "voucher_no"]
    );
    assert!(RepostPaymentLedgerItems::IS_TABLE);
    assert_eq!(
        RepostPaymentLedgerItems::fields(),
        vec![
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type"),
        ]
    );
}

#[test]
fn repost_payment_ledger_items_controller_is_pass_through() {
    let row = RepostPaymentLedgerItems::new("Payment Entry", "PAY-0001");
    assert_eq!(row.voucher_type.as_deref(), Some("Payment Entry"));
    assert_eq!(row.voucher_no.as_deref(), Some("PAY-0001"));
    assert_eq!(row.doctype(), "Repost Payment Ledger Items");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
