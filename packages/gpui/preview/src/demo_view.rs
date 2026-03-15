//! Demo screen renderer — converts adapter demo screens into real gpui elements.
//!
//! Re-renders each component spec through the adapter to get resolved styles,
//! then applies the style bridge to produce actual gpui Div trees.

use gpui::*;
use pug_adapter::{RenderComponent, ThemeProvider};
use pug_gpui::{GpuiAdapter, GpuiThemeProvider};
use pug_style::StyleDescriptor;

use crate::app_state::DemoScreen;
use crate::style_bridge::color_to_hsla;

/// Render a single demo screen by id.
pub fn render_single_screen(theme: &GpuiThemeProvider, screen: DemoScreen) -> Div {
    let adapter = GpuiAdapter::new(theme.clone());
    let style = StyleDescriptor::new();

    let elevated_bg = theme.resolve_color("semantic.color.background.elevated");
    let border = theme.resolve_color("semantic.color.border.default");
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");
    let success = theme.resolve_color("semantic.color.status.success");
    let warning = theme.resolve_color("semantic.color.status.warning");
    let danger = theme.resolve_color("semantic.color.status.danger");

    match screen {
        DemoScreen::OverviewShell => render_overview_shell(
            &adapter, theme, &style, text_primary, text_secondary, accent, success, warning,
            danger, elevated_bg, border,
        ),
        DemoScreen::FormAndValidation => render_form_screen(
            &adapter, theme, &style, text_primary, text_secondary, accent, border, elevated_bg,
            danger,
        ),
        DemoScreen::BrowseAndTable => render_browse_screen(
            &adapter, theme, &style, text_primary, text_secondary, border, elevated_bg,
        ),
        DemoScreen::DetailAndRelatedData => render_detail_screen(
            &adapter, theme, &style, text_primary, text_secondary, border, elevated_bg,
        ),
        DemoScreen::PickerAndMedia => render_picker_screen(
            &adapter, theme, &style, text_primary, text_secondary, accent, border,
        ),
        DemoScreen::CommandAndWorkspace => render_workspace_screen(
            &adapter, theme, &style, text_primary, text_secondary, border, elevated_bg,
        ),
    }
}

// --- Screen renderers ---

fn render_overview_shell(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    _text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    accent: pug_tokens::typed::ColorValue,
    success: pug_tokens::typed::ColorValue,
    warning: pug_tokens::typed::ColorValue,
    danger: pug_tokens::typed::ColorValue,
    elevated_bg: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    // Render through adapter to get resolved styles
    let _surface = adapter.render(&pug_primitives::SurfaceSpec::new(), sd, theme);
    let _surface_style = pug_gpui::map_style(sd);

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Banner row
    let banner_handle = adapter.render(&pug_primitives::BannerSpec::new(), sd, theme);
    content = content.child(
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(12.0))
            .rounded(px(6.0))
            .bg(color_to_hsla(accent).opacity(0.1))
            .border_1()
            .border_color(color_to_hsla(accent).opacity(0.3))
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(accent))
                    .child(format!("Banner — {}", banner_handle.element_id)),
            ),
    );

    // State tiles grid
    let tiles = [
        ("Active projects", "12", success),
        ("Pending reviews", "3", warning),
        ("Open issues", "27", danger),
    ];

    let mut tile_row = div().flex().gap(px(8.0));
    for (label, value, status_color) in &tiles {
        tile_row = tile_row.child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .p(px(12.0))
                .rounded(px(6.0))
                .bg(color_to_hsla(elevated_bg))
                .border_1()
                .border_color(color_to_hsla(border))
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
                ),
        );
    }
    content = content.child(tile_row);

    // Progress bar
    let _progress = adapter.render(&pug_primitives::ProgressSpec::new(), sd, theme);
    content = content.child(
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Progress"),
            )
            .child(
                div()
                    .h(px(6.0))
                    .rounded(px(3.0))
                    .bg(color_to_hsla(border))
                    .child(
                        div()
                            .w_3_4()
                            .h_full()
                            .rounded(px(3.0))
                            .bg(color_to_hsla(accent)),
                    ),
            ),
    );

    content
}

fn render_form_screen(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    accent: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
    elevated_bg: pug_tokens::typed::ColorValue,
    danger: pug_tokens::typed::ColorValue,
) -> Div {
    let _text_input = adapter.render(&pug_primitives::TextInputSpec::new(), sd, theme);
    let _button = adapter.render(&pug_primitives::ButtonSpec::new(), sd, theme);

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Form fields
    let fields = ["Title", "Description", "Category"];
    for field_label in &fields {
        content = content.child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_primary))
                        .child(field_label.to_string()),
                )
                .child(
                    div()
                        .h(px(36.0))
                        .px(px(12.0))
                        .rounded(px(6.0))
                        .bg(color_to_hsla(elevated_bg))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Enter {}...", field_label.to_lowercase())),
                        ),
                ),
        );
    }

    // Validation banner
    content = content.child(
        div()
            .p(px(10.0))
            .rounded(px(6.0))
            .bg(color_to_hsla(danger).opacity(0.1))
            .border_1()
            .border_color(color_to_hsla(danger).opacity(0.3))
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(danger))
                    .child("Please correct the errors above"),
            ),
    );

    // Action buttons
    content = content.child(
        div()
            .flex()
            .gap(px(8.0))
            .justify_end()
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(color_to_hsla(border))
                    .text_sm()
                    .child("Cancel"),
            )
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(px(6.0))
                    .bg(color_to_hsla(accent))
                    .text_color(gpui::white())
                    .text_sm()
                    .child("Submit"),
            ),
    );

    content
}

fn render_browse_screen(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
    elevated_bg: pug_tokens::typed::ColorValue,
) -> Div {
    let _table = adapter.render(&pug_composites::DataTableSpec::new(vec![], vec![]), sd, theme);

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Search bar
    content = content.child(
        div()
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(color_to_hsla(elevated_bg))
            .border_1()
            .border_color(color_to_hsla(border))
            .flex()
            .items_center()
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Search assets..."),
            ),
    );

    // Table header
    let columns = ["Name", "Type", "Status", "Modified"];
    let mut header = div()
        .flex()
        .px(px(8.0))
        .py(px(6.0))
        .border_b_1()
        .border_color(color_to_hsla(border));
    for col in &columns {
        header = header.child(
            div()
                .flex_1()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child(col.to_string()),
        );
    }
    content = content.child(header);

    // Table rows
    let rows = [
        ["hero-banner.png", "Image", "Published", "2 hours ago"],
        ["main.css", "Stylesheet", "Draft", "Yesterday"],
        ["index.html", "Document", "Published", "3 days ago"],
        ["config.json", "Config", "Review", "1 week ago"],
    ];
    for row in &rows {
        let mut row_el = div()
            .flex()
            .px(px(8.0))
            .py(px(8.0))
            .border_b_1()
            .border_color(color_to_hsla(border).opacity(0.5));
        for (i, cell) in row.iter().enumerate() {
            let cell_el = div().flex_1().text_sm();
            let cell_el = if i == 0 {
                cell_el.text_color(color_to_hsla(text_primary))
            } else {
                cell_el.text_color(color_to_hsla(text_secondary))
            };
            row_el = row_el.child(cell_el.child(cell.to_string()));
        }
        content = content.child(row_el);
    }

    // Pagination
    content = content.child(
        div()
            .flex()
            .justify_between()
            .pt(px(8.0))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Showing 1-25 of 142"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Page 1 of 6"),
            ),
    );

    content
}

fn render_detail_screen(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
    elevated_bg: pug_tokens::typed::ColorValue,
) -> Div {
    let _detail = adapter.render(&pug_composites::DetailShellSpec::new(), sd, theme);

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Breadcrumbs
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

    // Detail sections
    let sections = [
        ("Overview", "Image file uploaded 2 hours ago. 1920×1080 pixels, 2.4 MB."),
        ("Media", "Preview area for the selected asset."),
    ];
    for (title, desc) in &sections {
        content = content.child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .p(px(12.0))
                .rounded(px(6.0))
                .bg(color_to_hsla(elevated_bg))
                .border_1()
                .border_color(color_to_hsla(border))
                .child(div().text_sm().child(title.to_string()))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(desc.to_string()),
                ),
        );
    }

    content
}

fn render_picker_screen(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    accent: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    let _picker = adapter.render(&pug_composites::PickerShellSpec::new("Select assets"), sd, theme);

    let mut content = div().flex().flex_col().gap(px(12.0));

    // Picker items
    let items = ["hero-banner.png", "logo.svg", "background.jpg", "icon-set.zip"];
    for (i, item) in items.iter().enumerate() {
        let selected = i < 2;
        let mut row = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(10.0))
            .py(px(8.0))
            .rounded(px(4.0))
            .border_1();

        row = if selected {
            row.border_color(color_to_hsla(accent))
                .bg(color_to_hsla(accent).opacity(0.08))
        } else {
            row.border_color(color_to_hsla(border))
        };

        // Checkbox indicator
        let checkbox = div()
            .w(px(16.0))
            .h(px(16.0))
            .rounded(px(3.0))
            .border_1()
            .flex()
            .items_center()
            .justify_center();

        let checkbox = if selected {
            checkbox
                .bg(color_to_hsla(accent))
                .border_color(color_to_hsla(accent))
                .text_color(gpui::white())
                .text_xs()
                .child("✓")
        } else {
            checkbox.border_color(color_to_hsla(border))
        };

        row = row.child(checkbox).child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_primary))
                .child(item.to_string()),
        );
        content = content.child(row);
    }

    // Selection summary
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
            .child(
                div()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(color_to_hsla(accent))
                    .text_color(gpui::white())
                    .text_sm()
                    .child("Confirm"),
            ),
    );

    content
}

fn render_workspace_screen(
    adapter: &GpuiAdapter,
    theme: &GpuiThemeProvider,
    sd: &StyleDescriptor,
    text_primary: pug_tokens::typed::ColorValue,
    text_secondary: pug_tokens::typed::ColorValue,
    border: pug_tokens::typed::ColorValue,
    elevated_bg: pug_tokens::typed::ColorValue,
) -> Div {
    let _workspace = adapter.render(&pug_workstation::WorkspaceShellSpec::new(), sd, theme);
    let _app_header = adapter.render(&pug_workstation::AppHeaderSpec::new(), sd, theme);

    let mut content = div().flex().flex_col().gap(px(0.0));

    // App header bar
    content = content.child(
        div()
            .h(px(36.0))
            .flex()
            .items_center()
            .px(px(12.0))
            .gap(px(12.0))
            .bg(color_to_hsla(elevated_bg))
            .border_b_1()
            .border_color(color_to_hsla(border))
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_primary))
                    .child("Demo Project"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("main branch"),
            ),
    );

    // Split view: dock + panel
    content = content.child(
        div()
            .flex()
            .flex_1()
            .min_h(px(200.0))
            // Left dock
            .child(
                div()
                    .w(px(180.0))
                    .flex()
                    .flex_col()
                    .bg(color_to_hsla(elevated_bg))
                    .border_r_1()
                    .border_color(color_to_hsla(border))
                    .p(px(8.0))
                    .gap(px(4.0))
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
                    ),
            )
            // Main panel
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_secondary))
                            .child("Panel content area"),
                    ),
            ),
    );

    // Status bar
    content = content.child(
        div()
            .h(px(24.0))
            .flex()
            .items_center()
            .px(px(12.0))
            .gap(px(16.0))
            .bg(color_to_hsla(elevated_bg))
            .border_t_1()
            .border_color(color_to_hsla(border))
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Ready"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Ln 42, Col 18"),
            ),
    );

    content
}
