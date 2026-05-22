use tokio_erp::erpnext::accounts::doctype::pegged_currencies::pegged_currencies::PeggedCurrencies;
use tokio_erp::erpnext::accounts::doctype::pegged_currency_details::pegged_currency_details::PeggedCurrencyDetails;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pegged_currencies_matches_erpnext_metadata() {
    assert_eq!(PeggedCurrencies::DOCTYPE, "Pegged Currencies");
    assert_eq!(PeggedCurrencies::MODULE, "Accounts");
    assert_eq!(
        PeggedCurrencies::FIELD_ORDER,
        ["pegged_currencies_item_section", "pegged_currency_item"]
    );
    assert!(PeggedCurrencies::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PeggedCurrencies::fields(),
        vec![
            FieldSpec::section_break("pegged_currencies_item_section"),
            FieldSpec::table_unlabeled("pegged_currency_item").options("Pegged Currency Details"),
        ]
    );
}

#[test]
fn pegged_currencies_preserves_pass_controller_behavior() {
    let blank = PeggedCurrencies::default();
    assert!(blank.pegged_currency_item.is_empty());
    assert!(blank.custom_hooks().is_empty());

    let row = PeggedCurrencyDetails::new("USD", "AED", "3.6725");
    let doc = PeggedCurrencies::new(vec![row.clone()]);
    assert_eq!(doc.pegged_currency_item, vec![row]);
    assert_eq!(doc.doctype(), "Pegged Currencies");
    assert_eq!(doc.module(), "Accounts");
    assert!(doc.custom_hooks().is_empty());
}
