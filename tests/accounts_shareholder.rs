use tokio_erp::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use tokio_erp::erpnext::accounts::doctype::shareholder::shareholder::{
    shareholder_dashboard, shareholder_js_hooks, Shareholder, ShareholderAction,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn shareholder_matches_erpnext_metadata() {
    assert_eq!(Shareholder::DOCTYPE, "Shareholder");
    assert_eq!(Shareholder::MODULE, "Accounts");
    assert_eq!(Shareholder::AUTONAME, "naming_series:");
    assert_eq!(Shareholder::TITLE_FIELD, "title");
    assert_eq!(Shareholder::SEARCH_FIELDS, "folio_no");
    assert_eq!(Shareholder::FIELD_ORDER.len(), 15);
    assert_eq!(Shareholder::FIELD_ORDER[0], "title");
    assert_eq!(Shareholder::FIELD_ORDER[14], "contact_list");
    assert!(Shareholder::TRACK_CHANGES);

    let fields = Shareholder::fields();
    assert_eq!(fields.len(), 15);
    assert!(fields.contains(&FieldSpec::data("title", "Title").required()));
    assert!(fields.contains(&FieldSpec::select("naming_series", "").options("ACC-SH-.YYYY.-")));
    assert!(fields.contains(
        &FieldSpec::data("folio_no", "Folio no.")
            .read_only()
            .unique()
    ));
    assert!(fields.contains(
        &FieldSpec::table("share_balance", "Share Balance")
            .options("Share Balance")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::code("contact_list", "Contact List")
            .description("Hidden list maintaining the list of contacts linked to Shareholder")
            .hidden()
            .read_only()
    ));
}

#[test]
fn shareholder_hooks_dashboard_js_and_before_save_match_erpnext() {
    let mut doc = Shareholder::new("SH-0001", "_Test Company");
    doc.share_balance = vec![ShareBalance {
        share_type: Some("Equity".to_string()),
        from_no: 1,
        to_no: 5,
        rate: 10,
        no_of_shares: 5,
        amount: 0,
        ..Default::default()
    }];
    doc.before_save();
    assert_eq!(doc.share_balance[0].amount, 50);
    assert_eq!(doc.onload(), ShareholderAction::LoadAddressAndContact);
    assert_eq!(
        doc.on_trash(),
        ShareholderAction::DeleteContactAndAddress {
            doctype: "Shareholder".to_string(),
            name: "SH-0001".to_string(),
        }
    );
    assert_eq!(doc.custom_hooks(), ["onload", "on_trash", "before_save"]);

    let dashboard = shareholder_dashboard();
    assert_eq!(dashboard.fieldname, "shareholder");
    assert_eq!(
        dashboard.non_standard_fieldnames,
        vec![("Share Transfer", "to_shareholder")]
    );
    assert_eq!(dashboard.transactions, vec![vec!["Share Transfer"]]);
    assert_eq!(shareholder_js_hooks(), ["refresh", "validate"]);
    assert_eq!(doc.doctype(), "Shareholder");
    assert_eq!(doc.module(), "Accounts");
}
