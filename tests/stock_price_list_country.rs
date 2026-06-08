use tokio_erp::erpnext::stock::doctype::price_list_country::price_list_country::PriceListCountry;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn price_list_country_matches_erpnext_metadata_and_fields() {
    assert_eq!(PriceListCountry::DOCTYPE, "Price List Country");
    assert_eq!(PriceListCountry::MODULE, "Stock");
    assert_eq!(PriceListCountry::FIELD_ORDER, ["country"]);
    assert!(PriceListCountry::EDITABLE_GRID);
    assert!(PriceListCountry::IS_TABLE);
    assert_eq!(PriceListCountry::SORT_FIELD, "creation");
    assert_eq!(PriceListCountry::SORT_ORDER, "DESC");

    assert_eq!(
        PriceListCountry::fields(),
        vec![FieldSpec::link("country", "Country")
            .options("Country")
            .in_list_view()
            .required(),]
    );
}

#[test]
fn price_list_country_preserves_pass_controller_behavior() {
    let row = PriceListCountry::new(
        Some("Uzbekistan"),
        Some("Standard Selling"),
        Some("countries"),
        Some("Price List"),
    );

    assert_eq!(row.country.as_deref(), Some("Uzbekistan"));
    assert_eq!(row.parent.as_deref(), Some("Standard Selling"));
    assert_eq!(row.parentfield.as_deref(), Some("countries"));
    assert_eq!(row.parenttype.as_deref(), Some("Price List"));
    assert_eq!(row.doctype(), "Price List Country");
    assert_eq!(row.module(), "Stock");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn price_list_country_defaults_match_empty_child_row_state() {
    let row = PriceListCountry::default();

    assert_eq!(row.country, None);
    assert_eq!(row.parent, None);
    assert_eq!(row.parentfield, None);
    assert_eq!(row.parenttype, None);
    assert!(row.custom_hooks().is_empty());
}
