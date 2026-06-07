use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportFilterValue {
    Text(String),
    Int(i64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportFilterCase {
    pub report_name: &'static str,
    pub filters: BTreeMap<&'static str, ReportFilterValue>,
}

pub fn default_filters() -> Vec<(&'static str, &'static str)> {
    vec![
        ("company", "_Test Company"),
        ("from_date", "2010-01-01"),
        ("to_date", "2030-01-01"),
        ("period_start_date", "2010-01-01"),
        ("period_end_date", "2030-01-01"),
    ]
}

pub fn optional_filters() -> BTreeMap<&'static str, ReportFilterValue> {
    BTreeMap::new()
}

pub fn report_filter_test_cases() -> Vec<ReportFilterCase> {
    vec![
        report_case(
            "General Ledger",
            [(
                "categorize_by",
                text("Categorize by Voucher (Consolidated)"),
            )],
        ),
        report_case(
            "General Ledger",
            [
                (
                    "categorize_by",
                    text("Categorize by Voucher (Consolidated)"),
                ),
                ("include_dimensions", ReportFilterValue::Int(1)),
            ],
        ),
        report_case("Accounts Payable", [("range", text("30, 60, 90, 120"))]),
        report_case("Accounts Receivable", [("range", text("30, 60, 90, 120"))]),
        report_case(
            "Consolidated Financial Statement",
            [("report", text("Balance Sheet"))],
        ),
        report_case(
            "Consolidated Financial Statement",
            [("report", text("Profit and Loss Statement"))],
        ),
        report_case(
            "Consolidated Financial Statement",
            [("report", text("Cash Flow"))],
        ),
        report_case("Gross Profit", [("group_by", text("Invoice"))]),
        report_case("Gross Profit", [("group_by", text("Item Code"))]),
        report_case("Gross Profit", [("group_by", text("Item Group"))]),
        report_case("Gross Profit", [("group_by", text("Customer"))]),
        report_case("Gross Profit", [("group_by", text("Customer Group"))]),
        report_case("Item-wise Sales Register", []),
        report_case("Item-wise Purchase Register", []),
        report_case("Sales Register", []),
        report_case("Sales Register", [("item_group", text("All Item Groups"))]),
        report_case("Purchase Register", []),
    ]
}

fn report_case<const N: usize>(
    report_name: &'static str,
    filters: [(&'static str, ReportFilterValue); N],
) -> ReportFilterCase {
    ReportFilterCase {
        report_name,
        filters: BTreeMap::from(filters),
    }
}

fn text(value: &str) -> ReportFilterValue {
    ReportFilterValue::Text(value.to_string())
}
