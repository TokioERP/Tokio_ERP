use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("taxes_and_charges")
        .non_standard_fieldnames(vec![("Tax Rule", "purchase_tax_template")])
        .transactions(vec![
            DashboardSection::labeled(
                "Transactions",
                vec!["Purchase Invoice", "Purchase Order", "Purchase Receipt"],
            ),
            DashboardSection::labeled("References", vec!["Supplier Quotation", "Tax Rule"]),
        ])
}
