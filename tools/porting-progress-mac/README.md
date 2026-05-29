# Porting Progress Mac App

SwiftUI dashboard for tracking the Tokio ERP rewrite progress from `porting_manifest.json`.

## Run

Build and open the macOS app:

```bash
../../script/build_and_run.sh
```

Build the `.app` without opening it:

```bash
../../script/build_and_run.sh --no-open
```

The generated app is:

```text
tools/porting-progress-mac/dist/Tokio ERP Progress.app
```

The app icon is bundled from:

```text
tools/porting-progress-mac/Resources/AppIcon.icns
```

The terminal-only SwiftPM runner still works from this folder:

```bash
swift run PortingProgressApp --repo ../..
```

To verify the same calculation without opening the window:

```bash
swift run PortingProgressApp --repo ../.. --print
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
