//! StatusIndicator — colored dot with optional label.
//!
//! Contract: `docs/contracts/components/status-indicator.md`
//! Ported from: `packages/jetstream/components/src/status_indicator.rs`.
//!
//! Contract dimensions: dot 0.5625rem square at md (size-scaled), pill
//! radius; gap 0.4375rem; label 0.75rem weight 600. The dot's box-shadow
//! glow and label line-height remain documented runtime deltas, as in the
//! reference tier.

use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node, NodeKind, ShadowLayer};
use poodle_specs::{IconSpec, StatusIndicatorSpec, TextSpec, TextWeight};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::{icon, text};

pub fn status_indicator(spec: &StatusIndicatorSpec, ctx: &RenderContext<'_>) -> Node {
    let status_color = ctx.theme().resolve_color(spec.status_color_token());
    let text_primary = ctx.theme().resolve_color(spec.label_color_token());

    // Contract §8: dot/gap/label metrics resolve from the effective size
    // (size override → size_role against the inherited scale) and density.
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let effective_density = ctx.resolve_density(spec.density);

    let dot_size = rem_to_px(spec.dot_size_rem_for(effective_size));
    let gap = rem_to_px(spec.gap_rem_for(effective_size, effective_density));
    let label_size = rem_to_px(spec.label_font_size_rem_for(effective_size));

    let mut dot = icon(&IconSpec::new("dot"), ctx);
    if let NodeKind::Icon { size, .. } = &mut dot.kind {
        *size = dot_size;
    }
    {
        let s = &mut dot.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(dot_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(dot_size);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = 999.0;
        c.top_right = 999.0;
        c.bottom_right = 999.0;
        c.bottom_left = 999.0;
        s.descriptor.background = Some(status_color);
        s.descriptor.text_color = Some(status_color);
        s.shadow_layers.push(ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.125),
            color: poodle_node::ColorValue(
                status_color.0,
                status_color.1,
                status_color.2,
                status_color.3 * 0.18,
            ),
            inset: false,
        });
    }

    // Root: inline-flex, gap.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    root.roles.insert(
        "status".to_owned(),
        format!("{:?}", spec.status).to_ascii_lowercase(),
    );
    root.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{effective_density:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "typography".to_owned(),
        format!("{:?}", spec.typography).to_ascii_lowercase(),
    );
    let mut root = root.child(dot);

    // Contract: optional label.
    if let Some(ref label_text) = spec.label {
        let mut label = text(&TextSpec::new(label_text).with_weight(TextWeight::Semibold), ctx);
        label.style.descriptor.text_color = Some(text_primary);
        label.style.text_size = Some(label_size);
        label.style.text_weight = Some(600);
        label.style.line_height = Some(1.3);
        root = root.child(label);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::StatusTone;

    #[test]
    fn composes_icon_and_text_with_exact_status_tokens() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let spec = StatusIndicatorSpec::new()
            .with_status(StatusTone::Success)
            .with_label("Ready");
        let node = status_indicator(&spec, &ctx);
        assert!(matches!(
            &node.children[0].kind,
            NodeKind::Icon { name, size } if name == "dot" && *size == 9.0
        ));
        assert!(matches!(
            &node.children[1].kind,
            NodeKind::Text { content } if content == "Ready"
        ));
        assert_eq!(
            node.roles.get("status").map(String::as_str),
            Some("success")
        );
        assert_eq!(node.roles.get("size").map(String::as_str), Some("md"));
        assert_eq!(
            node.roles.get("density").map(String::as_str),
            Some("default")
        );
        assert_eq!(node.children[0].style.shadow_layers.len(), 1);
        assert_eq!(
            node.children[0].style.shadow_layers[0].spread,
            rem_to_px(0.125)
        );
        assert_eq!(node.children[1].style.line_height, Some(1.3));
    }
}
