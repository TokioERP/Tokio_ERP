use std::collections::HashSet;

use tokio_erp::erpnext::accounts::doctype::pos_opening_entry::pos_opening_entry::{
    PosOpeningEntry, PosOpeningEntryError, PosOpeningEntryValidationContext,
};
use tokio_erp::erpnext::accounts::doctype::pos_opening_entry_detail::pos_opening_entry_detail::PosOpeningEntryDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_opening_entry_matches_erpnext_metadata() {
    assert_eq!(PosOpeningEntry::DOCTYPE, "POS Opening Entry");
    assert_eq!(PosOpeningEntry::MODULE, "Accounts");
    assert_eq!(PosOpeningEntry::AUTONAME, "POS-OPE-.YYYY.-.#####");
    assert!(PosOpeningEntry::IS_SUBMITTABLE);
    assert_eq!(PosOpeningEntry::SORT_FIELD, "creation");
    assert_eq!(PosOpeningEntry::SORT_ORDER, "DESC");
    assert!(PosOpeningEntry::TRACK_CHANGES);
    assert_eq!(
        PosOpeningEntry::FIELD_ORDER,
        [
            "period_start_date",
            "period_end_date",
            "status",
            "column_break_3",
            "posting_date",
            "set_posting_date",
            "section_break_5",
            "company",
            "pos_profile",
            "pos_closing_entry",
            "column_break_7",
            "user",
            "opening_balance_details_section",
            "balance_details",
            "section_break_9",
            "amended_from",
        ]
    );

    assert_eq!(
        PosOpeningEntry::fields(),
        vec![
            FieldSpec::datetime("period_start_date", "Period Start Date")
                .in_list_view()
                .required(),
            FieldSpec::date("period_end_date", "Period End Date")
                .in_list_view()
                .read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("pos_profile", "POS Profile")
                .options("POS Profile")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("user", "Cashier")
                .options("User")
                .required(),
            FieldSpec::section_break("section_break_9").read_only(),
            FieldSpec::link("amended_from", "Amended From")
                .options("POS Opening Entry")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::check("set_posting_date", "Set Posting Date").default("0"),
            FieldSpec::select("status", "Status")
                .options("Draft\nOpen\nClosed\nCancelled")
                .default("Draft")
                .allow_on_submit()
                .hidden()
                .read_only(),
            FieldSpec::data("pos_closing_entry", "POS Closing Entry")
                .allow_on_submit()
                .read_only(),
            FieldSpec::section_break("opening_balance_details_section"),
            FieldSpec::table("balance_details", "Opening Balance Details")
                .options("POS Opening Entry Detail")
                .required(),
        ]
    );
}

#[test]
fn pos_opening_entry_validation_matches_erpnext() {
    let entry = PosOpeningEntry {
        company: "Acme".to_string(),
        pos_profile: "Main POS".to_string(),
        user: "cashier@example.com".to_string(),
        balance_details: vec![
            PosOpeningEntryDetail::new("Cash", "100"),
            PosOpeningEntryDetail::new("Card", "0"),
        ],
        ..Default::default()
    };
    assert_eq!(
        entry.custom_hooks(),
        ["validate", "on_submit", "before_cancel", "on_cancel"]
    );
    assert_eq!(entry.doctype(), "POS Opening Entry");
    assert_eq!(entry.module(), "Accounts");

    let ok = PosOpeningEntryValidationContext {
        pos_profile_exists: true,
        pos_profile_company: Some("Acme".to_string()),
        pos_profile_disabled: false,
        user_enabled: true,
        open_pos_exists: false,
        open_user_exists: false,
        modes_with_default_account: HashSet::from(["Cash".to_string(), "Card".to_string()]),
        unconsolidated_invoices_exist: false,
    };
    assert_eq!(entry.validate(&ok), Ok(()));

    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            pos_profile_exists: false,
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::PosProfileMissing {
            pos_profile: "Main POS".to_string(),
        })
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            pos_profile_disabled: true,
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::PosProfileDisabled {
            pos_profile: "Main POS".to_string(),
        })
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            pos_profile_company: Some("Other".to_string()),
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::PosProfileWrongCompany {
            pos_profile: "Main POS".to_string(),
            company: "Acme".to_string(),
        })
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            user_enabled: false,
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::UserDisabled {
            user: "cashier@example.com".to_string(),
        })
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            open_pos_exists: true,
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::OpenPosExists {
            pos_profile: "Main POS".to_string(),
        })
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            open_user_exists: true,
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::UserAlreadyAssigned)
    );
    assert_eq!(
        entry.validate(&PosOpeningEntryValidationContext {
            modes_with_default_account: HashSet::from(["Cash".to_string()]),
            ..ok.clone()
        }),
        Err(PosOpeningEntryError::MissingPaymentMethodAccount {
            modes: vec!["Card".to_string()],
        })
    );
    assert_eq!(
        entry.check_poe_is_cancellable(&PosOpeningEntryValidationContext {
            unconsolidated_invoices_exist: true,
            ..ok
        }),
        Err(PosOpeningEntryError::UnconsolidatedInvoicesExist)
    );
}

#[test]
fn pos_opening_entry_status_and_cancel_event_match_erpnext() {
    let mut entry = PosOpeningEntry {
        name: Some("POS-OPE-2026-00001".to_string()),
        docstatus: 0,
        ..Default::default()
    };
    entry.set_status();
    assert_eq!(entry.status, "Draft");

    entry.docstatus = 1;
    entry.set_status();
    assert_eq!(entry.status, "Open");

    entry.pos_closing_entry = Some("POS-CLO-0001".to_string());
    entry.set_status();
    assert_eq!(entry.status, "Closed");

    entry.docstatus = 2;
    entry.set_status();
    assert_eq!(entry.status, "Cancelled");

    let event = entry.on_cancel_event();
    assert_eq!(event.event, "poe_POS-OPE-2026-00001");
    assert_eq!(event.operation, "Cancelled");
    assert_eq!(event.docname, "POS Opening Entry/POS-OPE-2026-00001");
}
