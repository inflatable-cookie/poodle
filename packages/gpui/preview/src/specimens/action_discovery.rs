use crate::app_state::AppState;
use crate::PreviewRoot;
use gpui::*;
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

    let workflow_spec = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "publishing",
            "Publishing",
            vec![
                CommandActionItem::new("publish", "Publish release")
                    .with_description("Ship the approved release to production.")
                    .with_badge("Dangerous"),
                CommandActionItem::new("open-preview", "Open preview")
                    .with_description("Launch preview environment for current branch")
                    .with_shortcut("\u{2318}P"),
            ],
        ),
        ActionDiscoverySection::new(
            "maintenance",
            "Maintenance",
            vec![
                CommandActionItem::new("run-linter", "Run linter").with_shortcut("\u{2318}L"),
                CommandActionItem::new("reindex-search", "Reindex search")
                    .with_description("Refresh the workspace search index."),
            ],
        ),
    ]);

    let empty_spec = ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Empty);

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
                    ActionDiscoveryPanel::from_spec(grouped_spec, theme)
                        .with_id("action-disc-grouped"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Workflow actions"),
                    theme,
                ))
                .child(
                    ActionDiscoveryPanel::from_spec(workflow_spec, theme)
                        .with_id("action-disc-workflow"),
                ),
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
                .child(
                    ActionDiscoveryPanel::from_spec(empty_spec, theme).with_id("action-disc-empty"),
                ),
        )
}
