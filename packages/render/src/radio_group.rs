//! RadioGroup — one choice from a list.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Ported from: `packages/jetstream/components/src/radio_group.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeKey, NodeRole, NodeToggled, StylePatch,
};
use poodle_specs::{ControlDensity, ControlSize, Orientation, RadioGroupSpec};

use crate::color::hex_color;
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};

/// Indicator (outer circle) size — contract §8 table over the icon-md token.
fn indicator_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px - rem_to_px(0.125),
        ControlSize::Sm => icon_md_px,
        ControlSize::Md => rem_to_px(1.125),
        ControlSize::Lg => icon_md_px + rem_to_px(0.375),
        ControlSize::Xl => icon_md_px + rem_to_px(0.625),
    }
}

/// Dot (inner circle) size — contract §8 ratios over icon-md; md explicit.
fn dot_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px * 0.40,
        ControlSize::Sm => icon_md_px * 0.45,
        ControlSize::Md => rem_to_px(0.5),
        ControlSize::Lg => icon_md_px * 0.55,
        ControlSize::Xl => icon_md_px * 0.60,
    }
}

fn circle(node: &mut Node, diameter: f32) {
    let s = &mut node.style;
    s.descriptor.layout.width = LayoutSizing::Fixed(diameter);
    s.descriptor.layout.height = LayoutSizing::Fixed(diameter);
    let r = diameter * 0.5;
    s.descriptor.corner_radii.top_left = r;
    s.descriptor.corner_radii.top_right = r;
    s.descriptor.corner_radii.bottom_right = r;
    s.descriptor.corner_radii.bottom_left = r;
}

pub fn radio_group(
    spec: &RadioGroupSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_md = ctx.theme().resolve_space("size.icon.md");
    let indicator_size = indicator_size_px(effective_size, icon_md);
    let dot_size = dot_size_px(effective_size, icon_md);
    let border_width = rem_to_px(0.0625);

    // Density override wins over the orientation gap.
    let group_gap = match density {
        ControlDensity::Compact => ctx.theme().resolve_space("space.stack.sm"),
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.stack.lg"),
        ControlDensity::Default => ctx.theme().resolve_space(spec.option_gap_token()),
    };
    let item_gap = ctx.theme().resolve_space("space.inline.sm");

    // Custom hex wins over accent. Colour-space note as in checkbox: the hex
    // lands in sRGB and converts at the backend edge — the old tier passed it
    // raw; divergence pinned in the parity suite.
    let accent = spec
        .selected_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| ctx.theme().resolve_color("color.accent.base"));
    let border = ctx.theme().resolve_color("color.border.default");
    let text_color = ctx.theme().resolve_color("color.text.primary");
    let selected_value = spec.current_value();
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    let instance_scope = spec.name.as_deref().unwrap_or("group");
    let roving = roving_values(spec);
    let tab_stop = tab_stop_value(spec, &roving);
    let focus_ring = ctx.theme().resolve_color("color.accent.focusRing");

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = match spec.orientation {
            Orientation::Horizontal => LayoutDirection::Row,
            Orientation::Vertical => LayoutDirection::Column,
        };
        s.descriptor.layout.spacing.gap = group_gap;
    }

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = ctx.theme().resolve_color("color.background.surface");

        let mut indicator = Node::container();
        circle(&mut indicator, indicator_size);
        {
            let s = &mut indicator.style;
            s.descriptor.background = Some(indicator_bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = indicator_color;
            // Explicit Row (see switch.rs): the old tier got taffy's Row default.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }

        if is_selected {
            let mut dot = Node::container();
            circle(&mut dot, dot_size);
            dot.style.descriptor.background = Some(accent);
            indicator = indicator.child(dot);
        }

        let option_disabled = spec.is_disabled || option.is_disabled;

        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = item_gap;
            s.descriptor.cursor = if option_disabled {
                CursorHint::Default
            } else {
                CursorHint::Pointer
            };
        }
        row = row.child(indicator);

        let mut label = Node::text(&option.label);
        label.style.descriptor.text_color = Some(text_color);
        label.style.text_size = Some(font_size);
        row = row.child(label);

        // Per-option disabled dims that row only.
        if option.is_disabled {
            row.style.descriptor.opacity = disabled_opacity;
        }

        let option_id = option_focus_id(instance_scope, &option.value);
        row.id = Some(option_id.clone());
        row.runtime_id = Some(option_id);
        row.a11y.role = Some(NodeRole::RadioButton);
        row.a11y.selected = Some(is_selected);
        row.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        if let Some(name) = option.aria_label.as_deref() {
            row.a11y.label = Some(name.to_string());
        }

        if !option_disabled {
            row.interaction.focusable = true;
            row.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(focus_ring),
                text_color: None,
                opacity: None,
            });
            row.a11y.tab_index = Some(if tab_stop == Some(option.value.as_str()) {
                0
            } else {
                -1
            });
            // Same-value selection is inert: native radios do not re-fire.
            if !is_selected {
                if let Some(handler) = &on_change {
                    let handler = Arc::clone(handler);
                    let value = option.value.clone();
                    row.interaction.on_activate = Some(Arc::new(move || handler(&value)));
                }
            }
            row.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                instance_scope.to_string(),
                on_change.clone(),
            );
        } else {
            row.interaction.disabled = true;
            row.interaction.focusable = false;
            row.a11y.tab_index = Some(-1);
        }

        el = el.child(row);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::RadioGroup);
    el
}

fn option_focus_id(instance_scope: &str, value: &str) -> String {
    format!("radio:{instance_scope}:option:{value}")
}

fn roving_values(spec: &RadioGroupSpec) -> Vec<String> {
    spec.options
        .iter()
        .filter(|option| !spec.is_disabled && !option.is_disabled)
        .map(|option| option.value.clone())
        .collect()
}

fn tab_stop_value<'a>(spec: &'a RadioGroupSpec, roving: &'a [String]) -> Option<&'a str> {
    let selected = spec.current_value();
    if selected.is_some_and(|value| roving.iter().any(|candidate| candidate == value)) {
        selected
    } else {
        roving.first().map(String::as_str)
    }
}

fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
    let current = value.to_string();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = match key {
            NodeKey::ArrowRight | NodeKey::ArrowDown => {
                Some(if index == last { 0 } else { index + 1 })
            }
            NodeKey::ArrowLeft | NodeKey::ArrowUp => {
                Some(if index == 0 { last } else { index - 1 })
            }
            NodeKey::Home => Some(0),
            NodeKey::End => Some(last),
            _ => None,
        }?;
        let target = ids[next].clone();
        if target != current {
            if let Some(handler) = &on_change {
                handler(&target);
            }
        }
        Some(option_focus_id(&instance_scope, &target))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeModifiers;
    use poodle_specs::ChoiceOption;
    use std::sync::{Arc, Mutex};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn plan_options() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise"),
        ]
    }

    fn option<'a>(node: &'a Node, value: &str) -> &'a Node {
        let id = format!("radio:plan:option:{value}");
        node.find(&|n| n.id.as_deref() == Some(id.as_str()))
            .unwrap_or_else(|| panic!("option {value} exists"))
    }

    #[test]
    fn same_value_activation_is_inert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(plan_options())
            .with_name("plan")
            .with_value("pro");
        let node = radio_group(
            &spec,
            &ctx,
            Some(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        let pro = option(&node, "pro");
        assert!(pro.interaction.on_activate.is_none());
        assert_eq!(pro.a11y.selected, Some(true));
        assert_eq!(pro.a11y.tab_index, Some(0));
        (option(&node, "enterprise")
            .interaction
            .on_activate
            .as_ref()
            .unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise"]);
    }

    #[test]
    fn arrows_wrap_and_skip_a_disabled_option() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro").with_disabled(true),
            ChoiceOption::new("enterprise", "Enterprise"),
        ])
        .with_name("plan")
        .with_value("free");
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = radio_group(
            &spec,
            &ctx,
            Some(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        let keys = option(&node, "free")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let modifiers = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowRight, modifiers),
            Some(option_focus_id("plan", "enterprise"))
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(option(&node, "pro").interaction.on_key.is_none());
        assert!(!option(&node, "pro").interaction.focusable);
    }

    #[test]
    fn disabled_group_emits_nothing() {
        let spec = RadioGroupSpec {
            is_disabled: true,
            ..RadioGroupSpec::new(plan_options())
                .with_name("plan")
                .with_value("free")
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = radio_group(
            &spec,
            &ctx,
            Some(Arc::new(|_: &str| panic!("disabled group must not emit"))),
        );
        for value in ["free", "pro", "enterprise"] {
            let row = option(&node, value);
            assert!(row.interaction.on_activate.is_none(), "{value}");
            assert!(row.interaction.disabled, "{value}");
            assert!(!row.interaction.focusable, "{value}");
        }
    }
}
