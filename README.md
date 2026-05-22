# tokio_erp

Rust rewrite workspace for ERPNext v16 core modules.

The sibling `../erpnext` directory contains the local ERPNext v16 bench baseline used for source reading and parity checks.

## Baseline Pins

- Frappe: `version-16` at `69be97cf314e53e314678c5af98fef25f5c2d7e6`
- ERPNext: `version-16` at `6ef4a2d82cfe3815a8bd491436256df3e4ed79b8`

## Goal

Port ERPNext core logic module-by-module, file-by-file, and function-by-function into Rust while preserving observable behavior against the original ERPNext baseline.
