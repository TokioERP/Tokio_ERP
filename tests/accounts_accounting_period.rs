use tokio_erp::erpnext::accounts::doctype::accounting_period::accounting_period::{
    validate_accounting_period_on_doc_save, AccountingPeriod, AccountingPeriodDocSaveContext,
    AccountingPeriodError, AccountingPeriodOverlap, ClosedAccountingPeriodMatch,
};
use tokio_erp::erpnext::accounts::doctype::closed_document::closed_document::ClosedDocument;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn accounting_period_matches_erpnext_metadata() {
    assert_eq!(AccountingPeriod::DOCTYPE, "Accounting Period");
    assert_eq!(AccountingPeriod::MODULE, "Accounts");
    assert_eq!(
        AccountingPeriod::FIELD_ORDER,
        [
            "period_name",
            "start_date",
            "end_date",
            "column_break_4",
            "company",
            "disabled",
            "exempted_role",
            "section_break_7",
            "closed_documents",
        ]
    );
    assert_eq!(AccountingPeriod::AUTONAME, "field:period_name");
    assert_eq!(AccountingPeriod::SORT_FIELD, "creation");
    assert_eq!(AccountingPeriod::SORT_ORDER, "DESC");
    assert!(AccountingPeriod::TRACK_CHANGES);

    assert_eq!(
        AccountingPeriod::fields(),
        vec![
            FieldSpec::data("period_name", "Period Name")
                .required()
                .unique(),
            FieldSpec::date("start_date", "Start Date")
                .in_list_view()
                .required(),
            FieldSpec::date("end_date", "End Date")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .required(),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .in_list_view(),
            FieldSpec::link("exempted_role", "Exempted Role")
                .options("Role")
                .description("Role allowed to bypass period restrictions."),
            FieldSpec::section_break("section_break_7"),
            FieldSpec::table("closed_documents", "Closed Documents")
                .options("Closed Document")
                .required(),
        ]
    );
}

#[test]
fn accounting_period_controller_logic_matches_erpnext() {
    let mut period = AccountingPeriod {
        name: Some("FY26 - AC".to_string()),
        period_name: "FY26".to_string(),
        start_date: "2026-01-01".to_string(),
        end_date: "2026-12-31".to_string(),
        company: Some("Acme".to_string()),
        ..Default::default()
    };

    assert_eq!(
        period.custom_hooks(),
        ["validate", "before_insert", "autoname"]
    );
    assert_eq!(period.doctype(), "Accounting Period");
    assert_eq!(period.module(), "Accounts");
    assert_eq!(period.validate_overlap(&[]), Ok(()));
    assert_eq!(
        period.validate_overlap(&[AccountingPeriodOverlap {
            name: "FY25 - AC".to_string()
        }]),
        Err(AccountingPeriodError::Overlap {
            accounting_period: "FY25 - AC".to_string()
        })
    );

    assert_eq!(period.autoname_with_company_abbr("AC"), "FY26 - AC");
    assert_eq!(period.name.as_deref(), Some("FY26 - AC"));

    let closing = AccountingPeriod::get_doctypes_for_closing(&[
        "Sales Invoice".to_string(),
        "Purchase Invoice".to_string(),
    ]);
    assert_eq!(
        closing,
        vec![
            ClosedDocument::new("Sales Invoice", true),
            ClosedDocument::new("Purchase Invoice", true),
        ]
    );

    period.bootstrap_doctypes_for_closing(&["Journal Entry".to_string()]);
    assert_eq!(
        period.closed_documents,
        vec![ClosedDocument::new("Journal Entry", true)]
    );
    period.bootstrap_doctypes_for_closing(&["Sales Invoice".to_string()]);
    assert_eq!(
        period.closed_documents,
        vec![ClosedDocument::new("Journal Entry", true)]
    );
}

#[test]
fn accounting_period_doc_save_guard_matches_erpnext() {
    let base = AccountingPeriodDocSaveContext {
        doctype: "Sales Invoice".to_string(),
        company: "Acme".to_string(),
        posting_date: Some("2026-06-01".to_string()),
        asset_type: None,
        available_for_use_date: None,
        completion_date: None,
        period_end_date: None,
        accounting_period: Some(ClosedAccountingPeriodMatch {
            name: "FY26 - AC".to_string(),
            exempted_role: None,
        }),
        user_roles: vec![],
    };

    assert_eq!(
        validate_accounting_period_on_doc_save(&base),
        Err(AccountingPeriodError::ClosedAccountingPeriod {
            doctype: "Sales Invoice".to_string(),
            accounting_period: "FY26 - AC".to_string(),
        })
    );

    let bypass = AccountingPeriodDocSaveContext {
        accounting_period: Some(ClosedAccountingPeriodMatch {
            name: "FY26 - AC".to_string(),
            exempted_role: Some("Accounts Manager".to_string()),
        }),
        user_roles: vec!["Accounts Manager".to_string()],
        ..base.clone()
    };
    assert_eq!(validate_accounting_period_on_doc_save(&bypass), Ok(()));

    let bank_clearance = AccountingPeriodDocSaveContext {
        doctype: "Bank Clearance".to_string(),
        ..base.clone()
    };
    assert_eq!(
        validate_accounting_period_on_doc_save(&bank_clearance),
        Ok(())
    );

    let existing_asset = AccountingPeriodDocSaveContext {
        doctype: "Asset".to_string(),
        asset_type: Some("Existing Asset".to_string()),
        available_for_use_date: Some("2026-06-01".to_string()),
        ..base.clone()
    };
    assert_eq!(
        validate_accounting_period_on_doc_save(&existing_asset),
        Ok(())
    );

    let asset_repair = AccountingPeriodDocSaveContext {
        doctype: "Asset Repair".to_string(),
        posting_date: None,
        completion_date: Some("2026-05-20".to_string()),
        accounting_period: None,
        ..base
    };
    assert_eq!(
        validate_accounting_period_on_doc_save(&asset_repair),
        Ok(())
    );
}
