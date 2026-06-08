use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShipmentDeliveryNote {
    pub delivery_note: Option<String>,
    pub grand_total: Option<f64>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ShipmentDeliveryNote {
    pub const DOCTYPE: &'static str = "Shipment Delivery Note";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 2] = ["delivery_note", "grand_total"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        delivery_note: impl Into<String>,
        grand_total: Option<f64>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            delivery_note: Some(delivery_note.into()),
            grand_total,
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("delivery_note", "Delivery Note")
                .options("Delivery Note")
                .in_list_view()
                .required(),
            FieldSpec::currency("grand_total", "Value")
                .in_list_view()
                .read_only(),
        ]
    }
}

impl DocumentController for ShipmentDeliveryNote {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
