//! Demo screen renderer — converts shared demo-contract screens into real Poodle components.
//!
//! The preview-level demo should read like one contract-owned app target rather
//! than a pile of detached specimens, so each screen composes real Poodle
//! primitives and composites into a coherent shell story.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{
    ActionDiscoveryPanel, AppHeader, Breadcrumbs, Button, Callout, CommandPalette, DataTable,
    DetailItem, DetailShell, Field, FilterToolbar, MediaPreview, MetaBar, MetaItem, PageHeader,
    PaginationSummary, PickerShell, Pill, Progress, RelationPicker, SelectionSummary, Separator,
    SidebarNav, SplitView, StatusBar, Surface, TextInput,
};
use poodle_specs::{
    ActionDiscoveryPanelSpec, ActionDiscoverySection, AppHeaderSpec, BreadcrumbItem,
    BreadcrumbsSpec, BrowseState, ButtonSpec, ButtonVariant, CallOutSpec, CommandActionItem,
    CommandPaletteSpec, DataTableSpec, DetailItemSpec, DetailShellSpec, FieldSpec,
    FilterToolbarSpec, MediaKind, MediaPreviewSpec, MediaState, MetaBarSpec, MetaItemSpec,
    PageHeaderSpec, PaginationSummarySpec, PickerItemSpec, PickerShellSpec, PillSpec, ProgressSpec,
    RelationPickerSpec, RemediationAction, SelectionSummaryItem, SelectionSummarySpec,
    SeparatorSpec, ShellStatusBarSpec, SidebarNavGroup, SidebarNavItem, SidebarNavSpec,
    SplitOrientation, SplitViewSpec, StatusTone, SurfaceSpec, SurfaceTone, TableColumnSpec,
    TableRowSpec, TextInputSpec,
};

use crate::app_state::DemoScreen;
use crate::style_bridge::color_to_hsla;

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

fn render_overview_shell(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let success = theme.resolve_color("color.status.success");
    let warning = theme.resolve_color("color.status.warning");
    let danger = theme.resolve_color("color.status.danger");

    let callout_spec = CallOutSpec::new()
        .with_tone(StatusTone::Info)
        .with_title("Welcome")
        .with_content("Your workspace is ready. 3 items need attention.");

    let tiles = [
        ("Active projects", "12", success),
        ("Pending reviews", "3", warning),
        ("Open issues", "27", danger),
    ];

    let mut tile_row = div().flex().gap(px(8.0));
    for (label, value, status_color) in &tiles {
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
                Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
                    .with_content(tile_content),
            ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(Callout::from_spec(callout_spec, theme))
        .child(tile_row)
        .child(
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
        )
}

fn render_form_screen(theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(12.0));

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

    content
        .child(Callout::from_spec(
            CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_content("Please correct the errors above"),
            theme,
        ))
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .justify_end()
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Cancel"),
                        theme,
                    )
                    .with_id("cancel"),
                )
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Submit"),
                        theme,
                    )
                    .with_id("submit"),
                ),
        )
}

fn render_browse_screen(theme: &GpuiThemeProvider) -> Div {
    let filter_toolbar = FilterToolbar::from_spec(
        FilterToolbarSpec::new()
            .with_aria_label("Browse filters")
            .with_summary_text("Showing 4 of 142 assets")
            .with_collapsed(false)
            .with_columns(3),
        theme,
    )
    .with_child(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("Search assets...")
                .with_leading_icon("search")
                .with_id("browse-search"),
            theme,
        )
        .with_id("browse-search"),
    )
    .with_child(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("Owner")
                .with_id("browse-owner"),
            theme,
        )
        .with_id("browse-owner"),
    )
    .with_child(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("Published")
                .with_id("browse-status"),
            theme,
        )
        .with_id("browse-status"),
    );

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

    let selection = SelectionSummary::from_spec(
        SelectionSummarySpec::new(vec![
            SelectionSummaryItem::new("hero-banner", "hero-banner.png").with_meta("Image"),
            SelectionSummaryItem::new("config", "config.json").with_meta("Config"),
        ])
        .with_clear_action(RemediationAction::new("clear", "Clear selection"))
        .with_max_visible_items(2),
        theme,
    );

    let companion = Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
        .with_content(
            div()
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(div().text_sm().child("Selection overview"))
                .child(selection)
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                        .child("Bulk actions, pagination, and visible-scope selection stay attached to the same browse story."),
                ),
        );

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(filter_toolbar)
        .child(
            SplitView::from_spec(
                SplitViewSpec::new(SplitOrientation::Horizontal).with_default_ratio(0.72),
                theme,
            )
            .with_primary(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(DataTable::from_spec(
                        DataTableSpec::new(columns, rows).with_aria_label("Asset browser"),
                        theme,
                    ))
                    .child(PaginationSummary::from_spec(
                        PaginationSummarySpec::new(1, 25, 142),
                        theme,
                    )),
            )
            .with_secondary(companion),
        )
}

fn render_detail_screen(theme: &GpuiThemeProvider) -> Div {
    let breadcrumbs = Breadcrumbs::from_spec(
        BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home"),
            BreadcrumbItem::new("assets", "Assets"),
            BreadcrumbItem::new("hero-banner", "hero-banner.png").with_is_current(true),
        ]),
        theme,
    );

    let metadata = MetaBar::from_spec(MetaBarSpec::new().with_aria_label("Asset metadata"), theme)
        .with_child(
            MetaItem::from_spec(MetaItemSpec::new().with_label("ID"), theme)
                .with_value(div().child("asset_01JY3F")),
        )
        .with_child(Pill::from_spec(
            PillSpec::new().with_label("Published"),
            theme,
        ))
        .with_child(
            MetaItem::from_spec(MetaItemSpec::new().with_label("Updated"), theme)
                .with_value(div().child("2 hours ago")),
        );

    let header = PageHeader::from_spec(
        PageHeaderSpec::new("hero-banner.png")
            .with_section("Media library")
            .with_subtitle("Homepage hero image with downstream references")
            .with_back("/assets", "Assets"),
        theme,
    )
    .with_breadcrumbs(breadcrumbs)
    .with_meta(metadata)
    .with_actions(
        div()
            .flex()
            .gap(px(8.0))
            .child(Button::from_spec(
                ButtonSpec::new().with_label("Replace"),
                theme,
            ))
            .child(Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Archive"),
                theme,
            )),
    );

    let overview_section =
        Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(div().text_sm().child("Overview"))
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                            .child("Image file uploaded 2 hours ago. 1920x1080 pixels, 2.4 MB."),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Owner").with_value("Clay"),
                                theme,
                            ))
                            .child(DetailItem::from_spec(
                                DetailItemSpec::new("Collection").with_value("Homepage"),
                                theme,
                            )),
                    ),
            );

    let related_summary = SelectionSummary::from_spec(
        SelectionSummarySpec::new(vec![
            SelectionSummaryItem::new("homepage", "Homepage").with_meta("Page"),
            SelectionSummaryItem::new("launch", "Launch campaign").with_meta("Collection"),
        ])
        .with_max_visible_items(2),
        theme,
    );

    div().flex().flex_col().gap(px(12.0)).child(header).child(
        DetailShell::from_spec(DetailShellSpec::new().with_title("hero-banner.png"), theme)
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(overview_section)
                    .child(Separator::from_spec(SeparatorSpec::new(), theme))
                    .child(
                        Surface::from_spec(
                            SurfaceSpec::new().with_tone(SurfaceTone::Elevated),
                            theme,
                        )
                        .with_content(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(div().text_sm().child("Related data"))
                                .child(related_summary),
                        ),
                    ),
            ),
    )
}

fn render_picker_screen(theme: &GpuiThemeProvider) -> Div {
    let relation_picker = RelationPicker::from_spec(
        RelationPickerSpec::new(vec![
            PickerItemSpec::new("hero-banner", "hero-banner.png")
                .with_description("Homepage hero image")
                .with_meta("Image"),
            PickerItemSpec::new("logo", "logo.svg")
                .with_description("Brand lockup")
                .with_meta("Vector"),
            PickerItemSpec::new("background", "background.jpg")
                .with_description("Campaign background")
                .with_meta("Image"),
        ])
        .with_selected_ids(vec!["hero-banner".to_string(), "logo".to_string()])
        .with_query("hero")
        .with_state(BrowseState::Ready),
        theme,
    );

    let picker_shell = PickerShell::from_spec(
        PickerShellSpec::new("Choose media")
            .with_description("Attach one or more assets to the current campaign.")
            .with_selected_count(2)
            .with_result_count(3)
            .with_state(BrowseState::Ready),
        theme,
    )
    .with_search(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("Search media...")
                .with_leading_icon("search")
                .with_id("picker-search"),
            theme,
        )
        .with_id("picker-search"),
    )
    .with_results(relation_picker)
    .with_actions(
        div()
            .flex()
            .gap(px(8.0))
            .child(Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Cancel"),
                theme,
            ))
            .child(Button::from_spec(
                ButtonSpec::new().with_label("Attach 2 items"),
                theme,
            )),
    );

    let preview = MediaPreview::from_spec(
        MediaPreviewSpec::new(MediaKind::Image, "hero-banner.png")
            .with_state(MediaState::Ready)
            .with_description("Current featured asset selection.")
            .with_thumbnail_meta("1920x1080")
            .with_metadata(vec!["2.4 MB".to_string(), "Published".to_string()])
            .with_footer_actions(vec![
                RemediationAction::new("replace", "Replace"),
                RemediationAction::new("remove", "Remove"),
            ]),
        theme,
    )
    .with_media_content(
        Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
            .with_content(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_h(px(180.0))
                    .child("Preview area"),
            ),
    );

    div().flex().flex_col().gap(px(12.0)).child(
        SplitView::from_spec(
            SplitViewSpec::new(SplitOrientation::Horizontal).with_default_ratio(0.6),
            theme,
        )
        .with_primary(picker_shell)
        .with_secondary(preview),
    )
}

fn render_workspace_screen(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let surface_bg = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color("color.border.default");

    let app_header_spec = AppHeaderSpec::new()
        .with_title("Demo Project")
        .with_drag_region(true);
    let branch_label = div()
        .text_xs()
        .text_color(color_to_hsla(text_secondary))
        .child("main branch");

    let shell_nav = SidebarNav::from_spec(
        SidebarNavSpec::new(vec![SidebarNavGroup::new(
            "workspace",
            vec![
                SidebarNavItem::new("review", "Review"),
                SidebarNavItem::new("history", "History"),
                SidebarNavItem::new("settings", "Settings"),
            ],
        )
        .with_label("Workspace")])
        .with_value("review")
        .with_aria_label("Workspace navigation"),
        theme,
    );

    let actions = vec![
        CommandActionItem::new("open-palette", "Open palette")
            .with_group("Navigation")
            .with_shortcut("Cmd+K"),
        CommandActionItem::new("rerun-validation", "Rerun validation")
            .with_group("Workspace")
            .with_badge("recent"),
        CommandActionItem::new("toggle-inspector", "Toggle inspector")
            .with_group("Panels")
            .with_shortcut("Cmd+I"),
    ];

    let discovery = ActionDiscoveryPanel::from_spec(
        ActionDiscoveryPanelSpec::new(vec![ActionDiscoverySection::new(
            "navigation",
            "Navigation",
            actions.clone(),
        )
        .with_description("Shared shell commands")]),
        theme,
    );

    let command_palette = CommandPalette::from_spec(
        CommandPaletteSpec::new(actions)
            .with_title("Command palette")
            .with_description("Grouped action discovery for the active workspace.")
            .with_query("open")
            .with_active_action_id("open-palette"),
        theme,
    );

    let main_panel = Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
        .with_content(
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .p(px(12.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("Primary content"),
                )
                .child(discovery)
                .child(command_palette),
        );

    let status_spec = ShellStatusBarSpec::new().with_summary("Ready");
    let trailing = div()
        .text_xs()
        .text_color(color_to_hsla(text_secondary))
        .child("Ln 42, Col 18");

    div()
        .flex()
        .flex_col()
        .gap(px(0.0))
        .child(AppHeader::from_spec(app_header_spec, theme).with_utility_items(branch_label))
        .child(
            SplitView::from_spec(
                SplitViewSpec::new(SplitOrientation::Horizontal).with_default_ratio(0.24),
                theme,
            )
            .with_primary(
                div()
                    .min_h(px(240.0))
                    .bg(color_to_hsla(surface_bg))
                    .border_r_1()
                    .border_color(color_to_hsla(border))
                    .child(shell_nav),
            )
            .with_secondary(main_panel),
        )
        .child(StatusBar::from_spec(status_spec, theme).with_trailing_items(trailing))
}
