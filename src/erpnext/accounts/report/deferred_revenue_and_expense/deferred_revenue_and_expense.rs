use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeferredType {
    Revenue,
    Expense,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BookingBasis {
    Months,
    Days,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredFilters {
    pub company: String,
    pub filter_based_on: String,
    pub period_start_date: String,
    pub period_end_date: String,
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub periodicity: String,
    pub deferred_type: DeferredType,
    pub with_upcoming_postings: bool,
    pub book_deferred_entries_based_on: BookingBasis,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredEntry {
    pub doc: String,
    pub company: String,
    pub docstatus: i32,
    pub is_cancelled: bool,
    pub enable_deferred_revenue: bool,
    pub enable_deferred_expense: bool,
    pub posting_date: String,
    pub item: String,
    pub item_name: String,
    pub service_start_date: String,
    pub service_end_date: String,
    pub base_net_amount: f64,
    pub deferred_revenue_account: Option<String>,
    pub deferred_expense_account: Option<String>,
    pub gle_posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub posted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Period {
    pub key: String,
    pub label: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeriodTotal {
    pub key: String,
    pub total: f64,
    pub actual: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReportRow {
    pub name: String,
    pub service_start_date: Option<String>,
    pub service_end_date: Option<String>,
    pub amount: f64,
    pub periods: BTreeMap<String, f64>,
    pub indent: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataset {
    pub name: String,
    pub chart_type: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chart {
    pub data: ChartData,
    pub chart_type: String,
    pub height: u16,
    pub x_axis_mode: String,
    pub x_is_series: bool,
    pub stacked: bool,
    pub space_ratio: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredReport {
    pub columns: Vec<ReportColumn>,
    pub data: Vec<ReportRow>,
    pub message: Vec<String>,
    pub chart: Chart,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DeferredItemType {
    Sale,
    Purchase,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredItem {
    pub name: String,
    pub parent: String,
    pub item_name: String,
    pub service_start_date: String,
    pub service_end_date: String,
    pub base_net_amount: f64,
    pub deferred_account: String,
    pub gle_entries: Vec<DeferredEntry>,
    pub period_total: Vec<PeriodTotal>,
    pub last_entry_date: String,
    filters: DeferredFilters,
    period_list: Vec<Period>,
    item_type: DeferredItemType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredInvoice {
    pub name: String,
    pub posting_date: String,
    pub items: Vec<DeferredItem>,
    pub amount_total: f64,
    pub period_total: Vec<PeriodTotal>,
    filters: DeferredFilters,
    period_list: Vec<Period>,
    invoice_type: DeferredType,
}

impl Period {
    pub fn new(key: &str, label: &str, from_date: &str, to_date: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
        }
    }
}

impl ReportColumn {
    pub fn new(label: &str, fieldname: &str, fieldtype: &str) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: fieldtype.to_string(),
            read_only: true,
        }
    }
}

impl DeferredItem {
    pub fn new(
        item: &str,
        parent: &str,
        gle_entries: Vec<DeferredEntry>,
        filters: DeferredFilters,
        period_list: Vec<Period>,
    ) -> Result<Self, String> {
        let first = gle_entries
            .first()
            .ok_or_else(|| "Deferred item requires at least one GL entry.".to_string())?;
        let (item_type, deferred_account) = if let Some(account) = &first.deferred_revenue_account {
            (DeferredItemType::Sale, account.clone())
        } else if let Some(account) = &first.deferred_expense_account {
            (DeferredItemType::Purchase, account.clone())
        } else {
            return Err("Deferred account is required.".to_string());
        };

        let mut last_entry_date = first.service_start_date.clone();
        for entry in &gle_entries {
            if amount_for_type(&item_type, entry) != 0.0 {
                last_entry_date = entry.gle_posting_date.clone();
            }
        }

        Ok(Self {
            name: item.to_string(),
            parent: parent.to_string(),
            item_name: first.item_name.clone(),
            service_start_date: first.service_start_date.clone(),
            service_end_date: first.service_end_date.clone(),
            base_net_amount: first.base_net_amount,
            deferred_account,
            gle_entries,
            period_total: Vec::new(),
            last_entry_date,
            filters,
            period_list,
            item_type,
        })
    }

    pub fn report_data(&self) -> ReportRow {
        let mut periods = BTreeMap::new();
        for period in &self.period_total {
            periods.insert(period.key.clone(), period.total);
        }

        ReportRow {
            name: self.item_name.clone(),
            service_start_date: Some(self.service_start_date.clone()),
            service_end_date: Some(self.service_end_date.clone()),
            amount: self.base_net_amount,
            periods,
            indent: Some(1),
        }
    }

    pub fn get_amount(&self, entry: &DeferredEntry) -> f64 {
        amount_for_type(&self.item_type, entry)
    }

    pub fn get_item_total(&self) -> f64 {
        self.gle_entries
            .iter()
            .map(|entry| self.get_amount(entry))
            .sum()
    }

    pub fn calculate_amount(&self, start_date: &str, end_date: &str) -> Result<f64, String> {
        match self.filters.book_deferred_entries_based_on {
            BookingBasis::Months => self.calculate_monthly_amount(start_date, end_date),
            BookingBasis::Days => self.calculate_days_amount(start_date, end_date),
        }
    }

    pub fn calculate_monthly_amount(
        &self,
        start_date: &str,
        end_date: &str,
    ) -> Result<f64, String> {
        let service_start = SimpleDate::parse(&self.service_start_date)?;
        let service_end = SimpleDate::parse(&self.service_end_date)?;
        let start = SimpleDate::parse(start_date)?;
        let end = SimpleDate::parse(end_date)?;
        let total_months = ((service_end.year - service_start.year) * 12 + service_end.month as i32
            - service_start.month as i32
            + 1) as f64;

        let prorate = date_diff(service_end, service_start) as f64
            / date_diff(last_day(service_end), first_day(service_start)) as f64;
        let actual_months = round_to(total_months * prorate, 1);
        let already_booked_amount = self.get_item_total();
        let mut base_amount = self.base_net_amount / actual_months;

        if base_amount + already_booked_amount > self.base_net_amount {
            base_amount = self.base_net_amount - already_booked_amount;
        }

        if first_day(start) != start || last_day(end) != end {
            let partial_month =
                date_diff(end, start) as f64 / date_diff(last_day(end), first_day(start)) as f64;
            base_amount *= round_to(partial_month, 1);
        }

        Ok(base_amount)
    }

    pub fn calculate_days_amount(&self, start_date: &str, end_date: &str) -> Result<f64, String> {
        let service_start = SimpleDate::parse(&self.service_start_date)?;
        let service_end = SimpleDate::parse(&self.service_end_date)?;
        let start = SimpleDate::parse(start_date)?;
        let end = SimpleDate::parse(end_date)?;

        let total_days = date_diff(service_end, service_start) + 1;
        let total_booking_days = date_diff(end, start) + 1;
        let already_booked_amount = self.get_item_total();
        let mut base_amount = self.base_net_amount * total_booking_days as f64 / total_days as f64;

        if base_amount + already_booked_amount > self.base_net_amount {
            base_amount = self.base_net_amount - already_booked_amount;
        }

        Ok(base_amount)
    }

    pub fn make_dummy_gle(&self, name: &str, date: &str, amount: f64) -> DeferredEntry {
        let mut entry = DeferredEntry {
            doc: self.parent.clone(),
            company: self.filters.company.clone(),
            docstatus: 1,
            is_cancelled: false,
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            posting_date: date.to_string(),
            item: self.name.clone(),
            item_name: self.item_name.clone(),
            service_start_date: self.service_start_date.clone(),
            service_end_date: self.service_end_date.clone(),
            base_net_amount: self.base_net_amount,
            deferred_revenue_account: None,
            deferred_expense_account: None,
            gle_posting_date: date.to_string(),
            debit: 0.0,
            credit: 0.0,
            posted: false,
        };

        match self.item_type {
            DeferredItemType::Sale => {
                entry.deferred_revenue_account = Some(self.deferred_account.clone());
                entry.enable_deferred_revenue = true;
                entry.debit = amount;
            }
            DeferredItemType::Purchase => {
                entry.deferred_expense_account = Some(self.deferred_account.clone());
                entry.enable_deferred_expense = true;
                entry.credit = amount;
            }
        }

        entry.gle_posting_date = date.to_string();
        entry.item = name.to_string();
        entry
    }

    pub fn simulate_future_posting(&mut self) -> Result<(), String> {
        let service_start = SimpleDate::parse(&self.service_start_date)?;
        let service_end = SimpleDate::parse(&self.service_end_date)?;
        let last_entry_date = SimpleDate::parse(&self.last_entry_date)?;

        if service_start != service_end && add_days(last_entry_date, 1) < service_end {
            let periods = monthly_period_list(add_days(last_entry_date, 1), service_end);
            for period in periods {
                let amount = self.calculate_amount(&period.from_date, &period.to_date)?;
                let gle = self.make_dummy_gle(&period.key, &period.to_date, amount);
                self.gle_entries.push(gle);
            }
        }

        Ok(())
    }

    pub fn calculate_item_revenue_expense_for_period(
        &mut self,
    ) -> Result<Vec<PeriodTotal>, String> {
        self.period_total.clear();
        for period in &self.period_list {
            let from = SimpleDate::parse(&period.from_date)?;
            let to = SimpleDate::parse(&period.to_date)?;
            let mut period_sum = 0.0;
            let mut actual = 0.0;

            for posting in &self.gle_entries {
                let posting_date = SimpleDate::parse(&posting.gle_posting_date)?;
                if from <= posting_date && posting_date <= to {
                    let amount = self.get_amount(posting);
                    period_sum += amount;
                    if posting.posted {
                        actual += amount;
                    }
                }
            }

            self.period_total.push(PeriodTotal {
                key: period.key.clone(),
                total: period_sum,
                actual,
            });
        }

        Ok(self.period_total.clone())
    }
}

impl DeferredInvoice {
    pub fn new(
        invoice: &str,
        items: Vec<DeferredEntry>,
        filters: DeferredFilters,
        period_list: Vec<Period>,
    ) -> Result<Self, String> {
        let first = items
            .first()
            .ok_or_else(|| "Deferred invoice requires at least one item.".to_string())?;
        let invoice_type = if first.deferred_revenue_account.is_some() {
            DeferredType::Revenue
        } else if first.deferred_expense_account.is_some() {
            DeferredType::Expense
        } else {
            return Err("Deferred account is required.".to_string());
        };

        let uniq_items = items
            .iter()
            .map(|entry| entry.item.clone())
            .collect::<BTreeSet<_>>();
        let mut deferred_items = Vec::new();
        for item in uniq_items {
            let item_entries = items
                .iter()
                .filter(|entry| entry.item == item)
                .cloned()
                .collect::<Vec<_>>();
            deferred_items.push(DeferredItem::new(
                &item,
                invoice,
                item_entries,
                filters.clone(),
                period_list.clone(),
            )?);
        }

        let amount_total = deferred_items
            .iter()
            .map(|item| item.base_net_amount)
            .sum::<f64>();

        Ok(Self {
            name: invoice.to_string(),
            posting_date: first.posting_date.clone(),
            items: deferred_items,
            amount_total,
            period_total: Vec::new(),
            filters,
            period_list,
            invoice_type,
        })
    }

    pub fn calculate_invoice_revenue_expense_for_period(
        &mut self,
    ) -> Result<Vec<PeriodTotal>, String> {
        self.period_total = self
            .period_list
            .iter()
            .map(|period| PeriodTotal {
                key: period.key.clone(),
                total: 0.0,
                actual: 0.0,
            })
            .collect();

        for item in &mut self.items {
            let item_total = item.calculate_item_revenue_expense_for_period()?;
            for (idx, total) in item_total.iter().enumerate() {
                self.period_total[idx].total += total.total;
                self.period_total[idx].actual += total.actual;
            }
        }

        Ok(self.period_total.clone())
    }

    pub fn estimate_future(&mut self) -> Result<(), String> {
        for item in &mut self.items {
            item.simulate_future_posting()?;
        }
        Ok(())
    }

    pub fn report_data(&self) -> Vec<ReportRow> {
        let mut invoice_periods = BTreeMap::new();
        for period in &self.period_total {
            invoice_periods.insert(period.key.clone(), period.total);
        }

        let mut rows = vec![ReportRow {
            name: self.name.clone(),
            service_start_date: None,
            service_end_date: None,
            amount: self.amount_total,
            periods: invoice_periods,
            indent: Some(0),
        }];

        rows.extend(self.items.iter().map(DeferredItem::report_data));
        rows
    }
}

pub fn execute(
    filters: DeferredFilters,
    period_list: Vec<Period>,
    entries: Vec<DeferredEntry>,
) -> Result<DeferredReport, String> {
    let mut report = DeferredRevenueAndExpenseReport::new(filters, period_list, entries);
    report.run()?;

    Ok(DeferredReport {
        columns: report.get_columns(),
        data: report.generate_report_data(),
        message: Vec::new(),
        chart: report.prepare_chart(),
    })
}

struct DeferredRevenueAndExpenseReport {
    filters: DeferredFilters,
    period_list: Vec<Period>,
    entries: Vec<DeferredEntry>,
    deferred_invoices: Vec<DeferredInvoice>,
    period_total: Vec<PeriodTotal>,
}

impl DeferredRevenueAndExpenseReport {
    fn new(
        filters: DeferredFilters,
        period_list: Vec<Period>,
        entries: Vec<DeferredEntry>,
    ) -> Self {
        Self {
            filters,
            period_list,
            entries,
            deferred_invoices: Vec::new(),
            period_total: Vec::new(),
        }
    }

    fn get_invoices(&mut self) -> Result<(), String> {
        let docs = self
            .entries
            .iter()
            .map(|entry| entry.doc.clone())
            .collect::<BTreeSet<_>>();

        for doc in docs {
            let items = self
                .entries
                .iter()
                .filter(|entry| entry.doc == doc && self.entry_matches_filters(entry))
                .cloned()
                .collect::<Vec<_>>();
            if !items.is_empty() {
                self.deferred_invoices.push(DeferredInvoice::new(
                    &doc,
                    items,
                    self.filters.clone(),
                    self.period_list.clone(),
                )?);
            }
        }

        Ok(())
    }

    fn entry_matches_filters(&self, entry: &DeferredEntry) -> bool {
        if entry.docstatus != 1 || entry.company != self.filters.company || entry.is_cancelled {
            return false;
        }

        match self.filters.deferred_type {
            DeferredType::Revenue => {
                if !entry.enable_deferred_revenue || entry.deferred_revenue_account.is_none() {
                    return false;
                }
            }
            DeferredType::Expense => {
                if !entry.enable_deferred_expense || entry.deferred_expense_account.is_none() {
                    return false;
                }
            }
        }

        service_overlaps_periods(
            &entry.service_start_date,
            &entry.service_end_date,
            &self.period_list,
        )
        .unwrap_or(false)
    }

    fn estimate_future(&mut self) -> Result<(), String> {
        for invoice in &mut self.deferred_invoices {
            invoice.estimate_future()?;
        }
        Ok(())
    }

    fn calculate_revenue_and_expense(&mut self) -> Result<(), String> {
        self.period_total = self
            .period_list
            .iter()
            .map(|period| PeriodTotal {
                key: period.key.clone(),
                total: 0.0,
                actual: 0.0,
            })
            .collect();

        for invoice in &mut self.deferred_invoices {
            let invoice_total = invoice.calculate_invoice_revenue_expense_for_period()?;
            for (idx, total) in invoice_total.iter().enumerate() {
                self.period_total[idx].total += total.total;
                self.period_total[idx].actual += total.actual;
            }
        }

        Ok(())
    }

    fn get_columns(&self) -> Vec<ReportColumn> {
        let mut columns = vec![
            ReportColumn::new("Name", "name", "Data"),
            ReportColumn::new("Service Start Date", "service_start_date", "Date"),
            ReportColumn::new("Service End Date", "service_end_date", "Date"),
            ReportColumn::new("Amount", "amount", "Currency"),
        ];
        columns.extend(
            self.period_list
                .iter()
                .map(|period| ReportColumn::new(&period.label, &period.key, "Currency")),
        );
        columns
    }

    fn generate_report_data(&self) -> Vec<ReportRow> {
        let mut rows = Vec::new();
        for invoice in &self.deferred_invoices {
            rows.extend(invoice.report_data());
        }

        rows.push(ReportRow {
            name: String::new(),
            service_start_date: None,
            service_end_date: None,
            amount: 0.0,
            periods: BTreeMap::new(),
            indent: None,
        });

        let mut periods = BTreeMap::new();
        for period in &self.period_total {
            periods.insert(period.key.clone(), period.total);
        }

        rows.push(ReportRow {
            name: match self.filters.deferred_type {
                DeferredType::Revenue => "Total Deferred Income".to_string(),
                DeferredType::Expense => "Total Deferred Expense".to_string(),
            },
            service_start_date: None,
            service_end_date: None,
            amount: self
                .deferred_invoices
                .iter()
                .map(|invoice| invoice.amount_total)
                .sum(),
            periods,
            indent: None,
        });

        rows
    }

    fn prepare_chart(&self) -> Chart {
        let mut datasets = vec![ChartDataset {
            name: "Actual Posting".to_string(),
            chart_type: "bar".to_string(),
            values: self
                .period_total
                .iter()
                .map(|period| period.actual)
                .collect(),
        }];

        if self.filters.with_upcoming_postings {
            datasets.push(ChartDataset {
                name: "Expected".to_string(),
                chart_type: "line".to_string(),
                values: self
                    .period_total
                    .iter()
                    .map(|period| period.total)
                    .collect(),
            });
        }

        Chart {
            data: ChartData {
                labels: self
                    .period_list
                    .iter()
                    .map(|period| period.label.clone())
                    .collect(),
                datasets,
            },
            chart_type: "axis-mixed".to_string(),
            height: 500,
            x_axis_mode: "Tick".to_string(),
            x_is_series: true,
            stacked: false,
            space_ratio: 0.5,
        }
    }

    fn run(&mut self) -> Result<(), String> {
        self.deferred_invoices.clear();
        self.get_invoices()?;
        if self.filters.with_upcoming_postings {
            self.estimate_future()?;
        }
        self.calculate_revenue_and_expense()
    }
}

fn amount_for_type(item_type: &DeferredItemType, entry: &DeferredEntry) -> f64 {
    match item_type {
        DeferredItemType::Sale => entry.debit - entry.credit,
        DeferredItemType::Purchase => -(entry.credit - entry.debit),
    }
}

fn monthly_period_list(start: SimpleDate, end: SimpleDate) -> Vec<Period> {
    let mut periods = Vec::new();
    let mut current = start;
    while current <= end {
        let period_end = last_day(current).min(end);
        periods.push(Period::new(
            &format!("{}_{:02}", current.year, current.month),
            &format!("{} {}", month_label(current.month), current.year),
            &current.format(),
            &period_end.format(),
        ));
        current = add_days(period_end, 1);
    }
    periods
}

fn service_overlaps_periods(
    service_start: &str,
    service_end: &str,
    period_list: &[Period],
) -> Result<bool, String> {
    let Some(first_period) = period_list.first() else {
        return Ok(false);
    };
    let Some(last_period) = period_list.last() else {
        return Ok(false);
    };

    let service_start = SimpleDate::parse(service_start)?;
    let service_end = SimpleDate::parse(service_end)?;
    let report_start = SimpleDate::parse(&first_period.from_date)?;
    let report_end = SimpleDate::parse(&last_period.to_date)?;

    Ok(
        (report_start >= service_start && service_end >= report_start)
            || (service_start >= report_start && service_start <= report_end),
    )
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: u32,
    day: u32,
}

impl SimpleDate {
    fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split('-');
        let year = parts
            .next()
            .ok_or_else(|| format!("Invalid date: {input}"))?
            .parse::<i32>()
            .map_err(|_| format!("Invalid date: {input}"))?;
        let month = parts
            .next()
            .ok_or_else(|| format!("Invalid date: {input}"))?
            .parse::<u32>()
            .map_err(|_| format!("Invalid date: {input}"))?;
        let day = parts
            .next()
            .ok_or_else(|| format!("Invalid date: {input}"))?
            .parse::<u32>()
            .map_err(|_| format!("Invalid date: {input}"))?;

        if parts.next().is_some() || !(1..=12).contains(&month) {
            return Err(format!("Invalid date: {input}"));
        }
        let last_day = days_in_month(year, month);
        if day == 0 || day > last_day {
            return Err(format!("Invalid date: {input}"));
        }

        Ok(Self { year, month, day })
    }

    fn format(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn date_diff(end: SimpleDate, start: SimpleDate) -> i64 {
    days_from_civil(end) - days_from_civil(start)
}

fn add_days(date: SimpleDate, days: i64) -> SimpleDate {
    civil_from_days(days_from_civil(date) + days)
}

fn first_day(date: SimpleDate) -> SimpleDate {
    SimpleDate {
        year: date.year,
        month: date.month,
        day: 1,
    }
}

fn last_day(date: SimpleDate) -> SimpleDate {
    SimpleDate {
        year: date.year,
        month: date.month,
        day: days_in_month(date.year, date.month),
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(date: SimpleDate) -> i64 {
    let mut year = date.year;
    let month = date.month as i32;
    let day = date.day as i32;
    year -= (month <= 2) as i32;
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_prime = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_prime + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146_097 + doe - 719_468) as i64
}

fn civil_from_days(days: i64) -> SimpleDate {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += (month <= 2) as i64;

    SimpleDate {
        year: year as i32,
        month: month as u32,
        day: day as u32,
    }
}

fn month_label(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

fn round_to(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    (value * multiplier).round() / multiplier
}
