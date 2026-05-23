use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankGuarantee {
    pub account: Option<String>,
    pub amended_from: Option<String>,
    pub amount: f64,
    pub bank: Option<String>,
    pub bank_account: Option<String>,
    pub bank_account_no: Option<String>,
    pub bank_guarantee_number: Option<String>,
    pub bg_type: Option<String>,
    pub branch_code: Option<String>,
    pub charges: f64,
    pub customer: Option<String>,
    pub end_date: Option<String>,
    pub fixed_deposit_number: Option<String>,
    pub iban: Option<String>,
    pub margin_money: f64,
    pub more_information: Option<String>,
    pub name_of_beneficiary: Option<String>,
    pub project: Option<String>,
    pub reference_docname: Option<String>,
    pub reference_doctype: Option<String>,
    pub start_date: Option<String>,
    pub supplier: Option<String>,
    pub swift_number: Option<String>,
    pub validity: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankGuaranteeError {
    CustomerOrSupplierRequired,
    BankGuaranteeNumberRequired,
    BeneficiaryRequired,
    BankRequired,
    ReferenceNameMustBeString,
    InvalidDate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoucherDetailsPlan {
    pub doctype: &'static str,
    pub reference_name: String,
    pub fields_to_fetch: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankGuaranteeClientConfig {
    pub add_fetches: [(&'static str, &'static str, &'static str); 5],
    pub bank_account_query_filters: [&'static str; 2],
    pub project_query_filter: &'static str,
}

impl BankGuarantee {
    pub const DOCTYPE: &'static str = "Bank Guarantee";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-BG-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 30] = [
        "bg_type",
        "reference_doctype",
        "reference_docname",
        "customer",
        "supplier",
        "project",
        "column_break_6",
        "amount",
        "start_date",
        "validity",
        "end_date",
        "bank_account_info",
        "bank",
        "bank_account",
        "account",
        "bank_account_no",
        "column_break_17",
        "iban",
        "branch_code",
        "swift_number",
        "section_break_14",
        "more_information",
        "margin_details",
        "bank_guarantee_number",
        "name_of_beneficiary",
        "column_break_19",
        "margin_money",
        "charges",
        "fixed_deposit_number",
        "amended_from",
    ];
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const EDITABLE_GRID: bool = true;
    pub const GRID_PAGE_LENGTH: u16 = 50;
    pub const IS_SUBMITTABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SEARCH_FIELDS: &'static str = "customer";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "customer";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("bg_type", "Bank Guarantee Type")
                .options("\nReceiving\nProviding")
                .required(),
            FieldSpec::link("reference_doctype", "Reference Document Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("reference_docname")
                .label("Reference Document Name")
                .options("reference_doctype"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .depends_on("eval: doc.bg_type == \"Receiving\""),
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .depends_on("eval: doc.bg_type == \"Providing\""),
            FieldSpec::link("project", "Project")
                .options("Project")
                .allow_on_submit(),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::currency("amount", "Amount")
                .in_list_view()
                .required(),
            FieldSpec::date("start_date", "Start Date").required(),
            FieldSpec::int("validity", "Validity in Days"),
            FieldSpec::date("end_date", "End Date").read_only(),
            FieldSpec::section_break("bank_account_info").label("Bank Account Info"),
            FieldSpec::link("bank", "Bank").options("Bank"),
            FieldSpec::link("bank_account", "Bank Account").options("Bank Account"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .read_only(),
            FieldSpec::data("bank_account_no", "Bank Account No").read_only(),
            FieldSpec::column_break("column_break_17"),
            FieldSpec::data("iban", "IBAN").options("IBAN").read_only(),
            FieldSpec::data("branch_code", "Branch Code").read_only(),
            FieldSpec::data("swift_number", "SWIFT number").read_only(),
            FieldSpec::section_break("section_break_14"),
            FieldSpec::text_editor("more_information", "Clauses and Conditions"),
            FieldSpec::section_break("margin_details").label("Other Details"),
            FieldSpec::data("bank_guarantee_number", "Bank Guarantee Number").unique(),
            FieldSpec::data("name_of_beneficiary", "Name of Beneficiary"),
            FieldSpec::column_break("column_break_19"),
            FieldSpec::currency("margin_money", "Margin Money"),
            FieldSpec::currency("charges", "Charges Incurred"),
            FieldSpec::data("fixed_deposit_number", "Fixed Deposit Number"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Bank Guarantee")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(&self) -> Result<(), BankGuaranteeError> {
        if !(self.customer.is_some() || self.supplier.is_some()) {
            return Err(BankGuaranteeError::CustomerOrSupplierRequired);
        }

        Ok(())
    }

    pub fn on_submit(&self) -> Result<(), BankGuaranteeError> {
        if self.bank_guarantee_number.is_none() {
            return Err(BankGuaranteeError::BankGuaranteeNumberRequired);
        }

        if self.name_of_beneficiary.is_none() {
            return Err(BankGuaranteeError::BeneficiaryRequired);
        }

        if self.bank.is_none() {
            return Err(BankGuaranteeError::BankRequired);
        }

        Ok(())
    }
}

impl DocumentController for BankGuarantee {
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

impl BankGuaranteeClientConfig {
    pub fn from_erpnext_js() -> Self {
        Self {
            add_fetches: [
                ("bank_account", "account", "account"),
                ("bank_account", "bank_account_no", "bank_account_no"),
                ("bank_account", "iban", "iban"),
                ("bank_account", "branch_code", "branch_code"),
                ("bank", "swift_number", "swift_number"),
            ],
            bank_account_query_filters: ["company", "bank"],
            project_query_filter: "customer",
        }
    }

    pub fn bg_type_reference_doctype(&self, bg_type: &str) -> Option<&'static str> {
        match bg_type {
            "Receiving" => Some("Sales Order"),
            "Providing" => Some("Purchase Order"),
            _ => None,
        }
    }

    pub fn reference_party_field(&self, reference_doctype: &str) -> &'static str {
        if reference_doctype == "Sales Order" {
            "customer"
        } else {
            "supplier"
        }
    }

    pub fn end_date(&self, start_date: &str, validity: i32) -> Result<String, BankGuaranteeError> {
        add_days(start_date, validity - 1)
    }
}

pub fn get_voucher_details_plan(
    bank_guarantee_type: &str,
    reference_name: Option<&str>,
) -> Result<VoucherDetailsPlan, BankGuaranteeError> {
    let reference_name = reference_name.ok_or(BankGuaranteeError::ReferenceNameMustBeString)?;
    let mut fields_to_fetch = vec!["grand_total"];

    let doctype = if bank_guarantee_type == "Receiving" {
        fields_to_fetch.push("customer");
        fields_to_fetch.push("project");
        "Sales Order"
    } else {
        fields_to_fetch.push("supplier");
        "Purchase Order"
    };

    Ok(VoucherDetailsPlan {
        doctype,
        reference_name: reference_name.to_string(),
        fields_to_fetch,
    })
}

fn add_days(date: &str, days: i32) -> Result<String, BankGuaranteeError> {
    let (year, month, day) = parse_date(date)?;
    let serial = days_from_civil(year, month, day) + days;
    let (year, month, day) = civil_from_days(serial);
    Ok(format!("{year:04}-{month:02}-{day:02}"))
}

fn parse_date(date: &str) -> Result<(i32, u32, u32), BankGuaranteeError> {
    let mut parts = date.split('-');
    let year = parts
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(BankGuaranteeError::InvalidDate)?;
    let month = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(BankGuaranteeError::InvalidDate)?;
    let day = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(BankGuaranteeError::InvalidDate)?;

    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(BankGuaranteeError::InvalidDate);
    }

    Ok((year, month, day))
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i32 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i32;
    let day = day as i32;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i32) -> (i32, u32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i32::from(month <= 2);
    (year, month as u32, day as u32)
}
