# Notices and Attribution

Tokio ERP is an independent Rust rewrite of ERPNext-compatible core logic.

## ERPNext Attribution

This project includes Rust code that is derived from, ported from, or written to preserve behavioral compatibility with ERPNext v16 source code.

ERPNext copyright is owned by Frappe Technologies Pvt. Ltd. and contributors.

ERPNext source code is licensed under the GNU General Public License v3.0.

Reference:

- ERPNext repository: https://github.com/frappe/erpnext
- ERPNext license and trademark page: https://erpnext.com/license-trademark
- GPLv3 license text: https://www.gnu.org/licenses/gpl-3.0.html

## Baseline Pins

The current parity baseline used by this repository is:

- Frappe: `version-16` at `69be97cf314e53e314678c5af98fef25f5c2d7e6`
- ERPNext: `version-16` at `6ef4a2d82cfe3815a8bd491436256df3e4ed79b8`

## Modification Notice

Tokio ERP is not a verbatim copy of ERPNext. It is a Rust rewrite and work-in-progress port. Behavior is tested against selected ERPNext source modules as they are ported.

Files under `src/erpnext` and matching tests may correspond to ERPNext source paths listed in `porting_manifest.json`.

## No Warranty

Tokio ERP is distributed under GPLv3 without warranty. See `LICENSE`.
