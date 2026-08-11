//! Accordion — a stack of disclosure items.
//!
//! Contract: `docs/contracts/components/accordion.md`
//! Ported from: `packages/jetstream/components/src/accordion.rs`. The chevron
//! swaps its icon name with the expanded state (no rotation channel), matching
//! both old tiers.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, ShadowLayer,
};
use poodle_specs::{AccordionItemSpec, AccordionSpec, ControlDensity, ControlSize};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

pub fn accordion(
    spec: &AccordionSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    accordion_with_content(spec, theme, &[], on_change)
}

/// Render with per-item content keyed by accordion item value.
pub fn accordion_with_content(
    spec: &AccordionSpec,
    theme: &dyn ThemeProvider,
    content: &[(String, Node)],
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let root_gap = theme.resolve_space(spec.root_gap_token());
    let expanded = spec.expanded_values();

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
        s.self_stretch = true;
    }

    for item in &spec.items {
        let is_expanded = expanded.iter().any(|v| *v == item.value);
        root = root.child(render_item(
            spec,
            item,
            is_expanded,
            effective_size,
            spec.density,
            theme,
            content
                .iter()
                .find(|(value, _)| value == &item.value)
                .map(|(_, node)| node),
            on_change.as_ref(),
        ));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::Group);
    root
}

#[expect(
    clippy::too_many_arguments,
    reason = "item rendering keeps shared visual metrics explicit"
)]
fn render_item(
    spec: &AccordionSpec,
    item: &AccordionItemSpec,
    is_expanded: bool,
    effective_size: ControlSize,
    density: ControlDensity,
    theme: &dyn ThemeProvider,
    content: Option<&Node>,
    on_change: Option<&Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let border_subtle = theme.resolve_color("color.border.subtle");
    let elevated = theme.resolve_color(spec.item_bg_elevated_token());
    let panel = theme.resolve_color(spec.item_bg_panel_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let radius = theme.resolve_radius("radius.surface");

    let item_border = with_alpha(border_subtle, border_subtle.3 * spec.border_subtle_alpha());
    let item_bg = mix_srgb(elevated, panel, spec.item_bg_elevated_ratio());

    let pad_x = rem_to_px(spec.inline_padding_rem(density));
    let pad_y = rem_to_px(spec.block_padding_rem());
    let item_gap = rem_to_px(spec.item_internal_gap_rem());
    let summary_gap = theme.resolve_space(spec.summary_gap_token());
    let trigger_gap = theme.resolve_space(spec.trigger_grid_gap_token());

    let title_font_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    let description_font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(0.75);

    // Summary: title over optional description, always visible.
    let mut summary = Node::container();
    {
        let s = &mut summary.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = summary_gap;
        s.min_width = Some(0.0);
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
    }
    let mut title = Node::text(&item.label);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_font_size);
    title.style.text_weight = Some(700);
    title.style.line_height = Some(1.2);
    summary = summary.child(title);
    if let Some(ref desc) = item.description {
        let mut d = Node::text(desc);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(description_font_size);
        d.style.line_height = Some(1.45);
        summary = summary.child(d);
    }

    // Indicator swaps chevron name with the state.
    let chevron_icon = if is_expanded {
        "chevron-up"
    } else {
        "chevron-down"
    };
    let mut indicator = Node::icon(chevron_icon, indicator_size);
    indicator.style.flex_shrink_zero = true;
    indicator.style.descriptor.text_color = Some(text_secondary);

    // Trigger: the whole header row.
    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.self_stretch = true;
        s.descriptor.layout.spacing.gap = trigger_gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    trigger.interaction.focusable = true;
    trigger = trigger.child(summary).child(indicator);

    if let (false, Some(handler)) = (item.is_disabled, on_change) {
        let handler = Arc::clone(handler);
        let value = item.value.clone();
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.interaction.on_activate = Some(Arc::new(move || handler(&value)));
    }

    // Item shell: inset top highlight in inverse-text at 8%.
    let text_inverse = theme.resolve_color("color.text.inverse");
    let highlight = ColorValue(text_inverse.0, text_inverse.1, text_inverse.2, 0.08);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.self_stretch = true;
        s.descriptor.layout.spacing.gap = item_gap;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(item_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = item_border;
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: highlight,
            inset: true,
        }];
    }
    el = el.child(trigger);

    // Panel: role=region, conditional.
    if is_expanded {
        let mut panel = Node::container();
        {
            let s = &mut panel.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(0.0);
            s.self_stretch = true;
        }
        panel.a11y.role = Some(NodeRole::Region);
        if let Some(content) = content {
            panel = panel.child(content.clone());
        }
        el = el.child(panel);
    }

    if item.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        el.interaction.disabled = true;
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::AccordionSelectionValue;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn keyed_content_and_toggle_handlers_reach_the_expanded_item() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let toggle_seen = Arc::clone(&seen);
        let spec = AccordionSpec::new(vec![
            AccordionItemSpec::new("first", "First"),
            AccordionItemSpec::new("second", "Second"),
        ])
        .with_value(AccordionSelectionValue::Single("first".to_string()));
        let content = vec![
            ("first".to_string(), Node::text("Expanded body")),
            ("second".to_string(), Node::text("Hidden body")),
        ];
        let node = accordion_with_content(
            &spec,
            &theme(),
            &content,
            Some(Arc::new(move |value| {
                toggle_seen.lock().unwrap().push(value.to_string());
            })),
        );

        assert!(node.has_text("Expanded body"));
        assert!(!node.has_text("Hidden body"));
        let second_trigger = node
            .find(&|node| node.interaction.on_activate.is_some() && node.has_text("Second"))
            .expect("second accordion trigger");
        (second_trigger.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["second"]);
    }
}
