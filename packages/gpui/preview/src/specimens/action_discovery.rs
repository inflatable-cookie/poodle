use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{ActionDiscoveryPanel, Eyebrow};
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem, DiscoveryState,
};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

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
            vec![CommandActionItem::new("lint", "Run Linter").with_shortcut("\u{21E7}\u{2318}L")],
        ),
    ]);

    let empty_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Empty);

    // Active-item state: same grouped data, with one item highlighted.
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
    let error_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Error);
    let no_results_spec =
        ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::NoResults);

    // Shared frame wrapper so every panel sits in the same bordered surface.
    let framed = |inner: ActionDiscoveryPanel| {
        div()
            .max_w(px(512.0))
            .max_h(px(320.0))
            .border_1()
            .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
            .rounded(px(8.0))
            .p(px(12.0))
            .overflow_hidden()
            .child(inner)
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Grouped actions"),
                    theme,
                ))
                .child(
                    div()
                        .max_w(px(512.0))
                        .max_h(px(320.0))
                        .border_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .rounded(px(8.0))
                        .p(px(12.0))
                        .overflow_hidden()
                        .child(
                            ActionDiscoveryPanel::from_spec(grouped_spec, theme)
                                .with_id("action-disc-grouped"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With descriptions and badges"),
                    theme,
                ))
                .child(
                    div()
                        .max_w(px(512.0))
                        .max_h(px(320.0))
                        .border_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .rounded(px(8.0))
                        .p(px(12.0))
                        .overflow_hidden()
                        .child(
                            ActionDiscoveryPanel::from_spec(described_spec, theme)
                                .with_id("action-disc-described"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Active item"),
                    theme,
                ))
                .child(framed(
                    ActionDiscoveryPanel::from_spec(active_spec, theme)
                        .with_id("action-disc-active"),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading (skeleton)"),
                    theme,
                ))
                .child(framed(
                    ActionDiscoveryPanel::from_spec(loading_spec, theme)
                        .with_id("action-disc-loading"),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty state"),
                    theme,
                ))
                .child(framed(
                    ActionDiscoveryPanel::from_spec(empty_spec, theme)
                        .with_id("action-disc-empty"),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Error state"),
                    theme,
                ))
                .child(framed(
                    ActionDiscoveryPanel::from_spec(error_spec, theme)
                        .with_id("action-disc-error"),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("No results"),
                    theme,
                ))
                .child(framed(
                    ActionDiscoveryPanel::from_spec(no_results_spec, theme)
                        .with_id("action-disc-no-results"),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .grid()
                        .gap(px(12.0))
                        .child(
                            div()
                                .max_w(px(512.0))
                                .border_1()
                                .border_color(color_to_hsla(
                                    theme.resolve_color("color.border.subtle"),
                                ))
                                .rounded(px(8.0))
                                .p(px(12.0))
                                .overflow_hidden()
                                .child(
                                    ActionDiscoveryPanel::from_spec(
                                        ActionDiscoveryPanelSpec::new(vec![
                                            ActionDiscoverySection::new(
                                                "file",
                                                "File",
                                                vec![CommandActionItem::new("save", "Save")
                                                    .with_shortcut("\u{2318}S")],
                                            ),
                                            ActionDiscoverySection::new(
                                                "release",
                                                "Release",
                                                vec![CommandActionItem::new("deploy", "Deploy")
                                                    .with_badge("Dangerous")],
                                            ),
                                        ])
                                        .with_density(poodle_specs::ControlDensity::Compact),
                                        theme,
                                    )
                                    .with_id("action-disc-semantic-compact"),
                                ),
                        )
                        .child(
                            div()
                                .max_w(px(512.0))
                                .border_1()
                                .border_color(color_to_hsla(
                                    theme.resolve_color("color.border.subtle"),
                                ))
                                .rounded(px(8.0))
                                .p(px(12.0))
                                .overflow_hidden()
                                .child(
                                    ActionDiscoveryPanel::from_spec(
                                        ActionDiscoveryPanelSpec::new(vec![
                                            ActionDiscoverySection::new(
                                                "file",
                                                "File",
                                                vec![CommandActionItem::new("save", "Save")
                                                    .with_shortcut("\u{2318}S")],
                                            ),
                                            ActionDiscoverySection::new(
                                                "release",
                                                "Release",
                                                vec![CommandActionItem::new("deploy", "Deploy")
                                                    .with_badge("Dangerous")],
                                            ),
                                        ])
                                        .with_size_role(
                                            poodle_specs::SemanticControlSizeRole::Prominent,
                                        ),
                                        theme,
                                    )
                                    .with_id("action-disc-semantic-prominent"),
                                ),
                        ),
                ),
        )
}
