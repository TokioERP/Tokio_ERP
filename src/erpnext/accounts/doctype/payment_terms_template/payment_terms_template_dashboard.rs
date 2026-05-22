#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardData {
    pub fieldname: &'static str,
    pub non_standard_fieldnames: Vec<(&'static str, &'static str)>,
    pub transactions: Vec<DashboardTransaction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardTransaction {
    pub label: Option<&'static str>,
    pub items: Vec<&'static str>,
}

pub fn get_data() -> DashboardData {
    DashboardData {
        fieldname: "payment_terms_template",
        non_standard_fieldnames: vec![
            ("Customer Group", "payment_terms"),
            ("Supplier Group", "payment_terms"),
            ("Supplier", "payment_terms"),
            ("Customer", "payment_terms"),
        ],
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
                label: Some("Party"),
                items: vec!["Customer", "Supplier"],
            },
            DashboardTransaction {
                label: Some("Group"),
                items: vec!["Customer Group", "Supplier Group"],
            },
        ],
    }
}
