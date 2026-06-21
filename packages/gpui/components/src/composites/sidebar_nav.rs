//! SidebarNav — real GPUI component backed by SidebarNavSpec.
//!
//! Renders a vertical navigation with grouped, labelled items. Active item
//! gets an accent rail indicator and highlighted text. Size/density responsive.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_specs::{SidebarNavGroup, SidebarNavSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    focus_ring_shadow, resolve_color, resolve_opacity, resolve_radius,
};

/// A real GPUI sidebar navigation component backed by `SidebarNavSpec`.
pub struct SidebarNav {
    spec: SidebarNavSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SidebarNav {
    type Target = SidebarNavSpec;
    fn deref(&self) -> &SidebarNavSpec {
        &self.spec
    }
}

impl SidebarNav {
    pub fn new(groups: Vec<SidebarNavGroup>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SidebarNavSpec::new(groups),
            theme: theme.clone(),
            on_select: None,
        }
    }

    pub fn from_spec(spec: SidebarNavSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn groups(mut self, v: Vec<SidebarNavGroup>) -> Self {
        self.spec.groups = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for SidebarNav {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Active-state alpha factors (contract color-mix percentages) ───
        const ACTIVE_BG_ALPHA: f32 = 0.10; // accent-base @ 10%
        const ACTIVE_RING_ALPHA: f32 = 0.20; // inset ring accent-base @ 20%
        const HOVER_BG_ALPHA: f32 = 0.60; // elevated @ 60%
        const SEPARATOR_ALPHA: f32 = 0.54; // border-subtle @ 54%

        // ── Size / density geometry (contract §8 tables, token-resolved rem) ─
        let item_height = rem_to_px(spec.item_height_rem());
        let item_font = rem_to_px(spec.item_font_rem());
        let title_font = rem_to_px(spec.title_font_rem());

        let group_gap = rem_to_px(spec.group_gap_rem());
        let item_pad_x = rem_to_px(spec.item_pad_inline_rem());
        let item_pad_y = rem_to_px(spec.item_pad_block_rem());
        let title_gap = rem_to_px(spec.title_gap_rem());
        // Contract group internal gap `0.3125rem`; list gap `0.125rem`;
        // separator margin-top `0.125rem`; rail width `0.1875rem` (3px).
        let group_internal_gap = rem_to_px(0.3125);
        let list_gap = rem_to_px(0.125);
        let separator_mt = rem_to_px(0.125);
        let item_radius_offset = rem_to_px(0.125); // calc(radius-control - 0.125rem)
        // Root horizontal padding (contract `0.375rem`).
        let nav_pad_x = rem_to_px(0.375);

        // ── Token resolution ──────────────────────────────────────
        let item_color = resolve_color(theme, spec.item_color_token());
        let item_active_color = resolve_color(theme, spec.item_active_color_token());
        let group_title_color = resolve_color(theme, spec.group_title_color_token());
        let separator_color = resolve_color(theme, spec.separator_color_token());
        let active_indicator_color = resolve_color(theme, spec.active_indicator_color_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());
        let hover_fill = resolve_color(theme, spec.hover_fill_token());
        let active_fill = resolve_color(theme, spec.active_fill_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let control_radius = resolve_radius(theme, "radius.control");
        let item_radius = (control_radius - px(item_radius_offset)).max(px(0.0));

        // Active item background: accent at 10%; inset ring accent at 20%.
        let active_bg = Hsla { a: active_fill.a * ACTIVE_BG_ALPHA, ..active_fill };
        let active_ring = Hsla { a: active_fill.a * ACTIVE_RING_ALPHA, ..active_fill };
        // Hover background: elevated at 60%.
        let hover_bg = Hsla { a: hover_fill.a * HOVER_BG_ALPHA, ..hover_fill };

        let visible_groups = spec.visible_groups();
        let multi_group = visible_groups.len() > 1;

        // ── Root nav container ────────────────────────────────────
        // Contract root padding: `var(--space-panel-y) 0.375rem`.
        // space-panel-y is density-driven (compact 0.5 / default 0.75 / comfortable 1rem).
        let panel_y = rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        });
        let mut nav = div()
            .flex()
            .flex_col()
            .gap(px(group_gap))
            .min_w(px(0.0))
            .py(px(panel_y))
            .px(px(nav_pad_x));

        for (group_idx, group) in visible_groups.iter().enumerate() {
            // Contract group section internal gap = 0.3125rem (title → list).
            let mut section = div()
                .flex()
                .flex_col()
                .gap(px(group_internal_gap))
                .min_w(px(0.0));

            // Separator between groups: top border + top padding.
            if multi_group && group_idx > 0 {
                section = section
                    .mt(px(separator_mt))
                    .pt(px(group_gap - separator_mt))
                    .border_t_1()
                    .border_color(Hsla {
                        a: separator_color.a * SEPARATOR_ALPHA,
                        ..separator_color
                    });
            }

            // Group title
            if let Some(ref label) = group.label {
                section = section.child(
                    div()
                        .px(px(item_pad_x))
                        .pb(px(title_gap))
                        .text_size(px(title_font))
                        .font_weight(FontWeight::BOLD)
                        .text_color(group_title_color)
                        .line_height(relative(1.2))
                        .child(label.to_uppercase()),
                );
            }

            // Items list (contract list gap 0.125rem)
            let mut list = div().flex().flex_col().gap(px(list_gap)).min_w(px(0.0));

            for item in &group.items {
                let is_active = spec.is_active(&item.value);
                let item_id = SharedString::from(format!("sidebar-nav-{}", item.value));

                // Reserve a 3px (0.1875rem) left border on every item so active
                // ↔ inactive does not shift horizontally; transparent until active.
                // Contract: active indicator is a LEFT BORDER on the item itself.
                let mut el = div()
                    .id(item_id)
                    .focusable()
                    .relative()
                    .w_full()
                    .min_w(px(0.0))
                    .min_h(px(item_height))
                    .px(px(item_pad_x))
                    .py(px(item_pad_y))
                    .border_l_3()
                    .border_color(gpui::transparent_black())
                    .rounded(item_radius)
                    .bg(gpui::transparent_black())
                    .text_color(item_color)
                    .text_size(px(item_font))
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(relative(1.3))
                    .flex()
                    .items_center()
                    .cursor_pointer();

                // Focus ring
                let fr = focus_ring_color;
                el = el.focus(move |s| s.shadow(focus_ring_shadow(fr)));

                if is_active {
                    // Active: accent left rail + bg fill + bolder weight, plus an
                    // inset accent@20% ring emulated as a full-bleed bordered overlay
                    // (GPUI BoxShadow has no inset variant).
                    el = el
                        .text_color(item_active_color)
                        .font_weight(FontWeight::SEMIBOLD)
                        .bg(active_bg)
                        .border_color(active_indicator_color)
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .border_1()
                                .border_color(active_ring)
                                .rounded(item_radius),
                        );
                }

                if item.is_disabled {
                    el = el
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
                } else {
                    let h_bg = hover_bg;
                    let active_c = item_active_color;
                    el = el.hover(move |s| s.text_color(active_c).bg(h_bg));

                    // Click handler
                    if let Some(ref handler) = self.on_select {
                        let handler = Rc::clone(handler);
                        let val = item.value.clone();
                        el = el.on_click(move |_event, window, cx| {
                            handler(&val, window, cx);
                        });
                    }
                }

                el = el.child(item.label.clone());

                list = list.child(el);
            }

            section = section.child(list);
            nav = nav.child(section);
        }

        nav.into_any_element()
    }
}
