use tokio_erp::erpnext::accounts::doctype::cashier_closing::cashier_closing::{
    CashierClosing, CashierClosingOutstandingQuery, CashierClosingValidationError,
};
use tokio_erp::erpnext::accounts::doctype::cashier_closing_payments::cashier_closing_payments::CashierClosingPayments;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cashier_closing_matches_erpnext_metadata_and_client_setup() {
    assert_eq!(CashierClosing::DOCTYPE, "Cashier Closing");
    assert_eq!(CashierClosing::MODULE, "Accounts");
    assert_eq!(
        CashierClosing::FIELD_ORDER,
        [
            "naming_series",
            "user",
            "date",
            "from_time",
            "time",
            "expense",
            "custody",
            "returns",
            "outstanding_amount",
            "payments",
            "net_amount",
            "amended_from",
        ]
    );
    assert_eq!(CashierClosing::AUTONAME, "naming_series:");
    assert!(CashierClosing::IS_SUBMITTABLE);
    assert!(CashierClosing::TRACK_CHANGES);
    assert_eq!(CashierClosing::SORT_FIELD, "creation");
    assert_eq!(CashierClosing::SORT_ORDER, "DESC");

    assert_eq!(
        CashierClosing::fields(),
        vec![
            FieldSpec::select("naming_series", "Series")
                .options("POS-CLO-")
                .default("POS-CLO-")
                .read_only()
                .in_filter()
                .in_global_search()
                .in_standard_filter(),
            FieldSpec::link("user", "User")
                .options("User")
                .read_only()
                .required()
                .in_filter()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::date("date", "Date")
                .default("Today")
                .read_only()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::time("from_time", "From Time")
                .required()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::time("time", "To Time")
                .required()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::float("expense", "Expense")
                .default("0.00")
                .in_filter(),
            FieldSpec::float("custody", "Custody")
                .default("0.00")
                .in_filter(),
            FieldSpec::float("returns", "Returns")
                .default("0.00")
                .precision("2")
                .in_filter(),
            FieldSpec::float("outstanding_amount", "Outstanding Amount")
                .default("0.00")
                .read_only(),
            FieldSpec::table("payments", "Payments")
                .options("Cashier Closing Payments")
                .in_filter(),
            FieldSpec::float("net_amount", "Net Amount")
                .read_only()
                .in_filter()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Cashier Closing")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );

    let mut blank_user = CashierClosing::default();
    blank_user.setup_default_user("test@example.com");
    assert_eq!(blank_user.user.as_deref(), Some("test@example.com"));

    let mut explicit_user = CashierClosing {
        user: Some("cashier@example.com".to_string()),
        ..CashierClosing::default()
    };
    explicit_user.setup_default_user("test@example.com");
    assert_eq!(explicit_user.user.as_deref(), Some("cashier@example.com"));
}

#[test]
fn cashier_closing_lifecycle_query_and_calculation_match_erpnext() {
    let mut closing = CashierClosing {
        user: Some("cashier@example.com".to_string()),
        date: Some("2026-05-30".to_string()),
        from_time: Some("09:00:00".to_string()),
        time: Some("18:00:00".to_string()),
        expense: 25.75,
        custody: 100.0,
        returns: 12.5,
        payments: vec![
            CashierClosingPayments::new("Cash", "200.25"),
            CashierClosingPayments::new("Card", "300.50"),
            CashierClosingPayments::default(),
        ],
        ..CashierClosing::default()
    };

    assert_eq!(closing.validate(), Ok(()));
    assert_eq!(
        closing.outstanding_query(),
        CashierClosingOutstandingQuery {
            query: "select sum(outstanding_amount) from `tabSales Invoice` where posting_date=%s and posting_time>=%s and posting_time<=%s and owner=%s",
            params: [
                "2026-05-30".to_string(),
                "09:00:00".to_string(),
                "18:00:00".to_string(),
                "cashier@example.com".to_string(),
            ],
        }
    );

    closing.before_save(Some(87.25));
    assert_eq!(closing.outstanding_amount, 87.25);
    assert_eq!(closing.net_amount, 526.25);
    assert_eq!(closing.doctype(), "Cashier Closing");
    assert_eq!(closing.module(), "Accounts");
    assert_eq!(
        closing.custom_hooks(),
        [
            "validate",
            "before_save",
            "get_outstanding",
            "make_calculations",
            "validate_time"
        ]
    );
}

#[test]
fn cashier_closing_validate_time_matches_erpnext_guard() {
    let mut closing = CashierClosing {
        from_time: Some("18:00:00".to_string()),
        time: Some("18:00:00".to_string()),
        ..CashierClosing::default()
    };
    assert_eq!(
        closing.validate(),
        Err(CashierClosingValidationError::FromTimeShouldBeLessThanToTime)
    );

    closing.time = Some("17:59:59".to_string());
    assert_eq!(
        closing.validate_time(),
        Err(CashierClosingValidationError::FromTimeShouldBeLessThanToTime)
    );
}
