//! LogList — timestamped log entry list backed by LogListSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};
use poodle_specs::LogListSpec;
use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A single log entry for display in the LogList.
///
/// Audit-style rows can populate the optional actor/resource/action
/// fields; when any of them are set the component renders an
/// additional metadata line under the main message.
#[derive(Clone, Debug, Default)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    /// Display name of the principal who triggered the entry
    /// (e.g. "Alice Chen"). When None the row reads as "System".
    pub actor_name: Option<String>,
    /// Optional link target for the actor — when Some, the actor
    /// name renders as an accent-coloured link.
    pub actor_href: Option<String>,
    /// Human-readable label for the affected resource
    /// (e.g. "Workspace » Acme").
    pub resource_label: Option<String>,
    /// Optional link target for the resource.
    pub resource_href: Option<String>,
    /// Action verb for audit rows (e.g. "updated", "deleted").
    pub action: Option<String>,
}

impl LogEntry {
    /// New stream-style entry (no actor/resource metadata).
    pub fn new(timestamp: impl Into<String>, level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: timestamp.into(),
            level,
            message: message.into(),
            actor_name: None,
            actor_href: None,
            resource_label: None,
            resource_href: None,
            action: None,
        }
    }

    pub fn with_actor(mut self, name: impl Into<String>) -> Self {
        self.actor_name = Some(name.into());
        self
    }

    pub fn with_actor_href(mut self, href: impl Into<String>) -> Self {
        self.actor_href = Some(href.into());
        self
    }

    pub fn with_resource(mut self, label: impl Into<String>) -> Self {
        self.resource_label = Some(label.into());
        self
    }

    pub fn with_resource_href(mut self, href: impl Into<String>) -> Self {
        self.resource_href = Some(href.into());
        self
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Whether any of the audit-style fields are populated.
    pub fn is_audit(&self) -> bool {
        self.actor_name.is_some()
            || self.resource_label.is_some()
            || self.action.is_some()
    }
}

/// Log severity level, mapped to badge colours.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    #[default]
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
            Self::Debug => "color.text.secondary",
            Self::Info  => "color.accent.base",
            Self::Warn  => "color.warning.base",
            Self::Error => "color.danger.base",
        }
    }
}

pub struct LogList {
    spec: LogListSpec,
    theme: GpuiThemeProvider,
    entries: Vec<LogEntry>,
    children: Vec<AnyElement>,
    on_filter_change: Option<Box<dyn Fn(&str, &mut Window, &mut App)>>,
    on_search: Option<Box<dyn Fn(&str, &mut Window, &mut App)>>,
}

impl std::ops::Deref for LogList {
    type Target = LogListSpec;
    fn deref(&self) -> &LogListSpec { &self.spec }
}

impl LogList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: LogListSpec::new(), theme: theme.clone(), entries: Vec::new(), children: Vec::new(), on_filter_change: None, on_search: None }
    }
    pub fn from_spec(spec: LogListSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), entries: Vec::new(), children: Vec::new(), on_filter_change: None, on_search: None }
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
    pub fn on_filter_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_filter_change = Some(Box::new(handler)); self
    }
    pub fn on_search(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_search = Some(Box::new(handler)); self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for LogList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let item_gap = rem_to_px(control_space_x_rem(self.spec.density));

        let fill = resolve_color(theme, self.spec.fill_token());
        let label_size = px(font_size);
        let gap = resolve_px(theme, self.spec.entry_gap_token());
        let border_color = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let radius_control = resolve_radius(theme, "radius.control");
        let label_token_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");

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
            .gap(px(item_gap))
            .px(px(pad_x))
            .py(px(pad_y * 0.5))
            .border_b_1()
            .border_color(border_color);

        // Filter level button
        let filter_label: SharedString = self.spec.filter_level
            .clone()
            .unwrap_or_else(|| "All levels".into())
            .into();
        let filter_label_cb = filter_label.clone();
        let mut filter_btn = div()
            .id("log-list-filter-btn")
            .cursor_pointer()
            .flex()
            .items_center()
            .gap(gap_sm)
            .rounded(radius_control)
            .px(gap_sm)
            .py(px(2.0))
            .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.08)))
            .focus(move |s| s.border_color(resolve_color(theme, "color.accent.focusRing")))
            .child(filter_icon)
            .child(
                div()
                    .text_size(label_token_size)
                    .text_color(text_secondary)
                    .child(filter_label),
            );
        if let Some(handler) = self.on_filter_change {
            filter_btn = filter_btn.on_click(move |_, window, cx| {
                handler(&filter_label_cb, window, cx);
            });
        }
        toolbar = toolbar.child(filter_btn);

        // Search button
        let mut search_btn = div()
            .id("log-list-search-btn")
            .cursor_pointer()
            .flex_grow()
            .flex()
            .items_center()
            .justify_end()
            .gap(gap_sm)
            .rounded(radius_control)
            .px(gap_sm)
            .py(px(2.0))
            .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.08)))
            .focus(move |s| s.border_color(resolve_color(theme, "color.accent.focusRing")))
            .child(search_icon)
            .child(
                div()
                    .text_size(label_token_size)
                    .text_color(text_secondary)
                    .child("Search logs\u{2026}"),
            );
        if let Some(handler) = self.on_search {
            search_btn = search_btn.on_click(move |_, window, cx| {
                handler("", window, cx);
            });
        }
        toolbar = toolbar.child(search_btn);

        // ── Entry rows ───────────────────────────────────────────
        let mut rows = div()
            .id("log-list-entries")
            .flex()
            .flex_col()
            .gap(gap)
            .overflow_y_scroll()
            .flex_grow()
            .px(px(pad_x))
            .py(px(pad_y * 0.5));

        if self.entries.is_empty() && self.children.is_empty() {
            rows = rows.child(
                div()
                    .py(px(16.0))
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .text_center()
                    .child("No log entries"),
            );
        }

        let accent = resolve_color(theme, "color.accent.base");

        for entry in &self.entries {
            let badge_fill = resolve_color(theme, entry.level.badge_fill_token());

            let badge = div()
                .px(gap_sm)
                .py(px(1.0))
                .rounded(radius_control)
                .bg(badge_fill.opacity(0.15))
                .text_color(badge_fill)
                .text_size(caption_size)
                .font_weight(FontWeight::BOLD)
                .child(entry.level.label());

            let timestamp = div()
                .text_size(caption_size)
                .text_color(text_secondary)
                .min_w(px(70.0))
                .child(entry.timestamp.clone());

            let message = div()
                .flex_grow()
                .text_size(label_size)
                .text_color(text_primary)
                .child(entry.message.clone());

            let top_row = div()
                .flex()
                .items_center()
                .gap(px(item_gap))
                .child(timestamp)
                .child(badge)
                .child(message);

            // Audit metadata line — renders "Alice updated Workspace »
            // Acme" when any of actor/action/resource are populated.
            // Links use the accent colour as a visual cue; GPUI doesn't
            // expose an anchor tag so the href is stored but not
            // activated here.
            let audit_row = if entry.is_audit() {
                let mut row = div()
                    .flex()
                    .items_center()
                    .gap(gap_sm)
                    .pl(px(70.0 + 8.0)) // align under the message column
                    .text_size(caption_size)
                    .text_color(text_secondary);

                if let Some(ref name) = entry.actor_name {
                    let color = if entry.actor_href.is_some() { accent } else { text_secondary };
                    row = row.child(
                        div()
                            .text_color(color)
                            .font_weight(FontWeight::MEDIUM)
                            .child(name.clone()),
                    );
                } else {
                    row = row.child(
                        div()
                            .text_color(text_secondary)
                            .font_weight(FontWeight::MEDIUM)
                            .child("System"),
                    );
                }

                if let Some(ref action) = entry.action {
                    row = row.child(
                        div()
                            .text_color(text_secondary)
                            .child(format!(" {} ", action)),
                    );
                }

                if let Some(ref label) = entry.resource_label {
                    let color = if entry.resource_href.is_some() { accent } else { text_secondary };
                    row = row.child(
                        div()
                            .text_color(color)
                            .child(label.clone()),
                    );
                }

                Some(row)
            } else {
                None
            };

            let mut entry_block = div()
                .flex()
                .flex_col()
                .py(px(2.0))
                .child(top_row);
            if let Some(audit) = audit_row {
                entry_block = entry_block.child(audit);
            }

            rows = rows.child(entry_block);
        }

        // Append any extra children
        for child in self.children { rows = rows.child(child); }

        // ── Scroll-to-bottom hint ────────────────────────────────
        let mut container = div()
            .id("log-list-container")
            .bg(fill)
            .flex()
            .flex_col()
            .overflow_hidden()
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
