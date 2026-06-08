use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShipmentParcelTemplate {
    pub parcel_template_name: Option<String>,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
}

impl ShipmentParcelTemplate {
    pub const DOCTYPE: &'static str = "Shipment Parcel Template";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "parcel_template_name",
        "length",
        "width",
        "height",
        "weight",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        parcel_template_name: impl Into<String>,
        length: f64,
        width: f64,
        height: f64,
        weight: f64,
    ) -> Self {
        Self {
            parcel_template_name: Some(parcel_template_name.into()),
            length: Some(length),
            width: Some(width),
            height: Some(height),
            weight: Some(weight),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("parcel_template_name", "Parcel Template Name")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::float("length", "Length (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("width", "Width (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("height", "Height (cm)")
                .in_list_view()
                .required(),
            FieldSpec::float("weight", "Weight (kg)")
                .in_list_view()
                .precision("1")
                .required(),
        ]
    }
}

impl DocumentController for ShipmentParcelTemplate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
