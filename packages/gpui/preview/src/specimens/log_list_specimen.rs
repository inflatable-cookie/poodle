use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, LogEntry, LogLevel, LogList};
use poodle_specs::EyebrowSpec;
use poodle_specs::{LogFilter, LogListSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let sample_entries = || {
        vec![
            LogEntry::new("10:23:01", LogLevel::Info, "Server started on port 3000"),
            LogEntry::new(
                "10:23:02",
                LogLevel::Debug,
                "Loading configuration from env",
            ),
            LogEntry::new("10:23:05", LogLevel::Warn, "Cache miss for key 'user:42'"),
            LogEntry::new(
                "10:23:08",
                LogLevel::Error,
                "Failed to connect to database: timeout",
            ),
            LogEntry::new(
                "10:23:10",
                LogLevel::Info,
                "Retrying connection (attempt 2/3)",
            ),
        ]
    };

    let audit_entries = || {
        vec![
            LogEntry::new("09:14:22", LogLevel::Info, "Workspace role updated")
                .with_actor("Alice Chen")
                .with_actor_href("/users/alice")
                .with_action("updated")
                .with_resource("Workspace » Acme")
                .with_resource_href("/workspaces/acme"),
            LogEntry::new("09:17:03", LogLevel::Warn, "Access policy modified")
                .with_actor("Bob Martinez")
                .with_actor_href("/users/bob")
                .with_action("modified")
                .with_resource("Policy » Backups")
                .with_resource_href("/policies/backups"),
            LogEntry::new("09:22:41", LogLevel::Error, "API key revoked")
                .with_actor("Carol Patel")
                .with_actor_href("/users/carol")
                .with_action("revoked")
                .with_resource("API key » pk_live_abc123")
                .with_resource_href("/keys/pk_live_abc123"),
            LogEntry::new("09:30:12", LogLevel::Info, "Scheduled backup started")
                .with_action("started")
                .with_resource("Scheduled task » Nightly backup"),
            // ^ actor_name is None on purpose — renders as "System"
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
                            .with_entry_count(5)
                            .with_auto_scroll(true),
                        theme,
                    )
                    .with_entries(sample_entries()),
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
                            .with_entry_count(1)
                            .with_filter_level("error")
                            .with_auto_scroll(true),
                        theme,
                    )
                    .with_entries(sample_entries()),
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
                            .with_entry_count(4)
                            .with_auto_scroll(false),
                        theme,
                    )
                    .with_entries(audit_entries()),
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
                            .with_entry_count(4)
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
                    )
                    .with_entries(audit_entries()),
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
                    LogListSpec::new().with_error("Failed to load audit entries: request timed out"),
                    theme,
                )),
        )
}
