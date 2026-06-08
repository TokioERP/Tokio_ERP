use tokio_erp::erpnext::stock::doctype::quality_inspection_parameter::quality_inspection_parameter::QualityInspectionParameter;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn quality_inspection_parameter_matches_erpnext_metadata_and_fields() {
    assert_eq!(
        QualityInspectionParameter::DOCTYPE,
        "Quality Inspection Parameter"
    );
    assert_eq!(QualityInspectionParameter::MODULE, "Stock");
    assert_eq!(QualityInspectionParameter::AUTONAME, "field:parameter");
    assert_eq!(
        QualityInspectionParameter::FIELD_ORDER,
        ["parameter", "parameter_group", "description"]
    );
    assert!(QualityInspectionParameter::EDITABLE_GRID);
    assert!(QualityInspectionParameter::QUICK_ENTRY);
    assert_eq!(QualityInspectionParameter::SORT_FIELD, "creation");
    assert_eq!(QualityInspectionParameter::SORT_ORDER, "DESC");
    assert!(QualityInspectionParameter::TRACK_CHANGES);

    assert_eq!(
        QualityInspectionParameter::fields(),
        vec![
            FieldSpec::data("parameter", "Parameter")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::link("parameter_group", "Parameter Group")
                .options("Quality Inspection Parameter Group")
                .in_list_view(),
            FieldSpec::text_editor("description", "Description"),
        ]
    );
}

#[test]
fn quality_inspection_parameter_preserves_pass_controller_behavior() {
    let parameter = QualityInspectionParameter::new(
        "Length",
        Some("Dimensional Checks"),
        Some("Measure with calibrated scale"),
    );

    assert_eq!(parameter.parameter.as_deref(), Some("Length"));
    assert_eq!(
        parameter.parameter_group.as_deref(),
        Some("Dimensional Checks")
    );
    assert_eq!(
        parameter.description.as_deref(),
        Some("Measure with calibrated scale")
    );
    assert_eq!(parameter.doctype(), "Quality Inspection Parameter");
    assert_eq!(parameter.module(), "Stock");
    assert!(parameter.custom_hooks().is_empty());
}

#[test]
fn quality_inspection_parameter_test_class_is_noop() {
    let parameter = QualityInspectionParameter::default();

    assert_eq!(parameter.parameter, None);
    assert_eq!(parameter.parameter_group, None);
    assert_eq!(parameter.description, None);
    assert!(parameter.custom_hooks().is_empty());
}
