use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlEntry {
    pub name: Option<String>,
    pub company: String,
    pub account: String,
    pub posting_date: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub cost_center: Option<String>,
    pub party: Option<String>,
    pub party_type: Option<String>,
    pub voucher_detail_no: Option<String>,
    pub against_voucher: Option<String>,
    pub against_voucher_type: Option<String>,
    pub project: Option<String>,
    pub finance_book: Option<String>,
    pub advance_voucher_type: Option<String>,
    pub advance_voucher_no: Option<String>,
    pub account_currency: Option<String>,
    pub remarks: Option<String>,
    pub is_opening: Option<String>,
    pub is_cancelled: i32,
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub debit_in_transaction_currency: f64,
    pub credit_in_transaction_currency: f64,
    pub post_net_value: bool,
    pub skip_merge: bool,
    pub merge_key: Vec<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralLedgerContext {
    pub precision: u32,
    pub round_off_account: Option<String>,
    pub round_off_settings: RoundOffSettings,
    pub cost_center_allocations: BTreeMap<String, Vec<(String, f64)>>,
    pub accounting_dimensions: Vec<String>,
    pub exchange_gain_loss_journal_entries: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingDimensionOffset {
    pub fieldname: String,
    pub name: String,
    pub offsetting_account: String,
    pub account_currency: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingPeriod {
    pub name: String,
    pub exempted_role: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingDimensionDefault {
    pub company: String,
    pub fieldname: String,
    pub mandatory_for_pl: bool,
    pub mandatory_for_bs: bool,
    pub default_dimension: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DimensionPolicy {
    Allow,
    Restrict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionFilterRule {
    pub is_mandatory: bool,
    pub policy: DimensionPolicy,
    pub allowed_dimensions: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RoundOffSettings {
    pub round_off_account: Option<String>,
    pub round_off_cost_center: Option<String>,
    pub round_off_for_opening: Option<String>,
    pub default_expense_account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RoundOffLookup {
    pub company_round_off_account: Option<String>,
    pub company_round_off_cost_center: Option<String>,
    pub round_off_for_opening: Option<String>,
    pub default_expense_account: Option<String>,
    pub voucher_has_cost_center: bool,
    pub parent_cost_center: Option<String>,
    pub use_company_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CancelOriginalPlan {
    None,
    ByNames(Vec<String>),
    ByVoucher {
        voucher_type: String,
        voucher_no: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReverseGlPlan {
    pub cancel_original_entries: bool,
    pub cancel_original_plan: CancelOriginalPlan,
    pub partial_cancel: bool,
    pub reversed_entries: Vec<GlEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaveEntriesPlan {
    pub validate_budget: bool,
    pub create_payment_ledger_entry: bool,
    pub payment_ledger_cancel: i32,
    pub adv_adj: bool,
    pub update_outstanding: String,
    pub from_repost: bool,
    pub entries: Vec<GlEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntrySubmitPlan {
    pub ignore_permissions: bool,
    pub from_repost: bool,
    pub adv_adj: bool,
    pub update_outstanding: String,
    pub notify_update: bool,
    pub validate_expense_against_budget: bool,
    pub entry: GlEntry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MakeGlEntriesPlan {
    Noop,
    Save(SaveEntriesPlan),
    Reverse(ReverseGlPlan),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeneralLedgerError {
    ClosedAccountingPeriod(String),
    InvalidAccountDimension(String),
    MandatoryAccountDimension(String),
    Validation(String),
}

pub fn make_gl_entries_plan(
    gl_map: Vec<GlEntry>,
    cancel: bool,
    adv_adj: bool,
    merge_entries: bool,
    update_outstanding: &str,
    from_repost: bool,
    use_legacy_budget_controller: bool,
    context: &GeneralLedgerContext,
) -> Result<MakeGlEntriesPlan, GeneralLedgerError> {
    if gl_map.is_empty() {
        return Ok(MakeGlEntriesPlan::Noop);
    }

    if cancel {
        return Ok(MakeGlEntriesPlan::Reverse(make_reverse_gl_entries_plan(
            &gl_map, false, None,
        )));
    }

    let validate_budget =
        !use_legacy_budget_controller && gl_map[0].voucher_type != "Period Closing Voucher";
    let processed_gl_map = process_gl_map(gl_map, merge_entries, context);

    if processed_gl_map.len() > 1 {
        return Ok(MakeGlEntriesPlan::Save(SaveEntriesPlan {
            validate_budget,
            create_payment_ledger_entry: processed_gl_map[0].voucher_type
                != "Period Closing Voucher",
            payment_ledger_cancel: 0,
            adv_adj,
            update_outstanding: if update_outstanding.is_empty() {
                "Yes".to_string()
            } else {
                update_outstanding.to_string()
            },
            from_repost,
            entries: processed_gl_map,
        }));
    }

    if !processed_gl_map.is_empty() {
        return Err(GeneralLedgerError::Validation(
            "Incorrect number of General Ledger Entries found. You might have selected a wrong Account in the transaction.".to_string(),
        ));
    }

    Ok(MakeGlEntriesPlan::Noop)
}

pub fn make_entry_plan(
    entry: &GlEntry,
    adv_adj: bool,
    update_outstanding: &str,
    from_repost: bool,
) -> EntrySubmitPlan {
    EntrySubmitPlan {
        ignore_permissions: true,
        from_repost,
        adv_adj,
        update_outstanding: if update_outstanding.is_empty() {
            "Yes".to_string()
        } else {
            update_outstanding.to_string()
        },
        notify_update: false,
        validate_expense_against_budget: !from_repost
            && entry.voucher_type != "Period Closing Voucher"
            && (entry.is_cancelled == 0 || entry.voucher_type == "Journal Entry"),
        entry: entry.clone(),
    }
}

pub fn get_accounting_dimensions_for_offsetting_entry(
    gl_map: &[GlEntry],
    accounting_dimensions: &[AccountingDimensionOffset],
) -> Vec<AccountingDimensionOffset> {
    accounting_dimensions
        .iter()
        .filter(|dimension| {
            gl_map
                .iter()
                .map(|entry| entry.dimensions.get(&dimension.fieldname).cloned())
                .collect::<BTreeSet<_>>()
                .len()
                > 1
        })
        .cloned()
        .collect()
}

pub fn get_cost_center_allocation_data(
    context: &GeneralLedgerContext,
    _company: &str,
    _posting_date: &str,
    cost_center: &str,
) -> Vec<(String, f64)> {
    context
        .cost_center_allocations
        .get(cost_center)
        .cloned()
        .unwrap_or_default()
}

pub fn validate_accounting_period(
    first_entry: &GlEntry,
    accounting_periods: &[AccountingPeriod],
    user_roles: &BTreeSet<String>,
) -> Result<(), GeneralLedgerError> {
    let Some(period) = accounting_periods.first() else {
        return Ok(());
    };
    if period
        .exempted_role
        .as_ref()
        .is_some_and(|role| user_roles.contains(role))
    {
        return Ok(());
    }
    let _ = first_entry;
    Err(GeneralLedgerError::ClosedAccountingPeriod(format!(
        "You cannot create or cancel any accounting entries with in the closed Accounting Period <b>{}</b>",
        period.name
    )))
}

pub fn validate_cwip_accounts(
    gl_map: &[GlEntry],
    cwip_enabled: bool,
    cwip_accounts: &BTreeSet<String>,
) -> Result<(), GeneralLedgerError> {
    if gl_map
        .first()
        .is_some_and(|entry| entry.voucher_type != "Journal Entry")
    {
        return Ok(());
    }
    if !cwip_enabled {
        return Ok(());
    }
    for entry in gl_map {
        if cwip_accounts.contains(&entry.account) {
            return Err(GeneralLedgerError::Validation(format!(
                "Account: <b>{}</b> is capital Work in progress and can not be updated by Journal Entry",
                entry.account
            )));
        }
    }
    Ok(())
}

pub fn get_round_off_account_and_cost_center(
    company: &str,
    lookup: RoundOffLookup,
) -> Result<RoundOffSettings, GeneralLedgerError> {
    let round_off_account = lookup
        .company_round_off_account
        .clone()
        .or(lookup.default_expense_account.clone())
        .ok_or_else(|| {
            GeneralLedgerError::Validation(format!(
                "Please mention '<b>Round Off Account</b>' in Company: {company}"
            ))
        })?;
    let mut round_off_cost_center = lookup.company_round_off_cost_center.clone();
    if !lookup.use_company_default && lookup.voucher_has_cost_center {
        if let Some(parent_cost_center) = lookup.parent_cost_center.clone() {
            round_off_cost_center = Some(parent_cost_center);
        }
    }
    let round_off_cost_center = round_off_cost_center.ok_or_else(|| {
        GeneralLedgerError::Validation(format!(
            "Please mention '<b>Round Off Cost Center</b>' in Company: {company}"
        ))
    })?;
    Ok(RoundOffSettings {
        round_off_account: Some(round_off_account),
        round_off_cost_center: Some(round_off_cost_center),
        round_off_for_opening: lookup.round_off_for_opening,
        default_expense_account: lookup.default_expense_account,
    })
}

pub fn save_entries_plan(
    mut gl_map: Vec<GlEntry>,
    adv_adj: bool,
    update_outstanding: &str,
    from_repost: bool,
    context: &GeneralLedgerContext,
) -> Result<SaveEntriesPlan, GeneralLedgerError> {
    if !from_repost {
        validate_cwip_accounts(&gl_map, false, &BTreeSet::new())?;
    }
    process_debit_credit_difference(
        &mut gl_map,
        context.precision,
        context.round_off_settings.clone(),
        false,
    )?;
    Ok(SaveEntriesPlan {
        validate_budget: gl_map
            .first()
            .is_some_and(|entry| !from_repost && entry.voucher_type != "Period Closing Voucher"),
        create_payment_ledger_entry: false,
        payment_ledger_cancel: 0,
        adv_adj,
        update_outstanding: if update_outstanding.is_empty() {
            "Yes".to_string()
        } else {
            update_outstanding.to_string()
        },
        from_repost,
        entries: gl_map,
    })
}

pub fn update_accounting_dimensions(
    round_off_gle: &mut GlEntry,
    dimensions: &[String],
    voucher_dimension_values: &BTreeMap<String, String>,
    account_report_type: &str,
    checks_for_pl_and_bs_accounts: &[AccountingDimensionDefault],
) {
    if !dimensions.is_empty()
        && dimensions
            .iter()
            .all(|dimension| voucher_dimension_values.contains_key(dimension))
    {
        for dimension in dimensions {
            if let Some(value) = voucher_dimension_values.get(dimension) {
                round_off_gle
                    .dimensions
                    .insert(dimension.clone(), value.clone());
            }
        }
        return;
    }

    for dimension in checks_for_pl_and_bs_accounts {
        let matches_company = round_off_gle.company == dimension.company;
        let matches_report_type = (account_report_type == "Profit and Loss"
            && dimension.mandatory_for_pl)
            || (account_report_type == "Balance Sheet" && dimension.mandatory_for_bs);
        if matches_company && matches_report_type {
            if let Some(default_dimension) = &dimension.default_dimension {
                round_off_gle
                    .dimensions
                    .insert(dimension.fieldname.clone(), default_dimension.clone());
            }
        }
    }
}

pub fn process_gl_map(
    mut gl_map: Vec<GlEntry>,
    merge_entries: bool,
    context: &GeneralLedgerContext,
) -> Vec<GlEntry> {
    if gl_map.is_empty() {
        return Vec::new();
    }
    if gl_map[0].voucher_type != "Period Closing Voucher" {
        gl_map = distribute_gl_based_on_cost_center_allocation(gl_map, context);
    }
    if merge_entries {
        gl_map = merge_similar_entries(gl_map, context);
    }
    toggle_debit_credit_if_negative(gl_map)
}

pub fn distribute_gl_based_on_cost_center_allocation(
    gl_map: Vec<GlEntry>,
    context: &GeneralLedgerContext,
) -> Vec<GlEntry> {
    let mut new_gl_map = Vec::new();
    for entry in gl_map {
        let Some(cost_center) = entry.cost_center.as_deref() else {
            new_gl_map.push(entry);
            continue;
        };
        let Some(allocation) = context.cost_center_allocations.get(cost_center) else {
            new_gl_map.push(entry);
            continue;
        };
        if allocation.is_empty() {
            new_gl_map.push(entry);
            continue;
        }

        if context.round_off_account.as_deref() == Some(entry.account.as_str()) {
            let mut round_off_entry = entry;
            round_off_entry.cost_center = Some(allocation[0].0.clone());
            new_gl_map.push(round_off_entry);
            continue;
        }

        for (sub_cost_center, percentage) in allocation {
            let mut split = entry.clone();
            split.cost_center = Some(sub_cost_center.clone());
            split.debit = round_to_precision(entry.debit * percentage / 100.0, context.precision);
            split.credit = round_to_precision(entry.credit * percentage / 100.0, context.precision);
            split.debit_in_account_currency = round_to_precision(
                entry.debit_in_account_currency * percentage / 100.0,
                context.precision,
            );
            split.credit_in_account_currency = round_to_precision(
                entry.credit_in_account_currency * percentage / 100.0,
                context.precision,
            );
            split.debit_in_transaction_currency = round_to_precision(
                entry.debit_in_transaction_currency * percentage / 100.0,
                context.precision,
            );
            split.credit_in_transaction_currency = round_to_precision(
                entry.credit_in_transaction_currency * percentage / 100.0,
                context.precision,
            );
            new_gl_map.push(split);
        }
    }
    new_gl_map
}

pub fn merge_similar_entries(gl_map: Vec<GlEntry>, context: &GeneralLedgerContext) -> Vec<GlEntry> {
    let merge_properties = get_merge_properties(&context.accounting_dimensions);
    let mut merged_gl_map: Vec<GlEntry> = Vec::new();

    for mut entry in gl_map {
        if entry.skip_merge {
            merged_gl_map.push(entry);
            continue;
        }
        entry.merge_key = get_merge_key(&entry, &merge_properties);
        if let Some(existing) = merged_gl_map
            .iter_mut()
            .find(|existing| existing.merge_key == entry.merge_key)
        {
            existing.debit += entry.debit;
            existing.debit_in_account_currency += entry.debit_in_account_currency;
            existing.debit_in_transaction_currency += entry.debit_in_transaction_currency;
            existing.credit += entry.credit;
            existing.credit_in_account_currency += entry.credit_in_account_currency;
            existing.credit_in_transaction_currency += entry.credit_in_transaction_currency;
        } else {
            merged_gl_map.push(entry);
        }
    }

    merged_gl_map
        .into_iter()
        .filter(|entry| {
            round_to_precision(entry.debit, context.precision) != 0.0
                || round_to_precision(entry.credit, context.precision) != 0.0
                || (entry.voucher_type == "Journal Entry"
                    && context
                        .exchange_gain_loss_journal_entries
                        .contains(&entry.voucher_no))
        })
        .collect()
}

pub fn get_merge_properties(dimensions: &[String]) -> Vec<String> {
    let mut merge_properties = [
        "account",
        "cost_center",
        "party",
        "party_type",
        "voucher_detail_no",
        "against_voucher",
        "against_voucher_type",
        "project",
        "finance_book",
        "voucher_no",
        "advance_voucher_type",
        "advance_voucher_no",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect::<Vec<_>>();
    merge_properties.extend(dimensions.iter().cloned());
    merge_properties
}

pub fn get_merge_key(entry: &GlEntry, merge_properties: &[String]) -> Vec<String> {
    merge_properties
        .iter()
        .map(|fieldname| entry_value(entry, fieldname).unwrap_or_default())
        .collect()
}

pub fn toggle_debit_credit_if_negative(mut gl_map: Vec<GlEntry>) -> Vec<GlEntry> {
    for entry in &mut gl_map {
        toggle_pair(&mut entry.debit, &mut entry.credit, entry.post_net_value);
        toggle_pair(
            &mut entry.debit_in_account_currency,
            &mut entry.credit_in_account_currency,
            entry.post_net_value,
        );
        toggle_pair(
            &mut entry.debit_in_transaction_currency,
            &mut entry.credit_in_transaction_currency,
            entry.post_net_value,
        );
    }
    gl_map
}

pub fn make_acc_dimensions_offsetting_entry(
    gl_map: &mut Vec<GlEntry>,
    accounting_dimensions_to_offset: &[AccountingDimensionOffset],
) {
    let no_of_dimensions = accounting_dimensions_to_offset.len();
    if no_of_dimensions == 0 {
        return;
    }

    let original_entries = gl_map.clone();
    for entry in original_entries {
        for dimension in accounting_dimensions_to_offset {
            let mut offsetting_entry = entry.clone();
            let debit = if entry.credit != 0.0 {
                entry.credit / no_of_dimensions as f64
            } else {
                0.0
            };
            let credit = if entry.debit != 0.0 {
                entry.debit / no_of_dimensions as f64
            } else {
                0.0
            };
            offsetting_entry.account = dimension.offsetting_account.clone();
            offsetting_entry.debit = debit;
            offsetting_entry.credit = credit;
            offsetting_entry.debit_in_account_currency = debit;
            offsetting_entry.credit_in_account_currency = credit;
            offsetting_entry.remarks = Some(format!(
                "Offsetting for Accounting Dimension - {}",
                dimension.name
            ));
            offsetting_entry.against_voucher = None;
            offsetting_entry.against_voucher_type = None;
            offsetting_entry.account_currency = Some(dimension.account_currency.clone());
            offsetting_entry.party_type = None;
            offsetting_entry.party = None;
            gl_map.push(offsetting_entry);
        }
    }
}

pub fn get_debit_credit_difference(gl_map: &mut [GlEntry], precision: u32) -> (f64, f64) {
    let mut debit_credit_diff = 0.0;
    let mut trx_cur_debit_credit_diff = 0.0;
    for entry in gl_map {
        entry.debit = round_to_precision(entry.debit, precision);
        entry.credit = round_to_precision(entry.credit, precision);
        debit_credit_diff += entry.debit - entry.credit;

        entry.debit_in_transaction_currency =
            round_to_precision(entry.debit_in_transaction_currency, precision);
        entry.credit_in_transaction_currency =
            round_to_precision(entry.credit_in_transaction_currency, precision);
        trx_cur_debit_credit_diff +=
            entry.debit_in_transaction_currency - entry.credit_in_transaction_currency;
    }
    (
        round_to_precision(debit_credit_diff, precision),
        round_to_precision(trx_cur_debit_credit_diff, precision),
    )
}

pub fn get_debit_credit_allowance(voucher_type: &str, precision: u32) -> f64 {
    if matches!(voucher_type, "Journal Entry" | "Payment Entry") {
        5.0 / 10_f64.powi(precision as i32)
    } else {
        0.5
    }
}

pub fn process_debit_credit_difference(
    gl_map: &mut Vec<GlEntry>,
    precision: u32,
    round_off_settings: RoundOffSettings,
    is_exchange_gain_or_loss: bool,
) -> Result<(), GeneralLedgerError> {
    if gl_map.is_empty() {
        return Ok(());
    }
    let voucher_type = gl_map[0].voucher_type.clone();
    let voucher_no = gl_map[0].voucher_no.clone();
    let allowance = get_debit_credit_allowance(&voucher_type, precision);
    let (debit_credit_diff, trx_cur_debit_credit_diff) =
        get_debit_credit_difference(gl_map, precision);

    if debit_credit_diff.abs() > allowance {
        if !is_exchange_gain_or_loss {
            return Err(debit_credit_not_equal_error(
                debit_credit_diff,
                &voucher_type,
                &voucher_no,
            ));
        }
    } else if debit_credit_diff.abs() >= 1.0 / 10_f64.powi(precision as i32) {
        make_round_off_gle(
            gl_map,
            debit_credit_diff,
            trx_cur_debit_credit_diff,
            precision,
            round_off_settings,
        )?;
    }

    let (debit_credit_diff, _) = get_debit_credit_difference(gl_map, precision);
    if debit_credit_diff.abs() > allowance && !is_exchange_gain_or_loss {
        return Err(debit_credit_not_equal_error(
            debit_credit_diff,
            &voucher_type,
            &voucher_no,
        ));
    }
    Ok(())
}

pub fn make_round_off_gle(
    gl_map: &mut Vec<GlEntry>,
    debit_credit_diff: f64,
    trx_cur_debit_credit_diff: f64,
    _precision: u32,
    settings: RoundOffSettings,
) -> Result<(), GeneralLedgerError> {
    let has_opening_entry = has_opening_entries(gl_map);
    let account = if has_opening_entry {
        settings.round_off_for_opening.clone().ok_or_else(|| {
            GeneralLedgerError::Validation(format!(
                "Please set '<b>Round Off for Opening</b>' in Company: {}",
                gl_map[0].company
            ))
        })?
    } else {
        settings
            .round_off_account
            .clone()
            .or(settings.default_expense_account.clone())
            .ok_or_else(|| {
                GeneralLedgerError::Validation(format!(
                    "Please mention '<b>Round Off Account</b>' in Company: {}",
                    gl_map[0].company
                ))
            })?
    };
    let cost_center = settings.round_off_cost_center.clone().ok_or_else(|| {
        GeneralLedgerError::Validation(format!(
            "Please mention '<b>Round Off Cost Center</b>' in Company: {}",
            gl_map[0].company
        ))
    })?;

    if gl_map[0].voucher_type != "Period Closing Voucher" {
        if let Some(index) = gl_map.iter().position(|entry| entry.account == account) {
            let mut adjusted_diff = debit_credit_diff;
            if gl_map[index].debit != 0.0 {
                adjusted_diff -= gl_map[index].debit - gl_map[index].credit;
            } else {
                adjusted_diff += gl_map[index].credit;
            }
            if adjusted_diff.abs() < 1.0 / 10_f64.powi(_precision as i32) {
                gl_map.remove(index);
                return Ok(());
            }
        }
    }

    let mut round_off_gle = GlEntry {
        voucher_type: gl_map[0].voucher_type.clone(),
        voucher_no: gl_map[0].voucher_no.clone(),
        company: gl_map[0].company.clone(),
        posting_date: gl_map[0].posting_date.clone(),
        remarks: gl_map[0].remarks.clone(),
        account,
        debit_in_account_currency: if debit_credit_diff < 0.0 {
            debit_credit_diff.abs()
        } else {
            0.0
        },
        credit_in_account_currency: if debit_credit_diff > 0.0 {
            debit_credit_diff
        } else {
            0.0
        },
        debit: if debit_credit_diff < 0.0 {
            debit_credit_diff.abs()
        } else {
            0.0
        },
        credit: if debit_credit_diff > 0.0 {
            debit_credit_diff
        } else {
            0.0
        },
        debit_in_transaction_currency: if trx_cur_debit_credit_diff < 0.0 {
            trx_cur_debit_credit_diff.abs()
        } else {
            0.0
        },
        credit_in_transaction_currency: if trx_cur_debit_credit_diff > 0.0 {
            trx_cur_debit_credit_diff
        } else {
            0.0
        },
        cost_center: Some(cost_center),
        party_type: None,
        party: None,
        is_opening: Some(if has_opening_entry { "Yes" } else { "No" }.to_string()),
        against_voucher_type: None,
        against_voucher: None,
        ..GlEntry::default()
    };
    if round_off_gle.remarks.is_none() {
        round_off_gle.remarks = Some(String::new());
    }
    gl_map.push(round_off_gle);
    Ok(())
}

pub fn has_opening_entries(gl_map: &[GlEntry]) -> bool {
    gl_map
        .iter()
        .any(|entry| entry.is_opening.as_deref() == Some("Yes"))
}

pub fn make_reverse_gl_entries_plan(
    gl_entries: &[GlEntry],
    immutable_ledger_enabled: bool,
    posting_date: Option<&str>,
) -> ReverseGlPlan {
    let mut reversed_entries = Vec::new();
    let cancel_original_plan = if immutable_ledger_enabled || gl_entries.is_empty() {
        CancelOriginalPlan::None
    } else {
        let gle_names = gl_entries
            .iter()
            .map(|entry| entry.name.clone())
            .collect::<Vec<_>>();
        if gle_names.iter().all(Option::is_some) {
            CancelOriginalPlan::ByNames(gle_names.into_iter().flatten().collect())
        } else {
            CancelOriginalPlan::ByVoucher {
                voucher_type: gl_entries[0].voucher_type.clone(),
                voucher_no: gl_entries[0].voucher_no.clone(),
            }
        }
    };
    for entry in gl_entries {
        let mut new_gle = entry.clone();
        new_gle.name = None;
        new_gle.debit = entry.credit;
        new_gle.credit = entry.debit;
        new_gle.debit_in_account_currency = entry.credit_in_account_currency;
        new_gle.credit_in_account_currency = entry.debit_in_account_currency;
        new_gle.debit_in_transaction_currency = entry.credit_in_transaction_currency;
        new_gle.credit_in_transaction_currency = entry.debit_in_transaction_currency;
        new_gle.remarks = Some(format!("On cancellation of {}", entry.voucher_no));
        new_gle.is_cancelled = if immutable_ledger_enabled { 0 } else { 1 };
        if immutable_ledger_enabled {
            if let Some(posting_date) = posting_date {
                new_gle.posting_date = posting_date.to_string();
            }
        } else if let Some(posting_date) = posting_date {
            new_gle.posting_date = posting_date.to_string();
        }
        if new_gle.debit != 0.0 || new_gle.credit != 0.0 {
            reversed_entries.push(new_gle);
        }
    }
    ReverseGlPlan {
        cancel_original_entries: !immutable_ledger_enabled,
        cancel_original_plan,
        partial_cancel: false,
        reversed_entries,
    }
}

pub fn validate_disabled_accounts(
    gl_map: &[GlEntry],
    disabled_accounts: &BTreeSet<String>,
) -> Result<(), GeneralLedgerError> {
    let used_disabled_accounts = gl_map
        .iter()
        .filter(|entry| disabled_accounts.contains(&entry.account))
        .map(|entry| entry.account.clone())
        .collect::<BTreeSet<_>>();
    if used_disabled_accounts.is_empty() {
        return Ok(());
    }
    Err(GeneralLedgerError::Validation(format!(
        "Cannot create accounting entries against disabled accounts: <br>{}",
        used_disabled_accounts
            .iter()
            .map(|account| format!("<b>{account}</b>"))
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

pub fn check_freezing_date(
    posting_date: &str,
    acc_frozen_till_date: Option<&str>,
    frozen_accounts_modifier: Option<&str>,
    user_roles: &BTreeSet<String>,
    session_user: &str,
    adv_adj: bool,
) -> Result<(), GeneralLedgerError> {
    if adv_adj {
        return Ok(());
    }
    let Some(frozen_date) = acc_frozen_till_date else {
        return Ok(());
    };
    if posting_date <= frozen_date
        && (frozen_accounts_modifier.is_none_or(|role| !user_roles.contains(role))
            || session_user == "Administrator")
    {
        return Err(GeneralLedgerError::Validation(format!(
            "You are not authorized to add or update entries before {frozen_date}"
        )));
    }
    Ok(())
}

pub fn validate_against_pcv(
    is_opening: bool,
    posting_date: &str,
    pcv_exists: bool,
    last_pcv_date: Option<&str>,
) -> Result<(), GeneralLedgerError> {
    if is_opening && pcv_exists {
        return Err(GeneralLedgerError::Validation(
            "Opening Entry can not be created after Period Closing Voucher is created.".to_string(),
        ));
    }
    if let Some(last_pcv_date) = last_pcv_date {
        if posting_date <= last_pcv_date {
            return Err(GeneralLedgerError::Validation(format!(
                "Books have been closed till the period ending on {last_pcv_date}</br >You cannot create/amend any accounting entries till this date."
            )));
        }
    }
    Ok(())
}

pub fn validate_allowed_dimensions(
    gl_entry: &GlEntry,
    dimension_filter_map: &BTreeMap<(String, String), DimensionFilterRule>,
) -> Result<(), GeneralLedgerError> {
    for ((dimension, account), rule) in dimension_filter_map {
        if gl_entry.account != *account {
            continue;
        }
        let dimension_value = gl_entry.dimensions.get(dimension);
        if rule.is_mandatory && dimension_value.is_none_or(|value| value.is_empty()) {
            return Err(GeneralLedgerError::MandatoryAccountDimension(format!(
                "{} is mandatory for account {}",
                unscrub(dimension),
                gl_entry.account
            )));
        }
        if let Some(value) = dimension_value {
            match rule.policy {
                DimensionPolicy::Allow if !rule.allowed_dimensions.contains(value) => {
                    return Err(GeneralLedgerError::InvalidAccountDimension(format!(
                        "Invalid value {value} for {} against account {}",
                        unscrub(dimension),
                        gl_entry.account
                    )));
                }
                DimensionPolicy::Restrict if rule.allowed_dimensions.contains(value) => {
                    return Err(GeneralLedgerError::InvalidAccountDimension(format!(
                        "Invalid value {value} for {} against account {}",
                        unscrub(dimension),
                        gl_entry.account
                    )));
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn debit_credit_not_equal_error(
    debit_credit_diff: f64,
    voucher_type: &str,
    voucher_no: &str,
) -> GeneralLedgerError {
    GeneralLedgerError::Validation(format!(
        "Debit and Credit not equal for {voucher_type} #{voucher_no}. Difference is {debit_credit_diff}."
    ))
}

fn entry_value(entry: &GlEntry, fieldname: &str) -> Option<String> {
    match fieldname {
        "account" => Some(entry.account.clone()),
        "cost_center" => entry.cost_center.clone(),
        "party" => entry.party.clone(),
        "party_type" => entry.party_type.clone(),
        "voucher_detail_no" => entry.voucher_detail_no.clone(),
        "against_voucher" => entry.against_voucher.clone(),
        "against_voucher_type" => entry.against_voucher_type.clone(),
        "project" => entry.project.clone(),
        "finance_book" => entry.finance_book.clone(),
        "voucher_no" => Some(entry.voucher_no.clone()),
        "advance_voucher_type" => entry.advance_voucher_type.clone(),
        "advance_voucher_no" => entry.advance_voucher_no.clone(),
        dimension => entry.dimensions.get(dimension).cloned(),
    }
}

fn toggle_pair(debit: &mut f64, credit: &mut f64, post_net_value: bool) {
    if *debit < 0.0 && *credit < 0.0 && *debit == *credit {
        *debit *= -1.0;
        *credit *= -1.0;
    }
    if *debit < 0.0 {
        *credit -= *debit;
        *debit = 0.0;
    }
    if *credit < 0.0 {
        *debit -= *credit;
        *credit = 0.0;
    }
    if post_net_value && *debit != 0.0 && *credit != 0.0 {
        if *debit > *credit {
            *debit -= *credit;
            *credit = 0.0;
        } else {
            *credit -= *debit;
            *debit = 0.0;
        }
    }
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn unscrub(value: &str) -> String {
    value
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
