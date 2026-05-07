use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{ContextMenu, Eyebrow, Icon, ListCard, ListCardCounter, Pill, StatusIndicator};
use poodle_specs::{
    ContextMenuSpec, EyebrowSpec, IconSize, IconSpec, InlineTypographyMode, LeadingFill,
    LeadingShape, ListCardCounterSpec, ListCardSpec, MenuEntry, MenuItemKind, PillSpec, PillTone,
    StatusIndicatorSpec, StatusTone,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_muted = theme.resolve_color("color.text.muted");
    let footer_counter_gap = px(theme.resolve_space("space.inline.md"));

    let last_clicked = state.specimens.text.get("list-card-clicked").cloned();

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(440.0))
        // -- Interactive list cards --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Interactive list cards"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("design-system-v2.figma")
                                    .with_subtitle("Updated by Clay \u{00b7} 2h ago")
                                    .with_meta("14.2 MB")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "design-system-v2.figma".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("component-specs.pdf")
                                    .with_subtitle("Shared with team \u{00b7} Yesterday")
                                    .with_meta("2.8 MB")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "component-specs.pdf".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("brand-assets.zip")
                                    .with_subtitle("Archived")
                                    .with_meta("48 MB")
                                    .with_disabled(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("folder").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "brand-assets.zip".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
                        ),
                ),
        )
        // -- Rounded-square leading --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Rounded-square leading (thumbnails)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("hero-banner.png")
                                    .with_subtitle("Uploaded by Jamie \u{00b7} 4h ago")
                                    .with_meta("3.1 MB")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("layout-grid").with_size(IconSize::Md),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("onboarding-flow.mp4")
                                    .with_subtitle("Screen recording \u{00b7} Today")
                                    .with_meta("128 MB")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("image").with_size(IconSize::Md),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        ),
                ),
        )
        // -- With badges --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With badges"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child({
                            let mut status =
                                StatusIndicatorSpec::new().with_status(StatusTone::Success);
                            status.aria_label = Some("Active".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("API Server")
                                    .with_subtitle("Running on port 8080"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Active")
                                    .with_tone(PillTone::Success),
                                theme,
                            ))
                        })
                        .child({
                            let mut status =
                                StatusIndicatorSpec::new().with_status(StatusTone::Warning);
                            status.aria_label = Some("Degraded".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Background Worker")
                                    .with_subtitle("High queue depth"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Degraded")
                                    .with_tone(PillTone::Warning),
                                theme,
                            ))
                        })
                        .child({
                            let mut status =
                                StatusIndicatorSpec::new().with_status(StatusTone::Danger);
                            status.aria_label = Some("Down".to_string());
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Database Replica")
                                    .with_subtitle("Connection timeout"),
                                theme,
                            )
                            .with_leading(StatusIndicator::from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Down")
                                    .with_tone(PillTone::Danger),
                                theme,
                            ))
                        }),
                ),
        )
        // -- Footer counters with inherited typography --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Footer counters with inherited typography"),
                    theme,
                ))
                .child(
                    div()
                        .text_size(px(20.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Activity Feed")
                                    .with_subtitle("Inline metadata scales with the parent"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("activity").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_footer(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(footer_counter_gap)
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("eye", 128)
                                            .with_typography(InlineTypographyMode::Inherit),
                                        theme,
                                    ))
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("message-circle", 14)
                                            .with_typography(InlineTypographyMode::Inherit),
                                        theme,
                                    )),
                            ),
                        ),
                ),
        )
        // -- With footer counters --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With footer counters"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_footer(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(footer_counter_gap)
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("eye", 12)
                                            .with_tooltip("12 views"),
                                        theme,
                                    ))
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("share", 3)
                                            .with_tooltip("3 shares"),
                                        theme,
                                    )),
                            ),
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_footer(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(footer_counter_gap)
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("eye", 156)
                                            .with_tooltip("156 reads"),
                                        theme,
                                    ))
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("pencil", 24)
                                            .with_tooltip("24 edits"),
                                        theme,
                                    ))
                                    .child(ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("message-circle", 8)
                                            .with_tooltip("8 comments")
                                            .with_href("#comments"),
                                        theme,
                                    )
                                    .on_link_click(|_ev, _w, _a| {})),
                            ),
                        ),
                ),
        )
        // -- Solid fill with accent colors --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Solid fill with accent colors"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
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
                                )
                                .with_color(gpui::white()),
                            ),
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
                                )
                                .with_color(gpui::white()),
                            ),
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
                                )
                                .with_color(gpui::white()),
                            ),
                        ),
                ),
        )
        // -- With context menu --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With context menu"),
                    theme,
                ))
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
                            )
                            .with_color(color_to_hsla(text_muted)),
                        ),
                    )
                    .on_select(cx.listener(|this, val: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("list-card-clicked".to_string(), format!("Action: {}", val));
                        cx.notify();
                    })),
                ),
        )
        // -- Not live (draft state) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Not live (dashed border, interactive)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Draft")
                                    .with_tone(PillTone::Neutral),
                                theme,
                            )),
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Scheduled")
                                    .with_tone(PillTone::Info),
                                theme,
                            )),
                        ),
                ),
        )
        // -- Corner sash badges --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Corner sash badges"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "Free tier plan".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
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
                                )
                                .with_color(gpui::white()),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "Premium integration".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
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
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .on_click(cx.listener(
                                |this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "list-card-clicked".to_string(),
                                        "Legacy connector".to_string(),
                                    );
                                    cx.notify();
                                },
                            )),
                        ),
                ),
        )
        // -- Selectable (multi-select checkbox) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Selectable (multi-select)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Alice Chen")
                                    .with_subtitle("alice@example.com")
                                    .with_selectable(true)
                                    .with_selected(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("user").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Bob Martinez")
                                    .with_subtitle("bob@example.com")
                                    .with_selectable(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("user").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Carol Patel")
                                    .with_subtitle("carol@example.com")
                                    .with_selectable(true)
                                    .with_selected(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("user").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            ),
                        ),
                ),
        )
        // -- Reorder handle --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Reorder handle"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("design-system")
                                    .with_subtitle("Primary rubric")
                                    .with_reorder_handle(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("hash").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("accessibility")
                                    .with_subtitle("WCAG AA baseline")
                                    .with_reorder_handle(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("hash").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            ),
                        ),
                ),
        )
        // -- Static list card --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Static list card"),
                    theme,
                ))
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("System Configuration")
                            .with_subtitle("Read-only \u{2014} managed by admin")
                            .with_meta("v2.1.0"),
                        theme,
                    )
                    .with_leading(
                        Icon::from_spec(IconSpec::new("settings").with_size(IconSize::Sm), theme)
                            .with_color(color_to_hsla(text_muted)),
                    ),
                ),
        )
        // -- Last clicked indicator --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Last click"),
                    theme,
                ))
                .child(
                    div().flex().items_center().gap(px(6.0)).child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child(match last_clicked {
                                Some(ref name) => format!("Last clicked: {}", name),
                                None => "Click an interactive card above.".to_string(),
                            }),
                    ),
                ),
        )
}
