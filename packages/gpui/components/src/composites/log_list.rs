//! LogList — timestamped log entry list backed by LogListSpec.
//!
//! Two variants, matching the contract/Svelte `variant="auto"` split:
//! - **stream**: runtime/console logs — level-filter chips with live counts,
//!   a text-search affordance, monospace entry rows, scroll-to-latest hint.
//! - **audit**: activity history — each entry renders as a distinct row with a
//!   circular action-type icon and an actor/action/resource line.
//!
//! The variant is auto-detected from the entry shape (any entry with audit
//! metadata flips the whole list to audit mode), mirroring the Svelte
//! `entries.some(isAuditEntry)` detection.

use crate::presentation::{
    panel_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::primitives::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::LogListSpec;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

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
        self.actor_name.is_some() || self.resource_label.is_some() || self.action.is_some()
    }

    /// Single-letter marker shown in the audit action icon — derived from the
    /// action verb (or the level when no verb is present), matching the Svelte
    /// `actionType.slice(0, 1).toUpperCase()`.
    fn audit_marker(&self) -> String {
        self.action
            .as_deref()
            .and_then(|a| a.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| self.level.label().chars().take(1).collect())
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

    /// Filter-chip label (matches Svelte stream chips: All/Info/Warn/Error).
    fn chip_label(self) -> &'static str {
        match self {
            Self::Debug => "Debug",
            Self::Info => "Info",
            Self::Warn => "Warn",
            Self::Error => "Error",
        }
    }

    /// Lowercase key matching `LogListSpec::filter_level` values.
    fn key(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    fn badge_fill_token(self) -> &'static str {
        match self {
            Self::Debug => "color.text.secondary",
            Self::Info => "color.accent.base",
            Self::Warn => "color.status.warning",
            Self::Error => "color.status.danger",
        }
    }
}

type FilterHandler = std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;

pub struct LogList {
    spec: LogListSpec,
    theme: GpuiThemeProvider,
    entries: Vec<LogEntry>,
    children: Vec<AnyElement>,
    on_filter_change: Option<FilterHandler>,
    on_search: Option<Box<dyn Fn(&str, &mut Window, &mut App)>>,
}

impl std::ops::Deref for LogList {
    type Target = LogListSpec;
    fn deref(&self) -> &LogListSpec {
        &self.spec
    }
}

impl LogList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: LogListSpec::new(),
            theme: theme.clone(),
            entries: Vec::new(),
            children: Vec::new(),
            on_filter_change: None,
            on_search: None,
        }
    }
    pub fn from_spec(spec: LogListSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            entries: Vec::new(),
            children: Vec::new(),
            on_filter_change: None,
            on_search: None,
        }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    pub fn with_entry(mut self, entry: LogEntry) -> Self {
        self.entries.push(entry);
        self
    }
    pub fn with_entries(mut self, entries: impl IntoIterator<Item = LogEntry>) -> Self {
        self.entries.extend(entries);
        self
    }
    pub fn on_filter_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_filter_change = Some(std::rc::Rc::new(handler));
        self
    }
    pub fn on_search(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_search = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for LogList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));

        // ── Token-resolved palette + metrics ─────────────────────
        let fill = resolve_color(theme, self.spec.fill_token());
        let elevated = resolve_color(theme, "color.background.elevated");
        // Svelte toolbar bg: color-mix(elevated 92%, transparent).
        let toolbar_fill = Hsla {
            a: elevated.a * 0.92,
            ..elevated
        };
        let label_size = px(font_size);
        let entry_gap = resolve_px(theme, self.spec.entry_gap_token());
        let inline_sm = resolve_px(theme, "space.inline.sm");
        let inline_md = resolve_px(theme, "space.inline.md");
        let toolbar_pad_y = resolve_px(theme, "space.stack.md"); // 0.75rem
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_default = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let accent = resolve_color(theme, "color.accent.base");
        let radius_control = resolve_radius(theme, "radius.control");
        let radius_surface = resolve_radius(theme, "radius.surface");
        let radius_pill = resolve_radius(theme, "radius.pill");
        let label_token_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let border_w = resolve_px(theme, "border.width.default");

        // ── Variant auto-detection (matches Svelte) ──────────────
        let is_audit = self.entries.iter().any(LogEntry::is_audit);

        let container = div()
            .id("log-list-container")
            .bg(fill)
            .flex()
            .flex_col()
            .border(border_w)
            .border_color(border_subtle)
            .rounded(radius_surface)
            .overflow_hidden();

        if is_audit {
            return self.render_audit(
                container,
                AuditPalette {
                    border_subtle,
                    text_primary,
                    text_secondary,
                    caption_size,
                    radius_pill,
                    inline_sm,
                    pad_x,
                    accent,
                },
            );
        }

        self.render_stream(
            container,
            StreamPalette {
                toolbar_fill,
                border_subtle,
                border_default,
                text_primary,
                text_secondary,
                text_inverse,
                accent,
                radius_control,
                radius_pill,
                label_token_size,
                caption_size,
                label_size,
                entry_gap,
                inline_sm,
                inline_md,
                toolbar_pad_y,
                pad_x,
                border_w,
            },
        )
    }
}

struct StreamPalette {
    toolbar_fill: Hsla,
    border_subtle: Hsla,
    border_default: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    text_inverse: Hsla,
    accent: Hsla,
    radius_control: Pixels,
    radius_pill: Pixels,
    label_token_size: Pixels,
    caption_size: Pixels,
    label_size: Pixels,
    entry_gap: Pixels,
    inline_sm: Pixels,
    inline_md: Pixels,
    toolbar_pad_y: Pixels,
    pad_x: f32,
    border_w: Pixels,
}

struct AuditPalette {
    border_subtle: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    caption_size: Pixels,
    radius_pill: Pixels,
    inline_sm: Pixels,
    pad_x: f32,
    accent: Hsla,
}

impl LogList {
    fn render_stream(self, container: Stateful<Div>, p: StreamPalette) -> AnyElement {
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

    fn render_audit(self, container: Stateful<Div>, p: AuditPalette) -> AnyElement {
        let theme = &self.theme;

        let audit: Vec<&LogEntry> = self.entries.iter().filter(|e| e.is_audit()).collect();

        let mut list = div().flex().flex_col();

        if audit.is_empty() {
            // Empty state (Svelte status surface; min-height 12rem, centred).
            list = list.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_h(px(rem_to_px(12.0)))
                    .px(px(p.pad_x))
                    .py(px(rem_to_px(2.0)))
                    .text_color(p.text_secondary)
                    .text_center()
                    .child("No log entries found"),
            );
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
                row = row.border_t(px(1.0)).border_color(p.border_subtle);
            }

            list = list.child(row);
        }

        container.child(list).into_any_element()
    }
}

/// Resolve the status-tinted icon background + foreground for an audit entry,
/// keyed off the action verb (matches Svelte `data-action-type` colours).
fn audit_icon_palette(
    entry: &LogEntry,
    p: &AuditPalette,
    theme: &GpuiThemeProvider,
) -> (Hsla, Hsla) {
    let action = entry.action.as_deref().unwrap_or("").to_lowercase();
    let token = if action.contains("creat") || action.contains("restore") {
        Some("color.status.success")
    } else if action.contains("delet") || action.contains("remov") || action.contains("revok") {
        Some("color.status.danger")
    } else if action.contains("updat")
        || action.contains("upload")
        || action.contains("modif")
        || action.contains("edit")
    {
        Some("color.accent.base")
    } else if action.contains("role")
        || action.contains("suspend")
        || action.contains("permission")
    {
        Some("color.status.warning")
    } else {
        None
    };

    match token {
        Some(t) => {
            let c = resolve_color(theme, t);
            (
                Hsla {
                    a: c.a * 0.18,
                    ..c
                },
                c,
            )
        }
        // Default: neutral 35% border-subtle tint, primary text.
        None => (
            Hsla {
                a: p.border_subtle.a * 0.35,
                ..p.border_subtle
            },
            p.text_primary,
        ),
    }
}

/// Build a stream-mode level filter chip with a count badge. Active chips take
/// the accent-tinted treatment (Svelte `.poodle-active`).
fn filter_chip(
    id: &str,
    label: &str,
    count: usize,
    active: bool,
    p: &StreamPalette,
    handler: Option<FilterHandler>,
    key: &str,
) -> Stateful<Div> {
    let (bg, border, fg) = if active {
        (
            Some(Hsla {
                a: p.accent.a * 0.16,
                ..p.accent
            }),
            Hsla {
                a: p.accent.a * 0.42,
                ..p.accent
            },
            p.text_primary,
        )
    } else {
        (None, p.border_default, p.text_secondary)
    };

    let mut chip = div()
        .id(SharedString::from(id.to_string()))
        .flex()
        .items_center()
        .gap(px(rem_to_px(0.25))) // Svelte chip gap: 0.25rem
        .px(p.inline_sm)
        .py(px(rem_to_px(0.1875))) // Svelte chip padding-block: 0.1875rem
        .border(p.border_w)
        .border_color(border)
        .rounded(p.radius_control)
        .text_size(p.label_token_size)
        .text_color(fg)
        .child(label.to_string())
        .child(
            div()
                .text_size(p.caption_size)
                .text_color(p.text_secondary)
                .child(count.to_string()),
        );
    if let Some(b) = bg {
        chip = chip.bg(b);
    }
    if let Some(handler) = handler {
        let key = key.to_string();
        chip = chip.cursor_pointer().on_click(move |_, window, cx| {
            handler(&key, window, cx);
        });
    }
    chip
}
