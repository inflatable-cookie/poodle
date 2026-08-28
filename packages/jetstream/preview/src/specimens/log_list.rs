//! LogList specimen — timestamped log viewer.

use crate::compat::js_log_list;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{LogEntry, LogFilter, LogLevel, LogListSpec, StreamLogEntry};

/// Stream rows for the specimen. `LogListSpec` carries the entries themselves
/// since g12.019 — it used to take only a count, and the renderer drew a
/// placeholder line instead of rows.
fn stream_entries(count: usize) -> Vec<LogEntry> {
    const MESSAGES: [(LogLevel, &str); 4] = [
        (LogLevel::Info, "Server started on port 3000"),
        (LogLevel::Warn, "Cache miss for key 'user:42'"),
        (LogLevel::Error, "Failed to connect to database: timeout"),
        (LogLevel::Info, "Retrying connection (attempt 2/3)"),
    ];
    (0..count)
        .map(|i| {
            let (level, message) = MESSAGES[i % MESSAGES.len()];
            LogEntry::Stream(StreamLogEntry::new(
                format!("10:23:{:02}", i),
                level,
                message,
            ))
        })
        .collect()
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "With entries",
            secondary,
            js_log_list(
                &LogListSpec::new()
                    .with_entries(stream_entries(8))
                    .with_auto_scroll(true),
                theme,
                "log-list-1",
            ),
        ))
        .child(group(
            "Filtered to errors",
            secondary,
            js_log_list(
                &LogListSpec::new()
                    .with_entries(stream_entries(12))
                    .with_filter_level("error"),
                theme,
                "log-list-2",
            ),
        ))
        .child(group(
            "Audit toolbar + pagination",
            secondary,
            js_log_list(
                &LogListSpec::new()
                    .with_filter(
                        LogFilter::select("action", "Action")
                            .with_placeholder("All actions")
                            .with_option("create", "Create")
                            .with_option("delete", "Delete"),
                    )
                    .with_filter(LogFilter::date("from", "From"))
                    .with_filter_value("action", "delete")
                    .with_page(2)
                    .with_page_size(20)
                    .with_total(85),
                theme,
                "log-list-3",
            ),
        ))
        .child(group(
            "Loading",
            secondary,
            js_log_list(&LogListSpec::new().with_loading(true), theme, "log-list-5"),
        ))
        .child(group(
            "Error",
            secondary,
            js_log_list(
                &LogListSpec::new().with_error("Failed to load audit entries"),
                theme,
                "log-list-4",
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
