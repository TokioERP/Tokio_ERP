use tokio_erp::erpnext::accounts::doctype::bank_transaction_mapping::bank_transaction_mapping::BankTransactionMapping;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_transaction_mapping_matches_erpnext_metadata() {
    assert_eq!(BankTransactionMapping::DOCTYPE, "Bank Transaction Mapping");
    assert_eq!(BankTransactionMapping::MODULE, "Accounts");
    assert_eq!(
        BankTransactionMapping::FIELD_ORDER,
        ["bank_transaction_field", "file_field"]
    );
    assert!(BankTransactionMapping::IS_TABLE);

    assert_eq!(
        BankTransactionMapping::fields(),
        vec![
            FieldSpec::select("bank_transaction_field", "Field in Bank Transaction")
                .required()
                .in_list_view(),
            FieldSpec::data("file_field", "Column in Bank File")
                .required()
                .in_list_view(),
        ]
    );
}

#[test]
fn bank_transaction_mapping_preserves_pass_controller_behavior() {
    let blank = BankTransactionMapping::default();
    assert_eq!(blank.bank_transaction_field, None);
    assert_eq!(blank.file_field, None);
    assert!(blank.custom_hooks().is_empty());

    let row = BankTransactionMapping::new("date", "Posting Date");
    assert_eq!(row.bank_transaction_field.as_deref(), Some("date"));
    assert_eq!(row.file_field.as_deref(), Some("Posting Date"));
    assert_eq!(row.doctype(), "Bank Transaction Mapping");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
