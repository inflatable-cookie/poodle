//! ThemeSelect — a grid of theme swatches behind a trigger.
//!
//! Contract: `docs/contracts/components/theme-select.md`
//! Ported from: `packages/jetstream/components/src/theme_select.rs`.
//!
//! Swatch hexes land in sRGB and linearise at the adapter edge — the same
//! endpoint as the reference tier's explicit `to_linear` conversion.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodePosition,
};
use poodle_specs::{ControlSize, ThemeOption, ThemeSelectSpec};

use crate::color::hex_color;
use crate::presentation::{rem_to_px, resolve_semantic_size};

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn abs_block(
    top: Option<f32>,
    left: Option<f32>,
    right: Option<f32>,
    bottom: Option<f32>,
    w: f32,
    h: f32,
    radius: f32,
    bg: ColorValue,
) -> Node {
    let mut n = Node::container();
    n.position = NodePosition::Absolute {
        top,
        left,
        right,
        bottom,
    };
    {
        let s = &mut n.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(w);
        s.descriptor.layout.height = LayoutSizing::Fixed(h);
        s.descriptor.background = Some(bg);
    }
    all_corners(&mut n, radius);
    n
}

/// Mini theme preview: canvas fill + surface card + accent dot + text bar.
fn swatch(option: &ThemeOption, theme: &dyn ThemeProvider, w: f32, h: f32, selected: bool) -> Node {
    let fallback = theme.resolve_color("color.background.surface");
    let color = |hex: &str| hex_color(hex).unwrap_or(fallback);
    let border = theme.resolve_color("color.border.subtle");
    let accent = theme.resolve_color("color.accent.base");

    let mut root = Node::container();
    root.position = NodePosition::Relative;
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(w));
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(h));
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = if selected { accent } else { border };
        s.descriptor.background = Some(color(&option.swatch.canvas));
    }
    all_corners(&mut root, rem_to_px(0.375));

    root.child(abs_block(
        None,
        Some(rem_to_px(w * 0.14)),
        None,
        Some(0.0),
        rem_to_px(w * 0.72),
        rem_to_px(h * 0.52),
        0.0,
        color(&option.swatch.surface),
    ))
    .child(abs_block(
        Some(rem_to_px(h * 0.18)),
        Some(rem_to_px(w * 0.16)),
        None,
        None,
        rem_to_px(h * 0.26),
        rem_to_px(h * 0.26),
        rem_to_px(h * 0.13),
        color(&option.swatch.accent),
    ))
    .child(abs_block(
        Some(rem_to_px(h * 0.24)),
        None,
        Some(rem_to_px(w * 0.16)),
        None,
        rem_to_px(w * 0.34),
        rem_to_px(0.125),
        rem_to_px(0.0625),
        color(&option.swatch.text),
    ))
}

/// Host callbacks. `on_change` fires with the chosen theme's value;
/// `on_open_change` fires with the open state the trigger is moving **to**,
/// since `ThemeSelectSpec::is_open` is controlled by the host.
#[derive(Default)]
pub struct ThemeSelectHandlers {
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn theme_select(
    spec: &ThemeSelectSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    theme_select_with_handlers(
        spec,
        theme,
        ThemeSelectHandlers {
            on_change,
            ..ThemeSelectHandlers::default()
        },
    )
}

pub fn theme_select_with_handlers(
    spec: &ThemeSelectSpec,
    theme: &dyn ThemeProvider,
    handlers: ThemeSelectHandlers,
) -> Node {
    let on_change = handlers.on_change;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });

    let text_primary = theme.resolve_color(spec.field_text_token());
    let text_secondary = theme.resolve_color(spec.label_color_token());
    let border = theme.resolve_color(spec.field_border_token());
    let surface = theme.resolve_color(spec.field_fill_token());
    let elevated = theme.resolve_color(spec.surface_fill_token());
    let item_border = theme.resolve_color(spec.item_border_token());
    let accent = theme.resolve_color(spec.accent_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let surface_radius = theme.resolve_radius(spec.surface_radius_token());

    // ── Trigger ─────────────────────────────────────────────────────────
    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.min_height = Some(trigger_h);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = rem_to_px(0.75);
        pad.right = rem_to_px(0.75);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(surface);
    }
    all_corners(&mut trigger, radius);
    // Contract §States: "open | click trigger | popover grid of swatch tiles".
    // `is_open` is controlled, so the trigger reports the state it is moving to
    // and the host flips the spec.
    if !spec.is_disabled {
        trigger.id = Some("theme-select-trigger".to_string());
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.interaction.focusable = true;
        if let Some(handler) = &handlers.on_open_change {
            let handler = Arc::clone(handler);
            let next = !spec.is_open;
            trigger.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    let mut trigger = trigger;
    if let Some(current) = spec.current_option() {
        trigger = trigger.child(swatch(current, theme, 1.25, 1.25, false));
    }
    if spec.show_label {
        let mut label = Node::text(&spec.trigger_label());
        label.style.descriptor.text_color = Some(text_primary);
        label.style.text_size = Some(rem_to_px(0.8125));
        trigger = trigger.child(label);
    }
    let mut chevron = Node::text("▾");
    chevron.style.descriptor.text_color = Some(text_secondary);
    let trigger = trigger.child(chevron);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
    }
    // Contract §2: the root is the `position: relative` anchor its surface
    // hangs from. The old GPUI tier laid the open surface out beside the
    // trigger as an ordinary flow sibling, which pushed the trigger around and
    // let neighbouring controls collide with it.
    root.position = NodePosition::Relative;
    let mut root = root.child(trigger);

    // ── Popover grid (rendered inline when open) ────────────────────────
    if spec.is_open {
        let mut grid = Node::container();
        {
            let s = &mut grid.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
            s.max_width = Some(rem_to_px(22.0));
        }
        for option in spec.themes.iter() {
            let selected = spec.is_selected(option);
            let mut tile = Node::container();
            // Stable per-option id. Backends that dispatch by id (Jetstream
            // routes on `token_key`) need it to reach the tile at all, and
            // GPUI needs identity that survives a rebuild between a click's
            // press and release.
            tile.id = Some(format!("theme-select-tile-{}", option.value));
            {
                let s = &mut tile.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.375);
                s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(4.5));
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.375);
                pad.right = rem_to_px(0.375);
                pad.top = rem_to_px(0.375);
                pad.bottom = rem_to_px(0.375);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = if selected {
                    accent
                } else {
                    ColorValue(0.0, 0.0, 0.0, 0.0)
                };
            }
            all_corners(&mut tile, radius);
            let mut label = Node::text(&option.label);
            label.style.descriptor.text_color = Some(text_primary);
            label.style.text_size = Some(rem_to_px(0.71875));
            let mut tile = tile
                .child(swatch(option, theme, 2.75, 2.0, selected))
                .child(label);

            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let id = option.value.clone();
                tile.style.descriptor.cursor = CursorHint::Pointer;
                tile.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            grid = grid.child(tile);
        }

        let mut panel = Node::container();
        {
            let s = &mut panel.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = item_border;
            s.descriptor.background = Some(elevated);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.5);
            pad.right = rem_to_px(0.5);
            pad.top = rem_to_px(0.5);
            pad.bottom = rem_to_px(0.5);
        }
        all_corners(&mut panel, surface_radius);
        // Anchored bottom-start with the reference's 0.5rem offset
        // (`use:anchored={{ placement: "bottom-start", offset: 8 }}`). Absolute,
        // so opening the picker never reflows the trigger or its neighbours.
        // Painting it above later siblings is the backend's job — GPUI's host
        // deferres it, the web target portals it.
        panel.position = NodePosition::Absolute {
            top: Some(trigger_h + rem_to_px(0.5)),
            left: Some(0.0),
            right: None,
            bottom: None,
        };
        root = root.child(panel.child(grid));
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    if !spec.aria_label.is_empty() {
        root.a11y.label = Some(spec.aria_label.clone());
    }
    root
}
