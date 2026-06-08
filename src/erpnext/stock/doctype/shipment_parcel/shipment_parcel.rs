use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShipmentParcel {
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    pub count: Option<i32>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ShipmentParcel {
    pub const DOCTYPE: &'static str = "Shipment Parcel";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 5] = ["length", "width", "height", "weight", "count"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        length: Option<f64>,
        width: Option<f64>,
        height: Option<f64>,
        weight: f64,
        count: i32,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            length,
            width,
            height,
            weight: Some(weight),
            count: Some(count),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::float("length", "Length (cm)").in_list_view(),
            FieldSpec::float("width", "Width (cm)").in_list_view(),
            FieldSpec::float("height", "Height (cm)").in_list_view(),
            FieldSpec::float("weight", "Weight (kg)")
                .in_list_view()
                .precision("1")
                .required(),
            FieldSpec::int("count", "Count")
                .default("1")
                .in_list_view()
                .required(),
        ]
    }
}

impl DocumentController for ShipmentParcel {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
