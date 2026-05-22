use tokio_erp::erpnext::accounts::doctype::pos_field::pos_field::PosField;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_field_matches_erpnext_metadata() {
    assert_eq!(PosField::DOCTYPE, "POS Field");
    assert_eq!(PosField::MODULE, "Accounts");
    assert_eq!(
        PosField::FIELD_ORDER,
        [
            "fieldname",
            "label",
            "fieldtype",
            "column_break_7",
            "options",
            "default_value",
            "reqd",
            "read_only",
        ]
    );
    assert!(PosField::IS_TABLE);
    assert!(PosField::EDITABLE_GRID);

    assert_eq!(
        PosField::fields(),
        vec![
            FieldSpec::select("fieldname", "Fieldname").in_list_view(),
            FieldSpec::data("label", "Label").in_list_view().read_only(),
            FieldSpec::data("fieldtype", "Fieldtype")
                .in_list_view()
                .read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::text("options", "Options")
                .in_list_view()
                .read_only(),
            FieldSpec::data("default_value", "Default Value"),
            FieldSpec::check("reqd", "Mandatory").default("0"),
            FieldSpec::check("read_only", "Read Only").default("0"),
        ]
    );
}

#[test]
fn pos_field_preserves_pass_controller_behavior() {
    let blank = PosField::default();
    assert_eq!(blank.fieldname, None);
    assert_eq!(blank.label, None);
    assert_eq!(blank.fieldtype, None);
    assert_eq!(blank.options, None);
    assert_eq!(blank.default_value, None);
    assert!(!blank.reqd);
    assert!(!blank.read_only);
    assert!(blank.custom_hooks().is_empty());

    let row = PosField::new("customer", "Customer", "Link");
    assert_eq!(row.fieldname.as_deref(), Some("customer"));
    assert_eq!(row.label.as_deref(), Some("Customer"));
    assert_eq!(row.fieldtype.as_deref(), Some("Link"));
    assert_eq!(row.doctype(), "POS Field");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
