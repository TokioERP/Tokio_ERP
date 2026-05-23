use tokio_erp::erpnext::accounts::doctype::share_type::share_type::{
    share_type_dashboard, share_type_js_hooks, ShareType,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn share_type_matches_erpnext_metadata() {
    assert_eq!(ShareType::DOCTYPE, "Share Type");
    assert_eq!(ShareType::MODULE, "Accounts");
    assert_eq!(ShareType::AUTONAME, "field:title");
    assert_eq!(ShareType::FIELD_ORDER, ["title", "description"]);
    assert!(ShareType::EDITABLE_GRID);
    assert!(ShareType::QUICK_ENTRY);
    assert!(ShareType::TRACK_CHANGES);
    assert_eq!(
        ShareType::fields(),
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::long_text("description", "Description"),
        ]
    );
}

#[test]
fn share_type_dashboard_js_and_controller_are_pass_through() {
    let dashboard = share_type_dashboard();
    assert_eq!(dashboard.fieldname, "share_type");
    assert_eq!(
        dashboard.transactions,
        vec![("References", vec!["Share Transfer", "Shareholder"])]
    );
    assert_eq!(share_type_js_hooks(), ["refresh"]);

    let doc = ShareType::new("Equity");
    assert_eq!(doc.title, "Equity");
    assert_eq!(doc.doctype(), "Share Type");
    assert_eq!(doc.module(), "Accounts");
    assert!(doc.custom_hooks().is_empty());
}
