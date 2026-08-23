//! Breadcrumbs — the trail back up: crumbs separated by dim chevrons.
//!
//! Contract: `docs/contracts/components/breadcrumbs.md`
//! Ported from: `packages/jetstream/components/src/breadcrumbs_comp.rs`.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node};
use poodle_specs::{BreadcrumbItem, BreadcrumbsSpec, IconSize, IconSpec};

use crate::context::RenderContext;
use crate::icon::icon;
use crate::presentation::{
    breadcrumbs_density_gap_rem, breadcrumbs_font_rem, breadcrumbs_gap_rem, rem_to_px,
};

pub fn breadcrumbs(
    spec: &BreadcrumbsSpec,
    ctx: &RenderContext<'_>,
    on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let theme = ctx.theme();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(breadcrumbs_font_rem(effective_size));
    let gap_rem =
        breadcrumbs_density_gap_rem(density).unwrap_or_else(|| breadcrumbs_gap_rem(effective_size));
    let gap = rem_to_px(gap_rem);

    let text_color = theme.resolve_color("color.text.secondary");
    let current_color = theme.resolve_color("color.text.primary");
    let sep_color = theme.resolve_color("color.text.secondary");

    let sep_icon_spec = IconSpec::new("chevron-right").with_size(IconSize::Sm);
    // Item icons take the Breadcrumbs resolved size directly. `IconSpec`'s
    // default `Control` role is the identity mapping, so no second semantic
    // shift lands on top of it.
    let item_icon_size = IconSize::from(effective_size);
    // `resolve_space` already yields pixels.
    let icon_gap = theme.resolve_space(spec.icon_gap_token());

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
            let mut sep_icon = icon(&sep_icon_spec, ctx);
            // The native primitive overrides the icon tint to the separator
            // tier; the node backend does not inherit text color from a
            // parent container, so carry that override on the icon itself.
            sep_icon.style.descriptor.text_color = Some(sep_color);
            el = el.child(sep.child(sep_icon));
        }

        let is_current = spec.is_current_at(item, i, visible_len);
        let color = if is_current {
            current_color
        } else {
            text_color
        };

        let mut crumb = crumb_node(item, color, font_size, item_icon_size, icon_gap, ctx);

        if let (false, Some(href), Some(handler)) = (is_current, item.href.as_ref(), &on_navigate) {
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

/// One crumb's content. A text-only item stays a bare text node — the shape the
/// component has always emitted. An icon-bearing item becomes a single row that
/// carries the crumb's accessible name and, later, its activation handler, so
/// glyph and label remain one navigation target.
fn crumb_node(
    item: &BreadcrumbItem,
    color: poodle_tokens::typed::ColorValue,
    font_size: f32,
    icon_size: IconSize,
    icon_gap: f32,
    ctx: &RenderContext<'_>,
) -> Node {
    let Some(icon_name) = item.icon.as_deref() else {
        return crumb_label(&item.label, color, font_size);
    };

    let mut icon_node = icon(&IconSpec::new(icon_name).with_size(icon_size), ctx);
    // Item icons are decorative and inherit the crumb's tier; the node backend
    // does not inherit text colour from a parent container.
    icon_node.style.descriptor.text_color = Some(color);

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = icon_gap;
        s.descriptor.text_color = Some(color);
    }
    row = row.child(icon_node);

    // `shows_label` is false only for a well-formed icon-only crumb. A
    // malformed item — `icon_only` with no icon — never reaches here, so it
    // renders its label instead of a blank crumb.
    if item.shows_label() {
        row = row.child(crumb_label(&item.label, color, font_size));
    }

    // The label is either visible or hidden; either way it is the crumb's
    // accessible name, and the icon beside it is not announced separately.
    row.a11y.label = Some(item.label.clone());
    row
}

fn crumb_label(label: &str, color: poodle_tokens::typed::ColorValue, font_size: f32) -> Node {
    let mut node = Node::text(label);
    node.style.descriptor.text_color = Some(color);
    node.style.text_size = Some(font_size);
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider as _;
    use poodle_node::NodeKind;
    use poodle_specs::{BreadcrumbItem, ControlSize};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn crumbs(items: Vec<BreadcrumbItem>) -> Node {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        breadcrumbs(&BreadcrumbsSpec::new(items), &ctx, None)
    }

    fn text_of(node: &Node) -> Option<&str> {
        match &node.kind {
            NodeKind::Text { content } => Some(content.as_str()),
            _ => None,
        }
    }

    fn icon_of(node: &Node) -> Option<(&str, f32)> {
        match &node.kind {
            NodeKind::Icon { name, size } => Some((name.as_str(), *size)),
            _ => None,
        }
    }

    #[test]
    fn a_text_only_crumb_stays_a_bare_text_node() {
        let tree = crumbs(vec![BreadcrumbItem::new("home", "Home")]);
        let crumb = &tree.children[0];
        assert_eq!(text_of(crumb), Some("Home"));
        assert!(crumb.children.is_empty());
    }

    #[test]
    fn an_icon_crumb_keeps_glyph_and_label_in_one_target() {
        let tree = crumbs(vec![
            BreadcrumbItem::new("projects", "Projects").with_icon("folder")
        ]);
        let crumb = &tree.children[0];

        assert_eq!(crumb.children.len(), 2);
        assert_eq!(icon_of(&crumb.children[0]).map(|(name, _)| name), Some("folder"));
        assert_eq!(text_of(&crumb.children[1]), Some("Projects"));
        assert_eq!(crumb.a11y.label.as_deref(), Some("Projects"));
        // The icon child carries no name of its own: it is decorative.
        assert!(crumb.children[0].a11y.label.is_none());
    }

    #[test]
    fn an_icon_only_crumb_drops_the_text_but_keeps_the_name() {
        let tree = crumbs(vec![BreadcrumbItem::new("home", "Home").with_icon_only("home")]);
        let crumb = &tree.children[0];

        assert_eq!(crumb.children.len(), 1);
        assert_eq!(icon_of(&crumb.children[0]).map(|(name, _)| name), Some("home"));
        assert_eq!(crumb.a11y.label.as_deref(), Some("Home"));
    }

    #[test]
    fn a_malformed_icon_only_item_renders_its_label() {
        let mut item = BreadcrumbItem::new("home", "Home");
        item.icon_only = true;
        let tree = crumbs(vec![item]);
        let crumb = &tree.children[0];

        // No icon means the text-only path; never a blank crumb.
        assert_eq!(text_of(crumb), Some("Home"));
    }

    #[test]
    fn item_icons_take_the_resolved_breadcrumbs_size_without_a_role_shift() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home").with_icon("home")
        ])
        .with_size(ControlSize::Lg);
        let tree = breadcrumbs(&spec, &ctx, None);

        // Explicit lg is the final Breadcrumbs size; the icon follows it and
        // does not apply the chrome role a second time.
        let expected = theme.resolve_space(IconSize::Lg.size_token());
        assert_eq!(icon_of(&tree.children[0].children[0]).map(|(_, size)| size), Some(expected));
    }

    #[test]
    fn the_icon_label_gap_is_the_tighter_inline_step() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("projects", "Projects").with_icon("folder")
        ]);
        let tree = breadcrumbs(&spec, &ctx, None);

        let expected = theme.resolve_space(spec.icon_gap_token());
        assert_eq!(tree.children[0].style.descriptor.layout.spacing.gap, expected);
        assert!(expected < theme.resolve_space(spec.gap_token()));
    }

    #[test]
    fn the_truncation_ellipsis_stays_a_plain_crumb() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home").with_icon_only("home"),
            BreadcrumbItem::new("workspace", "Workspace").with_icon("folder"),
            BreadcrumbItem::new("projects", "Projects").with_icon("folder"),
            BreadcrumbItem::new("poodle", "Poodle").with_icon("package"),
        ])
        .with_max_visible_items(3);
        let tree = breadcrumbs(&spec, &ctx, None);

        // children: crumb, sep, crumb, sep, crumb, sep, crumb
        let ellipsis = &tree.children[2];
        assert_eq!(text_of(ellipsis), Some("\u{2026}"));
        assert!(ellipsis.children.is_empty());
        assert!(ellipsis.a11y.label.is_none());
    }

    #[test]
    fn an_icon_crumb_carries_its_own_activation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home")
                .with_icon_only("home")
                .with_href("/"),
            BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
        ]);
        let tree = breadcrumbs(&spec, &ctx, Some(Arc::new(|_: &str| {})));

        let crumb = &tree.children[0];
        assert!(crumb.interaction.on_activate.is_some());
        assert_eq!(crumb.style.descriptor.cursor, CursorHint::Pointer);
    }
}
