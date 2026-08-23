//! Pagination — pages, arrows and bounds.
//!
//! Contract: `docs/contracts/components/pagination.md`
//! Ported from: `packages/jetstream/components/src/pagination_comp.rs`.
//!
//! Anatomy (per contract §2):
//! ```text
//! [Wrapping Row]
//!   ├── [Info]           (optional — show_info && total > 0)
//!   ├── [Limit Selector] (optional — show_limit_selector)
//!   ├── First            (wired full variant)
//!   ├── Previous
//!   ├── Pages / Summary  (variant-specific center)
//!   ├── Next
//!   └── Last             (wired full variant)
//! ```
//!
//! `on_page_change` carries the destination page: prev and next resolve to a
//! number here rather than being separate events. The current page, a disabled
//! arrow and the ellipsis never fire. `pagination_with_handlers` additionally
//! exposes the limit Select's controlled open and page-size changes.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole,
};
use poodle_specs::{ChoiceOption, PageItem, PaginationSpec, PaginationVariant, SelectSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
};

#[derive(Default, Clone)]
pub struct PaginationHandlers {
    pub page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub limit_open: bool,
    pub limit_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub page_size_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

pub fn pagination(
    spec: &PaginationSpec,
    ctx: &RenderContext<'_>,
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Node {
    pagination_with_handlers(
        spec,
        ctx,
        &PaginationHandlers {
            page_change: on_page_change,
            ..PaginationHandlers::default()
        },
    )
}

pub fn pagination_with_handlers(
    spec: &PaginationSpec,
    ctx: &RenderContext<'_>,
    handlers: &PaginationHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);

    let base_height = ctx.theme().resolve_space("size.control.height");
    let height = base_height + rem_to_px(size_height_offset_rem(effective_size)) - rem_to_px(0.125);
    let button_min_width = base_height + rem_to_px(size_height_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.control.x")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let gap_sm = ctx.theme().resolve_space("space.inline.sm");
    let gap_md = ctx.theme().resolve_space("space.inline.md");
    let root_gap = if spec.is_compact { gap_sm } else { gap_md };
    let label_size = ctx.theme().resolve_space("typography.label.size");
    let icon_size = ctx.theme().resolve_space("size.icon.sm");
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let text_primary = ctx.theme().resolve_color(spec.button_text_token());
    let text_secondary = ctx.theme().resolve_color(spec.ellipsis_color_token());
    let border_color = ctx.theme().resolve_color(spec.button_border_token());
    let surface = ctx.theme().resolve_color(spec.button_fill_token());
    let accent = ctx.theme().resolve_color(spec.current_fill_token());

    // Current-page button: 18% accent bg, border at 42% accent / 58% default.
    let current_bg = with_alpha(accent, accent.3 * 0.18);
    let current_border = mix_srgb(accent, border_color, 0.58);

    // Border at 78% opacity (matches Svelte `color-mix … 78%`).
    let button_border = with_alpha(border_color, border_color.3 * 0.78);

    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());

    // One wrapping row: info, optional limit selector and navigation controls.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = root_gap;
        s.flex_wrap = true;
    }

    // Chrome root treatment: padding, uniform subtle border and surface fill.
    // `resolved_chrome` applies the contract's precedence — `standalone` only
    // overrides when the host set it, otherwise `chrome` decides.
    if spec.resolved_chrome() {
        let surface_bg = ctx.theme().resolve_color("color.background.surface");
        let border_subtle = ctx.theme().resolve_color("color.border.subtle");
        let chrome_radius = ctx.theme().resolve_radius("radius.control");
        let s = &mut root.style;
        s.descriptor.background = Some(surface_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_subtle;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = chrome_radius;
        c.top_right = chrome_radius;
        c.bottom_right = chrome_radius;
        c.bottom_left = chrome_radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = if spec.is_compact { gap_sm } else { gap_md };
        pad.right = pad.left;
        pad.top = if spec.is_compact {
            ctx.theme().resolve_space("space.inline.xs")
        } else {
            gap_sm
        };
        pad.bottom = pad.top;
    }

    if spec.is_loading {
        root.style.descriptor.opacity = disabled_opacity;
    }

    // ── info row ───────────────────────────────────────────────────────────────
    if spec.show_info {
        if let Some(text) = spec.info_string() {
            let mut info = Node::text(&text);
            info.style.text_size = Some(label_size);
            info.style.descriptor.text_color = Some(text_secondary);
            root = root.child(info);
        }
    }

    // Limit selector (all variants per contract §2 — "Show [n] per page").
    if spec.show_limit_selector && !spec.limit_options.is_empty() {
        root = root.child(build_limit_selector(
            spec, ctx, handlers, height, label_size, pad_x, radius, gap_sm,
        ));
    }

    // Helper: build a page-item / nav button (square-min-width, content-driven width).
    let make_button =
        |label: &str, is_page: bool, is_current: bool, is_disabled: bool, goto: Option<usize>| {
            let bg = if is_current { current_bg } else { surface };
            let bc = if is_current {
                current_border
            } else {
                button_border
            };

            let mut btn = Node::button(label);
            {
                let s = &mut btn.style;
                // Contract: min-width = control-height, height = control-height,
                // content-driven width with padding 0 control-x.
                if is_page {
                    s.min_width = Some(button_min_width);
                }
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
                if is_page {
                    s.text_weight = Some(if is_current { 700 } else { 600 });
                }
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.cursor = CursorHint::Pointer;
            }
            btn.a11y.role = Some(NodeRole::Button);
            btn.interaction.focusable = true;

            if is_disabled || spec.is_loading {
                btn.style.descriptor.opacity = disabled_opacity;
                btn.interaction.disabled = true;
            } else if !is_current {
                // The current page is where you already are, so it is not a route.
                if let (Some(page), Some(handler)) = (goto, &handlers.page_change) {
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
    if is_full && handlers.page_change.is_some() {
        root = root.child(make_button("««", false, false, prev_disabled, Some(1)));
    }

    // Previous button — text "Prev" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        root = root.child(make_button(
            "Prev",
            false,
            false,
            prev_disabled,
            Some(spec.current_page.saturating_sub(1).max(1)),
        ));
    } else {
        root = root.child(arrow_button(
            "chevron-left",
            prev_disabled || spec.is_loading,
            Some(spec.current_page.saturating_sub(1).max(1)),
            handlers.page_change.as_ref(),
            height,
            button_min_width,
            icon_size,
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
                s.descriptor.layout.spacing.gap = gap_sm;
            }
            for item in spec.visible_pages() {
                match item {
                    PageItem::Page(n) => {
                        let is_current = n == spec.current_page;
                        pages_row = pages_row.child(make_button(
                            &n.to_string(),
                            true,
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
                        let mut dots = Node::text("...");
                        dots.style.text_size = Some(font_size);
                        dots.style.text_weight = Some(600);
                        dots.style.descriptor.text_color = Some(text_secondary);
                        pages_row = pages_row.child(cell.child(dots));
                    }
                }
            }
            root = root.child(pages_row);
        }
        PaginationVariant::Full => {
            // Contract: full center summary = "Page X of Y".
            let mut label = Node::text(spec.full_summary());
            label.style.text_size = Some(label_size);
            label.style.descriptor.text_color = Some(text_secondary);
            root = root.child(label);
        }
        PaginationVariant::Simple => {
            // Contract: simple center summary = item range "X–Y of Z".
            let mut label = Node::text(spec.simple_summary());
            label.style.text_size = Some(label_size);
            label.style.descriptor.text_color = Some(text_secondary);
            root = root.child(label);
        }
    }

    // Next button — text "Next" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        root = root.child(make_button(
            "Next",
            false,
            false,
            next_disabled,
            Some((spec.current_page + 1).min(spec.total_pages)),
        ));
    } else {
        root = root.child(arrow_button(
            "chevron-right",
            next_disabled || spec.is_loading,
            Some((spec.current_page + 1).min(spec.total_pages)),
            handlers.page_change.as_ref(),
            height,
            button_min_width,
            icon_size,
            radius,
            button_border,
            surface,
            text_primary,
            disabled_opacity,
            pad_x,
        ));
    }

    // Last button (`»»`) — full variant only.
    if is_full && handlers.page_change.is_some() {
        root = root.child(make_button(
            "»»",
            false,
            false,
            next_disabled,
            Some(spec.total_pages),
        ));
    }

    let mut root = root;
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

// ── helpers ────────────────────────────────────────────────────────────────────

/// Chevron arrow nav button (prev / next). Square min-width, icon-only.
#[expect(
    clippy::too_many_arguments,
    reason = "navigation button rendering keeps resolved token metrics explicit"
)]
fn arrow_button(
    icon: &str,
    is_disabled: bool,
    goto: Option<usize>,
    on_page_change: Option<&Arc<dyn Fn(usize) + Send + Sync>>,
    height: f32,
    min_width: f32,
    icon_size: f32,
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
    btn.a11y.role = Some(NodeRole::Button);
    btn.a11y.label = Some(action_label.to_string());
    {
        let s = &mut btn.style;
        s.min_width = Some(min_width);
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

    let mut chevron = Node::icon(icon, icon_size);
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

/// Limit selector: "Show [n ▾] per page" — matches the old GPUI row anatomy.
/// Fully wired hosts get the shared Select; static callers keep the closed box.
#[expect(
    clippy::too_many_arguments,
    reason = "limit selector rendering keeps pagination state and metrics explicit"
)]
fn build_limit_selector(
    spec: &PaginationSpec,
    ctx: &RenderContext<'_>,
    handlers: &PaginationHandlers,
    height: f32,
    font_size: f32,
    pad_x: f32,
    radius: f32,
    gap: f32,
) -> Node {
    let text_secondary = ctx.theme().resolve_color(spec.ellipsis_color_token());
    let text_primary = ctx.theme().resolve_color(spec.button_text_token());
    let raw_border = ctx.theme().resolve_color(spec.button_border_token());
    let border_color = with_alpha(raw_border, raw_border.3 * 0.78);
    let surface = ctx.theme().resolve_color(spec.button_fill_token());

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

    if handlers.limit_open_change.is_some() && handlers.page_size_change.is_some() {
        let options = spec
            .limit_options
            .iter()
            .copied()
            .map(|size| ChoiceOption::new(size.to_string(), size.to_string()))
            .collect();
        let select_spec = SelectSpec::new(options)
            .with_value(page_size_label.clone())
            .with_open(handlers.limit_open)
            .with_size(ctx.base_size(spec.size))
            .with_size_role(spec.size_role)
            .with_density(ctx.resolve_density(spec.density))
            .with_aria_label("Items per page");

        let toggle = handlers.limit_open_change.as_ref().map(|handler| {
            let handler = Arc::clone(handler);
            let next = !handlers.limit_open;
            Arc::new(move || handler(next)) as Arc<dyn Fn() + Send + Sync>
        });
        let change = handlers.page_size_change.as_ref().map(|handler| {
            let handler = Arc::clone(handler);
            Arc::new(move |value: &str| {
                if let Ok(size) = value.parse::<usize>() {
                    handler(size);
                }
            }) as Arc<dyn Fn(&str) + Send + Sync>
        });
        let select_handlers = crate::SelectHandlers {
            toggle,
            change,
            clear: None,
        };
        let select_box = crate::select(&select_spec, ctx, &select_handlers);

        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
        }
        return row
            .child(text("Show", text_secondary))
            .child(select_box)
            .child(text("per page", text_secondary));
    }

    // Static <select> visual when the host did not wire controlled state.
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
