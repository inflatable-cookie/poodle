//! Tabs — pill variant rendering.
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
    pub(super) fn render_pill(&self) -> Div {
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
                tab = tab.bg(active_bg).text_color(text_primary);
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
