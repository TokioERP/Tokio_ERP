import Darwin
import AppKit
import Foundation
import PortingProgressCore
import SwiftUI

@main
struct PortingProgressApplication: App {
    init() {
        CommandLineReporter.printAndExitIfRequested()
        NSApplication.shared.setActivationPolicy(.regular)
    }

    var body: some Scene {
        WindowGroup {
            DashboardView()
                .frame(minWidth: 1080, minHeight: 720)
        }
        .windowStyle(.titleBar)
    }
}

enum CommandLineReporter {
    static func printAndExitIfRequested() {
        guard CommandLine.arguments.contains("--print") else {
            return
        }

        do {
            let calculator = ProgressCalculator()
            let repoRoot = try RepositoryResolver.resolve(calculator: calculator)
            let report = try calculator.report(repoRoot: repoRoot)
            print("repo=\(report.repoRoot.path)")
            printScope("overall", report.overall)
            printScope("accounts", report.accounts)
            exit(0)
        } catch {
            fputs("\(error.localizedDescription)\n", stderr)
            exit(1)
        }
    }

    private static func printScope(_ name: String, _ scope: ProgressScope) {
        print("\(name).files=\(scope.completeFiles)/\(scope.files)")
        print("\(name).lines=\(scope.completeLines)/\(scope.lines)")
        print(String(format: "\(name).file_percent=%.6f", scope.completeFilePercent))
        print(String(format: "\(name).line_percent=%.6f", scope.completeLinePercent))
        print(String(format: "\(name).parity_line_percent=%.6f", scope.parityLinePercent))
    }
}

@MainActor
final class DashboardModel: ObservableObject {
    @Published var report: ProgressReport?
    @Published var errorMessage: String?
    @Published var repoRoot: URL?

    private let calculator = ProgressCalculator()

    func refresh() {
        do {
            let root = try resolveRepoRoot()
            report = try calculator.report(repoRoot: root)
            repoRoot = root
            errorMessage = nil
        } catch {
            report = nil
            errorMessage = error.localizedDescription
        }
    }

    private func resolveRepoRoot() throws -> URL {
        try RepositoryResolver.resolve(calculator: calculator)
    }
}

enum RepositoryResolver {
    private static let defaultRepoRoot = URL(
        fileURLWithPath: "/Volumes/Samsung990P/rust_erp/tokio_erp",
        isDirectory: true
    )

    static func resolve(calculator: ProgressCalculator) throws -> URL {
        let args = CommandLine.arguments
        if let index = args.firstIndex(of: "--repo"), args.indices.contains(index + 1) {
            return URL(fileURLWithPath: args[index + 1], isDirectory: true)
        }

        var lastError: Error?
        for start in candidateStarts() {
            do {
                return try calculator.findRepositoryRoot(startingAt: start)
            } catch {
                lastError = error
            }
        }

        let defaultManifest = defaultRepoRoot.appendingPathComponent("porting_manifest.json")
        if FileManager.default.fileExists(atPath: defaultManifest.path) {
            return defaultRepoRoot
        }

        throw lastError ?? ProgressCalculatorError.cannotFindRepositoryRoot(defaultRepoRoot)
    }

    private static func candidateStarts() -> [URL] {
        var starts = [
            URL(fileURLWithPath: FileManager.default.currentDirectoryPath, isDirectory: true),
            Bundle.main.bundleURL,
            Bundle.main.bundleURL.deletingLastPathComponent(),
        ]

        if let executableDirectory = Bundle.main.executableURL?.deletingLastPathComponent() {
            starts.append(executableDirectory)
        }

        var seen = Set<String>()
        return starts.filter { url in
            let path = url.standardizedFileURL.path
            guard !seen.contains(path) else {
                return false
            }
            seen.insert(path)
            return true
        }
    }
}

struct DashboardView: View {
    @StateObject private var model = DashboardModel()

    var body: some View {
        NavigationSplitView {
            sidebar
        } detail: {
            detail
        }
        .onAppear {
            model.refresh()
            NSApplication.shared.activate(ignoringOtherApps: true)
        }
    }

    private var sidebar: some View {
        List {
            Label("Overview", systemImage: "chart.pie")
            Label("Accounts", systemImage: "folder")
            Label("Incomplete", systemImage: "tray")
        }
        .listStyle(.sidebar)
        .navigationTitle("Tokio ERP")
        .toolbar {
            Button {
                model.refresh()
            } label: {
                Label("Refresh", systemImage: "arrow.clockwise")
            }
        }
    }

    @ViewBuilder
    private var detail: some View {
        if let report = model.report {
            ScrollView {
                VStack(alignment: .leading, spacing: 22) {
                    header(report)
                    LazyVGrid(
                        columns: [
                            GridItem(.flexible(), spacing: 16),
                            GridItem(.flexible(), spacing: 16),
                        ],
                        spacing: 16
                    ) {
                        ScopeCard(scope: report.overall, systemImage: "globe")
                        ScopeCard(scope: report.accounts, systemImage: "building.columns")
                    }
                    StatusTable(scope: report.overall, title: "Overall Status")
                    IncompleteList(entries: Array(report.incompleteEntries.prefix(80)))
                }
                .padding(24)
            }
        } else {
            VStack(spacing: 12) {
                Image(systemName: "exclamationmark.triangle")
                    .font(.system(size: 42))
                    .foregroundStyle(.secondary)
                Text("Progress data unavailable")
                    .font(.title2.weight(.semibold))
                Text(model.errorMessage ?? "Run from the Tokio ERP repository or pass --repo.")
                    .foregroundStyle(.secondary)
                    .multilineTextAlignment(.center)
                    .frame(maxWidth: 520)
            }
            .padding(32)
        }
    }

    private func header(_ report: ProgressReport) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Porting Progress")
                .font(.system(size: 34, weight: .semibold))
            Text(report.repoRoot.path)
                .font(.callout)
                .foregroundStyle(.secondary)
                .lineLimit(1)
                .truncationMode(.middle)
        }
    }
}

struct ScopeCard: View {
    let scope: ProgressScope
    let systemImage: String

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            HStack {
                Label(scope.title, systemImage: systemImage)
                    .font(.title3.weight(.semibold))
                Spacer()
                Text(format(scope.completeLinePercent))
                    .font(.title2.monospacedDigit().weight(.semibold))
            }

            ProgressView(value: scope.completeLinePercent, total: 100)
                .progressViewStyle(.linear)

            HStack(spacing: 20) {
                MetricView(title: "Files", value: "\(scope.completeFiles)/\(scope.files)")
                MetricView(title: "LOC", value: "\(scope.completeLines)/\(scope.lines)")
                MetricView(title: "Parity", value: format(scope.parityLinePercent))
            }
        }
        .padding(18)
        .background(.regularMaterial)
        .clipShape(RoundedRectangle(cornerRadius: 8))
    }

    private func format(_ value: Double) -> String {
        String(format: "%.2f%%", value)
    }
}

struct MetricView: View {
    let title: String
    let value: String

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(title)
                .font(.caption)
                .foregroundStyle(.secondary)
            Text(value)
                .font(.system(.body, design: .monospaced).weight(.medium))
        }
    }
}

struct StatusTable: View {
    let scope: ProgressScope
    let title: String

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text(title)
                .font(.headline)

            Grid(alignment: .leading, horizontalSpacing: 18, verticalSpacing: 8) {
                GridRow {
                    Text("Status").foregroundStyle(.secondary)
                    Text("Files").foregroundStyle(.secondary)
                    Text("LOC").foregroundStyle(.secondary)
                }
                ForEach(PortStatus.allCases) { status in
                    let value = scope.byStatus[status] ?? StatusBreakdown(files: 0, lines: 0)
                    GridRow {
                        Text(status.displayName)
                        Text("\(value.files)").monospacedDigit()
                        Text("\(value.lines)").monospacedDigit()
                    }
                }
            }
            .font(.callout)
        }
    }
}

struct IncompleteList: View {
    let entries: [ManifestEntry]

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Incomplete Files")
                .font(.headline)

            Table(entries) {
                TableColumn("Status") { entry in
                    Text(entry.status.displayName)
                }
                TableColumn("Source") { entry in
                    Text(entry.source)
                        .lineLimit(1)
                        .truncationMode(.middle)
                }
                TableColumn("Target") { entry in
                    Text(entry.target)
                        .lineLimit(1)
                        .truncationMode(.middle)
                }
            }
            .frame(minHeight: 260)
        }
    }
}
