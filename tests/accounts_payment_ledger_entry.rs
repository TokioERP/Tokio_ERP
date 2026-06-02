use std::collections::HashMap;

use tokio_erp::erpnext::accounts::doctype::accounting_dimension_filter::accounting_dimension_filter::DimensionFilterInfo;
use tokio_erp::erpnext::accounts::doctype::payment_ledger_entry::payment_ledger_entry::{
    AccountSnapshot, AccountingDimensionCheck, PaymentLedgerEntry, PaymentLedgerEntryError,
    PaymentLedgerEntryFlags, PaymentLedgerOnUpdatePlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_ledger_entry_matches_erpnext_metadata() {
    assert_eq!(PaymentLedgerEntry::DOCTYPE, "Payment Ledger Entry");
    assert_eq!(PaymentLedgerEntry::MODULE, "Accounts");
    assert!(PaymentLedgerEntry::ALLOW_RENAME);
    assert!(PaymentLedgerEntry::IN_CREATE);
    assert!(PaymentLedgerEntry::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        PaymentLedgerEntry::SEARCH_FIELDS,
        "voucher_no, against_voucher_no"
    );
    assert_eq!(PaymentLedgerEntry::SORT_FIELD, "creation");
    assert_eq!(PaymentLedgerEntry::SORT_ORDER, "DESC");
    assert_eq!(
        PaymentLedgerEntry::FIELD_ORDER,
        [
            "posting_date",
            "company",
            "account_type",
            "account",
            "party_type",
            "party",
            "due_date",
            "voucher_detail_no",
            "cost_center",
            "finance_book",
            "voucher_type",
            "voucher_no",
            "against_voucher_type",
            "against_voucher_no",
            "amount",
            "account_currency",
            "amount_in_account_currency",
            "delinked",
            "remarks",
        ]
    );

    assert_eq!(
        PaymentLedgerEntry::fields(),
        vec![
            FieldSpec::date("posting_date", "Posting Date").search_index(),
            FieldSpec::select("account_type", "Account Type").options("Receivable\nPayable"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .search_index(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .search_index(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .search_index(),
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .in_standard_filter()
                .search_index(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::link("against_voucher_type", "Against Voucher Type")
                .options("DocType")
                .in_standard_filter()
                .search_index(),
            FieldSpec::dynamic_link("against_voucher_no")
                .label("Against Voucher No")
                .options("against_voucher_type")
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::currency("amount", "Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
            FieldSpec::link("account_currency", "Currency").options("Currency"),
            FieldSpec::currency("amount_in_account_currency", "Amount in Account Currency")
                .options("account_currency"),
            FieldSpec::check("delinked", "DeLinked")
                .default("0")
                .in_list_view(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .search_index(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::date("due_date", "Due Date"),
            FieldSpec::link("finance_book", "Finance Book").options("Finance Book"),
            FieldSpec::text("remarks", "Remarks"),
            FieldSpec::data("voucher_detail_no", "Voucher Detail No").search_index(),
        ]
    );
}

#[test]
fn payment_ledger_entry_account_validation_matches_erpnext() {
    let entry = PaymentLedgerEntry {
        account: Some("Debtors - AC".to_string()),
        account_type: "Receivable".to_string(),
        company: Some("Acme".to_string()),
        voucher_type: Some("Sales Invoice".to_string()),
        voucher_no: Some("SINV-0001".to_string()),
        ..Default::default()
    };
    assert_eq!(entry.custom_hooks(), ["validate", "on_update"]);
    assert_eq!(entry.doctype(), "Payment Ledger Entry");
    assert_eq!(entry.module(), "Accounts");

    let account = AccountSnapshot {
        account_type: "Receivable".to_string(),
        company: "Acme".to_string(),
        is_group: false,
        docstatus: 0,
        report_type: "Balance Sheet".to_string(),
    };
    assert_eq!(entry.validate_account(&account), Ok(()));
    assert_eq!(entry.validate_account_details(&account), Ok(()));

    assert_eq!(
        entry.validate_account(&AccountSnapshot {
            company: "Other".to_string(),
            ..account.clone()
        }),
        Err(PaymentLedgerEntryError::AccountWrongCompany {
            account: "Debtors - AC".to_string(),
            company: "Acme".to_string(),
        })
    );
    assert_eq!(
        entry.validate_account(&AccountSnapshot {
            account_type: "Payable".to_string(),
            ..account.clone()
        }),
        Err(PaymentLedgerEntryError::AccountWrongType {
            account: "Debtors - AC".to_string(),
            account_type: "Receivable".to_string(),
        })
    );
    assert_eq!(
        entry.validate_account_details(&AccountSnapshot {
            is_group: true,
            ..account.clone()
        }),
        Err(PaymentLedgerEntryError::GroupAccount {
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "SINV-0001".to_string(),
            account: "Debtors - AC".to_string(),
        })
    );
    assert_eq!(
        entry.validate_account_details(&AccountSnapshot {
            docstatus: 2,
            ..account
        }),
        Err(PaymentLedgerEntryError::InactiveAccount {
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "SINV-0001".to_string(),
            account: "Debtors - AC".to_string(),
        })
    );
}

#[test]
fn payment_ledger_entry_dimension_validations_match_erpnext() {
    let entry = PaymentLedgerEntry {
        account: Some("Sales - AC".to_string()),
        company: Some("Acme".to_string()),
        cost_center: Some("Main - AC".to_string()),
        project: Some("Project X".to_string()),
        ..Default::default()
    };

    let filter_map = HashMap::from([
        (
            ("cost_center".to_string(), "Sales - AC".to_string()),
            DimensionFilterInfo {
                allowed_dimensions: vec!["Main - AC".to_string()],
                is_mandatory: true,
                allow_or_restrict: "Allow".to_string(),
            },
        ),
        (
            ("project".to_string(), "Sales - AC".to_string()),
            DimensionFilterInfo {
                allowed_dimensions: vec!["Banned".to_string()],
                is_mandatory: false,
                allow_or_restrict: "Restrict".to_string(),
            },
        ),
    ]);
    assert_eq!(entry.validate_allowed_dimensions(&filter_map), Ok(()));

    let missing = PaymentLedgerEntry {
        cost_center: None,
        ..entry.clone()
    };
    assert_eq!(
        missing.validate_allowed_dimensions(&filter_map),
        Err(PaymentLedgerEntryError::MandatoryDimension {
            dimension: "cost_center".to_string(),
            account: "Sales - AC".to_string(),
        })
    );

    let checks = vec![AccountingDimensionCheck {
        fieldname: "cost_center".to_string(),
        label: "Cost Center".to_string(),
        company: "Acme".to_string(),
        mandatory_for_pl: true,
        mandatory_for_bs: false,
    }];
    assert_eq!(
        missing.validate_dimensions_for_pl_and_bs("Profit and Loss", &checks),
        Err(PaymentLedgerEntryError::MandatoryPlBsDimension {
            label: "Cost Center".to_string(),
            account: "Sales - AC".to_string(),
            report_type: "Profit and Loss".to_string(),
        })
    );
}

#[test]
fn payment_ledger_entry_on_update_plan_matches_erpnext_conditions() {
    let entry = PaymentLedgerEntry {
        account: Some("Debtors - AC".to_string()),
        company: Some("Acme".to_string()),
        party_type: Some("Customer".to_string()),
        party: Some("CUST-001".to_string()),
        against_voucher_type: Some("Sales Invoice".to_string()),
        against_voucher_no: Some("SINV-0001".to_string()),
        delinked: false,
        ..Default::default()
    };

    assert_eq!(
        entry.on_update_plan(PaymentLedgerEntryFlags {
            adv_adj: true,
            from_repost: false,
            update_outstanding: "Yes".to_string(),
            is_reverse_depr_entry: false,
        }),
        PaymentLedgerOnUpdatePlan {
            validate_frozen_account: true,
            validate_account_details: true,
            validate_dimensions: true,
            validate_balance_type: true,
            update_outstanding: Some((
                "Sales Invoice".to_string(),
                "SINV-0001".to_string(),
                "Debtors - AC".to_string(),
                "Customer".to_string(),
                "CUST-001".to_string(),
            )),
            adv_adj: true,
        }
    );

    let repost = entry.on_update_plan(PaymentLedgerEntryFlags {
        from_repost: true,
        update_outstanding: "No".to_string(),
        ..PaymentLedgerEntryFlags::default()
    });
    assert!(!repost.validate_frozen_account);
    assert_eq!(repost.update_outstanding, None);
}
