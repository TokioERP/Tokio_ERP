#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosRegisterError {
    MissingCompany,
    MissingDateRange,
    FromDateAfterToDate,
    GroupedByFilteredField(&'static str),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosRegisterFilters {
    pub company: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub pos_profile: Option<String>,
    pub owner: Option<String>,
    pub customer: Option<String>,
    pub is_return: Option<bool>,
    pub mode_of_payment: Option<String>,
    pub group_by: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosRegisterRow {
    pub posting_date: String,
    pub pos_invoice: String,
    pub pos_profile: String,
    pub company: String,
    pub owner: String,
    pub customer: String,
    pub is_return: bool,
    pub grand_total: f64,
    pub paid_amount: Option<f64>,
    pub mode_of_payment: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PosRegisterSubtotalRow {
    Row(PosRegisterRow),
    Subtotal {
        group_by_field: String,
        group_by_value: String,
        grand_total: f64,
        paid_amount: f64,
        bold: bool,
    },
    Blank,
}

impl ReportColumn {
    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            options,
            width,
        }
    }

    pub const fn date(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Date",
            options: "",
            width,
        }
    }

    pub const fn data(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub const fn currency(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Currency",
            options,
            width,
        }
    }
}

impl PosRegisterRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        posting_date: impl Into<String>,
        pos_invoice: impl Into<String>,
        pos_profile: impl Into<String>,
        company: impl Into<String>,
        owner: impl Into<String>,
        customer: impl Into<String>,
        is_return: bool,
        grand_total: f64,
        paid_amount: Option<f64>,
        mode_of_payment: Option<&str>,
    ) -> Self {
        Self {
            posting_date: posting_date.into(),
            pos_invoice: pos_invoice.into(),
            pos_profile: pos_profile.into(),
            company: company.into(),
            owner: owner.into(),
            customer: customer.into(),
            is_return,
            grand_total,
            paid_amount,
            mode_of_payment: mode_of_payment.map(str::to_string),
        }
    }

    fn group_value(&self, group_by_field: &str) -> String {
        match group_by_field {
            "pos_profile" => self.pos_profile.clone(),
            "owner" => self.owner.clone(),
            "customer" => self.customer.clone(),
            "mode_of_payment" => self.mode_of_payment.clone().unwrap_or_default(),
            _ => String::new(),
        }
    }
}

pub fn execute(
    filters: PosRegisterFilters,
    mut pos_entries: Vec<PosRegisterRow>,
    mode_of_payments: &[(String, Vec<String>)],
) -> Result<(Vec<ReportColumn>, Vec<PosRegisterSubtotalRow>), PosRegisterError> {
    if filters_is_empty(&filters) {
        return Ok((Vec::new(), Vec::new()));
    }

    validate_filters(&filters)?;

    let mut columns = get_columns();
    let group_by_field = get_group_by_field(filters.group_by.as_deref());

    if group_by_field != "mode_of_payment" {
        concat_mode_of_payments(&mut pos_entries, mode_of_payments);
    }

    if group_by_field.is_empty() {
        return Ok((
            columns,
            pos_entries
                .into_iter()
                .map(PosRegisterSubtotalRow::Row)
                .collect(),
        ));
    }

    let mut invoice_map: Vec<(String, Vec<PosRegisterRow>)> = Vec::new();
    for entry in pos_entries {
        let group_value = entry.group_value(group_by_field);
        if let Some((_, entries)) = invoice_map
            .iter_mut()
            .find(|(key, _)| key.as_str() == group_value)
        {
            entries.push(entry);
        } else {
            invoice_map.push((group_value, vec![entry]));
        }
    }

    let mut grouped_data = Vec::new();
    for (key, invoices) in invoice_map {
        grouped_data.extend(invoices.iter().cloned().map(PosRegisterSubtotalRow::Row));
        add_subtotal_row(&mut grouped_data, &invoices, group_by_field, &key);
    }

    if let Some(column_index) = columns
        .iter()
        .position(|column| column.fieldname == group_by_field)
    {
        let column = columns.remove(column_index);
        columns.insert(0, column);
    }

    Ok((columns, grouped_data))
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::date("Posting Date", "posting_date", 90),
        ReportColumn::link("POS Invoice", "pos_invoice", "POS Invoice", 120),
        ReportColumn::link("Customer", "customer", "Customer", 120),
        ReportColumn::link("POS Profile", "pos_profile", "POS Profile", 160),
        ReportColumn::link("Cashier", "owner", "User", 140),
        ReportColumn::currency(
            "Grand Total",
            "grand_total",
            "Company:company:default_currency",
            120,
        ),
        ReportColumn::currency(
            "Paid Amount",
            "paid_amount",
            "Company:company:default_currency",
            120,
        ),
        ReportColumn::data("Payment Method", "mode_of_payment", 150),
        ReportColumn::data("Is Return", "is_return", 80),
        ReportColumn::link("Company", "company", "Company", 120),
    ]
}

pub fn validate_filters(filters: &PosRegisterFilters) -> Result<(), PosRegisterError> {
    if filters.company.is_none() {
        return Err(PosRegisterError::MissingCompany);
    }

    if filters.from_date.is_none() && filters.to_date.is_none() {
        return Err(PosRegisterError::MissingDateRange);
    }

    if let (Some(from_date), Some(to_date)) = (&filters.from_date, &filters.to_date) {
        if from_date > to_date {
            return Err(PosRegisterError::FromDateAfterToDate);
        }
    }

    match filters.group_by.as_deref() {
        Some("POS Profile") if filters.pos_profile.is_some() => {
            Err(PosRegisterError::GroupedByFilteredField("POS Profile"))
        }
        Some("Customer") if filters.customer.is_some() => {
            Err(PosRegisterError::GroupedByFilteredField("Customer"))
        }
        Some("Cashier") if filters.owner.is_some() => {
            Err(PosRegisterError::GroupedByFilteredField("Cashier"))
        }
        Some("Payment Method") if filters.mode_of_payment.is_some() => {
            Err(PosRegisterError::GroupedByFilteredField("Payment Method"))
        }
        _ => Ok(()),
    }
}

pub fn get_conditions(filters: &PosRegisterFilters) -> Vec<&'static str> {
    let mut conditions = vec![
        "company = %(company)s",
        "posting_date >= %(from_date)s",
        "posting_date <= %(to_date)s",
    ];

    if filters.pos_profile.is_some() {
        conditions.push("pos_profile = %(pos_profile)s");
    }
    if filters.owner.is_some() {
        conditions.push("owner = %(owner)s");
    }
    if filters.customer.is_some() {
        conditions.push("customer = %(customer)s");
    }
    if filters.is_return.is_some() {
        conditions.push("is_return = %(is_return)s");
    }
    if filters.mode_of_payment.is_some() {
        conditions.push("exists Sales Invoice Payment matching mode_of_payment");
    }

    conditions
}

pub fn get_group_by_field(group_by: Option<&str>) -> &'static str {
    match group_by {
        Some("POS Profile") => "pos_profile",
        Some("Cashier") => "owner",
        Some("Customer") => "customer",
        Some("Payment Method") => "mode_of_payment",
        _ => "",
    }
}

pub fn get_pos_entries_query_plan(
    filters: &PosRegisterFilters,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match get_group_by_field(filters.group_by.as_deref()) {
        "mode_of_payment" => (
            "p.posting_date, sip.mode_of_payment",
            "Sales Invoice Payment",
            "sip.parent = p.name AND adjusted base_amount != 0",
            "sip.base_amount - IF(sip.type='Cash', p.change_amount, 0)",
        ),
        "" => ("p.posting_date", "", "", ""),
        group_by_field => (
            if group_by_field == "customer" {
                "p.posting_date, p.customer"
            } else if group_by_field == "owner" {
                "p.posting_date, p.owner"
            } else {
                "p.posting_date, p.pos_profile"
            },
            "",
            "",
            "p.base_paid_amount - p.change_amount",
        ),
    }
}

pub fn concat_mode_of_payments(
    pos_entries: &mut [PosRegisterRow],
    mode_of_payments: &[(String, Vec<String>)],
) {
    for entry in pos_entries {
        if let Some((_, payments)) = mode_of_payments
            .iter()
            .find(|(invoice, _)| invoice == &entry.pos_invoice)
        {
            entry.mode_of_payment = Some(payments.join(", "));
        }
    }
}

pub fn add_subtotal_row(
    data: &mut Vec<PosRegisterSubtotalRow>,
    group_invoices: &[PosRegisterRow],
    group_by_field: &str,
    group_by_value: &str,
) {
    let grand_total = group_invoices
        .iter()
        .map(|invoice| invoice.grand_total)
        .sum::<f64>();
    let paid_amount = group_invoices
        .iter()
        .map(|invoice| invoice.paid_amount.unwrap_or(0.0))
        .sum::<f64>();

    data.push(PosRegisterSubtotalRow::Subtotal {
        group_by_field: group_by_field.to_string(),
        group_by_value: group_by_value.to_string(),
        grand_total,
        paid_amount,
        bold: true,
    });
    data.push(PosRegisterSubtotalRow::Blank);
}

fn filters_is_empty(filters: &PosRegisterFilters) -> bool {
    filters == &PosRegisterFilters::default()
}
