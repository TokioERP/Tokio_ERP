use tokio_erp::erpnext::accounts::doctype::bank_guarantee::bank_guarantee::{
    get_voucher_details_plan, BankGuarantee, BankGuaranteeClientConfig, BankGuaranteeError,
    VoucherDetailsPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_guarantee_matches_erpnext_metadata_and_fields() {
    assert_eq!(BankGuarantee::DOCTYPE, "Bank Guarantee");
    assert_eq!(BankGuarantee::MODULE, "Accounts");
    assert_eq!(BankGuarantee::AUTONAME, "ACC-BG-.YYYY.-.#####");
    assert_eq!(
        BankGuarantee::FIELD_ORDER,
        [
            "bg_type",
            "reference_doctype",
            "reference_docname",
            "customer",
            "supplier",
            "project",
            "column_break_6",
            "amount",
            "start_date",
            "validity",
            "end_date",
            "bank_account_info",
            "bank",
            "bank_account",
            "account",
            "bank_account_no",
            "column_break_17",
            "iban",
            "branch_code",
            "swift_number",
            "section_break_14",
            "more_information",
            "margin_details",
            "bank_guarantee_number",
            "name_of_beneficiary",
            "column_break_19",
            "margin_money",
            "charges",
            "fixed_deposit_number",
            "amended_from",
        ]
    );
    assert!(BankGuarantee::EDITABLE_GRID);
    assert_eq!(BankGuarantee::GRID_PAGE_LENGTH, 50);
    assert!(BankGuarantee::IS_SUBMITTABLE);
    assert!(BankGuarantee::QUICK_ENTRY);
    assert_eq!(BankGuarantee::ROW_FORMAT, "Dynamic");
    assert_eq!(BankGuarantee::SEARCH_FIELDS, "customer");
    assert_eq!(BankGuarantee::SORT_FIELD, "creation");
    assert_eq!(BankGuarantee::SORT_ORDER, "DESC");
    assert_eq!(BankGuarantee::TITLE_FIELD, "customer");

    let fields = BankGuarantee::fields();
    assert!(fields.contains(
        &FieldSpec::select("bg_type", "Bank Guarantee Type")
            .options("\nReceiving\nProviding")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("reference_doctype", "Reference Document Type")
            .options("DocType")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::dynamic_link("reference_docname")
            .label("Reference Document Name")
            .options("reference_doctype")
    ));
    assert!(fields.contains(
        &FieldSpec::link("customer", "Customer")
            .options("Customer")
            .depends_on("eval: doc.bg_type == \"Receiving\"")
    ));
    assert!(fields.contains(
        &FieldSpec::link("supplier", "Supplier")
            .options("Supplier")
            .depends_on("eval: doc.bg_type == \"Providing\"")
    ));
    assert!(fields.contains(
        &FieldSpec::link("project", "Project")
            .options("Project")
            .allow_on_submit()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("amount", "Amount")
            .in_list_view()
            .required()
    ));
    assert!(fields.contains(&FieldSpec::date("start_date", "Start Date").required()));
    assert!(fields.contains(&FieldSpec::int("validity", "Validity in Days")));
    assert!(fields.contains(&FieldSpec::date("end_date", "End Date").read_only()));
    assert!(
        fields.contains(&FieldSpec::section_break("bank_account_info").label("Bank Account Info"))
    );
    assert!(fields.contains(&FieldSpec::link("bank", "Bank").options("Bank")));
    assert!(
        fields.contains(&FieldSpec::link("bank_account", "Bank Account").options("Bank Account"))
    );
    assert!(fields
        .contains(&FieldSpec::data("bank_guarantee_number", "Bank Guarantee Number").unique()));
    assert!(fields.contains(&FieldSpec::text_editor(
        "more_information",
        "Clauses and Conditions"
    )));
    assert!(fields.contains(
        &FieldSpec::link("amended_from", "Amended From")
            .options("Bank Guarantee")
            .no_copy()
            .print_hide()
            .read_only()
    ));
}

#[test]
fn bank_guarantee_validate_and_submit_guards_match_erpnext() {
    let mut doc = BankGuarantee::default();
    assert_eq!(
        doc.validate(),
        Err(BankGuaranteeError::CustomerOrSupplierRequired)
    );

    doc.customer = Some("_Test Customer".to_string());
    assert_eq!(doc.validate(), Ok(()));
    assert_eq!(
        doc.on_submit(),
        Err(BankGuaranteeError::BankGuaranteeNumberRequired)
    );

    doc.bank_guarantee_number = Some("BG-0001".to_string());
    assert_eq!(
        doc.on_submit(),
        Err(BankGuaranteeError::BeneficiaryRequired)
    );

    doc.name_of_beneficiary = Some("Beneficiary".to_string());
    assert_eq!(doc.on_submit(), Err(BankGuaranteeError::BankRequired));

    doc.bank = Some("Atlas Bank".to_string());
    assert_eq!(doc.on_submit(), Ok(()));
}

#[test]
fn bank_guarantee_voucher_details_and_client_helpers_match_erpnext() {
    assert_eq!(
        get_voucher_details_plan("Receiving", Some("SO-0001")),
        Ok(VoucherDetailsPlan {
            doctype: "Sales Order",
            reference_name: "SO-0001".to_string(),
            fields_to_fetch: vec!["grand_total", "customer", "project"],
        })
    );
    assert_eq!(
        get_voucher_details_plan("Providing", Some("PO-0001")),
        Ok(VoucherDetailsPlan {
            doctype: "Purchase Order",
            reference_name: "PO-0001".to_string(),
            fields_to_fetch: vec!["grand_total", "supplier"],
        })
    );
    assert_eq!(
        get_voucher_details_plan("Receiving", None),
        Err(BankGuaranteeError::ReferenceNameMustBeString)
    );

    let client = BankGuaranteeClientConfig::from_erpnext_js();
    assert_eq!(
        client.add_fetches,
        [
            ("bank_account", "account", "account"),
            ("bank_account", "bank_account_no", "bank_account_no"),
            ("bank_account", "iban", "iban"),
            ("bank_account", "branch_code", "branch_code"),
            ("bank", "swift_number", "swift_number"),
        ]
    );
    assert_eq!(
        client.bg_type_reference_doctype("Receiving"),
        Some("Sales Order")
    );
    assert_eq!(
        client.bg_type_reference_doctype("Providing"),
        Some("Purchase Order")
    );
    assert_eq!(client.reference_party_field("Sales Order"), "customer");
    assert_eq!(client.reference_party_field("Purchase Order"), "supplier");
    assert_eq!(
        client.end_date("2026-05-10", 30),
        Ok("2026-06-08".to_string())
    );
    assert_eq!(client.bank_account_query_filters, ["company", "bank"]);
    assert!(!BankGuarantee::FIELD_ORDER.contains(&"company"));
}

#[test]
fn bank_guarantee_controller_hooks_match_erpnext_methods() {
    let doc = BankGuarantee::default();
    assert_eq!(doc.doctype(), "Bank Guarantee");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["validate", "on_submit"]);
}
