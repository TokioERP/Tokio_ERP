use crate::erpnext::accounts::report::non_billed_report::{
    get_ordered_to_be_billed_data, NonBilledArgs, NonBilledDocument, NonBilledFilters,
    NonBilledItem, NonBilledItemMaster, NonBilledRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReceivedItemsToBeBilledReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<NonBilledRow>,
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

pub fn execute(
    filters: &NonBilledFilters,
    documents: &[NonBilledDocument],
    children: &[NonBilledItem],
    item_masters: &[NonBilledItemMaster],
    precision: Option<u32>,
) -> ReceivedItemsToBeBilledReport {
    let columns = get_column();
    let args = get_args();
    let rows =
        get_ordered_to_be_billed_data(&args, filters, documents, children, item_masters, precision);

    ReceivedItemsToBeBilledReport { columns, rows }
}

pub fn get_column() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Purchase Receipt", "name", "Purchase Receipt", 160),
        ReportColumn::date("Date", "date", 100),
        ReportColumn::link("Supplier", "supplier", "Supplier", 120),
        ReportColumn::data("Supplier Name", "supplier_name", 120),
        ReportColumn::link("Item Code", "item_code", "Item", 120),
        ReportColumn::currency("Amount", "amount", "Company:company:default_currency", 100),
        ReportColumn::currency(
            "Billed Amount",
            "billed_amount",
            "Company:company:default_currency",
            100,
        ),
        ReportColumn::currency(
            "Returned Amount",
            "returned_amount",
            "Company:company:default_currency",
            120,
        ),
        ReportColumn::currency(
            "Pending Amount",
            "pending_amount",
            "Company:company:default_currency",
            120,
        ),
        ReportColumn::data("Item Name", "item_name", 120),
        ReportColumn::data("Description", "description", 120),
        ReportColumn::link("Project", "project", "Project", 120),
    ]
}

pub fn get_args() -> NonBilledArgs {
    NonBilledArgs {
        doctype: "Purchase Receipt".to_string(),
        party: "supplier".to_string(),
        date: "posting_date".to_string(),
        order: "name".to_string(),
        order_by: "desc".to_string(),
        reference_field: "purchase_receipt".to_string(),
    }
}
