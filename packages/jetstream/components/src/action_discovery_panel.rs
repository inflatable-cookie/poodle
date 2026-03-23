//! ActionDiscoveryPanel — Jetstream keyboard shortcut panel backed by ActionDiscoveryPanelSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::ActionDiscoveryPanelSpec;
use crate::theme_ext::resolve_color;

pub fn js_action_discovery_panel(spec: &ActionDiscoveryPanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .flex_col().gap(8.0)
        .pl(16.0).pr(16.0).pt(12.0).pb(12.0);

    for section in &spec.sections {
        el = el.child(ui_element::label(&section.title).text_color(text_secondary).text_size(11.0).text_weight(600));
    }

    el
}
