use tokio_erp::erpnext::accounts::doctype::item_tax_template::item_tax_template::{
    ItemTaxAccount, ItemTaxTemplate, ItemTaxTemplateError,
};
use tokio_erp::erpnext::accounts::doctype::item_tax_template_detail::item_tax_template_detail::ItemTaxTemplateDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_tax_template_matches_erpnext_metadata() {
    assert_eq!(ItemTaxTemplate::DOCTYPE, "Item Tax Template");
    assert_eq!(ItemTaxTemplate::MODULE, "Accounts");
    assert_eq!(
        ItemTaxTemplate::FIELD_ORDER,
        [
            "title",
            "company",
            "column_break_3",
            "disabled",
            "section_break_5",
            "taxes",
        ]
    );
    assert!(ItemTaxTemplate::ALLOW_IMPORT);
    assert!(ItemTaxTemplate::ALLOW_RENAME);
    assert_eq!(ItemTaxTemplate::DOCUMENT_TYPE, "Setup");
    assert_eq!(ItemTaxTemplate::SORT_FIELD, "creation");
    assert_eq!(ItemTaxTemplate::SORT_ORDER, "DESC");
    assert_eq!(ItemTaxTemplate::TITLE_FIELD, "title");
    assert!(ItemTaxTemplate::TRACK_CHANGES);

    assert_eq!(
        ItemTaxTemplate::fields(),
        vec![
            FieldSpec::data("title", "Title")
                .in_filter()
                .in_list_view()
                .no_copy()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_filter()
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("taxes", "Tax Rates")
                .options("Item Tax Template Detail")
                .required(),
        ]
    );
}

#[test]
fn item_tax_template_autoname_and_zero_rate_match_erpnext() {
    let mut template = ItemTaxTemplate {
        title: Some("VAT 12".to_string()),
        company: Some("Acme".to_string()),
        taxes: vec![
            ItemTaxTemplateDetail {
                tax_type: Some("VAT - AC".to_string()),
                tax_rate: 12.0,
                not_applicable: false,
            },
            ItemTaxTemplateDetail {
                tax_type: Some("Reverse VAT - AC".to_string()),
                tax_rate: 8.0,
                not_applicable: true,
            },
        ],
        ..Default::default()
    };

    assert_eq!(template.custom_hooks(), ["validate", "autoname"]);
    assert_eq!(template.doctype(), "Item Tax Template");
    assert_eq!(template.module(), "Accounts");
    assert_eq!(
        template.autoname_with_company_abbr("AC"),
        Some("VAT 12 - AC".to_string())
    );

    template.set_zero_rate_for_not_applicable_tax();
    assert_eq!(template.taxes[0].tax_rate, 12.0);
    assert_eq!(template.taxes[1].tax_rate, 0.0);
}

#[test]
fn item_tax_template_validates_accounts_like_erpnext() {
    let template = ItemTaxTemplate {
        company: Some("Acme".to_string()),
        taxes: vec![
            ItemTaxTemplateDetail {
                tax_type: Some("VAT - AC".to_string()),
                tax_rate: 12.0,
                not_applicable: false,
            },
            ItemTaxTemplateDetail {
                tax_type: Some("Expense Tax - AC".to_string()),
                tax_rate: 2.0,
                not_applicable: false,
            },
        ],
        ..Default::default()
    };
    let accounts = vec![
        ItemTaxAccount {
            name: "VAT - AC".to_string(),
            account_type: "Tax".to_string(),
            company: "Acme".to_string(),
        },
        ItemTaxAccount {
            name: "Expense Tax - AC".to_string(),
            account_type: "Expenses Included In Valuation".to_string(),
            company: "Acme".to_string(),
        },
    ];
    assert_eq!(template.validate_tax_accounts(&accounts), Ok(()));

    let wrong_company = vec![ItemTaxAccount {
        name: "VAT - AC".to_string(),
        account_type: "Tax".to_string(),
        company: "Other".to_string(),
    }];
    assert_eq!(
        template.validate_tax_accounts(&wrong_company),
        Err(ItemTaxTemplateError::AccountBelongsToDifferentCompany {
            row: 1,
            company: "Acme".to_string(),
        })
    );

    let invalid_type = vec![ItemTaxAccount {
        name: "VAT - AC".to_string(),
        account_type: "Asset".to_string(),
        company: "Acme".to_string(),
    }];
    assert_eq!(
        template.validate_tax_accounts(&invalid_type),
        Err(ItemTaxTemplateError::InvalidAccountType { row: 1 })
    );

    let duplicate = ItemTaxTemplate {
        company: Some("Acme".to_string()),
        taxes: vec![
            ItemTaxTemplateDetail {
                tax_type: Some("VAT - AC".to_string()),
                tax_rate: 12.0,
                not_applicable: false,
            },
            ItemTaxTemplateDetail {
                tax_type: Some("VAT - AC".to_string()),
                tax_rate: 5.0,
                not_applicable: false,
            },
        ],
        ..Default::default()
    };
    assert_eq!(
        duplicate.validate_tax_accounts(&accounts),
        Err(ItemTaxTemplateError::DuplicateTaxType {
            tax_type: "VAT - AC".to_string(),
        })
    );
}
