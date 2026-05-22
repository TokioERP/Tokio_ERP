#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardData {
    pub fieldname: &'static str,
    pub transactions: Vec<DashboardTransaction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardTransaction {
    pub label: Option<&'static str>,
    pub items: Vec<&'static str>,
}

pub fn get_data() -> DashboardData {
    DashboardData {
        fieldname: "payment_term",
        transactions: vec![
            DashboardTransaction {
                label: Some("Sales"),
                items: vec!["Sales Invoice", "Sales Order", "Quotation"],
            },
            DashboardTransaction {
                label: Some("Purchase"),
                items: vec!["Purchase Invoice", "Purchase Order"],
            },
            DashboardTransaction {
                label: None,
                items: vec!["Payment Terms Template"],
            },
        ],
    }
}
