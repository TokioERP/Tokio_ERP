use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use tokio_erp::erpnext::accounts::doctype::subscription::subscription::{
    get_prorata_factor_at, GenerateInvoiceAt, PlanRateSource, Subscription,
    SubscriptionPlanSnapshot,
};
use tokio_erp::erpnext::accounts::doctype::subscription_plan_detail::subscription_plan_detail::SubscriptionPlanDetail;

const PRORATA_ITERS: usize = 1_000_000;
const GATE_ITERS: usize = 1_000_000;
const INVOICE_ITERS: usize = 100_000;

fn main() {
    let prorata_start = Instant::now();
    let mut prorata_acc = 0.0;
    for index in 0..PRORATA_ITERS {
        let now_date = if index % 2 == 0 {
            "2018-01-15"
        } else {
            "2018-01-20"
        };
        prorata_acc += black_box(get_prorata_factor_at(
            "2018-01-31",
            "2018-01-01",
            Some(0),
            now_date,
        ));
    }
    let prorata_ns = prorata_start.elapsed().as_nanos();

    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::DaysBeforeCurrentPeriod;
    subscription.number_of_days = 10;

    let gate_start = Instant::now();
    let mut gate_acc = 0usize;
    for index in 0..GATE_ITERS {
        let posting_date = if index % 2 == 0 {
            "2017-12-22"
        } else {
            "2017-12-21"
        };
        gate_acc += black_box(subscription.can_generate_new_invoice(posting_date, false)) as usize;
    }
    let gate_ns = gate_start.elapsed().as_nanos();

    subscription.name = Some("ACC-SUB-0001".to_string());
    subscription.company = Some("_Test Company".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::BeginningOfCurrentPeriod;
    subscription.days_until_due = 10;
    subscription.cost_center = Some("Main - TC".to_string());
    subscription.sales_tax_template = Some("_Test Sales Taxes".to_string());
    subscription.additional_discount_percentage = 10.0;
    subscription.plans = vec![SubscriptionPlanDetail::new("_Test Plan", 1)];

    let mut dimensions = BTreeMap::new();
    dimensions.insert("project".to_string(), "PROJ-001".to_string());
    let plan = SubscriptionPlanSnapshot {
        name: "_Test Plan".to_string(),
        item: "Service Item".to_string(),
        currency: "USD".to_string(),
        cost_center: Some("Main - TC".to_string()),
        rate_source: PlanRateSource::FixedRate { cost: 900.0 },
        enable_deferred_revenue: false,
        enable_deferred_expense: false,
        dimensions,
    };

    let invoice_start = Instant::now();
    let mut invoice_acc = 0usize;
    for _ in 0..INVOICE_ITERS {
        let invoice = subscription
            .create_invoice_plan(
                &[plan.clone()],
                false,
                "2018-01-15",
                Some("_Default Company"),
                false,
            )
            .expect("invoice plan");
        invoice_acc += black_box(invoice.items.len());
    }
    let invoice_ns = invoice_start.elapsed().as_nanos();

    println!(
        "{{\"prorata\":{{\"iters\":{PRORATA_ITERS},\"ns\":{prorata_ns},\"acc\":{prorata_acc}}},\"gate\":{{\"iters\":{GATE_ITERS},\"ns\":{gate_ns},\"acc\":{gate_acc}}},\"invoice\":{{\"iters\":{INVOICE_ITERS},\"ns\":{invoice_ns},\"acc\":{invoice_acc}}}}}"
    );
}
