//! LogList — audit-variant render.
//!
//! Split out of `log_list/mod.rs` (god-file decomposition); unchanged.

use crate::presentation::rem_to_px;
use crate::primitives::{Callout, Icon, Pagination, Spinner};
use gpui::*;
use poodle_specs::LogFilterKind;

use super::*;

impl LogList {
    pub(super) fn render_audit(self, mut container: Stateful<Div>, p: AuditPalette) -> AnyElement {
        let theme = &self.theme;

        let audit: Vec<&LogEntry> = self.entries.iter().filter(|e| e.is_audit()).collect();

        // ── Audit toolbar: filter controls + clear (Svelte hasAuditToolbar) ──
        if self.spec.has_audit_toolbar() {
            container = container.child(self.render_audit_toolbar(&p));
        }

        // ── Content: loading / error / empty / entries ───────────
        // Status surfaces share the Svelte min-height-12rem centred layout.
        let status_surface = |child: AnyElement| {
            div()
                .flex()
                .items_center()
                .justify_center()
                .min_h(px(rem_to_px(12.0)))
                .px(px(p.pad_x))
                .py(px(rem_to_px(2.0)))
                .text_color(p.text_secondary)
                .text_center()
                .child(child)
        };

        // Loading surface (Svelte: shown when loading && no current entries).
        if self.spec.is_loading() {
            let spinner = Spinner::new(theme)
                .size(SpinnerSize::Md)
                .aria_label("Loading log entries");
            container = container.child(status_surface(
                div()
                    .flex()
                    .items_center()
                    .gap(p.inline_sm)
                    .child(spinner)
                    .child(div().child("Loading log entries\u{2026}"))
                    .into_any_element(),
            ));
            return container.into_any_element();
        }

        // Error surface (Svelte: status--error, role="alert" — compose Callout danger).
        if let Some(error) = self.spec.error.clone() {
            container = container.child(
                div()
                    .px(px(p.pad_x))
                    .py(px(rem_to_px(1.0)))
                    .child(
                        Callout::new(theme)
                            .tone(StatusTone::Danger)
                            .message(error),
                    ),
            );
            return container.into_any_element();
        }

        let mut list = div().flex().flex_col();

        if audit.is_empty() {
            // Empty state (Svelte status surface; uses spec emptyMessage).
            list = list.child(status_surface(
                div().child(self.spec.empty_message.clone()).into_any_element(),
            ));
        }

        for (idx, entry) in audit.iter().enumerate() {
            // Action-type icon: circular, status-tinted by action verb.
            let (icon_bg, icon_fg) = audit_icon_palette(entry, &p, theme);
            let action_icon = div()
                .w(px(rem_to_px(2.0))) // Svelte audit icon: 2rem
                .h(px(rem_to_px(2.0)))
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .rounded(p.radius_pill)
                .bg(icon_bg)
                .text_size(p.caption_size)
                .text_color(icon_fg)
                .font_weight(FontWeight::BOLD)
                .child(entry.audit_marker());

            // Main line: actor · action pill · resource.
            let actor_color = if entry.actor_href.is_some() {
                p.accent
            } else {
                p.text_primary
            };
            let mut main = div()
                .flex()
                .items_center()
                .flex_wrap()
                .gap(p.inline_sm)
                .child(
                    div()
                        .text_color(actor_color)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(
                            entry
                                .actor_name
                                .clone()
                                .unwrap_or_else(|| "System".to_string()),
                        ),
                );

            if let Some(ref action) = entry.action {
                // Action badge (Svelte Pill, badge appearance).
                main = main.child(
                    div()
                        .px(p.inline_sm)
                        .rounded(p.radius_pill)
                        .bg(Hsla {
                            a: p.accent.a * 0.16,
                            ..p.accent
                        })
                        .text_size(p.caption_size)
                        .text_color(p.accent)
                        .font_weight(FontWeight::MEDIUM)
                        .child(action.clone()),
                );
            }

            if let Some(ref label) = entry.resource_label {
                let resource_color = if entry.resource_href.is_some() {
                    p.accent
                } else {
                    p.text_secondary
                };
                main = main.child(div().text_color(resource_color).child(label.clone()));
            }

            // Timestamp pushed to the trailing edge.
            main = main.child(
                div()
                    .ml_auto()
                    .text_size(p.caption_size)
                    .text_color(p.text_secondary)
                    .child(entry.timestamp.clone()),
            );

            // Audit row: grid auto/1fr, gap 0.875rem, padding 1rem.
            let mut row = div()
                .flex()
                .items_start()
                .gap(px(rem_to_px(0.875)))
                .px(px(rem_to_px(1.0)))
                .py(px(rem_to_px(1.0)))
                .child(action_icon)
                .child(
                    div()
                        .flex_grow()
                        .flex()
                        .flex_col()
                        .gap(p.inline_sm)
                        .min_w(px(0.0))
                        .child(main),
                );

            if idx > 0 {
                row = row.border_t(p.border_w).border_color(p.border_subtle);
            }

            list = list.child(row);
        }

        container = container.child(list);

        // ── Pagination (Svelte: total !== undefined && total > pageSize) ──
        if self.spec.show_pagination() {
            container = container.child(self.render_audit_pagination(&p));
        }

        container.into_any_element()
    }

    /// Audit filter toolbar — renders each `LogFilter` as a labelled control
    /// (a token-resolved select/date affordance; the preview event loop drives
    /// real value changes via `on_filter_change`), plus a Clear action when any
    /// filter value is active. Matches Svelte `poodle-log-list__toolbar`.
    pub(super) fn render_audit_toolbar(&self, p: &AuditPalette) -> Div {
        let theme = &self.theme;

        let mut filters_row = div().flex().items_end().gap(p.inline_sm).flex_wrap();

        for filter in &self.spec.filters {
            let current = self.spec.filter_value(&filter.field);
            // Display text: for a select, resolve the option label for the
            // current value (falling back to the placeholder / "All"); for a
            // date, show the raw value or a hint.
            let display = match filter.kind {
                LogFilterKind::Select => {
                    if current.is_empty() {
                        filter.placeholder.clone().unwrap_or_else(|| "All".into())
                    } else {
                        filter
                            .options
                            .iter()
                            .find(|o| o.value == current)
                            .map(|o| o.label.clone())
                            .unwrap_or_else(|| current.to_string())
                    }
                }
                LogFilterKind::Date => {
                    if current.is_empty() {
                        "mm/dd/yyyy".to_string()
                    } else {
                        current.to_string()
                    }
                }
            };

            // Label above, control affordance below (Svelte Field wrapping).
            let label = div()
                .text_size(p.caption_size)
                .text_color(p.text_secondary)
                .child(filter.label.clone());

            let chevron = Icon::from_spec(
                IconSpec::new(match filter.kind {
                    LogFilterKind::Select => "chevron-down",
                    LogFilterKind::Date => "calendar",
                })
                .with_size(IconSize::Sm),
                theme,
            )
            .with_color(p.text_secondary);

            let mut control = div()
                .id(SharedString::from(format!("log-filter-{}", filter.field)))
                .flex()
                .items_center()
                .justify_between()
                .gap(p.inline_sm)
                .min_w(px(rem_to_px(10.0))) // Svelte filter min-width: 10rem
                .px(p.inline_sm)
                .py(px(rem_to_px(0.1875)))
                .border(p.border_w)
                .border_color(p.border_default)
                .rounded(p.radius_control)
                .text_size(p.label_token_size)
                .text_color(if current.is_empty() {
                    p.text_secondary
                } else {
                    p.text_primary
                })
                .child(div().child(display))
                .child(chevron);

            if let Some(handler) = self.on_filter_change.clone() {
                let field = filter.field.clone();
                control = control
                    .cursor_pointer()
                    .on_click(move |_, window, cx| handler(&field, window, cx));
            }

            filters_row = filters_row.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(rem_to_px(0.25)))
                    .child(label)
                    .child(control),
            );
        }

        // Clear action — only when a filter value is active (Svelte ghost Button).
        if self.spec.has_active_filters() {
            let clear_icon =
                Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)
                    .with_color(p.text_secondary);
            let mut clear = div()
                .id("log-list-clear-filters")
                .flex()
                .items_center()
                .gap(px(rem_to_px(0.25)))
                .px(p.inline_sm)
                .py(px(rem_to_px(0.1875)))
                .rounded(p.radius_control)
                .text_size(p.label_token_size)
                .text_color(p.text_secondary)
                .child(clear_icon)
                .child(div().child("Clear"));
            if let Some(handler) = self.on_filter_change.clone() {
                // Clearing reports an empty value on a reserved sentinel field.
                clear = clear
                    .cursor_pointer()
                    .on_click(move |_, window, cx| handler("__clear__", window, cx));
            }
            filters_row = filters_row.child(clear);
        }

        div()
            .flex()
            .items_end()
            .justify_between()
            .gap(p.inline_md)
            .flex_wrap()
            .px(px(p.pad_x))
            .py(p.toolbar_pad_y)
            .bg(p.toolbar_fill)
            .border_b(p.border_w)
            .border_color(p.border_subtle)
            .child(filters_row)
    }

    /// Pagination row — info text plus a composed [`Pagination`] control,
    /// matching the Svelte audit pagination footer.
    pub(super) fn render_audit_pagination(&self, p: &AuditPalette) -> Div {
        let theme = &self.theme;
        let total = self.spec.total.unwrap_or(0);
        let page = self.spec.page.max(1);
        let total_pages = self.spec.total_pages();

        // Range copy: "Showing X-Y of Z" (Svelte pagination-info).
        let first = (page - 1) * self.spec.page_size + 1;
        let last = (page * self.spec.page_size).min(total);
        let info = format!("Showing {first}-{last} of {total}");

        let pagination = Pagination::new(theme)
            .current_page(page)
            .total_pages(total_pages)
            .page_size(self.spec.page_size)
            .standalone(true)
            .aria_label("Log pagination");

        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(rem_to_px(1.0)))
            .px(px(p.pad_x))
            .py(p.toolbar_pad_y)
            .border_t(p.border_w)
            .border_color(p.border_subtle)
            .child(
                div()
                    .text_size(p.caption_size)
                    .text_color(p.text_secondary)
                    .child(info),
            )
            .child(pagination)
    }
}
