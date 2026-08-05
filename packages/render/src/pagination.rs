//! Pagination — pages, arrows and bounds.
//!
//! Contract: `docs/contracts/components/pagination.md`
//! Ported from: `packages/jetstream/components/src/pagination_comp.rs`.
//!
//! Anatomy (per contract §2):
//! ```text
//! [Root]
//!   ├── [Info]               (optional — show_info && total > 0)
//!   └── [Controls Wrapper]
//!         ├── [Limit Selector] (optional — show_limit_selector)
//!         └── [Controls]
//!               ├── First   (full variant)
//!               ├── Previous
//!               ├── Pages / Summary  (variant-specific center)
//!               ├── Next
//!               └── Last    (full variant)
//! ```
//!
//! `on_page_change` carries the destination page: prev and next resolve to a
//! number here rather than being separate events. The current page, a disabled
//! arrow and the ellipsis never fire. No `on_limit_change` — the page-size
//! control renders its Select closed.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node,
};
use poodle_specs::{ControlDensity, PageItem, PaginationSpec, PaginationVariant};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

/// Inter-control / inter-page gap in px for a density.
///
/// Contract §8 Density: compact `3px`, default `0.25rem`, comfortable `0.375rem`.
fn density_gap_px(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 3.0,
        ControlDensity::Default => rem_to_px(0.25),
        ControlDensity::Comfortable => rem_to_px(0.375),
    }
}

pub fn pagination(
    spec: &PaginationSpec,
    theme: &dyn ThemeProvider,
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract: controls/pages gap is density-driven (not pad_x).
    let gap = density_gap_px(spec.density);
    let radius = theme.resolve_radius(spec.radius_token());

    let text_primary = theme.resolve_color(spec.button_text_token());
    let text_secondary = theme.resolve_color(spec.ellipsis_color_token());
    let border_color = theme.resolve_color(spec.button_border_token());
    let surface = theme.resolve_color(spec.button_fill_token());
    let accent = theme.resolve_color(spec.current_fill_token());

    // Current-page button: 18% accent bg, border at 42% accent / 58% default.
    let current_bg = with_alpha(accent, accent.3 * 0.18);
    let current_border = mix_srgb(accent, border_color, 0.58);

    // Border at 78% opacity (matches Svelte `color-mix … 78%`).
    let button_border = with_alpha(border_color, border_color.3 * 0.78);

    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    // ── root: info row + controls-wrapper stacked. ─────────────────────────────
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
    }

    // Chrome root treatment: padding + top border + elevated 92% background.
    // `resolved_chrome` applies the contract's precedence — `standalone` only
    // overrides when the host set it, otherwise `chrome` decides.
    if spec.resolved_chrome() {
        let elevated = theme.resolve_color("color.background.elevated");
        let s = &mut root.style;
        s.descriptor.background = Some(with_alpha(elevated, elevated.3 * 0.92));
        s.border_top_width = Some(1.0);
        s.descriptor.border.color = border_color;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = theme.resolve_space("space.panel.x");
        pad.right = theme.resolve_space("space.panel.x");
        pad.top = theme.resolve_space("space.control.y");
        pad.bottom = theme.resolve_space("space.control.y");
    }

    if spec.is_loading {
        root.style.descriptor.opacity = disabled_opacity;
    }

    // ── info row ───────────────────────────────────────────────────────────────
    if spec.show_info {
        if let Some(text) = spec.info_string() {
            let mut info = Node::text(&text);
            info.style.text_size = Some(font_size);
            info.style.descriptor.text_color = Some(text_secondary);
            root = root.child(info);
        }
    }

    // ── controls-wrapper: limit selector + controls. ───────────────────────────
    let mut wrapper = Node::container();
    {
        let s = &mut wrapper.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.md");
    }

    // Limit selector (all variants per contract §2 — "Show [n] per page").
    if spec.show_limit_selector && !spec.limit_options.is_empty() {
        wrapper = wrapper.child(build_limit_selector(
            spec, theme, height, font_size, pad_x, radius, gap,
        ));
    }

    // Controls row.
    let mut controls = Node::container();
    {
        let s = &mut controls.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    // Helper: build a page-item / nav button (square-min-width, content-driven width).
    let make_button = |label: &str, is_current: bool, is_disabled: bool, goto: Option<usize>| {
        let bg = if is_current { current_bg } else { surface };
        let bc = if is_current { current_border } else { button_border };

        let mut btn = Node::button(label);
        {
            let s = &mut btn.style;
            // Contract: min-width = control-height, height = control-height,
            // content-driven width with padding 0 control-x.
            s.min_width = Some(height);
            s.descriptor.layout.height = LayoutSizing::Fixed(height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            s.descriptor.corner_radii.top_left = radius;
            s.descriptor.corner_radii.top_right = radius;
            s.descriptor.corner_radii.bottom_right = radius;
            s.descriptor.corner_radii.bottom_left = radius;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = bc;
            s.descriptor.background = Some(bg);
            s.descriptor.text_color = Some(text_primary);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.cursor = CursorHint::Pointer;
        }
        btn.interaction.focusable = true;

        if is_disabled || spec.is_loading {
            btn.style.descriptor.opacity = disabled_opacity;
            btn.interaction.disabled = true;
        } else if !is_current {
            // The current page is where you already are, so it is not a route.
            if let (Some(page), Some(handler)) = (goto, &on_page_change) {
                let handler = Arc::clone(handler);
                btn.interaction.on_activate = Some(Arc::new(move || handler(page)));
            }
        }

        btn
    };

    let is_full = spec.variant == PaginationVariant::Full;
    let prev_disabled = spec.is_first_page();
    let next_disabled = spec.is_last_page();

    // First button (`««`) — full variant only.
    if is_full {
        controls = controls.child(make_button("««", false, prev_disabled, Some(1)));
    }

    // Previous button — text "Prev" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        controls = controls.child(make_button(
            "Prev",
            false,
            prev_disabled,
            Some(spec.current_page.saturating_sub(1).max(1)),
        ));
    } else {
        controls = controls.child(arrow_button(
            "chevron-left",
            prev_disabled || spec.is_loading,
            Some(spec.current_page.saturating_sub(1).max(1)),
            on_page_change.as_ref(),
            height,
            font_size,
            radius,
            button_border,
            surface,
            text_primary,
            disabled_opacity,
            pad_x,
        ));
    }

    // Center content — variant-specific.
    match spec.variant {
        PaginationVariant::Numbered => {
            let mut pages_row = Node::container();
            {
                let s = &mut pages_row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = gap;
            }
            for item in spec.visible_pages() {
                match item {
                    PageItem::Page(n) => {
                        let is_current = n == spec.current_page;
                        pages_row = pages_row.child(make_button(
                            &n.to_string(),
                            is_current,
                            false,
                            Some(n),
                        ));
                    }
                    PageItem::Ellipsis => {
                        // Non-interactive ellipsis; contract min-width 1.5rem.
                        let mut cell = Node::container();
                        {
                            let s = &mut cell.style;
                            s.min_width = Some(rem_to_px(1.5));
                            s.descriptor.layout.height = LayoutSizing::Fixed(height);
                            s.descriptor.layout.direction = LayoutDirection::Row;
                            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                        }
                        let mut dots = Node::text("…");
                        dots.style.text_size = Some(font_size);
                        dots.style.descriptor.text_color = Some(text_secondary);
                        pages_row = pages_row.child(cell.child(dots));
                    }
                }
            }
            controls = controls.child(pages_row);
        }
        PaginationVariant::Full => {
            // Contract: full center summary = "Page X of Y".
            controls = controls.child(summary_label(
                &spec.full_summary(),
                font_size,
                text_secondary,
            ));
        }
        PaginationVariant::Simple => {
            // Contract: simple center summary = item range "X–Y of Z".
            controls = controls.child(summary_label(
                &spec.simple_summary(),
                font_size,
                text_secondary,
            ));
        }
    }

    // Next button — text "Next" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        controls = controls.child(make_button(
            "Next",
            false,
            next_disabled,
            Some((spec.current_page + 1).min(spec.total_pages)),
        ));
    } else {
        controls = controls.child(arrow_button(
            "chevron-right",
            next_disabled || spec.is_loading,
            Some((spec.current_page + 1).min(spec.total_pages)),
            on_page_change.as_ref(),
            height,
            font_size,
            radius,
            button_border,
            surface,
            text_primary,
            disabled_opacity,
            pad_x,
        ));
    }

    // Last button (`»»`) — full variant only.
    if is_full {
        controls = controls.child(make_button("»»", false, next_disabled, Some(spec.total_pages)));
    }

    let wrapper = wrapper.child(controls);
    let mut root = root.child(wrapper);
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

// ── helpers ────────────────────────────────────────────────────────────────────

/// Variant-center summary text with 0.5rem side padding.
fn summary_label(text: &str, font_size: f32, color: ColorValue) -> Node {
    let mut label = Node::text(text);
    label.style.text_size = Some(font_size);
    label.style.descriptor.text_color = Some(color);
    let pad = &mut label.style.descriptor.layout.spacing.padding;
    pad.left = rem_to_px(0.5);
    pad.right = rem_to_px(0.5);
    label
}

/// Chevron arrow nav button (prev / next). Square min-width, icon-only.
#[allow(clippy::too_many_arguments)]
fn arrow_button(
    icon: &str,
    is_disabled: bool,
    goto: Option<usize>,
    on_page_change: Option<&Arc<dyn Fn(usize) + Send + Sync>>,
    height: f32,
    font_size: f32,
    radius: f32,
    button_border: ColorValue,
    surface: ColorValue,
    text_primary: ColorValue,
    disabled_opacity: f32,
    pad_x: f32,
) -> Node {
    // Icon-only, so there is no text anywhere for a screen reader to name it
    // from — not on the button and not in its children, since an icon carries
    // no text. Without this it is announced as an unnamed "button".
    let action_label = match icon {
        "chevron-left" => "Previous page",
        "chevron-right" => "Next page",
        "chevrons-left" => "First page",
        "chevrons-right" => "Last page",
        other => other,
    };

    let mut btn = Node::button("");
    btn.a11y.label = Some(action_label.to_string());
    {
        let s = &mut btn.style;
        s.min_width = Some(height);
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = button_border;
        s.descriptor.background = Some(surface);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    btn.interaction.focusable = true;

    let mut chevron = Node::icon(icon, font_size);
    chevron.style.descriptor.text_color = Some(text_primary);
    let mut btn = btn.child(chevron);

    if is_disabled {
        btn.style.descriptor.opacity = disabled_opacity;
        btn.interaction.disabled = true;
    } else if let (Some(page), Some(handler)) = (goto, on_page_change) {
        let handler = Arc::clone(handler);
        btn.interaction.on_activate = Some(Arc::new(move || handler(page)));
    }
    btn
}

/// Limit selector: "Show [n ▾] per page" — matches Svelte limit row anatomy.
/// The select is rendered as a static bordered box (interaction is host-side).
#[allow(clippy::too_many_arguments)]
fn build_limit_selector(
    spec: &PaginationSpec,
    theme: &dyn ThemeProvider,
    height: f32,
    font_size: f32,
    pad_x: f32,
    radius: f32,
    gap: f32,
) -> Node {
    let text_secondary = theme.resolve_color(spec.ellipsis_color_token());
    let text_primary = theme.resolve_color(spec.button_text_token());
    let raw_border = theme.resolve_color(spec.button_border_token());
    let border_color = with_alpha(raw_border, raw_border.3 * 0.78);
    let surface = theme.resolve_color(spec.button_fill_token());

    let page_size_label = spec
        .page_size
        .map(|s| s.to_string())
        .unwrap_or_else(|| spec.limit_options[0].to_string());

    let text = |content: &str, color: ColorValue| {
        let mut t = Node::text(content);
        t.style.text_size = Some(font_size);
        t.style.descriptor.text_color = Some(color);
        t
    };

    // <select> visual: bordered box + value + chevron.
    let mut select_box = Node::container();
    {
        let s = &mut select_box.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
        s.descriptor.background = Some(surface);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.layout.spacing.gap = gap;
    }
    let mut chevron = Node::icon("chevron-down", font_size);
    chevron.style.descriptor.text_color = Some(text_secondary);
    let select_box = select_box
        .child(text(&page_size_label, text_primary))
        .child(chevron);

    // Limit row gap = 0.375rem per contract §8 Limit Selector.
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.375);
    }
    row.child(text("Show", text_secondary))
        .child(select_box)
        .child(text("per page", text_secondary))
}
