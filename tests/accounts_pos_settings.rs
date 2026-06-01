use tokio_erp::erpnext::accounts::doctype::pos_field::pos_field::PosField;
use tokio_erp::erpnext::accounts::doctype::pos_search_fields::pos_search_fields::PosSearchFields;
use tokio_erp::erpnext::accounts::doctype::pos_settings::pos_settings::{
    PosSettings, PosSettingsError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_settings_matches_erpnext_metadata() {
    assert_eq!(PosSettings::DOCTYPE, "POS Settings");
    assert_eq!(PosSettings::MODULE, "Accounts");
    assert_eq!(
        PosSettings::FIELD_ORDER,
        [
            "invoice_type",
            "column_break_vwwt",
            "post_change_gl_entries",
            "section_break_gyos",
            "invoice_fields",
            "pos_search_fields",
        ]
    );
    assert!(PosSettings::IS_SINGLE);
    assert!(PosSettings::QUICK_ENTRY);
    assert_eq!(PosSettings::SORT_FIELD, "creation");
    assert_eq!(PosSettings::SORT_ORDER, "DESC");
    assert!(PosSettings::TRACK_CHANGES);

    assert_eq!(
        PosSettings::fields(),
        vec![
            FieldSpec::select("invoice_type", "Invoice Type Created via POS Screen")
                .options("Sales Invoice\nPOS Invoice")
                .default("Sales Invoice")
                .description("The system will create a Sales Invoice or a POS Invoice from the POS interface based on this setting. For high-volume transactions, it is recommended to use POS Invoice."),
            FieldSpec::column_break("column_break_vwwt"),
            FieldSpec::check("post_change_gl_entries", "Create Ledger Entries for Change Amount")
                .options("1")
                .default("0")
                .description("If enabled, ledger entries will be posted for change amount in POS transactions"),
            FieldSpec::section_break("section_break_gyos"),
            FieldSpec::table("invoice_fields", "POS Additional Fields").options("POS Field"),
            FieldSpec::table("pos_search_fields", "POS Search Fields").options("POS Search Fields"),
        ]
    );
}

#[test]
fn pos_settings_validate_matches_erpnext() {
    let settings = PosSettings {
        invoice_type: "POS Invoice".to_string(),
        invoice_fields: vec![
            PosField::new("customer", "Customer", "Link"),
            PosField::new("posting_date", "Posting Date", "Date"),
        ],
        pos_search_fields: vec![PosSearchFields::new("Customer")],
        ..Default::default()
    };

    assert_eq!(settings.custom_hooks(), ["validate"]);
    assert_eq!(settings.doctype(), "POS Settings");
    assert_eq!(settings.module(), "Accounts");
    assert_eq!(settings.validate(Some("Sales Invoice"), 0), Ok(()));
    assert_eq!(settings.validate(Some("POS Invoice"), 1), Ok(()));
    assert_eq!(
        settings.validate(Some("Sales Invoice"), 2),
        Err(PosSettingsError::CannotChangeInvoiceTypeWithOpenEntries)
    );

    let duplicate = PosSettings {
        invoice_fields: vec![
            PosField::new("customer", "Customer", "Link"),
            PosField::new("customer", "Customer", "Link"),
        ],
        ..settings
    };
    assert_eq!(
        duplicate.validate(None, 0),
        Err(PosSettingsError::DuplicatePosField {
            fieldname: "customer".to_string(),
        })
    );
}
