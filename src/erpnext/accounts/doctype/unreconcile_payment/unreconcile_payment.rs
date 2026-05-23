use crate::erpnext::accounts::doctype::unreconcile_payment_entries::unreconcile_payment_entries::UnreconcilePaymentEntries;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnreconcilePayment {
    pub company: Option<String>,
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub allocations: Vec<UnreconcilePaymentEntries>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnreconcileSelection {
    pub company: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerEntry {
    pub company: String,
    pub delinked: bool,
    pub voucher_type: String,
    pub voucher_no: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
    pub amount: f64,
    pub amount_in_account_currency: f64,
    pub account: String,
    pub party_type: String,
    pub party: String,
    pub account_currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdvancePaymentLedgerEntry {
    pub company: String,
    pub delinked: bool,
    pub voucher_type: String,
    pub voucher_no: String,
    pub event: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnreconcileSubmitAction {
    pub reference_doctype: String,
    pub reference_name: String,
    pub payment_voucher_no: String,
    pub payment_voucher_type: String,
    pub account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub allocation_name: Option<String>,
    pub mark_unlinked: bool,
}

impl UnreconcilePayment {
    pub const DOCTYPE: &'static str = "Unreconcile Payment";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "company",
        "voucher_type",
        "voucher_no",
        "get_allocations",
        "allocations",
        "amended_from",
    ];
    pub const IS_SUBMITTABLE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(
        company: impl Into<String>,
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
    ) -> Self {
        Self {
            company: Some(company.into()),
            voucher_type: Some(voucher_type.into()),
            voucher_no: Some(voucher_no.into()),
            allocations: Vec::new(),
            amended_from: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("amended_from", "Amended From")
                .options("Unreconcile Payment")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type"),
            FieldSpec::button("get_allocations", "Get Allocations"),
            FieldSpec::table("allocations", "Allocations").options("Unreconcile Payment Entries"),
        ]
    }

    pub const fn supported_types() -> [&'static str; 2] {
        ["Payment Entry", "Journal Entry"]
    }

    pub const fn voucher_type_query_filter() -> [&'static str; 2] {
        Self::supported_types()
    }

    pub fn voucher_no_query_filter(company: &str) -> [(&'static str, &str); 2] {
        [("company", company), ("docstatus", "1")]
    }

    pub fn validate(&self) -> Result<(), String> {
        let supported = Self::supported_types();
        if self
            .voucher_type
            .as_deref()
            .map(|voucher_type| supported.contains(&voucher_type))
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err("Only Payment Entry and Journal Entry are supported".to_string())
        }
    }

    pub fn get_allocations_from_payment(
        &self,
        payment_entries: &[PaymentLedgerEntry],
        advances: &[AdvancePaymentLedgerEntry],
    ) -> Vec<UnreconcilePaymentEntries> {
        get_linked_payments_for_doc(
            self.company.as_deref(),
            self.voucher_type.as_deref(),
            self.voucher_no.as_deref(),
            payment_entries,
            advances,
        )
    }

    pub fn add_references(&mut self, allocations: Vec<UnreconcilePaymentEntries>) {
        self.allocations.extend(allocations);
    }

    pub fn on_submit(&self) -> Vec<UnreconcileSubmitAction> {
        let payment_voucher_no = self.voucher_no.clone().unwrap_or_default();
        let payment_voucher_type = self.voucher_type.clone().unwrap_or_default();

        self.allocations
            .iter()
            .map(|allocation| UnreconcileSubmitAction {
                reference_doctype: allocation.reference_doctype.clone().unwrap_or_default(),
                reference_name: allocation.reference_name.clone().unwrap_or_default(),
                payment_voucher_no: payment_voucher_no.clone(),
                payment_voucher_type: payment_voucher_type.clone(),
                account: allocation.account.clone(),
                party_type: allocation.party_type.clone(),
                party: allocation.party.clone(),
                allocation_name: allocation.name.clone(),
                mark_unlinked: true,
            })
            .collect()
    }
}

impl DocumentController for UnreconcilePayment {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit"]
    }
}

pub fn doc_has_references(
    doctype: Option<&str>,
    docname: Option<&str>,
    payment_entries: &[PaymentLedgerEntry],
    advances: &[AdvancePaymentLedgerEntry],
) -> usize {
    let Some(docname) = docname else {
        return 0;
    };

    if matches!(doctype, Some("Sales Invoice" | "Purchase Invoice")) {
        payment_entries
            .iter()
            .filter(|entry| !entry.delinked)
            .filter(|entry| entry.against_voucher_no == docname)
            .filter(|entry| entry.amount < 0.0)
            .count()
    } else {
        let payment_count = payment_entries
            .iter()
            .filter(|entry| !entry.delinked)
            .filter(|entry| entry.voucher_no == docname)
            .filter(|entry| entry.against_voucher_no != docname)
            .count();

        let advance_count = advances
            .iter()
            .filter(|entry| !entry.delinked)
            .filter(|entry| entry.voucher_no == docname)
            .filter(|entry| doctype.map(|dt| entry.voucher_type == dt).unwrap_or(false))
            .filter(|entry| entry.event == "Submit")
            .count();

        payment_count + advance_count
    }
}

pub fn get_linked_payments_for_doc(
    company: Option<&str>,
    doctype: Option<&str>,
    docname: Option<&str>,
    payment_entries: &[PaymentLedgerEntry],
    advances: &[AdvancePaymentLedgerEntry],
) -> Vec<UnreconcilePaymentEntries> {
    let (Some(company), Some(doctype), Some(docname)) = (company, doctype, docname) else {
        return Vec::new();
    };

    if matches!(doctype, "Sales Invoice" | "Purchase Invoice") {
        let filtered = payment_entries
            .iter()
            .filter(|entry| entry.company == company)
            .filter(|entry| !entry.delinked)
            .filter(|entry| entry.against_voucher_no == docname)
            .filter(|entry| entry.amount < 0.0)
            .collect::<Vec<_>>();

        group_payment_entries(filtered, |entry| {
            (entry.voucher_no.as_str(), entry.against_voucher_no.as_str())
        })
        .into_iter()
        .filter(|entry| entry.allocated_amount > 0.0)
        .collect()
    } else {
        let filtered = payment_entries
            .iter()
            .filter(|entry| entry.company == company)
            .filter(|entry| !entry.delinked)
            .filter(|entry| entry.voucher_no == docname)
            .filter(|entry| entry.against_voucher_no != docname)
            .collect::<Vec<_>>();

        let mut rows =
            group_payment_entries(filtered, |entry| (entry.against_voucher_no.as_str(), ""));
        rows.extend(get_linked_advances(company, docname, advances));
        rows
    }
}

pub fn get_linked_advances(
    company: &str,
    docname: &str,
    advances: &[AdvancePaymentLedgerEntry],
) -> Vec<UnreconcilePaymentEntries> {
    let filtered = advances
        .iter()
        .filter(|entry| entry.company == company)
        .filter(|entry| !entry.delinked)
        .filter(|entry| entry.voucher_no == docname)
        .filter(|entry| entry.event == "Submit")
        .collect::<Vec<_>>();

    let mut groups: Vec<(&AdvancePaymentLedgerEntry, f64)> = Vec::new();
    for entry in filtered {
        if let Some((_, total)) = groups
            .iter_mut()
            .find(|(first, _)| first.against_voucher_no == entry.against_voucher_no)
        {
            *total += entry.amount;
        } else {
            groups.push((entry, entry.amount));
        }
    }

    groups
        .into_iter()
        .map(|(first, total)| {
            let mut row = UnreconcilePaymentEntries::new(
                first.against_voucher_type.clone(),
                first.against_voucher_no.clone(),
                total.abs(),
            );
            row.account_currency = Some(first.currency.clone());
            row
        })
        .filter(|entry| entry.allocated_amount > 0.0)
        .collect()
}

pub fn create_unreconcile_docs_for_selection(
    selections: &[UnreconcileSelection],
    payment_entries: &[PaymentLedgerEntry],
    advances: &[AdvancePaymentLedgerEntry],
) -> Vec<UnreconcilePayment> {
    selections
        .iter()
        .map(|selection| {
            let mut unreconcile = UnreconcilePayment::new(
                selection.company.clone(),
                selection.voucher_type.clone(),
                selection.voucher_no.clone(),
            );
            let allocations = unreconcile
                .get_allocations_from_payment(payment_entries, advances)
                .into_iter()
                .filter(|allocation| {
                    allocation.reference_doctype.as_deref()
                        == Some(selection.against_voucher_type.as_str())
                        && allocation.reference_name.as_deref()
                            == Some(selection.against_voucher_no.as_str())
                })
                .collect();
            unreconcile.add_references(allocations);
            unreconcile
        })
        .collect()
}

fn group_payment_entries<'a, F>(
    entries: Vec<&'a PaymentLedgerEntry>,
    key: F,
) -> Vec<UnreconcilePaymentEntries>
where
    F: Fn(&'a PaymentLedgerEntry) -> (&'a str, &'a str),
{
    let mut groups: Vec<(&PaymentLedgerEntry, f64)> = Vec::new();
    for entry in entries {
        let entry_key = key(entry);
        if let Some((_, total)) = groups.iter_mut().find(|(first, _)| key(first) == entry_key) {
            *total += entry.amount_in_account_currency;
        } else {
            groups.push((entry, entry.amount_in_account_currency));
        }
    }

    groups
        .into_iter()
        .map(|(first, total)| {
            let mut row = if matches!(
                first.against_voucher_type.as_str(),
                "Sales Invoice" | "Purchase Invoice"
            ) && first.amount < 0.0
            {
                UnreconcilePaymentEntries::new(
                    first.voucher_type.clone(),
                    first.voucher_no.clone(),
                    total.abs(),
                )
            } else {
                UnreconcilePaymentEntries::new(
                    first.against_voucher_type.clone(),
                    first.against_voucher_no.clone(),
                    total.abs(),
                )
            };
            row.account = Some(first.account.clone());
            row.party_type = Some(first.party_type.clone());
            row.party = Some(first.party.clone());
            row.account_currency = Some(first.account_currency.clone());
            row
        })
        .collect()
}
