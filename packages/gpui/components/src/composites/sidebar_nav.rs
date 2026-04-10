//! SidebarNav — real GPUI component backed by SidebarNavSpec.
//!
//! Renders a vertical navigation with grouped, labelled items. Active item
//! gets an accent rail indicator and highlighted text. Size/density responsive.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{SidebarNavGroup, SidebarNavSpec};
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius, focus_ring_shadow};

/// A real GPUI sidebar navigation component backed by `SidebarNavSpec`.
pub struct SidebarNav {
    spec: SidebarNavSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SidebarNav {
    type Target = SidebarNavSpec;
    fn deref(&self) -> &SidebarNavSpec { &self.spec }
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
    pub fn groups(mut self, v: Vec<SidebarNavGroup>) -> Self { self.spec.groups = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for SidebarNav {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size-dependent layout (from Svelte CSS) ───────────────
        let (item_height, item_font, title_font) = match effective_size {
            ControlSize::Xs => (rem_to_px(1.375), rem_to_px(0.6875), rem_to_px(0.46875)),
            ControlSize::Sm => (rem_to_px(1.625), rem_to_px(0.75), rem_to_px(0.5)),
            ControlSize::Md => (rem_to_px(1.875), rem_to_px(0.8125), rem_to_px(0.5625)),
            ControlSize::Lg => (rem_to_px(2.125), rem_to_px(0.875), rem_to_px(0.59375)),
            ControlSize::Xl => (rem_to_px(2.375), rem_to_px(0.9375), rem_to_px(0.625)),
        };

        // ── Density-dependent spacing (from Svelte CSS) ───────────
        let (group_gap, item_pad_x, item_pad_y, title_spacing, title_gap) = match spec.density {
            ControlDensity::Compact => (
                rem_to_px(0.625),
                rem_to_px(0.5),
                rem_to_px(0.3125),
                0.2_f32,  // letter-spacing em
                rem_to_px(0.125),
            ),
            ControlDensity::Default => (
                rem_to_px(0.75),
                rem_to_px(0.75),
                rem_to_px(0.375),
                0.18_f32,
                rem_to_px(0.1875),
            ),
            ControlDensity::Comfortable => (
                rem_to_px(0.875),
                rem_to_px(0.875),
                rem_to_px(0.4375),
                0.16_f32,
                rem_to_px(0.25),
            ),
        };
        let _ = title_spacing; // GPUI does not have letter-spacing; used in Svelte only

        // ── Token resolution ──────────────────────────────────────
        let item_color = resolve_color(theme, spec.item_color_token());
        let item_active_color = resolve_color(theme, spec.item_active_color_token());
        let group_title_color = resolve_color(theme, spec.group_title_color_token());
        let separator_color = resolve_color(theme, spec.separator_color_token());
        let active_indicator_color = resolve_color(theme, spec.active_indicator_color_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let control_radius = resolve_radius(theme, "radius.control");
        let item_radius = control_radius - px(2.0); // contract: calc(radius-control - 0.125rem)

        // Active item background: accent at 10% opacity
        let active_bg = color_mix(active_indicator_color, gpui::transparent_black(), 0.10);
        // Hover background: elevated at 60%
        let hover_bg = color_mix(elevated_bg, gpui::transparent_black(), 0.60);

        let visible_groups = spec.visible_groups();
        let multi_group = visible_groups.len() > 1;

        // ── Root nav container ────────────────────────────────────
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
            .py(px(panel_y));

        for (group_idx, group) in visible_groups.iter().enumerate() {
            let mut section = div().flex().flex_col().gap(px(5.0)).min_w(px(0.0));

            // Separator between groups
            if multi_group && group_idx > 0 {
                section = section
                    .mt(px(2.0))
                    .pt(px(group_gap - 2.0))
                    .border_t_1()
                    .border_color(color_mix(separator_color, gpui::transparent_black(), 0.54));
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

            // Items list
            let mut list = div().flex().flex_col().gap(px(2.0)).min_w(px(0.0));

            for item in &group.items {
                let is_active = spec.is_active(&item.value);
                let item_id = SharedString::from(format!("sidebar-nav-{}", item.value));

                let mut el = div()
                    .id(item_id)
                    .focusable()
                    .w_full()
                    .min_w(px(0.0))
                    .min_h(px(item_height))
                    .px(px(item_pad_x))
                    .py(px(item_pad_y))
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
                    el = el
                        .text_color(item_active_color)
                        .font_weight(FontWeight::SEMIBOLD)
                        .bg(active_bg);
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

                // Active indicator rail (left accent bar)
                if is_active {
                    let indicator = active_indicator_color;
                    el = el.relative().child(
                        div()
                            .absolute()
                            .left(px(2.0))
                            .top(px(4.0))
                            .bottom(px(4.0))
                            .w(px(2.0))
                            .rounded(px(999.0))
                            .bg(indicator),
                    );
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
