//! LogList — Jetstream log list backed by LogListSpec.
//!
//! Two surfaces, matching the contract/Svelte `variant` split:
//! - **stream**: level-filter chips + text-filter affordance + entry area.
//! - **audit**: filter toolbar (from `LogListSpec::filters`/`filter_values`),
//!   loading / error / empty status surfaces, and pagination (page/page_size/
//!   total → composed `js_pagination`).
//!
//! Build-unverified: the sibling `jetstream-renderer` is down (external
//! `encode.rs` break), so `poodle-jetstream-components` cannot compile here.
//! This was authored by reading the runtime builder + sibling component APIs.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    CallOutSpec, LogFilterKind, LogListSpec, PaginationSpec, SpinnerSize, SpinnerSpec, StatusTone,
};

use crate::callout::js_callout;
use crate::pagination_comp::js_pagination;
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// LogList — filterable log rows.
///
/// `on_clear_filters` is the one pointer-reachable event: the refresh, export
/// and paging affordances are not drawn by this component, and the filters
/// themselves are typed or open Select panels.
pub struct LogList {
    spec: LogListSpec,
    theme: JetstreamThemeProvider,
    on_clear_filters: Option<crate::element::ActionHandler>,
}

impl LogList {
    pub fn from_spec(spec: LogListSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_clear_filters: None,
        }
    }

    /// Fires when the clear affordance is pressed. It renders only while a
    /// filter is active, so there is nothing to fire otherwise.
    pub fn on_clear_filters(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_clear_filters = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for LogList {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_clear_filters)
    }
}

pub fn js_log_list(spec: &LogListSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &LogListSpec,
    theme: &JetstreamThemeProvider,
    on_clear_filters: Option<crate::element::ActionHandler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let label_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let entry_gap = resolve_px(theme, spec.entry_gap_token());
    let caption_size = resolve_px(theme, "typography.caption.size");
    let label_token_size = resolve_px(theme, "typography.label.size");
    let radius_control = resolve_radius(theme, "radius.control");

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "color.border.subtle");
    let border_default = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    // Token colors for log levels
    let info_color = resolve_color(theme, "color.accent.base");
    let warn_color = resolve_color(theme, "color.status.warning");
    let error_color = resolve_color(theme, "color.status.danger");

    // Audit mode is entered when the spec carries audit-only state, matching
    // the GPUI/Svelte audit branch which owns those surfaces.
    let is_audit =
        spec.loading || spec.error.is_some() || spec.has_audit_toolbar() || spec.show_pagination();

    // Root
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y);

    if is_audit {
        // ── Audit toolbar: filter controls (from spec) ───────────
        if spec.has_audit_toolbar() {
            let mut toolbar = ui_element::div()
                .flex_row()
                .items_end()
                .gap(rem_to_px(0.5))
                .flex_wrap()
                .pb(rem_to_px(0.5));

            for filter in &spec.filters {
                let current = spec.filter_value(&filter.field);
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

                let control = ui_element::div()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(rem_to_px(0.5))
                    .min_w(rem_to_px(10.0))
                    .px(rem_to_px(0.5))
                    .py(rem_to_px(0.1875))
                    .border(1.0)
                    .border_color(border_default)
                    .rounded(radius_control)
                    .child(
                        ui_element::label(&display)
                            .text_color(if current.is_empty() {
                                text_secondary
                            } else {
                                text_primary
                            })
                            .text_size(label_token_size),
                    );

                let field = ui_element::div().flex_col().gap(rem_to_px(0.25)).child(
                    ui_element::label(&filter.label)
                        .text_color(text_secondary)
                        .text_size(caption_size),
                );
                toolbar = toolbar.child(field.child(control));
            }

            // Clear affordance — only when a value is active.
            if spec.has_active_filters() {
                let mut clear = ui_element::button("Clear")
                    .text_color(text_secondary)
                    .text_size(label_token_size)
                    .focusable();

                if let Some(handler) = &on_clear_filters {
                    let handler = std::sync::Arc::clone(handler);
                    clear = clear.cursor_pointer().on_click(move |_event| handler());
                }

                toolbar = toolbar.child(clear);
            }

            el = el.child(toolbar);
        }

        // ── Status surfaces ──────────────────────────────────────
        if spec.is_loading() {
            // Loading: composed spinner + label, centred.
            el = el.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap(rem_to_px(0.5))
                    .py(rem_to_px(2.0))
                    .child(js_spinner(
                        &SpinnerSpec::new().with_size(SpinnerSize::Md),
                        theme,
                    ))
                    .child(
                        ui_element::label("Loading log entries\u{2026}")
                            .text_color(text_secondary)
                            .text_size(label_token_size),
                    ),
            );
            return el;
        }

        if let Some(error) = &spec.error {
            // Error: composed danger Callout (Svelte status--error / role=alert).
            el = el.child(
                ui_element::div().py(rem_to_px(1.0)).child(js_callout(
                    &CallOutSpec::new()
                        .with_tone(StatusTone::Danger)
                        .with_content(error.clone()),
                    theme,
                )),
            );
            return el;
        }

        // Empty audit surface (spec carries no entry payload yet).
        el = el.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_center()
                .py(rem_to_px(2.0))
                .child(
                    ui_element::label(&spec.empty_message)
                        .text_color(text_secondary)
                        .text_size(label_token_size),
                ),
        );

        // ── Pagination ───────────────────────────────────────────
        if spec.show_pagination() {
            let total = spec.total.unwrap_or(0);
            let page = spec.page.max(1);
            let first = (page - 1) * spec.page_size + 1;
            let last = (page * spec.page_size).min(total);
            let info = format!("Showing {first}-{last} of {total}");

            let mut footer = ui_element::div()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(rem_to_px(1.0))
                .pt(rem_to_px(0.75))
                .child(
                    ui_element::label(&info)
                        .text_color(text_secondary)
                        .text_size(caption_size),
                );
            footer = footer.child(js_pagination(
                &PaginationSpec::new()
                    .with_current_page(page)
                    .with_total_pages(spec.total_pages())
                    .with_page_size(spec.page_size)
                    .with_standalone(true)
                    .with_aria_label("Log pagination"),
                theme,
            ));
            el = el.child(footer);
        }

        return el;
    }

    // ── Stream mode ──────────────────────────────────────────────
    // Toolbar: level filter pills + text-filter affordance.
    let mut toolbar = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .pb(rem_to_px(0.5));

    // Level filter chips
    for level in &["info", "warn", "error"] {
        let is_active = spec.filter_level.as_deref() == Some(level);
        let chip_color = match *level {
            "info" => info_color,
            "warn" => warn_color,
            "error" => error_color,
            _ => text_secondary,
        };
        toolbar = toolbar.child(
            ui_element::button(*level)
                .text_color(if is_active {
                    chip_color
                } else {
                    text_secondary
                })
                .text_size(label_font)
                .text_weight(if is_active { 600 } else { 400 })
                .focusable(),
        );
    }

    toolbar = toolbar.child(ui_element::div().grow());

    // Text-filter affordance (Svelte stream search input). Shows the current
    // filter_text when set, else a placeholder. Live editing lives in the
    // preview event loop.
    let filter_display = if spec.filter_text.is_empty() {
        "Filter logs\u{2026}".to_string()
    } else {
        spec.filter_text.clone()
    };
    toolbar = toolbar.child(
        ui_element::div()
            .px(rem_to_px(0.5))
            .py(rem_to_px(0.1875))
            .border(1.0)
            .border_color(border_default)
            .rounded(radius_control)
            .child(
                ui_element::label(&filter_display)
                    .text_color(text_secondary)
                    .text_size(label_token_size),
            ),
    );
    el = el.child(toolbar);

    // Entry area. The spec carries no entry payload (only `entry_count`), so we
    // surface a count rather than fabricate log lines (per CLAUDE.md "No
    // Mockups" — a real count beats fake messages).
    let mut entries_area = ui_element::div()
        .flex_col()
        .gap(entry_gap)
        .grow()
        .overflow_hidden();

    if spec.entry_count == 0 {
        entries_area = entries_area.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_center()
                .py(rem_to_px(1.5))
                .child(
                    ui_element::label("No log entries")
                        .text_color(text_secondary)
                        .text_size(label_token_size),
                ),
        );
    } else {
        let shown = spec.entry_count.min(spec.max_entries);
        entries_area = entries_area.child(
            ui_element::label(&format!("{shown} entries"))
                .text_color(text_secondary)
                .text_size(label_token_size),
        );
    }

    el = el.child(entries_area);

    // Scroll-to-latest hint.
    if spec.auto_scroll {
        el = el.child(
            ui_element::label("New entries")
                .text_color(text_secondary)
                .text_size(label_font),
        );
    }

    el.aria_role(jetstream_ui::accesskit::Role::Log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{LogFilter, LogListSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn the_clear_affordance_reports_a_clear() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        // The Clear affordance lives in the audit toolbar, which needs a
        // filter definition and an active value.
        let el = LogList::from_spec(
            LogListSpec::new()
                .with_filter(LogFilter::select("level", "Level"))
                .with_filter_value("level", "error"),
            &theme(),
        )
        .on_clear_filters(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        crate::element::click_probe::click_text(&el, 800.0, 480.0, "Clear");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_clear_filters fired exactly once"
        );
    }
}
