//! CollapseToggle — the chevron that collapses a region.
//!
//! Contract: `docs/contracts/components/collapse-toggle.md`
//! Ported from: `packages/jetstream/components/src/collapse_toggle.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, MainAxisAlignment, Node, NodeRole,
    StylePatch,
};
use poodle_specs::CollapseToggleSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn collapse_toggle(
    spec: &CollapseToggleSpec,
    ctx: &RenderContext<'_>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let theme = ctx.theme();
    // The spec helpers apply `size_role` internally, so they take the base
    // size — never a role-resolved value.
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let icon_size = theme.resolve_space(spec.icon_size_token(base_size));
    let radius = theme.resolve_radius(spec.radius_token());
    let text_color = theme.resolve_color(spec.text_color_token());

    let pad_y = rem_to_px(spec.padding_rem(base_size));
    let pad_x = rem_to_px(spec.padding_inline_rem(density));

    let mut el = Node::button("");
    el.a11y.role = Some(NodeRole::Button);
    el.a11y.label = Some(spec.effective_aria_label().to_string());
    el.a11y.expanded = Some(!spec.is_collapsed);
    {
        let s = &mut el.style;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.line_height = Some(1.0);
    }

    let mut chevron = Node::icon(spec.effective_icon_name(), icon_size);
    chevron.style.descriptor.text_color = Some(text_color);
    el = el.child(chevron);

    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.style.descriptor.cursor = CursorHint::Default;
        el.interaction.disabled = true;
        el.interaction.focusable = false;
        el.a11y.tab_index = None;
        el.interaction.on_activate = None;
    } else {
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.interaction.focusable = true;
        el.a11y.tab_index = Some(0);
        el.style.hover = Some(StylePatch {
            background: Some(theme.resolve_color(spec.hover_fill_token())),
            border_color: None,
            text_color: Some(theme.resolve_color(spec.text_color_hover_token())),
            opacity: None,
        });
        el.style.focus_ring = Some(FocusRing {
            color: theme.resolve_color(spec.focus_ring_color_token()),
            width: theme.resolve_border_width(spec.focus_ring_width_token()),
            offset: rem_to_px(0.0625),
        });
        if let Some(handler) = on_toggle {
            let next = !spec.is_collapsed;
            el.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_node::NodeKind;
    use poodle_specs::CollapseDirection;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn icon_name(node: &Node) -> &str {
        node.find(&|child| matches!(&child.kind, NodeKind::Icon { .. }))
            .and_then(|child| match &child.kind {
                NodeKind::Icon { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .expect("chevron")
    }

    fn render(spec: &CollapseToggleSpec) -> Node {
        collapse_toggle(spec, &RenderContext::new(&theme()), None)
    }

    #[test]
    fn expanded_default_projects_collapse_label_and_expanded_true() {
        let node = render(&CollapseToggleSpec::new());
        assert_eq!(node.a11y.role, Some(NodeRole::Button));
        assert_eq!(node.a11y.label.as_deref(), Some("Collapse"));
        assert_eq!(node.a11y.expanded, Some(true));
        assert_eq!(icon_name(&node), "chevron-left");
    }

    #[test]
    fn collapsed_default_projects_expand_label_and_expanded_false() {
        let node = render(&CollapseToggleSpec::new().with_collapsed(true));
        assert_eq!(node.a11y.label.as_deref(), Some("Expand"));
        assert_eq!(node.a11y.expanded, Some(false));
        assert_eq!(icon_name(&node), "chevron-right");
    }

    #[test]
    fn explicit_label_overrides_both_collapsed_states() {
        let expanded = render(&CollapseToggleSpec::new().with_aria_label("Collapse left dock"));
        let collapsed = render(
            &CollapseToggleSpec::new()
                .with_collapsed(true)
                .with_aria_label("Collapse left dock"),
        );
        assert_eq!(expanded.a11y.label.as_deref(), Some("Collapse left dock"));
        assert_eq!(expanded.a11y.expanded, Some(true));
        assert_eq!(collapsed.a11y.label.as_deref(), Some("Collapse left dock"));
        assert_eq!(collapsed.a11y.expanded, Some(false));
    }

    #[test]
    fn every_direction_paints_the_authored_chevron_and_its_opposite() {
        let pairs = [
            (CollapseDirection::Left, "chevron-left", "chevron-right"),
            (CollapseDirection::Right, "chevron-right", "chevron-left"),
            (CollapseDirection::Up, "chevron-up", "chevron-down"),
            (CollapseDirection::Down, "chevron-down", "chevron-up"),
        ];
        for (direction, expanded, collapsed) in pairs {
            let open = render(&CollapseToggleSpec::new().with_direction(direction));
            let shut = render(
                &CollapseToggleSpec::new()
                    .with_direction(direction)
                    .with_collapsed(true),
            );
            assert_eq!(icon_name(&open), expanded, "{direction:?} expanded");
            assert_eq!(icon_name(&shut), collapsed, "{direction:?} collapsed");
        }
    }

    #[test]
    fn enabled_control_is_a_button_tab_stop_with_the_contracted_ring() {
        let theme = theme();
        let spec = CollapseToggleSpec::new();
        let node = render(&spec);
        assert!(node.interaction.focusable);
        assert!(!node.interaction.disabled);
        assert_eq!(node.a11y.tab_index, Some(0));
        assert_eq!(node.style.descriptor.cursor, CursorHint::Pointer);
        let ring = node
            .style
            .focus_ring
            .expect("enabled collapse toggle declares a ring");
        assert_eq!(
            ring.color,
            theme.resolve_color(spec.focus_ring_color_token())
        );
        assert_eq!(
            ring.width,
            theme.resolve_border_width(spec.focus_ring_width_token())
        );
        assert_eq!(ring.offset, rem_to_px(0.0625));
    }

    #[test]
    fn activation_reports_the_next_collapsed_value_without_owning_state() {
        let reported = Arc::new(std::sync::Mutex::new(Vec::<bool>::new()));
        let sink = Arc::clone(&reported);
        let node = collapse_toggle(
            &CollapseToggleSpec::new(),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next| {
                sink.lock().expect("report lock").push(next);
            })),
        );
        let activate = node
            .interaction
            .on_activate
            .as_ref()
            .expect("enabled activation handler");
        activate.as_ref()();
        activate.as_ref()();
        assert_eq!(*reported.lock().expect("report lock"), [true, true]);
    }

    #[test]
    fn disabled_suppresses_activation_focus_and_the_ring() {
        let theme = theme();
        let spec = CollapseToggleSpec::new().with_disabled(true);
        let node = collapse_toggle(
            &spec,
            &RenderContext::new(&theme),
            Some(Arc::new(|_| {
                panic!("disabled collapse toggle does not fire")
            })),
        );
        assert!(node.interaction.disabled);
        assert!(!node.interaction.focusable);
        assert_eq!(node.a11y.tab_index, None);
        assert!(node.interaction.on_activate.is_none());
        assert!(node.style.focus_ring.is_none());
        assert_eq!(node.style.descriptor.cursor, CursorHint::Default);
        assert!(
            node.style.descriptor.opacity > 0.0 && node.style.descriptor.opacity < 1.0,
            "disabled opacity stays contract-owned"
        );
        assert_eq!(node.a11y.label.as_deref(), Some("Collapse"));
        assert_eq!(node.a11y.expanded, Some(true));
    }
}
