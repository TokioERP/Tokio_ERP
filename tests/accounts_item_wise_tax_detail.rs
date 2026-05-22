use tokio_erp::erpnext::accounts::doctype::item_wise_tax_detail::item_wise_tax_detail::ItemWiseTaxDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_wise_tax_detail_matches_erpnext_metadata() {
    assert_eq!(ItemWiseTaxDetail::DOCTYPE, "Item Wise Tax Detail");
    assert_eq!(ItemWiseTaxDetail::MODULE, "Accounts");
    assert_eq!(
        ItemWiseTaxDetail::FIELD_ORDER,
        ["item_row", "tax_row", "rate", "amount", "taxable_amount"]
    );
    assert!(ItemWiseTaxDetail::IS_TABLE);
    assert!(ItemWiseTaxDetail::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(ItemWiseTaxDetail::GRID_PAGE_LENGTH, Some(50));
    assert_eq!(ItemWiseTaxDetail::ROW_FORMAT, Some("Dynamic"));

    assert_eq!(
        ItemWiseTaxDetail::fields(),
        vec![
            FieldSpec::data("item_row", "Item Row")
                .required()
                .in_list_view(),
            FieldSpec::data("tax_row", "Tax Row")
                .required()
                .in_list_view(),
            FieldSpec::float("rate", "Tax Rate").in_list_view(),
            FieldSpec::currency("amount", "Tax Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
            FieldSpec::currency("taxable_amount", "Taxable Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
        ]
    );
}

#[test]
fn item_wise_tax_detail_preserves_pass_controller_behavior() {
    let blank = ItemWiseTaxDetail::default();
    assert_eq!(blank.item_row, None);
    assert_eq!(blank.tax_row, None);
    assert_eq!(blank.rate, None);
    assert_eq!(blank.amount, None);
    assert_eq!(blank.taxable_amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = ItemWiseTaxDetail::new("1", "2");
    assert_eq!(row.item_row.as_deref(), Some("1"));
    assert_eq!(row.tax_row.as_deref(), Some("2"));
    assert_eq!(row.doctype(), "Item Wise Tax Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
