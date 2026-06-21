//! FormDialog — dialog wrapping a form workflow, backed by FormDialogSpec.
//!
//! Contract: `docs/contracts/components/form-dialog.md`
//! Reference: `packages/gpui/components/src/composites/form_dialog.rs`
//!
//! Composes `js_dialog` (shell: backdrop + panel + header + close + separator),
//! `js_form_layout` (body: fields + error/success callouts), and `js_button` /
//! `js_form_actions` (footer: default submit/cancel row) or a caller-owned custom
//! actions slot. ALL sizing/colors resolve from tokens.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, DialogSpec, FormActionAlign, FormActionsSpec, FormDialogSpec,
    FormLayoutSpec,
};

use crate::button::js_button;
use crate::dialog::js_dialog;
use crate::form_actions::js_form_actions;
use crate::form_layout::js_form_layout;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

/// Build a FormDialog element from a FormDialogSpec with field children and optional
/// custom actions override.
///
/// Contract anatomy:
/// ```text
/// [Dialog]  backdrop + panel (composed via js_dialog)
///   ├── [Header]   title + optional subtitle/description + close button
///   ├── [Body]     FormLayout (or bare content) + optional error/success callouts
///   └── [Actions]  custom slot OR default submit/cancel row (js_button)
/// ```
pub fn js_form_dialog(
    spec: &FormDialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    custom_actions: Option<JsEl>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let section_gap = rem_to_px(size_font_rem(effective_size));

    let text_secondary = resolve_color(theme, "color.text.secondary");

    // ── Body ──────────────────────────────────────────────────────────────────
    // Subtitle (when present) is rendered inline at the top of the body; the
    // Dialog `description` slot is then suppressed to avoid a duplicate
    // announcement (contract §9). When no subtitle, the description flows to the
    // Dialog header.
    let body: JsEl = if spec.is_bare {
        // Bare mode: children rendered directly, no FormLayout wrapper.
        let mut col = ui_element::div().flex_col().gap(section_gap);
        if let Some(ref subtitle) = spec.subtitle {
            col = col.child(
                ui_element::label(subtitle)
                    .text_size(font_size)
                    .text_color(text_secondary),
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
            ..FormLayoutSpec::default()
        };
        let form = js_form_layout(&layout_spec, theme, children, None);
        if let Some(ref subtitle) = spec.subtitle {
            ui_element::div()
                .flex_col()
                .gap(section_gap)
                .child(
                    ui_element::label(subtitle)
                        .text_size(font_size)
                        .text_color(text_secondary),
                )
                .child(form)
        } else {
            form
        }
    };

    // ── Actions ───────────────────────────────────────────────────────────────
    // Custom slot replaces the default row entirely; otherwise the default
    // ghost-Cancel + primary-Submit pair is composed from `js_button` and laid
    // out by `js_form_actions` with top separation removed so the row sits flush
    // on the Dialog footer rail (contract §7 nested-FormActions padding-top: 0).
    let actions: Option<JsEl> = if let Some(custom) = custom_actions {
        Some(ui_element::div().w_full().child(custom))
    } else if spec.show_default_actions {
        let submit_disabled = spec.is_submitting || spec.is_disabled;

        // Cancel — ghost Button, disabled during submitting (Svelte parity).
        let cancel = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label(spec.cancel_label.clone())
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density)
                .with_disabled(spec.is_submitting),
            theme,
        );

        // Submit — primary Button. Label flips to "Submitting…" and the button
        // is disabled while submitting or explicitly disabled.
        let submit_label = if spec.is_submitting {
            "Submitting\u{2026}".to_string()
        } else {
            spec.submit_label.clone()
        };
        let submit = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label(submit_label)
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density)
                .with_disabled(submit_disabled),
            theme,
        );

        let row_spec = FormActionsSpec::new()
            .with_align(FormActionAlign::End)
            .with_density(spec.density)
            // Sit flush on the Dialog footer rail; the Dialog draws the divider.
            .with_top_separation(false);
        Some(js_form_actions(&row_spec, theme, vec![cancel, submit]))
    } else {
        None
    };

    // ── Dialog shell ────────────────────────────────────────────────────────────
    // Compose js_dialog for the surface, backdrop, header (title + description +
    // close), and the action separator. Submitting blocks Escape/backdrop dismiss
    // (contract §6); dismiss wiring itself lives in the host preview loop.
    let mut dialog_spec = DialogSpec::new()
        .with_default_open(true)
        .with_show_close_button(true)
        .with_size(spec.size)
        .with_size_role(spec.size_role)
        .with_density(spec.density)
        .with_dismiss_on_escape(!spec.is_submitting)
        .with_dismiss_on_backdrop(!spec.is_submitting);

    if !spec.title.is_empty() {
        dialog_spec = dialog_spec.with_title(spec.title.clone());
    }
    // Subtitle takes precedence over description; when a subtitle is rendered
    // inline in the body, suppress the Dialog description to avoid duplication.
    if spec.subtitle.is_none() {
        if let Some(ref description) = spec.description {
            dialog_spec = dialog_spec.with_description(description.clone());
        }
    }
    if let Some(width) = spec.width {
        dialog_spec = dialog_spec.with_width(width);
    }
    if let Some(ref aria) = spec.aria_label {
        dialog_spec = dialog_spec.with_aria_label(aria.clone());
    }

    js_dialog(&dialog_spec, theme, vec![body], actions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::DialogWidth;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn field_stub() -> Vec<JsEl> {
        vec![ui_element::label("Email")]
    }

    #[test]
    fn renders_title_body_and_default_actions() {
        let th = theme();
        let spec = FormDialogSpec::new("Add new user")
            .with_submit_label("Add user")
            .with_cancel_label("Cancel");
        let el = js_form_dialog(&spec, &th, field_stub(), None);
        let tree = probe(&el, 800.0, 600.0);

        // Title from the composed Dialog header.
        assert!(tree.has_text("Add new user"), "title missing: {:?}", tree.texts());
        // Body field slot made it through.
        assert!(tree.has_text("Email"), "body field slot missing: {:?}", tree.texts());
        // Default footer actions: ghost Cancel + primary Submit.
        assert!(tree.has_text("Cancel"), "cancel action missing: {:?}", tree.texts());
        assert!(tree.has_text("Add user"), "submit action missing: {:?}", tree.texts());
        assert!(tree.count_kind("Button") >= 2, "expected at least 2 action buttons");
    }

    #[test]
    fn submitting_flips_submit_label() {
        let th = theme();
        let spec = FormDialogSpec::new("Create account")
            .with_submit_label("Create")
            .with_submitting(true);
        let el = js_form_dialog(&spec, &th, field_stub(), None);
        let tree = probe(&el, 800.0, 600.0);
        assert!(
            tree.has_text("Submitting\u{2026}"),
            "submit label should flip to submitting state: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn error_message_surfaces_in_body() {
        let th = theme();
        let spec = FormDialogSpec::new("Create account")
            .with_error("A user with this email already exists.");
        let el = js_form_dialog(&spec, &th, field_stub(), None);
        let tree = probe(&el, 800.0, 600.0);
        assert!(
            tree.has_text("A user with this email already exists."),
            "error callout missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn no_default_actions_uses_custom_slot() {
        let th = theme();
        let spec = FormDialogSpec::new("Edit workspace settings")
            .with_subtitle("Update shared defaults for this workspace.")
            .with_show_default_actions(false);
        let custom = ui_element::label("Custom footer");
        let el = js_form_dialog(&spec, &th, field_stub(), Some(custom));
        let tree = probe(&el, 800.0, 600.0);
        // Subtitle rendered, custom footer present, default Submit absent.
        assert!(
            tree.has_text("Update shared defaults for this workspace."),
            "subtitle missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Custom footer"), "custom actions missing: {:?}", tree.texts());
        assert!(!tree.has_text("Submit"), "default submit should be suppressed");
    }

    #[test]
    fn custom_width_widens_surface() {
        let th = theme();
        let narrow = js_form_dialog(&FormDialogSpec::new("Narrow"), &th, field_stub(), None);
        let wide = js_form_dialog(
            &FormDialogSpec::new("Wide").with_width(DialogWidth::Xl),
            &th,
            field_stub(),
            None,
        );
        // The dialog panel is the second node (root = backdrop overlay).
        let narrow_tree = probe(&narrow, 1200.0, 800.0);
        let wide_tree = probe(&wide, 1200.0, 800.0);
        let narrow_panel = &narrow_tree.nodes[1];
        let wide_panel = &wide_tree.nodes[1];
        assert!(
            wide_panel.w > narrow_panel.w,
            "Xl width ({}) should exceed default width ({})",
            wide_panel.w,
            narrow_panel.w
        );
    }

    #[test]
    fn bare_mode_skips_form_layout_wrapper() {
        let th = theme();
        let spec = FormDialogSpec::new("Bare").with_bare(true);
        let el = js_form_dialog(&spec, &th, field_stub(), None);
        let tree = probe(&el, 800.0, 600.0);
        // Body field slot still rendered directly.
        assert!(tree.has_text("Email"), "bare body content missing: {:?}", tree.texts());
        // bare auto-suppresses default actions only when show_default_actions
        // is also cleared by the host; default true still renders the row here,
        // so just assert the dialog built without panicking and shows the title.
        assert!(tree.has_text("Bare"), "title missing in bare mode: {:?}", tree.texts());
    }
}
