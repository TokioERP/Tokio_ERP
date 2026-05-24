use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("fiscal_year")
        .non_standard_fieldnames(vec![("Budget", "from_fiscal_year")])
        .transactions(vec![
            DashboardSection::labeled("Budgets", vec!["Budget"]),
            DashboardSection::labeled("References", vec!["Period Closing Voucher"]),
            DashboardSection::labeled(
                "Target Details",
                vec![
                    "Sales Person",
                    "Sales Partner",
                    "Territory",
                    "Monthly Distribution",
                ],
            ),
        ])
}
