use tokio_erp::erpnext::stock::doctype::item_tax::item_tax::ItemTax;
use tokio_erp::erpnext::stock::doctype::item_variant_attribute::item_variant_attribute::ItemVariantAttribute;
use tokio_erp::erpnext::stock::doctype::landed_cost_purchase_receipt::landed_cost_purchase_receipt::LandedCostPurchaseReceipt;
use tokio_erp::erpnext::stock::doctype::shipment_parcel_template::shipment_parcel_template::ShipmentParcelTemplate;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_tax_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemTax::DOCTYPE, "Item Tax");
    assert_eq!(ItemTax::MODULE, "Stock");
    assert_eq!(
        ItemTax::FIELD_ORDER,
        [
            "item_tax_template",
            "tax_category",
            "valid_from",
            "minimum_net_rate",
            "maximum_net_rate",
        ]
    );
    assert!(ItemTax::EDITABLE_GRID);
    assert!(ItemTax::IS_TABLE);
    assert_eq!(ItemTax::SORT_FIELD, "creation");
    assert_eq!(ItemTax::SORT_ORDER, "DESC");

    assert_eq!(
        ItemTax::fields(),
        vec![
            FieldSpec::link("item_tax_template", "Item Tax Template")
                .options("Item Tax Template")
                .oldfield("tax_type", "Link")
                .in_list_view()
                .required(),
            FieldSpec::link("tax_category", "Tax Category")
                .options("Tax Category")
                .oldfield("tax_rate", "Currency")
                .in_list_view(),
            FieldSpec::date("valid_from", "Valid From").in_list_view(),
            FieldSpec::float("minimum_net_rate", "Minimum Net Rate").in_list_view(),
            FieldSpec::float("maximum_net_rate", "Maximum Net Rate").in_list_view(),
        ]
    );

    let tax = ItemTax::new(
        "GST 15",
        Some("Standard"),
        Some("2026-06-08"),
        Some(10.0),
        Some(100.0),
        Some("ITEM-001"),
        Some("taxes"),
        Some("Item"),
    );
    assert_eq!(tax.item_tax_template.as_deref(), Some("GST 15"));
    assert_eq!(tax.tax_category.as_deref(), Some("Standard"));
    assert_eq!(tax.valid_from.as_deref(), Some("2026-06-08"));
    assert_eq!(tax.minimum_net_rate, Some(10.0));
    assert_eq!(tax.maximum_net_rate, Some(100.0));
    assert_eq!(tax.doctype(), "Item Tax");
    assert!(tax.custom_hooks().is_empty());
}

#[test]
fn item_variant_attribute_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemVariantAttribute::DOCTYPE, "Item Variant Attribute");
    assert_eq!(ItemVariantAttribute::MODULE, "Stock");
    assert_eq!(
        ItemVariantAttribute::FIELD_ORDER,
        [
            "variant_of",
            "attribute",
            "column_break_2",
            "attribute_value",
            "numeric_values",
            "disabled",
            "section_break_4",
            "from_range",
            "increment",
            "column_break_8",
            "to_range",
        ]
    );
    assert!(ItemVariantAttribute::EDITABLE_GRID);
    assert!(ItemVariantAttribute::IS_TABLE);
    assert_eq!(ItemVariantAttribute::SORT_FIELD, "creation");
    assert_eq!(ItemVariantAttribute::SORT_ORDER, "DESC");

    assert_eq!(
        ItemVariantAttribute::fields(),
        vec![
            FieldSpec::link("variant_of", "Variant Of")
                .options("Item")
                .search_index(),
            FieldSpec::link("attribute", "Attribute")
                .options("Item Attribute")
                .in_list_view()
                .required()
                .search_index(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::data("attribute_value", "Attribute Value").in_list_view(),
            FieldSpec::check("numeric_values", "Numeric Values")
                .default("0")
                .depends_on("has_variants"),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .fetch_from("attribute.disabled"),
            FieldSpec::section_break("section_break_4").depends_on("numeric_values"),
            FieldSpec::float("from_range", "From Range"),
            FieldSpec::float("increment", "Increment"),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::float("to_range", "To Range"),
        ]
    );

    let attribute = ItemVariantAttribute::new(
        Some("T-Shirt"),
        "Size",
        Some("Large"),
        false,
        false,
        Some(1.0),
        Some(0.5),
        Some(10.0),
        Some("ITEM-001"),
        Some("attributes"),
        Some("Item"),
    );
    assert_eq!(attribute.variant_of.as_deref(), Some("T-Shirt"));
    assert_eq!(attribute.attribute.as_deref(), Some("Size"));
    assert_eq!(attribute.attribute_value.as_deref(), Some("Large"));
    assert_eq!(attribute.from_range, Some(1.0));
    assert_eq!(attribute.doctype(), "Item Variant Attribute");
    assert!(attribute.custom_hooks().is_empty());
}

#[test]
fn landed_cost_purchase_receipt_matches_erpnext_metadata_and_fields() {
    assert_eq!(
        LandedCostPurchaseReceipt::DOCTYPE,
        "Landed Cost Purchase Receipt"
    );
    assert_eq!(LandedCostPurchaseReceipt::MODULE, "Stock");
    assert_eq!(
        LandedCostPurchaseReceipt::FIELD_ORDER,
        [
            "receipt_document_type",
            "receipt_document",
            "supplier",
            "col_break1",
            "posting_date",
            "grand_total",
        ]
    );
    assert!(LandedCostPurchaseReceipt::EDITABLE_GRID);
    assert!(LandedCostPurchaseReceipt::IS_TABLE);
    assert_eq!(LandedCostPurchaseReceipt::SORT_FIELD, "creation");
    assert_eq!(LandedCostPurchaseReceipt::SORT_ORDER, "ASC");

    assert_eq!(
        LandedCostPurchaseReceipt::fields(),
        vec![
            FieldSpec::select("receipt_document_type", "Receipt Document Type")
                .options(
                    "\nPurchase Invoice\nPurchase Receipt\nStock Entry\nSubcontracting Receipt"
                )
                .in_list_view()
                .required(),
            FieldSpec::dynamic_link("receipt_document")
                .label("Receipt Document")
                .options("receipt_document_type")
                .oldfield("purchase_receipt_no", "Link")
                .in_list_view()
                .required()
                .width("220px"),
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .in_list_view()
                .read_only(),
            FieldSpec::column_break("col_break1").width("50%"),
            FieldSpec::date("posting_date", "Posting Date").read_only(),
            FieldSpec::currency("grand_total", "Grand Total")
                .options("Company:company:default_currency")
                .in_list_view()
                .read_only(),
        ]
    );

    let receipt = LandedCostPurchaseReceipt::new(
        "Purchase Receipt",
        "PREC-0001",
        Some("Acme Supplies"),
        Some("2026-06-08"),
        Some(500.0),
        Some("LCV-0001"),
        Some("purchase_receipts"),
        Some("Landed Cost Voucher"),
    );
    assert_eq!(
        receipt.receipt_document_type.as_deref(),
        Some("Purchase Receipt")
    );
    assert_eq!(receipt.receipt_document.as_deref(), Some("PREC-0001"));
    assert_eq!(receipt.grand_total, Some(500.0));
    assert_eq!(receipt.doctype(), "Landed Cost Purchase Receipt");
    assert!(receipt.custom_hooks().is_empty());
}

#[test]
fn shipment_parcel_template_matches_erpnext_metadata_and_fields() {
    assert_eq!(ShipmentParcelTemplate::DOCTYPE, "Shipment Parcel Template");
    assert_eq!(ShipmentParcelTemplate::MODULE, "Stock");
    assert_eq!(
        ShipmentParcelTemplate::FIELD_ORDER,
        [
            "parcel_template_name",
            "length",
            "width",
            "height",
            "weight"
        ]
    );
    assert!(ShipmentParcelTemplate::EDITABLE_GRID);
    assert!(ShipmentParcelTemplate::QUICK_ENTRY);
    assert_eq!(ShipmentParcelTemplate::SORT_FIELD, "creation");
    assert_eq!(ShipmentParcelTemplate::SORT_ORDER, "DESC");
    assert!(ShipmentParcelTemplate::TRACK_CHANGES);

    assert_eq!(
        ShipmentParcelTemplate::fields(),
        vec![
            FieldSpec::data("parcel_template_name", "Parcel Template Name")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::float("length", "Length (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("width", "Width (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("height", "Height (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("weight", "Weight (kg)")
                .in_list_view()
                .precision("1")
                .required(),
        ]
    );

    let template = ShipmentParcelTemplate::new("Small Box", 10.0, 20.0, 30.0, 1.5);
    assert_eq!(template.parcel_template_name.as_deref(), Some("Small Box"));
    assert_eq!(template.length, Some(10.0));
    assert_eq!(template.weight, Some(1.5));
    assert_eq!(template.doctype(), "Shipment Parcel Template");
    assert!(template.custom_hooks().is_empty());
}
