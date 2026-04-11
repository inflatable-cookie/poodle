//! Demo screen renderer — converts adapter demo screens into real Poodle components.
//!
//! Each screen uses the real `Poodle*` component constructors from `poodle_gpui_components`,
//! backed by spec structs from the gpui spec crates. Raw `div()` calls are only used
//! for structural layout where no specific Poodle component maps (e.g., breadcrumb trails).

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{
    AppHeader, Button, Callout, Checkbox, DataTable, DetailShell,
    Field, PaginationSummary, Pill, Progress, Separator,
    StatusBar, Surface, TextInput,
};
use poodle_specs::{DataTableSpec, DetailShellSpec, PaginationSummarySpec, TableColumnSpec, TableRowSpec};
use poodle_specs::{
    ButtonSpec, ButtonVariant, CallOutSpec, CheckboxSpec, FieldSpec,
    PillSpec, ProgressSpec, SeparatorSpec, StatusTone, SurfaceSpec, SurfaceTone, TextInputSpec,
};
use poodle_specs::{AppHeaderSpec, ShellStatusBarSpec};

use crate::app_state::DemoScreen;
use crate::style_bridge::color_to_hsla;

/// Render a single demo screen by id.
pub fn render_single_screen(theme: &GpuiThemeProvider, screen: DemoScreen) -> Div {
    match screen {
        DemoScreen::OverviewShell => render_overview_shell(theme),
        DemoScreen::FormAndValidation => render_form_screen(theme),
        DemoScreen::BrowseAndTable => render_browse_screen(theme),
        DemoScreen::DetailAndRelatedData => render_detail_screen(theme),
        DemoScreen::PickerAndMedia => render_picker_screen(theme),
        DemoScreen::CommandAndWorkspace => render_workspace_screen(theme),
    }
}

// --- Screen renderers ---

fn render_overview_shell(theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(12.0));

    // Callout — real Callout with info tone
    let callout_spec = CallOutSpec::new()
        .with_tone(StatusTone::Info)
        .with_title("Welcome")
        .with_content("Your workspace is ready. 3 items need attention.");
    content = content.child(Callout::from_spec(callout_spec, theme));

    // State tiles grid — each tile is a Surface with elevated tone
    let text_secondary = theme.resolve_color("color.text.secondary");
    let success = theme.resolve_color("color.status.success");
    let warning = theme.resolve_color("color.status.warning");
    let danger = theme.resolve_color("color.status.danger");

    let tiles = [
        ("Active projects", "12", success),
        ("Pending reviews", "3", warning),
        ("Open issues", "27", danger),
    ];

    let mut tile_row = div().flex().gap(px(8.0));
    for (label, value, status_color) in &tiles {
        let surface_spec = SurfaceSpec::new().with_tone(SurfaceTone::Elevated);
        let tile_content = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child(label.to_string()),
            )
            .child(
                div()
                    .text_xl()
                    .text_color(color_to_hsla(*status_color))
                    .child(value.to_string()),
            );
        tile_row = tile_row.child(
            div().flex_1().child(
                Surface::from_spec(surface_spec, theme).with_content(tile_content),
            ),
        );
    }
    content = content.child(tile_row);

    // Progress bar — real Progress at 75%
    content = content.child(
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Sprint progress"),
            )
            .child(Progress::from_spec(
                ProgressSpec::new().with_value(75.0),
                theme,
            )),
    );

    content
}

fn render_form_screen(theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(12.0));

    // Form fields — Field wrapping TextInput
    let fields = [
        ("title", "Title", "Enter title...", true),
        ("description", "Description", "Enter description...", false),
        ("category", "Category", "Select category...", false),
    ];

    for (id, label, placeholder, is_required) in &fields {
        let field_spec = FieldSpec::new(*id, *label).with_required(*is_required);
        let input_spec = TextInputSpec::new()
            .with_placeholder(*placeholder)
            .with_id(*id);
        let input = TextInput::from_spec(input_spec, theme).with_id(*id);
        content = content.child(Field::from_spec(field_spec, theme).with_control(input));
    }

    // Validation callout — Callout with danger tone
    let validation_callout = CallOutSpec::new()
        .with_tone(StatusTone::Danger)
        .with_content("Please correct the errors above");
    content = content.child(Callout::from_spec(validation_callout, theme));

    // Action buttons — real Button components
    let cancel_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Secondary)
        .with_label("Cancel");
    let submit_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Primary)
        .with_label("Submit");
    content = content.child(
        div()
            .flex()
            .gap(px(8.0))
            .justify_end()
            .child(Button::from_spec(cancel_spec, theme).with_id("cancel"))
            .child(Button::from_spec(submit_spec, theme).with_id("submit")),
    );

    content
}

fn render_browse_screen(theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(12.0));

    // Search bar — real TextInput with search placeholder
    let search_spec = TextInputSpec::new()
        .with_placeholder("Search assets...")
        .with_leading_icon("search")
        .with_id("browse-search");
    content = content.child(TextInput::from_spec(search_spec, theme).with_id("browse-search"));

    // Data table — real DataTable with column/row specs
    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("type", "Type"),
        TableColumnSpec::new("status", "Status"),
        TableColumnSpec::new("modified", "Modified"),
    ];

    let rows = vec![
        TableRowSpec::new(
            "1",
            vec![
                (String::from("name"), String::from("hero-banner.png")),
                (String::from("type"), String::from("Image")),
                (String::from("status"), String::from("Published")),
                (String::from("modified"), String::from("2 hours ago")),
            ],
        ),
        TableRowSpec::new(
            "2",
            vec![
                (String::from("name"), String::from("main.css")),
                (String::from("type"), String::from("Stylesheet")),
                (String::from("status"), String::from("Draft")),
                (String::from("modified"), String::from("Yesterday")),
            ],
        ),
        TableRowSpec::new(
            "3",
            vec![
                (String::from("name"), String::from("index.html")),
                (String::from("type"), String::from("Document")),
                (String::from("status"), String::from("Published")),
                (String::from("modified"), String::from("3 days ago")),
            ],
        ),
        TableRowSpec::new(
            "4",
            vec![
                (String::from("name"), String::from("config.json")),
                (String::from("type"), String::from("Config")),
                (String::from("status"), String::from("Review")),
                (String::from("modified"), String::from("1 week ago")),
            ],
        ),
    ];

    let table_spec = DataTableSpec::new(columns, rows)
        .with_aria_label("Asset browser");
    content = content.child(DataTable::from_spec(table_spec, theme));

    // Pagination — real PaginationSummary
    let pagination_spec = PaginationSummarySpec::new(1, 25, 142);
    content = content.child(PaginationSummary::from_spec(pagination_spec, theme));

    content
}

fn render_detail_screen(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Breadcrumbs — kept as raw div() (no specific Poodle component for breadcrumbs)
    content = content.child(
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .text_sm()
            .text_color(color_to_hsla(text_secondary))
            .child("Home")
            .child(div().child("/"))
            .child("Assets")
            .child(div().child("/"))
            .child(
                div()
                    .text_color(color_to_hsla(text_primary))
                    .child("hero-banner.png"),
            ),
    );

    // Detail shell — real DetailShell with title and content sections
    let detail_spec = DetailShellSpec::new().with_title("hero-banner.png");

    // Build content for the detail shell: overview section + separator + media section
    let overview_section = Surface::from_spec(
        SurfaceSpec::new().with_tone(SurfaceTone::Elevated),
        theme,
    )
    .with_content(
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(div().text_sm().child("Overview"))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Image file uploaded 2 hours ago. 1920\u{00d7}1080 pixels, 2.4 MB."),
            )
            .child(
                div()
                    .flex()
                    .gap(px(6.0))
                    .child(Pill::from_spec(PillSpec::new().with_label("Published"), theme))
                    .child(Pill::from_spec(PillSpec::new().with_label("Image"), theme)),
            ),
    );

    let media_section = Surface::from_spec(
        SurfaceSpec::new().with_tone(SurfaceTone::Elevated),
        theme,
    )
    .with_content(
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(div().text_sm().child("Media"))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Preview area for the selected asset."),
            ),
    );

    let detail_content = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(overview_section)
        .child(Separator::from_spec(SeparatorSpec::new(), theme))
        .child(media_section);

    content = content.child(
        DetailShell::from_spec(detail_spec, theme).with_content(detail_content),
    );

    content
}

fn render_picker_screen(theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(12.0));

    // Picker items — real Checkbox components for selection
    let items = [
        ("hero-banner.png", true),
        ("logo.svg", true),
        ("background.jpg", false),
        ("icon-set.zip", false),
    ];

    for (item, selected) in &items {
        let checkbox_spec = CheckboxSpec::new()
            .with_label(*item)
            .with_checked(*selected);
        content = content.child(Checkbox::from_spec(checkbox_spec, theme).with_id(*item));
    }

    // Selection summary + confirm button
    let text_secondary = theme.resolve_color("color.text.secondary");
    let confirm_spec = ButtonSpec::new()
        .with_variant(ButtonVariant::Primary)
        .with_label("Confirm");
    content = content.child(
        div()
            .flex()
            .justify_between()
            .items_center()
            .pt(px(8.0))
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child("2 items selected"),
            )
            .child(Button::from_spec(confirm_spec, theme).with_id("confirm")),
    );

    content
}

fn render_workspace_screen(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let surface_bg = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color("color.border.default");

    let mut content = div().flex().flex_col().gap(px(0.0));

    // App header — real AppHeader
    let app_header_spec = AppHeaderSpec::new()
        .with_title("Demo Project")
        .with_drag_region(true);
    let branch_label = div()
        .text_xs()
        .text_color(color_to_hsla(text_secondary))
        .child("main branch");
    content = content.child(
        AppHeader::from_spec(app_header_spec, theme).with_utility_items(branch_label),
    );

    // Split view: left dock panel + main panel (using raw divs since PanelSurface was removed)
    let left_dock_content = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .p(px(8.0))
        .bg(color_to_hsla(surface_bg))
        .border_r_1()
        .border_color(color_to_hsla(border))
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child("EXPLORER"),
        )
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_primary))
                .child("src/"),
        )
        .child(
            div()
                .text_sm()
                .pl(px(12.0))
                .text_color(color_to_hsla(text_secondary))
                .child("main.rs"),
        )
        .child(
            div()
                .text_sm()
                .pl(px(12.0))
                .text_color(color_to_hsla(text_secondary))
                .child("lib.rs"),
        );

    let main_panel_content = div()
        .flex_1()
        .flex()
        .items_center()
        .justify_center()
        .bg(color_to_hsla(surface_bg))
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("Panel content area"),
        );

    content = content.child(
        div()
            .flex()
            .flex_1()
            .min_h(px(200.0))
            .child(div().w(px(180.0)).child(left_dock_content))
            .child(div().flex_1().child(main_panel_content)),
    );

    // Status bar — real StatusBar
    let status_spec = ShellStatusBarSpec::new().with_summary("Ready");
    let trailing = div()
        .text_xs()
        .text_color(color_to_hsla(text_secondary))
        .child("Ln 42, Col 18");
    content = content.child(
        StatusBar::from_spec(status_spec, theme).with_trailing_items(trailing),
    );

    content
}
