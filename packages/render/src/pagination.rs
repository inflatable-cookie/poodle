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
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing,
    MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{ChoiceOption, PageItem, PaginationSpec, PaginationVariant, SelectSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
};

#[derive(Clone)]
pub struct PaginationHandlers {
    pub instance_id: String,
    pub page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub limit_open: bool,
    pub limit_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub page_size_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl PaginationHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "PaginationHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            page_change: None,
            limit_open: false,
            limit_open_change: None,
            page_size_change: None,
        }
    }
}

pub fn pagination(
    spec: &PaginationSpec,
    ctx: &RenderContext<'_>,
    instance_id: impl Into<String>,
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Node {
    pagination_with_handlers(
        spec,
        ctx,
        &PaginationHandlers {
            page_change: on_page_change,
            ..PaginationHandlers::new(instance_id)
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
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };

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
                btn.interaction.focusable = false;
                btn.a11y.tab_index = None;
            } else {
                btn.a11y.tab_index = Some(0);
                btn.style.focus_ring = Some(FocusRing {
                    color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
                    width: ctx.theme().resolve_border_width("border.width.focus"),
                    offset: rem_to_px(0.125),
                });
                // The current page is where you already are, so it is not a route.
                if !is_current {
                    if let (Some(page), Some(handler)) = (goto, &handlers.page_change) {
                        let handler = Arc::clone(handler);
                        btn.interaction.on_activate = Some(Arc::new(move || handler(page)));
                    }
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
            Some(focus_ring),
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
            Some(focus_ring),
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
    focus_ring: Option<FocusRing>,
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
        btn.interaction.focusable = false;
        btn.a11y.tab_index = None;
    } else {
        btn.a11y.tab_index = Some(0);
        btn.style.focus_ring = focus_ring;
        if let (Some(page), Some(handler)) = (goto, on_page_change) {
            let handler = Arc::clone(handler);
            btn.interaction.on_activate = Some(Arc::new(move || handler(page)));
        }
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
        let mut select_spec = SelectSpec::new(options)
            .with_value(page_size_label.clone())
            .with_open(handlers.limit_open && !spec.is_loading)
            .with_size(ctx.base_size(spec.size))
            .with_size_role(spec.size_role)
            .with_density(ctx.resolve_density(spec.density))
            .with_aria_label("Items per page");
        // Match Svelte/React: loading disables the page-size Select as well as
        // every page button. Use Select's public disabled field — do not fork it.
        select_spec.is_disabled = spec.is_loading;

        let open_change = handlers.limit_open_change.clone();
        let size_change = handlers.page_size_change.clone();
        let mut select_handlers = crate::SelectHandlers::new(&handlers.instance_id);
        if open_change.is_some() || size_change.is_some() {
            select_handlers = select_handlers.on_transition(Arc::new(move |result| {
                for effect in &result.effects {
                    match effect {
                        crate::SelectEffect::OpenChanged { open } => {
                            if let Some(handler) = &open_change {
                                handler(*open);
                            }
                        }
                        crate::SelectEffect::ValueChanged { value } => {
                            if let Some(handler) = &size_change {
                                if let Ok(size) = value.parse::<usize>() {
                                    handler(size);
                                }
                            }
                        }
                        crate::SelectEffect::QueryChanged { .. } => {}
                    }
                }
            }));
        }
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
    select_box.runtime_id = Some(format!("select:{}:trigger", handlers.instance_id));
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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn numbered_spec() -> PaginationSpec {
        PaginationSpec::new()
            .with_current_page(5)
            .with_total_pages(20)
            .with_sibling_count(1)
            .with_show_limit_selector(true)
            .with_limit_options(vec![10, 25, 50])
            .with_page_size(10)
            .with_aria_label("Results pagination")
    }

    fn wired_handlers(
        pages: Arc<Mutex<Vec<usize>>>,
        opens: Arc<Mutex<Vec<bool>>>,
        sizes: Arc<Mutex<Vec<usize>>>,
        limit_open: bool,
    ) -> PaginationHandlers {
        let page_sink = Arc::clone(&pages);
        let open_sink = Arc::clone(&opens);
        let size_sink = Arc::clone(&sizes);
        PaginationHandlers {
            page_change: Some(Arc::new(move |page| {
                page_sink.lock().expect("page lock").push(page);
            })),
            limit_open,
            limit_open_change: Some(Arc::new(move |open| {
                open_sink.lock().expect("open lock").push(open);
            })),
            page_size_change: Some(Arc::new(move |size| {
                size_sink.lock().expect("size lock").push(size);
            })),
            ..PaginationHandlers::new("pagination-test")
        }
    }

    fn render(spec: &PaginationSpec, handlers: &PaginationHandlers) -> Node {
        pagination_with_handlers(spec, &RenderContext::new(&theme()), handlers)
    }

    fn activatable_by_label<'a>(root: &'a Node, label: &str) -> &'a Node {
        root.find(&|n| {
            n.a11y.label.as_deref() == Some(label) && n.interaction.on_activate.is_some()
        })
        .unwrap_or_else(|| panic!("activatable '{label}'"))
    }

    fn button_by_text<'a>(root: &'a Node, text: &str) -> &'a Node {
        root.find(&|n| {
            matches!(&n.kind, NodeKind::Button { .. })
                && n.has_text(text)
                && n.a11y.role == Some(NodeRole::Button)
        })
        .unwrap_or_else(|| panic!("button '{text}'"))
    }

    fn limit_trigger(root: &Node) -> &Node {
        root.find(&|n| {
            n.a11y.label.as_deref() == Some("Items per page")
                || (n.a11y.role == Some(NodeRole::ComboBox)
                    && n.find(&|c| c.has_text("10") || c.has_text("25") || c.has_text("50"))
                        .is_some())
        })
        .or_else(|| {
            // Closed Select returns the trigger root with no ComboBox role on a
            // wrapping container — find the trigger that owns the chevron.
            root.find(&|n| {
                n.interaction.focusable
                    && n.find(&|c| {
                        matches!(&c.kind, NodeKind::Icon { name, .. } if name == "chevron-down")
                    })
                    .is_some()
                    && n.find(&|c| c.has_text("10") || c.has_text("25") || c.has_text("50"))
                        .is_some()
            })
        })
        .expect("limit select trigger")
    }

    #[test]
    fn loading_disables_wired_page_size_select_and_page_buttons() {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(
            Arc::clone(&pages),
            Arc::clone(&opens),
            Arc::clone(&sizes),
            false,
        );
        let node = render(&numbered_spec().with_loading(true), &handlers);

        let trigger = limit_trigger(&node);
        assert!(
            trigger.interaction.disabled,
            "loading disables the Select trigger"
        );
        assert!(
            trigger.interaction.on_activate.is_none(),
            "loading Select must not report open changes"
        );

        let page_four = button_by_text(&node, "4");
        assert!(page_four.interaction.disabled);
        assert!(page_four.interaction.on_activate.is_none());

        let next = node
            .find(&|n| n.a11y.label.as_deref() == Some("Next page"))
            .expect("next");
        assert!(next.interaction.disabled);
        assert!(next.interaction.on_activate.is_none());

        assert!(pages.lock().expect("page lock").is_empty());
        assert!(opens.lock().expect("open lock").is_empty());
        assert!(sizes.lock().expect("size lock").is_empty());
    }

    #[test]
    fn loading_keeps_open_limit_options_from_reporting_size_changes() {
        // Host may still hold limit_open=true while loading; Pagination presents
        // the composed Select closed and disabled so neither trigger nor options
        // can emit.
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(
            Arc::clone(&pages),
            Arc::clone(&opens),
            Arc::clone(&sizes),
            true,
        );
        let node = render(&numbered_spec().with_loading(true), &handlers);

        let trigger = limit_trigger(&node);
        assert!(trigger.interaction.disabled);
        assert!(trigger.interaction.on_activate.is_none());
        assert!(
            node.find(&|n| n.a11y.role == Some(NodeRole::ListBoxOption))
                .is_none(),
            "loading must not present open page-size options"
        );

        assert!(sizes.lock().expect("size lock").is_empty());
        assert!(opens.lock().expect("open lock").is_empty());
    }

    #[test]
    fn numbered_page_and_adjacent_requests_report_destinations() {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(pages.clone(), opens, sizes, false);
        let node = render(&numbered_spec(), &handlers);

        let activate = button_by_text(&node, "4")
            .interaction
            .on_activate
            .as_ref()
            .expect("page 4");
        activate.as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [4]);

        pages.lock().expect("page lock").clear();
        activatable_by_label(&node, "Next page")
            .interaction
            .on_activate
            .as_ref()
            .expect("next")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [6]);

        pages.lock().expect("page lock").clear();
        activatable_by_label(&node, "Previous page")
            .interaction
            .on_activate
            .as_ref()
            .expect("prev")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [4]);
    }

    #[test]
    fn current_page_ellipsis_and_boundary_controls_emit_nothing() {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(pages.clone(), opens, sizes, false);

        // Middle of range: current page 5 has no activation; ellipsis is text.
        let mid = render(&numbered_spec(), &handlers);
        let current = button_by_text(&mid, "5");
        assert!(current.interaction.on_activate.is_none());
        assert!(mid.has_text("..."));

        // First page: previous is disabled.
        let first = render(
            &PaginationSpec::new()
                .with_current_page(1)
                .with_total_pages(5),
            &handlers,
        );
        let prev = first
            .find(&|n| n.a11y.label.as_deref() == Some("Previous page"))
            .expect("prev");
        assert!(prev.interaction.disabled);
        assert!(prev.interaction.on_activate.is_none());

        // Last page: next is disabled.
        let last = render(
            &PaginationSpec::new()
                .with_current_page(5)
                .with_total_pages(5),
            &handlers,
        );
        let next = last
            .find(&|n| n.a11y.label.as_deref() == Some("Next page"))
            .expect("next");
        assert!(next.interaction.disabled);
        assert!(next.interaction.on_activate.is_none());

        assert!(pages.lock().expect("page lock").is_empty());
    }

    #[test]
    fn simple_and_full_variants_report_adjacent_and_first_last_destinations() {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(pages.clone(), opens, sizes, false);

        let simple = render(
            &PaginationSpec::new()
                .with_current_page(3)
                .with_total_pages(10)
                .with_page_size(25)
                .with_total_items(248)
                .with_variant(PaginationVariant::Simple),
            &handlers,
        );
        assert!(simple.has_text("51–75 of 248"));
        button_by_text(&simple, "Next")
            .interaction
            .on_activate
            .as_ref()
            .expect("simple next")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [4]);
        pages.lock().expect("page lock").clear();
        button_by_text(&simple, "Prev")
            .interaction
            .on_activate
            .as_ref()
            .expect("simple prev")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [2]);
        pages.lock().expect("page lock").clear();

        let full = render(
            &PaginationSpec::new()
                .with_current_page(3)
                .with_total_pages(10)
                .with_variant(PaginationVariant::Full),
            &handlers,
        );
        assert!(full.has_text("Page 3 of 10"));
        button_by_text(&full, "««")
            .interaction
            .on_activate
            .as_ref()
            .expect("first")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [1]);
        pages.lock().expect("page lock").clear();
        button_by_text(&full, "»»")
            .interaction
            .on_activate
            .as_ref()
            .expect("last")
            .as_ref()();
        assert_eq!(*pages.lock().expect("page lock"), [10]);
    }

    #[test]
    fn enabled_limit_select_reports_open_and_parsed_page_size() {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sizes = Arc::new(Mutex::new(Vec::new()));
        let handlers = wired_handlers(
            Arc::clone(&pages),
            Arc::clone(&opens),
            Arc::clone(&sizes),
            false,
        );
        let closed = render(&numbered_spec(), &handlers);
        let trigger = limit_trigger(&closed);
        assert!(!trigger.interaction.disabled);
        trigger
            .interaction
            .on_activate
            .as_ref()
            .expect("open toggle")
            .as_ref()();
        assert_eq!(*opens.lock().expect("open lock"), [true]);

        let open_handlers = wired_handlers(
            Arc::clone(&pages),
            Arc::clone(&opens),
            Arc::clone(&sizes),
            true,
        );
        let open = render(&numbered_spec(), &open_handlers);
        let option = open
            .find(&|n| {
                n.a11y.role == Some(NodeRole::ListBoxOption)
                    && n.a11y.label.as_deref() == Some("25")
                    && n.interaction.on_activate.is_some()
            })
            .expect("page-size option 25");
        option.interaction.on_activate.as_ref().unwrap().as_ref()();
        assert_eq!(*sizes.lock().expect("size lock"), [25]);
    }

    #[test]
    fn two_paginations_do_not_share_select_runtime_ids() {
        let spec = numbered_spec();
        let left = render(
            &spec,
            &PaginationHandlers {
                instance_id: "pager-a".to_string(),
                ..wired_handlers(
                    Arc::new(Mutex::new(Vec::new())),
                    Arc::new(Mutex::new(Vec::new())),
                    Arc::new(Mutex::new(Vec::new())),
                    false,
                )
            },
        );
        let right = render(
            &spec,
            &PaginationHandlers {
                instance_id: "pager-b".to_string(),
                ..wired_handlers(
                    Arc::new(Mutex::new(Vec::new())),
                    Arc::new(Mutex::new(Vec::new())),
                    Arc::new(Mutex::new(Vec::new())),
                    false,
                )
            },
        );
        let mut tree = Node::container();
        tree = tree.child(left).child(right);
        let left_trigger = tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:pager-a:trigger"))
            .expect("left limit trigger");
        let right_trigger = tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:pager-b:trigger"))
            .expect("right limit trigger");
        assert_ne!(left_trigger.runtime_id, right_trigger.runtime_id);
    }

    #[test]
    fn public_pagination_path_does_not_share_select_runtime_ids() {
        let spec = numbered_spec();
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let left = pagination(&spec, &ctx, "pager-public-a", None);
        let right = pagination(&spec, &ctx, "pager-public-b", None);
        let mut tree = Node::container();
        tree = tree.child(left).child(right);
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:pager-public-a:trigger"))
            .is_some());
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:pager-public-b:trigger"))
            .is_some());
    }

    #[test]
    #[should_panic(
        expected = "PaginationHandlers requires a non-empty lifetime-stable instance_id"
    )]
    fn empty_instance_scope_is_rejected() {
        let _ = PaginationHandlers::new("");
    }

    #[test]
    #[should_panic(
        expected = "PaginationHandlers requires a non-empty lifetime-stable instance_id"
    )]
    fn public_pagination_path_rejects_empty_instance_scope() {
        let spec = numbered_spec();
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let _ = pagination(&spec, &ctx, "", None);
    }

    #[test]
    #[should_panic(
        expected = "PaginationHandlers requires a non-empty lifetime-stable instance_id"
    )]
    fn public_pagination_path_rejects_blank_instance_scope() {
        let spec = numbered_spec();
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let _ = pagination(&spec, &ctx, "  \t", None);
    }
}
