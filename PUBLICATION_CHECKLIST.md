# Public Release Checklist

Use this checklist before switching the GitHub repository from private to public.

## Required

- [ ] Keep `LICENSE` as GPL-3.0-only.
- [ ] Keep `NOTICE.md` and `TRADEMARKS.md` in the repository root.
- [ ] Keep README disclaimer that Tokio ERP is independent and not affiliated with Frappe Technologies.
- [ ] Do not use ERPNext or Frappe logos, icons, screenshots, or brand assets.
- [ ] Do not use ERPNext or Frappe in the repository owner, repository name, package name, domain, product name, service name, or logo.
- [ ] Keep all source code required to build and modify Tokio ERP available under GPLv3.
- [ ] Preserve attribution for ERPNext/Frappe and contributors.
- [ ] Run `rg -n "/Users/|/Volumes/|password|secret|token|api_key|BEGIN .*KEY" -S . --glob '!target'` and review every hit.
- [ ] Run `cargo test`.
- [ ] Run `git diff --check`.

## Recommended

- [ ] Run a dependency license audit before release.
- [ ] Review GitHub repository description: avoid implying official ERPNext or Frappe affiliation.
- [ ] Review GitHub topics: use descriptive topics only.
- [ ] Review issue templates and contribution docs before accepting outside contributions.
- [ ] Register or clear the Tokio ERP trademark if commercial use is planned.

## Safe Wording

Use:

```text
Tokio ERP is an independent GPLv3 Rust rewrite of ERPNext-compatible core logic.
It is not affiliated with or endorsed by Frappe Technologies Pvt. Ltd.
```

Avoid:

```text
Official ERPNext Rust
ERPNext Rust Edition
Frappe-approved ERP
```
