//! LogList — filterable log rows.
//!
//! Contract: `docs/contracts/components/log-list.md`
//! Ported from: `packages/jetstream/components/src/log_list.rs`.
//!
//! Two surfaces, matching the contract/Svelte `variant` split:
//! - **stream**: level-filter chips + text-filter affordance + entry area.
//! - **audit**: filter toolbar, loading / error / empty status surfaces, and
//!   pagination (page/page_size/total → composed `pagination`).
//!
//! `on_clear_filters` is the one pointer-reachable event: the refresh, export
//! and paging affordances are not drawn by this component, and the filters
//! themselves are typed or open Select panels.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{
    CallOutSpec, LogFilterKind, LogLevel, LogListSpec, PaginationSpec, SpinnerSize, SpinnerSpec,
    StatusTone,
};

use crate::callout::callout;
use crate::color::{mix_srgb, with_alpha};
use crate::pagination::pagination;
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::spinner::spinner;

pub fn log_list(
    spec: &LogListSpec,
    theme: &dyn ThemeProvider,
    on_clear_filters: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let label_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let entry_gap = theme.resolve_space(spec.entry_gap_token());
    let caption_size = theme.resolve_space("typography.caption.size");
    let label_token_size = theme.resolve_space("typography.label.size");
    let radius_control = theme.resolve_radius("radius.control");

    let fill = theme.resolve_color(spec.fill_token());
    let border = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");
    let radius = theme.resolve_radius("radius.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Token colors for log levels
    let info_color = theme.resolve_color("color.accent.base");
    let warn_color = theme.resolve_color("color.status.warning");
    let error_color = theme.resolve_color("color.status.danger");

    // Audit mode is entered when the spec carries audit-only state, matching
    // the GPUI/Svelte audit branch which owns those surfaces.
    let is_audit =
        spec.loading || spec.error.is_some() || spec.has_audit_toolbar() || spec.show_pagination();

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };
    let text = |content: String, color, size| -> Node {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(size);
        t
    };

    // Root
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Column;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }
    all_radius(&mut el, radius);
    el.a11y.role = Some(NodeRole::Log);
    let mut el = el;

    if is_audit {
        // ── Audit toolbar: filter controls (from spec) ───────────
        if spec.has_audit_toolbar() {
            let mut toolbar = Node::container();
            {
                let s = &mut toolbar.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
                s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                s.flex_wrap = true;
                s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.5);
            }

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

                let mut control = Node::container();
                {
                    let s = &mut control.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                    s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                    s.min_width = Some(rem_to_px(10.0));
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = rem_to_px(0.5);
                    pad.right = rem_to_px(0.5);
                    pad.top = rem_to_px(0.1875);
                    pad.bottom = rem_to_px(0.1875);
                    s.descriptor.border.width = 1.0;
                    s.descriptor.border.color = border_default;
                }
                all_radius(&mut control, radius_control);
                let control = control.child(text(
                    display,
                    if current.is_empty() {
                        text_secondary
                    } else {
                        text_primary
                    },
                    label_token_size,
                ));

                let mut field = Node::container();
                field.style.descriptor.layout.direction = LayoutDirection::Column;
                field.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
                let field = field.child(text(filter.label.clone(), text_secondary, caption_size));
                toolbar = toolbar.child(field.child(control));
            }

            // Clear affordance — only when a value is active.
            if spec.has_active_filters() {
                let mut clear = Node::button("Clear");
                clear.style.descriptor.text_color = Some(text_secondary);
                clear.style.text_size = Some(label_token_size);
                clear.interaction.focusable = true;
                if let Some(handler) = &on_clear_filters {
                    let handler = Arc::clone(handler);
                    clear.style.descriptor.cursor = CursorHint::Pointer;
                    clear.interaction.on_activate = Some(Arc::new(move || handler()));
                }
                toolbar = toolbar.child(clear);
            }

            el = el.child(toolbar);
        }

        // ── Status surfaces ──────────────────────────────────────
        if spec.is_loading() {
            // Loading: composed spinner + label, centred.
            let mut state = Node::container();
            {
                let s = &mut state.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = rem_to_px(2.0);
                pad.bottom = rem_to_px(2.0);
            }
            return el.child(
                state
                    .child(spinner(
                        &SpinnerSpec::new().with_size(SpinnerSize::Md),
                        theme,
                    ))
                    .child(text(
                        "Loading log entries\u{2026}".to_string(),
                        text_secondary,
                        label_token_size,
                    )),
            );
        }

        if let Some(error) = &spec.error {
            // Error: composed danger Callout (Svelte status--error / role=alert).
            let mut frame = Node::container();
            // Explicit Row (see switch.rs).
            frame.style.descriptor.layout.direction = LayoutDirection::Row;
            frame.style.descriptor.layout.spacing.padding.top = rem_to_px(1.0);
            frame.style.descriptor.layout.spacing.padding.bottom = rem_to_px(1.0);
            return el.child(
                frame.child(callout(
                    &CallOutSpec::new()
                        .with_tone(StatusTone::Danger)
                        .with_content(error.clone()),
                    theme,
                    None,
                )),
            );
        }

        // Empty audit surface (spec carries no entry payload yet).
        let mut empty = Node::container();
        {
            let s = &mut empty.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = rem_to_px(2.0);
            pad.bottom = rem_to_px(2.0);
        }
        el = el.child(empty.child(text(
            spec.empty_message.clone(),
            text_secondary,
            label_token_size,
        )));

        // ── Pagination ───────────────────────────────────────────
        if spec.show_pagination() {
            let total = spec.total.unwrap_or(0);
            let page = spec.page.max(1);
            let first = (page - 1) * spec.page_size + 1;
            let last = (page * spec.page_size).min(total);
            let info = format!("Showing {first}-{last} of {total}");

            let mut footer = Node::container();
            {
                let s = &mut footer.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                s.descriptor.layout.spacing.gap = rem_to_px(1.0);
                s.descriptor.layout.spacing.padding.top = rem_to_px(0.75);
            }
            let footer = footer
                .child(text(info, text_secondary, caption_size))
                .child(pagination(
                    &PaginationSpec::new()
                        .with_current_page(page)
                        .with_total_pages(spec.total_pages())
                        .with_page_size(spec.page_size)
                        .with_standalone(true)
                        .with_aria_label("Log pagination"),
                    theme,
                    None,
                ));
            el = el.child(footer);
        }

        return el;
    }

    // ── Stream mode ──────────────────────────────────────────────
    // Toolbar: level filter pills + text-filter affordance.
    let mut toolbar = Node::container();
    {
        let s = &mut toolbar.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.5);
    }

    // Level filter chips
    for level in &["info", "warn", "error"] {
        let is_active = spec.filter_level.as_deref() == Some(level);
        let chip_color = match *level {
            "info" => info_color,
            "warn" => warn_color,
            "error" => error_color,
            _ => text_secondary,
        };
        let mut chip = Node::button(*level);
        chip.style.descriptor.text_color = Some(if is_active {
            chip_color
        } else {
            text_secondary
        });
        chip.style.text_size = Some(label_font);
        chip.style.text_weight = Some(if is_active { 600 } else { 400 });
        chip.interaction.focusable = true;
        toolbar = toolbar.child(chip);
    }

    let mut spacer = Node::container();
    // Explicit Row (see switch.rs).
    spacer.style.descriptor.layout.direction = LayoutDirection::Row;
    spacer.style.descriptor.layout.width = LayoutSizing::Grow;
    toolbar = toolbar.child(spacer);

    // Text-filter affordance (Svelte stream search input). Shows the current
    // filter_text when set, else a placeholder. Live editing is host-owned.
    let filter_display = if spec.filter_text.is_empty() {
        "Filter logs\u{2026}".to_string()
    } else {
        spec.filter_text.clone()
    };
    let mut filter_box = Node::container();
    {
        let s = &mut filter_box.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = rem_to_px(0.5);
        pad.right = rem_to_px(0.5);
        pad.top = rem_to_px(0.1875);
        pad.bottom = rem_to_px(0.1875);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_default;
    }
    all_radius(&mut filter_box, radius_control);
    toolbar =
        toolbar.child(filter_box.child(text(filter_display, text_secondary, label_token_size)));
    el = el.child(toolbar);

    // Entry area — stream rows, filtered and capped by the spec.
    let mut entries_area = Node::container();
    {
        let s = &mut entries_area.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = entry_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }

    let rows = spec.stream_entries();
    if rows.is_empty() {
        let mut empty = Node::container();
        {
            let s = &mut empty.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = rem_to_px(1.5);
            pad.bottom = rem_to_px(1.5);
        }
        entries_area = entries_area.child(empty.child(text(
            "No log entries".to_string(),
            text_secondary,
            label_token_size,
        )));
    } else {
        // Contract `.poodle-log-list__entry`: a mono row of
        // [timestamp | level | message], tinted by level, capped at
        // `maxEntries` and separated by a half-strength subtle rule.
        let entry_font = rem_to_px(0.8125);
        let row_gap = rem_to_px(0.75);
        let row_pad_y = rem_to_px(0.5);
        let row_pad_x = rem_to_px(0.875);
        let rule = with_alpha(border, border.3 * 0.55);
        let level_tint = |level: LogLevel| match level {
            LogLevel::Info => text_primary,
            LogLevel::Warn => mix_srgb(warn_color, text_primary, 0.84),
            LogLevel::Error => mix_srgb(error_color, text_primary, 0.84),
        };

        for (i, entry) in rows.iter().take(spec.max_entries).enumerate() {
            let mut row = Node::container();
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
                s.descriptor.layout.spacing.gap = row_gap;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = row_pad_y;
                pad.bottom = row_pad_y;
                pad.left = row_pad_x;
                pad.right = row_pad_x;
                s.font_family = Some(FontFamily::Mono);
                s.text_size = Some(entry_font);
                s.line_height = Some(1.45);
                s.descriptor.text_color = Some(level_tint(entry.level));
                // The first row carries no rule; every later one is separated.
                if i > 0 {
                    s.border_top_width = Some(1.0);
                    s.border_color_top = Some(rule);
                }
            }

            let mut stamp = text(entry.timestamp.clone(), text_secondary, entry_font);
            stamp.style.no_wrap = true;
            let mut level = text(
                entry.level.value().to_uppercase(),
                text_secondary,
                entry_font,
            );
            level.style.no_wrap = true;
            level.style.text_weight = Some(700);
            let mut message = text(entry.message.clone(), level_tint(entry.level), entry_font);
            message.style.min_width = Some(0.0);
            message.style.text_wrap = true;
            message.style.flex_grow = Some(1.0);

            entries_area = entries_area.child(row.child(stamp).child(level).child(message));
        }
    }

    el = el.child(entries_area);

    // Scroll-to-latest hint.
    if spec.auto_scroll {
        el = el.child(text("New entries".to_string(), text_secondary, label_font));
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use poodle_specs::{AuditLogEntry, LogEntry, StreamLogEntry};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Every text run in the tree, in order.
    fn texts(node: &Node) -> Vec<String> {
        let mut out = Vec::new();
        fn walk(node: &Node, out: &mut Vec<String>) {
            if let NodeKind::Text { content } = &node.kind {
                out.push(content.clone());
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        walk(node, &mut out);
        out
    }

    fn stream_spec() -> LogListSpec {
        LogListSpec::new().with_entries([
            LogEntry::Stream(StreamLogEntry::new(
                "10:23:01",
                LogLevel::Info,
                "Server started",
            )),
            LogEntry::Stream(StreamLogEntry::new(
                "10:23:05",
                LogLevel::Warn,
                "Cache miss",
            )),
            LogEntry::Stream(StreamLogEntry::new("10:23:08", LogLevel::Error, "Timeout")),
        ])
    }

    #[test]
    fn stream_rows_render_timestamp_level_and_message() {
        let node = log_list(&stream_spec(), &theme(), None);
        let runs = texts(&node);
        for expected in [
            "10:23:01",
            "INFO",
            "Server started",
            "10:23:05",
            "WARN",
            "Cache miss",
            "10:23:08",
            "ERROR",
            "Timeout",
        ] {
            assert!(
                runs.iter().any(|run| run == expected),
                "missing {expected:?} in {runs:?}"
            );
        }
    }

    #[test]
    fn level_filter_drops_non_matching_rows() {
        let node = log_list(&stream_spec().with_filter_level("error"), &theme(), None);
        let runs = texts(&node);
        assert!(runs.iter().any(|run| run == "Timeout"));
        assert!(!runs.iter().any(|run| run == "Server started"));
    }

    #[test]
    fn text_filter_matches_the_message_case_insensitively() {
        let node = log_list(&stream_spec().with_filter_text("CACHE"), &theme(), None);
        let runs = texts(&node);
        assert!(runs.iter().any(|run| run == "Cache miss"));
        assert!(!runs.iter().any(|run| run == "Timeout"));
    }

    #[test]
    fn max_entries_caps_the_rendered_rows() {
        let node = log_list(&stream_spec().with_max_entries(1), &theme(), None);
        let runs = texts(&node);
        assert!(runs.iter().any(|run| run == "Server started"));
        assert!(!runs.iter().any(|run| run == "Cache miss"));
    }

    #[test]
    fn an_empty_stream_renders_the_empty_surface() {
        let node = log_list(&LogListSpec::new(), &theme(), None);
        assert!(texts(&node).iter().any(|run| run == "No log entries"));
    }

    #[test]
    fn audit_entries_switch_the_list_into_audit_mode() {
        let spec = LogListSpec::new().with_entries([LogEntry::Audit(AuditLogEntry::new(
            "a1",
            "2026-01-01T00:00:00Z",
            "user_login",
            "workspace",
            "w-1",
        ))]);
        assert!(spec.is_audit());
        // Audit rows are not stream rows, so the stream surface stays empty.
        assert!(spec.stream_entries().is_empty());
    }
}
