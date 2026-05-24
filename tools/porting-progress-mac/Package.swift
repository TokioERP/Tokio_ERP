// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "PortingProgress",
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "PortingProgressCore", targets: ["PortingProgressCore"]),
        .executable(name: "PortingProgressApp", targets: ["PortingProgressApp"]),
    ],
    targets: [
        .target(name: "PortingProgressCore"),
        .executableTarget(
            name: "PortingProgressApp",
            dependencies: ["PortingProgressCore"]
        ),
        .testTarget(
            name: "PortingProgressCoreTests",
            dependencies: ["PortingProgressCore"]
        ),
    ]
)
