use crate::app_state::AppState;
use crate::node_compat::{Breadcrumbs, Button, Eyebrow, IntoCompatNode, PageHeader, Pill};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_specs::PageHeaderSpec;
use poodle_specs::{
    BreadcrumbItem, BreadcrumbsSpec, ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec,
    PillAppearance, PillSpec, PillTone, StatusTone,
};

fn actions(theme: &GpuiThemeProvider, items: &[(&str, &str)]) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.spacing.gap = 6.0;
    for (id, label) in items {
        row = row.child(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label(*label)
                    .with_size(ControlSize::Sm),
                theme,
            )
            .with_id(*id)
            .into_compat_node(),
        );
    }
    row
}

fn meta(theme: &GpuiThemeProvider) -> Node {
    let secondary = theme.resolve_color("color.text.secondary");
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = 12.0;
    row = row.child(
        Pill::from_spec(
            PillSpec::new()
                .with_label("Active")
                .with_tone(PillTone::Success)
                .with_appearance(PillAppearance::Badge),
            theme,
        )
        .into_compat_node(),
    );
    let mut every = Node::text("Every 6 hours");
    every.style.text_size = Some(13.0);
    every.style.descriptor.text_color = Some(secondary);
    row = row.child(every);
    let mut last = Node::container();
    last.style.descriptor.layout.direction = LayoutDirection::Row;
    last.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    last.style.descriptor.layout.spacing.gap = 4.0;
    let mut last_label = Node::text("Last run");
    last_label.style.text_size = Some(13.0);
    last_label.style.descriptor.text_color = Some(secondary);
    let mut last_value = Node::text("4mo ago");
    last_value.style.text_size = Some(13.0);
    last_value.style.descriptor.text_color = Some(secondary);
    row.child(last.child(last_label).child(last_value))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic"),
                    theme,
                ))
                .child(PageHeader::from_spec(
                    PageHeaderSpec::new("Components")
                        .with_subtitle("Browse and manage your component library."),
                    theme,
                )),
        )
        // --- With back link and actions ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With back link and actions"),
                    theme,
                ))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Media Library")
                            .with_subtitle("Browse, review, and manage uploaded files.")
                            .with_back("/dashboard", "Dashboard"),
                        theme,
                    )
                    .with_actions(actions(
                        theme,
                        &[
                            ("ph-media-upload", "Upload"),
                            ("ph-media-settings", "Settings"),
                        ],
                    )),
                ),
        )
        // --- With eyebrow and actions ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With eyebrow and actions"),
                    theme,
                ))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Button")
                            .with_eyebrow("Primitive")
                            .with_subtitle("Primary interactive control for triggering actions."),
                        theme,
                    )
                    .with_actions(actions(
                        theme,
                        &[("ph-source", "View source"), ("ph-edit", "Edit")],
                    )),
                ),
        )
        // --- With count ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With count"),
                    theme,
                ))
                .child(PageHeader::from_spec(
                    PageHeaderSpec::new("Users")
                        .with_count(128)
                        .with_back("/dashboard", "Dashboard"),
                    theme,
                )),
        )
        // --- Section and banner ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Section and banner"),
                    theme,
                ))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Nightly Sync")
                            .with_section("Scheduled Task")
                            .with_back("/system/tasks", "Tasks")
                            .with_back_is_contextual(true)
                            .with_banner("This task is currently paused.", StatusTone::Warning),
                        theme,
                    )
                    .with_actions(actions(
                        theme,
                        &[("ph-sync-run", "Run now"), ("ph-sync-edit", "Edit")],
                    )),
                ),
        )
        // --- With MetaBar ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With MetaBar"),
                    theme,
                ))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Nightly Sync")
                            .with_section("Scheduled Task")
                            .with_back("/system/tasks", "Tasks"),
                        theme,
                    )
                    .with_meta(meta(theme))
                    .with_actions(actions(
                        theme,
                        &[
                            ("ph-meta-run", "Run now"),
                            ("ph-meta-edit", "Edit schedule"),
                        ],
                    )),
                ),
        )
        // --- Title only ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Title only"),
                    theme,
                ))
                .child(PageHeader::from_spec(
                    PageHeaderSpec::new("Settings"),
                    theme,
                )),
        )
        // --- With breadcrumbs ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With breadcrumbs"),
                    theme,
                ))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Button")
                            .with_subtitle("Primary interactive control for triggering actions."),
                        theme,
                    )
                    .with_breadcrumbs(Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("components", "Components"),
                            BreadcrumbItem::new("primitives", "Primitives"),
                        ]),
                        theme,
                    )),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "page-header",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                PageHeader::from_spec(
                    PageHeaderSpec::new("Media Library")
                        .with_subtitle("Browse, review, and manage uploaded files.")
                        .with_back("/dashboard", "Dashboard")
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                PageHeader::from_spec(
                    PageHeaderSpec::new("Media Library")
                        .with_subtitle("Browse, review, and manage uploaded files.")
                        .with_back("/dashboard", "Dashboard")
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
