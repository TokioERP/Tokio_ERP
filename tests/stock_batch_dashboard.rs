use tokio_erp::erpnext::stock::doctype::batch::batch_dashboard::get_data as batch_dashboard;
use tokio_erp::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

#[test]
fn batch_dashboard_matches_erpnext_frontend_payload_shape() {
    assert_eq!(
        batch_dashboard(),
        DashboardData::new("batch_no").transactions(vec![
            DashboardSection::labeled("Buy", vec!["Purchase Invoice", "Purchase Receipt"]),
            DashboardSection::labeled("Sell", vec!["Sales Invoice", "Delivery Note"]),
            DashboardSection::labeled("Move", vec!["Stock Entry", "Serial and Batch Bundle"]),
            DashboardSection::labeled("Quality", vec!["Quality Inspection"]),
        ])
    );
}
