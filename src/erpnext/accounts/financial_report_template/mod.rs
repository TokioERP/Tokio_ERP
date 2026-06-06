pub mod financial_ratios_analysis;
#[path = "horizontal_balance_sheet_(columnar)/mod.rs"]
pub mod horizontal_balance_sheet_columnar;
#[path = "horizontal_profit_and_loss_(columnar)/mod.rs"]
pub mod horizontal_profit_and_loss_columnar;
#[path = "standard_balance_sheet_(ifrs)/mod.rs"]
pub mod standard_balance_sheet_ifrs;
#[path = "standard_cash_flow_statement_(ifrs)/mod.rs"]
pub mod standard_cash_flow_statement_ifrs;
#[path = "standard_profit_and_loss_(ifrs)/mod.rs"]
pub mod standard_profit_and_loss_ifrs;

pub const MODULE: &str = "Accounts";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialReportTemplate {
    pub template_name: &'static str,
    pub module: &'static str,
    pub report_type: &'static str,
    pub rows: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountCategoryTemplate {
    pub account_category_name: &'static str,
    pub root_type: &'static str,
    pub description: &'static str,
}

pub const FINANCIAL_REPORT_TEMPLATES: [FinancialReportTemplate; 6] = [
    FinancialReportTemplate {
        template_name: financial_ratios_analysis::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Custom Financial Statement",
        rows: 63,
    },
    FinancialReportTemplate {
        template_name: horizontal_balance_sheet_columnar::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Balance Sheet",
        rows: 64,
    },
    FinancialReportTemplate {
        template_name: horizontal_profit_and_loss_columnar::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Profit and Loss Statement",
        rows: 66,
    },
    FinancialReportTemplate {
        template_name: standard_balance_sheet_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Balance Sheet",
        rows: 53,
    },
    FinancialReportTemplate {
        template_name: standard_cash_flow_statement_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Cash Flow",
        rows: 48,
    },
    FinancialReportTemplate {
        template_name: standard_profit_and_loss_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Profit and Loss Statement",
        rows: 26,
    },
];

pub const ACCOUNT_CATEGORIES: [AccountCategoryTemplate; 29] = [
    AccountCategoryTemplate {
        account_category_name: "Cash and Cash Equivalents",
        root_type: "Asset",
        description: "Cash on hand, demand deposits, and short-term highly liquid investments readily convertible to cash with original maturities of three months or less. Examples: Cash in hand, bank current accounts, money market funds, treasury bills <=3 months.",
    },
    AccountCategoryTemplate {
        account_category_name: "Cost of Goods Sold",
        root_type: "Expense",
        description: "Direct costs attributable to cost of goods sold. Examples: Raw materials, stock in trade.",
    },
    AccountCategoryTemplate {
        account_category_name: "Current Tax Liabilities",
        root_type: "Liability",
        description: "Income tax obligations for current and prior periods. Examples: Provision for income tax, advance tax paid, tax deducted at source.",
    },
    AccountCategoryTemplate {
        account_category_name: "Finance Costs",
        root_type: "Expense",
        description: "Interest and financing-related expenses. Examples: Interest on borrowings, bank charges, lease interest, foreign exchange losses.",
    },
    AccountCategoryTemplate {
        account_category_name: "Intangible Assets",
        root_type: "Asset",
        description: "Identifiable non-monetary assets without physical substance. Examples: Software, patents, trademarks, licenses, development costs.",
    },
    AccountCategoryTemplate {
        account_category_name: "Investment Income",
        root_type: "Income",
        description: "Returns generated from financial investments and cash management. Examples: Interest income, dividend income, rental income, fair value gains.",
    },
    AccountCategoryTemplate {
        account_category_name: "Long-term Borrowings",
        root_type: "Liability",
        description: "Interest-bearing debt obligations with maturity beyond one year. Examples: Term loans, bonds, debentures, mortgages.",
    },
    AccountCategoryTemplate {
        account_category_name: "Long-term Investments",
        root_type: "Asset",
        description: "Investments held for strategic purposes or extended periods. Examples: Equity investments, bonds, associates, joint ventures, deposits.",
    },
    AccountCategoryTemplate {
        account_category_name: "Long-term Provisions",
        root_type: "Liability",
        description: "Present obligations beyond one year with uncertain timing/amount. Examples: Asset retirement obligations, environmental remediation, legal settlements.",
    },
    AccountCategoryTemplate {
        account_category_name: "Operating Expenses",
        root_type: "Expense",
        description: "Costs incurred in ordinary business operations excluding direct costs. Examples: Selling expenses, administrative costs, marketing, utilities, rent.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Current Assets",
        root_type: "Asset",
        description: "Current assets not classified elsewhere including prepaid expenses and advances. Examples: Prepaid insurance, prepaid rent, advance to suppliers, security deposits recoverable within one year.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Current Liabilities",
        root_type: "Liability",
        description: "Short-term obligations not classified elsewhere. Examples: Accrued expenses, statutory liabilities, employee payables.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Direct Costs",
        root_type: "Expense",
        description: "Direct costs excluding cost of goods sold. Examples: Direct labor, manufacturing overhead, freight inward.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Non-current Assets",
        root_type: "Asset",
        description: "Long-term assets not classified elsewhere. Examples: Security deposits, long-term prepayments, advances for capital goods.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Non-current Liabilities",
        root_type: "Liability",
        description: "Long-term obligations not classified elsewhere. Examples: Long-term deposits, deferred income, government grants.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Operating Income",
        root_type: "Income",
        description: "Incidental income related to business operations but not core revenue. Examples: Scrap sales, government grants, insurance claims, foreign exchange gains.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Payables",
        root_type: "Liability",
        description: "Non-trade payables and obligations to parties other than suppliers. Examples: Employee payables, accrued expenses, customer advances, security deposits received.",
    },
    AccountCategoryTemplate {
        account_category_name: "Other Receivables",
        root_type: "Asset",
        description: "Non-trade amounts due to the entity excluding financing arrangements. Examples: Employee advances, insurance claims, tax refunds, deposits recoverable.",
    },
    AccountCategoryTemplate {
        account_category_name: "Reserves and Surplus",
        root_type: "Equity",
        description: "Accumulated profits and other reserves created from profits or share premium. Examples: General reserves, retained earnings, statutory reserves, share premium.",
    },
    AccountCategoryTemplate {
        account_category_name: "Revenue from Operations",
        root_type: "Income",
        description: "Income from primary business activities in ordinary course. Examples: Sales of goods, service revenue, commission income, royalty income.",
    },
    AccountCategoryTemplate {
        account_category_name: "Share Capital",
        root_type: "Equity",
        description: "Nominal value of issued and paid-up equity shares. Examples: Common stock, ordinary shares, preference shares.",
    },
    AccountCategoryTemplate {
        account_category_name: "Short-term Borrowings",
        root_type: "Liability",
        description: "Interest-bearing debt obligations due within one year. Examples: Bank overdrafts, short-term loans, current portion of long-term debt.",
    },
    AccountCategoryTemplate {
        account_category_name: "Short-term Investments",
        root_type: "Asset",
        description: "Financial instruments held for short-term investment purposes, readily convertible to cash. Examples: Marketable securities, fixed deposits >3 months, mutual funds.",
    },
    AccountCategoryTemplate {
        account_category_name: "Short-term Provisions",
        root_type: "Liability",
        description: "Present obligations due within one year with uncertain timing or amount. Examples: Warranty provisions, legal claims, restructuring costs.",
    },
    AccountCategoryTemplate {
        account_category_name: "Stock Assets",
        root_type: "Asset",
        description: "Inventory and stock-related assets including raw materials, work in progress, finished goods, and stock in trade. Examples: Raw materials, finished goods, trading merchandise, consumables.",
    },
    AccountCategoryTemplate {
        account_category_name: "Tangible Assets",
        root_type: "Asset",
        description: "Physical assets used in business operations including property, plant, and equipment. Examples: Land, buildings, machinery, equipment, vehicles, furniture, capital work in progress.",
    },
    AccountCategoryTemplate {
        account_category_name: "Tax Expense",
        root_type: "Expense",
        description: "Current and deferred income tax obligations. Examples: Current tax provision, deferred tax expense, withholding taxes.",
    },
    AccountCategoryTemplate {
        account_category_name: "Trade Payables",
        root_type: "Liability",
        description: "Amounts owed to suppliers. Examples: Supplier invoices, accrued purchases, bills payable.",
    },
    AccountCategoryTemplate {
        account_category_name: "Trade Receivables",
        root_type: "Asset",
        description: "Amounts due from customers for goods sold or services provided in ordinary course of business. Examples: Accounts receivable, notes receivable from customers, unbilled revenue.",
    },
];
