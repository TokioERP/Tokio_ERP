use crate::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShareBalanceError {
    MissingDate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareBalanceFilters {
    pub date: Option<String>,
    pub shareholder: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareBalanceRow {
    pub shareholder: String,
    pub share_type: String,
    pub no_of_shares: i32,
    pub rate: f64,
    pub amount: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShareBalanceReport {
    pub columns: Vec<&'static str>,
    pub rows: Vec<ShareBalanceRow>,
}

impl ShareBalanceFilters {
    pub fn new(date: Option<&str>, shareholder: Option<&str>) -> Self {
        Self {
            date: date.map(str::to_string),
            shareholder: shareholder.map(str::to_string),
        }
    }
}

pub fn execute(
    filters: ShareBalanceFilters,
    share_balance: Vec<ShareBalance>,
) -> Result<ShareBalanceReport, ShareBalanceError> {
    if filters.date.is_none() {
        return Err(ShareBalanceError::MissingDate);
    }

    let columns = get_columns();
    let Some(shareholder) = filters.shareholder.as_deref() else {
        return Ok(ShareBalanceReport {
            columns,
            rows: Vec::new(),
        });
    };

    Ok(ShareBalanceReport {
        columns,
        rows: build_rows(shareholder, share_balance),
    })
}

pub fn get_columns() -> Vec<&'static str> {
    vec![
        "Shareholder:Link/Shareholder:150",
        "Share Type::90",
        "No of Shares::90",
        "Average Rate:Currency:90",
        "Amount:Currency:90",
    ]
}

pub fn get_all_shares_query_plan(shareholder: &str) -> String {
    format!(
        "frappe.get_doc(\"Shareholder\", \"{}\").share_balance",
        shareholder
    )
}

pub fn build_rows(shareholder: &str, share_balance: Vec<ShareBalance>) -> Vec<ShareBalanceRow> {
    let mut rows: Vec<ShareBalanceRow> = Vec::new();

    for share_entry in share_balance {
        let share_type = share_entry.share_type.unwrap_or_default();

        if let Some(row) = rows
            .iter_mut()
            .find(|row| row.share_type.as_str() == share_type)
        {
            row.no_of_shares += share_entry.no_of_shares;
            row.amount += share_entry.amount;
            row.rate = if row.no_of_shares == 0 {
                0.0
            } else {
                f64::from(row.amount) / f64::from(row.no_of_shares)
            };
        } else {
            rows.push(ShareBalanceRow {
                shareholder: shareholder.to_string(),
                share_type,
                no_of_shares: share_entry.no_of_shares,
                rate: f64::from(share_entry.rate),
                amount: share_entry.amount,
            });
        }
    }

    rows
}
