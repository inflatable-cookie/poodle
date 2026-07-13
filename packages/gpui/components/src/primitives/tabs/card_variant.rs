//! Tabs — card variant rendering.
//!
//! Split out of `tabs/mod.rs` (god-file decomposition). One `impl Tabs`
//! block per visual variant keeps each rendering path individually
//! navigable; behavior is unchanged.

use gpui::*;
use poodle_specs::{
    IconSize, IconSpec,
};

use crate::primitives::icon::Icon;
use crate::presentation::{
    panel_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

use super::{build_tab_label, Tabs};

impl Tabs {
    pub(super) fn render_card(&self) -> Div {
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
                // Svelte: selected bg = color-mix(accent 14%, surface)
                use crate::theme_ext::color_mix;
                let card_selected_bg = color_mix(accent, surface_bg, 0.14);
                tab = tab
                    .text_color(text_primary)
                    .bg(card_selected_bg)
                    .border_color(card_selected_border);
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

}
