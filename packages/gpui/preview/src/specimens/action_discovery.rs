use gpui::*;
use pug_gpui_workstation::{ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem};
use pug_gpui_components::PugActionDiscoveryPanel;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let sections = vec![
        ActionDiscoverySection::new(
            "file",
            "File Actions",
            vec![
                CommandActionItem::new("new-file", "New File").with_shortcut("\u{2318}N"),
                CommandActionItem::new("save", "Save").with_shortcut("\u{2318}S"),
            ],
        ),
        ActionDiscoverySection::new(
            "edit",
            "Edit Actions",
            vec![
                CommandActionItem::new("find", "Find").with_shortcut("\u{2318}F"),
                CommandActionItem::new("replace", "Find and Replace")
                    .with_shortcut("\u{2318}H")
                    .with_badge("new"),
            ],
        ),
    ];

    let spec = ActionDiscoveryPanelSpec::new(sections);

    div().child(
        PugActionDiscoveryPanel::new(spec, theme)
            .with_id("action-disc")
    )
}
