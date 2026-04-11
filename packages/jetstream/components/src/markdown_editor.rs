//! MarkdownEditor — Jetstream markdown editor backed by MarkdownEditorSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MarkdownEditorSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_markdown_editor(spec: &MarkdownEditorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Size-driven tool button size from contract
    let tool_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 1.5,
        poodle_specs::ControlSize::Sm => 1.75,
        poodle_specs::ControlSize::Md => 2.0,
        poodle_specs::ControlSize::Lg => 2.25,
        poodle_specs::ControlSize::Xl => 2.5,
    });
    let mode_px = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.375,
        poodle_specs::ControlSize::Sm => 0.5,
        poodle_specs::ControlSize::Md => 0.5,
        poodle_specs::ControlSize::Lg => 0.625,
        poodle_specs::ControlSize::Xl => 0.75,
    });

    // Density-driven spacing from contract
    let (toolbar_y, toolbar_x, tool_gap, pane_pad) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.25, 0.375, 0.0625, 0.625),
        poodle_specs::ControlDensity::Default => (0.375, 0.5, 0.125, 0.75),
        poodle_specs::ControlDensity::Comfortable => (0.5, 0.625, 0.1875, 0.875),
    };

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let _toolbar_fill = resolve_color(theme, spec.toolbar_fill_token());
    let radius = resolve_radius(theme, "radius.surface");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border_subtle = resolve_color(theme, "color.border.subtle");

    let elevated = resolve_color(theme, "color.background.elevated");
    let toolbar_bg = tint(elevated, 0.72);

    let is_edit = spec.mode == "edit" || spec.mode == "split";
    let is_preview = spec.mode == "preview" || spec.mode == "split";

    // Root
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .overflow_hidden();

    if spec.is_disabled {
        el = el.opacity(0.48);
    }

    // Toolbar
    let mut toolbar = ui_element::div()
        .bg(toolbar_bg)
        .flex_row().items_center()
        .gap(rem_to_px(0.5))
        .pl(rem_to_px(toolbar_x)).pr(rem_to_px(toolbar_x))
        .pt(rem_to_px(toolbar_y)).pb(rem_to_px(toolbar_y))
        .border(1.0).border_color(border_subtle);

    // Tool buttons
    let tools = ["B", "I", "H", "#", "<>", "\u{201C}", "\u{2022}"];
    let mut tools_row = ui_element::div().flex_row().gap(rem_to_px(tool_gap));
    for tool in &tools {
        let is_disabled_tool = spec.is_disabled || spec.mode == "preview";
        let mut btn = ui_element::button(*tool)
            .w(tool_size).h(tool_size)
            .text_color(text_secondary)
            .text_size(rem_to_px(0.75))
            .text_weight(600)
            .rounded(ctrl_radius);
        if is_disabled_tool {
            btn = btn.opacity(0.4);
        }
        tools_row = tools_row.child(btn);
    }
    toolbar = toolbar.child(tools_row);

    toolbar = toolbar.child(ui_element::div().grow());

    // Mode switcher
    let mut modes = ui_element::div()
        .flex_row().gap(rem_to_px(tool_gap))
        .border(1.0).border_color(border)
        .rounded(ctrl_radius)
        .overflow_hidden();

    for mode_name in &["Edit", "Split", "Preview"] {
        let is_active = (mode_name == &"Edit" && spec.mode == "edit")
            || (mode_name == &"Split" && spec.mode == "split")
            || (mode_name == &"Preview" && spec.mode == "preview");
        let mut btn = ui_element::button(*mode_name)
            .text_color(if is_active { text_primary } else { text_secondary })
            .text_size(rem_to_px(0.8125))
            .pl(mode_px).pr(mode_px)
            .pt(rem_to_px(0.1875)).pb(rem_to_px(0.1875));
        if is_active {
            btn = btn.bg(tint(accent, 0.16));
        }
        modes = modes.child(btn);
    }
    toolbar = toolbar.child(modes);
    el = el.child(toolbar);

    // Body
    let mut body = ui_element::div().flex_row().grow();

    if is_edit {
        // Textarea area
        let mut textarea = ui_element::div()
            .grow()
            .pl(rem_to_px(pane_pad)).pr(rem_to_px(pane_pad))
            .pt(rem_to_px(pane_pad)).pb(rem_to_px(pane_pad));

        if spec.value.is_empty() {
            let placeholder_text = spec.placeholder.as_deref().unwrap_or("Write markdown...");
            textarea = textarea.child(
                ui_element::label(placeholder_text)
                    .text_color(text_secondary).text_size(rem_to_px(0.8125))
            );
        } else {
            textarea = textarea.child(
                ui_element::label(&spec.value)
                    .text_color(text_primary).text_size(rem_to_px(0.8125))
            );
        }

        if spec.mode == "split" {
            textarea = textarea.border(1.0).border_color(border_subtle);
        }
        body = body.child(textarea);
    }

    if is_preview {
        // Preview pane
        let mut preview = ui_element::div()
            .grow()
            .pl(rem_to_px(pane_pad)).pr(rem_to_px(pane_pad))
            .pt(rem_to_px(pane_pad)).pb(rem_to_px(pane_pad));

        if spec.value.is_empty() {
            preview = preview.child(
                ui_element::label("Nothing to preview")
                    .text_color(text_secondary).text_size(rem_to_px(0.875))
            );
        } else {
            preview = preview.child(
                ui_element::label(&spec.value)
                    .text_color(text_primary).text_size(rem_to_px(0.875))
            );
        }
        body = body.child(preview);
    }

    el = el.child(body);

    el
}
