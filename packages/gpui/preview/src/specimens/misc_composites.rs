use gpui::*;
use poodle_composites::{
    PaginationSummarySpec,
    SelectionSummarySpec, SelectionSummaryItem, RemediationAction,
    ConfirmActionSpec,
    SlugFieldSpec,
    EmbedInputSpec, EmbedPreviewSpec,
    LogListSpec,
    FilterToolbarSpec,
    InlineEditableFieldSpec,
};
use poodle_gpui_components::{
    PaginationSummary, SelectionSummary,
    ConfirmAction, SlugField, EmbedInput, EmbedPreview,
    LogList, LogEntry, LogLevel, FilterToolbar, InlineEditableField,
    EditableList, Button, BulkActionBar, FormDialog, Field, TextInput, Eyebrow,
};
use poodle_primitives::{ButtonSpec, ButtonVariant, StatusTone, BulkActionBarSpec, BulkAction, BulkActionTone, TextInputSpec, EyebrowSpec};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().flex_col().gap(px(24.0)).max_w(px(560.0))
        // -- FormDialog --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Form dialog: basic"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Add new user")
                        .description("Invite a user to this workspace.")
                        .submit_label("Add user")
                        .cancel_label("Cancel")
                        .with_child(
                            Field::new("fd-name", "Full name", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_placeholder("Enter name"),
                                        theme,
                                    ).with_id("fd-name")
                                )
                        )
                        .with_child(
                            Field::new("fd-role", "Role", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_placeholder("Select role"),
                                        theme,
                                    ).with_id("fd-role")
                                )
                        )
                )
        )

        // -- FormDialog: With error --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Form dialog: with error"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Create account")
                        .submit_label("Create")
                        .error_message("A user with this email already exists.")
                        .with_child(
                            Field::new("fd-email", "Email", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("existing@example.com"),
                                        theme,
                                    ).with_id("fd-email")
                                )
                        )
                )
        )

        // -- FormDialog: Submitting --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Form dialog: submitting state"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Add new user")
                        .submit_label("Add user")
                        .submitting(true)
                        .with_child(
                            Field::new("fd-name-sub", "Full name", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("Clay Tercek"),
                                        theme,
                                    ).with_id("fd-name-sub")
                                )
                        )
                )
        )

        // -- ConfirmAction --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Confirm action"), theme))
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
        )

        // -- SlugField --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Slug field"), theme))
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
        )

        // -- InlineEditableField --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Inline editable field"), theme))
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
                                    .with_placeholder("Empty \u{2014} click to add"),
                                theme,
                            )
                        )
                )
        )

        // -- EmbedInput --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Embed input"), theme))
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
        )

        // -- EmbedPreview --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Embed preview"), theme))
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
        )

        // -- FilterToolbar --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Filter toolbar"), theme))
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
        )

        // -- LogList --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Log list"), theme))
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
        )

        // -- EditableList --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Editable list"), theme))
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
        )

        // -- PaginationSummary --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Pagination summary"), theme))
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
        )

        // -- BulkActionBar --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Bulk action bar"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            BulkActionBar::from_spec(
                                BulkActionBarSpec::new()
                                    .with_selection_count(5)
                                    .with_total_count(42)
                                    .with_actions(vec![
                                        BulkAction::new("export", "Export"),
                                        BulkAction::new("archive", "Archive"),
                                        BulkAction::new("delete", "Delete").with_tone(BulkActionTone::Danger),
                                    ]),
                                theme,
                            )
                        )
                        .child(
                            BulkActionBar::from_spec(
                                BulkActionBarSpec::new()
                                    .with_selection_count(1)
                                    .with_actions(vec![
                                        BulkAction::new("export", "Export"),
                                        BulkAction::new("archive", "Archive"),
                                    ]),
                                theme,
                            )
                        )
                )
        )

        // -- SelectionSummary --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Selection summary"), theme))
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
        )
}
