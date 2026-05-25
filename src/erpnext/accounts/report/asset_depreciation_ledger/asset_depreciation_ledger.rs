#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepreciationColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FinanceBookSelection {
    Selected(String),
    EmptyOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetDepreciationFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub asset: Option<String>,
    pub asset_category: Option<String>,
    pub include_default_book_assets: bool,
    pub finance_book: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssetDepreciationInput {
    pub filters: AssetDepreciationFilters,
    pub depreciation_accounts: Vec<String>,
    pub assets_for_category: Vec<String>,
    pub company_default_finance_book: Option<String>,
    pub gl_entries: Vec<GlEntry>,
    pub assets: Vec<AssetDetail>,
    pub schedule_amounts: Vec<DepreciationScheduleAmount>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetDepreciationFilterPlan {
    pub filters: Vec<String>,
    pub or_filters: Vec<String>,
    pub finance_book: FinanceBookSelection,
    pub asset_category_query: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssetDepreciationReport {
    pub columns: Vec<DepreciationColumn>,
    pub rows: Vec<AssetDepreciationRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub against_voucher: String,
    pub debit: f64,
    pub voucher_no: String,
    pub posting_date: String,
    pub finance_book: Option<String>,
    pub company: String,
    pub against_voucher_type: String,
    pub account: String,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssetDetail {
    pub asset: String,
    pub asset_name: String,
    pub net_purchase_amount: f64,
    pub opening_accumulated_depreciation: f64,
    pub accumulated_depreciation_amount: Option<f64>,
    pub asset_category: String,
    pub status: String,
    pub depreciation_method: String,
    pub purchase_date: String,
    pub cost_center: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DepreciationScheduleAmount {
    pub asset: String,
    pub schedule_date: String,
    pub accumulated_depreciation_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssetDepreciationRow {
    pub asset: String,
    pub asset_name: String,
    pub depreciation_date: String,
    pub net_purchase_amount: f64,
    pub opening_accumulated_depreciation: f64,
    pub depreciation_amount: f64,
    pub accumulated_depreciation_amount: f64,
    pub value_after_depreciation: f64,
    pub depreciation_entry: String,
    pub asset_category: String,
    pub cost_center: String,
    pub status: String,
    pub depreciation_method: String,
    pub purchase_date: String,
}

impl Default for AssetDepreciationFilters {
    fn default() -> Self {
        Self {
            company: String::new(),
            from_date: String::new(),
            to_date: String::new(),
            asset: None,
            asset_category: None,
            include_default_book_assets: false,
            finance_book: None,
        }
    }
}

impl Default for AssetDepreciationInput {
    fn default() -> Self {
        Self {
            filters: AssetDepreciationFilters::default(),
            depreciation_accounts: Vec::new(),
            assets_for_category: Vec::new(),
            company_default_finance_book: None,
            gl_entries: Vec::new(),
            assets: Vec::new(),
            schedule_amounts: Vec::new(),
        }
    }
}

impl DepreciationColumn {
    pub fn new(
        label: &'static str,
        fieldname: &'static str,
        fieldtype: &'static str,
        options: Option<&'static str>,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype,
            options,
            width,
        }
    }
}

impl GlEntry {
    pub fn new(
        against_voucher: impl Into<String>,
        debit: f64,
        voucher_no: impl Into<String>,
        posting_date: impl Into<String>,
        finance_book: impl Into<String>,
    ) -> Self {
        let finance_book = finance_book.into();
        Self {
            against_voucher: against_voucher.into(),
            debit,
            voucher_no: voucher_no.into(),
            posting_date: posting_date.into(),
            finance_book: (!finance_book.is_empty()).then_some(finance_book),
            company: "_Test Company".to_string(),
            against_voucher_type: "Asset".to_string(),
            account: "Depreciation - TC".to_string(),
            is_cancelled: false,
        }
    }
}

impl AssetDetail {
    pub fn new(
        asset: impl Into<String>,
        asset_name: impl Into<String>,
        net_purchase_amount: f64,
        opening_accumulated_depreciation: f64,
        asset_category: impl Into<String>,
    ) -> Self {
        Self {
            asset: asset.into(),
            asset_name: asset_name.into(),
            net_purchase_amount,
            opening_accumulated_depreciation,
            accumulated_depreciation_amount: None,
            asset_category: asset_category.into(),
            status: String::new(),
            depreciation_method: String::new(),
            purchase_date: String::new(),
            cost_center: String::new(),
        }
    }
}

impl DepreciationScheduleAmount {
    pub fn new(
        asset: impl Into<String>,
        schedule_date: impl Into<String>,
        accumulated_depreciation_amount: f64,
    ) -> Self {
        Self {
            asset: asset.into(),
            schedule_date: schedule_date.into(),
            accumulated_depreciation_amount,
        }
    }
}

impl AssetDepreciationRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        asset: impl Into<String>,
        asset_name: impl Into<String>,
        depreciation_date: impl Into<String>,
        net_purchase_amount: f64,
        opening_accumulated_depreciation: f64,
        depreciation_amount: f64,
        accumulated_depreciation_amount: f64,
        value_after_depreciation: f64,
        depreciation_entry: impl Into<String>,
        asset_category: impl Into<String>,
    ) -> Self {
        Self {
            asset: asset.into(),
            asset_name: asset_name.into(),
            depreciation_date: depreciation_date.into(),
            net_purchase_amount,
            opening_accumulated_depreciation,
            depreciation_amount,
            accumulated_depreciation_amount,
            value_after_depreciation,
            depreciation_entry: depreciation_entry.into(),
            asset_category: asset_category.into(),
            cost_center: String::new(),
            status: String::new(),
            depreciation_method: String::new(),
            purchase_date: String::new(),
        }
    }
}

pub fn execute(input: AssetDepreciationInput) -> Result<AssetDepreciationReport, String> {
    let plan = get_filter_plan(
        &input.filters,
        input.depreciation_accounts.clone(),
        input.assets_for_category.clone(),
        input.company_default_finance_book.clone(),
    )?;
    let gl_entries = filtered_gl_entries(
        input.gl_entries,
        &input.filters,
        &plan,
        &input.depreciation_accounts,
        &input.assets_for_category,
    );
    let rows = build_rows(gl_entries, input.assets, input.schedule_amounts);

    Ok(AssetDepreciationReport {
        columns: get_columns(),
        rows,
    })
}

pub fn get_filter_plan(
    filters: &AssetDepreciationFilters,
    depreciation_accounts: Vec<String>,
    assets_for_category: Vec<String>,
    company_default_finance_book: Option<String>,
) -> Result<AssetDepreciationFilterPlan, String> {
    let mut filters_data = vec![
        format!("company = {}", filters.company),
        format!("posting_date >= {}", filters.from_date),
        format!("posting_date <= {}", filters.to_date),
        "against_voucher_type = Asset".to_string(),
        format!("account in {}", format_list(&depreciation_accounts)),
        "is_cancelled = 0".to_string(),
    ];

    if let Some(asset) = &filters.asset {
        filters_data.push(format!("against_voucher = {asset}"));
    }

    let asset_category_query = filters.asset_category.as_ref().map(|asset_category| {
        filters_data.push(format!(
            "against_voucher in {}",
            format_list(&assets_for_category)
        ));
        format!("asset_category = {asset_category} and docstatus = 1")
    });

    let finance_book = select_finance_book(filters, company_default_finance_book.as_deref())?;
    let or_filters = match &finance_book {
        FinanceBookSelection::Selected(book) => vec![
            format!("finance_book in ['', '{book}']"),
            "finance_book is not set".to_string(),
        ],
        FinanceBookSelection::EmptyOnly => vec![
            "finance_book in ['']".to_string(),
            "finance_book is not set".to_string(),
        ],
    };

    Ok(AssetDepreciationFilterPlan {
        filters: filters_data,
        or_filters,
        finance_book,
        asset_category_query,
    })
}

pub fn get_columns() -> Vec<DepreciationColumn> {
    vec![
        DepreciationColumn::new("Asset", "asset", "Link", Some("Asset"), 120),
        DepreciationColumn::new("Asset Name", "asset_name", "Data", None, 140),
        DepreciationColumn::new("Depreciation Date", "depreciation_date", "Date", None, 120),
        DepreciationColumn::new(
            "Purchase Amount",
            "net_purchase_amount",
            "Currency",
            None,
            120,
        ),
        DepreciationColumn::new(
            "Opening Accumulated Depreciation",
            "opening_accumulated_depreciation",
            "Currency",
            None,
            140,
        ),
        DepreciationColumn::new(
            "Depreciation Amount",
            "depreciation_amount",
            "Currency",
            None,
            140,
        ),
        DepreciationColumn::new(
            "Accumulated Depreciation Amount",
            "accumulated_depreciation_amount",
            "Currency",
            None,
            210,
        ),
        DepreciationColumn::new(
            "Value After Depreciation",
            "value_after_depreciation",
            "Currency",
            None,
            180,
        ),
        DepreciationColumn::new(
            "Depreciation Entry",
            "depreciation_entry",
            "Link",
            Some("Journal Entry"),
            140,
        ),
        DepreciationColumn::new(
            "Asset Category",
            "asset_category",
            "Link",
            Some("Asset Category"),
            120,
        ),
        DepreciationColumn::new(
            "Cost Center",
            "cost_center",
            "Link",
            Some("Cost Center"),
            100,
        ),
        DepreciationColumn::new("Current Status", "status", "Data", None, 120),
        DepreciationColumn::new("Purchase Date", "purchase_date", "Date", None, 120),
    ]
}

fn select_finance_book(
    filters: &AssetDepreciationFilters,
    company_default_finance_book: Option<&str>,
) -> Result<FinanceBookSelection, String> {
    if filters.include_default_book_assets {
        if let Some(company_book) = company_default_finance_book {
            if let Some(finance_book) = &filters.finance_book {
                if finance_book != company_book {
                    return Err(
                        "To use a different finance book, please uncheck 'Include Default FB Assets'"
                            .to_string(),
                    );
                }
            }
            Ok(FinanceBookSelection::Selected(company_book.to_string()))
        } else if let Some(finance_book) = &filters.finance_book {
            Ok(FinanceBookSelection::Selected(finance_book.clone()))
        } else {
            Ok(FinanceBookSelection::EmptyOnly)
        }
    } else if let Some(finance_book) = &filters.finance_book {
        Ok(FinanceBookSelection::Selected(finance_book.clone()))
    } else {
        Ok(FinanceBookSelection::EmptyOnly)
    }
}

fn filtered_gl_entries(
    entries: Vec<GlEntry>,
    filters: &AssetDepreciationFilters,
    plan: &AssetDepreciationFilterPlan,
    depreciation_accounts: &[String],
    assets_for_category: &[String],
) -> Vec<GlEntry> {
    let mut rows = entries
        .into_iter()
        .filter(|entry| entry.company == filters.company)
        .filter(|entry| entry.posting_date >= filters.from_date)
        .filter(|entry| entry.posting_date <= filters.to_date)
        .filter(|entry| entry.against_voucher_type == "Asset")
        .filter(|entry| depreciation_accounts.contains(&entry.account))
        .filter(|entry| !entry.is_cancelled)
        .filter(|entry| {
            filters
                .asset
                .as_ref()
                .is_none_or(|asset| entry.against_voucher == *asset)
        })
        .filter(|entry| {
            filters
                .asset_category
                .as_ref()
                .is_none_or(|_| assets_for_category.contains(&entry.against_voucher))
        })
        .filter(|entry| finance_book_matches(entry, &plan.finance_book))
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        left.against_voucher
            .cmp(&right.against_voucher)
            .then(left.posting_date.cmp(&right.posting_date))
    });
    rows
}

fn finance_book_matches(entry: &GlEntry, finance_book: &FinanceBookSelection) -> bool {
    match finance_book {
        FinanceBookSelection::Selected(book) => {
            entry.finance_book.as_deref().is_none_or(str::is_empty)
                || entry.finance_book.as_deref() == Some(book.as_str())
        }
        FinanceBookSelection::EmptyOnly => entry.finance_book.as_deref().is_none_or(str::is_empty),
    }
}

fn build_rows(
    gl_entries: Vec<GlEntry>,
    assets: Vec<AssetDetail>,
    schedule_amounts: Vec<DepreciationScheduleAmount>,
) -> Vec<AssetDepreciationRow> {
    let mut asset_details = assets;
    let mut rows = Vec::new();

    for entry in gl_entries {
        let Some(asset_data) = asset_details
            .iter_mut()
            .find(|asset| asset.asset == entry.against_voucher)
        else {
            continue;
        };

        let accumulated = if let Some(existing) = asset_data
            .accumulated_depreciation_amount
            .filter(|amount| *amount != 0.0)
        {
            existing + entry.debit
        } else {
            schedule_amounts
                .iter()
                .find(|schedule| {
                    schedule.asset == entry.against_voucher
                        && schedule.schedule_date == entry.posting_date
                })
                .map(|schedule| schedule.accumulated_depreciation_amount)
                .unwrap_or(0.0)
        };

        asset_data.accumulated_depreciation_amount = Some(accumulated);
        asset_data.opening_accumulated_depreciation = accumulated - entry.debit;

        rows.push(AssetDepreciationRow {
            asset: asset_data.asset.clone(),
            asset_name: asset_data.asset_name.clone(),
            depreciation_date: entry.posting_date,
            net_purchase_amount: asset_data.net_purchase_amount,
            opening_accumulated_depreciation: asset_data.opening_accumulated_depreciation,
            depreciation_amount: entry.debit,
            accumulated_depreciation_amount: accumulated,
            value_after_depreciation: asset_data.net_purchase_amount - accumulated,
            depreciation_entry: entry.voucher_no,
            asset_category: asset_data.asset_category.clone(),
            cost_center: asset_data.cost_center.clone(),
            status: asset_data.status.clone(),
            depreciation_method: asset_data.depreciation_method.clone(),
            purchase_date: asset_data.purchase_date.clone(),
        });
    }

    rows
}

fn format_list(values: &[String]) -> String {
    let joined = values
        .iter()
        .map(|value| format!("'{value}'"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{joined}]")
}
