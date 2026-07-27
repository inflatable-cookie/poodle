//! MarkdownEditor — Jetstream markdown editor backed by MarkdownEditorSpec.
//!
//! Contract: `docs/contracts/components/markdown-editor.md`
//! Reference: `packages/svelte/components/src/MarkdownEditor.svelte`
//!
//! All geometry resolves from the spec's contract §8 size/density tables; all
//! colors resolve from semantic tokens. Real text editing + toolbar markdown
//! insertion live in the preview event loop (Tier-3), so the edit pane is a
//! `text_input` (placeholder + current value) and the preview shows source text.
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonVariant, IconButtonSpec, MarkdownEditorSpec};

use crate::icon_button::js_icon_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Toolbar formatting tools in contract §2 anatomy order, with their icons.
const TOOLS: [(&str, &str); 7] = [
    ("bold", "Bold"),
    ("italic", "Italic"),
    ("heading", "Heading"),
    ("link", "Link"),
    ("code", "Code"),
    ("quote", "Quote"),
    ("list", "List"),
];

pub fn js_markdown_editor(spec: &MarkdownEditorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    let tool_size = rem_to_px(spec.tool_size_rem());
    let toolbar_y = rem_to_px(spec.toolbar_y_rem());
    let toolbar_x = rem_to_px(spec.toolbar_x_rem());
    let tool_gap = rem_to_px(spec.tool_gap_rem());
    let pane_pad = rem_to_px(spec.pane_pad_rem());
    let mode_x = rem_to_px(spec.mode_x_rem());
    let mode_y = rem_to_px(spec.mode_y_rem());
    let toolbar_gap = rem_to_px(0.5); // contract toolbar `gap: 0.5rem`
    let tool_font = rem_to_px(0.75); // contract tool font-size
    let textarea_font = rem_to_px(0.8125); // contract textarea font-size
    let preview_font = rem_to_px(0.875); // contract preview font-size
    let min_h = rem_to_px(spec.min_height_rem());

    // ── Token resolution ──────────────────────────────────────
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let toolbar_border = resolve_color(theme, spec.toolbar_border_token());
    let split_divider = resolve_color(theme, spec.split_divider_color_token());
    let text_primary = resolve_color(theme, spec.textarea_color_token());
    let tool_color = resolve_color(theme, spec.tool_color_token());
    let placeholder_color = resolve_color(theme, spec.placeholder_color_token());
    let preview_empty_color = resolve_color(theme, spec.preview_empty_color_token());
    let radius = resolve_radius(theme, "radius.surface");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    // Toolbar bg: `color-mix(elevated 72%, transparent)` (alpha reduction).
    let elevated = resolve_color(theme, "color.background.elevated");
    let toolbar_bg = tint(elevated, 0.72);

    let is_edit = spec.shows_editor();
    let is_preview = spec.shows_preview();
    let tools_disabled = spec.tools_disabled();

    // ── Root ──────────────────────────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .min_h(min_h)
        .overflow_hidden();

    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    // ── Toolbar (bottom border only, space-between) ───────────
    let mut toolbar = ui_element::div()
        .bg(toolbar_bg)
        .flex_row()
        .items_center()
        .justify_between()
        .gap(toolbar_gap)
        .pl(toolbar_x)
        .pr(toolbar_x)
        .pt(toolbar_y)
        .pb(toolbar_y)
        .border_b_1()
        .border_color(toolbar_border);

    // Tools container — real Icon glyphs in contract order.
    let mut tools_row = ui_element::div().flex_row().gap(tool_gap);
    for (icon_name, _label) in &TOOLS {
        let mut btn = ui_element::button("")
            .w(tool_size)
            .h(tool_size)
            .flex_row()
            .items_center()
            .justify_center()
            .rounded(ctrl_radius)
            .child(
                ui_element::icon(*icon_name)
                    .w(tool_font)
                    .h(tool_font)
                    .text_color(tool_color),
            );
        if tools_disabled {
            btn = btn.opacity(0.4); // contract tool `:disabled` opacity
        } else {
            btn = btn.focusable().cursor_pointer();
        }
        tools_row = tools_row.child(btn);
    }
    toolbar = toolbar.child(tools_row);

    // Mode switcher — delegates chrome to IconButton.
    let modes = ui_element::div()
        .flex_row()
        .gap(tool_gap)
        .pl(mode_x)
        .pr(mode_x)
        .pt(mode_y)
        .pb(mode_y)
        .child(mode_button(spec, theme, "pencil", "Edit", "edit"))
        .child(mode_button(spec, theme, "columns-2", "Split", "split"))
        .child(mode_button(spec, theme, "eye", "Preview", "preview"));
    toolbar = toolbar.child(modes);
    el = el.child(toolbar);

    // ── Body ──────────────────────────────────────────────────
    let mut body = ui_element::div().flex_row().grow();

    if is_edit {
        let placeholder = spec.placeholder.as_deref().unwrap_or("Write markdown...");
        // Real text surface — a text_input carrying the value + placeholder.
        // Editing/insertion is owned by the preview loop (Tier-3).
        let mut textarea = ui_element::div()
            .grow()
            .pl(pane_pad)
            .pr(pane_pad)
            .pt(pane_pad)
            .pb(pane_pad)
            .child(
                // Textarea is monospace (contract §8 `.md-editor__textarea`).
                ui_element::text_input(spec.value.clone(), placeholder)
                    .w_full()
                    .text_size(textarea_font)
                    .font_family(FontFamily::Mono)
                    .text_color(if spec.value.is_empty() {
                        placeholder_color
                    } else {
                        text_primary
                    }),
            );

        // Split mode: textarea gets a right border (contract border-right).
        if is_preview {
            textarea = textarea.border_r_1().border_color(split_divider);
        }
        body = body.child(textarea);
    }

    if is_preview {
        let mut preview = ui_element::div()
            .grow()
            .pl(pane_pad)
            .pr(pane_pad)
            .pt(pane_pad)
            .pb(pane_pad);

        if spec.value.is_empty() {
            preview = preview.child(
                ui_element::label("Nothing to preview")
                    .text_color(preview_empty_color)
                    .text_size(preview_font),
            );
        } else {
            // Rendered HTML is Tier-3; show source text until a renderer is plugged.
            preview = preview.child(
                ui_element::label(&spec.value)
                    .text_color(text_primary)
                    .text_size(preview_font),
            );
        }
        body = body.child(preview);
    }

    el = el.child(body);
    crate::aria::with_aria_label(el, Some(spec.aria_label.as_str()))
}

fn mode_button(
    spec: &MarkdownEditorSpec,
    theme: &JetstreamThemeProvider,
    icon: &str,
    aria: &str,
    mode_val: &str,
) -> JsEl {
    js_icon_button(
        &IconButtonSpec::new()
            .with_icon(icon)
            .with_aria_label(aria)
            .with_variant(if spec.mode == mode_val {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Ghost
            })
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density),
        theme,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn toolbar_renders_all_seven_tool_icons() {
        let spec = MarkdownEditorSpec::new().with_mode("edit");
        let tree = probe(&js_markdown_editor(&spec, &theme()), 480.0, 240.0);
        for (icon, _label) in &TOOLS {
            assert!(
                tree.has_text(icon),
                "toolbar missing icon `{icon}`; got texts {:?}",
                tree.texts()
            );
        }
        // Three mode buttons present too (pencil/columns-2/eye).
        for icon in ["pencil", "columns-2", "eye"] {
            assert!(tree.has_text(icon), "mode switcher missing `{icon}`");
        }
    }

    #[test]
    fn edit_pane_shows_placeholder_when_empty() {
        let spec = MarkdownEditorSpec::new()
            .with_mode("edit")
            .with_placeholder("Start writing...");
        let tree = probe(&js_markdown_editor(&spec, &theme()), 480.0, 240.0);
        assert!(
            tree.has_text("Start writing..."),
            "placeholder not rendered: {:?}",
            tree.texts()
        );
        assert_eq!(tree.count_kind("TextInput"), 1, "edit pane should be a text input");
    }

    #[test]
    fn edit_pane_shows_value_text() {
        let spec = MarkdownEditorSpec::new()
            .with_mode("edit")
            .with_value("# Hello world");
        let tree = probe(&js_markdown_editor(&spec, &theme()), 480.0, 240.0);
        assert!(tree.has_text("# Hello world"), "value not rendered: {:?}", tree.texts());
    }

    #[test]
    fn preview_empty_placeholder() {
        let spec = MarkdownEditorSpec::new().with_mode("preview");
        let tree = probe(&js_markdown_editor(&spec, &theme()), 480.0, 240.0);
        assert!(tree.has_text("Nothing to preview"), "empty preview copy missing");
        // No editable text input in preview mode.
        assert_eq!(tree.count_kind("TextInput"), 0);
    }

    #[test]
    fn split_mode_shows_both_panes() {
        let spec = MarkdownEditorSpec::new()
            .with_mode("split")
            .with_value("body text");
        let tree = probe(&js_markdown_editor(&spec, &theme()), 600.0, 240.0);
        assert_eq!(tree.count_kind("TextInput"), 1, "split has a textarea");
        // Both the editor value and a preview label of the same source text exist.
        let body_hits = tree.texts().iter().filter(|t| **t == "body text").count();
        assert!(body_hits >= 2, "expected value in editor + preview, got {body_hits}");
    }
}
