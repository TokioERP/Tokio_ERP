# Porting Progress Mac App

SwiftUI dashboard for tracking the Tokio ERP rewrite progress from `porting_manifest.json`.

## Run

From this folder:

```bash
swift run PortingProgressApp --repo /Volumes/Samsung990P/rust_erp/tokio_erp
```

To verify the same calculation without opening the window:

```bash
swift run PortingProgressApp --repo /Volumes/Samsung990P/rust_erp/tokio_erp --print
```

The app reads:

- `porting_manifest.json`
- `../erpnext/apps/erpnext/erpnext`
- `src/erpnext`
- `tests`

## Verify

```bash
swift test
swift build
```
