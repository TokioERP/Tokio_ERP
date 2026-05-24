use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidLedgerEntryFilters {
    pub company: Option<String>,
    pub from_date: String,
    pub to_date: String,
    pub account: Vec<String>,
    pub voucher_no: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LedgerVoucher {
    pub voucher_type: String,
    pub voucher_no: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveVoucherQueryPlan {
    pub source_doctype: &'static str,
    pub cancelled_field: &'static str,
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub extra_filters: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidLedgerEntriesReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<LedgerVoucher>,
}

impl InvalidLedgerEntryFilters {
    pub fn new(
        company: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
    ) -> Self {
        Self {
            company: Some(company.into()),
            from_date: from_date.into(),
            to_date: to_date.into(),
            account: Vec::new(),
            voucher_no: None,
        }
    }
}

impl ReportColumn {
    pub const fn link(label: &'static str, fieldname: &'static str, options: &'static str) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            options,
        }
    }

    pub const fn dynamic_link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Dynamic Link",
            options,
        }
    }
}

impl LedgerVoucher {
    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
        }
    }
}

impl ActiveVoucherQueryPlan {
    pub fn from_filters(filters: &InvalidLedgerEntryFilters) -> Vec<Self> {
        let company = filters.company.clone().unwrap_or_default();
        let extra_filters = build_query_filters(filters);
        vec![
            Self {
                source_doctype: "GL Entry",
                cancelled_field: "is_cancelled",
                company: company.clone(),
                from_date: filters.from_date.clone(),
                to_date: filters.to_date.clone(),
                extra_filters: extra_filters.clone(),
            },
            Self {
                source_doctype: "Payment Ledger Entry",
                cancelled_field: "delinked",
                company,
                from_date: filters.from_date.clone(),
                to_date: filters.to_date.clone(),
                extra_filters,
            },
        ]
    }
}

pub fn execute(
    filters: InvalidLedgerEntryFilters,
    active_vouchers: Vec<LedgerVoucher>,
    non_active_vouchers: Vec<LedgerVoucher>,
) -> Result<InvalidLedgerEntriesReport, String> {
    validate_filters(Some(&filters))?;
    Ok(InvalidLedgerEntriesReport {
        columns: get_columns(),
        rows: identify_cancelled_vouchers(&active_vouchers, &non_active_vouchers),
    })
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Voucher Type", "voucher_type", "DocType"),
        ReportColumn::dynamic_link("Voucher No", "voucher_no", "voucher_type"),
    ]
}

pub fn validate_filters(filters: Option<&InvalidLedgerEntryFilters>) -> Result<(), String> {
    let Some(filters) = filters else {
        return Err("Filters missing".to_string());
    };
    if filters.company.as_deref().unwrap_or_default().is_empty() {
        return Err("Company is mandatory".to_string());
    }
    if filters.from_date > filters.to_date {
        return Err("Start Date should be lower than End Date".to_string());
    }
    Ok(())
}

pub fn build_query_filters(filters: &InvalidLedgerEntryFilters) -> Vec<String> {
    let mut query_filters = Vec::new();
    if !filters.account.is_empty() {
        let accounts = filters
            .account
            .iter()
            .map(|account| format!("'{account}'"))
            .collect::<Vec<_>>()
            .join(", ");
        query_filters.push(format!("account in [{accounts}]"));
    }
    if let Some(voucher_no) = filters.voucher_no.as_deref() {
        query_filters.push(format!("voucher_no = {voucher_no}"));
    }
    query_filters
}

pub fn identify_cancelled_vouchers(
    active_vouchers: &[LedgerVoucher],
    non_active_vouchers: &[LedgerVoucher],
) -> Vec<LedgerVoucher> {
    if active_vouchers.is_empty() {
        return Vec::new();
    }

    let active = active_vouchers.iter().cloned().collect::<BTreeSet<_>>();
    let non_active = non_active_vouchers.iter().cloned().collect::<BTreeSet<_>>();
    active.intersection(&non_active).cloned().collect()
}
