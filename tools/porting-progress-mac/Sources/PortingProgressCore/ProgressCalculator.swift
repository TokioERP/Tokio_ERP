import Foundation

public struct Manifest: Decodable {
    public let entries: [ManifestEntry]
}

public struct ManifestEntry: Decodable, Identifiable, Equatable {
    public var id: String { source }
    public let source: String
    public let target: String
    public let status: PortStatus
}

public enum PortStatus: String, CaseIterable, Codable, Identifiable {
    case notStarted = "not_started"
    case mapped
    case ported
    case parityTested = "parity_tested"
    case externalKept = "external_kept"

    public var id: String { rawValue }

    public var displayName: String {
        switch self {
        case .notStarted: "Not Started"
        case .mapped: "Mapped"
        case .ported: "Ported"
        case .parityTested: "Parity Tested"
        case .externalKept: "External Kept"
        }
    }

    public var isComplete: Bool {
        self == .ported || self == .parityTested
    }
}

public struct StatusBreakdown: Equatable {
    public var files: Int
    public var lines: Int

    public init(files: Int, lines: Int) {
        self.files = files
        self.lines = lines
    }
}

public struct ProgressScope: Equatable {
    public let title: String
    public let files: Int
    public let lines: Int
    public let byStatus: [PortStatus: StatusBreakdown]

    public var completeFiles: Int {
        total(for: [.ported, .parityTested], keyPath: \.files)
    }

    public var completeLines: Int {
        total(for: [.ported, .parityTested], keyPath: \.lines)
    }

    public var parityFiles: Int {
        total(for: [.parityTested], keyPath: \.files)
    }

    public var parityLines: Int {
        total(for: [.parityTested], keyPath: \.lines)
    }

    public var completeFilePercent: Double {
        percent(Double(completeFiles), Double(files))
    }

    public var completeLinePercent: Double {
        percent(Double(completeLines), Double(lines))
    }

    public var parityFilePercent: Double {
        percent(Double(parityFiles), Double(files))
    }

    public var parityLinePercent: Double {
        percent(Double(parityLines), Double(lines))
    }

    private func total(
        for statuses: [PortStatus],
        keyPath: KeyPath<StatusBreakdown, Int>
    ) -> Int {
        statuses.reduce(0) { total, status in
            total + (byStatus[status]?[keyPath: keyPath] ?? 0)
        }
    }
}

public struct ProgressReport: Equatable {
    public let repoRoot: URL
    public let sourceRoot: URL
    public let generatedAt: Date
    public let overall: ProgressScope
    public let accounts: ProgressScope
    public let entries: [ManifestEntry]

    public var incompleteEntries: [ManifestEntry] {
        entries.filter { !$0.status.isComplete }
    }

    public var completeEntries: [ManifestEntry] {
        entries.filter { $0.status.isComplete }
    }
}

public enum ProgressCalculatorError: Error, Equatable, LocalizedError {
    case manifestNotFound(URL)
    case sourceRootNotFound(URL)
    case cannotFindRepositoryRoot(URL)

    public var errorDescription: String? {
        switch self {
        case .manifestNotFound(let url):
            "Manifest not found at \(url.path)"
        case .sourceRootNotFound(let url):
            "ERPNext source root not found at \(url.path)"
        case .cannotFindRepositoryRoot(let url):
            "Could not find porting_manifest.json from \(url.path)"
        }
    }
}

public struct ProgressCalculator {
    public let fileManager: FileManager

    public init(fileManager: FileManager = .default) {
        self.fileManager = fileManager
    }

    public func report(
        repoRoot: URL,
        sourceRoot: URL? = nil,
        now: Date = Date()
    ) throws -> ProgressReport {
        let manifestURL = repoRoot.appendingPathComponent("porting_manifest.json")
        guard fileManager.fileExists(atPath: manifestURL.path) else {
            throw ProgressCalculatorError.manifestNotFound(manifestURL)
        }

        let resolvedSourceRoot = sourceRoot ?? repoRoot
            .deletingLastPathComponent()
            .appendingPathComponent("erpnext/apps/erpnext/erpnext")
        guard fileManager.fileExists(atPath: resolvedSourceRoot.path) else {
            throw ProgressCalculatorError.sourceRootNotFound(resolvedSourceRoot)
        }

        let manifestData = try Data(contentsOf: manifestURL)
        let manifest = try JSONDecoder().decode(Manifest.self, from: manifestData)
        let pythonEntries = manifest.entries.filter { $0.source.hasSuffix(".py") }

        let overall = scope(
            title: "Overall ERPNext",
            entries: pythonEntries,
            sourceRoot: resolvedSourceRoot
        )
        let accountsEntries = pythonEntries.filter { $0.source.hasPrefix("accounts/") }
        let accounts = scope(
            title: "Accounts",
            entries: accountsEntries,
            sourceRoot: resolvedSourceRoot
        )

        return ProgressReport(
            repoRoot: repoRoot,
            sourceRoot: resolvedSourceRoot,
            generatedAt: now,
            overall: overall,
            accounts: accounts,
            entries: pythonEntries
        )
    }

    public func findRepositoryRoot(startingAt start: URL) throws -> URL {
        var current = start.standardizedFileURL
        while true {
            let candidate = current.appendingPathComponent("porting_manifest.json")
            if fileManager.fileExists(atPath: candidate.path) {
                return current
            }

            let parent = current.deletingLastPathComponent()
            if parent.path == current.path {
                throw ProgressCalculatorError.cannotFindRepositoryRoot(start)
            }
            current = parent
        }
    }

    private func scope(
        title: String,
        entries: [ManifestEntry],
        sourceRoot: URL
    ) -> ProgressScope {
        var totalLines = 0
        var byStatus: [PortStatus: StatusBreakdown] = [:]

        for entry in entries {
            let lines = lineCount(
                sourceRoot.appendingPathComponent(entry.source)
            )
            totalLines += lines
            var breakdown = byStatus[entry.status] ?? StatusBreakdown(files: 0, lines: 0)
            breakdown.files += 1
            breakdown.lines += lines
            byStatus[entry.status] = breakdown
        }

        return ProgressScope(
            title: title,
            files: entries.count,
            lines: totalLines,
            byStatus: byStatus
        )
    }

    private func lineCount(_ url: URL) -> Int {
        guard let data = try? Data(contentsOf: url) else {
            return 0
        }
        return data.reduce(0) { count, byte in
            count + (byte == 10 ? 1 : 0)
        }
    }
}

private func percent(_ numerator: Double, _ denominator: Double) -> Double {
    guard denominator > 0 else { return 0 }
    return numerator * 100 / denominator
}
