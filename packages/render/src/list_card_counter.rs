//! ListCardCounter — icon + count for list card footers.
//!
//! Contract: `docs/contracts/components/list-card-counter.md`
//! Ported from: `packages/jetstream/components/src/list_card_counter.rs`.
//!
//! `href`: no anchor widget — linked styling (pointer cursor, hover colour)
//! matches the contract's linked state; navigation is a shell concern.
//! `on_link_click` fires only when linked (`is_linked` is the condition that
//! already draws the pointer cursor).

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, StylePatch,
};
use poodle_specs::{IconSpec, ListCardCounterSpec};

use crate::icon::icon;
use crate::presentation::rem_to_px;

pub fn list_card_counter(
    spec: &ListCardCounterSpec,
    theme: &dyn ThemeProvider,
    on_link_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let gap = rem_to_px(spec.gap_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let secondary = theme.resolve_color(ListCardCounterSpec::text_secondary_token());
    let primary = theme.resolve_color(ListCardCounterSpec::text_primary_token());

    let mut icon_el = icon(
        &IconSpec::new(spec.icon.clone()).with_size(ListCardCounterSpec::icon_size()),
        theme,
    );
    let dim = rem_to_px(spec.icon_size_rem());
    icon_el.style.descriptor.layout.width = LayoutSizing::Fixed(dim);
    icon_el.style.descriptor.layout.height = LayoutSizing::Fixed(dim);
    icon_el.style.descriptor.text_color = Some(secondary);

    let count = Node::text(format!("{}", spec.count));

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.text_size = Some(font_size);
        s.descriptor.text_color = Some(secondary);
    }
    let mut row = row.child(icon_el).child(count);

    if spec.is_linked() {
        row.id = Some(format!(
            "poodle-lcc-{}-{}",
            spec.icon.replace(['/', '\\', ' '], "-"),
            spec.count
        ));
        row.style.descriptor.cursor = CursorHint::Pointer;
        row.style.hover = Some(StylePatch {
            background: None,
            border_color: None,
            text_color: Some(primary),
            opacity: None,
        });
        if let Some(handler) = on_link_click {
            row.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_and_count_share_the_secondary_tone() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let spec = ListCardCounterSpec::new("file-text", 24);
        let node = list_card_counter(&spec, &theme, None);
        let secondary = theme.resolve_color(ListCardCounterSpec::text_secondary_token());

        assert_eq!(node.style.descriptor.text_color, Some(secondary));
        assert_eq!(
            node.children[0].style.descriptor.text_color,
            Some(secondary)
        );
    }
}
