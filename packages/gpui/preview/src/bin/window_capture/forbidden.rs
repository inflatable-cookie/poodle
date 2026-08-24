//! The activation boundary, pinned as a source check.
//!
//! The capture contract's central claim — "opening the capture window does not
//! change the frontmost application or key window" — cannot be proved by
//! running this binary in a worker, because running it needs a window server
//! and Screen Recording permission. It CAN be proved structurally: if no
//! focus-taking API appears anywhere in the capture target's code, no code
//! path can take focus.
//!
//! So this module reads its sibling sources back and fails if any code line
//! names an activation, window-raising, desktop-capture, region-capture, or
//! System Events call. Adding one is then a test failure at the moment it is
//! written, not a surprise on the operator's screen. Comment lines are
//! skipped on purpose: this file and its siblings have to be able to say what
//! they refuse to do.
//!
//! Runtime evidence still exists and is stronger where it applies: every
//! receipt carries the run's own frontmost-application samples, and a run that
//! observed a change publishes nothing. This check covers the paths a single
//! run would not exercise.

/// Every source file that makes up the capture target.
pub const CAPTURE_SOURCES: &[(&str, &str)] = &[
    ("window_capture.rs", include_str!("../window_capture.rs")),
    ("window_capture/transport.rs", include_str!("transport.rs")),
    (
        "window_capture/fixture_capture.rs",
        include_str!("fixture_capture.rs"),
    ),
    (
        "window_capture/focus_evidence.rs",
        include_str!("focus_evidence.rs"),
    ),
    (
        "window_capture/inset_evidence.rs",
        include_str!("inset_evidence.rs"),
    ),
    ("window_capture/inventory.rs", include_str!("inventory.rs")),
];

/// Fragments that must not appear in any capture code line, with the reason.
pub const FORBIDDEN: &[(&str, &str)] = &[
    ("cx.activate(", "App::activate brings the application to the foreground"),
    (".activate_window(", "Window::activate_window raises and focuses the window"),
    ("makeKeyAndOrderFront", "AppKit window activation"),
    ("orderFrontRegardless", "AppKit window raising"),
    ("activateIgnoringOtherApps", "AppKit application activation"),
    ("osascript", "System Events scripting is the old focus-stealing path"),
    ("\"-R\"", "screencapture -R captures a screen region, not one window"),
    ("\"-D\"", "screencapture -D captures a whole display"),
    ("\"-C\"", "screencapture -C captures the cursor into the frame"),
];

/// Words that would make a receipt field, constant, or diagnostic claim
/// capabilities stock crates.io GPUI 0.2.2 does not have. Naming the
/// transport dishonestly is the exact defect g16.005 exists to correct.
pub const FORBIDDEN_CLAIMS: &[(&str, &str)] = &[
    ("offscreen", "this transport uses a real window"),
    ("metal-headless", "there is no headless renderer on the published crate"),
    ("render_to_image", "window-level readback is not a published API"),
    ("HeadlessAppContext", "the headless app context is not a published API"),
    ("gpui_platform", "the platform crate is unpublished"),
];

/// A single violation: which file, which fragment, and why it is refused.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    pub file: &'static str,
    pub line: usize,
    pub fragment: &'static str,
    pub reason: &'static str,
}

/// Scan one source for forbidden fragments and false capability claims,
/// ignoring comment lines.
pub fn scan(file: &'static str, source: &'static str) -> Vec<Violation> {
    let mut found = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue;
        }
        for (fragment, reason) in FORBIDDEN.iter().chain(FORBIDDEN_CLAIMS) {
            if trimmed.contains(fragment) {
                found.push(Violation {
                    file,
                    line: index + 1,
                    fragment,
                    reason,
                });
            }
        }
    }
    found
}

/// Every violation across the whole capture target.
pub fn violations() -> Vec<Violation> {
    CAPTURE_SOURCES
        .iter()
        .flat_map(|(name, source)| scan(name, source))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The focus and honesty contracts, as far as source can carry them.
    #[test]
    fn no_capture_code_names_an_activation_desktop_capture_or_false_claim() {
        let violations = violations();
        assert!(
            violations.is_empty(),
            "the capture target must contain no focus-taking, desktop-capture, or \
             offscreen-claiming code: {violations:#?}"
        );
    }

    /// The check must be able to fail. A list that matches nothing would pass
    /// forever and prove nothing, which is how g15.044's first verifier
    /// reported success while its claims had drifted.
    #[test]
    fn the_boundary_check_detects_planted_violations() {
        let planted = concat!(
            "// a comment naming cx.activate( must not trip the check\n",
            "fn raise(cx: &mut App) { cx.activate(true); }\n",
            "let out = Command::new(\"osascript\");\n",
            "const RENDERER: &str = \"metal-headless\";\n",
        );
        let found = scan("planted.rs", planted);
        let fragments: Vec<&str> = found.iter().map(|v| v.fragment).collect();
        assert_eq!(
            fragments,
            vec!["cx.activate(", "osascript", "metal-headless"],
            "each planted line must be caught exactly once, and the comment ignored"
        );
        assert_eq!(found[0].line, 2, "line numbers must point at the offender");
    }

    /// The window capture path must be exactly one window id, with the
    /// window's drop shadow excluded so the captured frame is the content
    /// rect the receipt claims.
    #[test]
    fn the_capture_command_targets_one_window_id_without_its_shadow() {
        let transport = include_str!("transport.rs");
        assert!(transport.contains("\"-l\""), "capture must be in window mode");
        assert!(transport.contains("\"-o\""), "the drop shadow must be excluded");
        assert!(
            transport.contains("focus: false"),
            "the capture window must never be opened focused"
        );
    }
}
