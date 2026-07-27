// Read a running process's real macOS accessibility tree.
//
// The Jetstream accessibility work is proven by test up to the AccessKit
// boundary: `tree_update` produces the right nodes. That says nothing about
// whether macOS ever sees them. This crosses that boundary the same way a
// screen reader would — through AXUIElement — so the answer comes from the OS
// rather than from our own code agreeing with itself.
//
//   swift ax-probe.swift <pid>
//
// Prints one line per element as `depth|role|title|value`, then a summary.
//
// Two things make a run meaningless rather than failing:
//
//   - The calling process needs Accessibility permission (System Settings >
//     Privacy & Security > Accessibility). Without it every query returns
//     `apiDisabled` or an empty tree, which looks identical to "the app
//     exposes nothing". The probe checks first and says which it is.
//   - AccessKit activates lazily: it builds no tree until something asks. The
//     act of walking the tree is what triggers `request_initial_tree`, so a
//     first pass can legitimately come back thinner than a second.

import ApplicationServices
import Foundation

let args = CommandLine.arguments
guard args.count > 1, let pid = Int32(args[1]) else {
    FileHandle.standardError.write("usage: ax-probe.swift <pid>\n".data(using: .utf8)!)
    exit(2)
}

guard AXIsProcessTrusted() else {
    print("UNTRUSTED")
    print("The calling process lacks Accessibility permission, so every query")
    print("would return nothing whether or not the app exposes a tree.")
    print("Grant it in System Settings > Privacy & Security > Accessibility.")
    exit(3)
}

func copyAttribute(_ element: AXUIElement, _ attribute: String) -> CFTypeRef? {
    var value: CFTypeRef?
    let result = AXUIElementCopyAttributeValue(element, attribute as CFString, &value)
    return result == .success ? value : nil
}

func string(_ element: AXUIElement, _ attribute: String) -> String {
    guard let raw = copyAttribute(element, attribute) else { return "" }
    if let text = raw as? String { return text }
    if let number = raw as? NSNumber { return number.stringValue }
    return ""
}

var counts: [String: Int] = [:]
var named = 0
var total = 0

/// Ancestors of the element currently being walked.
///
/// The tree can genuinely contain cycles: probed before AccessKit has built it,
/// the application element reports *itself* as its own child. A depth cap alone
/// does not save you — the walk still branches at every level, so a 40-deep cap
/// on a cyclic graph explodes combinatorially and hangs.
///
/// This tracks the current *path*, not every element ever seen. A global
/// visited-set is the obvious version and it is wrong here: sibling elements
/// are not always distinct under `CFEqual`, so a global set prunes real
/// subtrees and reports an almost-empty UI. Only an element that is its own
/// ancestor is a cycle.
var ancestors: [AXUIElement] = []

func walk(_ element: AXUIElement, depth: Int) {
    if ancestors.contains(where: { CFEqual($0, element) }) {
        print("\(depth)|CYCLE||")
        return
    }
    ancestors.append(element)
    defer { ancestors.removeLast() }

    if depth > 40 {
        print("\(depth)|DEPTH-LIMIT||")
        return
    }

    let role = string(element, kAXRoleAttribute as String)
    let title = string(element, kAXTitleAttribute as String)
    let value = string(element, kAXValueAttribute as String)
    let description = string(element, kAXDescriptionAttribute as String)
    let name = title.isEmpty ? description : title

    total += 1
    counts[role.isEmpty ? "(none)" : role, default: 0] += 1
    if !name.isEmpty { named += 1 }

    print("\(depth)|\(role)|\(name)|\(value)")

    guard let children = copyAttribute(element, kAXChildrenAttribute as String) as? [AXUIElement]
    else { return }
    for child in children {
        walk(child, depth: depth + 1)
    }
}

let app = AXUIElementCreateApplication(pid)
walk(app, depth: 0)

print("SUMMARY elements=\(total) named=\(named)")
for (role, count) in counts.sorted(by: { $0.value > $1.value }) {
    print("ROLE \(role) \(count)")
}
