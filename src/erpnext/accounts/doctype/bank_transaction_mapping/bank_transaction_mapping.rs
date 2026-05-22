use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankTransactionMapping {
    pub bank_transaction_field: Option<String>,
    pub file_field: Option<String>,
}

impl BankTransactionMapping {
    pub const DOCTYPE: &'static str = "Bank Transaction Mapping";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["bank_transaction_field", "file_field"];
    pub const IS_TABLE: bool = true;

    pub fn new(bank_transaction_field: impl Into<String>, file_field: impl Into<String>) -> Self {
        Self {
            bank_transaction_field: Some(bank_transaction_field.into()),
            file_field: Some(file_field.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("bank_transaction_field", "Field in Bank Transaction")
                .required()
                .in_list_view(),
            FieldSpec::data("file_field", "Column in Bank File")
                .required()
                .in_list_view(),
        ]
    }
}

impl DocumentController for BankTransactionMapping {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
