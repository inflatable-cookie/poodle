//! BulkActionBar — contextual selection bar.
//!
//! Contract: `docs/contracts/components/bulk-action-bar.md`
//! Ported from: `packages/jetstream/components/src/bulk_action_bar.rs`.
//!
//! Anatomy (contract §2):
//! ```text
//! [Root]  accent-tinted, justify-between
//!   ├── [Summary]  "{N} selected" (strong) + "of {M}" (secondary) + select-all
//!   └── [Actions]  ghost icon_button per action (danger/warning toned) + clear "x"
//! ```
//!
//! - Actions render as ghost `icon_button`s showing the action icon
//!   (`resolved_icon()`). The label is the accessible name only.
//! - Danger/warning tone colors the action icon; `icon_button` has no tone
//!   channel, so toned actions are built locally with the same ghost anatomy
//!   but a tone-resolved icon color.
//! - `loading`/`disabled` dim the bar via `state.opacity.disabled`; per-action
//!   `is_disabled` and the shared `actions_disabled()` gate disable buttons.
//!
//! Token gap: the contract root background is
//! `color-mix(panel 93%, text-primary)` — no single semantic token, resolved
//! via `mix_srgb`. Vertical pad is the contract-exact flat `0.5rem`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, StylePatch,
};
use poodle_specs::{
    BulkAction, BulkActionBarSpec, BulkActionTone, ControlDensity, ControlSize, IconButtonSpec,
    SemanticControlSizeRole,
};

use crate::color::mix_srgb;
use crate::icon_button::icon_button;
use crate::presentation::{
    control_height_rem, panel_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};

/// Host callbacks. `on_action` fires with the pressed action's id; disabled
/// actions never fire.
#[derive(Default)]
pub struct BulkActionBarHandlers {
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_select_all: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
}

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
fn actions_gap_px(spec: &BulkActionBarSpec, theme: &dyn ThemeProvider) -> f32 {
    match spec.density {
        ControlDensity::Compact => rem_to_px(0.125),
        ControlDensity::Default => theme.resolve_space(spec.gap_token()),
        ControlDensity::Comfortable => rem_to_px(0.5),
    }
}

/// Build a ghost icon-button whose icon is colored by an action tone. Mirrors
/// `icon_button` anatomy (square, ghost, hover bg) but applies the tone
/// color, which `icon_button` does not carry.
fn toned_icon_button(
    action: &BulkAction,
    size: ControlSize,
    icon_color: ColorValue,
    disabled: bool,
    theme: &dyn ThemeProvider,
    on_press: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(size, SemanticControlSizeRole::Control);
    let height = rem_to_px(control_height_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));
    let radius = theme.resolve_radius("radius.control");

    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let hover_bg = mix_srgb(surface, elevated, 0.84);

    let mut el = Node::button("");
    // Same reason as the default-tone branch: icon-only, and the action
    // already carries the words.
    el.a11y.label = Some(action.label.clone());
    {
        let s = &mut el.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.width = LayoutSizing::Fixed(height);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
        s.hover = Some(StylePatch {
            background: Some(hover_bg),
            border_color: None,
            text_color: None,
            opacity: None,
        });
    }
    el.interaction.focusable = true;

    let mut glyph = Node::icon(action.resolved_icon(), icon_size);
    glyph.style.descriptor.text_color = Some(icon_color);
    let mut el = el.child(glyph);

    if disabled {
        el.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        el.interaction.disabled = true;
    } else if let Some(handler) = on_press {
        el.interaction.on_activate = Some(Arc::new(move || handler()));
    }

    el
}

pub fn bulk_action_bar(
    spec: &BulkActionBarSpec,
    theme: &dyn ThemeProvider,
    handlers: BulkActionBarHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let summary_font = rem_to_px(summary_font_rem(effective_size));

    // Padding: horizontal density-driven, vertical flat 0.5rem (contract §8).
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.5);

    // Gaps: root/summary gap = space.inline.md (root), summary inner & actions per spec.
    let root_gap = match spec.density {
        ControlDensity::Compact => rem_to_px(0.375),
        ControlDensity::Default => theme.resolve_space("space.inline.md"),
        ControlDensity::Comfortable => rem_to_px(1.0),
    };
    let summary_gap = theme.resolve_space(spec.gap_token());
    let actions_gap = actions_gap_px(spec, theme);

    // ── Colors ──────────────────────────────────────────────────
    let panel_bg = theme.resolve_color("color.background.panel");
    let text_primary = theme.resolve_color(spec.text_token());
    // Contract §8 accent fill: color-mix(panel 93%, text-primary).
    let fill = mix_srgb(panel_bg, text_primary, 0.93);
    let border = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let total_color = theme.resolve_color(spec.total_text_token());
    let danger_color = theme.resolve_color(spec.danger_text_token());
    let warning_color = theme.resolve_color(spec.warning_text_token());

    // Shared availability gates (Svelte isUnavailable / actionsDisabled).
    let is_unavailable = spec.is_unavailable();
    let actions_disabled = spec.actions_disabled();

    // ── Summary (left) ──────────────────────────────────────────
    let mut summary = Node::container();
    {
        let s = &mut summary.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = summary_gap;
    }

    let mut count = Node::text(format!("{} selected", spec.selection_count));
    count.style.descriptor.text_color = Some(text_primary);
    count.style.text_size = Some(summary_font);
    count.style.text_weight = Some(600);
    summary = summary.child(count);

    if let Some(total) = spec.total_count {
        let mut of_total = Node::text(format!("of {total}"));
        of_total.style.descriptor.text_color = Some(total_color);
        of_total.style.text_size = Some(summary_font);
        summary = summary.child(of_total);
    }

    // Select-all: ghost `check-check` icon_button, chrome size-role.
    if spec.show_select_all && !spec.all_selected {
        let mut select_spec = IconButtonSpec::new()
            .with_aria_label("Select all")
            .with_icon("check-check")
            .with_size(spec.size)
            .with_size_role(SemanticControlSizeRole::Chrome);
        if is_unavailable {
            select_spec = select_spec.with_disabled(true);
        }
        summary = summary.child(icon_button(
            &select_spec,
            theme,
            handlers.on_select_all.clone(),
        ));
    }

    // ── Actions (right): ghost icon_buttons + clear "x" ─────────
    let mut actions = Node::container();
    {
        let s = &mut actions.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = actions_gap;
    }

    for action in &spec.actions {
        let disabled = actions_disabled || action.is_disabled;
        let on_press = if disabled {
            None
        } else {
            handlers.on_action.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let id = action.id.clone();
                Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
            })
        };
        match action.tone {
            BulkActionTone::Danger => {
                actions = actions.child(toned_icon_button(
                    action,
                    spec.size,
                    danger_color,
                    disabled,
                    theme,
                    on_press,
                ));
            }
            BulkActionTone::Warning => {
                actions = actions.child(toned_icon_button(
                    action,
                    spec.size,
                    warning_color,
                    disabled,
                    theme,
                    on_press,
                ));
            }
            BulkActionTone::Default => {
                // Default tone uses the shared icon_button (text-primary icon).
                let mut a_spec = IconButtonSpec::new()
                    // The action already carries the words for this control;
                    // rendering it icon-only would leave a bar of identical
                    // unnamed buttons.
                    .with_aria_label(action.label.clone())
                    .with_icon(action.resolved_icon())
                    .with_size(spec.size)
                    .with_size_role(spec.size_role);
                if disabled {
                    a_spec = a_spec.with_disabled(true);
                }
                actions = actions.child(icon_button(&a_spec, theme, on_press));
            }
        }
    }

    // Clear-selection (`x`) ghost icon_button — contract §2 (always present).
    let mut clear_spec = IconButtonSpec::new()
        .with_aria_label("Clear selection")
        .with_icon("x")
        .with_size(spec.size)
        .with_size_role(spec.size_role);
    if is_unavailable {
        clear_spec = clear_spec.with_disabled(true);
    }
    actions = actions.child(icon_button(&clear_spec, theme, handlers.on_clear.clone()));

    // ── Root ────────────────────────────────────────────────────
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = root_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = border;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
    }
    let mut root = root.child(summary).child(actions);

    // Loading/disabled dim the whole bar (contract §4).
    if is_unavailable {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    root.a11y.role = Some(NodeRole::Region);
    root
}
