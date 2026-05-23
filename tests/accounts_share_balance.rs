use tokio_erp::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn share_balance_matches_erpnext_metadata() {
    assert_eq!(ShareBalance::DOCTYPE, "Share Balance");
    assert_eq!(ShareBalance::MODULE, "Accounts");
    assert_eq!(
        ShareBalance::FIELD_ORDER,
        [
            "share_type",
            "from_no",
            "rate",
            "column_break_4",
            "no_of_shares",
            "to_no",
            "amount",
            "section_break_8",
            "is_company",
            "current_state",
        ]
    );
    assert!(ShareBalance::IS_TABLE);
    assert!(ShareBalance::EDITABLE_GRID);
    assert!(ShareBalance::QUICK_ENTRY);
    assert!(ShareBalance::TRACK_CHANGES);
    assert_eq!(
        ShareBalance::fields(),
        vec![
            FieldSpec::link("share_type", "Share Type")
                .options("Share Type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::int("from_no", "From No").required().read_only(),
            FieldSpec::currency("rate", "Rate")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::int("no_of_shares", "No of Shares")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::int("to_no", "To No").required().read_only(),
            FieldSpec::currency("amount", "Amount")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_8"),
            FieldSpec::check("is_company", "Is Company")
                .default("0")
                .hidden()
                .read_only(),
            FieldSpec::select("current_state", "Current State")
                .options("\nIssued\nPurchased")
                .hidden()
                .read_only(),
        ]
    );
}

#[test]
fn share_balance_preserves_pass_controller_behavior() {
    let row = ShareBalance::new("Equity", 1, 100, 10);
    assert_eq!(row.share_type.as_deref(), Some("Equity"));
    assert_eq!(row.no_of_shares, 100);
    assert_eq!(row.amount, 1000);
    assert_eq!(row.doctype(), "Share Balance");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
