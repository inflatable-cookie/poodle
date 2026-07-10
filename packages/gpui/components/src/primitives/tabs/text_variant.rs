//! Tabs — underline (text) variant rendering.
//!
//! Split out of `tabs/mod.rs` (god-file decomposition). One `impl Tabs`
//! block per visual variant keeps each rendering path individually
//! navigable; behavior is unchanged.

use gpui::*;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

use super::{build_tab_label, Tabs};

impl Tabs {
    pub(super) fn render_underline(&self) -> Div {
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

}
