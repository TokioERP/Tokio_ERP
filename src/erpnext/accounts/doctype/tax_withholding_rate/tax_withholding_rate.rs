use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxWithholdingRate {
    pub from_date: String,
    pub to_date: String,
    pub tax_withholding_group: Option<String>,
    pub tax_withholding_rate: f64,
    pub cumulative_threshold: f64,
    pub single_threshold: f64,
    pub idx: usize,
}

impl TaxWithholdingRate {
    pub const DOCTYPE: &'static str = "Tax Withholding Rate";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "from_date",
        "to_date",
        "tax_withholding_group",
        "column_break_3",
        "tax_withholding_rate",
        "cumulative_threshold",
        "single_threshold",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        from_date: impl Into<String>,
        to_date: impl Into<String>,
        tax_withholding_group: impl Into<String>,
        tax_withholding_rate: f64,
    ) -> Self {
        let group = tax_withholding_group.into();
        Self {
            from_date: from_date.into(),
            to_date: to_date.into(),
            tax_withholding_group: if group.is_empty() { None } else { Some(group) },
            tax_withholding_rate,
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
            "from_date" => FieldSpec::date("from_date", "From Date")
                .required()
                .in_list_view()
                .columns(2),
            "to_date" => FieldSpec::date("to_date", "To Date")
                .required()
                .in_list_view()
                .columns(2),
            "tax_withholding_group" => {
                FieldSpec::link("tax_withholding_group", "Tax Withholding Group")
                    .options("Tax Withholding Group")
                    .in_list_view()
            }
            "column_break_3" => FieldSpec::column_break("column_break_3"),
            "tax_withholding_rate" => {
                FieldSpec::float("tax_withholding_rate", "Tax Withholding Rate")
                    .required()
                    .in_list_view()
                    .columns(1)
            }
            "cumulative_threshold" => {
                FieldSpec::float("cumulative_threshold", "Cumulative Threshold")
                    .in_list_view()
                    .columns(3)
            }
            "single_threshold" => FieldSpec::float("single_threshold", "Transaction Threshold")
                .in_list_view()
                .columns(2),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }
}

impl DocumentController for TaxWithholdingRate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
