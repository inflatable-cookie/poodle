#!/usr/bin/env swift
//
// capture-window.swift — Capture a macOS window by owner name to PNG.
//
// Usage:
//   swift capture-window.swift [owner_substring] [output_path]
//
// Defaults:
//   owner_substring = "poodle-preview"
//   output_path     = "/tmp/gpui-capture.png"
//
// The script finds the first on-screen normal window (layer 0) whose
// owner name contains the given substring, brings it to front,
// captures the screen region, and saves a PNG.

import CoreGraphics
import Foundation

struct WindowInfo {
    let id: Int
    let owner: String
    let x: Int
    let y: Int
    let width: Int
    let height: Int
}

func findWindow(ownerContains: String) -> WindowInfo? {
    let windowList = CGWindowListCopyWindowInfo([.optionAll], kCGNullWindowID)! as NSArray
    var best: WindowInfo? = nil
    var bestArea = 0
    for w in windowList {
        let d = w as! NSDictionary
        let name = d["kCGWindowOwnerName"] as? String ?? ""
        let title = d["kCGWindowName"] as? String ?? ""
        let layer = d["kCGWindowLayer"] as? Int ?? -1
        let needle = ownerContains.lowercased()
        let ownerMatch = name.lowercased().contains(needle)
        let titleMatch = title.lowercased() == needle  // exact title match to avoid "Poodle" matching "Poodle Docs Preview"
        if layer == 0 && (ownerMatch || titleMatch) {
            if let bounds = d["kCGWindowBounds"] as? NSDictionary {
                let x = bounds["X"] as? Int ?? 0
                let y = bounds["Y"] as? Int ?? 0
                let w = bounds["Width"] as? Int ?? 0
                let h = bounds["Height"] as? Int ?? 0
                let area = w * h
                // Prefer exact title matches over owner-substring matches
                let score = titleMatch ? area + 10_000_000 : area
                if w > 100 && h > 100 && score > bestArea {
                    let wid = d["kCGWindowNumber"] as? Int ?? 0
                    best = WindowInfo(id: wid, owner: name, x: x, y: y, width: w, height: h)
                    bestArea = score
                }
            }
        }
    }
    return best
}

func bringToFront(processName: String) {
    let script = """
    tell application "System Events"
        set frontmost of (first process whose name is "\(processName)") to true
    end tell
    """
    let proc = Process()
    proc.executableURL = URL(fileURLWithPath: "/usr/bin/osascript")
    proc.arguments = ["-e", script]
    try? proc.run()
    proc.waitUntilExit()
    // Brief pause for window manager
    Thread.sleep(forTimeInterval: 0.3)
}

func captureRegion(x: Int, y: Int, width: Int, height: Int, outputPath: String) -> Bool {
    let proc = Process()
    proc.executableURL = URL(fileURLWithPath: "/usr/sbin/screencapture")
    proc.arguments = ["-R", "\(x),\(y),\(width),\(height)", "-x", "-o", outputPath]
    try? proc.run()
    proc.waitUntilExit()
    return proc.terminationStatus == 0
}

// --- Main ---

let args = CommandLine.arguments
let ownerSubstring = args.count > 1 ? args[1] : "poodle-preview"
let outputPath = args.count > 2 ? args[2] : "/tmp/gpui-capture.png"

guard let win = findWindow(ownerContains: ownerSubstring) else {
    fputs("Error: No window found containing '\(ownerSubstring)'\n", stderr)
    exit(1)
}

bringToFront(processName: win.owner)

if captureRegion(x: win.x, y: win.y, width: win.width, height: win.height, outputPath: outputPath) {
    print("\(outputPath) [\(win.width)x\(win.height)]")
} else {
    fputs("Error: screencapture failed\n", stderr)
    exit(1)
}
