use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AssetDepreciationsAndBalancesFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub group_by: String,
    pub asset_category: Option<String>,
    pub asset: Option<String>,
    pub finance_book: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetDepreciationsAndBalancesRow {
    pub asset_category: Option<String>,
    pub asset: Option<String>,
    pub asset_name: Option<String>,
    pub value_as_on_from_date: f64,
    pub value_of_new_purchase: f64,
    pub value_of_sold_asset: f64,
    pub value_of_scrapped_asset: f64,
    pub value_of_capitalized_asset: f64,
    pub adjustment_before_from_date: f64,
    pub adjustment_till_to_date: f64,
    pub adjustment_during_period: f64,
    pub value_as_on_to_date: f64,
    pub accumulated_depreciation_as_on_from_date: f64,
    pub depreciation_amount_during_the_period: f64,
    pub depreciation_eliminated_during_the_period: f64,
    pub accumulated_depreciation_as_on_to_date: f64,
    pub depreciation_eliminated_via_reversal: f64,
    pub net_asset_value_as_on_from_date: f64,
    pub net_asset_value_as_on_to_date: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetValueByCategoryRow {
    pub asset_category: String,
    pub value_as_on_from_date: f64,
    pub value_of_new_purchase: f64,
    pub value_of_sold_asset: f64,
    pub value_of_scrapped_asset: f64,
    pub value_of_capitalized_asset: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetDetailValueRow {
    pub name: String,
    pub asset_name: String,
    pub value_as_on_from_date: f64,
    pub value_of_new_purchase: f64,
    pub value_of_sold_asset: f64,
    pub value_of_scrapped_asset: f64,
    pub value_of_capitalized_asset: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetDepreciationByCategoryRow {
    pub asset_category: String,
    pub accumulated_depreciation_as_on_from_date: f64,
    pub depreciation_eliminated_via_reversal: f64,
    pub depreciation_eliminated_during_the_period: f64,
    pub depreciation_amount_during_the_period: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetDepreciationByAssetRow {
    pub asset: String,
    pub accumulated_depreciation_as_on_from_date: f64,
    pub depreciation_eliminated_via_reversal: f64,
    pub depreciation_eliminated_during_the_period: f64,
    pub depreciation_amount_during_the_period: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetOpeningDepreciationByCategoryRow {
    pub asset_category: String,
    pub accumulated_depreciation_as_on_from_date: f64,
    pub depreciation_eliminated_during_the_period: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetOpeningDepreciationByAssetRow {
    pub asset: String,
    pub accumulated_depreciation_as_on_from_date: f64,
    pub depreciation_eliminated_during_the_period: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssetValueAdjustmentRow {
    pub key: String,
    pub adjustment_before_from_date: f64,
    pub adjustment_till_to_date: f64,
}

impl ReportColumn {
    pub fn link(
        label: impl Into<String>,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Link",
            options,
            width,
        }
    }

    pub fn data(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub fn currency(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Currency",
            options: "",
            width,
        }
    }
}

pub fn combine_category_depreciation_rows(
    gl_rows: &[AssetDepreciationByCategoryRow],
    opening_rows: &[AssetOpeningDepreciationByCategoryRow],
) -> Vec<AssetDepreciationByCategoryRow> {
    let mut combined = gl_rows.to_vec();

    for opening in opening_rows {
        if let Some(row) = combined
            .iter_mut()
            .find(|row| row.asset_category == opening.asset_category)
        {
            row.accumulated_depreciation_as_on_from_date +=
                opening.accumulated_depreciation_as_on_from_date;
            row.depreciation_eliminated_during_the_period +=
                opening.depreciation_eliminated_during_the_period;
        } else {
            combined.push(AssetDepreciationByCategoryRow {
                asset_category: opening.asset_category.clone(),
                accumulated_depreciation_as_on_from_date: opening
                    .accumulated_depreciation_as_on_from_date,
                depreciation_eliminated_via_reversal: 0.0,
                depreciation_eliminated_during_the_period: opening
                    .depreciation_eliminated_during_the_period,
                depreciation_amount_during_the_period: 0.0,
            });
        }
    }

    combined
}

pub fn combine_asset_depreciation_rows(
    gl_rows: &[AssetDepreciationByAssetRow],
    opening_rows: &[AssetOpeningDepreciationByAssetRow],
) -> Vec<AssetDepreciationByAssetRow> {
    let mut combined = gl_rows.to_vec();

    for opening in opening_rows {
        if let Some(row) = combined.iter_mut().find(|row| row.asset == opening.asset) {
            row.accumulated_depreciation_as_on_from_date +=
                opening.accumulated_depreciation_as_on_from_date;
            row.depreciation_eliminated_during_the_period +=
                opening.depreciation_eliminated_during_the_period;
        } else {
            combined.push(AssetDepreciationByAssetRow {
                asset: opening.asset.clone(),
                accumulated_depreciation_as_on_from_date: opening
                    .accumulated_depreciation_as_on_from_date,
                depreciation_eliminated_via_reversal: 0.0,
                depreciation_eliminated_during_the_period: opening
                    .depreciation_eliminated_during_the_period,
                depreciation_amount_during_the_period: 0.0,
            });
        }
    }

    combined
}

pub fn assemble_group_by_asset_category_data(
    asset_categories: &[AssetValueByCategoryRow],
    assets: &[AssetDepreciationByCategoryRow],
    adjustments: &[AssetValueAdjustmentRow],
) -> Vec<AssetDepreciationsAndBalancesRow> {
    let asset_map: BTreeMap<&str, &AssetDepreciationByCategoryRow> = assets
        .iter()
        .map(|asset| (asset.asset_category.as_str(), asset))
        .collect();
    let adjustment_map = adjustment_map(adjustments);

    asset_categories
        .iter()
        .filter_map(|asset_category| {
            let depreciation = asset_map.get(asset_category.asset_category.as_str())?;
            let adjustment = adjustment_map.get(asset_category.asset_category.as_str());

            Some(build_row(
                Some(asset_category.asset_category.clone()),
                None,
                None,
                AssetValueParts {
                    value_as_on_from_date: asset_category.value_as_on_from_date,
                    value_of_new_purchase: asset_category.value_of_new_purchase,
                    value_of_sold_asset: asset_category.value_of_sold_asset,
                    value_of_scrapped_asset: asset_category.value_of_scrapped_asset,
                    value_of_capitalized_asset: asset_category.value_of_capitalized_asset,
                },
                DepreciationParts {
                    accumulated_depreciation_as_on_from_date: depreciation
                        .accumulated_depreciation_as_on_from_date,
                    depreciation_amount_during_the_period: depreciation
                        .depreciation_amount_during_the_period,
                    depreciation_eliminated_during_the_period: depreciation
                        .depreciation_eliminated_during_the_period,
                    depreciation_eliminated_via_reversal: depreciation
                        .depreciation_eliminated_via_reversal,
                },
                adjustment.copied().unwrap_or_default(),
            ))
        })
        .collect()
}

pub fn assemble_group_by_asset_data(
    asset_details: &[AssetDetailValueRow],
    assets: &[AssetDepreciationByAssetRow],
    adjustments: &[AssetValueAdjustmentRow],
) -> Vec<AssetDepreciationsAndBalancesRow> {
    let asset_map: BTreeMap<&str, &AssetDepreciationByAssetRow> = assets
        .iter()
        .map(|asset| (asset.asset.as_str(), asset))
        .collect();
    let adjustment_map = adjustment_map(adjustments);

    asset_details
        .iter()
        .filter_map(|asset_detail| {
            let depreciation = asset_map.get(asset_detail.name.as_str())?;
            let adjustment = adjustment_map.get(asset_detail.name.as_str());

            Some(build_row(
                None,
                Some(asset_detail.name.clone()),
                Some(asset_detail.asset_name.clone()),
                AssetValueParts {
                    value_as_on_from_date: asset_detail.value_as_on_from_date,
                    value_of_new_purchase: asset_detail.value_of_new_purchase,
                    value_of_sold_asset: asset_detail.value_of_sold_asset,
                    value_of_scrapped_asset: asset_detail.value_of_scrapped_asset,
                    value_of_capitalized_asset: asset_detail.value_of_capitalized_asset,
                },
                DepreciationParts {
                    accumulated_depreciation_as_on_from_date: depreciation
                        .accumulated_depreciation_as_on_from_date,
                    depreciation_amount_during_the_period: depreciation
                        .depreciation_amount_during_the_period,
                    depreciation_eliminated_during_the_period: depreciation
                        .depreciation_eliminated_during_the_period,
                    depreciation_eliminated_via_reversal: depreciation
                        .depreciation_eliminated_via_reversal,
                },
                adjustment.copied().unwrap_or_default(),
            ))
        })
        .collect()
}

pub fn get_columns(filters: &AssetDepreciationsAndBalancesFilters) -> Vec<ReportColumn> {
    let mut columns = Vec::new();

    match filters.group_by.as_str() {
        "Asset Category" => columns.push(ReportColumn::link(
            "Asset Category",
            "asset_category",
            "Asset Category",
            120,
        )),
        "Asset" => {
            columns.push(ReportColumn::link("Asset", "asset", "Asset", 120));
            columns.push(ReportColumn::data("Asset Name", "asset_name", 140));
        }
        _ => {}
    }

    let day_before_from_date =
        add_days(&filters.from_date, -1).unwrap_or_else(|| filters.from_date.clone());

    columns.extend([
        ReportColumn::currency(
            format!("Value as on {}", formatdate(&day_before_from_date)),
            "value_as_on_from_date",
            140,
        ),
        ReportColumn::currency("Value of New Purchase", "value_of_new_purchase", 140),
        ReportColumn::currency("Value of Sold Asset", "value_of_sold_asset", 140),
        ReportColumn::currency("Value of Scrapped Asset", "value_of_scrapped_asset", 140),
        ReportColumn::currency(
            "Value of New Capitalized Asset",
            "value_of_capitalized_asset",
            140,
        ),
        ReportColumn::currency(
            format!("Value as on {}", formatdate(&filters.to_date)),
            "value_as_on_to_date",
            140,
        ),
        ReportColumn::currency(
            format!(
                "Accumulated Depreciation as on {}",
                formatdate(&day_before_from_date)
            ),
            "accumulated_depreciation_as_on_from_date",
            270,
        ),
        ReportColumn::currency(
            "Depreciation Amount during the period",
            "depreciation_amount_during_the_period",
            240,
        ),
        ReportColumn::currency(
            "Depreciation Eliminated due to disposal of assets",
            "depreciation_eliminated_during_the_period",
            300,
        ),
        ReportColumn::currency(
            format!(
                "Accumulated Depreciation as on {}",
                formatdate(&filters.to_date)
            ),
            "accumulated_depreciation_as_on_to_date",
            270,
        ),
        ReportColumn::currency(
            "Depreciation eliminated via reversal",
            "depreciation_eliminated_via_reversal",
            270,
        ),
        ReportColumn::currency(
            format!(
                "Net Asset value as on {}",
                formatdate(&day_before_from_date)
            ),
            "net_asset_value_as_on_from_date",
            200,
        ),
        ReportColumn::currency(
            format!("Net Asset value as on {}", formatdate(&filters.to_date)),
            "net_asset_value_as_on_to_date",
            200,
        ),
    ]);

    columns
}

fn formatdate(date: &str) -> String {
    date.to_string()
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct AdjustmentParts {
    adjustment_before_from_date: f64,
    adjustment_till_to_date: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct AssetValueParts {
    value_as_on_from_date: f64,
    value_of_new_purchase: f64,
    value_of_sold_asset: f64,
    value_of_scrapped_asset: f64,
    value_of_capitalized_asset: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct DepreciationParts {
    accumulated_depreciation_as_on_from_date: f64,
    depreciation_amount_during_the_period: f64,
    depreciation_eliminated_during_the_period: f64,
    depreciation_eliminated_via_reversal: f64,
}

fn adjustment_map(adjustments: &[AssetValueAdjustmentRow]) -> BTreeMap<&str, AdjustmentParts> {
    adjustments
        .iter()
        .map(|adjustment| {
            (
                adjustment.key.as_str(),
                AdjustmentParts {
                    adjustment_before_from_date: adjustment.adjustment_before_from_date,
                    adjustment_till_to_date: adjustment.adjustment_till_to_date,
                },
            )
        })
        .collect()
}

fn build_row(
    asset_category: Option<String>,
    asset: Option<String>,
    asset_name: Option<String>,
    value: AssetValueParts,
    depreciation: DepreciationParts,
    adjustment: AdjustmentParts,
) -> AssetDepreciationsAndBalancesRow {
    let adjustment_during_period =
        adjustment.adjustment_till_to_date - adjustment.adjustment_before_from_date;
    let value_as_on_from_date =
        value.value_as_on_from_date + adjustment.adjustment_before_from_date;
    let value_as_on_to_date = value_as_on_from_date + value.value_of_new_purchase
        - value.value_of_sold_asset
        - value.value_of_scrapped_asset
        - value.value_of_capitalized_asset
        + adjustment_during_period;
    let accumulated_depreciation_as_on_to_date = depreciation
        .accumulated_depreciation_as_on_from_date
        + depreciation.depreciation_amount_during_the_period
        - depreciation.depreciation_eliminated_during_the_period
        - depreciation.depreciation_eliminated_via_reversal;
    let net_asset_value_as_on_from_date =
        value_as_on_from_date - depreciation.accumulated_depreciation_as_on_from_date;
    let net_asset_value_as_on_to_date =
        value_as_on_to_date - accumulated_depreciation_as_on_to_date;

    AssetDepreciationsAndBalancesRow {
        asset_category,
        asset,
        asset_name,
        value_as_on_from_date,
        value_of_new_purchase: value.value_of_new_purchase,
        value_of_sold_asset: value.value_of_sold_asset,
        value_of_scrapped_asset: value.value_of_scrapped_asset,
        value_of_capitalized_asset: value.value_of_capitalized_asset,
        adjustment_before_from_date: adjustment.adjustment_before_from_date,
        adjustment_till_to_date: adjustment.adjustment_till_to_date,
        adjustment_during_period,
        value_as_on_to_date,
        accumulated_depreciation_as_on_from_date: depreciation
            .accumulated_depreciation_as_on_from_date,
        depreciation_amount_during_the_period: depreciation.depreciation_amount_during_the_period,
        depreciation_eliminated_during_the_period: depreciation
            .depreciation_eliminated_during_the_period,
        accumulated_depreciation_as_on_to_date,
        depreciation_eliminated_via_reversal: depreciation.depreciation_eliminated_via_reversal,
        net_asset_value_as_on_from_date,
        net_asset_value_as_on_to_date,
    }
}

fn add_days(date: &str, days: i32) -> Option<String> {
    let (year, month, day) = parse_date(date)?;
    let days = days_from_civil(year, month, day) + i64::from(days);
    let (year, month, day) = civil_from_days(days);
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

fn parse_date(date: &str) -> Option<(i32, u32, u32)> {
    let mut parts = date.split('-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((year, month, day))
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i32;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    i64::from(era * 146_097 + doe - 719_468)
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    (year as i32, month as u32, day as u32)
}
