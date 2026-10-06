// Read a running GPUI proof window through macOS AXUIElement without activating it.
import ApplicationServices
import Foundation

struct Element: Encodable {
    let depth: Int
    let role: String
    let subrole: String
    let name: String
    let value: String
    let minimum: String
    let maximum: String
    let enabled: String
    let expanded: String
    let selected: String
}

struct Result: Encodable {
    let status: String
    let reason: String
    let elements: [Element]
}

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
guard args.count == 2, let pid = pid_t(args[1]) else {
    FileHandle.standardError.write("usage: gpui-ax-probe <pid>\n".data(using: .utf8)!)
    exit(2)
}

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

var ancestors: [AXUIElement] = []
var elements: [Element] = []
var encounteredCycle = false
var hitDepthLimit = false

func walk(_ element: AXUIElement, depth: Int) {
    if ancestors.contains(where: { CFEqual($0, element) }) {
        encounteredCycle = true
        return
    }
    guard depth <= 40 else {
        hitDepthLimit = true
        return
    }

    ancestors.append(element)
    defer { ancestors.removeLast() }

    let title = string(element, kAXTitleAttribute as String)
    let description = string(element, kAXDescriptionAttribute as String)
    elements.append(Element(
        depth: depth,
        role: string(element, kAXRoleAttribute as String),
        subrole: string(element, kAXSubroleAttribute as String),
        name: title.isEmpty ? description : title,
        value: string(element, kAXValueAttribute as String),
        minimum: string(element, kAXMinValueAttribute as String),
        maximum: string(element, kAXMaxValueAttribute as String),
        enabled: string(element, kAXEnabledAttribute as String),
        expanded: string(element, kAXExpandedAttribute as String),
        selected: string(element, kAXSelectedAttribute as String)
    ))

    guard let children = attribute(element, kAXChildrenAttribute as String) as? [AXUIElement]
    else { return }
    for child in children {
        walk(child, depth: depth + 1)
    }
}

walk(AXUIElementCreateApplication(pid), depth: 0)
if encounteredCycle || hitDepthLimit {
    write(Result(status: "not_ready", reason: "tree walk encountered a cycle or depth limit", elements: elements))
}
guard elements.contains(where: { $0.role == "AXWindow" }),
      elements.contains(where: { $0.depth >= 3 && !$0.name.isEmpty }) else {
    write(Result(status: "not_ready", reason: "window content is not exposed yet", elements: elements))
}
write(Result(status: "ready", reason: "", elements: elements))
