use tokio_erp::erpnext::accounts::doctype::psoa_cost_center::psoa_cost_center::PsoaCostCenter;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn psoa_cost_center_matches_erpnext_metadata() {
    assert_eq!(PsoaCostCenter::DOCTYPE, "PSOA Cost Center");
    assert_eq!(PsoaCostCenter::MODULE, "Accounts");
    assert_eq!(PsoaCostCenter::FIELD_ORDER, ["cost_center_name"]);
    assert!(PsoaCostCenter::IS_TABLE);
    assert!(PsoaCostCenter::EDITABLE_GRID);
    assert_eq!(
        PsoaCostCenter::fields(),
        vec![FieldSpec::link("cost_center_name", "Cost Center")
            .options("Cost Center")
            .required()
            .in_list_view()]
    );
}

#[test]
fn psoa_cost_center_preserves_pass_controller_behavior() {
    let blank = PsoaCostCenter::default();
    assert_eq!(blank.cost_center_name, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PsoaCostCenter::new("Main - TC");
    assert_eq!(row.cost_center_name.as_deref(), Some("Main - TC"));
    assert_eq!(row.doctype(), "PSOA Cost Center");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
