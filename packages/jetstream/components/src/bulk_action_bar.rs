//! BulkActionBar — contextual selection bar that mirrors the GPUI rebuild
//! (`packages/gpui/components/src/primitives/bulk_action_bar.rs`) and the Svelte
//! authority (`BulkActionBar.svelte`).
//!
//! Anatomy (contract §2):
//! ```text
//! [Root]  accent-tinted, fixed bottom-docked, justify-between
//!   ├── [Summary]  "{N} selected" (strong) + "of {M}" (secondary) + select-all
//!   └── [Actions]  ghost IconButton per action (danger/warning toned) + clear "x"
//! ```
//!
//! - Actions render as ghost `js_icon_button`s showing the action icon
//!   (`resolved_icon()` — `trash-2` for danger, `circle` otherwise). The label
//!   is the accessible name / tooltip only.
//! - Danger/warning tone colors the action icon (contract §4/§8). `js_icon_button`
//!   has no tone channel, so toned actions are built locally with the same ghost
//!   anatomy (height/icon-size/radius/hover) but a tone-resolved icon color.
//! - Select-all (`check-check`, chrome size-role) and clear (`x`) are ghost
//!   `js_icon_button`s in the summary / actions rows respectively.
//! - `loading`/`disabled` dim the bar via `state.opacity.disabled`; per-action
//!   `is_disabled` and the shared `actions_disabled()` gate disable the buttons.
//!
//! Preview-loop / accepted limits:
//! - Per-action / clear / select-all click wiring lives in the preview event
//!   loop, not the component (`js_icon_button` exposes `.focusable()` only here).
//! - `role="region"` is emitted, and each action button is named from the
//!   action's own `label` — icon-only buttons have no text to be named from.
//!
//! Token gap: the contract root background is
//! `color-mix(panel 93%, text-primary)`; resolved here via `theme_ext::color_mix`
//! (no single semantic token for the mixed fill — same approach as GPUI).
//! Vertical pad is the contract-exact flat `0.5rem` (`rem_to_px(0.5)`); there is
//! no dedicated token for it.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    BulkAction, BulkActionBarSpec, BulkActionTone, ControlDensity, ControlSize, IconButtonSpec,
    SemanticControlSizeRole,
};

use crate::icon_button::js_icon_button;
use crate::presentation::{
    control_height_rem, panel_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// Summary font size per the contract size table (one step up from the control
/// font scale — matches the GPUI body-font map).
fn summary_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Actions-row gap per density (contract §8: compact 0.125rem, default
/// space.inline.sm, comfortable 0.5rem).
fn actions_gap_px(spec: &BulkActionBarSpec, theme: &JetstreamThemeProvider) -> f32 {
    match spec.density {
        ControlDensity::Compact => rem_to_px(0.125),
        ControlDensity::Default => crate::theme_ext::resolve_px(theme, spec.gap_token()),
        ControlDensity::Comfortable => rem_to_px(0.5),
    }
}

/// Build a ghost icon-button whose icon is colored by an action tone. Mirrors
/// `js_icon_button` anatomy (square, ghost, hover bg) but applies the tone
/// color, which `js_icon_button` does not carry.
fn toned_icon_button(
    action: &BulkAction,
    size: ControlSize,
    icon_color: Color,
    disabled: bool,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(size, SemanticControlSizeRole::Control);
    let height = rem_to_px(control_height_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let radius = resolve_radius(theme, "radius.control");

    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let hover_bg = surface.mix_srgb(elevated, 0.84);

    let mut el = ui_element::button("")
        // Same reason as the default-tone branch: icon-only, and the action
        // already carries the words.
        .aria_label(action.label.clone())
        .h(height)
        .w(height)
        .rounded(radius)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable()
        .cursor_pointer()
        .hover(|s| s.bg(hover_bg))
        .child(
            ui_element::icon(action.resolved_icon())
                .w(icon_size)
                .h(icon_size)
                .text_color(icon_color),
        );

    if disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}

pub fn js_bulk_action_bar(spec: &BulkActionBarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let summary_font = rem_to_px(summary_font_rem(effective_size));

    // Padding: horizontal density-driven, vertical flat 0.5rem (contract §8).
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.5);

    // Gaps: root/summary gap = space.inline.md (root), summary inner & actions per spec.
    let root_gap = match spec.density {
        ControlDensity::Compact => rem_to_px(0.375),
        ControlDensity::Default => crate::theme_ext::resolve_px(theme, "space.inline.md"),
        ControlDensity::Comfortable => rem_to_px(1.0),
    };
    let summary_gap = crate::theme_ext::resolve_px(theme, spec.gap_token());
    let actions_gap = actions_gap_px(spec, theme);

    // ── Colors ──────────────────────────────────────────────────
    let panel_bg = resolve_color(theme, "color.background.panel");
    let text_primary_v = resolve_color(theme, spec.text_token());
    // Contract §8 accent fill: color-mix(panel 93%, text-primary).
    let fill: Color = color_mix(panel_bg, text_primary_v, 0.93).into();
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_token());
    let total_color = resolve_color(theme, spec.total_text_token());
    let danger_color: Color = resolve_color(theme, spec.danger_text_token()).into();
    let warning_color: Color = resolve_color(theme, spec.warning_text_token()).into();

    // Shared availability gates (Svelte isUnavailable / actionsDisabled).
    let is_unavailable = spec.is_unavailable();
    let actions_disabled = spec.actions_disabled();

    // ── Summary (left) ──────────────────────────────────────────
    let mut summary = ui_element::div()
        .flex_row()
        .items_center()
        .flex_wrap()
        .gap(summary_gap);

    summary = summary.child(
        ui_element::label(&format!("{} selected", spec.selection_count))
            .text_color(text_color)
            .text_size(summary_font)
            .text_weight(600),
    );

    if let Some(total) = spec.total_count {
        summary = summary.child(
            ui_element::label(&format!("of {total}"))
                .text_color(total_color)
                .text_size(summary_font),
        );
    }

    // Select-all: ghost `check-check` IconButton, chrome size-role.
    if spec.show_select_all && !spec.all_selected {
        let mut select_spec = IconButtonSpec::new()
            .with_aria_label("Select all")
            .with_icon("check-check")
            .with_size(spec.size)
            .with_size_role(SemanticControlSizeRole::Chrome);
        if is_unavailable {
            select_spec = select_spec.with_disabled(true);
        }
        summary = summary.child(js_icon_button(&select_spec, theme));
    }

    // ── Actions (right): ghost IconButtons + clear "x" ──────────
    let mut actions = ui_element::div()
        .flex_row()
        .items_center()
        .flex_wrap()
        .gap(actions_gap);

    for action in &spec.actions {
        let disabled = actions_disabled || action.is_disabled;
        match action.tone {
            BulkActionTone::Danger => {
                actions = actions
                    .child(toned_icon_button(action, spec.size, danger_color, disabled, theme));
            }
            BulkActionTone::Warning => {
                actions = actions
                    .child(toned_icon_button(action, spec.size, warning_color, disabled, theme));
            }
            BulkActionTone::Default => {
                // Default tone uses the shared js_icon_button (text-primary icon).
                let mut a_spec = IconButtonSpec::new()
                    // The action already carries the words for this control;
                    // rendering it icon-only threw them away, leaving a bar of
                    // identical unnamed buttons.
                    .with_aria_label(action.label.clone())
                    .with_icon(action.resolved_icon())
                    .with_size(spec.size)
                    .with_size_role(spec.size_role);
                if disabled {
                    a_spec = a_spec.with_disabled(true);
                }
                actions = actions.child(js_icon_button(&a_spec, theme));
            }
        }
    }

    // Clear-selection (`x`) ghost IconButton — contract §2 (always present).
    let mut clear_spec = IconButtonSpec::new()
        .with_aria_label("Clear selection")
        .with_icon("x")
        .with_size(spec.size)
        .with_size_role(spec.size_role);
    if is_unavailable {
        clear_spec = clear_spec.with_disabled(true);
    }
    actions = actions.child(js_icon_button(&clear_spec, theme));

    // ── Root ────────────────────────────────────────────────────
    let mut root = ui_element::div()
        .flex_row()
        .items_center()
        .flex_wrap()
        .justify_between()
        .gap(root_gap)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .bg(fill)
        .border(rem_to_px(0.0625))
        .border_color(border)
        .rounded(radius)
        .child(summary)
        .child(actions);

    // Loading/disabled dim the whole bar (contract §4).
    if is_unavailable {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity);
    }

    root.aria_role(jetstream_ui::accesskit::Role::Region)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> BulkActionBarSpec {
        BulkActionBarSpec::new()
            .with_selection_count(5)
            .with_total_count(42)
            .with_show_select_all(true)
            .with_actions(vec![
                BulkAction::new("export", "Export").with_icon("download"),
                BulkAction::new("delete", "Delete").with_tone(BulkActionTone::Danger),
            ])
    }

    #[test]
    fn renders_total_count() {
        let th = theme();
        let el = js_bulk_action_bar(&spec(), &th);
        let tree = probe(&el, 600.0, 120.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("5 selected"),
            "count missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("of 42"),
            "total 'of N' missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_action_icon_and_clear_control() {
        let th = theme();
        let el = js_bulk_action_bar(&spec(), &th);
        let tree = probe(&el, 600.0, 120.0);

        // Default action icon (download) renders.
        assert!(
            tree.has_text("download"),
            "ghost action icon missing: {:?}",
            tree.texts()
        );
        // Danger action falls back to its resolved icon (trash-2).
        assert!(
            tree.has_text("trash-2"),
            "danger action icon missing: {:?}",
            tree.texts()
        );
        // Clear control renders the "x" icon.
        assert!(
            tree.has_text("x"),
            "clear control icon missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_select_all_when_shown() {
        let th = theme();
        let el = js_bulk_action_bar(&spec(), &th);
        let tree = probe(&el, 600.0, 120.0);
        assert!(
            tree.has_text("check-check"),
            "select-all icon missing: {:?}",
            tree.texts()
        );
    }
}
