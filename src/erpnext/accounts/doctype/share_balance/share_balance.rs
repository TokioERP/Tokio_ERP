use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShareBalance {
    pub share_type: Option<String>,
    pub from_no: i32,
    pub rate: i32,
    pub no_of_shares: i32,
    pub to_no: i32,
    pub amount: i32,
    pub is_company: bool,
    pub current_state: Option<String>,
}

impl ShareBalance {
    pub const DOCTYPE: &'static str = "Share Balance";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "share_type",
        "from_no",
        "rate",
        "column_break_4",
        "no_of_shares",
        "to_no",
        "amount",
        "section_break_8",
        "is_company",
        "current_state",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(share_type: impl Into<String>, from_no: i32, to_no: i32, rate: i32) -> Self {
        let no_of_shares = to_no - from_no + 1;
        Self {
            share_type: Some(share_type.into()),
            from_no,
            to_no,
            rate,
            no_of_shares,
            amount: no_of_shares * rate,
            is_company: false,
            current_state: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("share_type", "Share Type")
                .options("Share Type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::int("from_no", "From No").required().read_only(),
            FieldSpec::currency("rate", "Rate")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::int("no_of_shares", "No of Shares")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::int("to_no", "To No").required().read_only(),
            FieldSpec::currency("amount", "Amount")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_8"),
            FieldSpec::check("is_company", "Is Company")
                .default("0")
                .hidden()
                .read_only(),
            FieldSpec::select("current_state", "Current State")
                .options("\nIssued\nPurchased")
                .hidden()
                .read_only(),
        ]
    }
}

impl DocumentController for ShareBalance {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
