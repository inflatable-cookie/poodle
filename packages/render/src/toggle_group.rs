//! ToggleGroup — a row of options, single- or multi-select.
//!
//! Contract: `docs/contracts/components/toggle-group.md`
//! Ported from: `packages/jetstream/components/src/toggle_group.rs`.
//!
//! Selection always flows through `toggle_group_transition`. The callback
//! receives the owned resulting `ToggleGroupValue`, not the activated option.

use std::sync::Arc;

use poodle_headless::single_select::SelectOption;
use poodle_headless::toggle_group::{
    toggle_group_transition, SelectionMode, ToggleGroupContext, ToggleGroupEffect,
    ToggleGroupEvent, ToggleGroupValue,
};
use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, MainAxisAlignment, Node, NodeKey,
    NodeRole, NodeToggled, StylePatch,
};
use poodle_specs::{ToggleGroupSelectionMode, ToggleGroupSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, rem_to_px, size_font_rem, toggle_group_gap_rem};

/// Host-owned native interaction for one ToggleGroup instance.
///
/// `instance_id` is the lifetime-stable scope. It is construction data, not a
/// semantic option value, and the renderer never invents one from render
/// order or option values.
#[derive(Clone)]
pub struct ToggleGroupHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(ToggleGroupValue) + Send + Sync>>,
}

impl ToggleGroupHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(ToggleGroupValue) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }
}

fn option_id(value: &str) -> String {
    format!("toggle:{value}")
}

fn option_focus_id(instance_scope: &str, value: &str) -> String {
    format!("toggle:{instance_scope}:option:{value}")
}

fn headless_context(spec: &ToggleGroupSpec) -> ToggleGroupContext {
    let selection_mode = match spec.selection_mode {
        ToggleGroupSelectionMode::Single => SelectionMode::Single,
        ToggleGroupSelectionMode::Multiple => SelectionMode::Multiple,
    };
    let selected = spec.selected_values();
    let value = match selection_mode {
        // Spec stores both modes as a vector. Single mode keeps the first
        // stored value; extra members cannot round-trip through the headless
        // enum and are not a legal public selection.
        SelectionMode::Single => ToggleGroupValue::Single(selected.first().cloned()),
        SelectionMode::Multiple => ToggleGroupValue::Multiple(selected.to_vec()),
    };
    ToggleGroupContext {
        value,
        options: spec
            .options
            .iter()
            .map(|option| SelectOption {
                value: option.value.clone(),
                disabled: option.is_disabled,
            })
            .collect(),
        selection_mode,
        allow_deactivation: spec.allow_deactivation,
        disabled: spec.is_disabled,
    }
}

fn option_selected(context: &ToggleGroupContext, option_value: &str) -> bool {
    match &context.value {
        ToggleGroupValue::Multiple(values) => values.iter().any(|value| value == option_value),
        ToggleGroupValue::Single(Some(value)) => value == option_value,
        ToggleGroupValue::Single(None) => false,
    }
}

fn roving_values(context: &ToggleGroupContext) -> Vec<String> {
    if context.selection_mode != SelectionMode::Single || context.disabled {
        return Vec::new();
    }
    context
        .options
        .iter()
        .filter(|option| !option.disabled)
        .map(|option| option.value.clone())
        .collect()
}

fn tab_stop_value<'a>(context: &'a ToggleGroupContext, roving: &'a [String]) -> Option<&'a str> {
    if context.selection_mode != SelectionMode::Single {
        return None;
    }
    match &context.value {
        ToggleGroupValue::Single(Some(value))
            if roving.iter().any(|candidate| candidate == value) =>
        {
            Some(value.as_str())
        }
        _ => roving.first().map(String::as_str),
    }
}

fn emit_toggle(
    context: &ToggleGroupContext,
    option_value: &str,
    on_value_change: &Option<Arc<dyn Fn(ToggleGroupValue) + Send + Sync>>,
) {
    let Some(handler) = on_value_change else {
        return;
    };
    let (_, effects) = toggle_group_transition(
        context.clone(),
        ToggleGroupEvent::Toggle {
            value: option_value.to_string(),
        },
    );
    for effect in effects {
        let ToggleGroupEffect::EmitValueChange { value } = effect;
        handler(value);
    }
}

fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    context: ToggleGroupContext,
    on_value_change: Option<Arc<dyn Fn(ToggleGroupValue) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = match key {
            NodeKey::ArrowRight => {
                if index == last {
                    0
                } else {
                    index + 1
                }
            }
            NodeKey::ArrowLeft => {
                if index == 0 {
                    last
                } else {
                    index - 1
                }
            }
            _ => return None,
        };
        let target = ids[next].clone();
        if target == ids[index] {
            return None;
        }
        emit_toggle(&context, &target, &on_value_change);
        Some(option_focus_id(&instance_scope, &target))
    }))
}

pub fn toggle_group(
    spec: &ToggleGroupSpec,
    ctx: &RenderContext<'_>,
    handlers: ToggleGroupHandlers,
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
    let instance_scope = handlers.instance_id.as_str();
    let context = headless_context(spec);
    let is_single = context.selection_mode == SelectionMode::Single;
    let roving = roving_values(&context);
    let tab_stop = tab_stop_value(&context, &roving);
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };
    for option in &spec.options {
        let is_selected = option_selected(&context, &option.value);
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
        item.id = Some(option_id(&option.value));
        item.runtime_id = Some(option_focus_id(instance_scope, &option.value));
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

        if is_item_disabled {
            item.style.descriptor.opacity =
                ctx.theme().resolve_opacity(spec.disabled_opacity_token());
            item.style.descriptor.cursor = CursorHint::NotAllowed;
            item.interaction.disabled = true;
            item.interaction.focusable = false;
            item.a11y.tab_index = Some(-1);
        } else {
            item.interaction.focusable = true;
            item.a11y.tab_index = Some(if is_single {
                if tab_stop == Some(option.value.as_str()) {
                    0
                } else {
                    -1
                }
            } else {
                0
            });
            item.style.focus_ring = Some(focus_ring);
            let hover_fill = mix_srgb(bg, elevated, 0.84);
            item.style.hover = Some(StylePatch {
                background: Some(hover_fill),
                border_color: None,
                text_color: None,
                opacity: None,
            });
            item.style.descriptor.cursor = CursorHint::Pointer;

            if handlers.on_value_change.is_some() {
                let context = context.clone();
                let on_value_change = handlers.on_value_change.clone();
                let value = option.value.clone();
                item.interaction.on_activate = Some(Arc::new(move || {
                    emit_toggle(&context, &value, &on_value_change);
                }));
            }
            if is_single {
                item.interaction.on_key = roving_key_handler(
                    &option.value,
                    &roving,
                    instance_scope.to_string(),
                    context.clone(),
                    handlers.on_value_change.clone(),
                );
            }
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

    fn render(spec: &ToggleGroupSpec) -> Node {
        toggle_group(
            spec,
            &RenderContext::new(&theme()),
            ToggleGroupHandlers::new("view"),
        )
    }

    fn render_with(spec: &ToggleGroupSpec, handlers: ToggleGroupHandlers) -> Node {
        toggle_group(spec, &RenderContext::new(&theme()), handlers)
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
            let node = render(&spec);
            assert_eq!(
                item(&node, "grid").style.min_height,
                Some(expected),
                "min-height for {size:?}"
            );
        }

        // Horizontal padding is the `space.control.x` token, density-only —
        // no per-size offset (the old GPUI tier's `resolve_px`).
        let theme = theme();
        let pad_x = poodle_adapter::ThemeProvider::resolve_space(&theme, "space.control.x");
        for size in [ControlSize::Xs, ControlSize::Md, ControlSize::Xl] {
            let spec = ToggleGroupSpec::new(view_options()).with_size(size);
            let node = render(&spec);
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
            let node = render(&spec);
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
            let node = render(&spec);
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
        let node = render(&spec);

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
    fn selection_mode_sets_roles_and_activation_reports_the_result() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<ToggleGroupValue>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_value_change: Arc<dyn Fn(ToggleGroupValue) + Send + Sync> =
            Arc::new(move |value| sink.lock().unwrap().push(value));

        let spec = ToggleGroupSpec::new(view_options()).with_value(vec!["grid".into()]);
        let node = render_with(
            &spec,
            ToggleGroupHandlers::new("view").on_value_change(Arc::clone(&on_value_change)),
        );
        assert_eq!(node.a11y.role, Some(NodeRole::RadioGroup));
        let list = item(&node, "list");
        assert_eq!(list.a11y.role, Some(NodeRole::RadioButton));
        (list.interaction.on_activate.as_ref().expect("activatable"))();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [ToggleGroupValue::Single(Some("list".into()))]
        );

        (item(&node, "grid")
            .interaction
            .on_activate
            .as_ref()
            .expect("same-value still activates"))();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [
                ToggleGroupValue::Single(Some("list".into())),
                ToggleGroupValue::Single(Some("grid".into())),
            ]
        );

        let spec = ToggleGroupSpec::new(view_options())
            .with_selection_mode(ToggleGroupSelectionMode::Multiple)
            .with_value(vec!["grid".into()]);
        let node = render_with(
            &spec,
            ToggleGroupHandlers::new("tags").on_value_change(on_value_change),
        );
        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        assert_eq!(item(&node, "grid").a11y.role, Some(NodeRole::Button));
        (item(&node, "board")
            .interaction
            .on_activate
            .as_ref()
            .expect("activatable"))();
        assert_eq!(
            seen.lock().unwrap().last(),
            Some(&ToggleGroupValue::Multiple(vec![
                "grid".into(),
                "board".into()
            ]))
        );
    }

    #[test]
    fn disabled_items_paint_the_disabled_recipe_and_do_not_activate() {
        let disabled_opacity =
            poodle_adapter::ThemeProvider::resolve_opacity(&theme(), "state.opacity.disabled");

        let spec = ToggleGroupSpec::new(vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List").with_disabled(true),
        ])
        .with_aria_label("View");
        let node = render_with(
            &spec,
            ToggleGroupHandlers::new("view")
                .on_value_change(Arc::new(|_| panic!("disabled items never fire"))),
        );
        assert_eq!(node.a11y.label.as_deref(), Some("View"));

        let disabled = item(&node, "list");
        assert_eq!(disabled.style.descriptor.opacity, disabled_opacity);
        assert_eq!(disabled.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(disabled.interaction.disabled);
        assert!(disabled.interaction.on_activate.is_none());
        assert!(disabled.style.hover.is_none());
        assert!(disabled.style.focus_ring.is_none());

        let enabled = item(&node, "grid");
        assert_eq!(enabled.style.descriptor.cursor, CursorHint::Pointer);
        assert!(enabled.interaction.on_activate.is_some());
        assert!(enabled.style.focus_ring.is_some());

        let spec = ToggleGroupSpec::new(view_options()).with_disabled(true);
        let node = render(&spec);
        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        let grid = item(&node, "grid");
        assert!(grid.interaction.disabled);
        assert_eq!(grid.style.descriptor.opacity, disabled_opacity);
        assert_eq!(grid.a11y.tab_index, Some(-1));
        assert!(!grid.interaction.focusable);
    }

    #[test]
    fn selected_option_is_the_single_mode_tab_stop() {
        let spec = ToggleGroupSpec::new(view_options()).with_value(vec!["list".into()]);
        let node = render(&spec);
        assert_eq!(item(&node, "grid").a11y.tab_index, Some(-1));
        assert_eq!(item(&node, "list").a11y.tab_index, Some(0));
        assert_eq!(item(&node, "board").a11y.tab_index, Some(-1));
    }

    #[test]
    fn unknown_or_disabled_selection_falls_back_to_the_first_enabled_option() {
        let unknown = ToggleGroupSpec::new(view_options()).with_value(vec!["missing".into()]);
        assert_eq!(item(&render(&unknown), "grid").a11y.tab_index, Some(0));

        let selected_disabled = ToggleGroupSpec::new(vec![
            ToggleGroupOption::new("grid", "Grid").with_disabled(true),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ])
        .with_value(vec!["grid".into()]);
        let node = render(&selected_disabled);
        assert_eq!(item(&node, "grid").a11y.tab_index, Some(-1));
        assert!(!item(&node, "grid").interaction.focusable);
        assert_eq!(item(&node, "list").a11y.tab_index, Some(0));
    }

    #[test]
    fn left_right_wrap_skips_disabled_and_emits_the_result() {
        use poodle_node::NodeModifiers;
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<ToggleGroupValue>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = ToggleGroupSpec::new(vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List").with_disabled(true),
            ToggleGroupOption::new("board", "Board"),
        ])
        .with_value(vec!["grid".into()]);
        let node = render_with(
            &spec,
            ToggleGroupHandlers::new("view")
                .on_value_change(Arc::new(move |value| sink.lock().unwrap().push(value))),
        );
        let keys = item(&node, "grid")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let modifiers = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowRight, modifiers),
            Some(option_focus_id("view", "board"))
        );
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [ToggleGroupValue::Single(Some("board".into()))]
        );
        assert!(keys(NodeKey::ArrowUp, modifiers).is_none());
        assert!(keys(NodeKey::ArrowDown, modifiers).is_none());
        assert!(item(&node, "list").interaction.on_key.is_none());

        let wrap = item(&node, "board")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        assert_eq!(
            wrap(NodeKey::ArrowRight, modifiers),
            Some(option_focus_id("view", "grid"))
        );
    }

    #[test]
    fn multiple_mode_keeps_every_enabled_item_tabbable_and_ignores_arrows() {
        let spec = ToggleGroupSpec::new(view_options())
            .with_selection_mode(ToggleGroupSelectionMode::Multiple)
            .with_value(vec!["grid".into()]);
        let node = render(&spec);
        assert_eq!(item(&node, "grid").a11y.tab_index, Some(0));
        assert_eq!(item(&node, "list").a11y.tab_index, Some(0));
        assert!(item(&node, "grid").interaction.on_key.is_none());
        assert!(item(&node, "list").interaction.on_key.is_none());
    }

    #[test]
    fn instance_scope_keeps_roving_focus_inside_the_originating_control() {
        use poodle_node::NodeModifiers;
        let spec = ToggleGroupSpec::new(view_options()).with_value(vec!["grid".into()]);
        let a = render_with(&spec, ToggleGroupHandlers::new("left"));
        let b = render_with(&spec, ToggleGroupHandlers::new("right"));
        let a_grid = item(&a, "grid");
        let b_grid = item(&b, "grid");
        assert_eq!(a_grid.id.as_deref(), Some("toggle:grid"));
        assert_eq!(b_grid.id.as_deref(), Some("toggle:grid"));
        assert_eq!(
            a_grid.runtime_id.as_deref(),
            Some("toggle:left:option:grid")
        );
        assert_eq!(
            b_grid.runtime_id.as_deref(),
            Some("toggle:right:option:grid")
        );
        let modifiers = NodeModifiers::default();
        assert_eq!(
            (a_grid.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowRight, modifiers),
            Some("toggle:left:option:list".to_string())
        );
        assert_eq!(
            (b_grid.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowRight, modifiers),
            Some("toggle:right:option:list".to_string())
        );
    }

    #[test]
    fn allow_deactivation_emits_single_none() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<ToggleGroupValue>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = ToggleGroupSpec::new(view_options())
            .with_value(vec!["grid".into()])
            .with_allow_deactivation(true);
        let node = render_with(
            &spec,
            ToggleGroupHandlers::new("view")
                .on_value_change(Arc::new(move |value| sink.lock().unwrap().push(value))),
        );
        (item(&node, "grid")
            .interaction
            .on_activate
            .as_ref()
            .unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [ToggleGroupValue::Single(None)]
        );
    }
}
