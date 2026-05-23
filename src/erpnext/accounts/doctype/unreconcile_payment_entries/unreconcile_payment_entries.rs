use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnreconcilePaymentEntries {
    pub name: Option<String>,
    pub account: Option<String>,
    pub account_currency: Option<String>,
    pub allocated_amount: f64,
    pub party: Option<String>,
    pub party_type: Option<String>,
    pub reference_doctype: Option<String>,
    pub reference_name: Option<String>,
    pub unlinked: bool,
}

impl UnreconcilePaymentEntries {
    pub const DOCTYPE: &'static str = "Unreconcile Payment Entries";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "account",
        "party_type",
        "party",
        "reference_doctype",
        "reference_name",
        "allocated_amount",
        "account_currency",
        "unlinked",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(
        reference_doctype: impl Into<String>,
        reference_name: impl Into<String>,
        allocated_amount: f64,
    ) -> Self {
        Self {
            reference_doctype: Some(reference_doctype.into()),
            reference_name: Some(reference_name.into()),
            allocated_amount,
            unlinked: false,
            ..Self::default()
        }
    }

    pub fn with_account_party(
        mut self,
        account: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
    ) -> Self {
        self.account = Some(account.into());
        self.party_type = Some(party_type.into());
        self.party = Some(party.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_doctype")
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("account_currency")
                .in_list_view(),
            FieldSpec::check("unlinked", "Unlinked")
                .default("0")
                .read_only()
                .in_list_view(),
            FieldSpec::link("reference_doctype", "Reference Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::data("account", "Account"),
            FieldSpec::data("party_type", "Party Type"),
            FieldSpec::data("party", "Party"),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .read_only(),
        ]
    }
}

impl DocumentController for UnreconcilePaymentEntries {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
