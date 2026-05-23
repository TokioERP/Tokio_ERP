use tokio_erp::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use tokio_erp::erpnext::accounts::doctype::share_transfer::share_transfer::{
    make_jv_entry, share_transfer_js_hooks, JournalEntryDraft, JournalEntryLine, ShareExistence,
    ShareTransfer, ShareTransferAction,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn share_transfer_matches_erpnext_metadata() {
    assert_eq!(ShareTransfer::DOCTYPE, "Share Transfer");
    assert_eq!(ShareTransfer::MODULE, "Accounts");
    assert_eq!(ShareTransfer::AUTONAME, "ACC-SHT-.YYYY.-.#####");
    assert_eq!(ShareTransfer::FIELD_ORDER.len(), 26);
    assert_eq!(ShareTransfer::FIELD_ORDER[0], "transfer_type");
    assert_eq!(ShareTransfer::FIELD_ORDER[25], "amended_from");
    assert!(ShareTransfer::EDITABLE_GRID);
    assert!(ShareTransfer::IS_SUBMITTABLE);
    assert!(ShareTransfer::TRACK_CHANGES);

    let fields = ShareTransfer::fields();
    assert_eq!(fields.len(), 26);
    assert!(fields.contains(
        &FieldSpec::select("transfer_type", "Transfer Type")
            .options("\nIssue\nPurchase\nTransfer")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::link("from_shareholder", "From Shareholder")
            .options("Shareholder")
            .depends_on("eval:doc.transfer_type != 'Issue'")
    ));
    assert!(fields.contains(
        &FieldSpec::link("asset_account", "Asset Account")
            .options("Account")
            .depends_on("eval:(doc.transfer_type != 'Transfer') && (doc.company)")
    ));
    assert!(fields.contains(
        &FieldSpec::currency("amount", "Amount")
            .options("Company:company:default_currency")
            .read_only()
    ));
}

#[test]
fn share_transfer_basic_validation_and_share_existence_match_erpnext() {
    let mut doc = ShareTransfer::new_issue("_Test Company", "SH-0001", "Equity", 1, 100, 10);
    doc.equity_or_liability_account = Some("Creditors - TC".to_string());
    doc.asset_account = Some("Cash - TC".to_string());
    doc.validate_basic().unwrap();
    assert_eq!(doc.from_shareholder.as_deref(), Some(""));
    assert_eq!(doc.amount, 1000);

    doc.no_of_shares = 90;
    assert_eq!(
        doc.validate_basic().unwrap_err(),
        "The number of shares and the share numbers are inconsistent"
    );

    let balances = vec![ShareBalance::new("Equity", 1, 500, 10)];
    let transfer = ShareTransfer::new_transfer("_Test Company", "A", "B", "Equity", 101, 200, 15);
    assert_eq!(transfer.share_exists(&balances), ShareExistence::Complete);

    let missing = ShareTransfer::new_transfer("_Test Company", "A", "B", "Equity", 600, 700, 15);
    assert_eq!(missing.share_exists(&balances), ShareExistence::Outside);
}

#[test]
fn share_transfer_remove_shares_splits_ranges_like_erpnext() {
    let doc = ShareTransfer::new_transfer("_Test Company", "A", "B", "Equity", 101, 200, 15);
    let balances = vec![ShareBalance::new("Equity", 1, 500, 10)];
    assert_eq!(
        doc.remove_shares_from_entries(&balances),
        vec![
            ShareBalance {
                amount: 1500,
                ..ShareBalance::new("Equity", 1, 100, 10)
            },
            ShareBalance {
                amount: 4500,
                ..ShareBalance::new("Equity", 201, 500, 10)
            },
        ]
    );

    let purchase = ShareTransfer::new_transfer("_Test Company", "A", "B", "Equity", 1, 100, 15);
    assert_eq!(
        purchase.remove_shares_from_entries(&balances),
        vec![ShareBalance {
            amount: 6000,
            ..ShareBalance::new("Equity", 101, 500, 10)
        }]
    );
}

#[test]
fn share_transfer_submit_cancel_jv_and_js_match_erpnext() {
    let mut issue = ShareTransfer::new_issue("_Test Company", "SH-0001", "Equity", 1, 100, 10);
    issue.equity_or_liability_account = Some("Creditors - TC".to_string());
    issue.asset_account = Some("Cash - TC".to_string());
    assert_eq!(
        issue.on_submit(),
        vec![
            ShareTransferAction::AppendShareBalance {
                shareholder: "_Test Company".to_string(),
                balance: ShareBalance {
                    is_company: true,
                    current_state: Some("Issued".to_string()),
                    ..ShareBalance::new("Equity", 1, 100, 10)
                },
            },
            ShareTransferAction::AppendShareBalance {
                shareholder: "SH-0001".to_string(),
                balance: ShareBalance::new("Equity", 1, 100, 10),
            },
        ]
    );
    assert_eq!(
        issue.on_cancel(),
        vec![
            ShareTransferAction::RemoveShares {
                shareholder: "_Test Company".to_string()
            },
            ShareTransferAction::RemoveShares {
                shareholder: "SH-0001".to_string()
            },
        ]
    );

    let jv = make_jv_entry(
        "_Test Company",
        "Cash - TC",
        1000,
        "Creditors - TC",
        "Shareholder",
        "SH-0001",
        "",
        "",
    );
    assert_eq!(
        jv,
        JournalEntryDraft {
            voucher_type: "Journal Entry".to_string(),
            company: "_Test Company".to_string(),
            accounts: vec![
                JournalEntryLine {
                    account: "Cash - TC".to_string(),
                    debit_in_account_currency: 1000,
                    credit_in_account_currency: 0,
                    party_type: "".to_string(),
                    party: "".to_string(),
                },
                JournalEntryLine {
                    account: "Creditors - TC".to_string(),
                    debit_in_account_currency: 0,
                    credit_in_account_currency: 1000,
                    party_type: "Shareholder".to_string(),
                    party: "SH-0001".to_string(),
                },
            ],
        }
    );
    assert_eq!(
        share_transfer_js_hooks(),
        [
            "refresh",
            "no_of_shares",
            "rate",
            "company",
            "transfer_type"
        ]
    );
    assert_eq!(issue.custom_hooks(), ["validate", "on_submit", "on_cancel"]);
    assert_eq!(issue.doctype(), "Share Transfer");
    assert_eq!(issue.module(), "Accounts");
}
