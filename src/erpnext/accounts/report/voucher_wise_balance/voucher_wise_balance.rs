#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VoucherBalanceFilters {
    pub company: Option<String>,
    pub voucher_type: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub voucher_type: String,
    pub voucher_no: String,
    pub company: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoucherBalanceRow {
    pub voucher_type: String,
    pub voucher_no: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoucherBalanceReport {
    pub columns: Vec<(&'static str, &'static str, &'static str, &'static str, u16)>,
    pub rows: Vec<VoucherBalanceRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoucherBalanceQueryPlan {
    pub source_doctype: &'static str,
    pub selected_fields: Vec<&'static str>,
    pub base_filter: &'static str,
    pub group_by: &'static str,
    pub optional_filters: Vec<&'static str>,
}

impl Default for VoucherBalanceQueryPlan {
    fn default() -> Self {
        Self {
            source_doctype: "GL Entry",
            selected_fields: vec!["voucher_type", "voucher_no", "Sum(debit)", "Sum(credit)"],
            base_filter: "is_cancelled = 0",
            group_by: "voucher_no",
            optional_filters: vec![
                "company = filters.company",
                "voucher_type = filters.voucher_type",
                "posting_date >= filters.from_date",
                "posting_date <= filters.to_date",
            ],
        }
    }
}

impl GlEntry {
    pub fn new(
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        company: impl Into<String>,
        posting_date: impl Into<String>,
        debit: f64,
        credit: f64,
        is_cancelled: bool,
    ) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
            company: company.into(),
            posting_date: posting_date.into(),
            debit,
            credit,
            is_cancelled,
        }
    }
}

pub fn execute(entries: Vec<GlEntry>, filters: VoucherBalanceFilters) -> VoucherBalanceReport {
    VoucherBalanceReport {
        columns: get_columns(),
        rows: get_data(entries, &filters),
    }
}

pub fn get_columns() -> Vec<(&'static str, &'static str, &'static str, &'static str, u16)> {
    vec![
        ("Voucher Type", "voucher_type", "", "", 300),
        (
            "Voucher No",
            "voucher_no",
            "Dynamic Link",
            "voucher_type",
            300,
        ),
        ("Debit", "debit", "Currency", "currency", 300),
        ("Credit", "credit", "Currency", "currency", 300),
    ]
}

pub fn get_data(entries: Vec<GlEntry>, filters: &VoucherBalanceFilters) -> Vec<VoucherBalanceRow> {
    let filtered = apply_filters(entries, filters);
    let mut grouped: Vec<VoucherBalanceRow> = Vec::new();

    for entry in filtered {
        if let Some(row) = grouped
            .iter_mut()
            .find(|row| row.voucher_no == entry.voucher_no)
        {
            row.debit += entry.debit;
            row.credit += entry.credit;
        } else {
            grouped.push(VoucherBalanceRow {
                voucher_type: entry.voucher_type,
                voucher_no: entry.voucher_no,
                debit: entry.debit,
                credit: entry.credit,
            });
        }
    }

    grouped
        .into_iter()
        .filter(|entry| entry.debit != entry.credit)
        .collect()
}

pub fn apply_filters(entries: Vec<GlEntry>, filters: &VoucherBalanceFilters) -> Vec<GlEntry> {
    entries
        .into_iter()
        .filter(|entry| !entry.is_cancelled)
        .filter(|entry| {
            filters
                .company
                .as_ref()
                .is_none_or(|company| entry.company == *company)
        })
        .filter(|entry| {
            filters
                .voucher_type
                .as_ref()
                .is_none_or(|voucher_type| entry.voucher_type == *voucher_type)
        })
        .filter(|entry| {
            filters
                .from_date
                .as_ref()
                .is_none_or(|from_date| entry.posting_date >= *from_date)
        })
        .filter(|entry| {
            filters
                .to_date
                .as_ref()
                .is_none_or(|to_date| entry.posting_date <= *to_date)
        })
        .collect()
}
