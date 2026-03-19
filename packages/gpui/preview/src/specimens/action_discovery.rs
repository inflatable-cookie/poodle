use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem, DiscoveryState};
use pug_gpui_components::PugActionDiscoveryPanel;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Grouped actions ---
    let grouped_sections = vec![
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
                CommandActionItem::new("find-in-files", "Find in Files").with_shortcut("\u{21E7}\u{2318}F"),
                CommandActionItem::new("find-and-replace", "Find and Replace").with_shortcut("\u{2318}H"),
            ],
        ),
        ActionDiscoverySection::new(
            "view",
            "View",
            vec![
                CommandActionItem::new("toggle-terminal", "Toggle Terminal").with_shortcut("\u{2318}`"),
                CommandActionItem::new("toggle-sidebar", "Toggle Sidebar").with_shortcut("\u{2318}B"),
            ],
        ),
    ];

    let grouped_spec = ActionDiscoveryPanelSpec::new(grouped_sections);

    // --- With descriptions and badges ---
    let desc_sections = vec![
        ActionDiscoverySection::new(
            "cicd",
            "CI/CD",
            vec![
                CommandActionItem::new("deploy", "Deploy to Production")
                    .with_description("Push current build to production environment")
                    .with_badge("Dangerous"),
                CommandActionItem::new("open-preview", "Open Preview")
                    .with_description("Launch preview environment for current branch")
                    .with_shortcut("\u{2318}P"),
            ],
        ),
        ActionDiscoverySection::new(
            "tools",
            "Tools",
            vec![
                CommandActionItem::new("run-linter", "Run Linter").with_shortcut("\u{2318}L"),
            ],
        ),
    ];

    let desc_spec = ActionDiscoveryPanelSpec::new(desc_sections);

    // --- Empty state ---
    let empty_spec = ActionDiscoveryPanelSpec::new(vec![])
        .with_state(DiscoveryState::Empty);

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("GROUPED ACTIONS", text_secondary))
        .child(
            PugActionDiscoveryPanel::new(grouped_spec, theme)
                .with_id("action-disc-grouped")
        )
        .child(section_label("WITH DESCRIPTIONS AND BADGES", text_secondary))
        .child(
            PugActionDiscoveryPanel::new(desc_spec, theme)
                .with_id("action-disc-desc")
        )
        .child(section_label("EMPTY STATE", text_secondary))
        .child(
            PugActionDiscoveryPanel::new(empty_spec, theme)
                .with_id("action-disc-empty")
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
