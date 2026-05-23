use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessStatementOfAccounts {
    pub name: String,
    pub report: String,
    pub from_date: Option<String>,
    pub posting_date: Option<String>,
    pub company: String,
    pub account: Option<String>,
    pub categorize_by: Option<String>,
    pub cost_center: Vec<String>,
    pub territory: Option<String>,
    pub ignore_exchange_rate_revaluation_journals: bool,
    pub ignore_cr_dr_notes: bool,
    pub to_date: Option<String>,
    pub finance_book: Option<String>,
    pub currency: Option<String>,
    pub project: Vec<String>,
    pub payment_terms_template: Option<String>,
    pub sales_partner: Option<String>,
    pub sales_person: Option<String>,
    pub show_remarks: bool,
    pub based_on_payment_terms: bool,
    pub show_future_payments: bool,
    pub customer_collection: Option<String>,
    pub collection_name: Option<String>,
    pub primary_mandatory: bool,
    pub show_net_values_in_party_account: bool,
    pub customers: Vec<CustomerRow>,
    pub print_format: Option<String>,
    pub orientation: String,
    pub include_break: bool,
    pub include_ageing: bool,
    pub ageing_based_on: String,
    pub letter_head: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub enable_auto_email: bool,
    pub sender: Option<String>,
    pub frequency: String,
    pub filter_duration: i32,
    pub start_date: Option<String>,
    pub pdf_name: Option<String>,
    pub subject: Option<String>,
    pub cc_to: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomerRow {
    pub customer: String,
    pub customer_name: Option<String>,
    pub primary_email: Option<String>,
    pub billing_email: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintFormatInfo {
    pub print_format_type: String,
    pub print_format_for: String,
    pub report: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoEmailDates {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub posting_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommonFilters {
    pub company: String,
    pub account: Vec<String>,
    pub finance_book: Option<String>,
    pub cost_center: Vec<String>,
    pub show_remarks: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlFilters {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub party_type: String,
    pub party: Vec<String>,
    pub party_name: Vec<String>,
    pub presentation_currency: String,
    pub categorize_by: Option<String>,
    pub currency: Option<String>,
    pub project: Vec<String>,
    pub show_opening_entries: bool,
    pub include_default_book_entries: bool,
    pub tax_id: Option<String>,
    pub show_net_values_in_party_account: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArFilters {
    pub report_date: Option<String>,
    pub party_type: String,
    pub party: Vec<String>,
    pub customer_name: Option<String>,
    pub payment_terms_template: Option<String>,
    pub sales_partner: Option<String>,
    pub sales_person: Option<String>,
    pub territory: Option<String>,
    pub based_on_payment_terms: bool,
    pub show_future_payments: bool,
    pub report_name: String,
    pub ageing_based_on: String,
    pub range1: i32,
    pub range2: i32,
    pub range3: i32,
    pub range4: i32,
}

impl CustomerRow {
    pub fn new(customer: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            customer_name: None,
            primary_email: None,
            billing_email: None,
        }
    }

    pub fn named(customer: impl Into<String>, customer_name: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            customer_name: Some(customer_name.into()),
            primary_email: None,
            billing_email: None,
        }
    }
}

impl PrintFormatInfo {
    pub fn new(
        print_format_type: impl Into<String>,
        print_format_for: impl Into<String>,
        report: impl Into<String>,
        disabled: bool,
    ) -> Self {
        Self {
            print_format_type: print_format_type.into(),
            print_format_for: print_format_for.into(),
            report: report.into(),
            disabled,
        }
    }
}

impl ProcessStatementOfAccounts {
    pub const DOCTYPE: &'static str = "Process Statement Of Accounts";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "Prompt";
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 57] = [
        "report",
        "section_break_11",
        "from_date",
        "posting_date",
        "company",
        "account",
        "categorize_by",
        "cost_center",
        "territory",
        "ignore_exchange_rate_revaluation_journals",
        "ignore_cr_dr_notes",
        "column_break_14",
        "to_date",
        "finance_book",
        "currency",
        "project",
        "payment_terms_template",
        "sales_partner",
        "sales_person",
        "show_remarks",
        "based_on_payment_terms",
        "show_future_payments",
        "section_break_3",
        "customer_collection",
        "collection_name",
        "fetch_customers",
        "column_break_6",
        "primary_mandatory",
        "show_net_values_in_party_account",
        "column_break_17",
        "customers",
        "preferences",
        "print_format",
        "orientation",
        "include_break",
        "include_ageing",
        "ageing_based_on",
        "section_break_14",
        "letter_head",
        "terms_and_conditions",
        "section_break_1",
        "enable_auto_email",
        "column_break_ocfq",
        "sender",
        "section_break_18",
        "frequency",
        "filter_duration",
        "column_break_21",
        "start_date",
        "section_break_33",
        "pdf_name",
        "subject",
        "column_break_28",
        "cc_to",
        "section_break_30",
        "body",
        "help_text",
    ];

    pub fn new(
        name: impl Into<String>,
        company: impl Into<String>,
        report: impl Into<String>,
        customers: Vec<CustomerRow>,
    ) -> Self {
        Self {
            name: name.into(),
            report: report.into(),
            from_date: None,
            posting_date: None,
            company: company.into(),
            account: None,
            categorize_by: None,
            cost_center: Vec::new(),
            territory: None,
            ignore_exchange_rate_revaluation_journals: false,
            ignore_cr_dr_notes: false,
            to_date: None,
            finance_book: None,
            currency: None,
            project: Vec::new(),
            payment_terms_template: None,
            sales_partner: None,
            sales_person: None,
            show_remarks: false,
            based_on_payment_terms: false,
            show_future_payments: false,
            customer_collection: None,
            collection_name: None,
            primary_mandatory: false,
            show_net_values_in_party_account: false,
            customers,
            print_format: None,
            orientation: "Landscape".to_string(),
            include_break: true,
            include_ageing: false,
            ageing_based_on: "Due Date".to_string(),
            letter_head: None,
            terms_and_conditions: None,
            enable_auto_email: false,
            sender: None,
            frequency: "Weekly".to_string(),
            filter_duration: 1,
            start_date: None,
            pdf_name: None,
            subject: None,
            cc_to: Vec::new(),
            body: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("report", "Report")
                .options("General Ledger\nAccounts Receivable")
                .required(),
            FieldSpec::section_break("section_break_11"),
            FieldSpec::date("from_date", "From Date")
                .depends_on("eval:(!doc.enable_auto_email && doc.report == 'General Ledger');")
                .mandatory_depends_on(
                    "eval:(!doc.enable_auto_email && doc.report == \"General Ledger\") ",
                ),
            FieldSpec::date("posting_date", "Posting Date")
                .depends_on("eval:(!doc.enable_auto_email && doc.report == 'Accounts Receivable');"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("account", "Account").options("Account"),
            FieldSpec::select("categorize_by", "Categorize by").options(
                "Categorize by Voucher (Consolidated)\nCategorize by Voucher\nCategorize by Account",
            ),
            FieldSpec::table_multiselect("cost_center", "Cost Center").options("PSOA Cost Center"),
            FieldSpec::link("territory", "Territory").options("Territory"),
            FieldSpec::check(
                "ignore_exchange_rate_revaluation_journals",
                "Ignore Exchange Rate Revaluation Journals",
            ),
            FieldSpec::check("ignore_cr_dr_notes", "Ignore CR/DR Notes"),
            FieldSpec::column_break("column_break_14"),
            FieldSpec::date("to_date", "To Date"),
            FieldSpec::link("finance_book", "Finance Book").options("Finance Book"),
            FieldSpec::link("currency", "Currency").options("Currency"),
            FieldSpec::table_multiselect("project", "Project").options("PSOA Project"),
            FieldSpec::link("payment_terms_template", "Payment Terms Template")
                .options("Payment Terms Template"),
            FieldSpec::link("sales_partner", "Sales Partner").options("Sales Partner"),
            FieldSpec::link("sales_person", "Sales Person").options("Sales Person"),
            FieldSpec::check("show_remarks", "Show Remarks"),
            FieldSpec::check("based_on_payment_terms", "Based On Payment Terms"),
            FieldSpec::check("show_future_payments", "Show Future Payments"),
            FieldSpec::section_break("section_break_3"),
            FieldSpec::select("customer_collection", "Customer Collection")
                .options("\nCustomer Group\nTerritory\nSales Partner\nSales Person"),
            FieldSpec::dynamic_link("collection_name")
                .label("Collection Name")
                .options("customer_collection"),
            FieldSpec::button("fetch_customers", "Fetch Customers")
                .options("fetch_customers")
                .print_hide()
                .report_hide()
                .depends_on("eval: doc.customer_collection !== ''"),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::check("primary_mandatory", "Send To Primary Contact"),
            FieldSpec::check(
                "show_net_values_in_party_account",
                "Show Net Values in Party Account",
            ),
            FieldSpec::column_break("column_break_17"),
            FieldSpec::table("customers", "Customers").options("Process Statement Of Accounts Customer"),
            FieldSpec::section_break("preferences").label("Preferences"),
            FieldSpec::link("print_format", "Print Format").options("Print Format"),
            FieldSpec::select("orientation", "Orientation").options("Landscape\nPortrait"),
            FieldSpec::check("include_break", "Include Break"),
            FieldSpec::check("include_ageing", "Include Ageing"),
            FieldSpec::select("ageing_based_on", "Ageing Based On").options("Due Date\nPosting Date"),
            FieldSpec::section_break("section_break_14"),
            FieldSpec::link("letter_head", "Letter Head").options("Letter Head"),
            FieldSpec::link("terms_and_conditions", "Terms and Conditions")
                .options("Terms and Conditions"),
            FieldSpec::section_break("section_break_1"),
            FieldSpec::check("enable_auto_email", "Enable Auto Email"),
            FieldSpec::column_break("column_break_ocfq"),
            FieldSpec::link("sender", "Sender").options("Email Account"),
            FieldSpec::section_break("section_break_18"),
            FieldSpec::select("frequency", "Frequency")
                .options("Daily\nWeekly\nBiweekly\nMonthly\nQuarterly"),
            FieldSpec::int("filter_duration", "Filter Duration"),
            FieldSpec::column_break("column_break_21"),
            FieldSpec::date("start_date", "Start Date"),
            FieldSpec::section_break("section_break_33"),
            FieldSpec::data("pdf_name", "PDF Name"),
            FieldSpec::data("subject", "Subject"),
            FieldSpec::column_break("column_break_28"),
            FieldSpec::table("cc_to", "CC To").options("Process Statement Of Accounts CC"),
            FieldSpec::section_break("section_break_30"),
            FieldSpec::text_editor("body", "Body"),
            FieldSpec::html("help_text", "Help Text"),
        ]
    }

    pub fn validate(
        &mut self,
        account_company: Option<&str>,
        invalid_cost_centers: &[&str],
        invalid_projects: &[&str],
        print_format: Option<PrintFormatInfo>,
        today: &str,
    ) -> Result<(), String> {
        if self.customers.is_empty() {
            return Err("Customers not selected.".to_string());
        }

        if let Some(account) = &self.account {
            if let Some(account_company) = account_company {
                if account_company != self.company {
                    return Err(format!(
                        "Account {account} doesn't belong to Company {}",
                        self.company
                    ));
                }
            }
        }

        if let Some(cost_center) = self
            .cost_center
            .iter()
            .find(|cost_center| invalid_cost_centers.contains(&cost_center.as_str()))
        {
            return Err(format!(
                "Cost Center {cost_center} doesn't belong to Company {}",
                self.company
            ));
        }

        if let Some(project) = self
            .project
            .iter()
            .find(|project| invalid_projects.contains(&project.as_str()))
        {
            return Err(format!(
                "Project {project} doesn't belong to Company {}",
                self.company
            ));
        }

        if let Some(print_format) = print_format {
            if print_format.print_format_type != "Jinja" {
                return Err("Print Format Type should be Jinja.".to_string());
            }

            if print_format.print_format_for != "Report"
                || print_format.report != self.report
                || print_format.disabled
            {
                return Err(
                    "Print Format must be an enabled Report Print Format matching the selected Report."
                        .to_string(),
                );
            }
        }

        if self.subject.is_none() {
            self.subject =
                Some("Statement Of Accounts for {{ customer.customer_name }}".to_string());
        }

        if self.body.is_none() {
            self.body = Some(if self.report == "Accounts Receivable" {
                "Hello {{ customer.customer_name }},<br>PFA your Statement Of Accounts until {{ doc.posting_date }}.".to_string()
            } else {
                "Hello {{ customer.customer_name }},<br>PFA your Statement Of Accounts from {{ doc.from_date }} to {{ doc.to_date }}.".to_string()
            });
        }

        if self.pdf_name.is_none() {
            self.pdf_name = Some("{{ customer.customer_name }}".to_string());
        }

        if self.enable_auto_email {
            if let Some(start_date) = &self.start_date {
                if start_date.as_str() >= today {
                    if self.report == "General Ledger" {
                        self.to_date = Some(start_date.clone());
                        self.from_date = Some(add_months(start_date, -self.filter_duration)?);
                    } else {
                        self.posting_date = Some(start_date.clone());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn next_auto_email_dates(&self, current_date: &str) -> AutoEmailDates {
        let next_date = add_frequency(current_date, &self.frequency)
            .unwrap_or_else(|_| current_date.to_string());

        if self.report == "General Ledger" {
            AutoEmailDates {
                from_date: add_months(&next_date, -self.filter_duration).ok(),
                to_date: Some(next_date),
                posting_date: None,
            }
        } else {
            AutoEmailDates {
                from_date: None,
                to_date: None,
                posting_date: Some(next_date),
            }
        }
    }

    pub fn common_filters(&self) -> CommonFilters {
        CommonFilters {
            company: self.company.clone(),
            account: self.account.iter().cloned().collect(),
            finance_book: self.finance_book.clone(),
            cost_center: self.cost_center.clone(),
            show_remarks: self.show_remarks,
        }
    }

    pub fn gl_filters(
        &self,
        customer: &CustomerRow,
        tax_id: Option<&str>,
        presentation_currency: &str,
    ) -> GlFilters {
        GlFilters {
            from_date: self.from_date.clone(),
            to_date: self.to_date.clone(),
            party_type: "Customer".to_string(),
            party: vec![customer.customer.clone()],
            party_name: customer.customer_name.iter().cloned().collect(),
            presentation_currency: presentation_currency.to_string(),
            categorize_by: self.categorize_by.clone(),
            currency: self.currency.clone(),
            project: self.project.clone(),
            show_opening_entries: false,
            include_default_book_entries: false,
            tax_id: tax_id.map(str::to_string),
            show_net_values_in_party_account: self.show_net_values_in_party_account,
        }
    }

    pub fn ar_filters(&self, customer: &CustomerRow) -> ArFilters {
        ArFilters {
            report_date: self.posting_date.clone(),
            party_type: "Customer".to_string(),
            party: vec![customer.customer.clone()],
            customer_name: customer.customer_name.clone(),
            payment_terms_template: self.payment_terms_template.clone(),
            sales_partner: self.sales_partner.clone(),
            sales_person: self.sales_person.clone(),
            territory: self.territory.clone(),
            based_on_payment_terms: self.based_on_payment_terms,
            show_future_payments: self.show_future_payments,
            report_name: self.report.clone(),
            ageing_based_on: self.ageing_based_on.clone(),
            range1: 30,
            range2: 60,
            range3: 90,
            range4: 120,
        }
    }

    pub fn recipients_and_cc(&self, customer: &str) -> (Vec<String>, Vec<String>) {
        let mut recipients = Vec::new();

        if let Some(customer) = self.customers.iter().find(|row| row.customer == customer) {
            push_emails(&mut recipients, customer.billing_email.as_deref());
            if self.primary_mandatory {
                push_emails(&mut recipients, customer.primary_email.as_deref());
            }
        }

        let mut cc = Vec::new();
        for (_, email) in &self.cc_to {
            push_emails(&mut cc, Some(email));
        }

        (recipients, cc)
    }
}

impl DocumentController for ProcessStatementOfAccounts {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

fn push_emails(target: &mut Vec<String>, emails: Option<&str>) {
    if let Some(emails) = emails {
        for email in emails
            .split(',')
            .map(str::trim)
            .filter(|email| !email.is_empty())
        {
            if !target.iter().any(|existing| existing == email) {
                target.push(email.to_string());
            }
        }
    }
}

fn add_frequency(date: &str, frequency: &str) -> Result<String, String> {
    match frequency {
        "Daily" => add_days(date, 1),
        "Weekly" => add_days(date, 7),
        "Biweekly" => add_days(date, 14),
        "Monthly" => add_months(date, 1),
        "Quarterly" => add_months(date, 3),
        _ => add_days(date, 7),
    }
}

fn add_days(date: &str, days: i64) -> Result<String, String> {
    Ok(format_date(parse_date(date)? + days))
}

fn add_months(date: &str, months: i32) -> Result<String, String> {
    let (mut year, month, day) = parse_date_parts(date)?;
    let zero_based = month as i32 - 1 + months;
    year += zero_based.div_euclid(12) as i64;
    let month = zero_based.rem_euclid(12) as i64 + 1;
    let day = day.min(days_in_month(year, month));

    Ok(format!("{year:04}-{month:02}-{day:02}"))
}

fn parse_date_parts(date: &str) -> Result<(i64, i64, i64), String> {
    let mut parts = date.split('-');
    let year = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;
    let month = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;
    let day = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;

    if parts.next().is_some()
        || !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
    {
        return Err(format!("Invalid date: {date}"));
    }

    Ok((year, month, day))
}

fn parse_date(date: &str) -> Result<i64, String> {
    let (year, month, day) = parse_date_parts(date)?;
    Ok(days_from_civil(year, month, day))
}

fn format_date(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_prime = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_prime + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let month_prime = (5 * doy + 2) / 153;
    let day = doy - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };

    (year, month, day)
}
