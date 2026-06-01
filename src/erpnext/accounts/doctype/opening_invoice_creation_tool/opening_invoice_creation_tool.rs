use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum InvoiceType {
    #[default]
    Sales,
    Purchase,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceCreationTool {
    pub name: String,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub create_missing_party: bool,
    pub invoice_type: InvoiceType,
    pub invoices: Vec<OpeningInvoiceRow>,
    pub project: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceRow {
    pub idx: usize,
    pub party: Option<String>,
    pub party_name: Option<String>,
    pub party_type: Option<String>,
    pub item_name: Option<String>,
    pub qty: f64,
    pub temporary_opening_account: Option<String>,
    pub posting_date: Option<String>,
    pub due_date: Option<String>,
    pub outstanding_amount: f64,
    pub cost_center: Option<String>,
    pub invoice_number: Option<String>,
    pub supplier_invoice_date: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyDetails {
    pub default_currency: Option<String>,
    pub default_letter_head: Option<String>,
    pub cost_center: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyDefaults {
    pub default_currency: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyCreationPlan {
    pub party_type: String,
    pub party_name: String,
    pub supplier_group: Option<String>,
    pub ignore_mandatory: bool,
    pub ignore_permissions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceItem {
    pub uom: String,
    pub rate: f64,
    pub qty: f64,
    pub conversion_factor: f64,
    pub item_name: String,
    pub description: String,
    pub income_account: Option<String>,
    pub expense_account: Option<String>,
    pub cost_center: String,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceDoc {
    pub items: Vec<OpeningInvoiceItem>,
    pub is_opening: String,
    pub set_posting_time: bool,
    pub company: String,
    pub cost_center: Option<String>,
    pub due_date: Option<String>,
    pub posting_date: Option<String>,
    pub party_field: String,
    pub party: String,
    pub is_pos: bool,
    pub doctype: String,
    pub update_stock: bool,
    pub invoice_number: Option<String>,
    pub disable_rounded_total: bool,
    pub bill_date: Option<String>,
    pub currency: Option<String>,
    pub letter_head: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MakeInvoicesPlan {
    pub mode: String,
    pub job_id: Option<String>,
    pub enqueue: bool,
    pub queue: Option<String>,
    pub timeout: Option<u64>,
    pub event: Option<String>,
    pub run_now: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImportAttempt {
    pub doctype: String,
    pub invoice_number: Option<String>,
    pub generated_name: String,
    pub succeeds: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImportPlanResult {
    pub names: Vec<String>,
    pub errors: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PublishPayload {
    pub title: String,
    pub message: String,
    pub count: usize,
    pub total: usize,
    pub user: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompanyCurrency {
    pub company: String,
    pub currency: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceAggregate {
    pub company: String,
    pub total_invoices: usize,
    pub outstanding_amount: f64,
    pub paid_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompanyOpeningInvoiceSummary {
    pub currency: Option<String>,
    pub sales_invoice: Option<OpeningInvoiceAggregate>,
    pub purchase_invoice: Option<OpeningInvoiceAggregate>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceMaxCount {
    pub max_paid: f64,
    pub max_due: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpeningInvoiceCreationError {
    MissingCompany,
    MissingPartyId {
        idx: usize,
    },
    MissingPartyIdOrName {
        idx: usize,
    },
    PartyDoesNotExist {
        idx: usize,
        party_type: String,
        party: String,
    },
    MissingSupplierGroup,
    MissingOutstandingAmount {
        idx: usize,
        invoice_type: InvoiceType,
    },
    MissingTemporaryOpeningAccountForRow {
        idx: usize,
        invoice_type: InvoiceType,
    },
    MissingCompanyCostCenter {
        company: String,
    },
    SchedulerInactive,
    MissingTemporaryOpeningAccount,
}

impl OpeningInvoiceCreationTool {
    pub fn validate_company(&self) -> Result<(), OpeningInvoiceCreationError> {
        if self.company.as_deref().unwrap_or_default().is_empty() {
            return Err(OpeningInvoiceCreationError::MissingCompany);
        }
        Ok(())
    }

    pub fn set_missing_values(
        &self,
        row: &mut OpeningInvoiceRow,
        temporary_opening_account: Option<&str>,
        today: &str,
    ) {
        if row.qty == 0.0 {
            row.qty = 1.0;
        }
        if row.temporary_opening_account.is_none() {
            row.temporary_opening_account = temporary_opening_account.map(str::to_string);
        }
        row.party_type = Some(
            match self.invoice_type {
                InvoiceType::Sales => "Customer",
                InvoiceType::Purchase => "Supplier",
            }
            .to_string(),
        );
        if row.item_name.is_none() {
            row.item_name = Some("Opening Invoice Item".to_string());
        }
        if row.posting_date.is_none() {
            row.posting_date = Some(today.to_string());
        }
        if row.due_date.is_none() {
            row.due_date = Some(today.to_string());
        }
    }

    pub fn validate_mandatory_invoice_fields(
        &self,
        row: &mut OpeningInvoiceRow,
        existing_parties: &BTreeSet<(String, String)>,
        supplier_group: Option<&str>,
    ) -> Result<Option<PartyCreationPlan>, OpeningInvoiceCreationError> {
        let party_type = row.party_type.clone().unwrap_or_else(|| {
            match self.invoice_type {
                InvoiceType::Sales => "Customer",
                InvoiceType::Purchase => "Supplier",
            }
            .to_string()
        });

        let mut party_plan = None;
        if self.create_missing_party {
            if row.party.as_deref().unwrap_or_default().is_empty()
                && row.party_name.as_deref().unwrap_or_default().is_empty()
            {
                return Err(OpeningInvoiceCreationError::MissingPartyIdOrName { idx: row.idx });
            }

            if row.party.as_deref().unwrap_or_default().is_empty() {
                let party_name = row.party_name.clone().unwrap_or_default();
                party_plan = Some(add_party_plan(&party_type, &party_name, supplier_group)?);
                row.party = Some(party_name);
            } else if !existing_parties.contains(&(party_type.clone(), row.party.clone().unwrap()))
            {
                let party = row.party.clone().unwrap();
                party_plan = Some(add_party_plan(&party_type, &party, supplier_group)?);
                row.party = Some(party);
            }
        } else {
            if row.party.as_deref().unwrap_or_default().is_empty() {
                return Err(OpeningInvoiceCreationError::MissingPartyId { idx: row.idx });
            }
            let party = row.party.clone().unwrap();
            if !existing_parties.contains(&(party_type.clone(), party.clone())) {
                return Err(OpeningInvoiceCreationError::PartyDoesNotExist {
                    idx: row.idx,
                    party_type,
                    party,
                });
            }
        }

        if row.outstanding_amount == 0.0 {
            return Err(OpeningInvoiceCreationError::MissingOutstandingAmount {
                idx: row.idx,
                invoice_type: self.invoice_type.clone(),
            });
        }
        if row
            .temporary_opening_account
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            return Err(
                OpeningInvoiceCreationError::MissingTemporaryOpeningAccountForRow {
                    idx: row.idx,
                    invoice_type: self.invoice_type.clone(),
                },
            );
        }

        Ok(party_plan)
    }

    pub fn get_invoices(
        &self,
        temporary_opening_account: Option<&str>,
        today: &str,
        existing_parties: &BTreeSet<(String, String)>,
        party_defaults: &BTreeMap<(String, String), PartyDefaults>,
        company_details: &CompanyDetails,
        default_uom: &str,
        accounting_dimensions: &[String],
        supplier_group: Option<&str>,
    ) -> Result<Vec<OpeningInvoiceDoc>, OpeningInvoiceCreationError> {
        let mut invoices = Vec::new();
        for source_row in &self.invoices {
            let mut row = source_row.clone();
            self.set_missing_values(&mut row, temporary_opening_account, today);
            self.validate_mandatory_invoice_fields(&mut row, existing_parties, supplier_group)?;
            let mut invoice = self.get_invoice_dict(
                &mut row,
                company_details,
                default_uom,
                accounting_dimensions,
            )?;

            if company_details.default_currency.is_some()
                || company_details.default_letter_head.is_some()
            {
                let party_key = (
                    row.party_type.clone().unwrap_or_default(),
                    row.party.clone().unwrap_or_default(),
                );
                invoice.currency = party_defaults
                    .get(&party_key)
                    .and_then(|defaults| defaults.default_currency.clone())
                    .or_else(|| company_details.default_currency.clone());
                invoice.letter_head = company_details.default_letter_head.clone();
            }
            invoices.push(invoice);
        }
        Ok(invoices)
    }

    pub fn get_invoice_dict(
        &self,
        row: &mut OpeningInvoiceRow,
        company_details: &CompanyDetails,
        default_uom: &str,
        accounting_dimensions: &[String],
    ) -> Result<OpeningInvoiceDoc, OpeningInvoiceCreationError> {
        let item = self.get_item_dict(row, company_details, default_uom, accounting_dimensions)?;
        let party_type = row.party_type.clone().unwrap_or_else(|| {
            match self.invoice_type {
                InvoiceType::Sales => "Customer",
                InvoiceType::Purchase => "Supplier",
            }
            .to_string()
        });
        let party_field = scrub(&party_type);
        let mut dimensions = BTreeMap::new();
        for dimension in accounting_dimensions {
            if let Some(value) = self
                .dimensions
                .get(dimension)
                .cloned()
                .or_else(|| item.dimensions.get(dimension).cloned())
            {
                dimensions.insert(dimension.clone(), value);
            }
        }

        Ok(OpeningInvoiceDoc {
            items: vec![item],
            is_opening: "Yes".to_string(),
            set_posting_time: true,
            company: self.company.clone().unwrap_or_default(),
            cost_center: self.cost_center.clone(),
            due_date: row.due_date.clone(),
            posting_date: row.posting_date.clone(),
            party_field,
            party: row.party.clone().unwrap_or_default(),
            is_pos: false,
            doctype: match self.invoice_type {
                InvoiceType::Sales => "Sales Invoice",
                InvoiceType::Purchase => "Purchase Invoice",
            }
            .to_string(),
            update_stock: false,
            invoice_number: row.invoice_number.clone(),
            disable_rounded_total: true,
            bill_date: (self.invoice_type == InvoiceType::Purchase)
                .then(|| row.supplier_invoice_date.clone())
                .flatten(),
            dimensions,
            ..Default::default()
        })
    }

    pub fn make_invoices_plan(
        &self,
        invoice_count: usize,
        scheduler_inactive: bool,
        in_test: bool,
        job_enqueued: bool,
    ) -> Result<MakeInvoicesPlan, OpeningInvoiceCreationError> {
        self.validate_company()?;
        if invoice_count < 50 {
            return Ok(MakeInvoicesPlan {
                mode: "sync".to_string(),
                ..Default::default()
            });
        }
        if scheduler_inactive && !in_test {
            return Err(OpeningInvoiceCreationError::SchedulerInactive);
        }
        if job_enqueued {
            return Ok(MakeInvoicesPlan {
                mode: "noop".to_string(),
                ..Default::default()
            });
        }
        let job_id = format!("opening_invoice::{}", self.name);
        Ok(MakeInvoicesPlan {
            mode: "enqueue".to_string(),
            job_id: Some(job_id),
            enqueue: true,
            queue: Some("default".to_string()),
            timeout: Some(6000),
            event: Some("opening_invoice_creation".to_string()),
            run_now: in_test,
        })
    }

    fn get_item_dict(
        &self,
        row: &OpeningInvoiceRow,
        company_details: &CompanyDetails,
        default_uom: &str,
        accounting_dimensions: &[String],
    ) -> Result<OpeningInvoiceItem, OpeningInvoiceCreationError> {
        let cost_center = row
            .cost_center
            .clone()
            .or_else(|| company_details.cost_center.clone())
            .ok_or_else(|| OpeningInvoiceCreationError::MissingCompanyCostCenter {
                company: self.company.clone().unwrap_or_default(),
            })?;
        let rate = if row.qty == 0.0 {
            0.0
        } else {
            row.outstanding_amount / row.qty
        };
        let item_name = row
            .item_name
            .clone()
            .unwrap_or_else(|| "Opening Invoice Item".to_string());
        let temporary_opening_account = row.temporary_opening_account.clone().unwrap_or_default();
        let mut item = OpeningInvoiceItem {
            uom: default_uom.to_string(),
            rate,
            qty: row.qty,
            conversion_factor: 1.0,
            item_name: item_name.clone(),
            description: item_name,
            cost_center,
            ..Default::default()
        };
        match row.party_type.as_deref() {
            Some("Supplier") => item.expense_account = Some(temporary_opening_account),
            _ => item.income_account = Some(temporary_opening_account),
        }
        for dimension in accounting_dimensions {
            if let Some(value) = row.dimensions.get(dimension) {
                item.dimensions.insert(dimension.clone(), value.clone());
            }
        }
        Ok(item)
    }
}

pub fn add_party_plan(
    party_type: &str,
    party: &str,
    supplier_group: Option<&str>,
) -> Result<PartyCreationPlan, OpeningInvoiceCreationError> {
    let supplier_group = if party_type == "Supplier" {
        Some(
            supplier_group
                .ok_or(OpeningInvoiceCreationError::MissingSupplierGroup)?
                .to_string(),
        )
    } else {
        None
    };
    Ok(PartyCreationPlan {
        party_type: party_type.to_string(),
        party_name: party.to_string(),
        supplier_group,
        ignore_mandatory: true,
        ignore_permissions: true,
    })
}

pub fn start_import_plan(attempts: &[ImportAttempt]) -> ImportPlanResult {
    let mut result = ImportPlanResult::default();
    for attempt in attempts {
        if attempt.succeeds {
            result.names.push(
                attempt
                    .invoice_number
                    .clone()
                    .unwrap_or_else(|| attempt.generated_name.clone()),
            );
        } else {
            result.errors += 1;
        }
    }
    result
}

pub fn publish_payload(index: usize, total: usize, doctype: &str, user: &str) -> PublishPayload {
    PublishPayload {
        title: "Opening Invoice Creation In Progress".to_string(),
        message: format!("Creating {} out of {} {}", index + 1, total, doctype),
        count: index + 1,
        total,
        user: user.to_string(),
    }
}

pub fn get_opening_invoice_summary(
    companies: &[CompanyCurrency],
    sales_invoices: &[OpeningInvoiceAggregate],
    purchase_invoices: &[OpeningInvoiceAggregate],
) -> (
    Option<BTreeMap<String, CompanyOpeningInvoiceSummary>>,
    Option<BTreeMap<String, OpeningInvoiceMaxCount>>,
) {
    if companies.is_empty() {
        return (None, None);
    }

    let company_wise_currency = companies
        .iter()
        .map(|company| (company.company.clone(), company.currency.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut summary = BTreeMap::new();
    let mut max_count = BTreeMap::new();

    prepare_invoice_summary(
        "Sales Invoice",
        sales_invoices,
        &company_wise_currency,
        &mut summary,
        &mut max_count,
    );
    prepare_invoice_summary(
        "Purchase Invoice",
        purchase_invoices,
        &company_wise_currency,
        &mut summary,
        &mut max_count,
    );

    let escaped_summary = summary
        .into_iter()
        .map(|(company, value)| (escape_html(&company), value))
        .collect::<BTreeMap<_, _>>();
    (Some(escaped_summary), Some(max_count))
}

pub fn get_temporary_opening_account(
    company: Option<&str>,
    temporary_accounts: &[String],
) -> Result<Option<String>, OpeningInvoiceCreationError> {
    if company.unwrap_or_default().is_empty() {
        return Ok(None);
    }
    temporary_accounts
        .first()
        .cloned()
        .map(Some)
        .ok_or(OpeningInvoiceCreationError::MissingTemporaryOpeningAccount)
}

fn prepare_invoice_summary(
    doctype: &str,
    invoices: &[OpeningInvoiceAggregate],
    company_wise_currency: &BTreeMap<String, String>,
    summary: &mut BTreeMap<String, CompanyOpeningInvoiceSummary>,
    max_count: &mut BTreeMap<String, OpeningInvoiceMaxCount>,
) {
    let mut paid_amounts = Vec::new();
    let mut outstanding_amounts = Vec::new();

    for invoice in invoices {
        let company = invoice.company.clone();
        let entry = summary.entry(company.clone()).or_default();
        entry.currency = company_wise_currency.get(&company).cloned();
        match doctype {
            "Sales Invoice" => entry.sales_invoice = Some(invoice.clone()),
            "Purchase Invoice" => entry.purchase_invoice = Some(invoice.clone()),
            _ => {}
        }
        if invoice.paid_amount != 0.0 {
            paid_amounts.push(invoice.paid_amount);
        }
        if invoice.outstanding_amount != 0.0 {
            outstanding_amounts.push(invoice.outstanding_amount);
        }
    }

    if !paid_amounts.is_empty() || !outstanding_amounts.is_empty() {
        max_count.insert(
            doctype.to_string(),
            OpeningInvoiceMaxCount {
                max_paid: paid_amounts.into_iter().fold(0.0, f64::max),
                max_due: outstanding_amounts.into_iter().fold(0.0, f64::max),
            },
        );
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn scrub(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(' ', "_")
}
