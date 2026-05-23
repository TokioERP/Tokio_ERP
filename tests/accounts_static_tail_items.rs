use tokio_erp::erpnext::accounts::doctype::tax_withholding_group::tax_withholding_group::TaxWithholdingGroup;
use tokio_erp::erpnext::accounts::doctype::territory_item::territory_item::TerritoryItem;
use tokio_erp::erpnext::accounts::doctype::transaction_deletion_record_details::transaction_deletion_record_details::TransactionDeletionRecordDetails;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn tax_withholding_group_matches_erpnext_metadata() {
    assert_eq!(TaxWithholdingGroup::DOCTYPE, "Tax Withholding Group");
    assert_eq!(TaxWithholdingGroup::MODULE, "Accounts");
    assert_eq!(TaxWithholdingGroup::AUTONAME, "field:group_name");
    assert_eq!(TaxWithholdingGroup::FIELD_ORDER, ["group_name"]);
    assert!(TaxWithholdingGroup::ALLOW_RENAME);
    assert!(TaxWithholdingGroup::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        TaxWithholdingGroup::fields(),
        vec![FieldSpec::data("group_name", "Group Name")
            .required()
            .unique()
            .in_list_view()]
    );
}

#[test]
fn territory_item_matches_erpnext_metadata() {
    assert_eq!(TerritoryItem::DOCTYPE, "Territory Item");
    assert_eq!(TerritoryItem::MODULE, "Accounts");
    assert_eq!(TerritoryItem::FIELD_ORDER, ["territory"]);
    assert!(TerritoryItem::IS_TABLE);
    assert!(TerritoryItem::EDITABLE_GRID);
    assert!(TerritoryItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(TerritoryItem::TRACK_CHANGES);
    assert_eq!(
        TerritoryItem::fields(),
        vec![FieldSpec::link("territory", "Territory")
            .options("Territory")
            .in_list_view()]
    );
}

#[test]
fn transaction_deletion_record_details_matches_erpnext_metadata() {
    assert_eq!(
        TransactionDeletionRecordDetails::DOCTYPE,
        "Transaction Deletion Record Details"
    );
    assert_eq!(TransactionDeletionRecordDetails::MODULE, "Accounts");
    assert_eq!(
        TransactionDeletionRecordDetails::FIELD_ORDER,
        ["doctype_name", "docfield_name", "no_of_docs", "done"]
    );
    assert!(TransactionDeletionRecordDetails::ALLOW_RENAME);
    assert!(TransactionDeletionRecordDetails::IS_TABLE);
    assert!(TransactionDeletionRecordDetails::EDITABLE_GRID);
    assert!(TransactionDeletionRecordDetails::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        TransactionDeletionRecordDetails::fields(),
        vec![
            FieldSpec::link("doctype_name", "DocType")
                .options("DocType")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("docfield_name", "DocField").read_only(),
            FieldSpec::int("no_of_docs", "No of Docs")
                .read_only()
                .in_list_view(),
            FieldSpec::check("done", "Done")
                .default("0")
                .read_only()
                .in_list_view(),
        ]
    );
}

#[test]
fn static_tail_items_preserve_pass_controller_behavior() {
    let group = TaxWithholdingGroup::new("Contractors");
    assert_eq!(group.group_name.as_deref(), Some("Contractors"));
    assert_eq!(group.doctype(), "Tax Withholding Group");
    assert_eq!(group.module(), "Accounts");
    assert!(group.custom_hooks().is_empty());

    let territory = TerritoryItem::new("Uzbekistan");
    assert_eq!(territory.territory.as_deref(), Some("Uzbekistan"));
    assert_eq!(territory.doctype(), "Territory Item");
    assert_eq!(territory.module(), "Accounts");
    assert!(territory.custom_hooks().is_empty());

    let record = TransactionDeletionRecordDetails::new("Sales Invoice", 12);
    assert_eq!(record.doctype_name.as_deref(), Some("Sales Invoice"));
    assert_eq!(record.no_of_docs, 12);
    assert!(!record.done);
    assert_eq!(record.doctype(), "Transaction Deletion Record Details");
    assert_eq!(record.module(), "Accounts");
    assert!(record.custom_hooks().is_empty());
}
