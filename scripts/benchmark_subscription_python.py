#!/usr/bin/env python3
import json
import time
from datetime import date, timedelta

PRORATA_ITERS = 1_000_000
GATE_ITERS = 1_000_000
INVOICE_ITERS = 100_000


def parse_date(value):
    return date.fromisoformat(value)


def date_diff(left, right):
    return (parse_date(left) - parse_date(right)).days


def add_days(value, days):
    return (parse_date(value) + timedelta(days=days)).isoformat()


def get_prorata_factor(period_end, period_start, is_prepaid=None, now_date="2018-01-15"):
    if is_prepaid:
        return 1

    diff = date_diff(now_date, period_start) + 1
    plan_days = date_diff(period_end, period_start) + 1
    return diff / plan_days


def can_generate_new_invoice(subscription, posting_date, has_outstanding_invoice):
    if subscription["cancelation_date"]:
        return False

    if has_outstanding_invoice and not subscription["generate_new_invoices_past_due_date"]:
        return False

    if (
        subscription["generate_invoice_at"] == "Beginning of the current subscription period"
        and parse_date(posting_date) == parse_date(subscription["current_invoice_start"])
    ):
        return True
    elif (
        subscription["generate_invoice_at"] == "Days before the current subscription period"
        and parse_date(posting_date)
        == parse_date(add_days(subscription["current_invoice_start"], -subscription["number_of_days"]))
    ):
        return True
    elif parse_date(posting_date) == parse_date(subscription["current_invoice_end"]):
        return True
    else:
        return False


def create_invoice_plan(subscription, plan_snapshot, prorate, now_date, default_company, supplier_has_tax_withholding):
    company = subscription["company"] or default_company
    if not company:
        raise ValueError("company required")

    document_type = "Sales Invoice" if subscription["party_type"] == "Customer" else "Purchase Invoice"
    current_start = subscription["current_invoice_start"]
    current_end = subscription["current_invoice_end"]
    if subscription["generate_invoice_at"] == "Beginning of the current subscription period":
        posting_date = current_start
    elif subscription["generate_invoice_at"] == "Days before the current subscription period":
        posting_date = now_date
    else:
        posting_date = current_end

    is_sales_invoice = document_type == "Sales Invoice"
    payment_schedule = []
    if subscription["days_until_due"]:
        payment_schedule.append(
            {
                "due_date": add_days(posting_date, subscription["days_until_due"]),
                "invoice_portion": 100,
            }
        )

    has_discount = (
        subscription["additional_discount_percentage"] != 0
        or subscription["additional_discount_amount"] != 0
    )
    apply_discount_on = None
    if has_discount:
        apply_discount_on = subscription["apply_additional_discount"] or "Grand Total"

    item = {
        "item_code": plan_snapshot["item"],
        "qty": 1,
        "rate": plan_snapshot["cost"] * (get_prorata_factor(current_end, current_start, 1, now_date) if prorate else 1),
        "cost_center": plan_snapshot["cost_center"],
        "enable_deferred_revenue": False,
        "enable_deferred_expense": False,
        "service_start_date": None,
        "service_end_date": None,
        "dimensions": dict(plan_snapshot["dimensions"]),
    }

    return {
        "document_type": document_type,
        "company": company,
        "set_posting_time": True,
        "posting_date": posting_date,
        "cost_center": subscription["cost_center"],
        "customer": subscription["party"] if is_sales_invoice else None,
        "supplier": subscription["party"] if not is_sales_invoice else None,
        "apply_tds": (not is_sales_invoice) and supplier_has_tax_withholding,
        "currency": plan_snapshot["currency"],
        "items": [item],
        "taxes_and_charges": subscription["sales_tax_template"] if is_sales_invoice else subscription["purchase_tax_template"],
        "payment_schedule": payment_schedule,
        "additional_discount_percentage": subscription["additional_discount_percentage"],
        "discount_amount": subscription["additional_discount_amount"],
        "apply_discount_on": apply_discount_on,
        "subscription": subscription["name"],
        "from_date": current_start,
        "to_date": current_end,
        "ignore_mandatory": True,
        "submit": subscription["submit_invoice"],
    }


def elapsed_ns(fn):
    start = time.perf_counter_ns()
    acc = fn()
    return time.perf_counter_ns() - start, acc


def main():
    prorata_ns, prorata_acc = elapsed_ns(
        lambda: sum(
            get_prorata_factor(
                "2018-01-31",
                "2018-01-01",
                0,
                "2018-01-15" if index % 2 == 0 else "2018-01-20",
            )
            for index in range(PRORATA_ITERS)
        )
    )

    subscription = {
        "name": "ACC-SUB-0001",
        "party_type": "Customer",
        "party": "_Test Customer",
        "company": "_Test Company",
        "cancelation_date": None,
        "generate_new_invoices_past_due_date": False,
        "current_invoice_start": "2018-01-01",
        "current_invoice_end": "2018-01-31",
        "generate_invoice_at": "Days before the current subscription period",
        "number_of_days": 10,
        "days_until_due": 10,
        "cost_center": "Main - TC",
        "sales_tax_template": "_Test Sales Taxes",
        "purchase_tax_template": None,
        "additional_discount_percentage": 10.0,
        "additional_discount_amount": 0.0,
        "apply_additional_discount": None,
        "submit_invoice": True,
    }

    def run_gate():
        acc = 0
        for index in range(GATE_ITERS):
            posting_date = "2017-12-22" if index % 2 == 0 else "2017-12-21"
            acc += int(can_generate_new_invoice(subscription, posting_date, False))
        return acc

    gate_ns, gate_acc = elapsed_ns(run_gate)

    subscription["generate_invoice_at"] = "Beginning of the current subscription period"
    plan = {
        "name": "_Test Plan",
        "item": "Service Item",
        "currency": "USD",
        "cost_center": "Main - TC",
        "cost": 900.0,
        "dimensions": {"project": "PROJ-001"},
    }

    def run_invoice():
        acc = 0
        for _ in range(INVOICE_ITERS):
            invoice = create_invoice_plan(subscription, plan, False, "2018-01-15", "_Default Company", False)
            acc += len(invoice["items"])
        return acc

    invoice_ns, invoice_acc = elapsed_ns(run_invoice)

    print(
        json.dumps(
            {
                "prorata": {"iters": PRORATA_ITERS, "ns": prorata_ns, "acc": prorata_acc},
                "gate": {"iters": GATE_ITERS, "ns": gate_ns, "acc": gate_acc},
                "invoice": {"iters": INVOICE_ITERS, "ns": invoice_ns, "acc": invoice_acc},
            },
            separators=(",", ":"),
        )
    )


if __name__ == "__main__":
    main()
