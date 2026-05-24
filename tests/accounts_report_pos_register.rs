use tokio_erp::erpnext::accounts::report::pos_register::pos_register::{
    add_subtotal_row, execute, get_columns, get_conditions, get_group_by_field,
    get_pos_entries_query_plan, PosRegisterError, PosRegisterFilters, PosRegisterRow,
    PosRegisterSubtotalRow, ReportColumn,
};

fn filters() -> PosRegisterFilters {
    PosRegisterFilters {
        company: Some("_Test Company".to_string()),
        from_date: Some("2026-05-01".to_string()),
        to_date: Some("2026-05-31".to_string()),
        pos_profile: None,
        owner: None,
        customer: None,
        is_return: None,
        mode_of_payment: None,
        group_by: None,
    }
}

#[test]
fn pos_register_columns_and_group_by_field_match_erpnext() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::date("Posting Date", "posting_date", 90),
            ReportColumn::link("POS Invoice", "pos_invoice", "POS Invoice", 120),
            ReportColumn::link("Customer", "customer", "Customer", 120),
            ReportColumn::link("POS Profile", "pos_profile", "POS Profile", 160),
            ReportColumn::link("Cashier", "owner", "User", 140),
            ReportColumn::currency(
                "Grand Total",
                "grand_total",
                "Company:company:default_currency",
                120,
            ),
            ReportColumn::currency(
                "Paid Amount",
                "paid_amount",
                "Company:company:default_currency",
                120,
            ),
            ReportColumn::data("Payment Method", "mode_of_payment", 150),
            ReportColumn::data("Is Return", "is_return", 80),
            ReportColumn::link("Company", "company", "Company", 120),
        ]
    );

    assert_eq!(get_group_by_field(Some("POS Profile")), "pos_profile");
    assert_eq!(get_group_by_field(Some("Cashier")), "owner");
    assert_eq!(get_group_by_field(Some("Customer")), "customer");
    assert_eq!(
        get_group_by_field(Some("Payment Method")),
        "mode_of_payment"
    );
    assert_eq!(get_group_by_field(None), "");
}

#[test]
fn pos_register_validation_and_conditions_match_erpnext_filters() {
    assert_eq!(
        execute(PosRegisterFilters::default(), vec![], &[]).unwrap(),
        (Vec::new(), Vec::new())
    );

    let err = execute(
        PosRegisterFilters {
            company: None,
            ..filters()
        },
        vec![],
        &[],
    )
    .unwrap_err();
    assert_eq!(err, PosRegisterError::MissingCompany);

    let err = execute(
        PosRegisterFilters {
            group_by: Some("Customer".to_string()),
            customer: Some("CUST-001".to_string()),
            ..filters()
        },
        vec![],
        &[],
    )
    .unwrap_err();
    assert_eq!(err, PosRegisterError::GroupedByFilteredField("Customer"));

    assert_eq!(
        get_conditions(&PosRegisterFilters {
            pos_profile: Some("Main POS".to_string()),
            owner: Some("cashier@example.com".to_string()),
            customer: Some("CUST-001".to_string()),
            is_return: Some(true),
            mode_of_payment: Some("Cash".to_string()),
            ..filters()
        }),
        vec![
            "company = %(company)s",
            "posting_date >= %(from_date)s",
            "posting_date <= %(to_date)s",
            "pos_profile = %(pos_profile)s",
            "owner = %(owner)s",
            "customer = %(customer)s",
            "is_return = %(is_return)s",
            "exists Sales Invoice Payment matching mode_of_payment",
        ]
    );
}

#[test]
fn pos_register_query_plan_matches_mode_of_payment_and_other_group_branches() {
    let mut by_payment = filters();
    by_payment.group_by = Some("Payment Method".to_string());
    assert_eq!(
        get_pos_entries_query_plan(&by_payment),
        (
            "p.posting_date, sip.mode_of_payment",
            "Sales Invoice Payment",
            "sip.parent = p.name AND adjusted base_amount != 0",
            "sip.base_amount - IF(sip.type='Cash', p.change_amount, 0)",
        )
    );

    let mut by_customer = filters();
    by_customer.group_by = Some("Customer".to_string());
    assert_eq!(
        get_pos_entries_query_plan(&by_customer),
        (
            "p.posting_date, p.customer",
            "",
            "",
            "p.base_paid_amount - p.change_amount",
        )
    );
}

#[test]
fn pos_register_concatenates_payments_without_grouping_and_groups_with_subtotals() {
    let entries = vec![
        PosRegisterRow::new(
            "2026-05-01",
            "POS-0001",
            "Main POS",
            "_Test Company",
            "cashier@example.com",
            "CUST-001",
            false,
            100.0,
            None,
            None,
        ),
        PosRegisterRow::new(
            "2026-05-02",
            "POS-0002",
            "Main POS",
            "_Test Company",
            "cashier@example.com",
            "CUST-002",
            false,
            60.0,
            None,
            None,
        ),
    ];

    let report = execute(
        filters(),
        entries.clone(),
        &[
            (
                "POS-0001".to_string(),
                vec!["Cash".to_string(), "Card".to_string()],
            ),
            ("POS-0002".to_string(), vec!["Card".to_string()]),
        ],
    )
    .unwrap();
    assert!(matches!(
        report.1[0],
        PosRegisterSubtotalRow::Row(ref row)
            if row.mode_of_payment.as_deref() == Some("Cash, Card")
    ));
    assert!(matches!(
        report.1[1],
        PosRegisterSubtotalRow::Row(ref row)
            if row.mode_of_payment.as_deref() == Some("Card")
    ));

    let grouped = execute(
        PosRegisterFilters {
            group_by: Some("Customer".to_string()),
            ..filters()
        },
        entries,
        &[],
    )
    .unwrap();

    assert_eq!(grouped.0[0].fieldname, "customer");
    assert!(matches!(
        grouped.1[1],
        PosRegisterSubtotalRow::Subtotal {
            ref group_by_field,
            ref group_by_value,
            grand_total,
            ..
        } if group_by_field == "customer" && group_by_value == "CUST-001" && grand_total == 100.0
    ));
    assert!(matches!(grouped.1[2], PosRegisterSubtotalRow::Blank));
}

#[test]
fn pos_register_add_subtotal_row_sums_grand_total_and_paid_amount() {
    let invoices = vec![
        PosRegisterRow::new(
            "2026-05-01",
            "POS-0001",
            "Main POS",
            "_Test Company",
            "cashier@example.com",
            "CUST-001",
            false,
            100.0,
            Some(80.0),
            Some("Cash"),
        ),
        PosRegisterRow::new(
            "2026-05-02",
            "POS-0002",
            "Main POS",
            "_Test Company",
            "cashier@example.com",
            "CUST-001",
            false,
            60.0,
            Some(50.0),
            Some("Card"),
        ),
    ];
    let mut rows = invoices
        .iter()
        .cloned()
        .map(PosRegisterSubtotalRow::Row)
        .collect::<Vec<_>>();

    add_subtotal_row(&mut rows, &invoices, "customer", "CUST-001");

    assert!(matches!(
        rows[2],
        PosRegisterSubtotalRow::Subtotal {
            grand_total: 160.0,
            paid_amount: 130.0,
            bold: true,
            ..
        }
    ));
    assert!(matches!(rows[3], PosRegisterSubtotalRow::Blank));
}
