#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardData {
    pub fieldname: &'static str,
    pub non_standard_fieldnames: Vec<(&'static str, &'static str)>,
    pub internal_links: Vec<(&'static str, Vec<&'static str>)>,
    pub transactions: Vec<DashboardSection>,
    pub reports: Vec<DashboardSection>,
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
            non_standard_fieldnames: Vec::new(),
            internal_links: Vec::new(),
            transactions: Vec::new(),
            reports: Vec::new(),
        }
    }

    pub fn non_standard_fieldnames(
        mut self,
        non_standard_fieldnames: Vec<(&'static str, &'static str)>,
    ) -> Self {
        self.non_standard_fieldnames = non_standard_fieldnames;
        self
    }

    pub fn internal_links(
        mut self,
        internal_links: Vec<(&'static str, Vec<&'static str>)>,
    ) -> Self {
        self.internal_links = internal_links;
        self
    }

    pub fn transactions(mut self, transactions: Vec<DashboardSection>) -> Self {
        self.transactions = transactions;
        self
    }

    pub fn reports(mut self, reports: Vec<DashboardSection>) -> Self {
        self.reports = reports;
        self
    }
}

impl DashboardSection {
    pub fn items(items: Vec<&'static str>) -> Self {
        Self { label: None, items }
    }

    pub fn labeled(label: &'static str, items: Vec<&'static str>) -> Self {
        Self {
            label: Some(label),
            items,
        }
    }
}
