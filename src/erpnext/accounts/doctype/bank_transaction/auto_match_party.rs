use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AutoMatchParty {
    pub bank_party_account_number: Option<String>,
    pub bank_party_iban: Option<String>,
    pub bank_party_name: Option<String>,
    pub description: Option<String>,
    pub deposit: f64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AutoMatchByAccountIban {
    pub bank_party_account_number: Option<String>,
    pub bank_party_iban: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountIbanFilters(pub BTreeMap<String, String>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoMatchResult {
    pub party_type: String,
    pub party: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzyExtractResult {
    pub choice: String,
    pub score: i64,
    pub party_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzyMatchDecision {
    pub party_name: Option<String>,
    pub skip: bool,
}

impl AutoMatchParty {
    pub fn match_with_context(
        &self,
        fuzzy_matching_enabled: bool,
        account_iban_result: Option<AutoMatchResult>,
        fuzzy_result: Option<AutoMatchResult>,
    ) -> Option<AutoMatchResult> {
        account_iban_result.or_else(|| fuzzy_matching_enabled.then_some(fuzzy_result).flatten())
    }
}

impl AutoMatchByAccountIban {
    pub fn get_or_filters(&self, party: Option<&str>) -> AccountIbanFilters {
        let mut filters = BTreeMap::new();

        if let Some(account_number) = self.bank_party_account_number.as_deref() {
            let bank_account_field = if party == Some("Employee") {
                "bank_ac_no"
            } else {
                "bank_account_no"
            };
            filters.insert(bank_account_field.to_string(), account_number.to_string());
        }

        if let Some(iban) = self.bank_party_iban.as_deref() {
            filters.insert("iban".to_string(), iban.to_string());
        }

        AccountIbanFilters(filters)
    }

    pub fn match_account_in_party(
        &self,
        bank_account_party: Option<AutoMatchResult>,
        employee_name: Option<&str>,
    ) -> Option<AutoMatchResult> {
        if self.bank_party_account_number.is_none() && self.bank_party_iban.is_none() {
            return None;
        }

        bank_account_party
            .or_else(|| employee_name.map(|name| AutoMatchResult::new("Employee", name)))
    }
}

impl AutoMatchResult {
    pub fn new(party_type: impl Into<String>, party: impl Into<String>) -> Self {
        Self {
            party_type: party_type.into(),
            party: party.into(),
        }
    }
}

impl FuzzyExtractResult {
    pub fn new(choice: impl Into<String>, score: i64, party_id: impl Into<String>) -> Self {
        Self {
            choice: choice.into(),
            score,
            party_id: party_id.into(),
        }
    }
}

impl FuzzyMatchDecision {
    pub fn from_extract_results(result: &[FuzzyExtractResult]) -> Self {
        const CUTOFF: i64 = 80;

        let Some(first_result) = result.first() else {
            return Self {
                party_name: None,
                skip: false,
            };
        };

        if result.len() == 1 {
            return Self {
                party_name: (first_result.score > CUTOFF).then(|| first_result.party_id.clone()),
                skip: true,
            };
        }

        if first_result.score > CUTOFF {
            let second_result = &result[1];
            if first_result.score == second_result.score {
                return Self {
                    party_name: None,
                    skip: true,
                };
            }

            return Self {
                party_name: Some(first_result.party_id.clone()),
                skip: true,
            };
        }

        Self {
            party_name: None,
            skip: false,
        }
    }
}

pub fn get_parties_in_order(deposit: f64) -> [&'static str; 3] {
    if deposit > 0.0 {
        ["Customer", "Supplier", "Employee"]
    } else {
        ["Supplier", "Employee", "Customer"]
    }
}
