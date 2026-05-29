#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryReport {
    pub name: &'static str,
    pub module: &'static str,
    pub ref_doctype: &'static str,
    pub report_type: &'static str,
    pub roles: [&'static str; 2],
    pub add_total_row: bool,
    pub prepared_report: bool,
    pub filters: [&'static str; 0],
    pub columns: [&'static str; 0],
    pub query: &'static str,
}

pub const SALES_PARTNERS_COMMISSION_QUERY: &str = r#"SELECT
   sales_partner as "Sales Partner:Link / Sales Partner:220",
   sum(base_net_total) as "Invoiced Amount (Excl. Tax):Currency:220",
   sum(amount_eligible_for_commission) as "Amount Eligible for Commission:Currency:220",
   sum(total_commission) as "Total Commission:Currency:170",
   sum(total_commission)*100 / sum(amount_eligible_for_commission) as "Average Commission Rate:Percent:220"
FROM
   (
      SELECT
         sales_partner,
         base_net_total,
         total_commission,
         amount_eligible_for_commission
      FROM
         `tabSales Invoice`
      WHERE
         docstatus = 1
         AND IFNULL(base_net_total, 0) > 0
         AND IFNULL(total_commission, 0) > 0

      UNION ALL

      SELECT
         sales_partner,
         base_net_total,
         total_commission,
         amount_eligible_for_commission
      FROM
         `tabPOS Invoice`
      WHERE
         docstatus = 1
         AND IFNULL(base_net_total, 0) > 0
         AND IFNULL(total_commission, 0) > 0
   ) AS sub
GROUP BY
   sales_partner
ORDER BY
   "Total Commission:Currency:120""#;

pub const fn report() -> QueryReport {
    QueryReport {
        name: "Sales Partners Commission",
        module: "Accounts",
        ref_doctype: "Sales Invoice",
        report_type: "Query Report",
        roles: ["Accounts Manager", "Accounts User"],
        add_total_row: false,
        prepared_report: false,
        filters: [],
        columns: [],
        query: SALES_PARTNERS_COMMISSION_QUERY,
    }
}
