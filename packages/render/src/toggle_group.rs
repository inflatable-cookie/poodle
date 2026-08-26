//! ToggleGroup — a row of options, single- or multi-select.
//!
//! Contract: `docs/contracts/components/toggle-group.md`
//! Ported from: `packages/jetstream/components/src/toggle_group.rs`.
//!
//! `on_change` fires with the value of the option that was activated — the
//! option, not the resulting selection: in multi-select the host owns the
//! set.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, MainAxisAlignment, Node, NodeRole,
    NodeToggled, StylePatch,
};
use poodle_specs::ToggleGroupSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, rem_to_px, size_font_rem, toggle_group_gap_rem};

pub fn toggle_group(
    spec: &ToggleGroupSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // ── Token resolution ──
    let accent = ctx.theme().resolve_color("color.accent.base");
    let border_subtle = ctx.theme().resolve_color("color.border.subtle");
    let border_default = ctx.theme().resolve_color("color.border.default");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let surface = ctx.theme().resolve_color("color.background.surface");
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let radius = ctx.theme().resolve_radius("radius.control");

    // Contract §8 Root: gap is density-driven — the old GPUI tier's ladder
    // (compact 0.1875 / default 0.25 / comfortable 0.375rem), which
    // `toggle_group_gap_rem` transcribes exactly.
    let gap = rem_to_px(toggle_group_gap_rem(density));

    // Contract: min-height = calc(control-height − 0.25rem). Unlike
    // select/button, this component's old GPUI tier is deliberately
    // fixed-table here — `control_height_rem(size) − 0.25rem`, Svelte's
    // per-size stops verbatim — so the height does NOT follow the
    // density/control-size axis. Exact transcription (zero-diff at the
    // visual gate) outranks the token + offset form.
    let item_height = rem_to_px(control_height_rem(effective_size) - 0.25);
    // Contract §8 Item: padding `0 toggle-group-x`. The old GPUI tier
    // resolves the `space.control.x` token directly — density-only, no
    // per-size offset — so the density axis carries through the theme.
    let item_pad_x = ctx.theme().resolve_space("space.control.x");

    // Contract: item border-color = color-mix(border-subtle 82%, transparent).
    let item_border_color = with_alpha(border_subtle, border_subtle.3 * 0.82);

    // Contract: item background = color-mix(surface 93%, text-primary).
    let item_fill = mix_srgb(surface, text_primary, 0.93);

    // Contract: selected = accent tinted at 22% over the item fill.
    let selected_fill = mix_srgb(accent, item_fill, 0.22);

    // Contract: selected border = color-mix(accent-base 42%, border-default).
    let selected_border = mix_srgb(accent, border_default, 0.42);

    // Contract §8 Item: font-size follows the per-size label ladder (the old
    // GPUI tier's `size_font_rem`; md stop = typography-label-size, 13px at
    // base), font-weight 600, border 0.0625rem.
    let font_size = rem_to_px(size_font_rem(effective_size));
    let border_width = rem_to_px(0.0625);

    // ── Root container ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }

    // ── Build items ──
    let is_single = matches!(
        spec.selection_mode,
        poodle_specs::ToggleGroupSelectionMode::Single
    );
    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        let (bg, bc) = if is_selected {
            (selected_fill, selected_border)
        } else {
            (item_fill, item_border_color)
        };

        let mut item = Node::button(&option.label);
        // Contract: selection mode decides. Single-select options are
        // `radio`s; multi-select options stay buttons that toggle.
        item.a11y.role = Some(if is_single {
            NodeRole::RadioButton
        } else {
            NodeRole::Button
        });
        item.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        // Hit-test id so a host can route option activation.
        item.id = Some(format!("toggle:{}", option.value));
        {
            let s = &mut item.style;
            s.min_height = Some(item_height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = item_pad_x;
            pad.right = item_pad_x;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = bc;
            s.descriptor.text_color = Some(text_primary);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }

        if !is_item_disabled {
            item.interaction.focusable = true;
            item.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                text_color: None,
                opacity: None,
            });
            let hover_fill = mix_srgb(bg, elevated, 0.84);
            item.style.hover = Some(StylePatch {
                background: Some(hover_fill),
                border_color: None,
                text_color: None,
                opacity: None,
            });
            item.style.descriptor.cursor = CursorHint::Pointer;

            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = option.value.clone();
                item.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
        } else {
            item.style.descriptor.opacity =
                ctx.theme().resolve_opacity(spec.disabled_opacity_token());
            // The old GPUI tier's `CursorStyle::OperationNotAllowed`.
            item.style.descriptor.cursor = CursorHint::NotAllowed;
            item.interaction.disabled = true;
        }

        root = root.child(item);
    }

    // ── Group-level disabled ──
    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    // Contract: the group is a `radiogroup` when selection is single.
    root.a11y.role = Some(if is_single {
        NodeRole::RadioGroup
    } else {
        NodeRole::Group
    });
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ControlDensity, ControlSize, ToggleGroupOption, ToggleGroupSelectionMode};

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn view_options() -> Vec<ToggleGroupOption> {
        vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ]
    }

    fn item<'a>(node: &'a Node, value: &str) -> &'a Node {
        node.find(&|n| n.id.as_deref() == Some(format!("toggle:{value}").as_str()))
            .unwrap_or_else(|| panic!("item {value} exists"))
    }

    #[test]
    fn item_metrics_follow_the_old_tiers_recipe() {
        // min-height = control_height_rem(size) − 0.25rem — the old GPUI
        // tier's fixed per-size table (Svelte's stops), deliberately NOT the
        // token + offset ladder select/button use.
        let cases = [
            (ControlSize::Xs, 20.0),
            (ControlSize::Sm, 24.0),
            (ControlSize::Md, 32.0),
            (ControlSize::Lg, 40.0),
            (ControlSize::Xl, 48.0),
        ];
        for (size, expected) in cases {
            let spec = ToggleGroupSpec::new(view_options()).with_size(size);
            let theme = theme();
            let ctx = RenderContext::new(&theme);
            let node = toggle_group(&spec, &ctx, None);
            assert_eq!(
                item(&node, "grid").style.min_height,
                Some(expected),
                "min-height for {size:?}"
            );
        }

        // Horizontal padding is the `space.control.x` token, density-only —
        // no per-size offset (the old GPUI tier's `resolve_px`).
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let pad_x = poodle_adapter::ThemeProvider::resolve_space(&theme, "space.control.x");
        for size in [ControlSize::Xs, ControlSize::Md, ControlSize::Xl] {
            let spec = ToggleGroupSpec::new(view_options()).with_size(size);
            let node = toggle_group(&spec, &ctx, None);
            let pad = &item(&node, "grid").style.descriptor.layout.spacing.padding;
            assert_eq!(pad.left, pad_x, "pad-left for {size:?}");
            assert_eq!(pad.right, pad_x, "pad-right for {size:?}");
        }
    }

    #[test]
    fn gap_and_font_follow_the_density_and_size_ladders() {
        // The old GPUI tier's density ladder for the root gap.
        let gap_cases = [
            (ControlDensity::Compact, 3.0),
            (ControlDensity::Default, 4.0),
            (ControlDensity::Comfortable, 6.0),
        ];
        for (density, expected) in gap_cases {
            let spec = ToggleGroupSpec::new(view_options()).with_density(density);
            let theme = theme();
            let ctx = RenderContext::new(&theme);
            let node = toggle_group(&spec, &ctx, None);
            assert_eq!(
                node.style.descriptor.layout.spacing.gap, expected,
                "gap for {density:?}"
            );
        }

        // The old GPUI tier's per-size label ladder (`size_font_rem`), not a
        // flat typography-label-size.
        let font_cases = [
            (ControlSize::Xs, 11.0),
            (ControlSize::Sm, 12.0),
            (ControlSize::Md, 13.0),
            (ControlSize::Lg, 14.0),
            (ControlSize::Xl, 15.0),
        ];
        for (size, expected) in font_cases {
            let spec = ToggleGroupSpec::new(view_options()).with_size(size);
            let theme = theme();
            let ctx = RenderContext::new(&theme);
            let node = toggle_group(&spec, &ctx, None);
            assert_eq!(
                item(&node, "grid").style.text_size,
                Some(expected),
                "font size for {size:?}"
            );
        }
    }

    #[test]
    fn unselected_and_selected_items_use_the_old_tiers_recipes() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let surface =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.background.surface");
        let elevated =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.background.elevated");
        let text_primary =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.text.primary");
        let border_subtle =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.border.subtle");
        let border_default =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.border.default");
        let accent = poodle_adapter::ThemeProvider::resolve_color(&theme, "color.accent.base");
        let radius = poodle_adapter::ThemeProvider::resolve_radius(&theme, "radius.control");

        let item_fill = mix_srgb(surface, text_primary, 0.93);
        let item_border = with_alpha(border_subtle, border_subtle.3 * 0.82);
        let selected_fill = mix_srgb(accent, item_fill, 0.22);
        let selected_border = mix_srgb(accent, border_default, 0.42);

        let spec = ToggleGroupSpec::new(view_options()).with_value(vec!["list".into()]);
        let node = toggle_group(&spec, &ctx, None);

        let unselected = item(&node, "grid");
        assert_eq!(unselected.style.descriptor.background, Some(item_fill));
        assert_eq!(unselected.style.descriptor.border.color, item_border);
        assert_eq!(unselected.style.descriptor.border.width, 1.0);
        assert_eq!(unselected.style.descriptor.text_color, Some(text_primary));
        assert_eq!(unselected.style.text_weight, Some(600));
        assert_eq!(unselected.style.descriptor.corner_radii.top_left, radius);
        // Hover mixes the state fill toward elevated at 84%.
        assert_eq!(
            unselected.style.hover.and_then(|h| h.background),
            Some(mix_srgb(item_fill, elevated, 0.84))
        );
        assert_eq!(unselected.a11y.toggled, Some(NodeToggled::False));

        let selected = item(&node, "list");
        assert_eq!(selected.style.descriptor.background, Some(selected_fill));
        assert_eq!(selected.style.descriptor.border.color, selected_border);
        assert_eq!(
            selected.style.hover.and_then(|h| h.background),
            Some(mix_srgb(selected_fill, elevated, 0.84))
        );
        assert_eq!(selected.a11y.toggled, Some(NodeToggled::True));
    }

    #[test]
    fn selection_mode_sets_roles_and_activation_reports_the_value() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));

        // Single-select: radiogroup of radios.
        let spec = ToggleGroupSpec::new(view_options());
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = toggle_group(&spec, &ctx, Some(Arc::clone(&on_change)));
        assert_eq!(node.a11y.role, Some(NodeRole::RadioGroup));
        let list = item(&node, "list");
        assert_eq!(list.a11y.role, Some(NodeRole::RadioButton));
        assert!(list.style.focus.is_some());
        (list.interaction.on_activate.as_ref().expect("activatable"))();
        assert_eq!(seen.lock().unwrap().as_slice(), ["list"]);

        // Multi-select: a group of toggling buttons.
        let spec = ToggleGroupSpec::new(view_options())
            .with_selection_mode(ToggleGroupSelectionMode::Multiple);
        let node = toggle_group(&spec, &ctx, None);
        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        assert_eq!(item(&node, "grid").a11y.role, Some(NodeRole::Button));
    }

    #[test]
    fn disabled_items_paint_the_disabled_recipe_and_do_not_activate() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let disabled_opacity =
            poodle_adapter::ThemeProvider::resolve_opacity(&theme, "state.opacity.disabled");

        let spec = ToggleGroupSpec::new(vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List").with_disabled(true),
        ])
        .with_aria_label("View");
        let node = toggle_group(
            &spec,
            &ctx,
            Some(Arc::new(|_: &str| panic!("disabled items never fire"))),
        );
        assert_eq!(node.a11y.label.as_deref(), Some("View"));

        let disabled = item(&node, "list");
        assert_eq!(disabled.style.descriptor.opacity, disabled_opacity);
        assert_eq!(disabled.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(disabled.interaction.disabled);
        assert!(!disabled.interaction.focusable);
        assert!(disabled.interaction.on_activate.is_none());
        assert!(disabled.style.focus.is_none());
        assert!(disabled.style.hover.is_none());

        let enabled = item(&node, "grid");
        assert_eq!(enabled.style.descriptor.cursor, CursorHint::Pointer);
        assert!(enabled.interaction.on_activate.is_some());

        // Group-level disabled dims the whole root too (the old GPUI tier
        // applies both, item and root).
        let spec = ToggleGroupSpec::new(view_options()).with_disabled(true);
        let node = toggle_group(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        let grid = item(&node, "grid");
        assert!(grid.interaction.disabled);
        assert_eq!(grid.style.descriptor.opacity, disabled_opacity);
    }
}
