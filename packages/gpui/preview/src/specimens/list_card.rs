use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{
    ListCardSpec, LeadingShape, LeadingFill,
    StatusIndicatorSpec, StatusTone, PillSpec, PillTone,
    IconSpec, IconSize, ContextMenuSpec, MenuEntry, MenuItemKind,
    EyebrowSpec,
};
use poodle_gpui_components::{ListCard, StatusIndicator, Pill, Icon, ContextMenu, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_muted = theme.resolve_color("semantic.color.text.muted");

    let last_clicked = state.specimens.text.get("list-card-clicked").cloned();

    div().flex().flex_col().gap(px(24.0)).max_w(px(440.0))
        // -- Interactive list cards --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Interactive list cards"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Project Alpha")
                                    .with_subtitle("Last updated 2 hours ago")
                                    .with_meta("12 items")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Project Alpha".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Project Beta")
                                    .with_subtitle("Last updated yesterday")
                                    .with_meta("3 items")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Project Beta".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Project Gamma")
                                    .with_subtitle("Created last week")
                                    .with_meta("28 items")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Project Gamma".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                )
        )

        // -- Rounded-square leading --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Rounded-square leading (thumbnails)"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Design System")
                                    .with_subtitle("Component library")
                                    .with_leading_shape(LeadingShape::RoundedSquare),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("layout-grid").with_size(IconSize::Md),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Photo Album")
                                    .with_subtitle("48 photos")
                                    .with_leading_shape(LeadingShape::RoundedSquare),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("image").with_size(IconSize::Md),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                        )
                )
        )

        // -- With badges --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With badges"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child({
                            let mut status = StatusIndicatorSpec::new().with_status(StatusTone::Success);
                            status.aria_label = Some("Active".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("API Server")
                                    .with_subtitle("Running on port 8080"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("Active").with_tone(PillTone::Success),
                                theme,
                            ))
                        })
                        .child({
                            let mut status = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
                            status.aria_label = Some("Degraded".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Background Worker")
                                    .with_subtitle("High queue depth"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("Degraded").with_tone(PillTone::Warning),
                                theme,
                            ))
                        })
                        .child({
                            let mut status = StatusIndicatorSpec::new().with_status(StatusTone::Danger);
                            status.aria_label = Some("Down".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Database Replica")
                                    .with_subtitle("Connection timeout"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("Down").with_tone(PillTone::Danger),
                                theme,
                            ))
                        })
                )
        )

        // -- With footer counters --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With footer counters"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Dashboard")
                                    .with_subtitle("Analytics overview"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("bar-chart").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .with_footer(
                                div().flex().gap(px(12.0))
                                    .child(counter_item("12 views", "eye", text_muted, theme))
                                    .child(counter_item("3 shares", "share", text_muted, theme))
                            )
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("User Guide")
                                    .with_subtitle("Documentation"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("book").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .with_footer(
                                div().flex().gap(px(12.0))
                                    .child(counter_item("156 reads", "eye", text_muted, theme))
                                    .child(counter_item("24 edits", "pencil", text_muted, theme))
                                    .child(counter_item("8 comments", "message-circle", text_muted, theme))
                            )
                        )
                )
        )

        // -- Solid fill with accent colors --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Solid fill with accent colors"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Design Tokens")
                                    .with_subtitle("Color system")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_leading_fill(LeadingFill::Solid)
                                    .with_accent_color("#6366f1"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("palette").with_size(IconSize::Md),
                                    theme,
                                ).with_color(gpui::white())
                            )
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Typography")
                                    .with_subtitle("Font scales & families")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_leading_fill(LeadingFill::Solid)
                                    .with_accent_color("#ec4899"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("type").with_size(IconSize::Md),
                                    theme,
                                ).with_color(gpui::white())
                            )
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Spacing")
                                    .with_subtitle("Layout grid & spacing tokens")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_leading_fill(LeadingFill::Solid)
                                    .with_accent_color("#10b981"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("ruler").with_size(IconSize::Md),
                                    theme,
                                ).with_color(gpui::white())
                            )
                        )
                )
        )

        // -- With context menu --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With context menu"), theme))
                .child(
                    ContextMenu::from_spec(
                        ContextMenuSpec::new(vec![
                            MenuEntry::new("open", "Open"),
                            MenuEntry::new("rename", "Rename"),
                            MenuEntry::new("duplicate", "Duplicate"),
                            MenuEntry::new("sep", "").with_kind(MenuItemKind::Separator),
                            MenuEntry::new("delete", "Delete"),
                        ])
                        .with_open(true),
                        theme,
                    )
                    .with_trigger(
                        ListCard::from_spec(
                            ListCardSpec::new()
                                .with_title("Right-click for actions")
                                .with_subtitle("Context menu on the whole card")
                                .with_meta("12 KB")
                                .with_interactive(true),
                            theme,
                        )
                        .with_leading(
                            Icon::from_spec(
                                IconSpec::new("file-text").with_size(IconSize::Sm),
                                theme,
                            ).with_color(color_to_hsla(text_muted))
                        )
                    )
                    .on_select(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert(
                            "list-card-clicked".to_string(),
                            format!("Action: {}", val),
                        );
                        cx.notify();
                    }))
                )
        )

        // -- Not live (draft state) --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Not live (dashed border, interactive)"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Unpublished Draft")
                                    .with_subtitle("Last edited 3 days ago")
                                    .with_not_live(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("file-text").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("Draft").with_tone(PillTone::Neutral),
                                theme,
                            ))
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Scheduled Post")
                                    .with_subtitle("Publishes tomorrow at 9 AM")
                                    .with_not_live(true)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("clock").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("Scheduled").with_tone(PillTone::Info),
                                theme,
                            ))
                        )
                )
        )

        // -- Corner sash badges --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Corner sash badges"), theme))
                .child(
                    div().flex().flex_col().gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Free tier plan")
                                    .with_subtitle("No credit card required")
                                    .with_sash("Free")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("layers").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Free tier plan".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Premium integration")
                                    .with_subtitle("Unlocks advanced features")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_leading_fill(LeadingFill::Solid)
                                    .with_accent_color("#6366f1")
                                    .with_sash("New")
                                    .with_sash_color("#6366f1")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("grid-2x2").with_size(IconSize::Md),
                                    theme,
                                ).with_color(gpui::white())
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Premium integration".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Legacy connector")
                                    .with_subtitle("Deprecated \u{2014} migrate by Q2")
                                    .with_sash("EOL")
                                    .with_sash_color("#ef4444")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("file-text").with_size(IconSize::Sm),
                                    theme,
                                ).with_color(color_to_hsla(text_muted))
                            )
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert(
                                    "list-card-clicked".to_string(),
                                    "Legacy connector".to_string(),
                                );
                                cx.notify();
                            }))
                        )
                )
        )

        // -- Static list card --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Static list card"), theme))
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("System Configuration")
                            .with_subtitle("Read-only \u{2014} managed by admin")
                            .with_meta("v2.1.0"),
                        theme,
                    )
                    .with_leading(
                        Icon::from_spec(
                            IconSpec::new("settings").with_size(IconSize::Sm),
                            theme,
                        ).with_color(color_to_hsla(text_muted))
                    )
                )
        )

        // -- Last clicked indicator --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Last click"), theme))
                .child(
                    div().flex().items_center().gap(px(6.0))
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child(match last_clicked {
                                    Some(ref name) => format!("Last clicked: {}", name),
                                    None => "Click an interactive card above.".to_string(),
                                })
                        )
                )
        )
}

fn counter_item(
    label: &str,
    icon_name: &str,
    color: poodle_tokens::typed::ColorValue,
    theme: &poodle_gpui::GpuiThemeProvider,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(4.0))
        .child(
            Icon::from_spec(
                IconSpec::new(icon_name).with_size(IconSize::Sm),
                theme,
            ).with_color(color_to_hsla(color))
        )
        .child(
            div().text_size(px(11.0)).text_color(color_to_hsla(color))
                .child(label.to_string())
        )
}
