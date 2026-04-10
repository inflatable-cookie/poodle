use gpui::*;
use poodle_composites::LogListSpec;
use poodle_gpui_components::{LogList, LogEntry, LogLevel, Eyebrow};
use poodle_primitives::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let sample_entries = || vec![
        LogEntry { timestamp: "10:23:01".to_string(), level: LogLevel::Info, message: "Server started on port 3000".to_string() },
        LogEntry { timestamp: "10:23:02".to_string(), level: LogLevel::Debug, message: "Loading configuration from env".to_string() },
        LogEntry { timestamp: "10:23:05".to_string(), level: LogLevel::Warn, message: "Cache miss for key 'user:42'".to_string() },
        LogEntry { timestamp: "10:23:08".to_string(), level: LogLevel::Error, message: "Failed to connect to database: timeout".to_string() },
        LogEntry { timestamp: "10:23:10".to_string(), level: LogLevel::Info, message: "Retrying connection (attempt 2/3)".to_string() },
    ];

    div().flex().flex_col().gap(px(24.0))
        // -- Log output with filtering --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Log output with filtering"), theme))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entry_count(5)
                            .with_auto_scroll(true),
                        theme,
                    )
                    .with_entries(sample_entries())
                )
        )
        // -- Filtered (errors only) --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Filtered (errors only)"), theme))
                .child(
                    LogList::from_spec(
                        LogListSpec::new()
                            .with_entry_count(1)
                            .with_filter_level("error")
                            .with_auto_scroll(true),
                        theme,
                    )
                    .with_entries(sample_entries())
                )
        )
}
