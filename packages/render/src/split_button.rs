//! SplitButton — a primary action with a menu of alternates.
//!
//! Contract: `docs/contracts/components/split-button.md`
//! Ported from: `packages/jetstream/components/src/split_button.rs`.
//!
//! Menu open/close, click-outside and keyboard navigation are host-owned;
//! the menu panel renders from `spec.is_open` only.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, StylePatch,
};
use poodle_specs::{ButtonTone, ButtonVariant, SplitButtonSpec, SplitMenuItem};

use crate::color::{mix_srgb, with_alpha, BLACK, TRANSPARENT};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem, split_button_chevron_size_rem,
    split_button_toggle_width_rem,
};

/// Host callbacks: primary half, chevron half, and menu-item value.
#[derive(Default)]
pub struct SplitButtonHandlers {
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_dropdown: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Resolved variant × tone color set for the split-button halves (contract §8
/// custom-property matrix).
struct SplitColors {
    fill: ColorValue,
    border: ColorValue,
    text: ColorValue,
}

fn resolve_split_colors(spec: &SplitButtonSpec, theme: &dyn ThemeProvider) -> SplitColors {
    let surface = theme.resolve_color("color.background.surface");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");
    let accent = theme.resolve_color("color.accent.base");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_inverse = theme.resolve_color("color.text.inverse");

    // Status family for danger/success/warning tone mixes.
    let status: Option<ColorValue> = match spec.tone {
        ButtonTone::Danger => Some(theme.resolve_color("color.status.danger")),
        ButtonTone::Success => Some(theme.resolve_color("color.status.success")),
        ButtonTone::Warning => Some(theme.resolve_color("color.status.warning")),
        ButtonTone::Default => None,
    };

    match (spec.variant, status) {
        // Ghost with status: fully transparent fill+border, status text.
        (ButtonVariant::Ghost, Some(status_color)) => SplitColors {
            fill: TRANSPARENT,
            border: TRANSPARENT,
            text: status_color,
        },
        // Default ghost: surface@42% fill, border-subtle@72% border.
        (ButtonVariant::Ghost, None) => SplitColors {
            fill: with_alpha(surface, surface.3 * 0.42),
            border: with_alpha(border_subtle, border_subtle.3 * 0.72),
            text: text_primary,
        },
        // Primary with status: status fill, status↔black 84% border, inverse.
        (ButtonVariant::Primary, Some(status_color)) => SplitColors {
            fill: status_color,
            border: mix_srgb(status_color, BLACK, 0.84),
            text: text_inverse,
        },
        // Primary default: accent fill, accent↔black 84% border, inverse.
        (ButtonVariant::Primary, None) => SplitColors {
            fill: accent,
            border: mix_srgb(accent, BLACK, 0.84),
            text: text_inverse,
        },
        // Secondary with status: status@16%↔surface fill, status@46%↔default.
        (_, Some(status_color)) => SplitColors {
            fill: mix_srgb(status_color, surface, 0.16),
            border: mix_srgb(status_color, border_default, 0.46),
            text: text_primary,
        },
        // Secondary default.
        (_, None) => SplitColors {
            fill: surface,
            border: border_default,
            text: text_primary,
        },
    }
}

pub fn split_button(
    spec: &SplitButtonSpec,
    theme: &dyn ThemeProvider,
    handlers: SplitButtonHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 Chevron: per-size icon dimension.
    let chevron_size = rem_to_px(split_button_chevron_size_rem(effective_size));
    // Spinner glyph tracks the supporting-visual size (one stop down).
    let spinner_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));
    // Contract §8 Divider: 60% of control height, centered.
    let divider_h = height * 0.6;
    // Contract §8 Toggle half: per-size width, zero padding.
    let toggle_w = rem_to_px(split_button_toggle_width_rem(effective_size));
    // Contract §8 Primary half: spinner↔label gap = space.inline.sm.
    let primary_gap = theme.resolve_space("space.inline.sm");

    let colors = resolve_split_colors(spec, theme);
    let elevated = theme.resolve_color("color.background.elevated");
    // Contract §4 hover / active fills.
    let hover_fill = mix_srgb(colors.fill, elevated, 0.84);
    let active_fill = mix_srgb(colors.fill, elevated, 0.72);

    // Contract §8 Divider: color-mix(split-text 22%, transparent).
    let divider_color = with_alpha(colors.text, colors.text.3 * 0.22);
    let radius = theme.resolve_radius(spec.radius_token());

    let is_unavailable = spec.is_unavailable();
    let label = spec.label.as_deref().unwrap_or("");

    let hover_patch = || StylePatch {
        background: Some(hover_fill),
        border_color: None,
        text_color: None,
        opacity: None,
    };
    let active_patch = || StylePatch {
        background: Some(active_fill),
        border_color: None,
        text_color: None,
        opacity: None,
    };

    // ── Root row ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }

    // ── Primary half ──
    let mut primary = Node::button("");
    // The caption is a child so layout places it beside the spinner; name
    // the button from the spec.
    primary.a11y.label = Some(spec.label.clone().unwrap_or_default());
    {
        let s = &mut primary.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.min_width = Some(rem_to_px(4.0)); // contract §7: min-width 4rem flat
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        // Left-rounded only.
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(colors.fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = colors.border;
        s.descriptor.text_color = Some(colors.text);
        s.text_size = Some(font_size);
        s.text_weight = Some(500);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = primary_gap;
    }
    primary.interaction.focusable = true;

    if !is_unavailable {
        primary.style.hover = Some(hover_patch());
        primary.style.active = Some(active_patch());
        primary.style.descriptor.cursor = CursorHint::Pointer;

        if let Some(handler) = &handlers.on_click {
            let handler = Arc::clone(handler);
            primary.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    // Contract §4/§8: loading shows the spinner glyph before the label.
    let mut primary = primary;
    if spec.is_loading {
        let mut spin = Node::icon("loader", spinner_size);
        spin.style.descriptor.text_color = Some(colors.text);
        primary = primary.child(spin);
    }
    if !label.is_empty() {
        let mut caption = Node::text(label);
        caption.style.text_size = Some(font_size);
        caption.style.text_weight = Some(500);
        caption.style.descriptor.text_color = Some(colors.text);
        caption.style.letter_spacing_em = Some(0.01); // contract §8
        caption.style.no_wrap = true;
        primary = primary.child(caption);
    }

    let mut root = root.child(primary);

    // ── Divider (60% height, vertically centered by the row) ──
    let mut divider = Node::container();
    {
        let s = &mut divider.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(1.0);
        s.descriptor.layout.height = LayoutSizing::Fixed(divider_h);
        s.descriptor.background = Some(divider_color);
    }
    root = root.child(divider);

    // ── Toggle half (fixed per-size width, zero padding) ──
    let mut toggle = Node::button("");
    // Chevron-only: nothing in its subtree carries text.
    toggle.a11y.label = Some("More actions".to_string());
    {
        let s = &mut toggle.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.width = LayoutSizing::Fixed(toggle_w);
        // Right-rounded only.
        let c = &mut s.descriptor.corner_radii;
        c.top_right = radius;
        c.bottom_right = radius;
        s.descriptor.background = Some(colors.fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = colors.border;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    toggle.interaction.focusable = true;
    let mut chevron = Node::icon("chevron-down", chevron_size);
    chevron.style.descriptor.text_color = Some(colors.text);
    let mut toggle = toggle.child(chevron);

    if !is_unavailable {
        toggle.style.hover = Some(hover_patch());
        toggle.style.active = Some(active_patch());
        toggle.style.descriptor.cursor = CursorHint::Pointer;

        if let Some(handler) = &handlers.on_dropdown {
            let handler = Arc::clone(handler);
            toggle.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    root = root.child(toggle);

    // ── Disabled / loading: dim the whole control ──
    if is_unavailable {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    // ── Menu overlay (rendered when open) ──
    // Stacked below the row inside a column wrapper.
    if spec.is_open && !spec.items.is_empty() {
        let menu_fill = theme.resolve_color(spec.overlay_fill_token());
        let menu_border = theme.resolve_color("color.border.default");
        let menu_radius = theme.resolve_radius("radius.surface");
        let item_text = theme.resolve_color("color.text.primary");
        let accent = theme.resolve_color("color.accent.base");
        let sep_color = theme.resolve_color("color.border.subtle");
        // Contract §8 Menu: padding 0.25rem; min-width 12rem.
        let menu_pad = rem_to_px(0.25);
        let menu_min_w = rem_to_px(12.0);
        let item_pad_x = theme.resolve_space("space.control.x");
        let item_pad_y = theme.resolve_space("space.control.y");
        let item_min_h = theme.resolve_space(spec.control_height_token());
        // Contract §8 Item: border-radius calc(radius-control − 0.125rem).
        let item_radius = (theme.resolve_radius("radius.control") - rem_to_px(0.125)).max(0.0);
        // Contract §8 Item hover: accent@16%.
        let item_hover = with_alpha(accent, accent.3 * 0.16);
        // Contract §8 Menu separator: border-subtle@72%.
        let menu_sep_color = with_alpha(sep_color, sep_color.3 * 0.72);

        // Contract: the dropdown is a `menu` of `menuitem`s.
        let mut menu = Node::container();
        menu.a11y.role = Some(NodeRole::Menu);
        {
            let s = &mut menu.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(menu_min_w);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = menu_pad;
            pad.right = menu_pad;
            pad.top = menu_pad;
            pad.bottom = menu_pad;
            s.descriptor.layout.spacing.margin.top = rem_to_px(0.375); // contract §8
            s.descriptor.background = Some(menu_fill);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = menu_border;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = menu_radius;
            c.top_right = menu_radius;
            c.bottom_right = menu_radius;
            c.bottom_left = menu_radius;
        }

        for item in &spec.items {
            match item {
                SplitMenuItem::Action {
                    value,
                    label,
                    is_disabled,
                } => {
                    let mut item_el = Node::button(label);
                    item_el.a11y.role = Some(NodeRole::MenuItem);
                    {
                        let s = &mut item_el.style;
                        s.min_height = Some(item_min_h);
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = item_pad_x;
                        pad.right = item_pad_x;
                        pad.top = item_pad_y;
                        pad.bottom = item_pad_y;
                        let c = &mut s.descriptor.corner_radii;
                        c.top_left = item_radius;
                        c.top_right = item_radius;
                        c.bottom_right = item_radius;
                        c.bottom_left = item_radius;
                        s.text_size = Some(font_size);
                        s.descriptor.text_color = Some(item_text);
                        s.descriptor.layout.direction = LayoutDirection::Row;
                        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    }
                    item_el.interaction.focusable = true;
                    if *is_disabled {
                        item_el.style.descriptor.opacity =
                            theme.resolve_opacity(spec.disabled_opacity_token());
                        item_el.interaction.disabled = true;
                    } else {
                        item_el.style.hover = Some(StylePatch {
                            background: Some(item_hover),
                            border_color: None,
                            text_color: None,
                            opacity: None,
                        });
                        item_el.style.descriptor.cursor = CursorHint::Pointer;

                        if let Some(handler) = &handlers.on_action {
                            let handler = Arc::clone(handler);
                            let value = value.clone();
                            item_el.interaction.on_activate =
                                Some(Arc::new(move || handler(&value)));
                        }
                    }
                    menu = menu.child(item_el);
                }
                SplitMenuItem::Separator => {
                    let mut sep = Node::container();
                    // Contract §6: menu separators are announced.
                    sep.a11y.role = Some(NodeRole::Splitter);
                    {
                        let s = &mut sep.style;
                        // Explicit Row (see switch.rs).
                        s.descriptor.layout.direction = LayoutDirection::Row;
                        s.fill_width = true;
                        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.0625));
                        s.descriptor.layout.spacing.margin.top = menu_pad;
                        s.descriptor.layout.spacing.margin.bottom = menu_pad;
                        s.descriptor.background = Some(menu_sep_color);
                    }
                    menu = menu.child(sep);
                }
            }
        }

        // Wrap the row + menu in a column so the menu stacks beneath.
        let mut wrapper = Node::container();
        wrapper.style.descriptor.layout.direction = LayoutDirection::Column;
        root = wrapper.child(root).child(menu);
    }

    // The visible label is the accessible name unless something overrides it.
    if let Some(label) = spec.aria_label.as_deref().or(spec.label.as_deref()) {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
