use tokio_erp::erpnext::accounts::financial_report_template::{
    AccountCategoryTemplate, ACCOUNT_CATEGORIES, FINANCIAL_REPORT_TEMPLATES, MODULE,
};

#[test]
fn financial_report_template_registry_matches_erpnext_static_files() {
    assert_eq!(MODULE, "Accounts");
    assert_eq!(FINANCIAL_REPORT_TEMPLATES.len(), 6);
    assert_eq!(
        FINANCIAL_REPORT_TEMPLATES[0].template_name,
        "Financial Ratios Analysis"
    );
    assert_eq!(
        FINANCIAL_REPORT_TEMPLATES[0].report_type,
        "Custom Financial Statement"
    );
    assert_eq!(FINANCIAL_REPORT_TEMPLATES[0].rows, 63);
    assert_eq!(
        FINANCIAL_REPORT_TEMPLATES[5].template_name,
        "Standard Profit and Loss (IFRS)"
    );
    assert_eq!(
        FINANCIAL_REPORT_TEMPLATES[5].report_type,
        "Profit and Loss Statement"
    );
    assert_eq!(FINANCIAL_REPORT_TEMPLATES[5].rows, 26);

    assert_eq!(ACCOUNT_CATEGORIES.len(), 29);
    assert!(ACCOUNT_CATEGORIES.contains(&AccountCategoryTemplate {
        account_category_name: "Cash and Cash Equivalents",
        root_type: "Asset",
        description: "Cash on hand, demand deposits, and short-term highly liquid investments readily convertible to cash with original maturities of three months or less. Examples: Cash in hand, bank current accounts, money market funds, treasury bills <=3 months.",
    }));
    assert!(ACCOUNT_CATEGORIES.contains(&AccountCategoryTemplate {
        account_category_name: "Cost of Goods Sold",
        root_type: "Expense",
        description:
            "Direct costs attributable to cost of goods sold. Examples: Raw materials, stock in trade.",
    }));
}
