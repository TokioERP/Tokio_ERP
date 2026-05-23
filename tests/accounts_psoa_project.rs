use tokio_erp::erpnext::accounts::doctype::psoa_project::psoa_project::PsoaProject;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn psoa_project_matches_erpnext_metadata() {
    assert_eq!(PsoaProject::DOCTYPE, "PSOA Project");
    assert_eq!(PsoaProject::MODULE, "Accounts");
    assert_eq!(PsoaProject::FIELD_ORDER, ["project_name"]);
    assert!(PsoaProject::IS_TABLE);
    assert!(PsoaProject::EDITABLE_GRID);
    assert_eq!(
        PsoaProject::fields(),
        vec![FieldSpec::link("project_name", "Project").options("Project")]
    );
}

#[test]
fn psoa_project_preserves_pass_controller_behavior() {
    let blank = PsoaProject::default();
    assert_eq!(blank.project_name, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PsoaProject::new("PROJ-001");
    assert_eq!(row.project_name.as_deref(), Some("PROJ-001"));
    assert_eq!(row.doctype(), "PSOA Project");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
