//! FormDialog — dialog wrapping a form workflow, backed by FormDialogSpec.
//!
//! Contract: `docs/contracts/components/form-dialog.md`
//! Reference: `packages/gpui/components/src/composites/form_dialog.rs`
//!
//! Composes js_dialog + js_form_layout. Provides default submit/cancel actions
//! or accepts a custom actions slot. ALL sizing from tokens.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{FormDialogSpec, FormLayoutSpec};

use crate::form_layout::js_form_layout;
use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build a FormDialog element from a FormDialogSpec with field children and optional
/// custom actions override.
///
/// Contract anatomy:
/// ```text
/// [Dialog]  backdrop + panel
///   ├── [Header]   title + optional subtitle
///   ├── [Body]     FormLayout (or bare content) + optional error/success callouts
///   └── [Actions]  custom slot OR default submit/cancel row
/// ```
pub fn js_form_dialog(
    spec: &FormDialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    custom_actions: Option<JsEl>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let panel_bg: Color = resolve_color(theme, "color.background.panel").into();
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let fill = resolve_color(theme, "color.background.elevated");
    let radius = resolve_radius(theme, "radius.surface");
    let backdrop: Color = resolve_color(theme, "color.background.overlay").into();

    let title_gap = rem_to_px(0.5);
    let section_gap = rem_to_px(1.0);
    let actions_gap = resolve_px(theme, "space.inline.sm");

    // ── Body ──
    let body: JsEl = if spec.is_bare {
        // Bare mode: children rendered directly
        let mut col = ui_element::div().flex_col().gap(section_gap);
        if let Some(ref subtitle) = spec.subtitle {
            col = col.child(
                ui_element::label(subtitle)
                    .text_size(font_size)
                    .text_color(text_secondary)
            );
        }
        for child in children {
            col = col.child(child);
        }
        col
    } else {
        let layout_spec = FormLayoutSpec {
            columns: spec.columns,
            error: spec.error.clone(),
            success: spec.success.clone(),
            description: None,
        };
        let mut wrapper = ui_element::div().flex_col().gap(title_gap);
        if let Some(ref subtitle) = spec.subtitle {
            wrapper = wrapper.child(
                ui_element::label(subtitle)
                    .text_size(font_size)
                    .text_color(text_secondary)
            );
        }
        wrapper = wrapper.child(js_form_layout(&layout_spec, theme, children, None));
        wrapper
    };

    // ── Actions ──
    let actions: Option<JsEl> = if let Some(custom) = custom_actions {
        Some(ui_element::div().w_full().child(custom))
    } else if spec.show_default_actions {
        let submit_disabled = spec.is_submitting || spec.is_disabled;
        let control_radius = resolve_radius(theme, "radius.control");
        let btn_px = rem_to_px(0.75);
        let btn_py = rem_to_px(0.375);

        // Cancel
        let cancel = ui_element::button(&spec.cancel_label)
            .text_size(font_size)
            .text_color(text_primary)
            .pl(btn_px).pr(btn_px).pt(btn_py).pb(btn_py)
            .rounded(control_radius)
            .bg(Color::TRANSPARENT)
            .cursor_pointer();

        // Submit
        let submit_bg = if submit_disabled {
            accent.mix(panel_bg, 0.5)
        } else {
            accent
        };
        let submit_label = if spec.is_submitting {
            "Submitting\u{2026}".to_string()
        } else {
            spec.submit_label.clone()
        };
        let mut submit_btn = ui_element::button(&submit_label)
            .text_size(font_size)
            .text_color(Color::WHITE)
            .bg(submit_bg)
            .pl(btn_px).pr(btn_px).pt(btn_py).pb(btn_py)
            .rounded(control_radius);
        if !submit_disabled {
            submit_btn = submit_btn.cursor_pointer();
        } else {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            submit_btn = submit_btn.opacity(opacity);
        }

        Some(
            ui_element::div()
                .flex_row()
                .justify_end()
                .gap(actions_gap)
                .child(cancel)
                .child(submit_btn)
        )
    } else {
        None
    };

    // ── Panel ──
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
        .flex_col().gap(section_gap)
        .min_w(rem_to_px(25.0))
        .shadow_lg();

    // Title
    if !spec.title.is_empty() {
        panel = panel.child(
            ui_element::label(&spec.title)
                .text_color(text_primary)
                .text_size(rem_to_px(size_font_rem(effective_size) + 0.125))
                .text_weight(600)
        );
    }

    // Description: subtitle for non-bare (bare mode renders it inside body already)
    if !spec.is_bare {
        if let Some(desc_text) = spec.subtitle.as_deref().or(spec.description.as_deref()) {
            panel = panel.child(
                ui_element::label(desc_text)
                    .text_color(text_secondary)
                    .text_size(font_size)
            );
        }
    }

    panel = panel.child(body);

    if let Some(actions_el) = actions {
        // Thin separator
        panel = panel.child(
            ui_element::div()
                .h(1.0)
                .bg(border)
        );
        panel = panel.child(actions_el);
    }

    // Backdrop + centered panel
    ui_element::div()
        .bg(backdrop)
        .overlay()
        .items_center()
        .justify_center()
        .child(panel)
}
