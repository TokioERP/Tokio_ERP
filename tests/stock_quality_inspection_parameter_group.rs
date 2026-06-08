use tokio_erp::erpnext::stock::doctype::quality_inspection_parameter_group::quality_inspection_parameter_group::QualityInspectionParameterGroup;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn quality_inspection_parameter_group_matches_erpnext_metadata_and_fields() {
    assert_eq!(
        QualityInspectionParameterGroup::DOCTYPE,
        "Quality Inspection Parameter Group"
    );
    assert_eq!(QualityInspectionParameterGroup::MODULE, "Stock");
    assert_eq!(
        QualityInspectionParameterGroup::AUTONAME,
        "field:group_name"
    );
    assert_eq!(QualityInspectionParameterGroup::FIELD_ORDER, ["group_name"]);
    assert!(QualityInspectionParameterGroup::EDITABLE_GRID);
    assert!(QualityInspectionParameterGroup::QUICK_ENTRY);
    assert_eq!(QualityInspectionParameterGroup::SORT_FIELD, "creation");
    assert_eq!(QualityInspectionParameterGroup::SORT_ORDER, "DESC");
    assert!(QualityInspectionParameterGroup::TRACK_CHANGES);

    assert_eq!(
        QualityInspectionParameterGroup::fields(),
        vec![FieldSpec::data("group_name", "Parameter Group Name")
            .in_list_view()
            .required()
            .unique(),]
    );
}

#[test]
fn quality_inspection_parameter_group_preserves_pass_controller_behavior() {
    let group = QualityInspectionParameterGroup::new("Dimensional Checks");

    assert_eq!(group.group_name.as_deref(), Some("Dimensional Checks"));
    assert_eq!(group.doctype(), "Quality Inspection Parameter Group");
    assert_eq!(group.module(), "Stock");
    assert!(group.custom_hooks().is_empty());
}

#[test]
fn quality_inspection_parameter_group_test_class_is_noop() {
    let group = QualityInspectionParameterGroup::default();

    assert_eq!(group.group_name, None);
    assert!(group.custom_hooks().is_empty());
}
