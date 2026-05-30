use tokio_erp::erpnext::accounts::doctype::dunning_letter_text::dunning_letter_text::DunningLetterText;
use tokio_erp::erpnext::accounts::doctype::dunning_type::dunning_type::DunningType;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn dunning_type_matches_erpnext_metadata() {
    assert_eq!(DunningType::DOCTYPE, "Dunning Type");
    assert_eq!(DunningType::MODULE, "Accounts");
    assert_eq!(
        DunningType::FIELD_ORDER,
        [
            "dunning_type",
            "is_default",
            "column_break_3",
            "company",
            "section_break_6",
            "dunning_fee",
            "column_break_8",
            "rate_of_interest",
            "text_block_section",
            "dunning_letter_text",
            "section_break_9",
            "income_account",
            "column_break_13",
            "cost_center",
        ]
    );
    assert!(DunningType::ALLOW_RENAME);
    assert!(DunningType::BETA);
    assert!(DunningType::EDITABLE_GRID);
    assert_eq!(DunningType::NAMING_RULE, "By script");
    assert_eq!(DunningType::SORT_FIELD, "creation");
    assert_eq!(DunningType::SORT_ORDER, "DESC");
    assert!(DunningType::TRACK_CHANGES);

    assert_eq!(
        DunningType::fields(),
        vec![
            FieldSpec::data("dunning_type", "Dunning Type")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::check("is_default", "Is Default").default("0"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::currency("dunning_fee", "Dunning Fee").in_list_view(),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::float("rate_of_interest", "Rate of Interest (%) Yearly").in_list_view(),
            FieldSpec::section_break("text_block_section")
                .label("Dunning Letter")
                .description("This section allows the user to set the Body and Closing text of the Dunning Letter for the Dunning Type based on language, which can be used in Print."),
            FieldSpec::table_unlabeled("dunning_letter_text").options("Dunning Letter Text"),
            FieldSpec::section_break("section_break_9").label("Accounting Details"),
            FieldSpec::link("income_account", "Income Account").options("Account"),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
        ]
    );
}

#[test]
fn dunning_type_autoname_matches_erpnext_script() {
    let mut doc = DunningType {
        dunning_type: Some("Reminder".to_string()),
        company: Some("Acme LLC".to_string()),
        dunning_letter_text: vec![DunningLetterText::default()],
        ..Default::default()
    };

    assert_eq!(doc.custom_hooks(), ["autoname"]);
    assert_eq!(doc.autoname_with_company_abbr("AC"), "Reminder - AC");
    assert_eq!(doc.name.as_deref(), Some("Reminder - AC"));
    assert_eq!(doc.doctype(), "Dunning Type");
    assert_eq!(doc.module(), "Accounts");
}
