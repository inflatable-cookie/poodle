//! LogList — timestamped log entry list backed by LogListSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use pug_composites::LogListSpec;
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px};

/// A single log entry for display in the LogList.
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

/// Log severity level, mapped to badge colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn label(self) -> &'static str {
        match self {
            Self::Debug => "DBG",
            Self::Info => "INF",
            Self::Warn => "WRN",
            Self::Error => "ERR",
        }
    }

    fn badge_fill_token(self) -> &'static str {
        match self {
            Self::Debug => "semantic.color.text.secondary",
            Self::Info  => "semantic.color.accent.base",
            Self::Warn  => "semantic.color.warning.base",
            Self::Error => "semantic.color.danger.base",
        }
    }
}

pub struct LogList {
    spec: LogListSpec,
    theme: GpuiThemeProvider,
    entries: Vec<LogEntry>,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for LogList {
    type Target = LogListSpec;
    fn deref(&self) -> &LogListSpec { &self.spec }
}

impl LogList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: LogListSpec::new(), theme: theme.clone(), entries: Vec::new(), children: Vec::new() }
    }
    pub fn from_spec(spec: LogListSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), entries: Vec::new(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
    pub fn with_entry(mut self, entry: LogEntry) -> Self {
        self.entries.push(entry); self
    }
    pub fn with_entries(mut self, entries: impl IntoIterator<Item = LogEntry>) -> Self {
        self.entries.extend(entries); self
    }
}

impl IntoElement for LogList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let fill = resolve_color(theme, self.spec.fill_token());
        let gap = resolve_px(theme, self.spec.entry_gap_token());
        let border_color = resolve_color(theme, "semantic.color.border.subtle");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        // ── Toolbar ──────────────────────────────────────────────
        let filter_icon = Icon::from_spec(
            IconSpec::new("filter").with_size(IconSize::Sm),
            theme,
        ).with_color(text_secondary);

        let search_icon = Icon::from_spec(
            IconSpec::new("search").with_size(IconSize::Sm),
            theme,
        ).with_color(text_secondary);

        let mut toolbar = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(6.0))
            .border_b_1()
            .border_color(border_color);

        // Filter level indicator
        toolbar = toolbar.child(
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(filter_icon)
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child(
                            self.spec.filter_level
                                .clone()
                                .unwrap_or_else(|| "All levels".into()),
                        ),
                ),
        );

        // Search placeholder
        toolbar = toolbar.child(
            div()
                .flex_grow()
                .flex()
                .items_center()
                .justify_end()
                .gap(px(4.0))
                .child(search_icon)
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child("Search logs\u{2026}"),
                ),
        );

        // ── Entry rows ───────────────────────────────────────────
        let mut rows = div()
            .flex()
            .flex_col()
            .gap(gap)
            .overflow_hidden()
            .flex_grow()
            .px(px(12.0))
            .py(px(6.0));

        if self.entries.is_empty() && self.children.is_empty() {
            rows = rows.child(
                div()
                    .py(px(16.0))
                    .text_size(px(13.0))
                    .text_color(text_secondary)
                    .text_center()
                    .child("No log entries"),
            );
        }

        for entry in &self.entries {
            let badge_fill = resolve_color(theme, entry.level.badge_fill_token());

            let badge = div()
                .px(px(4.0))
                .py(px(1.0))
                .rounded(px(3.0))
                .bg(badge_fill.opacity(0.15))
                .text_color(badge_fill)
                .text_size(px(10.0))
                .font_weight(FontWeight::BOLD)
                .child(entry.level.label());

            let timestamp = div()
                .text_size(px(11.0))
                .text_color(text_secondary)
                .min_w(px(70.0))
                .child(entry.timestamp.clone());

            let message = div()
                .flex_grow()
                .text_size(px(13.0))
                .text_color(text_primary)
                .child(entry.message.clone());

            let row = div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .py(px(2.0))
                .child(timestamp)
                .child(badge)
                .child(message);

            rows = rows.child(row);
        }

        // Append any extra children
        for child in self.children { rows = rows.child(child); }

        // ── Scroll-to-bottom hint ────────────────────────────────
        let mut container = div()
            .bg(fill)
            .flex()
            .flex_col()
            .overflow_y_hidden()
            .child(toolbar)
            .child(rows);

        if self.spec.auto_scroll {
            let arrow_icon = Icon::from_spec(
                IconSpec::new("arrow-down").with_size(IconSize::Sm),
                theme,
            ).with_color(text_secondary);

            container = container.child(
                div()
                    .flex()
                    .justify_center()
                    .py(px(4.0))
                    .border_t_1()
                    .border_color(border_color)
                    .child(arrow_icon),
            );
        }

        container.into_any_element()
    }
}
