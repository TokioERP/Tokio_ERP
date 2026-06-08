use tokio_erp::erpnext::stock::doctype::customs_tariff_number::customs_tariff_number::CustomsTariffNumber;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn customs_tariff_number_matches_erpnext_metadata_and_fields() {
    assert_eq!(CustomsTariffNumber::DOCTYPE, "Customs Tariff Number");
    assert_eq!(CustomsTariffNumber::MODULE, "Stock");
    assert_eq!(CustomsTariffNumber::AUTONAME, "field:tariff_number");
    assert_eq!(
        CustomsTariffNumber::FIELD_ORDER,
        ["tariff_number", "description"]
    );
    assert!(CustomsTariffNumber::ALLOW_RENAME);
    assert!(CustomsTariffNumber::QUICK_ENTRY);
    assert_eq!(CustomsTariffNumber::SORT_FIELD, "creation");
    assert_eq!(CustomsTariffNumber::SORT_ORDER, "DESC");
    assert!(CustomsTariffNumber::TRACK_CHANGES);

    assert_eq!(
        CustomsTariffNumber::fields(),
        vec![
            FieldSpec::data("tariff_number", "Tariff Number")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::data("description", "Description").in_list_view(),
        ]
    );
}

#[test]
fn customs_tariff_number_preserves_pass_controller_behavior() {
    let blank = CustomsTariffNumber::default();
    assert_eq!(blank.tariff_number, None);
    assert_eq!(blank.description, None);
    assert!(blank.custom_hooks().is_empty());

    let tariff = CustomsTariffNumber::new("8708.99", Some("Vehicle parts"));
    assert_eq!(tariff.tariff_number.as_deref(), Some("8708.99"));
    assert_eq!(tariff.description.as_deref(), Some("Vehicle parts"));
    assert_eq!(tariff.doctype(), "Customs Tariff Number");
    assert_eq!(tariff.module(), "Stock");
    assert!(tariff.custom_hooks().is_empty());
}
