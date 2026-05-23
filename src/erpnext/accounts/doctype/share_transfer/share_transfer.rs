use crate::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShareTransfer {
    pub transfer_type: String,
    pub date: String,
    pub from_shareholder: Option<String>,
    pub from_folio_no: Option<String>,
    pub to_shareholder: Option<String>,
    pub to_folio_no: Option<String>,
    pub equity_or_liability_account: Option<String>,
    pub asset_account: Option<String>,
    pub share_type: String,
    pub from_no: i32,
    pub rate: i32,
    pub no_of_shares: i32,
    pub to_no: i32,
    pub amount: i32,
    pub company: String,
    pub remarks: Option<String>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShareExistence {
    Complete,
    Partial,
    Outside,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShareTransferAction {
    AppendShareBalance {
        shareholder: String,
        balance: ShareBalance,
    },
    RemoveShares {
        shareholder: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalEntryDraft {
    pub voucher_type: String,
    pub company: String,
    pub accounts: Vec<JournalEntryLine>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalEntryLine {
    pub account: String,
    pub debit_in_account_currency: i32,
    pub credit_in_account_currency: i32,
    pub party_type: String,
    pub party: String,
}

impl ShareTransfer {
    pub const DOCTYPE: &'static str = "Share Transfer";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-SHT-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 26] = [
        "transfer_type",
        "column_break_1",
        "date",
        "section_break_1",
        "from_shareholder",
        "from_folio_no",
        "column_break_3",
        "to_shareholder",
        "to_folio_no",
        "section_break_10",
        "equity_or_liability_account",
        "column_break_12",
        "asset_account",
        "section_break_4",
        "share_type",
        "from_no",
        "rate",
        "column_break_8",
        "no_of_shares",
        "to_no",
        "amount",
        "section_break_11",
        "company",
        "section_break_6",
        "remarks",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new_issue(
        company: impl Into<String>,
        to_shareholder: impl Into<String>,
        share_type: impl Into<String>,
        from_no: i32,
        to_no: i32,
        rate: i32,
    ) -> Self {
        Self::new(
            "Issue",
            company,
            None,
            Some(to_shareholder.into()),
            share_type,
            from_no,
            to_no,
            rate,
        )
    }

    pub fn new_transfer(
        company: impl Into<String>,
        from_shareholder: impl Into<String>,
        to_shareholder: impl Into<String>,
        share_type: impl Into<String>,
        from_no: i32,
        to_no: i32,
        rate: i32,
    ) -> Self {
        Self::new(
            "Transfer",
            company,
            Some(from_shareholder.into()),
            Some(to_shareholder.into()),
            share_type,
            from_no,
            to_no,
            rate,
        )
    }

    fn new(
        transfer_type: impl Into<String>,
        company: impl Into<String>,
        from_shareholder: Option<String>,
        to_shareholder: Option<String>,
        share_type: impl Into<String>,
        from_no: i32,
        to_no: i32,
        rate: i32,
    ) -> Self {
        let no_of_shares = to_no - from_no + 1;
        Self {
            transfer_type: transfer_type.into(),
            date: String::new(),
            from_shareholder,
            to_shareholder,
            share_type: share_type.into(),
            from_no,
            to_no,
            rate,
            no_of_shares,
            amount: no_of_shares * rate,
            company: company.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "transfer_type" => FieldSpec::select("transfer_type", "Transfer Type")
                .options("\nIssue\nPurchase\nTransfer")
                .required()
                .in_list_view(),
            "date" => FieldSpec::date("date", "Date").required(),
            "from_shareholder" => FieldSpec::link("from_shareholder", "From Shareholder")
                .options("Shareholder")
                .depends_on("eval:doc.transfer_type != 'Issue'"),
            "from_folio_no" => FieldSpec::data("from_folio_no", "From Folio No")
                .fetch_from("from_shareholder.folio_no")
                .depends_on("eval:doc.transfer_type != 'Issue'"),
            "to_shareholder" => FieldSpec::link("to_shareholder", "To Shareholder")
                .options("Shareholder")
                .depends_on("eval:doc.transfer_type != 'Purchase'"),
            "to_folio_no" => FieldSpec::data("to_folio_no", "To Folio No")
                .fetch_from("to_shareholder.folio_no")
                .depends_on("eval:doc.transfer_type != 'Purchase'"),
            "equity_or_liability_account" => {
                FieldSpec::link("equity_or_liability_account", "Equity/Liability Account")
                    .options("Account")
                    .depends_on("eval:doc.company")
                    .required()
            }
            "asset_account" => FieldSpec::link("asset_account", "Asset Account")
                .options("Account")
                .depends_on("eval:(doc.transfer_type != 'Transfer') && (doc.company)"),
            "share_type" => FieldSpec::link("share_type", "Share Type")
                .options("Share Type")
                .required(),
            "from_no" => FieldSpec::int("from_no", "From No")
                .description("(including)")
                .required(),
            "rate" => FieldSpec::currency("rate", "Rate")
                .options("Company:company:default_currency")
                .required(),
            "no_of_shares" => FieldSpec::int("no_of_shares", "No of Shares").required(),
            "to_no" => FieldSpec::int("to_no", "To No")
                .description("(including)")
                .required(),
            "amount" => FieldSpec::currency("amount", "Amount")
                .options("Company:company:default_currency")
                .read_only(),
            "company" => FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            "remarks" => FieldSpec::long_text("remarks", "Remarks"),
            "amended_from" => FieldSpec::link("amended_from", "Amended From")
                .options("Share Transfer")
                .no_copy()
                .print_hide()
                .read_only(),
            field if field.starts_with("column_break") => FieldSpec::column_break(fieldname),
            field if field.starts_with("section_break") => FieldSpec::section_break(fieldname),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn validate_basic(&mut self) -> Result<(), String> {
        match self.transfer_type.as_str() {
            "Purchase" => {
                self.to_shareholder = Some(String::new());
                if self
                    .from_shareholder
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                {
                    return Err("The field From Shareholder cannot be blank".to_string());
                }
                if self.asset_account.as_deref().unwrap_or_default().is_empty() {
                    return Err("The field Asset Account cannot be blank".to_string());
                }
            }
            "Issue" => {
                self.from_shareholder = Some(String::new());
                if self
                    .to_shareholder
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                {
                    return Err("The field To Shareholder cannot be blank".to_string());
                }
                if self.asset_account.as_deref().unwrap_or_default().is_empty() {
                    return Err("The field Asset Account cannot be blank".to_string());
                }
            }
            _ => {
                if self
                    .from_shareholder
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                    || self
                        .to_shareholder
                        .as_deref()
                        .unwrap_or_default()
                        .is_empty()
                {
                    return Err(
                        "The fields From Shareholder and To Shareholder cannot be blank"
                            .to_string(),
                    );
                }
            }
        }

        if self
            .equity_or_liability_account
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            return Err("The field Equity/Liability Account cannot be blank".to_string());
        }
        if self.from_shareholder == self.to_shareholder {
            return Err("The seller and the buyer cannot be the same".to_string());
        }
        if self.no_of_shares != self.to_no - self.from_no + 1 {
            return Err("The number of shares and the share numbers are inconsistent".to_string());
        }
        if self.amount == 0 {
            self.amount = self.rate * self.no_of_shares;
        }
        if self.amount != self.rate * self.no_of_shares {
            return Err(
                "There are inconsistencies between the rate, no of shares and the amount calculated"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub fn share_exists(&self, share_balance: &[ShareBalance]) -> ShareExistence {
        for entry in share_balance {
            if entry.share_type.as_deref() != Some(self.share_type.as_str())
                || entry.from_no > self.to_no
                || entry.to_no < self.from_no
            {
                continue;
            } else if entry.from_no <= self.from_no && entry.to_no >= self.to_no {
                return ShareExistence::Complete;
            } else if entry.from_no <= self.from_no && self.from_no <= self.to_no {
                return ShareExistence::Partial;
            } else if entry.from_no <= self.to_no && self.to_no <= entry.to_no {
                return ShareExistence::Partial;
            }
        }
        ShareExistence::Outside
    }

    pub fn remove_shares_from_entries(
        &self,
        current_entries: &[ShareBalance],
    ) -> Vec<ShareBalance> {
        let mut new_entries = Vec::new();
        for entry in current_entries {
            if entry.share_type.as_deref() != Some(self.share_type.as_str())
                || entry.from_no > self.to_no
                || entry.to_no < self.from_no
            {
                new_entries.push(entry.clone());
            } else if entry.from_no <= self.from_no && entry.to_no >= self.to_no {
                if entry.from_no == self.from_no {
                    if entry.to_no != self.to_no {
                        new_entries.push(self.return_share_balance_entry(
                            self.to_no + 1,
                            entry.to_no,
                            entry.rate,
                        ));
                    }
                } else if entry.to_no == self.to_no {
                    new_entries.push(self.return_share_balance_entry(
                        entry.from_no,
                        self.from_no - 1,
                        entry.rate,
                    ));
                } else {
                    new_entries.push(self.return_share_balance_entry(
                        entry.from_no,
                        self.from_no - 1,
                        entry.rate,
                    ));
                    new_entries.push(self.return_share_balance_entry(
                        self.to_no + 1,
                        entry.to_no,
                        entry.rate,
                    ));
                }
            } else if entry.from_no >= self.from_no && entry.to_no <= self.to_no {
            } else if self.from_no <= entry.from_no
                && entry.from_no <= self.to_no
                && entry.to_no >= self.to_no
            {
                new_entries.push(self.return_share_balance_entry(
                    self.to_no + 1,
                    entry.to_no,
                    entry.rate,
                ));
            } else if self.from_no <= entry.to_no
                && entry.to_no <= self.to_no
                && entry.from_no <= self.from_no
            {
                new_entries.push(self.return_share_balance_entry(
                    entry.from_no,
                    self.from_no - 1,
                    entry.rate,
                ));
            } else {
                new_entries.push(entry.clone());
            }
        }
        new_entries
    }

    pub fn return_share_balance_entry(&self, from_no: i32, to_no: i32, rate: i32) -> ShareBalance {
        ShareBalance {
            share_type: Some(self.share_type.clone()),
            from_no,
            to_no,
            rate,
            no_of_shares: to_no - from_no + 1,
            amount: self.rate * (to_no - from_no + 1),
            is_company: false,
            current_state: None,
        }
    }

    fn transfer_balance(&self) -> ShareBalance {
        ShareBalance::new(self.share_type.clone(), self.from_no, self.to_no, self.rate)
    }

    pub fn on_submit(&self) -> Vec<ShareTransferAction> {
        match self.transfer_type.as_str() {
            "Issue" => {
                let company_balance = ShareBalance {
                    is_company: true,
                    current_state: Some("Issued".to_string()),
                    ..self.transfer_balance()
                };
                vec![
                    ShareTransferAction::AppendShareBalance {
                        shareholder: self.company.clone(),
                        balance: company_balance,
                    },
                    ShareTransferAction::AppendShareBalance {
                        shareholder: self.to_shareholder.clone().unwrap_or_default(),
                        balance: self.transfer_balance(),
                    },
                ]
            }
            "Purchase" => vec![
                ShareTransferAction::RemoveShares {
                    shareholder: self.from_shareholder.clone().unwrap_or_default(),
                },
                ShareTransferAction::RemoveShares {
                    shareholder: self.company.clone(),
                },
            ],
            "Transfer" => vec![
                ShareTransferAction::RemoveShares {
                    shareholder: self.from_shareholder.clone().unwrap_or_default(),
                },
                ShareTransferAction::AppendShareBalance {
                    shareholder: self.to_shareholder.clone().unwrap_or_default(),
                    balance: self.transfer_balance(),
                },
            ],
            _ => Vec::new(),
        }
    }

    pub fn on_cancel(&self) -> Vec<ShareTransferAction> {
        match self.transfer_type.as_str() {
            "Issue" => vec![
                ShareTransferAction::RemoveShares {
                    shareholder: self.company.clone(),
                },
                ShareTransferAction::RemoveShares {
                    shareholder: self.to_shareholder.clone().unwrap_or_default(),
                },
            ],
            "Purchase" => vec![
                ShareTransferAction::AppendShareBalance {
                    shareholder: self.from_shareholder.clone().unwrap_or_default(),
                    balance: self.transfer_balance(),
                },
                ShareTransferAction::AppendShareBalance {
                    shareholder: self.company.clone(),
                    balance: self.transfer_balance(),
                },
            ],
            "Transfer" => vec![
                ShareTransferAction::RemoveShares {
                    shareholder: self.to_shareholder.clone().unwrap_or_default(),
                },
                ShareTransferAction::AppendShareBalance {
                    shareholder: self.from_shareholder.clone().unwrap_or_default(),
                    balance: self.transfer_balance(),
                },
            ],
            _ => Vec::new(),
        }
    }
}

impl DocumentController for ShareTransfer {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit", "on_cancel"]
    }
}

pub fn make_jv_entry(
    company: &str,
    account: &str,
    amount: i32,
    payment_account: &str,
    credit_applicant_type: &str,
    credit_applicant: &str,
    debit_applicant_type: &str,
    debit_applicant: &str,
) -> JournalEntryDraft {
    JournalEntryDraft {
        voucher_type: "Journal Entry".to_string(),
        company: company.to_string(),
        accounts: vec![
            JournalEntryLine {
                account: account.to_string(),
                debit_in_account_currency: amount,
                credit_in_account_currency: 0,
                party_type: debit_applicant_type.to_string(),
                party: debit_applicant.to_string(),
            },
            JournalEntryLine {
                account: payment_account.to_string(),
                debit_in_account_currency: 0,
                credit_in_account_currency: amount,
                party_type: credit_applicant_type.to_string(),
                party: credit_applicant.to_string(),
            },
        ],
    }
}

pub fn share_transfer_js_hooks() -> [&'static str; 5] {
    [
        "refresh",
        "no_of_shares",
        "rate",
        "company",
        "transfer_type",
    ]
}
