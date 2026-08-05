//! ActionDiscoveryPanel specimen — keyboard shortcut / command discovery surface.
//!
//! Mirrors the GPUI `action_discovery` specimen coverage: grouped sections with
//! label/shortcut/badge/description items, active-item accent, loading skeleton,
//! empty / error / no-results postures, and semantic (compact / prominent)
//! presentation. Every panel is rendered by the real `js_action_discovery_panel`
//! resolving values from the token system; the bordered frame uses token colors.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_action_discovery_panel;


use poodle_specs::{
    ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem, ControlDensity,
    DiscoveryState, SemanticControlSizeRole,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border_subtle = resolve_color(theme, "color.border.subtle");

    // Bordered, size-bounded frame so every panel sits in the same surface,
    // matching the GPUI specimen's `framed` wrapper.
    let framed = move |inner: El| -> El {
        div()
            .max_w(rem_to_px(32.0))
            .max_h(rem_to_px(20.0))
            .border_1()
            .border_color(border_subtle)
            .rounded(rem_to_px(0.5))
            .p(rem_to_px(0.75))
            .overflow_hidden()
            .child(inner)
    };

    // --- Grouped actions: 7 actions across 3 groups, each with a shortcut. ---
    let grouped_spec = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "file",
            "File",
            vec![
                CommandActionItem::new("save", "Save").with_shortcut("\u{2318}S"),
                CommandActionItem::new("open-file", "Open File").with_shortcut("\u{2318}O"),
                CommandActionItem::new("close-tab", "Close Tab").with_shortcut("\u{2318}W"),
            ],
        ),
        ActionDiscoverySection::new(
            "edit",
            "Edit",
            vec![
                CommandActionItem::new("find-in-files", "Find in Files")
                    .with_shortcut("\u{21E7}\u{2318}F"),
                CommandActionItem::new("find-and-replace", "Find and Replace")
                    .with_shortcut("\u{2318}H"),
            ],
        ),
        ActionDiscoverySection::new(
            "view",
            "View",
            vec![
                CommandActionItem::new("toggle-terminal", "Toggle Terminal")
                    .with_shortcut("\u{2318}`"),
                CommandActionItem::new("toggle-sidebar", "Toggle Sidebar")
                    .with_shortcut("\u{2318}B"),
            ],
        ),
    ]);

    // --- Descriptions + badges. ---
    let described_spec = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "ci-cd",
            "CI/CD",
            vec![
                CommandActionItem::new("deploy", "Deploy to Production")
                    .with_description("Push current branch to production environment")
                    .with_badge("Dangerous"),
                CommandActionItem::new("preview", "Open Preview")
                    .with_description("Launch preview in a new tab")
                    .with_shortcut("\u{21E7}\u{2318}P"),
            ],
        ),
        ActionDiscoverySection::new(
            "tools",
            "Tools",
            vec![
                CommandActionItem::new("lint", "Run Linter").with_shortcut("\u{21E7}\u{2318}L"),
            ],
        ),
    ]);

    // --- Active item: one item highlighted via active_id. ---
    let active_spec = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "file",
            "File",
            vec![
                CommandActionItem::new("save", "Save").with_shortcut("\u{2318}S"),
                CommandActionItem::new("open-file", "Open File").with_shortcut("\u{2318}O"),
            ],
        ),
        ActionDiscoverySection::new(
            "edit",
            "Edit",
            vec![
                CommandActionItem::new("find-in-files", "Find in Files")
                    .with_shortcut("\u{21E7}\u{2318}F"),
                CommandActionItem::new("find-and-replace", "Find and Replace")
                    .with_shortcut("\u{2318}H"),
            ],
        ),
    ])
    .with_active_id("open-file");

    let loading_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Loading);
    let empty_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Empty);
    let error_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Error);
    let no_results_spec =
        ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::NoResults);

    // Shared two-section data for the semantic presentation row.
    let semantic_sections = || {
        vec![
            ActionDiscoverySection::new(
                "file",
                "File",
                vec![CommandActionItem::new("save", "Save").with_shortcut("\u{2318}S")],
            ),
            ActionDiscoverySection::new(
                "release",
                "Release",
                vec![CommandActionItem::new("deploy", "Deploy").with_badge("Dangerous")],
            ),
        ]
    };

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Grouped actions",
            secondary,
            framed(js_action_discovery_panel(&grouped_spec, theme)),
        ))
        .child(group(
            "With descriptions and badges",
            secondary,
            framed(js_action_discovery_panel(&described_spec, theme)),
        ))
        .child(group(
            "Active item",
            secondary,
            framed(js_action_discovery_panel(&active_spec, theme)),
        ))
        .child(group(
            "Loading (skeleton)",
            secondary,
            framed(js_action_discovery_panel(&loading_spec, theme)),
        ))
        .child(group(
            "Empty state",
            secondary,
            framed(js_action_discovery_panel(&empty_spec, theme)),
        ))
        .child(group(
            "Error state",
            secondary,
            framed(js_action_discovery_panel(&error_spec, theme)),
        ))
        .child(group(
            "No results",
            secondary,
            framed(js_action_discovery_panel(&no_results_spec, theme)),
        ))
        .child(group(
            "Semantic presentation",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(framed(js_action_discovery_panel(
                    &ActionDiscoveryPanelSpec::new(semantic_sections())
                        .with_density(ControlDensity::Compact),
                    theme,
                )))
                .child(framed(js_action_discovery_panel(
                    &ActionDiscoveryPanelSpec::new(semantic_sections())
                        .with_size_role(SemanticControlSizeRole::Prominent),
                    theme,
                ))),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
