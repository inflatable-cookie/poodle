use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{
    ContextMenu, Eyebrow, Icon, IntoCompatNode, ListCard, ListCardCounter, Pill, StatusIndicator,
};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_specs::{
    ContextMenuSpec, EyebrowSpec, IconSize, IconSpec, InlineTypographyMode, LeadingFill,
    LeadingShape, ListCardCounterSpec, ListCardLayout, ListCardSpec, MenuEntry, MenuItemKind,
    PillSpec, PillTone, SelectionIndicator, StatusIndicatorSpec, StatusTone,
};

fn node_row(gap: f32) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = gap;
    row
}

fn card_click(state: &AppState, value: &'static str) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "list-card-clicked".to_string(),
            value: value.to_string(),
        });
    })
}

fn context_menu_select(state: &AppState) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "list-card-clicked".to_string(),
            value: format!("Action: {value}"),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_muted = theme.resolve_color("color.text.muted");
    let footer_counter_gap = theme.resolve_space("space.inline.md");

    let last_clicked = state.specimens.text.get("list-card-clicked").cloned();
    let examples = div()
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
                            .on_click(card_click(state, "design-system-v2.figma")),
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
                            .on_click(card_click(state, "component-specs.pdf")),
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
                            .on_click(card_click(state, "brand-assets.zip")),
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
                            .with_leading(StatusIndicator::node_from_spec(status, theme))
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
                            .with_leading(StatusIndicator::node_from_spec(status, theme))
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
                            .with_leading(StatusIndicator::node_from_spec(status, theme))
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Down")
                                    .with_tone(PillTone::Danger),
                                theme,
                            ))
                        }),
                ),
        )
        // -- With corner (header-corner slot, tertiary color) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With corner (header-corner slot)"),
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
                                    .with_title("Pipeline config")
                                    .with_subtitle("Build and deploy steps")
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_interactive(true),
                                theme,
                            )
                            // Corner: supplementary header-corner content (icon + label),
                            // tertiary-colored, top-right in the header row.
                            .with_corner(
                                node_row(4.0)
                                    .child(
                                        Icon::from_spec(
                                            IconSpec::new("git-branch").with_size(IconSize::Sm),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    )
                                    .child(Node::text("v2.1")),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Release candidate")
                                    .with_subtitle("Header corner shows recency")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_corner(
                                node_row(4.0)
                                    .child(
                                        Icon::from_spec(
                                            IconSpec::new("clock").with_size(IconSize::Sm),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    )
                                    .child(Node::text("2d")),
                            ),
                        ),
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
                    div().text_size(px(20.0)).child(
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
                            node_row(footer_counter_gap)
                                .child(
                                    ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("eye", 128)
                                            .with_typography(InlineTypographyMode::Inherit),
                                        theme,
                                    )
                                    .into_compat_node(),
                                )
                                .child(
                                    ListCardCounter::from_spec(
                                        ListCardCounterSpec::new("message-circle", 14)
                                            .with_typography(InlineTypographyMode::Inherit),
                                        theme,
                                    )
                                    .into_compat_node(),
                                ),
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
                                node_row(footer_counter_gap)
                                    .child(
                                        ListCardCounter::from_spec(
                                            ListCardCounterSpec::new("eye", 12)
                                                .with_tooltip("12 views"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    )
                                    .child(
                                        ListCardCounter::from_spec(
                                            ListCardCounterSpec::new("share", 3)
                                                .with_tooltip("3 shares"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    ),
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
                                node_row(footer_counter_gap)
                                    .child(
                                        ListCardCounter::from_spec(
                                            ListCardCounterSpec::new("eye", 156)
                                                .with_tooltip("156 reads"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    )
                                    .child(
                                        ListCardCounter::from_spec(
                                            ListCardCounterSpec::new("pencil", 24)
                                                .with_tooltip("24 edits"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    )
                                    .child(
                                        ListCardCounter::from_spec(
                                            ListCardCounterSpec::new("message-circle", 8)
                                                .with_tooltip("8 comments")
                                                .with_href("#comments"),
                                            theme,
                                        )
                                        .on_link_click(|| {})
                                        .into_compat_node(),
                                    ),
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
                    .on_select(context_menu_select(state)),
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
                            .on_click(card_click(state, "Free tier plan")),
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
                            .on_click(card_click(state, "Premium integration")),
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
                            .on_click(card_click(state, "Legacy connector")),
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
        // -- Link roots (href navigation) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Link roots (href)"),
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
                                    .with_title("Billing settings")
                                    .with_subtitle("Manage invoices and payment methods")
                                    .with_href("#billing"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("credit-card").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new().with_label("2").with_tone(PillTone::Neutral),
                                theme,
                            )),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Documentation portal")
                                    .with_subtitle("Opens the external guide")
                                    .with_meta("docs.example.com")
                                    .with_href("https://example.com/docs"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("book-open").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        ),
                ),
        )
        // -- Highlighted (accent emphasis) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Highlighted (accent emphasis)"),
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
                                    .with_title("Active selection")
                                    .with_subtitle("Accent-tinted border and inset ring")
                                    .with_meta("Now")
                                    .with_interactive(true)
                                    .with_highlighted(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("star").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Highlighted with custom accent")
                                    .with_subtitle("Accent gradient over the fill")
                                    .with_interactive(true)
                                    .with_highlighted(true)
                                    .with_accent_color("#6366f1"),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("sparkles").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        ),
                ),
        )
        // -- Active: the card you are currently on --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Active card"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(ListCard::from_spec(
                            ListCardSpec::new()
                                .with_title("a private consumer Build Test")
                                .with_subtitle("Registered 28/07/2026 16:35")
                                .with_interactive(true)
                                .with_active(true),
                            theme,
                        ))
                        .child(ListCard::from_spec(
                            ListCardSpec::new()
                                .with_title("a private consumer Build Test")
                                .with_subtitle("Registered 28/07/2026 16:35")
                                .with_interactive(true),
                            theme,
                        ))
                        // Orthogonal to selection, so both can be true at once.
                        .child(ListCard::from_spec(
                            ListCardSpec::new()
                                .with_title("Active and selected")
                                .with_subtitle("Both states at once")
                                .with_interactive(true)
                                .with_selectable(true)
                                .with_selected(true)
                                .with_active(true),
                            theme,
                        )),
                ),
        )
        // -- Selection indicator (checkbox) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Selection indicator (checkbox)"),
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
                                    .with_title("Selected row")
                                    .with_subtitle("Checkbox indicator, checked")
                                    .with_selectable(true)
                                    .with_selected(true)
                                    .with_selection_indicator(SelectionIndicator::Checkbox),
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
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Unselected row")
                                    .with_subtitle("Checkbox indicator, unchecked")
                                    .with_selectable(true)
                                    .with_selection_indicator(SelectionIndicator::Checkbox),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("file-text").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        ),
                ),
        )
        // -- Layout (default / compact / stacked) --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Layout (default / compact / stacked)"),
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
                                    .with_title("Default layout")
                                    .with_subtitle("Standard leading + body row")
                                    .with_meta("2.0rem")
                                    .with_layout(ListCardLayout::Default)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("rows").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Compact layout")
                                    .with_subtitle("Denser — smaller leading box")
                                    .with_meta("1.75rem")
                                    .with_layout(ListCardLayout::Compact)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("rows").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
                            ),
                        )
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Stacked layout")
                                    .with_subtitle("Leading on top, body below, bottom rail")
                                    .with_layout(ListCardLayout::Stacked)
                                    .with_leading_shape(LeadingShape::RoundedSquare)
                                    .with_leading_fill(LeadingFill::Solid)
                                    .with_accent_color("#6366f1")
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("layers").with_size(IconSize::Md),
                                    theme,
                                )
                                .with_color(gpui::white()),
                            )
                            .with_trailing(Pill::from_spec(
                                PillSpec::new()
                                    .with_label("Published")
                                    .with_tone(PillTone::Success),
                                theme,
                            )),
                        ),
                ),
        )
        // -- Leading size offset --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Leading size offset"),
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
                                    .with_title("Default leading")
                                    .with_subtitle("Matches the card size ladder")
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
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Offset leading (+1 step)")
                                    .with_subtitle("Leading block steps up by 0.25rem")
                                    .with_leading_size_offset(1)
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
                        .child(
                            ListCard::from_spec(
                                ListCardSpec::new()
                                    .with_title("Offset leading (+2 steps)")
                                    .with_subtitle("Larger leading box, same typography")
                                    .with_leading_size_offset(2)
                                    .with_interactive(true),
                                theme,
                            )
                            .with_leading(
                                Icon::from_spec(
                                    IconSpec::new("file-text").with_size(IconSize::Md),
                                    theme,
                                )
                                .with_color(color_to_hsla(text_muted)),
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
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "list-card",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                ListCard::from_spec(
                    ListCardSpec::new()
                        .with_title("Workspace settings")
                        .with_subtitle("Shared defaults and access controls")
                        .with_interactive(true)
                        .with_size(size),
                    theme,
                )
                .with_leading(Icon::from_spec(
                    IconSpec::new("folder").with_size(IconSize::Sm),
                    theme,
                ))
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                ListCard::from_spec(
                    ListCardSpec::new()
                        .with_title("Workspace settings")
                        .with_subtitle("Shared defaults and access controls")
                        .with_interactive(true)
                        .with_density(density),
                    theme,
                )
                .with_leading(Icon::from_spec(
                    IconSpec::new("folder").with_size(IconSize::Sm),
                    theme,
                ))
                .into_any_element()
            }),
    )
}
