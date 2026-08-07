use crate::node_compat::{Eyebrow, LogList};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    AuditLogEntry, LogActor, LogEntry, LogFilter, LogLevel, LogListSpec, StreamLogEntry,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    // The contract's level set is info | warn | error; the old GPUI tier's extra
    // `Debug` level had no contract or Svelte counterpart.
    let stream = |ts: &str, level: LogLevel, message: &str| {
        LogEntry::Stream(StreamLogEntry::new(ts, level, message))
    };
    let sample_entries = || {
        vec![
            stream("10:23:01", LogLevel::Info, "Server started on port 3000"),
            stream(
                "10:23:02",
                LogLevel::Info,
                "Loading configuration from env",
            ),
            stream("10:23:05", LogLevel::Warn, "Cache miss for key 'user:42'"),
            stream(
                "10:23:08",
                LogLevel::Error,
                "Failed to connect to database: timeout",
            ),
            stream(
                "10:23:10",
                LogLevel::Info,
                "Retrying connection (attempt 2/3)",
            ),
        ]
    };

    let audit_entries = || {
        vec![
            LogEntry::Audit(
                AuditLogEntry::new("a1", "09:14:22", "updated", "workspace", "acme")
                    .with_actor(
                        LogActor::new("u-alice")
                            .with_name("Alice Chen")
                            .with_href("/users/alice"),
                    )
                    .with_resource_label("Workspace \u{00BB} Acme")
                    .with_resource_href("/workspaces/acme"),
            ),
            LogEntry::Audit(
                AuditLogEntry::new("a2", "09:17:03", "modified", "policy", "backups")
                    .with_actor(
                        LogActor::new("u-bob")
                            .with_name("Bob Martinez")
                            .with_href("/users/bob"),
                    )
                    .with_resource_label("Policy \u{00BB} Backups")
                    .with_resource_href("/policies/backups"),
            ),
            LogEntry::Audit(
                AuditLogEntry::new("a3", "09:22:41", "revoked", "api_key", "pk_live_abc123")
                    .with_actor(
                        LogActor::new("u-carol")
                            .with_name("Carol Patel")
                            .with_href("/users/carol"),
                    )
                    .with_resource_label("API key \u{00BB} pk_live_abc123")
                    .with_resource_href("/keys/pk_live_abc123"),
            ),
            // No actor on purpose — contract §"resolveActorName" reads "System".
            LogEntry::Audit(
                AuditLogEntry::new("a4", "09:30:12", "started", "scheduled_task", "nightly")
                    .with_resource_label("Scheduled task \u{00BB} Nightly backup"),
            ),
        ]
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // -- Log output with filtering --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Log output with filtering"),
                    theme,
                ))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entries(sample_entries())
                            .with_auto_scroll(true),
                        theme,
                    ),
                ),
        )
        // -- Filtered (errors only) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Filtered (errors only)"),
                    theme,
                ))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entries(sample_entries())
                            .with_filter_level("error")
                            .with_auto_scroll(true),
                        theme,
                    ),
                ),
        )
        // -- Audit log (actor + resource links) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Audit log (actor + resource links)"),
                    theme,
                ))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entries(audit_entries())
                            .with_auto_scroll(false),
                        theme,
                    ),
                ),
        )
        // -- Audit log with filter toolbar + pagination --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Audit log (toolbar + pagination)"),
                    theme,
                ))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entries(audit_entries())
                            .with_filter(
                                LogFilter::select("action", "Action")
                                    .with_placeholder("All actions")
                                    .with_option("create", "Create")
                                    .with_option("update", "Update")
                                    .with_option("delete", "Delete"),
                            )
                            .with_filter(LogFilter::date("from", "From"))
                            .with_filter_value("action", "update")
                            .with_page(2)
                            .with_page_size(4)
                            .with_total(18),
                        theme,
                    ),
                ),
        )
        // -- Audit log loading state --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Audit log (loading)"),
                    theme,
                ))
                .child(LogList::from_spec(
                    LogListSpec::new().with_loading(true),
                    theme,
                )),
        )
        // -- Audit log error state --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Audit log (error)"),
                    theme,
                ))
                .child(LogList::from_spec(
                    LogListSpec::new()
                        .with_error("Failed to load audit entries: request timed out"),
                    theme,
                )),
        )
}
