#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardData {
    pub fieldname: &'static str,
    pub transactions: Vec<DashboardSection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardSection {
    pub label: Option<&'static str>,
    pub items: Vec<&'static str>,
}

impl DashboardData {
    pub fn new(fieldname: &'static str) -> Self {
        Self {
            fieldname,
            transactions: Vec::new(),
        }
    }

    pub fn transactions(mut self, transactions: Vec<DashboardSection>) -> Self {
        self.transactions = transactions;
        self
    }
}

impl DashboardSection {
    pub fn labeled(label: &'static str, items: Vec<&'static str>) -> Self {
        Self {
            label: Some(label),
            items,
        }
    }
}
