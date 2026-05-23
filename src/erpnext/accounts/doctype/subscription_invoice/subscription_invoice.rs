use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SubscriptionInvoice {
    pub document_type: Option<String>,
    pub invoice: Option<String>,
}

impl SubscriptionInvoice {
    pub const DOCTYPE: &'static str = "Subscription Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["document_type", "invoice"];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(document_type: impl Into<String>, invoice: impl Into<String>) -> Self {
        Self {
            document_type: Some(document_type.into()),
            invoice: Some(invoice.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("document_type", "Document Type ")
                .options("DocType")
                .read_only()
                .no_copy(),
            FieldSpec::dynamic_link("invoice")
                .label("Invoice")
                .options("document_type")
                .read_only()
                .no_copy()
                .in_list_view(),
        ]
    }
}

impl DocumentController for SubscriptionInvoice {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
