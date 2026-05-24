#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShareLedgerError {
    MissingDate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareLedgerFilters {
    pub date: Option<String>,
    pub shareholder: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareTransfer {
    pub name: String,
    pub date: String,
    pub transfer_type: String,
    pub share_type: String,
    pub no_of_shares: f64,
    pub rate: f64,
    pub amount: f64,
    pub company: String,
    pub from_shareholder: String,
    pub to_shareholder: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareLedgerRow {
    pub shareholder: String,
    pub date: String,
    pub transfer_type: String,
    pub share_type: String,
    pub no_of_shares: f64,
    pub rate: f64,
    pub amount: f64,
    pub company: String,
    pub share_transfer: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareLedgerReport {
    pub columns: Vec<&'static str>,
    pub rows: Vec<ShareLedgerRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareTransfersQueryPlan {
    pub date_filter: &'static str,
    pub shareholder_filter: &'static str,
    pub docstatus_filter: &'static str,
    pub order_by: &'static str,
    pub company_filter: &'static str,
}

impl ShareLedgerFilters {
    pub fn new(date: Option<&str>, shareholder: Option<&str>) -> Self {
        Self {
            date: date.map(str::to_string),
            shareholder: shareholder.map(str::to_string),
        }
    }
}

impl ShareTransfer {
    pub fn new(
        name: impl Into<String>,
        date: impl Into<String>,
        transfer_type: impl Into<String>,
        share_type: impl Into<String>,
        no_of_shares: f64,
        rate: f64,
        amount: f64,
        company: impl Into<String>,
        from_shareholder: impl Into<String>,
        to_shareholder: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            date: date.into(),
            transfer_type: transfer_type.into(),
            share_type: share_type.into(),
            no_of_shares,
            rate,
            amount,
            company: company.into(),
            from_shareholder: from_shareholder.into(),
            to_shareholder: to_shareholder.into(),
        }
    }
}

pub fn execute(
    filters: ShareLedgerFilters,
    transfers: Vec<ShareTransfer>,
) -> Result<ShareLedgerReport, ShareLedgerError> {
    if filters.date.is_none() {
        return Err(ShareLedgerError::MissingDate);
    }

    let columns = get_columns();
    let Some(shareholder) = filters.shareholder.as_deref() else {
        return Ok(ShareLedgerReport {
            columns,
            rows: Vec::new(),
        });
    };

    Ok(ShareLedgerReport {
        columns,
        rows: build_rows(shareholder, transfers),
    })
}

pub fn get_columns() -> Vec<&'static str> {
    vec![
        "Shareholder:Link/Shareholder:150",
        "Date:Date:100",
        "Transfer Type::140",
        "Share Type::90",
        "No of Shares::90",
        "Rate:Currency:90",
        "Amount:Currency:90",
        "Company::150",
        "Share Transfer:Link/Share Transfer:90",
    ]
}

pub fn get_all_transfers_query_plan() -> ShareTransfersQueryPlan {
    ShareTransfersQueryPlan {
        date_filter:
            "DATE(date) <= filters.date on both from_shareholder and to_shareholder branches",
        shareholder_filter:
            "from_shareholder = filters.shareholder or to_shareholder = filters.shareholder",
        docstatus_filter: "docstatus = 1",
        order_by: "date",
        company_filter: "unused placeholder condition only",
    }
}

pub fn build_rows(shareholder: &str, transfers: Vec<ShareTransfer>) -> Vec<ShareLedgerRow> {
    transfers
        .into_iter()
        .map(|transfer| {
            let mut transfer_type = transfer.transfer_type.clone();
            if transfer_type == "Transfer" {
                if transfer.from_shareholder == shareholder {
                    transfer_type.push_str(" to ");
                    transfer_type.push_str(&transfer.to_shareholder);
                } else {
                    transfer_type.push_str(" from ");
                    transfer_type.push_str(&transfer.from_shareholder);
                }
            }

            ShareLedgerRow {
                shareholder: shareholder.to_string(),
                date: transfer.date,
                transfer_type,
                share_type: transfer.share_type,
                no_of_shares: transfer.no_of_shares,
                rate: transfer.rate,
                amount: transfer.amount,
                company: transfer.company,
                share_transfer: transfer.name,
            }
        })
        .collect()
}
