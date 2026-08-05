//! CommandPalette specimen — command palettes with actions.
//!
//! Mirrors the GPUI specimen's full contract coverage: a grouped palette with
//! shortcuts, an active-item treatment, the five §6 open-state postures
//! (ready / loading / error / empty / no-results), and Sizes / Densities
//! sweeps. Every palette is a real `js_command_palette` over a
//! `CommandPaletteSpec` — no hand-rolled boxes.
//!
//! `js_command_palette` renders the full modal as an `.overlay()` backdrop
//! (`absolute().inset_0()`), so every open palette is wrapped in its own
//! `relative().min_h(...)` container that confines the scrim to that group's
//! region (the same posture as the GPUI specimen's `div().relative()`).

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_command_palette;

use poodle_specs::{
    CommandActionItem, CommandPaletteSpec, ControlDensity, ControlSize, DiscoveryState,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Grouped actions with shortcuts (File / Edit / View) — mirrors GPUI.
    let actions = vec![
        CommandActionItem::new("save", "Save")
            .with_group("File")
            .with_shortcut("\u{2318}S"),
        CommandActionItem::new("open-file", "Open File")
            .with_group("File")
            .with_shortcut("\u{2318}O"),
        CommandActionItem::new("close-tab", "Close Tab")
            .with_group("File")
            .with_shortcut("\u{2318}W"),
        CommandActionItem::new("find-in-files", "Find in Files")
            .with_group("Edit")
            .with_shortcut("\u{21E7}\u{2318}F"),
        CommandActionItem::new("find-and-replace", "Find and Replace")
            .with_group("Edit")
            .with_shortcut("\u{2318}H"),
        CommandActionItem::new("toggle-terminal", "Toggle Terminal")
            .with_group("View")
            .with_shortcut("\u{2318}`"),
        CommandActionItem::new("toggle-sidebar", "Toggle Sidebar")
            .with_group("View")
            .with_shortcut("\u{2318}B"),
    ];

    let mut root = div().flex_col().gap(24.0);

    // ── Grouped palette (title + hint pill + grouped results + shortcuts) ──
    root = root.child(group_open(
        "Command palette (grouped, shortcuts)",
        secondary,
        js_command_palette(
            &CommandPaletteSpec::new(actions.clone())
                .with_open(true)
                .with_title("Command palette")
                .with_invocation_hint("\u{2318}K"),
            theme,
        ),
    ));

    // ── Active item highlighted (accent tint + accent text) ──
    root = root.child(group_open(
        "Active item (Find in Files)",
        secondary,
        js_command_palette(
            &CommandPaletteSpec::new(actions.clone())
                .with_title("Command palette")
                .with_invocation_hint("\u{2318}K")
                .with_active_action_id("find-in-files"),
            theme,
        ),
    ));

    // ── Compact palette (size sm + compact density) ──
    root = root.child(group_open(
        "Compact palette (sm + compact)",
        secondary,
        js_command_palette(
            &CommandPaletteSpec::new(vec![
                CommandActionItem::new("save", "Save")
                    .with_group("File")
                    .with_shortcut("\u{2318}S"),
                CommandActionItem::new("open", "Open File")
                    .with_group("File")
                    .with_shortcut("\u{2318}O"),
            ])
            .with_title("Quick actions")
            .with_size(ControlSize::Sm)
            .with_density(ControlDensity::Compact)
            .with_invocation_hint("Cmd+K"),
            theme,
        ),
    ));

    // ── Open states (contract §6: ready / loading / error / empty / no-results) ──
    root = root.child(label("Open states").text_color(secondary).text_size(11.0));
    root = root.child(group_open(
        "Open / ready",
        secondary,
        js_command_palette(&open_state_spec(actions.clone(), "", DiscoveryState::Ready), theme),
    ));
    root = root.child(group_open(
        "Open / loading",
        secondary,
        js_command_palette(&open_state_spec(actions.clone(), "", DiscoveryState::Loading), theme),
    ));
    root = root.child(group_open(
        "Open / no-results",
        secondary,
        js_command_palette(
            &open_state_spec(actions.clone(), "zxqv", DiscoveryState::NoResults),
            theme,
        ),
    ));
    root = root.child(group_open(
        "Open / empty",
        secondary,
        js_command_palette(&open_state_spec(Vec::new(), "", DiscoveryState::Empty), theme),
    ));
    root = root.child(group_open(
        "Open / error",
        secondary,
        js_command_palette(&open_state_spec(actions.clone(), "", DiscoveryState::Error), theme),
    ));

    // ── Sizes (xs–xl): one open palette per intrinsic size ──
    root = root.child(label("Sizes").text_color(secondary).text_size(11.0));
    for (name, size) in [
        ("XS", ControlSize::Xs),
        ("SM", ControlSize::Sm),
        ("MD", ControlSize::Md),
        ("LG", ControlSize::Lg),
        ("XL", ControlSize::Xl),
    ] {
        root = root.child(group_open(
            name,
            secondary,
            js_command_palette(
                &CommandPaletteSpec::new(actions.clone())
                    .with_title(format!("{name} command palette"))
                    .with_invocation_hint("\u{2318}K")
                    .with_size(size),
                theme,
            ),
        ));
    }

    // ── Densities: one open palette per density ──
    root = root.child(label("Densities").text_color(secondary).text_size(11.0));
    for (name, density) in [
        ("Compact", ControlDensity::Compact),
        ("Default", ControlDensity::Default),
        ("Comfortable", ControlDensity::Comfortable),
    ] {
        root = root.child(group_open(
            name,
            secondary,
            js_command_palette(
                &CommandPaletteSpec::new(actions.clone())
                    .with_title(format!("{name} command palette"))
                    .with_invocation_hint("\u{2318}K")
                    .with_density(density),
                theme,
            ),
        ));
    }

    root
}

/// Spec for an open palette demonstrating a single `DiscoveryState`.
fn open_state_spec(
    actions: Vec<CommandActionItem>,
    query: &str,
    state: DiscoveryState,
) -> CommandPaletteSpec {
    let mut spec = CommandPaletteSpec::new(actions)
        .with_title("Command palette")
        .with_invocation_hint("\u{2318}K")
        .with_state(state);
    if !query.is_empty() {
        spec = spec.with_query(query);
    }
    spec
}

/// Labeled group wrapping an open palette. The palette renders an `.overlay()`
/// scrim that fills the nearest positioned ancestor, so the content area is
/// `relative()` with a `min_h` to confine and reveal the centered modal.
fn group_open(title: &str, text_secondary: ColorValue, palette: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(div().relative().min_h(360.0).child(palette))
}
