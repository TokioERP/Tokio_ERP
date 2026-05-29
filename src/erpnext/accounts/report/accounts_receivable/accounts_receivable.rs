#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountType {
    Receivable,
    Payable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsReceivableArgs {
    pub account_type: AccountType,
    pub naming_by: [String; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableFilters {
    pub company: Option<String>,
    pub report_date: Option<String>,
    pub calculate_ageing_with: Option<String>,
    pub range: Option<String>,
    pub account_type: Option<AccountType>,
    pub group_by_party: bool,
    pub in_party_currency: bool,
    pub party: Vec<String>,
    pub party_account: Option<String>,
    pub based_on_payment_terms: bool,
    pub show_future_payments: bool,
    pub show_delivery_notes: bool,
    pub show_sales_person: bool,
    pub show_remarks: bool,
    pub sales_partner: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableSettings {
    pub default_company: String,
    pub company_currency: String,
    pub currency_precision: u32,
    pub party_naming_by: String,
    pub ple_fetch_method: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableState {
    pub filters: ReceivablePayableFilters,
    pub default_company: String,
    pub age_as_on: String,
    pub ranges: Vec<String>,
    pub range_numbers: Vec<usize>,
    pub ple_fetch_method: String,
    pub company_currency: String,
    pub currency_precision: u32,
    pub dr_or_cr: &'static str,
    pub account_type: AccountType,
    pub party_type: Vec<&'static str>,
    pub skip_total_row: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableRuntime {
    pub account_type: AccountType,
    pub party_naming_by: String,
    pub ranges: Vec<String>,
    pub range_numbers: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: u16,
}

impl ReportColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data",
            options: None,
            width,
        }
    }

    pub fn text(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Text",
            options: None,
            width,
        }
    }

    pub fn date(label: &str, fieldname: &str) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Date",
            options: None,
            width: 90,
        }
    }

    pub fn int(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Int",
            options: None,
            width,
        }
    }

    pub fn link(label: &str, fieldname: &str, options: &'static str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link",
            options: Some(options),
            width,
        }
    }

    pub fn dynamic_link(label: &str, fieldname: &str, options: &'static str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Dynamic Link",
            options: Some(options),
            width,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency",
            options: Some("currency"),
            width,
        }
    }
}

pub fn accounts_receivable_args() -> AccountsReceivableArgs {
    AccountsReceivableArgs {
        account_type: AccountType::Receivable,
        naming_by: ["Selling Settings".to_string(), "cust_master_name".to_string()],
    }
}

impl ReceivablePayableState {
    pub fn new(
        filters: &ReceivablePayableFilters,
        settings: &ReceivablePayableSettings,
        today: &str,
    ) -> Self {
        let mut filters = filters.clone();
        if filters.report_date.is_none() {
            filters.report_date = Some(today.to_string());
        }
        if filters.range.is_none() {
            filters.range = Some("30, 60, 90, 120".to_string());
        }

        let report_date = filters.report_date.clone().unwrap_or_else(|| today.to_string());
        let age_as_on = match filters.calculate_ageing_with.as_deref() {
            None | Some("Today Date") => today.to_string(),
            _ => report_date,
        };
        let ranges: Vec<String> = filters
            .range
            .as_deref()
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit()))
            .map(str::to_string)
            .collect();
        let range_numbers = (1..=ranges.len() + 1).collect();

        Self {
            filters,
            default_company: settings.default_company.clone(),
            age_as_on,
            ranges,
            range_numbers,
            ple_fetch_method: settings
                .ple_fetch_method
                .clone()
                .unwrap_or_else(|| "Buffered Cursor".to_string()),
            company_currency: settings.company_currency.clone(),
            currency_precision: settings.currency_precision,
            dr_or_cr: "",
            account_type: AccountType::Receivable,
            party_type: Vec::new(),
            skip_total_row: 0,
        }
    }

    pub fn with_defaults(mut self) -> Self {
        self.filters
            .company
            .get_or_insert_with(|| self.default_company.clone());
        self.account_type = self.filters.account_type.unwrap_or(AccountType::Receivable);
        self.dr_or_cr = match self.account_type {
            AccountType::Receivable => "debit",
            AccountType::Payable => "credit",
        };
        self.party_type = match self.account_type {
            AccountType::Receivable => vec!["Customer"],
            AccountType::Payable => vec!["Supplier"],
        };

        self.skip_total_row = 0;
        if self.filters.group_by_party {
            self.skip_total_row = 1;
        }
        if self.filters.in_party_currency {
            self.skip_total_row = if self.filters.party.len() == 1 { 0 } else { 1 };
        }

        self
    }
}

pub fn get_currency_fields() -> Vec<&'static str> {
    vec![
        "invoiced",
        "paid",
        "credit_note",
        "outstanding",
        "range1",
        "range2",
        "range3",
        "range4",
        "range5",
        "future_amount",
        "remaining_balance",
    ]
}

pub fn get_columns(
    filters: &ReceivablePayableFilters,
    runtime: &ReceivablePayableRuntime,
) -> Vec<ReportColumn> {
    let mut columns = Vec::new();
    add_column(&mut columns, "Posting Date", Some("posting_date"), "Date", None, 120);
    add_column(&mut columns, "Party Type", Some("party_type"), "Data", None, 100);
    add_column(
        &mut columns,
        "Party",
        Some("party"),
        "Dynamic Link",
        Some("party_type"),
        180,
    );

    let party_account_label = match runtime.account_type {
        AccountType::Receivable => "Receivable Account",
        AccountType::Payable => "Payable Account",
    };
    add_column(
        &mut columns,
        party_account_label,
        Some("party_account"),
        "Link",
        Some("Account"),
        180,
    );

    if runtime.party_naming_by == "Naming Series" {
        match runtime.account_type {
            AccountType::Receivable => {
                add_column(&mut columns, "Customer Name", Some("customer_name"), "Data", None, 120)
            }
            AccountType::Payable => {
                add_column(&mut columns, "Supplier Name", Some("supplier_name"), "Data", None, 120)
            }
        }
    }

    if runtime.account_type == AccountType::Receivable {
        add_column(
            &mut columns,
            "Customer Contact",
            Some("customer_primary_contact"),
            "Link",
            Some("Contact"),
            120,
        );
    }

    add_column(&mut columns, "Cost Center", Some("cost_center"), "Data", None, 120);
    add_column(&mut columns, "Project", Some("project"), "Link", Some("Project"), 120);
    add_column(&mut columns, "Voucher Type", Some("voucher_type"), "Data", None, 120);
    add_column(
        &mut columns,
        "Voucher No",
        Some("voucher_no"),
        "Dynamic Link",
        Some("voucher_type"),
        180,
    );
    add_column(&mut columns, "Due Date", Some("due_date"), "Date", None, 120);

    if runtime.account_type == AccountType::Payable {
        add_column(&mut columns, "Bill No", Some("bill_no"), "Data", None, 120);
        add_column(&mut columns, "Bill Date", Some("bill_date"), "Date", None, 120);
    }

    if filters.based_on_payment_terms {
        add_column(&mut columns, "Payment Term", Some("payment_term"), "Data", None, 120);
        add_column(
            &mut columns,
            "Invoice Grand Total",
            Some("invoice_grand_total"),
            "Currency",
            None,
            120,
        );
    }

    add_column(&mut columns, "Invoiced Amount", Some("invoiced"), "Currency", None, 120);
    add_column(&mut columns, "Paid Amount", Some("paid"), "Currency", None, 120);
    let note_label = match runtime.account_type {
        AccountType::Receivable => "Credit Note",
        AccountType::Payable => "Debit Note",
    };
    add_column(&mut columns, note_label, Some("credit_note"), "Currency", None, 120);
    add_column(
        &mut columns,
        "Outstanding Amount",
        Some("outstanding"),
        "Currency",
        None,
        120,
    );
    add_column(&mut columns, "Age (Days)", Some("age"), "Int", None, 80);
    setup_ageing_columns(&mut columns, &runtime.ranges);
    add_column(
        &mut columns,
        "Currency",
        Some("currency"),
        "Link",
        Some("Currency"),
        80,
    );

    if filters.show_future_payments {
        add_column(&mut columns, "Future Payment Ref", Some("future_ref"), "Data", None, 120);
        add_column(
            &mut columns,
            "Future Payment Amount",
            Some("future_amount"),
            "Currency",
            None,
            120,
        );
        add_column(
            &mut columns,
            "Remaining Balance",
            Some("remaining_balance"),
            "Currency",
            None,
            120,
        );
    }

    if runtime.account_type == AccountType::Receivable {
        add_column(&mut columns, "Customer LPO", Some("po_no"), "Data", None, 120);
        if filters.show_delivery_notes {
            add_column(&mut columns, "Delivery Notes", Some("delivery_notes"), "Data", None, 120);
        }
        add_column(&mut columns, "Territory", Some("territory"), "Link", Some("Territory"), 120);
        add_column(
            &mut columns,
            "Customer Group",
            Some("customer_group"),
            "Link",
            Some("Customer Group"),
            120,
        );
        if filters.show_sales_person {
            add_column(&mut columns, "Sales Person", Some("sales_person"), "Data", None, 120);
        }
        if filters.sales_partner.is_some() {
            add_column(
                &mut columns,
                "Sales Partner",
                Some("default_sales_partner"),
                "Data",
                None,
                120,
            );
        }
    }

    if runtime.account_type == AccountType::Payable {
        add_column(
            &mut columns,
            "Supplier Group",
            Some("supplier_group"),
            "Link",
            Some("Supplier Group"),
            120,
        );
    }

    if filters.show_remarks {
        add_column(&mut columns, "Remarks", Some("remarks"), "Text", None, 200);
    }

    columns
}

fn setup_ageing_columns(columns: &mut Vec<ReportColumn>, ranges: &[String]) {
    let mut labels = ranges.to_vec();
    labels.push("Above".to_string());
    let mut previous = 0;

    add_column(columns, "<0", Some("range0"), "Currency", None, 120);
    for (index, current) in labels.iter().enumerate() {
        let label = format!("{previous}-{current}");
        add_column(
            columns,
            &label,
            Some(format!("range{}", index + 1).as_str()),
            "Currency",
            None,
            120,
        );
        if let Ok(value) = current.parse::<i32>() {
            previous = value + 1;
        }
    }
}

fn add_column(
    columns: &mut Vec<ReportColumn>,
    label: &str,
    fieldname: Option<&str>,
    fieldtype: &'static str,
    options: Option<&'static str>,
    width: u16,
) {
    let fieldname = fieldname
        .map(str::to_string)
        .unwrap_or_else(|| scrub(label));
    let (options, width) = match fieldtype {
        "Currency" => (Some("currency"), width),
        "Date" => (options, 90),
        _ => (options, width),
    };

    columns.push(ReportColumn {
        label: label.to_string(),
        fieldname,
        fieldtype,
        options,
        width,
    });
}

fn scrub(label: &str) -> String {
    label
        .trim()
        .to_ascii_lowercase()
        .replace(|ch: char| !ch.is_ascii_alphanumeric(), "_")
        .trim_matches('_')
        .to_string()
}
