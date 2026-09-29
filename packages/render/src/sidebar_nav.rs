//! SidebarNav — a grouped navigation rail.
//!
//! Contract: `docs/contracts/components/sidebar-nav.md`
//! Ported from: `packages/jetstream/components/src/sidebar_nav.rs`.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeKey, NodePoint,
    NodePosition, NodeRole, StylePatch,
};
use poodle_specs::SidebarNavSpec;

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

// ── Active-state alpha factors (contract color-mix percentages) ──
const ACTIVE_BG_ALPHA: f32 = 0.10; // accent-base @ 10%
const ACTIVE_RING_ALPHA: f32 = 0.20; // inset ring accent-base @ 20%
const HOVER_BG_ALPHA: f32 = 0.60; // elevated @ 60%
const SEPARATOR_ALPHA: f32 = 0.54; // border-subtle @ 54%

/// Where a per-item context-menu request came from.
///
/// A pointer request carries the secondary-click window point — the anchor,
/// exactly as [`poodle_node::Interaction::on_context`] reports it. A keyboard
/// request carries no point: the gesture landed on the focused item, and the
/// item's own geometry is the anchor (the web's `rect + 16px` rule), which
/// only the host can resolve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SidebarNavContextMenuOrigin {
    Pointer(NodePoint),
    Keyboard,
}

/// Host callbacks: item activation and per-item context-menu requests.
#[derive(Default)]
pub struct SidebarNavHandlers {
    /// Fires with the value of the item that was chosen.
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// A context menu was requested for the item with this value. Only items
    /// carrying rows raise it; disabled items never do. Opening the shared
    /// ContextMenu overlay, its anchor, its action routing (`(itemValue,
    /// actionValue)`), and focus on close are host-owned, matching the Tree
    /// host pattern.
    pub on_context_menu: Option<Arc<dyn Fn(&str, SidebarNavContextMenuOrigin) + Send + Sync>>,
}

/// Escapes a host value for use inside a generated element id.
///
/// `%` becomes `%25`, `~` becomes `%7E`, and whitespace becomes its `%XX`
/// form; every other character passes through unchanged. The escape is
/// injective, and its output never contains a raw `~` or whitespace — which
/// is what keeps the item and end-label id namespaces disjoint for arbitrary
/// values (see [`sidebar_nav_item_end_label_id`]) and keeps `described_by`
/// (a space-separated id list) well-formed.
fn escape_id_fragment(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '%' => escaped.push_str("%25"),
            '~' => escaped.push_str("%7E"),
            ' ' => escaped.push_str("%20"),
            '\t' => escaped.push_str("%09"),
            '\n' => escaped.push_str("%0A"),
            '\r' => escaped.push_str("%0D"),
            other => escaped.push(other),
        }
    }
    escaped
}

/// The element id of one sidebar item: stable across frames (the backend's
/// click pipeline needs it to survive from mouse-down to mouse-up) and the
/// focus-return destination after the item's context menu closes. The value
/// is escaped (see [`escape_id_fragment`]), so the id never contains a raw
/// `~` or whitespace and stays distinct from every end-label id.
pub fn sidebar_nav_item_id(value: &str) -> String {
    format!("sidebar-nav-{}", escape_id_fragment(value))
}

/// The element id of one sidebar item's end-label text.
///
/// Collision-safe for arbitrary item values: the value is escaped, so it can
/// never supply the separating `~`, and item ids (which never contain a raw
/// `~`) cannot mirror this form. `foo` and a second item valued
/// `foo-end-label` therefore stay three distinct ids — there is no suffix an
/// end-label id can share with an item id.
pub fn sidebar_nav_item_end_label_id(value: &str) -> String {
    format!("sidebar-nav-{}~end-label", escape_id_fragment(value))
}

/// `on_change` fires with the value of the item that was chosen.
pub fn sidebar_nav(
    spec: &SidebarNavSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    sidebar_nav_with_handlers(
        spec,
        ctx,
        SidebarNavHandlers {
            on_change,
            ..SidebarNavHandlers::default()
        },
    )
}

/// Full handler surface: activation plus per-item context-menu requests.
pub fn sidebar_nav_with_handlers(
    spec: &SidebarNavSpec,
    ctx: &RenderContext<'_>,
    handlers: SidebarNavHandlers,
) -> Node {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    // The sidebar's size tables key off the raw (base) size, not the
    // chrome-role-resolved size — matching Svelte's `[data-size]` CSS.
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let item_height = rem_to_px(spec.item_height_rem(base_size));
    let item_font = rem_to_px(spec.item_font_rem(base_size));
    let title_font = rem_to_px(spec.title_font_rem(base_size));
    let end_label_font = rem_to_px(spec.end_label_font_rem(base_size));

    let group_gap = rem_to_px(spec.group_gap_rem(density));
    let item_px = rem_to_px(spec.item_pad_inline_rem(density));
    let end_label_gap = rem_to_px(spec.end_label_gap_rem(density));
    let title_gap = rem_to_px(spec.title_gap_rem(density));
    let group_internal_gap = rem_to_px(0.3125); // contract group `gap`
    let list_gap = rem_to_px(0.125); // contract list `gap`
    let separator_mt = rem_to_px(0.125); // contract separator margin-top
    let rail_w = rem_to_px(0.1875); // contract left border 3px
    let nav_pad_x = rem_to_px(0.375); // contract root horizontal padding
                                      // Root vertical padding = space-panel-y (density-driven).
    let panel_y = rem_to_px(match density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 1.0,
    });

    // ── Token resolution ──────────────────────────────────────
    let item_color = ctx.theme().resolve_color(spec.item_color_token());
    let item_active_color = ctx.theme().resolve_color(spec.item_active_color_token());
    let end_label_color = ctx.theme().resolve_color(spec.end_label_color_token());
    let group_title_color = ctx.theme().resolve_color(spec.group_title_color_token());
    let separator_color = ctx.theme().resolve_color(spec.separator_color_token());
    let accent = ctx.theme().resolve_color(spec.active_indicator_color_token());
    let hover_fill = ctx.theme().resolve_color(spec.hover_fill_token());
    let focus_ring = ctx.theme().resolve_color(spec.focus_ring_color_token());
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    let ctrl_radius = ctx.theme().resolve_radius("radius.control");
    let item_radius = (ctrl_radius - rem_to_px(0.125)).max(0.0);

    let active_bg = with_alpha(accent, accent.3 * ACTIVE_BG_ALPHA);
    let active_ring = with_alpha(accent, accent.3 * ACTIVE_RING_ALPHA);
    let hover_bg = with_alpha(hover_fill, hover_fill.3 * HOVER_BG_ALPHA);

    let visible_groups = spec.visible_groups();
    let has_multiple_groups = visible_groups.len() > 1;

    // ── Root: <nav> as a flex column with panel padding ──────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = group_gap;
        // Contract §7: `min-width: 0` down the root → group → list → item
        // chain lets long titles shrink and wrap instead of overflowing.
        s.min_width = Some(0.0);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = panel_y;
        pad.bottom = panel_y;
        pad.left = nav_pad_x;
        pad.right = nav_pad_x;
    }

    for (gi, group) in visible_groups.iter().enumerate() {
        let mut group_el = Node::container();
        {
            let s = &mut group_el.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = group_internal_gap;
            s.min_width = Some(0.0);

            // Inter-group separator: top border + top padding on the group
            // element (matches the Svelte adjacent-sibling rule).
            if has_multiple_groups && gi > 0 {
                s.descriptor.layout.spacing.margin.top = separator_mt;
                s.descriptor.layout.spacing.padding.top = group_gap - separator_mt;
                s.border_top_width = Some(1.0);
                s.border_color_top = Some(with_alpha(
                    separator_color,
                    separator_color.3 * SEPARATOR_ALPHA,
                ));
            }
        }

        // Group title — uppercase, caption-sized, accent-tinted.
        if let Some(ref label_text) = group.label {
            let mut title = Node::text(label_text.to_uppercase());
            {
                let s = &mut title.style;
                s.descriptor.text_color = Some(group_title_color);
                s.text_size = Some(title_font);
                s.text_weight = Some(700);
                s.line_height = Some(1.2); // contract §8 title line-height
                s.letter_spacing_em = Some(0.18); // contract §8 title tracking
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = item_px;
                pad.right = item_px;
                s.descriptor.layout.spacing.margin.bottom = title_gap;
            }
            group_el = group_el.child(title);
        }

        // Item list
        let mut list = Node::container();
        {
            let s = &mut list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = list_gap;
            s.min_width = Some(0.0);
        }

        for item in &group.items {
            let is_active = spec.is_active(&item.value);
            // Contract §2: with an end label the item lays out as a row — a
            // flexible label plus end-aligned metadata — and the accessible
            // name stays exactly `label` while the metadata becomes the item's
            // description. Without one the label is the item's direct text.
            let end_label = item.end_label.as_deref().filter(|text| !text.is_empty());

            // Item box: min-height drives the row height; vertical centring
            // stands in for the contract padding-block on single-line labels.
            // End-label items compose their content from children, so the
            // button itself carries an empty label; the explicit a11y label
            // keeps the accessible name exactly `label` either way.
            let mut item_el = match end_label {
                Some(_) => {
                    let mut b = Node::button("");
                    b.a11y.role = Some(NodeRole::Button);
                    b.a11y.label = Some(item.label.clone());
                    b
                }
                None => {
                    let mut b = Node::button(&item.label);
                    b.a11y.role = Some(NodeRole::Button);
                    b
                }
            };
            // A STABLE id per item (sidebar_nav_item_id). Without one the
            // backend falls back to a per-build counter, so the element's
            // identity changes every frame — and gpui's `on_click` needs it
            // to survive from mouse-down to mouse-up, so every click is
            // dropped. It is also the focus-return destination after the
            // item's context menu closes.
            item_el.id = Some(sidebar_nav_item_id(&item.value));
            {
                let s = &mut item_el.style;
                s.min_height = Some(item_height);
                // Contract §7 resizing rules: `min-width: 0` lets a long title
                // shrink and wrap instead of pushing the row past its rail.
                s.min_width = Some(0.0);
                s.self_stretch = true;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                if end_label.is_some() {
                    // Contract §8 `[data-end-label="true"]`: the row gap
                    // between label and end label.
                    s.descriptor.layout.spacing.gap = end_label_gap;
                }
                s.text_size = Some(item_font);
                s.line_height = Some(1.3); // contract §8 item line-height
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = item_px;
                pad.right = item_px;
                let c = &mut s.descriptor.corner_radii;
                c.top_left = item_radius;
                c.top_right = item_radius;
                c.bottom_right = item_radius;
                c.bottom_left = item_radius;
                // Reserve a 3px transparent left rail on every item so
                // active ↔ inactive does not shift horizontally.
                s.border_left_width = Some(rail_w);
                s.border_color_left = Some(ColorValue(0.0, 0.0, 0.0, 0.0));

                if is_active {
                    // Active: accent left rail + bg fill + bolder weight. The
                    // inset ring is a separate child below, NOT a uniform
                    // border here: gpui has one border colour per element, so
                    // an accent left rail plus a ring-coloured box on the same
                    // node collapses to one colour on all four sides and the
                    // rail disappears into a full accent outline.
                    s.descriptor.text_color = Some(item_active_color);
                    s.text_weight = Some(600);
                    s.descriptor.background = Some(active_bg);
                    s.border_color_left = Some(accent);
                } else {
                    s.descriptor.text_color = Some(item_color);
                    s.text_weight = Some(500);
                }
            }

            if is_active {
                // Inset ring: a full-bleed 1px accent@20% overlay, matching the
                // old tier's emulation of an inset box-shadow.
                let mut ring = Node::container();
                {
                    let s = &mut ring.style;
                    s.descriptor.border.width = 1.0;
                    s.descriptor.border.color = active_ring;
                    let c = &mut s.descriptor.corner_radii;
                    c.top_left = item_radius;
                    c.top_right = item_radius;
                    c.bottom_right = item_radius;
                    c.bottom_left = item_radius;
                }
                ring.position = NodePosition::Absolute {
                    top: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    bottom: Some(0.0),
                };
                item_el.position = NodePosition::Relative;
                item_el = item_el.child(ring);
            }

            if let Some(text) = end_label {
                // The flexible label: grows, may shrink to zero min-width, and
                // wraps long titles (the web's `flex: 1 1 auto; min-width: 0`).
                let mut label = Node::text(item.label.clone());
                {
                    let s = &mut label.style;
                    s.flex_fill = true;
                    s.min_width = Some(0.0);
                    s.text_wrap = true;
                    s.line_height = Some(1.3);
                    s.text_size = Some(item_font);
                    s.descriptor.text_color = Some(if is_active {
                        item_active_color
                    } else {
                        item_color
                    });
                    s.text_weight = Some(if is_active { 600 } else { 500 });
                }
                // The end label: muted tabular-style metadata that never
                // shrinks or wraps, at 0.85× the item font (contract §8). It
                // keeps this muted colour and weight on hover and active
                // items; the item's colour/weight changes apply to the label
                // only. The web marks the span `aria-hidden` and references it
                // with `aria-describedby`; natively the explicit item label
                // owns the name and `described_by` owns the description, so
                // the same observable result falls out.
                let mut end = Node::text(text);
                {
                    let s = &mut end.style;
                    s.flex_shrink_zero = true;
                    s.no_wrap = true;
                    s.text_size = Some(end_label_font);
                    s.text_weight = Some(500);
                    s.descriptor.text_color = Some(end_label_color);
                }
                end.id = Some(sidebar_nav_item_end_label_id(&item.value));
                item_el.a11y.described_by = Some(sidebar_nav_item_end_label_id(&item.value));
                item_el = item_el.child(label).child(end);
            }

            if item.is_disabled {
                // Contract §4: reduced opacity, `cursor: not-allowed`, no
                // activation — and the backend's disabled state keeps the
                // element out of activation and focus entirely.
                item_el.style.descriptor.opacity = disabled_opacity;
                item_el.style.descriptor.cursor = CursorHint::NotAllowed;
                item_el.interaction.disabled = true;
            } else {
                if let Some(handler) = &handlers.on_change {
                    let handler = Arc::clone(handler);
                    let value = item.value.clone();
                    item_el.interaction.on_activate = Some(Arc::new(move || handler(&value)));
                }

                // Per-item context menu (contract §3/§4): only a non-disabled
                // item carrying rows intercepts the gestures. Both the
                // secondary click and the keyboard menu gesture (the
                // ContextMenu key, or Shift+F10 — a bare F10 means nothing)
                // report through one handler, exactly as Svelte funnels both
                // into `openContextMenu`.
                if item.has_context_menu() {
                    if let Some(handler) = &handlers.on_context_menu {
                        let pointer = Arc::clone(handler);
                        let value = item.value.clone();
                        item_el.interaction.on_context = Some(Arc::new(move |point: NodePoint| {
                            pointer(&value, SidebarNavContextMenuOrigin::Pointer(point));
                        }));
                        let keys = Arc::clone(handler);
                        let value = item.value.clone();
                        item_el.interaction.on_key = Some(Arc::new(move |key, mods| {
                            let keyboard_gesture = match key {
                                NodeKey::ContextMenu => true,
                                NodeKey::F10 => mods.shift,
                                _ => false,
                            };
                            if keyboard_gesture {
                                keys(&value, SidebarNavContextMenuOrigin::Keyboard);
                            }
                            None
                        }));
                    }
                }

                item_el.interaction.focusable = true;
                let s = &mut item_el.style;
                s.descriptor.cursor = CursorHint::Pointer;
                // Hover: text-primary + elevated@60% bg (contract §4/§8).
                s.hover = Some(StylePatch {
                    background: Some(hover_bg),
                    border_color: None,
                    text_color: Some(item_active_color),
                    opacity: None,
                });
                // Focus-visible: accent focus ring (contract §6/§8). This was
                // on `active` — gpui's *pressed* state — so the ring flashed on
                // mouse-down and never showed for keyboard focus, which is the
                // state the contract names. `focus` is the right channel.
                s.focus = Some(StylePatch {
                    background: None,
                    border_color: Some(focus_ring),
                    text_color: None,
                    opacity: None,
                });
            }

            list = list.child(item_el);
        }

        group_el = group_el.child(list);
        el = el.child(group_el);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use poodle_specs::{ControlSize, MenuEntry, SidebarNavItem as Item};

    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn find<'a>(node: &'a Node, id: &str) -> Option<&'a Node> {
        if node.id.as_deref() == Some(id) {
            return Some(node);
        }
        node.children.iter().find_map(|child| find(child, id))
    }

    fn spec_with(items: Vec<Item>) -> SidebarNavSpec {
        SidebarNavSpec::new(vec![poodle_specs::SidebarNavGroup::new("g", items)])
    }

    #[test]
    fn end_label_renders_muted_metadata_and_the_description_link() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = spec_with(vec![Item::new("videos", "Videos").with_end_label("198")]);
        let node = sidebar_nav(&spec, &ctx, None);

        let item = find(&node, "sidebar-nav-videos").expect("item");
        assert_eq!(item.a11y.label.as_deref(), Some("Videos"));
        assert_eq!(item.a11y.role, Some(NodeRole::Button));
        assert_eq!(
            item.a11y.described_by.as_deref(),
            Some("sidebar-nav-videos~end-label"),
            "the end label is the item's description, never its name"
        );

        let end = find(&node, "sidebar-nav-videos~end-label").expect("end label");
        assert!(matches!(&end.kind, poodle_node::NodeKind::Text { content } if content == "198"));
        assert_eq!(
            end.style.descriptor.text_color,
            Some(ctx.theme().resolve_color(spec.end_label_color_token())),
            "the end label keeps its muted tertiary colour"
        );
        assert_eq!(
            end.style.text_size,
            Some(rem_to_px(spec.end_label_font_rem(ControlSize::Md))),
            "the end label is 0.85× the item font"
        );
    }

    #[test]
    fn unset_end_label_keeps_direct_text_and_no_description() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = sidebar_nav(&spec_with(vec![Item::new("notes", "Notes")]), &ctx, None);

        let item = find(&node, "sidebar-nav-notes").expect("item");
        assert!(item.a11y.described_by.is_none());
        assert!(find(&node, "sidebar-nav-notes~end-label").is_none());
        assert!(matches!(&item.kind, poodle_node::NodeKind::Button { label } if label == "Notes"));
    }

    #[test]
    fn adversarial_values_keep_item_and_end_label_ids_distinct() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // `foo-end-label` is exactly the suffix the end-label id used to be
        // spelled with; both items carry end labels so every id exists.
        let spec = spec_with(vec![
            Item::new("foo", "Foo").with_end_label("198"),
            Item::new("foo-end-label", "Foo end label").with_end_label("7"),
        ]);
        let node = sidebar_nav(&spec, &ctx, None);

        // Every element id in the tree is unique.
        let mut ids: Vec<&str> = Vec::new();
        fn collect<'a>(node: &'a Node, ids: &mut Vec<&'a str>) {
            if let Some(id) = node.id.as_deref() {
                ids.push(id);
            }
            for child in &node.children {
                collect(child, ids);
            }
        }
        collect(&node, &mut ids);
        let unique = ids.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), ids.len(), "duplicate element ids: {ids:?}");

        // The three ids the adversarial pair produces are exactly the
        // intended ones — the end label of `foo` cannot double as the item
        // `foo-end-label`.
        assert_eq!(sidebar_nav_item_id("foo"), "sidebar-nav-foo");
        assert_eq!(
            sidebar_nav_item_end_label_id("foo"),
            "sidebar-nav-foo~end-label"
        );
        assert_eq!(
            sidebar_nav_item_id("foo-end-label"),
            "sidebar-nav-foo-end-label"
        );
        assert_ne!(
            sidebar_nav_item_id("foo-end-label"),
            sidebar_nav_item_end_label_id("foo")
        );
        assert!(find(&node, "sidebar-nav-foo~end-label").is_some());
        let foo_end_label_item =
            find(&node, "sidebar-nav-foo-end-label").expect("item foo-end-label");
        assert_eq!(
            foo_end_label_item.a11y.described_by.as_deref(),
            Some("sidebar-nav-foo-end-label~end-label")
        );

        // Values with the reserved characters stay distinct too: escaping is
        // injective and never emits a raw `~`, so no crafted value can cross
        // the namespaces.
        assert_eq!(escape_id_fragment("a%b"), "a%25b");
        assert_eq!(escape_id_fragment("a~b"), "a%7Eb");
        assert_eq!(escape_id_fragment("a b"), "a%20b");
        assert_ne!(escape_id_fragment("a%7Eb"), escape_id_fragment("a~b"));
        assert!(!escape_id_fragment("a~b %c").contains('~'));
        assert!(!escape_id_fragment("a~b %c").contains(' '));
    }

    #[test]
    fn context_menu_gestures_reach_only_items_carrying_rows() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let requests: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&requests);
        let spec = spec_with(vec![
            Item::new("q4", "Q4 close")
                .with_context_menu_items(vec![MenuEntry::new("delete", "Delete")]),
            Item::new("all", "All records"),
            Item::new("archive", "Archive")
                .with_context_menu_items(vec![MenuEntry::new("delete", "Delete")])
                .with_disabled(true),
        ]);
        let node = sidebar_nav_with_handlers(
            &spec,
            &ctx,
            SidebarNavHandlers {
                on_context_menu: Some(Arc::new(move |_value, origin| {
                    assert_eq!(origin, SidebarNavContextMenuOrigin::Pointer(NodePoint::default()));
                    sink.lock().unwrap().push("q4".to_string());
                })),
                ..SidebarNavHandlers::default()
            },
        );

        let hosted = find(&node, "sidebar-nav-q4").expect("item with menu");
        assert!(hosted.interaction.on_context.is_some());
        assert!(hosted.interaction.on_key.is_some());

        let plain = find(&node, "sidebar-nav-all").expect("plain item");
        assert!(plain.interaction.on_context.is_none());
        assert!(plain.interaction.on_key.is_none());

        let disabled = find(&node, "sidebar-nav-archive").expect("disabled item");
        assert!(disabled.interaction.on_context.is_none());
        assert!(disabled.interaction.on_key.is_none());
        assert!(disabled.interaction.on_activate.is_none());

        (hosted.interaction.on_context.as_ref().expect("pointer"))(NodePoint::default());
        assert_eq!(requests.lock().unwrap().as_slice(), ["q4"]);
    }

    #[test]
    fn keyboard_menu_gesture_maps_the_context_menu_key_and_shift_f10() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let requests: Arc<Mutex<Vec<(&'static str, bool)>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&requests);
        let spec = spec_with(vec![Item::new("q4", "Q4 close")
            .with_context_menu_items(vec![MenuEntry::new("delete", "Delete")])]);
        let node = sidebar_nav_with_handlers(
            &spec,
            &ctx,
            SidebarNavHandlers {
                on_context_menu: Some(Arc::new(move |value, origin| {
                    let keyboard = origin == SidebarNavContextMenuOrigin::Keyboard;
                    sink.lock().unwrap().push((
                        if keyboard { "keyboard" } else { "pointer" },
                        keyboard,
                    ));
                })),
                ..SidebarNavHandlers::default()
            },
        );
        let item = find(&node, "sidebar-nav-q4").expect("item");
        let keys = item.interaction.on_key.as_ref().expect("key handler");
        let mods = poodle_node::NodeModifiers::default();

        assert!(keys(NodeKey::ContextMenu, mods).is_none());
        assert!(keys(NodeKey::F10, poodle_node::NodeModifiers { shift: true, ..mods }).is_none());
        // A bare F10 is not a menu gesture, and other keys pass through.
        assert!(keys(NodeKey::F10, mods).is_none() && requests.lock().unwrap().len() == 2);
        assert!(keys(NodeKey::ArrowDown, mods).is_none());
        assert_eq!(requests.lock().unwrap().as_slice(), [("keyboard", true), ("keyboard", true)]);
    }
}
