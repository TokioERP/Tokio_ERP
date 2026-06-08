use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeliverySettings {
    pub dispatch_template: Option<String>,
    pub dispatch_attachment: Option<String>,
    pub send_with_attachment: bool,
    pub stop_delay: i32,
}

impl DeliverySettings {
    pub const DOCTYPE: &'static str = "Delivery Settings";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "sb_dispatch",
        "dispatch_template",
        "dispatch_attachment",
        "send_with_attachment",
        "cb_delivery",
        "stop_delay",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const GRID_PAGE_LENGTH: u16 = 50;
    pub const HIDE_TOOLBAR: bool = false;
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        dispatch_template: Option<&str>,
        dispatch_attachment: Option<&str>,
        send_with_attachment: bool,
        stop_delay: i32,
    ) -> Self {
        Self {
            dispatch_template: dispatch_template.map(ToOwned::to_owned),
            dispatch_attachment: dispatch_attachment.map(ToOwned::to_owned),
            send_with_attachment,
            stop_delay,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("sb_dispatch").label("Dispatch Settings"),
            FieldSpec::link("dispatch_template", "Dispatch Notification Template")
                .options("Email Template"),
            FieldSpec::link("dispatch_attachment", "Dispatch Notification Attachment")
                .options("Print Format")
                .depends_on("send_with_attachment")
                .description("Leave blank to use the standard Delivery Note format"),
            FieldSpec::check("send_with_attachment", "Send with Attachment").default("0"),
            FieldSpec::column_break("cb_delivery"),
            FieldSpec::int("stop_delay", "Delay between Delivery Stops").description("In minutes"),
        ]
    }
}

impl DocumentController for DeliverySettings {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
