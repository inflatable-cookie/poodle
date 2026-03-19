use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::ProjectHeaderSpec;
use pug_gpui_components::PugProjectHeader;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Clean state ---
    let clean_spec = ProjectHeaderSpec::new("My Project");

    // --- Dirty state ---
    let dirty_spec = ProjectHeaderSpec::new("My Project")
        .with_dirty(true);

    // --- With subtitle and actions ---
    let subtitle_spec = ProjectHeaderSpec::new("My Project")
        .with_subtitle("Last saved 2 minutes ago")
        .with_action_count(2);

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("CLEAN STATE", text_secondary))
        .child(PugProjectHeader::new(clean_spec, theme))
        .child(section_label("DIRTY STATE", text_secondary))
        .child(PugProjectHeader::new(dirty_spec, theme))
        .child(section_label("WITH SUBTITLE AND ACTIONS", text_secondary))
        .child(PugProjectHeader::new(subtitle_spec, theme))
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
