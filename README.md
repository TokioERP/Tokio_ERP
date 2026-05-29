# Tokio ERP

Tokio ERP is an independent Rust rewrite of ERPNext-compatible core logic.

The project is currently a parity-first rewrite workspace: modules are ported from the ERPNext v16 Python baseline into Rust with focused tests that preserve formulas, branching, report shapes, filters, and edge cases before runtime database/UI integration is added.

## Status

Tokio ERP is under active development and is not production-ready.

Most current ports model ERPNext behavior with explicit Rust structs, in-memory execution, and parity tests. Direct Frappe, MariaDB/MySQL, web UI, and deployment integration are later phases.

## Baseline

The current rewrite baseline is:

- Frappe: `version-16` at `69be97cf314e53e314678c5af98fef25f5c2d7e6`
- ERPNext: `version-16` at `6ef4a2d82cfe3815a8bd491436256df3e4ed79b8`

`porting_manifest.json` tracks source-to-target porting status.

## License

Tokio ERP is licensed under the GNU General Public License v3.0 only. See [LICENSE](LICENSE).

ERPNext is licensed under the GNU General Public License v3.0. Portions of Tokio ERP are derived from, ported from, or designed for behavioral compatibility with ERPNext. See [NOTICE](NOTICE.md) and [TRADEMARKS](TRADEMARKS.md).

## Trademark Notice

Tokio ERP is not affiliated with, endorsed by, sponsored by, or approved by Frappe Technologies Pvt. Ltd.

ERPNext and Frappe are trademarks of Frappe Technologies Pvt. Ltd. Their names are used in this repository only for attribution, source-reference, and compatibility description.

## Development

Run the Rust test suite:

```bash
cargo test
```

Run a focused test:

```bash
cargo test --test accounts_report_profit_and_loss_statement
```

Run compile checks:

```bash
cargo check
```

Check whitespace before committing:

```bash
git diff --check
```

## Repository Layout

- `src/erpnext`: Rust modules organized to mirror ERPNext source paths.
- `tests`: focused parity tests.
- `porting_manifest.json`: porting status manifest.
- `docs`: rewrite notes and port maps.
- `tools/porting-progress-mac`: optional local macOS progress dashboard.

## Public Use

If you redistribute Tokio ERP or modified versions of it, comply with GPLv3 requirements, including preserving license notices and providing corresponding source code.

Do not use ERPNext or Frappe names or logos as part of Tokio ERP branding, product names, service names, domains, or logos.
