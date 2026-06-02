use tokio_erp::erpnext::accounts::doctype::advance_payment_ledger_entry::advance_payment_ledger_entry::{
    advance_payment_ledger_entry_indexes, AdvancePaymentLedgerEntry,
    AdvancePaymentOutstandingUpdate,
};
use tokio_erp::erpnext::accounts::doctype::loyalty_point_entry::loyalty_point_entry::{
    get_loyalty_point_entries_plan, get_redemption_details_plan, LoyaltyPointEntry,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn loyalty_point_entry_matches_erpnext_metadata_and_query_helpers() {
    assert_eq!(LoyaltyPointEntry::DOCTYPE, "Loyalty Point Entry");
    assert_eq!(LoyaltyPointEntry::MODULE, "Accounts");
    assert_eq!(
        LoyaltyPointEntry::FIELD_ORDER,
        [
            "loyalty_program",
            "loyalty_program_tier",
            "customer",
            "invoice_type",
            "invoice",
            "redeem_against",
            "loyalty_points",
            "purchase_amount",
            "expiry_date",
            "posting_date",
            "company",
            "discretionary_reason",
        ]
    );
    assert_eq!(LoyaltyPointEntry::SORT_FIELD, "creation");
    assert_eq!(LoyaltyPointEntry::SORT_ORDER, "DESC");
    assert!(LoyaltyPointEntry::TRACK_CHANGES);
    assert!(LoyaltyPointEntry::IN_CREATE);
    assert!(LoyaltyPointEntry::QUICK_ENTRY);
    assert_eq!(LoyaltyPointEntry::TITLE_FIELD, "customer");
    assert!(LoyaltyPointEntry::EXCLUDE_FROM_LINKED_WITH);

    assert_eq!(
        LoyaltyPointEntry::fields(),
        vec![
            FieldSpec::link("loyalty_program", "Loyalty Program")
                .options("Loyalty Program")
                .required(),
            FieldSpec::data("loyalty_program_tier", "Loyalty Program Tier"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .in_list_view()
                .required(),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required(),
            FieldSpec::dynamic_link("invoice")
                .label("Invoice")
                .options("invoice_type")
                .in_list_view(),
            FieldSpec::link("redeem_against", "Redeem Against").options("Loyalty Point Entry"),
            FieldSpec::int("loyalty_points", "Loyalty Points")
                .in_list_view()
                .required(),
            FieldSpec::currency("purchase_amount", "Purchase Amount"),
            FieldSpec::date("expiry_date", "Expiry Date")
                .in_list_view()
                .required(),
            FieldSpec::date("posting_date", "Posting Date").required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::data("discretionary_reason", "Discretionary Reason"),
        ]
    );

    let doc = LoyaltyPointEntry::default();
    assert_eq!(doc.doctype(), "Loyalty Point Entry");
    assert_eq!(doc.module(), "Accounts");
    assert!(doc.custom_hooks().is_empty());

    let entries = get_loyalty_point_entries_plan("CUST-001", "Rewards", "Acme", None, "2026-05-30");
    assert_eq!(entries.doctype, "Loyalty Point Entry");
    assert_eq!(
        entries.fields,
        vec![
            "name",
            "loyalty_points",
            "expiry_date",
            "loyalty_program_tier",
            "invoice_type",
            "invoice",
        ]
    );
    assert_eq!(
        entries.filters,
        vec![
            ("customer", "=", "CUST-001".to_string()),
            ("loyalty_program", "=", "Rewards".to_string()),
            ("expiry_date", ">=", "2026-05-30".to_string()),
            ("loyalty_points", ">", "0".to_string()),
            ("company", "=", "Acme".to_string()),
        ]
    );
    assert_eq!(entries.order_by, Some("expiry_date"));

    let redemption = get_redemption_details_plan("CUST-001", "Rewards", "Acme");
    assert_eq!(redemption.select, "redeem_against, sum(loyalty_points)");
    assert_eq!(
        redemption.filters,
        vec![
            ("customer", "=", "CUST-001".to_string()),
            ("loyalty_program", "=", "Rewards".to_string()),
            ("loyalty_points", "<", "0".to_string()),
            ("company", "=", "Acme".to_string()),
        ]
    );
    assert_eq!(redemption.group_by, Some("redeem_against"));
}

#[test]
fn advance_payment_ledger_entry_matches_erpnext_metadata_and_update_plan() {
    assert_eq!(
        AdvancePaymentLedgerEntry::DOCTYPE,
        "Advance Payment Ledger Entry"
    );
    assert_eq!(AdvancePaymentLedgerEntry::MODULE, "Accounts");
    assert_eq!(
        AdvancePaymentLedgerEntry::FIELD_ORDER,
        [
            "company",
            "voucher_type",
            "voucher_no",
            "against_voucher_type",
            "against_voucher_no",
            "currency",
            "exchange_rate",
            "amount",
            "base_amount",
            "event",
            "delinked",
        ]
    );
    assert_eq!(AdvancePaymentLedgerEntry::SORT_FIELD, "creation");
    assert_eq!(AdvancePaymentLedgerEntry::SORT_ORDER, "DESC");

    assert_eq!(
        AdvancePaymentLedgerEntry::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .read_only(),
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .read_only(),
            FieldSpec::link("against_voucher_type", "Against Voucher Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("against_voucher_no")
                .label("Against Voucher No")
                .options("against_voucher_type")
                .read_only(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .read_only(),
            FieldSpec::float("exchange_rate", "Exchange Rate")
                .depends_on("exchange_rate")
                .precision("9")
                .read_only(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .read_only(),
            FieldSpec::currency("base_amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .depends_on("base_amount")
                .read_only(),
            FieldSpec::data("event", "Event").read_only(),
            FieldSpec::check("delinked", "DeLinked")
                .default("0")
                .read_only(),
        ]
    );

    let entry = AdvancePaymentLedgerEntry {
        against_voucher_type: Some("Sales Invoice".to_string()),
        against_voucher_no: Some("SINV-0001".to_string()),
        ..Default::default()
    };
    assert_eq!(entry.custom_hooks(), ["on_update"]);
    assert_eq!(
        entry.on_update_plan(&["Sales Invoice".to_string()], Some("Yes"), false),
        Some(AdvancePaymentOutstandingUpdate {
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: Some("SINV-0001".to_string()),
        })
    );
    assert_eq!(
        entry.on_update_plan(&["Purchase Invoice".to_string()], Some("Yes"), false),
        None
    );
    assert_eq!(
        entry.on_update_plan(&["Sales Invoice".to_string()], Some("No"), false),
        None
    );
    assert_eq!(
        entry.on_update_plan(&["Sales Invoice".to_string()], Some("Yes"), true),
        None
    );

    assert_eq!(
        advance_payment_ledger_entry_indexes(),
        [
            (
                "Advance Payment Ledger Entry",
                ["against_voucher_type", "against_voucher_no"]
            ),
            (
                "Advance Payment Ledger Entry",
                ["voucher_type", "voucher_no"]
            ),
        ]
    );
}
