use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemSupplier {
    pub supplier: Option<String>,
    pub supplier_part_no: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemSupplier {
    pub const DOCTYPE: &'static str = "Item Supplier";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 3] =
        ["supplier", "column_break_vcuv", "supplier_part_no"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        supplier: Option<&str>,
        supplier_part_no: Option<&str>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            supplier: supplier.map(ToOwned::to_owned),
            supplier_part_no: supplier_part_no.map(ToOwned::to_owned),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_vcuv"),
            FieldSpec::data("supplier_part_no", "Supplier Part Number")
                .in_global_search()
                .in_list_view()
                .width("200px"),
        ]
    }
}

impl DocumentController for ItemSupplier {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
