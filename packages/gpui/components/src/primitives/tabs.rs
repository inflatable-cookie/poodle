//! Tabs — real GPUI component backed by TabsSpec.
//!
//! Supports four variants matching the Svelte Tabs component:
//! - Underline (default): bottom border with accent indicator
//! - Card: bordered tabs
//! - Pill: rounded pill container with tinted active state
//! - Block: full-width tabs with separators and accent-tinted selected fill

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, IconSize, IconSpec, Orientation, TabActivationMode, TabDefinition, TabVariant,
    TabsSpec,
};

use super::icon::Icon;
use crate::presentation::{
    control_height_rem, control_space_x_rem, panel_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build the inner label composite for a tab: optional leading icon,
/// the label text, and an optional trailing count badge. Shared by all
/// four Tab variants (Underline / Card / Pill / Block) so they render
/// consistently.
fn build_tab_label(
    tab_def: &TabDefinition,
    theme: &GpuiThemeProvider,
    text_color: Hsla,
    icon_only: bool,
) -> AnyElement {
    let caption_size = resolve_px(theme, "typography.caption.size");

    // Vertical/icon-only mode: contract §8 hides `.poodle-tabs__label`. Show the
    // icon alone; fall back to the label when the tab has no icon.
    if icon_only {
        if let Some(ref icon_name) = tab_def.icon {
            return Icon::from_spec(
                IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                theme,
            )
            .with_color(text_color)
            .into_any_element();
        }
        return div().child(tab_def.label.clone()).into_any_element();
    }

    // Fast path: no decoration — just the label string.
    if tab_def.icon.is_none() && tab_def.count.is_none() {
        return div().child(tab_def.label.clone()).into_any_element();
    }

    // Svelte Tabs: gap between icon and label = space.inline.sm
    let mut inner = div().flex().items_center().gap(resolve_px(theme, "space.inline.sm"));

    if let Some(ref icon_name) = tab_def.icon {
        inner = inner.child(
            Icon::from_spec(
                IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                theme,
            )
            .with_color(text_color),
        );
    }

    inner = inner.child(div().child(tab_def.label.clone()));

    if let Some(count) = tab_def.count {
        // Mix text_color with the surface for the badge background so
        // the chip tone follows the tab's active/inactive state.
        let surface = resolve_color(theme, "color.background.surface");
        let badge_bg = crate::theme_ext::color_mix(text_color, surface, 0.14);
        inner = inner.child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .min_w(px(rem_to_px(1.125)))
                .px(px(rem_to_px(0.3125)))
                .rounded(px(rem_to_px(0.5625)))
                .bg(badge_bg)
                .text_size(caption_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_color)
                .child(format!("{count}")),
        );
    }

    inner.into_any_element()
}

/// A real GPUI tabs component backed by `TabsSpec`.
pub struct Tabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Content elements keyed by tab value.
    content: Vec<(String, AnyElement)>,
}

impl std::ops::Deref for Tabs {
    type Target = TabsSpec;
    fn deref(&self) -> &TabsSpec {
        &self.spec
    }
}

impl Tabs {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: TabsSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
            content: Vec::new(),
        }
    }

    pub fn from_spec(spec: TabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-tabs".to_string(),
            on_change: None,
            content: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tabs(mut self, v: Vec<TabDefinition>) -> Self {
        self.spec.tabs = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn variant(mut self, v: TabVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn activation_mode(mut self, v: TabActivationMode) -> Self {
        self.spec.activation_mode = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    /// Set the transient drag-source tab value (the tab being dragged).
    pub fn drag_value(mut self, v: Option<String>) -> Self {
        self.spec.drag_value = v;
        self
    }
    /// Set the transient drop-target tab value (the tab under the drag).
    pub fn drop_target_value(mut self, v: Option<String>) -> Self {
        self.spec.drop_target_value = v;
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Add content for a specific tab value.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

    fn render_underline(&self) -> Div {
        let theme = &self.theme;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        // Svelte: tabs-control-x is density-based (compact=0.5, default=0.75, comfortable=1.0rem)
        let inline_padding = px(rem_to_px(control_space_x_rem(self.spec.density)));
        let control_y = resolve_px(theme, "space.control.y");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let radius = resolve_radius(theme, "radius.control");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());

        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let vertical = self.spec.is_vertical();
        let full_width = self.spec.uses_full_width();

        // is_bordered (default true) controls whether the underline
        // indicator line renders under the whole tab list for the
        // Text/Underline variant. Setting it false gives a flush
        // layout with no baseline. Contract §8: horizontal carries a
        // bottom border, vertical shifts the rule to the right edge.
        let mut tab_row = if vertical {
            let mut row = div().flex().flex_col();
            if self.spec.is_bordered {
                row = row.border_r_1().border_color(border).pr(px(rem_to_px(0.5)));
            }
            row
        } else {
            let mut row = div().flex();
            if self.spec.is_bordered {
                row = row.border_b_1().border_color(border);
            }
            // Contract §8 Full-width: list becomes flex + width:100%.
            if full_width {
                row = row.w_full();
            }
            row
        };

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .focusable()
                .px(inline_padding)
                .py(control_y)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD);

            // Contract §8 Full-width: equal-width tabs with centered labels.
            if full_width {
                tab = tab.flex_grow().w_full().flex().justify_center();
            }

            if is_active {
                // Svelte text variant (selected): pill-shaped highlight with
                // accent 18% bg + text-primary. NO accent bottom border on the
                // tab — the indicator is the bg tint only.
                let active_bg = Hsla {
                    a: accent.a * 0.18,
                    ..accent
                };
                // Brand-raised treatment: gradient fill for active underline tab
                if theme.brand_raised && !is_disabled {
                    tab = tab
                        .text_color(text_primary)
                        .bg(crate::theme_ext::brand_raised_interactive_fill(active_bg))
                        .rounded(radius);
                } else {
                    tab = tab.text_color(text_primary).bg(active_bg).rounded(radius);
                }
            } else {
                tab = tab.text_color(text_secondary);
            }

            tab = tab.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                // Svelte text variant has no hover background.
                tab = tab.cursor_pointer();

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx =
                            if event.keystroke.key == "right" || event.keystroke.key == "down" {
                                Some((current_idx + 1) % nav_values.len())
                            } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                                Some(if current_idx == 0 {
                                    nav_values.len() - 1
                                } else {
                                    current_idx - 1
                                })
                            } else {
                                None
                            };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            // Contract §4 reorder drag states. drag-source: opacity 0.4.
            // drop-target: inset accent-base ring — GPUI box-shadow has no inset,
            // so the closest native equivalent is a 2px accent border.
            if self.spec.is_drag_value(&tab_def.value) {
                tab = tab.opacity(0.4);
            }
            if self.spec.is_drop_target(&tab_def.value) {
                tab = tab.border_2().border_color(accent).rounded(radius);
            }

            let label_color = if is_active {
                text_primary
            } else {
                text_secondary
            };
            // Contract §8 vertical: icon-only (label hidden).
            tab = tab.child(build_tab_label(tab_def, theme, label_color, vertical));
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_card(&self) -> Div {
        let theme = &self.theme;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        // Svelte: card tabs use panel-x padding (density-based, space.panel.x)
        let inline_padding = px(rem_to_px(panel_space_x_rem(self.spec.density)));
        let control_y = resolve_px(theme, "space.control.y");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border = resolve_color(theme, self.spec.list_border_token());
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let radius = resolve_radius(theme, "radius.control");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let vertical = self.spec.is_vertical();
        let full_width = self.spec.uses_full_width();

        // Card-row inter-item gap = list gap (space.inline.sm per contract §8 List).
        // Vertical stacks the cards into a column; full-width spans the row.
        let mut tab_row = if vertical {
            div().flex().flex_col().gap(resolve_px(theme, self.spec.list_gap_token()))
        } else {
            let mut row = div().flex().items_end().gap(resolve_px(theme, self.spec.list_gap_token()));
            if full_width {
                row = row.w_full();
            }
            row
        };

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(inline_padding)
                .py(control_y)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD)
                .border_1()
                .rounded(radius);

            // Contract §8 Full-width: equal-width cards with centered content.
            if full_width {
                tab = tab.flex_grow().w_full().flex().items_center().justify_center();
            }

            // Svelte: default = surface 92% fill, border-subtle 68% border
            // Selected = accent 32% + border-subtle border, text-primary color
            let card_default_bg = Hsla {
                a: surface_bg.a * 0.92,
                ..surface_bg
            };
            let card_default_border = Hsla {
                a: border.a * 0.68,
                ..border
            };
            let card_selected_border = {
                use crate::theme_ext::color_mix;
                color_mix(accent, border, 0.32)
            };

            if is_active {
                // Brand-raised treatment: gradient fill for active card tab
                if theme.brand_raised && !is_disabled {
                    tab = tab
                        .text_color(text_primary)
                        .bg(crate::theme_ext::brand_raised_interactive_fill(
                            card_default_bg,
                        ))
                        .border_color(card_selected_border);
                } else {
                    // Svelte: selected bg = color-mix(accent 14%, surface)
                    use crate::theme_ext::color_mix;
                    let card_selected_bg = color_mix(accent, surface_bg, 0.14);
                    tab = tab
                        .text_color(text_primary)
                        .bg(card_selected_bg)
                        .border_color(card_selected_border);
                }
            } else {
                tab = tab
                    .text_color(text_secondary)
                    .bg(card_default_bg)
                    .border_color(card_default_border);
            }

            tab = tab.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else if !is_active {
                // Svelte card variant has no hover background.
                tab = tab.cursor_pointer();
            }

            if !is_disabled {
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx =
                            if event.keystroke.key == "right" || event.keystroke.key == "down" {
                                Some((current_idx + 1) % nav_values.len())
                            } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                                Some(if current_idx == 0 {
                                    nav_values.len() - 1
                                } else {
                                    current_idx - 1
                                })
                            } else {
                                None
                            };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            // Contract §4 reorder drag states. drag-source: opacity 0.4.
            // drop-target: inset accent ring — GPUI box-shadow has no inset, so a
            // 2px accent border is the closest native equivalent (card already
            // carries radius-control).
            if self.spec.is_drag_value(&tab_def.value) {
                tab = tab.opacity(0.4);
            }
            if self.spec.is_drop_target(&tab_def.value) {
                tab = tab.border_2().border_color(accent);
            }

            let label_color = if is_active {
                text_primary
            } else {
                text_secondary
            };
            // Closable tab: label + close button in a flex row. Contract §8
            // vertical hides both the label (icon-only) and the close button.
            if tab_def.is_closable && !vertical {
                let icon_muted = resolve_color(&self.theme, "color.icon.muted");
                let gap_sm = resolve_px(theme, "space.inline.sm");
                tab = tab
                    .flex()
                    .items_center()
                    .gap(gap_sm)
                    .child(build_tab_label(tab_def, theme, label_color, false))
                    .child(
                        Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), &self.theme)
                            .with_color(icon_muted),
                    );
            } else {
                tab = tab.child(build_tab_label(tab_def, theme, label_color, vertical));
            }
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_block(&self) -> Div {
        let theme = &self.theme;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        // Svelte: --poodle-tabs-control-x is density-based (0.5/0.75/1.0rem)
        let inline_padding = px(rem_to_px(control_space_x_rem(self.spec.density)));
        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let panel_bg = resolve_color(theme, "color.background.panel");
        let elevated = resolve_color(theme, "color.background.elevated");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Svelte: color-mix(X N%, transparent) = alpha reduction
        let list_bg = Hsla { a: panel_bg.a * self.spec.block_list_bg_opacity(), ..panel_bg };
        let separator_color = Hsla { a: border.a * self.spec.block_separator_opacity(), ..border };
        let hover_bg = Hsla { a: elevated.a * self.spec.block_hover_bg_opacity(), ..elevated };

        // Selected item background: accent mixed into surface.
        let selected_bg =
            crate::theme_ext::color_mix(accent, surface_bg, self.spec.block_selected_accent_mix());
        let selected_hover_bg = crate::theme_ext::color_mix(
            accent,
            surface_bg,
            self.spec.block_selected_hover_accent_mix(),
        );

        let vertical = self.spec.is_vertical();

        // Contract §8: block list is a full-width flex row (horizontal) or a
        // stacked column (vertical) with the closing rule on the block-axis edge.
        let mut tab_row = if vertical {
            div()
                .flex()
                .flex_col()
                .bg(list_bg)
                .border_r_1()
                .border_color(border)
        } else {
            div()
                .flex()
                .w_full()
                .bg(list_bg)
                .border_b_1()
                .border_color(border)
        };

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .focusable()
                .flex()
                .items_center()
                .justify_center()
                .px(inline_padding)
                .h(control_height)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD);

            // Contract §8: vertical block tabs stack full-width in the column;
            // horizontal block items are content-sized (`flex: 0 0 auto`) unless
            // fullWidth flexes them to equal shares (`flex: 1 1 0`).
            if vertical {
                tab = tab.w_full();
            } else if self.spec.uses_full_width() {
                tab = tab.flex_grow().w_full().justify_center();
            }

            // Separator between sibling items: horizontal = left border,
            // vertical = top border (contract block vertical item table).
            if idx > 0 {
                tab = if vertical {
                    tab.border_t_1().border_color(separator_color)
                } else {
                    tab.border_l_1().border_color(separator_color)
                };
            }

            if is_active {
                let active_fill = if theme.brand_raised && !is_disabled {
                    // Reuse the selected_bg; brand-raised treatment is additive.
                    tab = tab.bg(crate::theme_ext::brand_raised_interactive_fill(selected_bg));
                    None
                } else {
                    Some(selected_bg)
                };
                if let Some(bg) = active_fill {
                    tab = tab.bg(bg);
                }
                tab = tab.text_color(text_primary);
            } else {
                tab = tab.text_color(text_secondary);
            }

            tab = tab.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab.cursor_pointer();

                // Hover: different treatment for active vs inactive items.
                if is_active {
                    tab = tab.hover(move |s| s.bg(selected_hover_bg));
                } else {
                    tab = tab.hover(move |s| s.bg(hover_bg));
                }

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx =
                            if event.keystroke.key == "right" || event.keystroke.key == "down" {
                                Some((current_idx + 1) % nav_values.len())
                            } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                                Some(if current_idx == 0 {
                                    nav_values.len() - 1
                                } else {
                                    current_idx - 1
                                })
                            } else {
                                None
                            };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            // Contract §4 reorder drag states. drag-source: opacity 0.4.
            // drop-target: inset accent ring + radius-control — GPUI box-shadow
            // has no inset, so a 2px accent border is the closest native fallback.
            if self.spec.is_drag_value(&tab_def.value) {
                tab = tab.opacity(0.4);
            }
            if self.spec.is_drop_target(&tab_def.value) {
                tab = tab
                    .border_2()
                    .border_color(accent)
                    .rounded(resolve_radius(theme, "radius.control"));
            }

            let label_color = if is_active {
                text_primary
            } else {
                text_secondary
            };
            // Contract §8 vertical: icon-only (label hidden).
            tab = tab.child(build_tab_label(tab_def, theme, label_color, vertical));
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_pill(&self) -> Div {
        let theme = &self.theme;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        // Svelte: --poodle-tabs-control-x is density-based (0.5/0.75/1.0rem)
        let control_x = px(rem_to_px(control_space_x_rem(self.spec.density)));
        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border_subtle = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let pill_radius = resolve_radius(theme, "radius.pill");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Container border: border-subtle with 68% opacity (alpha reduction only)
        let container_border =
            Hsla { a: border_subtle.a * self.spec.pill_border_opacity(), ..border_subtle };

        // Svelte: padding 0.1875rem, gap 0.125rem, border 2px
        let mut tabs = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(0.125)))
            .rounded(pill_radius)
            .border_2()
            .border_color(container_border)
            .p(px(rem_to_px(0.1875)));

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        // Svelte: min-height: calc(control-height - 0.5rem)
        let tab_height = control_height - px(rem_to_px(0.5));

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(control_x)
                .h(tab_height)
                .flex()
                .items_center()
                .rounded(pill_radius)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD);

            if is_active {
                let active_bg = accent.opacity(self.spec.pill_active_bg_opacity());
                // Brand-raised treatment: gradient fill for active pill tab
                if theme.brand_raised && !is_disabled {
                    tab = tab
                        .bg(crate::theme_ext::brand_raised_interactive_fill(active_bg))
                        .text_color(text_primary);
                } else {
                    tab = tab.bg(active_bg).text_color(text_primary);
                }
            } else {
                tab = tab.text_color(text_secondary);
            }

            tab = tab.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab.cursor_pointer();

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx =
                            if event.keystroke.key == "right" || event.keystroke.key == "down" {
                                Some((current_idx + 1) % nav_values.len())
                            } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                                Some(if current_idx == 0 {
                                    nav_values.len() - 1
                                } else {
                                    current_idx - 1
                                })
                            } else {
                                None
                            };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            // Contract §4 reorder drag states. drag-source: opacity 0.4.
            // drop-target: inset accent ring — GPUI box-shadow has no inset, so a
            // 2px accent border is the closest native fallback (pill keeps its
            // pill radius so the ring hugs the rounded tab).
            if self.spec.is_drag_value(&tab_def.value) {
                tab = tab.opacity(0.4);
            }
            if self.spec.is_drop_target(&tab_def.value) {
                tab = tab.border_2().border_color(accent);
            }

            let label_color = if is_active {
                text_primary
            } else {
                text_secondary
            };
            // Pill is always horizontal (vertical pill not in contract §8).
            tab = tab.child(build_tab_label(tab_def, theme, label_color, false));
            tabs = tabs.child(tab);
        }

        tabs
    }
}

impl IntoElement for Tabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let tab_row = match self.spec.variant {
            TabVariant::Pill => self.render_pill(),
            TabVariant::Card => self.render_card(),
            TabVariant::Block => self.render_block(),
            TabVariant::Underline => self.render_underline(),
        };

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let panel_padding = resolve_px(&self.theme, "space.panel.y");

        // Content pane
        let mut wrapper = div().flex().flex_col().child(tab_row);

        // Show content for active tab
        for (value, content) in self.content {
            if current_value.as_deref() == Some(&value) {
                wrapper = wrapper.child(div().p(panel_padding).child(content));
                break;
            }
        }

        wrapper.into_any_element()
    }
}
