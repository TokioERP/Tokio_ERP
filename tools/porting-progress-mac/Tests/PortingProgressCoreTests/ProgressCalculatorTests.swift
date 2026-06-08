import XCTest
@testable import PortingProgressCore

final class ProgressCalculatorTests: XCTestCase {
    func testReportCalculatesOverallAndStockProgressFromManifestAndSourceLines() throws {
        let root = try temporaryDirectory()
        let sourceRoot = root.appendingPathComponent("erpnext/apps/erpnext/erpnext")
        try FileManager.default.createDirectory(
            at: sourceRoot.appendingPathComponent("accounts/custom"),
            withIntermediateDirectories: true
        )
        try FileManager.default.createDirectory(
            at: sourceRoot.appendingPathComponent("stock"),
            withIntermediateDirectories: true
        )

        try write("a\nb\nc\n", to: sourceRoot.appendingPathComponent("accounts/custom/address.py"))
        try write("a\nb\n", to: sourceRoot.appendingPathComponent("accounts/open.py"))
        try write("a\n", to: sourceRoot.appendingPathComponent("stock/item.py"))

        let repoRoot = root.appendingPathComponent("tokio_erp")
        try FileManager.default.createDirectory(at: repoRoot, withIntermediateDirectories: true)
        try write(
            """
            {
              "entries": [
                {
                  "source": "accounts/custom/address.py",
                  "target": "accounts/custom/address.rs",
                  "status": "parity_tested"
                },
                {
                  "source": "accounts/open.py",
                  "target": "accounts/open.rs",
                  "status": "not_started"
                },
                {
                  "source": "stock/item.py",
                  "target": "stock/item.rs",
                  "status": "ported"
                },
                {
                  "source": "selling/customer.json",
                  "target": "selling/customer.rs",
                  "status": "not_started"
                }
              ]
            }
            """,
            to: repoRoot.appendingPathComponent("porting_manifest.json")
        )

        let report = try ProgressCalculator().report(
            repoRoot: repoRoot,
            sourceRoot: sourceRoot,
            now: Date(timeIntervalSince1970: 0)
        )

        XCTAssertEqual(report.overall.files, 3)
        XCTAssertEqual(report.overall.lines, 6)
        XCTAssertEqual(report.overall.completeFiles, 2)
        XCTAssertEqual(report.overall.completeLines, 4)
        XCTAssertEqual(report.overall.parityFiles, 1)
        XCTAssertEqual(report.stock.files, 1)
        XCTAssertEqual(report.stock.lines, 1)
        XCTAssertEqual(report.stock.completeFiles, 1)
        XCTAssertEqual(report.stock.completeLines, 1)
        XCTAssertEqual(report.incompleteEntries.map(\.source), ["accounts/open.py"])
    }

    func testFindRepositoryRootWalksUpUntilManifestExists() throws {
        let root = try temporaryDirectory()
        let repoRoot = root.appendingPathComponent("tokio_erp")
        let child = repoRoot.appendingPathComponent("tools/porting-progress-mac")
        try FileManager.default.createDirectory(at: child, withIntermediateDirectories: true)
        try write("{\"entries\":[]}", to: repoRoot.appendingPathComponent("porting_manifest.json"))

        let found = try ProgressCalculator().findRepositoryRoot(startingAt: child)

        XCTAssertEqual(found.standardizedFileURL.path, repoRoot.standardizedFileURL.path)
    }

    private func temporaryDirectory() throws -> URL {
        let url = FileManager.default.temporaryDirectory
            .appendingPathComponent(UUID().uuidString)
        try FileManager.default.createDirectory(at: url, withIntermediateDirectories: true)
        return url
    }

    private func write(_ content: String, to url: URL) throws {
        try FileManager.default.createDirectory(
            at: url.deletingLastPathComponent(),
            withIntermediateDirectories: true
        )
        try content.write(to: url, atomically: true, encoding: .utf8)
    }
}
