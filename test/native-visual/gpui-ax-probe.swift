// Read a running GPUI proof window through macOS AXUIElement without activating it.
import ApplicationServices
import Foundation

struct Element: Encodable {
    let depth: Int
    let role: String
    let subrole: String
    let title: String
    let description: String
    let name: String
    let attributes: [String]
    let value: String
    let minimum: String
    let maximum: String
    let orientation: String
    let enabled: String
    let expanded: String
    let selected: String
}

struct Result: Encodable {
    let status: String
    let reason: String
    let elements: [Element]
}

let maximumTreeDepth = 80

func write(_ result: Result, exitCode: Int32 = 0) -> Never {
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.sortedKeys]
    if let data = try? encoder.encode(result) {
        FileHandle.standardOutput.write(data)
        FileHandle.standardOutput.write(Data([10]))
    }
    exit(exitCode)
}

let args = CommandLine.arguments
guard args.count == 3,
      let pid = pid_t(args[1]),
      !args[2].isEmpty else {
    FileHandle.standardError.write("usage: gpui-ax-probe <pid> <proof-window-title>\n".data(using: .utf8)!)
    exit(2)
}
let expectedWindowTitle = args[2]

guard AXIsProcessTrusted() else {
    write(Result(status: "error", reason: "AXIsProcessTrusted returned false", elements: []), exitCode: 3)
}

func attribute(_ element: AXUIElement, _ name: String) -> CFTypeRef? {
    var value: CFTypeRef?
    let status = AXUIElementCopyAttributeValue(element, name as CFString, &value)
    return status == .success ? value : nil
}

func string(_ element: AXUIElement, _ name: String) -> String {
    guard let raw = attribute(element, name) else { return "" }
    if let value = raw as? String { return value }
    if let value = raw as? NSNumber { return value.stringValue }
    return ""
}

func attributeNames(_ element: AXUIElement) -> [String] {
    var raw: CFArray?
    guard AXUIElementCopyAttributeNames(element, &raw) == .success else { return [] }
    return raw as? [String] ?? []
}

var elements: [Element] = []
var visited: [CFHashCode: [AXUIElement]] = [:]
var walkIssue: String?

func describe(_ element: AXUIElement, depth: Int) -> String {
    let role = string(element, kAXRoleAttribute as String)
    let subrole = string(element, kAXSubroleAttribute as String)
    let title = string(element, kAXTitleAttribute as String)
    let description = string(element, kAXDescriptionAttribute as String)
    let name = title.isEmpty ? description : title
    return "depth=\(depth) cfHash=\(CFHash(element)) role=\(String(reflecting: role)) subrole=\(String(reflecting: subrole)) name=\(String(reflecting: name))"
}

func walk(_ element: AXUIElement, depth: Int) {
    guard walkIssue == nil else { return }
    guard depth <= maximumTreeDepth else {
        walkIssue = "depth limit \(maximumTreeDepth) exceeded at \(describe(element, depth: depth))"
        return
    }

    let identityHash = CFHash(element)
    if visited[identityHash]?.contains(where: { CFEqual($0, element) }) == true {
        walkIssue = "cycle or repeated element at \(describe(element, depth: depth))"
        return
    }
    visited[identityHash, default: []].append(element)

    let title = string(element, kAXTitleAttribute as String)
    let description = string(element, kAXDescriptionAttribute as String)
    elements.append(Element(
        depth: depth,
        role: string(element, kAXRoleAttribute as String),
        subrole: string(element, kAXSubroleAttribute as String),
        title: title,
        description: description,
        name: title.isEmpty ? description : title,
        attributes: attributeNames(element),
        value: string(element, kAXValueAttribute as String),
        minimum: string(element, kAXMinValueAttribute as String),
        maximum: string(element, kAXMaxValueAttribute as String),
        orientation: string(element, kAXOrientationAttribute as String),
        enabled: string(element, kAXEnabledAttribute as String),
        expanded: string(element, kAXExpandedAttribute as String),
        selected: string(element, kAXSelectedAttribute as String)
    ))

    guard let children = attribute(element, kAXChildrenAttribute as String) as? [AXUIElement]
    else { return }
    for child in children {
        walk(child, depth: depth + 1)
        if walkIssue != nil { return }
    }
}

let application = AXUIElementCreateApplication(pid)
guard let windows = attribute(application, kAXWindowsAttribute as String) as? [AXUIElement] else {
    write(Result(status: "not_ready", reason: "AXWindows is not available for pid \(pid)", elements: []))
}
guard let proofWindow = windows.first(where: {
    string($0, kAXTitleAttribute as String) == expectedWindowTitle
}) else {
    let titles = windows.map { string($0, kAXTitleAttribute as String) }
    let observedTitles = titles.map { String(reflecting: $0) }.joined(separator: ", ")
    write(Result(
        status: "not_ready",
        reason: "proof window title \(String(reflecting: expectedWindowTitle)) is not in AXWindows; observed titles: \(observedTitles)",
        elements: []
    ))
}
walk(proofWindow, depth: 0)
if let walkIssue {
    write(Result(status: "walk_error", reason: walkIssue, elements: elements))
}

guard let root = elements.first,
      root.role == "AXWindow",
      root.name == expectedWindowTitle else {
    let observed = elements.first.map { describe(proofWindow, depth: $0.depth) } ?? "no root element"
    write(Result(
        status: "not_ready",
        reason: "expected root role=AXWindow name=\(String(reflecting: expectedWindowTitle)); observed \(observed)",
        elements: elements
    ))
}
guard elements.contains(where: { $0.name == "GPUI AX proof: Save" }) else {
    write(Result(
        status: "not_ready",
        reason: "proof window root is ready but named Poodle content is not exposed yet",
        elements: elements
    ))
}
write(Result(status: "ready", reason: "", elements: elements))
