//! LogList — stream-variant render.
//!
//! Split out of `log_list/mod.rs` (god-file decomposition); unchanged.

use crate::presentation::rem_to_px;
use crate::primitives::Icon;
use crate::theme_ext::{color_mix, resolve_color};
use gpui::*;

use super::*;

impl LogList {
    pub(super) fn render_stream(self, container: Stateful<Div>, p: StreamPalette) -> AnyElement {
        let theme = &self.theme;

        // Active level filter (lowercase key), e.g. "error".
        let active = self.spec.filter_level.clone();

        // Level counts (Svelte stream chips show live per-level counts).
        let total = self.entries.len();
        let count_for = |lvl: LogLevel| -> usize {
            self.entries.iter().filter(|e| e.level == lvl).count()
        };

        // ── Toolbar: filter chips + search affordance ────────────
        let mut chips = div().flex().items_center().gap(p.inline_sm).flex_wrap();

        // "All" chip
        chips = chips.child(filter_chip(
            "log-list-chip-all",
            "All",
            total,
            active.is_none(),
            &p,
            self.on_filter_change.clone(),
            "",
        ));

        for lvl in [LogLevel::Info, LogLevel::Warn, LogLevel::Error] {
            let is_active = active.as_deref() == Some(lvl.key());
            chips = chips.child(filter_chip(
                &format!("log-list-chip-{}", lvl.key()),
                lvl.chip_label(),
                count_for(lvl),
                is_active,
                &p,
                self.on_filter_change.clone(),
                lvl.key(),
            ));
        }

        // Text-search affordance: a bound-looking input row. GPUI preview
        // can't host a live text field, so this is a click target that the
        // preview event loop drives via `on_search`.
        let search_icon = Icon::from_spec(IconSpec::new("search").with_size(IconSize::Sm), theme)
            .with_color(p.text_secondary);
        let mut search = div()
            .id("log-list-search")
            .flex_1()
            .flex()
            .items_center()
            .gap(p.inline_sm)
            .min_w(px(rem_to_px(10.0))) // Svelte search min-width: 10rem
            .px(p.inline_sm)
            .py(px(rem_to_px(0.1875))) // Svelte search padding-block: 0.1875rem
            .border(p.border_w)
            .border_color(p.border_default)
            .rounded(p.radius_control)
            .child(search_icon)
            .child(
                div()
                    .text_size(p.label_token_size)
                    .text_color(p.text_secondary)
                    .child("Filter logs\u{2026}"),
            );
        if let Some(handler) = self.on_search {
            search = search
                .cursor_pointer()
                .on_click(move |_, window, cx| handler("", window, cx));
        }

        let toolbar = div()
            .flex()
            .items_center()
            .justify_between()
            .gap(p.inline_md)
            .flex_wrap()
            .px(px(p.pad_x))
            .py(p.toolbar_pad_y)
            .bg(p.toolbar_fill)
            .border_b(p.border_w)
            .border_color(p.border_subtle)
            .child(chips)
            .child(search);

        // ── Entry rows ───────────────────────────────────────────
        let mut rows = div()
            .id("log-list-entries")
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .flex_grow();

        let visible: Vec<&LogEntry> = self
            .entries
            .iter()
            .filter(|e| active.as_deref().map_or(true, |a| e.level.key() == a))
            .collect();

        if visible.is_empty() {
            rows = rows.child(
                div()
                    .py(px(rem_to_px(1.5))) // Svelte empty padding: 1.5rem
                    .px(px(p.pad_x))
                    .text_size(p.label_size)
                    .text_color(p.text_secondary)
                    .text_center()
                    .child("No log entries"),
            );
        }

        for (idx, entry) in visible.iter().enumerate() {
            let badge_fill = resolve_color(theme, entry.level.badge_fill_token());

            let timestamp = div()
                .text_size(p.caption_size)
                .text_color(p.text_secondary)
                .child(entry.timestamp.clone());

            let level = div()
                .text_size(p.caption_size)
                .text_color(p.text_secondary)
                .font_weight(FontWeight::BOLD)
                .child(entry.level.label());

            let level_color = match entry.level {
                LogLevel::Warn => color_mix(badge_fill, p.text_primary, 0.84),
                LogLevel::Error => color_mix(badge_fill, p.text_primary, 0.84),
                _ => p.text_primary,
            };
            let message = div()
                .flex_grow()
                .text_size(p.label_size)
                .text_color(level_color)
                .child(entry.message.clone());

            // Svelte entry: grid auto/auto/1fr, gap 0.75rem, padding 0.5rem 0.875rem.
            let mut row = div()
                .flex()
                .items_start()
                .gap(p.entry_gap)
                .py(px(rem_to_px(0.5)))
                .px(px(rem_to_px(0.875)))
                .child(timestamp)
                .child(level)
                .child(message);

            // Top divider on all but the first row (Svelte :first-child border-top 0).
            if idx > 0 {
                row = row.border_t(p.border_w).border_color(Hsla {
                    a: p.border_subtle.a * 0.55,
                    ..p.border_subtle
                });
            }

            rows = rows.child(row);
        }

        for child in self.children {
            rows = rows.child(child);
        }

        let mut container = container.child(toolbar).child(rows);

        // Scroll-to-latest hint (Svelte: pill button, accent fill, on-accent text).
        if self.spec.auto_scroll {
            let arrow = Icon::from_spec(IconSpec::new("arrow-down").with_size(IconSize::Sm), theme)
                .with_color(p.text_inverse);
            container = container.child(
                div()
                    .flex()
                    .justify_center()
                    .py(px(rem_to_px(0.375)))
                    .border_t(p.border_w)
                    .border_color(p.border_subtle)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(p.inline_sm)
                            .px(px(rem_to_px(0.75)))
                            .py(px(rem_to_px(0.375)))
                            .rounded(p.radius_pill)
                            .bg(p.accent)
                            .child(arrow)
                            .child(
                                div()
                                    .text_size(p.caption_size)
                                    .text_color(p.text_inverse)
                                    .child("New entries"),
                            ),
                    ),
            );
        }

        container.into_any_element()
    }

}
