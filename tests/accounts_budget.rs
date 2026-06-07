use tokio_erp::erpnext::accounts::doctype::budget::budget::{
    compare_expense_with_budget, get_accumulated_monthly_budget, get_actions,
    get_actual_expense_query_plan, get_expense_breakup, get_fiscal_year_date_range,
    get_item_details, get_ordered_amount_query_plan, get_other_condition,
    get_requested_amount_query_plan, revise_budget, validate_budget_records,
    validate_expense_against_budget, Budget, BudgetAccountDetails, BudgetCheckMessage,
    BudgetCheckParams, BudgetCheckResult, BudgetContext, BudgetDistributionRow, BudgetError,
    BudgetExpenseValidationContext, BudgetExpenseValidationResult, BudgetRecord,
    DuplicateBudgetError, ExistingBudget, ExpenseQueryPlan, FiscalYearDates, ItemDefaultsContext,
    PendingAmountQueryPlan, RevisionPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_budget() -> Budget {
    Budget {
        name: Some("BUDGET-0001".to_string()),
        budget_against: "Cost Center".to_string(),
        company: Some("_Test Company".to_string()),
        cost_center: Some("Main - TC".to_string()),
        account: Some("Expense - TC".to_string()),
        from_fiscal_year: Some("FY2026".to_string()),
        to_fiscal_year: Some("FY2026".to_string()),
        budget_start_date: Some("2026-01-01".to_string()),
        budget_end_date: Some("2026-12-31".to_string()),
        budget_amount: 1200.0,
        distribution_frequency: "Quarterly".to_string(),
        applicable_on_booking_actual_expenses: true,
        ..Budget::default()
    }
}

fn base_context() -> BudgetContext {
    BudgetContext {
        from_fiscal_year_companies: vec!["_Test Company".to_string()],
        to_fiscal_year_companies: vec!["_Test Company".to_string()],
        from_fiscal_year_start_date: Some("2026-01-01".to_string()),
        to_fiscal_year_end_date: Some("2026-12-31".to_string()),
        existing_budgets: Vec::new(),
        account_details: Some(BudgetAccountDetails {
            is_group: false,
            company: "_Test Company".to_string(),
            report_type: "Profit and Loss".to_string(),
        }),
        budget_against_is_tree: false,
        actual_spent: 0.0,
        is_new: false,
        old_doc: None,
    }
}

#[test]
fn budget_metadata_matches_erpnext_json() {
    assert_eq!(Budget::DOCTYPE, "Budget");
    assert_eq!(Budget::MODULE, "Accounts");
    assert_eq!(Budget::AUTONAME, "naming_series:");
    assert_eq!(Budget::FIELD_ORDER.len(), 40);
    assert_eq!(Budget::FIELD_ORDER[0], "naming_series");
    assert_eq!(Budget::FIELD_ORDER[39], "revision_of");
    assert!(Budget::ALLOW_IMPORT);
    assert!(Budget::EDITABLE_GRID);

    let doc = Budget::default();
    assert_eq!(doc.doctype(), "Budget");
    assert_eq!(doc.module(), "Accounts");
    assert!(Budget::fields().contains(
        &FieldSpec::select("budget_against", "Budget Against")
            .options("\nCost Center\nProject")
            .default("Cost Center")
            .required()
            .in_list_view()
            .in_standard_filter()
            .read_only_depends_on("eval: doc.revision_of")
    ));
    assert!(Budget::fields().contains(
        &FieldSpec::table("budget_distribution", "Budget Distribution")
            .options("Budget Distribution")
    ));
    assert!(Budget::fields().contains(
        &FieldSpec::check(
            "applicable_on_cumulative_expense",
            "Applicable on Cumulative Expense"
        )
        .default("0")
        .description("(Purchase Order + Material Request + Actual Expense)")
    ));
    assert!(Budget::fields().contains(
        &FieldSpec::select(
            "action_if_accumulated_monthly_exceeded_on_cumulative_expense",
            "Action if Accumulative Monthly Budget Exceeded on Cumulative Expense"
        )
        .options("\nStop\nWarn\nIgnore")
        .depends_on("eval:doc.applicable_on_cumulative_expense == 1")
    ));
}

#[test]
fn budget_validate_matches_mandatory_fiscal_duplicate_account_and_actual_spent_guards() {
    let mut budget = base_budget();
    let mut ctx = base_context();
    budget.validate(&ctx).unwrap();
    assert_eq!(budget.budget_start_date.as_deref(), Some("2026-01-01"));
    assert_eq!(budget.budget_end_date.as_deref(), Some("2026-12-31"));

    budget.cost_center = None;
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::Validation("Cost Center is mandatory".to_string())
    );

    budget = base_budget();
    budget.budget_amount = 0.0;
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::Validation("Budget Amount can not be 0.".to_string())
    );

    budget = base_budget();
    ctx.from_fiscal_year_companies = vec!["Other Company".to_string()];
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::Validation(
            "Fiscal Year FY2026 is not available for Company _Test Company.".to_string()
        )
    );

    ctx = base_context();
    ctx.existing_budgets = vec![ExistingBudget {
        name: "BUDGET-OLD".to_string(),
        account: "Expense - TC".to_string(),
    }];
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::Duplicate(DuplicateBudgetError(
            "Another Budget record 'BUDGET-OLD' already exists against Cost Center 'Main - TC' and account 'Expense - TC' with overlapping fiscal years."
                .to_string()
        ))
    );

    ctx = base_context();
    ctx.account_details = Some(BudgetAccountDetails {
        is_group: true,
        ..ctx.account_details.clone().unwrap()
    });
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::Validation(
            "Budget cannot be assigned against Group Account Expense - TC".to_string()
        )
    );

    ctx = base_context();
    ctx.actual_spent = 1500.0;
    assert_eq!(
        budget.validate(&ctx).unwrap_err(),
        BudgetError::BudgetLimitExceeded(
            "Spending for Account <b>Expense - TC</b> (<b>_Test Company</b>) between <b>2026-01-01</b> and <b>2026-12-31</b> has already exceeded the new allocated budget. Spent: <b>1500</b>, Budget: <b>1200</b>"
                .to_string()
        )
    );
}

#[test]
fn budget_applicable_flags_nulling_and_distribution_generation_match_erpnext() {
    let mut budget = base_budget();
    budget.project = Some("PROJ-1".to_string());
    budget.applicable_on_booking_actual_expenses = false;
    budget.applicable_on_purchase_order = true;
    assert_eq!(
        budget.validate_applicable_for().unwrap_err(),
        BudgetError::Validation("Please enable Applicable on Booking Actual Expenses".to_string())
    );

    budget.applicable_on_purchase_order = false;
    budget.validate_applicable_for().unwrap();
    assert!(budget.applicable_on_booking_actual_expenses);

    budget.set_null_value();
    assert_eq!(budget.project, None);

    budget.before_save();
    assert_eq!(budget.budget_distribution.len(), 4);
    assert_eq!(
        budget.budget_distribution[0],
        BudgetDistributionRow {
            start_date: "2026-01-01".to_string(),
            end_date: "2026-03-31".to_string(),
            amount: 300.0,
            percent: 25.0,
        }
    );
    assert_eq!(budget.budget_distribution_total, 1200.0);

    let mut monthly = base_budget();
    monthly.distribution_frequency = "Monthly".to_string();
    monthly.budget_amount = 120000.0;
    monthly.before_save();
    assert_eq!(monthly.budget_distribution.len(), 12);
    assert_eq!(
        monthly
            .budget_distribution
            .iter()
            .map(|row| row.amount)
            .sum::<f64>(),
        120000.0
    );
    assert!(monthly
        .budget_distribution
        .iter()
        .all(|row| row.amount == 10000.0));
}

#[test]
fn budget_manual_distribution_recalc_and_total_validation_match_erpnext() {
    let mut budget = base_budget();
    budget.distribute_equally = false;
    budget.budget_distribution = vec![
        BudgetDistributionRow::new("2026-01-01", "2026-06-30", 600.0, 50.0),
        BudgetDistributionRow::new("2026-07-01", "2026-12-31", 600.0, 50.0),
    ];
    budget.old_doc = Some(Box::new(Budget {
        budget_amount: 1000.0,
        distribution_frequency: "Quarterly".to_string(),
        budget_start_date: Some("2026-01-01".to_string()),
        budget_end_date: Some("2026-12-31".to_string()),
        ..budget.clone()
    }));
    assert!(budget.should_recalculate_manual_distribution());
    budget.allocate_budget();
    assert_eq!(budget.budget_distribution[0].amount, 600.0);
    assert_eq!(budget.budget_distribution[1].amount, 600.0);

    let mut stable_old = budget.clone();
    stable_old.old_doc = None;
    budget.old_doc = Some(Box::new(stable_old));

    budget.budget_distribution[1].amount = 500.0;
    assert_eq!(
        budget.validate_distribution_totals().unwrap_err(),
        BudgetError::Validation(
            "Total distributed amount 1100 must be equal to Budget Amount 1200".to_string()
        )
    );

    budget.budget_distribution[1].amount = 600.0;
    budget.budget_distribution[1].percent = 40.0;
    assert_eq!(
        budget.validate_distribution_totals().unwrap_err(),
        BudgetError::Validation(
            "Total distribution percent must equal 100 (currently 90)".to_string()
        )
    );
}

fn base_check_params() -> BudgetCheckParams {
    BudgetCheckParams {
        company: Some("_Test Company".to_string()),
        posting_date: Some("2026-05-15".to_string()),
        account: Some("Expense - TC".to_string()),
        expense_account: Some("Expense - TC".to_string()),
        cost_center: Some("Main - TC".to_string()),
        item_code: Some("ITEM-001".to_string()),
        item_group: Some("Raw Material".to_string()),
        doctype: Some("Material Request".to_string()),
        budget_against_field: Some("cost_center".to_string()),
        budget_against_doctype: Some("Cost Center".to_string()),
        from_fiscal_year: Some("FY2026".to_string()),
        to_fiscal_year: Some("FY2026".to_string()),
        budget_start_date: Some("2026-01-01".to_string()),
        budget_end_date: Some("2026-12-31".to_string()),
        actual_expense: 900.0,
        requested_amount: 250.0,
        ordered_amount: 100.0,
        for_material_request: true,
        for_purchase_order: true,
        ..BudgetCheckParams::default()
    }
}

fn base_budget_record() -> BudgetRecord {
    BudgetRecord {
        name: "BUDGET-0001".to_string(),
        budget_against: "Main - TC".to_string(),
        budget_amount: 1000.0,
        from_fiscal_year: "FY2026".to_string(),
        to_fiscal_year: "FY2026".to_string(),
        budget_start_date: "2026-01-01".to_string(),
        budget_end_date: "2026-12-31".to_string(),
        for_material_request: true,
        for_purchase_order: true,
        for_actual_expenses: true,
        action_if_annual_budget_exceeded: Some("Stop".to_string()),
        action_if_accumulated_monthly_budget_exceeded: Some("Warn".to_string()),
        action_if_annual_budget_exceeded_on_mr: Some("Warn".to_string()),
        action_if_accumulated_monthly_budget_exceeded_on_mr: Some("Stop".to_string()),
        action_if_annual_budget_exceeded_on_po: Some("Stop".to_string()),
        action_if_accumulated_monthly_budget_exceeded_on_po: Some("Warn".to_string()),
    }
}

#[test]
fn budget_expense_actions_messages_and_breakup_match_erpnext() {
    let mut params = base_check_params();
    let budget = base_budget_record();

    assert_eq!(
        get_actions(&params, &budget),
        (Some("Warn".to_string()), Some("Stop".to_string()))
    );

    let result = compare_expense_with_budget(
        &mut params,
        budget.budget_amount,
        "Annual",
        "Stop",
        &budget.budget_against,
        0.0,
        "USD",
    );
    assert_eq!(
        result,
        BudgetCheckResult::Stop(BudgetCheckMessage {
            diff: 250.0,
            total_expense: 1250.0,
            message: "Annual Budget for Account <b>Expense - TC</b> against Cost Center <b>Main - TC</b> is <b>USD 1000</b>. It will be exceeded by <b>USD 250</b>.<hr> Total Expenses booked through - <ul><li>Actual Expenses - <b>USD 900</b></li><li>Material Requests - <b>USD 250</b></li><li>Unbilled Orders - <b>USD 100</b></li></ul>".to_string(),
        })
    );

    params.actual_expense = 1250.0;
    params.requested_amount = 0.0;
    params.ordered_amount = 0.0;
    params.exception_approver_role = Some("Budget Approver".to_string());
    params.user_roles = vec!["Budget Approver".to_string()];
    let result = compare_expense_with_budget(
        &mut params,
        budget.budget_amount,
        "Annual",
        "Stop",
        &budget.budget_against,
        0.0,
        "USD",
    );
    assert!(matches!(result, BudgetCheckResult::Warn(_)));
    assert!(matches!(
        compare_expense_with_budget(
            &mut params,
            1500.0,
            "Annual",
            "Stop",
            "Main - TC",
            0.0,
            "USD"
        ),
        BudgetCheckResult::WithinLimit {
            total_expense: 1250.0
        }
    ));

    assert_eq!(
        get_expense_breakup(&params, "USD", &budget.budget_against),
        "<hr> Total Expenses booked through - <ul><li>Actual Expenses - <b>USD 1250</b></li><li>Material Requests - <b>USD 0</b></li><li>Unbilled Orders - <b>USD 0</b></li></ul>"
    );
}

#[test]
fn budget_expense_query_item_revision_helpers_match_erpnext() {
    let mut params = base_check_params();
    let fiscal_years = FiscalYearDates {
        from_year_start_date: "2026-01-01".to_string(),
        from_year_end_date: "2026-12-31".to_string(),
        to_year_start_date: "2026-01-01".to_string(),
        to_year_end_date: "2026-12-31".to_string(),
    };

    assert_eq!(
        get_other_condition(&params, "Material Request", &fiscal_years),
        "expense_account = 'Expense - TC' and child.cost_center = 'Main - TC' and parent.schedule_date between '2026-01-01' and '2026-12-31'"
    );
    assert_eq!(
        get_other_condition(&params, "Purchase Order", &fiscal_years),
        "expense_account = 'Expense - TC' and child.cost_center = 'Main - TC' and parent.transaction_date between '2026-01-01' and '2026-12-31'"
    );

    params.month_end_date = Some("2026-05-31".to_string());
    assert_eq!(
        get_actual_expense_query_plan(&params),
        ExpenseQueryPlan {
            condition1: " and gle.posting_date <= %(month_end_date)s".to_string(),
            date_condition: "and gle.posting_date between '2026-01-01' and '2026-12-31'"
                .to_string(),
            condition2: "\n\t\t\t\tand gle.cost_center = %(cost_center)s\n\t\t\t".to_string(),
        }
    );

    params.is_tree = true;
    params.lft = Some(3);
    params.rgt = Some(8);
    assert_eq!(
        get_actual_expense_query_plan(&params).condition2,
        "\n\t\t\t\tand exists(\n\t\t\t\t\tselect name from `tabCost Center`\n\t\t\t\t\twhere lft >= %(lft)s and rgt <= %(rgt)s\n\t\t\t\t\tand name = gle.cost_center\n\t\t\t\t)\n\t\t\t"
    );

    assert_eq!(
        get_accumulated_monthly_budget(
            &[
                BudgetDistributionRow::new("2026-01-01", "2026-03-31", 300.0, 25.0),
                BudgetDistributionRow::new("2026-04-01", "2026-06-30", 300.0, 25.0),
                BudgetDistributionRow::new("2026-07-01", "2026-09-30", 300.0, 25.0),
            ],
            "2026-06-15",
        ),
        600.0
    );

    let defaults = ItemDefaultsContext {
        item_default: None,
        item_group_default: Some((Some("Group CC".to_string()), None)),
        company_default: Some((None, Some("Company Expense".to_string()))),
    };
    assert_eq!(
        get_item_details(&params, &defaults),
        (
            Some("Group CC".to_string()),
            Some("Company Expense".to_string())
        )
    );
    assert_eq!(
        get_fiscal_year_date_range(&fiscal_years),
        ("2026-01-01".to_string(), "2026-12-31".to_string())
    );

    let revision = revise_budget(&base_budget(), 1);
    assert_eq!(
        revision,
        RevisionPlan {
            cancel_old_budget: true,
            new_budget: Budget {
                name: None,
                revision_of: Some("BUDGET-0001".to_string()),
                ..base_budget()
            },
        }
    );
}

#[test]
fn budget_expense_validation_orchestration_matches_erpnext() {
    let mut params = base_check_params();
    params.account = None;
    params.expense_account = Some("Expense - TC".to_string());
    let budget = base_budget_record();
    let fiscal_years = FiscalYearDates {
        from_year_start_date: "2026-01-01".to_string(),
        from_year_end_date: "2026-12-31".to_string(),
        to_year_start_date: "2026-01-01".to_string(),
        to_year_end_date: "2026-12-31".to_string(),
    };

    assert_eq!(
        get_requested_amount_query_plan(&params, &fiscal_years),
        PendingAmountQueryPlan {
            source: "Material Request".to_string(),
            item_code: Some("ITEM-001".to_string()),
            condition: "expense_account = 'Expense - TC' and child.cost_center = 'Main - TC' and parent.schedule_date between '2026-01-01' and '2026-12-31'".to_string(),
        }
    );
    assert_eq!(
        get_ordered_amount_query_plan(&params, &fiscal_years),
        PendingAmountQueryPlan {
            source: "Purchase Order".to_string(),
            item_code: Some("ITEM-001".to_string()),
            condition: "expense_account = 'Expense - TC' and child.cost_center = 'Main - TC' and parent.transaction_date between '2026-01-01' and '2026-12-31'".to_string(),
        }
    );

    let results = validate_budget_records(
        &mut params,
        &[budget.clone()],
        0.0,
        &[("BUDGET-0001".to_string(), 700.0)],
        "USD",
    );
    assert_eq!(results.len(), 2);
    assert!(matches!(results[0], BudgetCheckResult::Warn(_)));
    assert!(matches!(results[1], BudgetCheckResult::Stop(_)));
    assert_eq!(params.month_end_date.as_deref(), Some("2026-05-31"));
    assert_eq!(params.account.as_deref(), None);

    let context = BudgetExpenseValidationContext {
        budget_count: 1,
        budget_exists_for_fiscal_year: true,
        posting_fiscal_year: Some("FY2026".to_string()),
        fiscal_years,
        exception_approver_role: Some("Budget Approver".to_string()),
        account_root_type: Some("Expense".to_string()),
        item_defaults: ItemDefaultsContext::default(),
        budget_records: vec![budget],
        monthly_budgets: vec![("BUDGET-0001".to_string(), 700.0)],
        currency: "USD".to_string(),
    };
    let mut params = base_check_params();
    params.fiscal_year = None;
    params.account = None;
    let result = validate_expense_against_budget(&mut params, &context, 0.0);
    assert!(matches!(
        result,
        BudgetExpenseValidationResult::Checked { ref results } if results.len() == 2
    ));
    assert_eq!(params.fiscal_year.as_deref(), Some("FY2026"));
    assert_eq!(params.account.as_deref(), Some("Expense - TC"));
    assert_eq!(params.budget_against_field.as_deref(), Some("cost_center"));
    assert_eq!(
        params.budget_against_doctype.as_deref(),
        Some("Cost Center")
    );

    let skipped = validate_expense_against_budget(
        &mut params,
        &BudgetExpenseValidationContext {
            budget_count: 0,
            ..context
        },
        0.0,
    );
    assert_eq!(
        skipped,
        BudgetExpenseValidationResult::Skipped("No Budget records".to_string())
    );
}
