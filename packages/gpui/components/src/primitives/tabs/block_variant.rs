//! Tabs — block variant rendering.
//!
//! Split out of `tabs/mod.rs` (god-file decomposition). One `impl Tabs`
//! block per visual variant keeps each rendering path individually
//! navigable; behavior is unchanged.

use gpui::*;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

use super::{build_tab_label, Tabs};

impl Tabs {
    pub(super) fn render_block(&self) -> Div {
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

}
