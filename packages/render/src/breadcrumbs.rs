//! Breadcrumbs — the trail back up: crumbs separated by dim chevrons.
//!
//! Contract: `docs/contracts/components/breadcrumbs.md`
//! Ported from: `packages/jetstream/components/src/breadcrumbs_comp.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node};
use poodle_specs::{BreadcrumbsSpec, IconSize, IconSpec};

use crate::icon::icon;
use crate::presentation::{
    breadcrumbs_density_gap_rem, breadcrumbs_font_rem, breadcrumbs_gap_rem, rem_to_px,
    resolve_semantic_size,
};

pub fn breadcrumbs(
    spec: &BreadcrumbsSpec,
    theme: &dyn ThemeProvider,
    on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(breadcrumbs_font_rem(effective_size));
    let gap_rem = breadcrumbs_density_gap_rem(spec.density)
        .unwrap_or_else(|| breadcrumbs_gap_rem(effective_size));
    let gap = rem_to_px(gap_rem);

    let text_color = theme.resolve_color("color.text.secondary");
    let current_color = theme.resolve_color("color.text.primary");
    let sep_color = theme.resolve_color("color.text.secondary");

    let sep_icon_spec = IconSpec::new("chevron-right").with_size(IconSize::Sm);

    let visible = spec.visible_items();
    let visible_len = visible.len();

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    for (i, item) in visible.iter().enumerate() {
        if i > 0 {
            // Separator chevron at contract opacity 0.4.
            let mut sep = Node::container();
            {
                let s = &mut sep.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.opacity = 0.4;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.text_color = Some(sep_color);
            }
            el = el.child(sep.child(icon(&sep_icon_spec, theme)));
        }

        let is_current = spec.is_current_at(item, i, visible_len);
        let color = if is_current { current_color } else { text_color };

        let mut crumb = Node::text(&item.label);
        crumb.style.descriptor.text_color = Some(color);
        crumb.style.text_size = Some(font_size);

        if let (false, Some(href), Some(handler)) = (is_current, item.href.as_ref(), &on_navigate)
        {
            let handler = Arc::clone(handler);
            let href = href.clone();
            crumb.style.descriptor.cursor = CursorHint::Pointer;
            crumb.interaction.on_activate = Some(Arc::new(move || handler(&href)));
        }

        el = el.child(crumb);
    }

    el.a11y.label = Some(spec.aria_label.clone());
    el
}
