use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    PaginationSummarySpec,
    SelectionSummarySpec, SelectionSummaryItem, RemediationAction,
    ConfirmActionSpec,
    SlugFieldSpec,
    EmbedInputSpec, EmbedPreviewSpec,
    LogListSpec,
    FilterToolbarSpec,
    InlineEditableFieldSpec,
};
use pug_gpui_components::{
    PaginationSummary, SelectionSummary,
    ConfirmAction, SlugField, EmbedInput, EmbedPreview,
    LogList, LogEntry, LogLevel, FilterToolbar, InlineEditableField,
    EditableList, Button,
};
use pug_primitives::{ButtonSpec, ButtonVariant, StatusTone};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(560.0))
        // ── ConfirmAction ───────────────────────────────────────
        .child(section_label("CONFIRM ACTION", text_secondary))
        .child(
            ConfirmAction::from_spec(
                ConfirmActionSpec::new(
                    "Delete item?",
                    "This action cannot be undone. The item will be permanently removed.",
                    "Delete",
                    "Cancel",
                ).with_tone(StatusTone::Danger),
                theme,
            ).with_trigger(
                Button::from_spec(
                    ButtonSpec::new().with_variant(ButtonVariant::Danger).with_label("Delete item"),
                    theme,
                ).with_id("misc-confirm-trigger")
            )
        )

        // ── SlugField ───────────────────────────────────────────
        .child(section_label("SLUG FIELD", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    SlugField::from_spec(
                        SlugFieldSpec::new("hello-world")
                            .with_prefix("/blog/"),
                        theme,
                    )
                )
                .child(
                    SlugField::from_spec(
                        SlugFieldSpec::new("my-project")
                            .with_prefix("/projects/")
                            .with_locked(true),
                        theme,
                    )
                )
        )

        // ── InlineEditableField ────────────────────────────────
        .child(section_label("INLINE EDITABLE FIELD", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    InlineEditableField::from_spec(
                        InlineEditableFieldSpec::new("Click to edit this value"),
                        theme,
                    )
                )
                .child(
                    InlineEditableField::from_spec(
                        InlineEditableFieldSpec::new("Editing mode")
                            .with_editing(true),
                        theme,
                    )
                )
                .child(
                    InlineEditableField::from_spec(
                        InlineEditableFieldSpec::new("")
                            .with_placeholder("Empty — click to add"),
                        theme,
                    )
                )
        )

        // ── EmbedInput ──────────────────────────────────────────
        .child(section_label("EMBED INPUT", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    EmbedInput::from_spec(
                        EmbedInputSpec::new()
                            .with_placeholder("Paste a URL to embed..."),
                        theme,
                    )
                )
                .child(
                    EmbedInput::from_spec(
                        EmbedInputSpec::new()
                            .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                        theme,
                    )
                )
        )

        // ── EmbedPreview ────────────────────────────────────────
        .child(section_label("EMBED PREVIEW", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_title("Rick Astley - Never Gonna Give You Up")
                            .with_description("The official video for Rick Astley's classic 1987 hit.")
                            .with_provider("YouTube"),
                        theme,
                    )
                )
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_loading(true),
                        theme,
                    )
                )
        )

        // ── FilterToolbar ───────────────────────────────────────
        .child(section_label("FILTER TOOLBAR", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_query("button")
                            .with_active_filter_count(2)
                            .with_result_count(14)
                            .with_show_clear_action(true),
                        theme,
                    )
                )
                .child(
                    FilterToolbar::from_spec(
                        FilterToolbarSpec::new()
                            .with_result_count(156),
                        theme,
                    )
                )
        )

        // ── LogList ─────────────────────────────────────────────
        .child(section_label("LOG LIST", text_secondary))
        .child(
            LogList::from_spec(
                LogListSpec::new()
                    .with_entry_count(5)
                    .with_auto_scroll(true),
                theme,
            )
            .with_entries(vec![
                LogEntry { timestamp: "10:23:01".to_string(), level: LogLevel::Info, message: "Server started on port 3000".to_string() },
                LogEntry { timestamp: "10:23:02".to_string(), level: LogLevel::Debug, message: "Loading configuration from env".to_string() },
                LogEntry { timestamp: "10:23:05".to_string(), level: LogLevel::Warn, message: "Cache miss for key 'user:42'".to_string() },
                LogEntry { timestamp: "10:23:08".to_string(), level: LogLevel::Error, message: "Failed to connect to database: timeout".to_string() },
                LogEntry { timestamp: "10:23:10".to_string(), level: LogLevel::Info, message: "Retrying connection (attempt 2/3)".to_string() },
            ])
        )

        // ── EditableList ────────────────────────────────────────
        .child(section_label("EDITABLE LIST", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    EditableList::new(theme)
                        .title("Tags")
                        .items(vec![
                            "design-system".to_string(),
                            "rust".to_string(),
                            "gpui".to_string(),
                        ])
                        .max_items(10)
                        .reorderable(true)
                )
                .child(
                    EditableList::new(theme)
                        .title("Disabled list")
                        .items(vec![
                            "locked-item".to_string(),
                        ])
                        .disabled(true)
                )
        )

        // ── PaginationSummary ───────────────────────────────────
        .child(section_label("PAGINATION SUMMARY", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    PaginationSummary::from_spec(
                        PaginationSummarySpec::new(1, 20, 156),
                        theme,
                    )
                )
                .child(
                    PaginationSummary::from_spec(
                        PaginationSummarySpec::new(5, 20, 1000),
                        theme,
                    )
                )
        )

        // ── SelectionSummary ────────────────────────────────────
        .child(section_label("SELECTION SUMMARY", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("btn", "Button"),
                            SelectionSummaryItem::new("card", "Card"),
                            SelectionSummaryItem::new("dialog", "Dialog"),
                            SelectionSummaryItem::new("table", "Table"),
                            SelectionSummaryItem::new("tabs", "Tabs"),
                        ])
                        .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
                    )
                )
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("primary-btn", "Primary button"),
                        ]),
                        theme,
                    )
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
