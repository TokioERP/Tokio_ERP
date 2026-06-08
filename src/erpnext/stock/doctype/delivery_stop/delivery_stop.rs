use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeliveryStop {
    pub customer: Option<String>,
    pub address: Option<String>,
    pub locked: bool,
    pub customer_address: Option<String>,
    pub visited: bool,
    pub delivery_note: Option<String>,
    pub grand_total: Option<f64>,
    pub contact: Option<String>,
    pub email_sent_to: Option<String>,
    pub customer_contact: Option<String>,
    pub distance: Option<f64>,
    pub estimated_arrival: Option<String>,
    pub lat: Option<f64>,
    pub uom: Option<String>,
    pub lng: Option<f64>,
    pub details: Option<String>,
}

impl DeliveryStop {
    pub const DOCTYPE: &'static str = "Delivery Stop";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 24] = [
        "customer",
        "address",
        "locked",
        "column_break_6",
        "customer_address",
        "visited",
        "order_information_section",
        "delivery_note",
        "cb_order",
        "grand_total",
        "section_break_7",
        "contact",
        "email_sent_to",
        "column_break_7",
        "customer_contact",
        "section_break_9",
        "distance",
        "estimated_arrival",
        "lat",
        "column_break_19",
        "uom",
        "lng",
        "more_information_section",
        "details",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .in_list_view()
                .columns(2),
            FieldSpec::link("address", "Address Name")
                .options("Address")
                .in_list_view()
                .print_hide()
                .required(),
            FieldSpec::check("locked", "Locked")
                .default("0")
                .in_list_view(),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::small_text("customer_address", "Customer Address").read_only(),
            FieldSpec::check("visited", "Visited")
                .default("0")
                .allow_on_submit()
                .depends_on("eval:doc.docstatus==1")
                .in_list_view()
                .no_copy()
                .print_hide(),
            FieldSpec::section_break("order_information_section").label("Order Information"),
            FieldSpec::link("delivery_note", "Delivery Note")
                .options("Delivery Note")
                .in_list_view()
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::column_break("cb_order"),
            FieldSpec::currency("grand_total", "Grand Total").read_only(),
            FieldSpec::section_break("section_break_7").label("Contact Information"),
            FieldSpec::link("contact", "Contact Name")
                .options("Contact")
                .print_hide(),
            FieldSpec::data("email_sent_to", "Email sent to").read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::small_text("customer_contact", "Customer Contact").read_only(),
            FieldSpec::section_break("section_break_9").label("Dispatch Information"),
            FieldSpec::float("distance", "Distance")
                .precision("2")
                .read_only(),
            FieldSpec::datetime("estimated_arrival", "Estimated Arrival").in_list_view(),
            FieldSpec::float("lat", "Latitude").hidden(),
            FieldSpec::column_break("column_break_19"),
            FieldSpec::link("uom", "UOM")
                .options("UOM")
                .depends_on("eval:doc.distance")
                .read_only(),
            FieldSpec::float("lng", "Longitude").hidden(),
            FieldSpec::section_break("more_information_section").label("More Information"),
            FieldSpec::text_editor("details", "Details"),
        ]
    }
}

impl DocumentController for DeliveryStop {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
