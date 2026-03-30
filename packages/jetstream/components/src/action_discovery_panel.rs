//! ActionDiscoveryPanel — Jetstream keyboard shortcut panel backed by ActionDiscoveryPanelSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::ActionDiscoveryPanelSpec;

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_action_discovery_panel(spec: &ActionDiscoveryPanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let _font_size = rem_to_px(size_font_rem(effective_size));
    let section_title_size = rem_to_px(size_font_rem(effective_size) - 0.125);
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let gap = resolve_px(theme, spec.gap_token());

    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let _text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .flex_col().gap(gap)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y);

    for section in &spec.sections {
        el = el.child(ui_element::label(&section.title).text_color(text_secondary).text_size(section_title_size).text_weight(600));
    }

    el
}
