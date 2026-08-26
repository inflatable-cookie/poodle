use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Tabs};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ActiveEdge, ActiveFill, EyebrowSpec, Orientation, TabDefinition, TabVariant, TabsSpec,
};
use std::sync::Arc;

fn node_value_handler(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

fn node_close_handler(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    node_value_handler(state, key)
}

fn ordered_tabs(base: Vec<TabDefinition>, state: &AppState, order_key: &str) -> Vec<TabDefinition> {
    let Some(order) = state.specimens.text.get(order_key) else {
        return base;
    };
    let wanted = order
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if wanted.is_empty() {
        return base;
    }
    let mut by_value = base
        .into_iter()
        .map(|tab| (tab.value.clone(), tab))
        .collect::<std::collections::BTreeMap<_, _>>();
    wanted
        .into_iter()
        .filter_map(|value| by_value.remove(&value))
        .collect()
}

fn node_order_handler(
    state: &AppState,
    key: &'static str,
) -> Arc<dyn Fn(Vec<String>) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |order: Vec<String>| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: order.join(","),
        });
    })
}

fn node_optional_text_handler(
    state: &AppState,
    key: &'static str,
) -> Arc<dyn Fn(Option<&str>) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value: Option<&str>| {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::SetOptionalText {
                key: key.to_string(),
                value: value.map(str::to_owned),
            });
    })
}

fn node_drag_end_handler(
    state: &AppState,
    drag_key: &'static str,
    drop_key: &'static str,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |_value: &str| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetOptionalText {
            key: drag_key.to_string(),
            value: None,
        });
        events.push(NodeSpecimenEvent::SetOptionalText {
            key: drop_key.to_string(),
            value: None,
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");
    let bg_surface = theme.resolve_color("color.background.surface");

    // 1. CARD VARIANT (DEFAULT, WITH PANEL)
    let basic_card_tabs = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ];

    let basic_card_value = state
        .specimens
        .text
        .get("tabs-card-default-value")
        .map(|s| s.as_str())
        .unwrap_or("overview")
        .to_string();

    let basic_card_spec = TabsSpec::new(basic_card_tabs)
        .with_variant(TabVariant::Card)
        .with_bordered(true)
        .with_value(&basic_card_value)
        .with_aria_label("Section tabs");

    let basic_card_component = Tabs::from_spec(basic_card_spec, theme)
        .with_id("specimen-card-default")
        .on_change(node_value_handler(state, "tabs-card-default-value"))
        .with_content(
            "overview".to_string(),
            div()
                .p(px(12.0))
                .text_size(px(14.0))
                .text_color(color_to_hsla(text_secondary))
                .child(
                    "Overview content — this is the landing page with a summary of all features."
                        .to_string(),
                ),
        )
        .with_content(
            "features".to_string(),
            div()
                .p(px(12.0))
                .text_size(px(14.0))
                .text_color(color_to_hsla(text_secondary))
                .child(
                    "Features content — explore the full feature set and capabilities.".to_string(),
                ),
        )
        .with_content(
            "pricing".to_string(),
            div()
                .p(px(12.0))
                .text_size(px(14.0))
                .text_color(color_to_hsla(text_secondary))
                .child(
                    "Pricing content — compare plans and find the right fit for your team."
                        .to_string(),
                ),
        );

    // 2. CARD VARIANT (CLOSABLE, REORDERABLE)
    let card_tabs = ordered_tabs(
        vec![
            TabDefinition::new("index.ts", "index.ts"),
            TabDefinition::new("App.svelte", "App.svelte").with_closable(true),
            TabDefinition::new("utils.ts", "utils.ts").with_closable(true),
            TabDefinition::new("types.ts", "types.ts").with_closable(true),
        ],
        state,
        "tabs-card-order",
    );

    let card_value = state
        .specimens
        .text
        .get("tabs-card-value")
        .map(|s| s.as_str())
        .unwrap_or("index.ts")
        .to_string();

    let card_spec = TabsSpec::new(card_tabs)
        .with_variant(TabVariant::Card)
        .with_value(&card_value)
        .with_reorderable(true)
        .with_drag_value(state.specimens.text.get("tabs-card-drag").cloned())
        .with_drop_target_value(state.specimens.text.get("tabs-card-drop").cloned())
        .with_aria_label("Open files");

    let card_component = Tabs::from_spec(card_spec, theme)
        .with_id("specimen-card")
        .on_change(node_value_handler(state, "tabs-card-value"))
        .on_close(node_close_handler(state, "tabs-card-closed"))
        .on_reorder(node_order_handler(state, "tabs-card-order"))
        .on_drag_start(node_value_handler(state, "tabs-card-drag"))
        .on_drag_end(node_drag_end_handler(
            state,
            "tabs-card-drag",
            "tabs-card-drop",
        ))
        .on_drop_target_change(node_optional_text_handler(state, "tabs-card-drop"));

    let last_card_closed = state
        .specimens
        .text
        .get("tabs-card-closed")
        .cloned()
        .unwrap_or_default();

    // 2b. CARD VARIANT WITH COUNTS — icons + count badges on each tab.
    let counts_tabs = vec![
        TabDefinition::new("inbox", "Inbox")
            .with_icon("inbox")
            .with_count(12),
        TabDefinition::new("drafts", "Drafts")
            .with_icon("file-text")
            .with_count(3),
        TabDefinition::new("sent", "Sent").with_icon("send"),
        TabDefinition::new("spam", "Spam")
            .with_icon("alert-triangle")
            .with_count(47),
    ];
    let counts_value = state
        .specimens
        .text
        .get("tabs-counts-value")
        .map(|s| s.as_str())
        .unwrap_or("inbox")
        .to_string();
    let counts_spec = TabsSpec::new(counts_tabs)
        .with_variant(TabVariant::Card)
        .with_value(&counts_value)
        .with_aria_label("Mailbox folders");
    let counts_component = Tabs::from_spec(counts_spec, theme)
        .with_id("specimen-card-counts")
        .on_change(node_value_handler(state, "tabs-counts-value"));

    // 2c. CARD VARIANT WITH ACTIVE EDGE OUTLINE — the former card variant's
    // selected border, opted back in via `activeEdge="outline"`.
    let outline_value = state
        .specimens
        .text
        .get("tabs-outline-value")
        .map(|s| s.as_str())
        .unwrap_or("overview")
        .to_string();

    let outline_spec = TabsSpec::new(vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ])
    .with_variant(TabVariant::Card)
    .with_active_edge(ActiveEdge::Outline)
    .with_value(&outline_value)
    .with_aria_label("Outlined section tabs");

    let outline_component = Tabs::from_spec(outline_spec, theme)
        .with_id("specimen-card-outline")
        .on_change(node_value_handler(state, "tabs-outline-value"));

    // 2d. CARD VARIANT WITH SOLID FILL — `activeFill="solid"`: the selected
    // tab is fully accent-filled with an inverse foreground.
    let solid_value = state
        .specimens
        .text
        .get("tabs-solid-value")
        .map(|s| s.as_str())
        .unwrap_or("overview")
        .to_string();

    let solid_spec = TabsSpec::new(vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ])
    .with_variant(TabVariant::Card)
    .with_active_fill(ActiveFill::Solid)
    .with_value(&solid_value)
    .with_aria_label("Solid section tabs");

    let solid_component = Tabs::from_spec(solid_spec, theme)
        .with_id("specimen-card-solid")
        .on_change(node_value_handler(state, "tabs-solid-value"));

    // 3. PILL VARIANT (WITH ICONS)
    let pill_tabs = vec![
        TabDefinition::new("home", "Home").with_icon("home"),
        TabDefinition::new("settings", "Settings").with_icon("settings"),
        TabDefinition::new("users", "Users").with_icon("users"),
    ];

    let pill_value = state
        .specimens
        .text
        .get("tabs-pill-value")
        .map(|s| s.as_str())
        .unwrap_or("home")
        .to_string();

    let pill_spec = TabsSpec::new(pill_tabs)
        .with_variant(TabVariant::Pill)
        .with_value(&pill_value)
        .with_aria_label("Navigation");

    let pill_component = Tabs::from_spec(pill_spec, theme)
        .with_id("specimen-pill")
        .on_change(node_value_handler(state, "tabs-pill-value"));

    // 3b. BLOCK VARIANT (FULL-WIDTH, SEPARATORS)
    let block_tabs = vec![
        TabDefinition::new("inbox", "Inbox"),
        TabDefinition::new("archive", "Archive"),
        TabDefinition::new("spam", "Spam"),
        TabDefinition::new("trash", "Trash"),
    ];

    let block_value = state
        .specimens
        .text
        .get("tabs-block-value")
        .map(|s| s.as_str())
        .unwrap_or("inbox")
        .to_string();

    let block_spec = TabsSpec::new(block_tabs)
        .with_variant(TabVariant::Block)
        .with_value(&block_value)
        .with_aria_label("Mailbox");

    let block_component = Tabs::from_spec(block_spec, theme)
        .with_id("specimen-block")
        .on_change(node_value_handler(state, "tabs-block-value"));

    // 3c. BLOCK VARIANT WITH ACTIVE EDGE OUTLINE — the outline edge on block
    // keeps the border-left separators (per-side overrides) and covers the
    // remaining sides.
    let block_outline_value = state
        .specimens
        .text
        .get("tabs-block-outline-value")
        .map(|s| s.as_str())
        .unwrap_or("inbox")
        .to_string();

    let block_outline_spec = TabsSpec::new(vec![
        TabDefinition::new("inbox", "Inbox"),
        TabDefinition::new("archive", "Archive"),
        TabDefinition::new("spam", "Spam"),
        TabDefinition::new("trash", "Trash"),
    ])
    .with_variant(TabVariant::Block)
    .with_active_edge(ActiveEdge::Outline)
    .with_value(&block_outline_value)
    .with_aria_label("Outlined mailbox");

    let block_outline_component = Tabs::from_spec(block_outline_spec, theme)
        .with_id("specimen-block-outline")
        .on_change(node_value_handler(state, "tabs-block-outline-value"));

    // 3d. BLOCK VARIANT WITH ACTIVE EDGE UNDERLINE + NO FILL — exactly the
    // former strip variant: an accent underline and no selected fill
    // (contract §13: block + activeFill="none" + activeEdge="underline").
    let underline_value = state
        .specimens
        .text
        .get("tabs-underline-value")
        .map(|s| s.as_str())
        .unwrap_or("editor")
        .to_string();

    let underline_spec = TabsSpec::new(ordered_tabs(
        vec![
            TabDefinition::new("editor", "Editor").with_icon("code"),
            TabDefinition::new("preview", "Preview").with_icon("eye"),
            TabDefinition::new("terminal", "Terminal").with_icon("terminal"),
            TabDefinition::new("output", "Output").with_icon("file-text"),
        ],
        state,
        "tabs-underline-order",
    ))
    .with_variant(TabVariant::Block)
    .with_active_edge(ActiveEdge::Underline)
    .with_active_fill(ActiveFill::None)
    .with_value(&underline_value)
    .with_reorderable(true)
    .with_drag_value(state.specimens.text.get("tabs-underline-drag").cloned())
    .with_drop_target_value(state.specimens.text.get("tabs-underline-drop").cloned())
    .with_aria_label("Strip-equivalent workspace surfaces");

    let underline_component = Tabs::from_spec(underline_spec, theme)
        .with_id("specimen-block-underline")
        .on_change(node_value_handler(state, "tabs-underline-value"))
        .on_reorder(node_order_handler(state, "tabs-underline-order"))
        .on_drag_start(node_value_handler(state, "tabs-underline-drag"))
        .on_drag_end(node_drag_end_handler(
            state,
            "tabs-underline-drag",
            "tabs-underline-drop",
        ))
        .on_drop_target_change(node_optional_text_handler(state, "tabs-underline-drop"));

    // 4. CARD WITH ICONS (NO PANEL)
    let card_icon_tabs = vec![
        TabDefinition::new("home", "Home").with_icon("home"),
        TabDefinition::new("settings", "Settings").with_icon("settings"),
        TabDefinition::new("users", "Users").with_icon("users"),
    ];

    let card_icon_value = state
        .specimens
        .text
        .get("tabs-card-icon-value")
        .map(|s| s.as_str())
        .unwrap_or("home")
        .to_string();

    let card_icon_spec = TabsSpec::new(card_icon_tabs)
        .with_variant(TabVariant::Card)
        .with_bordered(false)
        .with_value(&card_icon_value)
        .with_aria_label("Icon tabs");

    let card_icon_component = Tabs::from_spec(card_icon_spec, theme)
        .with_id("specimen-card-icons")
        .on_change(node_value_handler(state, "tabs-card-icon-value"));

    // 5. BLOCK VARIANT — VERTICAL (ICON-ONLY, ACTIVE EDGE UNDERLINE)
    // The former strip variant's vertical look: block + underline.
    let vertical_items = vec![
        TabDefinition::new("files", "Explorer").with_icon("folder"),
        TabDefinition::new("search", "Search").with_icon("search"),
        TabDefinition::new("git", "Source Control").with_icon("layers"),
        TabDefinition::new("debug", "Debug").with_icon("terminal"),
    ];

    let vertical_value = state
        .specimens
        .text
        .get("tabs-vertical-value")
        .map(|s| s.as_str())
        .unwrap_or("files")
        .to_string();

    let vertical_spec = TabsSpec::new(vertical_items)
        .with_variant(TabVariant::Block)
        .with_active_edge(ActiveEdge::Underline)
        .with_value(&vertical_value)
        .with_orientation(Orientation::Vertical)
        .with_aria_label("Side panel tabs");

    let vertical_component = Tabs::from_spec(vertical_spec, theme)
        .with_id("specimen-block-underline-vertical")
        .on_change(node_value_handler(state, "tabs-vertical-value"));

    // 6. COLLAPSE TOGGLE (block + underline, orientation toggle)
    let panel_collapsed = state.specimens.is_on("tabs-panel-collapsed");

    let collapse_items = vec![
        TabDefinition::new("editor", "Editor").with_icon("code"),
        TabDefinition::new("terminal", "Terminal").with_icon("terminal"),
        TabDefinition::new("output", "Output").with_icon("file-text"),
    ];

    let collapse_value = state
        .specimens
        .text
        .get("tabs-collapse-value")
        .map(|s| s.as_str())
        .unwrap_or("editor")
        .to_string();

    let collapse_orientation = if panel_collapsed {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };

    let collapse_spec = TabsSpec::new(collapse_items)
        .with_variant(TabVariant::Block)
        .with_active_edge(ActiveEdge::Underline)
        .with_value(&collapse_value)
        .with_orientation(collapse_orientation)
        .with_reorderable(true)
        .with_aria_label("Panel tabs");

    let collapse_component = Tabs::from_spec(collapse_spec, theme)
        .with_id("specimen-block-underline-collapse")
        .on_change(node_value_handler(state, "tabs-collapse-value"));

    // 8. FULL-WIDTH (tabs flex to fill the row)
    let full_width_tabs = vec![
        TabDefinition::new("details", "Details"),
        TabDefinition::new("usage", "Usage").with_count(12),
        TabDefinition::new("versions", "Versions").with_count(3),
    ];
    let full_width_value = state
        .specimens
        .text
        .get("tabs-fullwidth-value")
        .map(|s| s.as_str())
        .unwrap_or("details")
        .to_string();
    let full_width_spec = TabsSpec::new(full_width_tabs)
        .with_variant(TabVariant::Card)
        .with_full_width(true)
        .with_value(&full_width_value)
        .with_aria_label("Full-width sections");
    let full_width_component = Tabs::from_spec(full_width_spec, theme)
        .with_id("specimen-fullwidth")
        .on_change(node_value_handler(state, "tabs-fullwidth-value"));

    // 11. REORDER DRAG STATES (drag-source + drop-target)
    // Contract §4: drag-source = opacity 0.4; drop-target = inset accent ring
    // (GPUI fallback: 2px accent border). Host-set transient state, here pinned
    // on the spec so the visual renders without an active drag gesture.
    let drag_tabs = || {
        vec![
            TabDefinition::new("overview", "Overview"),
            TabDefinition::new("features", "Features"),
            TabDefinition::new("pricing", "Pricing"),
        ]
    };
    let drag_underline = Tabs::from_spec(
        TabsSpec::new(drag_tabs())
            .with_variant(TabVariant::Card)
            .with_value("overview")
            .with_drag_value(Some("features".to_string()))
            .with_drop_target_value(Some("pricing".to_string())),
        theme,
    )
    .with_id("specimen-drag-card");
    let drag_card = Tabs::from_spec(
        TabsSpec::new(drag_tabs())
            .with_variant(TabVariant::Card)
            .with_active_edge(ActiveEdge::Outline)
            .with_value("overview")
            .with_drag_value(Some("features".to_string()))
            .with_drop_target_value(Some("pricing".to_string())),
        theme,
    )
    .with_id("specimen-drag-card-outline");

    // ASSEMBLE
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // 1. Card variant (default, with panel)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Tabs over a panel — counts, a separator, and a flush variant"),
                    theme,
                ))
                .child(basic_card_component),
        )
        // 2. Card variant (closable, reorderable)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Editable tabs — close one, or drag to reorder"),
                    theme,
                ))
                .child(card_component),
        )
        .when(!last_card_closed.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last closed: {}", last_card_closed)),
            )
        })
        // 2b. Card variant with counts and icons
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Counts and separators"),
                    theme,
                ))
                .child(counts_component),
        )
        // Variants — one row per variant, same items.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Variants — card, pill, and block"),
                    theme,
                ))
                .child(pill_component)
                .child(block_component),
        )
        // Marking the active tab.
        // `activeEdge` and `activeFill` are variant-agnostic, so the four
        // permutation sections this replaced taught one idea four times.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Marking the active tab — an edge, a fill, or both"),
                    theme,
                ))
                .child(outline_component)
                .child(solid_component)
                .child(block_outline_component)
                .child(
                    div()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .overflow_hidden()
                        .child(underline_component)
                        .child(
                            div()
                                .p(px(16.0))
                                .bg(color_to_hsla(bg_surface))
                                .min_h(px(48.0))
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Surface content area"),
                        ),
                ),
        )
        // 4. Card with icons (no panel)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Without a border — for titlebars and toolbars"),
                    theme,
                ))
                .child(card_icon_component),
        )
        // 5. Block variant — vertical (icon-only, active underline)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Vertical — a side panel's tab rail"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .overflow_hidden()
                        .h(px(160.0))
                        .child(vertical_component)
                        .child(
                            div()
                                .flex_1()
                                .p(px(16.0))
                                .bg(color_to_hsla(bg_surface))
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Active: {}", vertical_value)),
                        ),
                ),
        )
        // 7. Collapse toggle
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content(
                        "Block variant — collapse toggle (click to toggle orientation)",
                    ),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex()
                                .gap(px(8.0))
                                .items_center()
                                .child(
                                    div()
                                        .id("collapse-toggle-btn")
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(4.0))
                                        .border_1()
                                        .border_color(color_to_hsla(border))
                                        .text_xs()
                                        .cursor_pointer()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child(if panel_collapsed {
                                            "Expand"
                                        } else {
                                            "Collapse"
                                        })
                                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                            this.state.specimens.toggle("tabs-panel-collapsed");
                                            cx.notify();
                                        })),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child(if panel_collapsed {
                                            "Vertical (collapsed)"
                                        } else {
                                            "Horizontal (expanded)"
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(color_to_hsla(border))
                                .overflow_hidden()
                                .when(panel_collapsed, |d| d.flex().h(px(120.0)))
                                .child(collapse_component)
                                .when(!panel_collapsed, |d| {
                                    d.child(
                                        div()
                                            .p(px(16.0))
                                            .bg(color_to_hsla(bg_surface))
                                            .min_h(px(48.0))
                                            .text_sm()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!("Panel: {}", collapse_value)),
                                    )
                                })
                                .when(panel_collapsed, |d| {
                                    d.child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .bg(color_to_hsla(bg_surface))
                                            .text_sm()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!("Panel: {}", collapse_value)),
                                    )
                                }),
                        ),
                ),
        )
        // 8. Full-width
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Full-width (tabs flex to fill the row)"),
                    theme,
                ))
                .child(div().w_full().child(full_width_component)),
        )
        // Reorder drag states (drag-source dimmed, drop-target ringed).
        // Native-specific evidence for contract §4: the transient drag visuals
        // are host-set, so they are pinned on the spec here rather than
        // produced by a gesture. The web pages have no equivalent.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content(
                        "Reorder drag states — 'Features' dragged (dimmed), 'Pricing' drop-target (ring); second row with active outline",
                    ),
                    theme,
                ))
                .child(drag_underline)
                .child(drag_card),
        )
        .into_any_element();

    // The axis sweeps used to be sections 9 and 10 of the page body. They are
    // panes now: Examples teaches the component, the tabs carry the matrices.
    let axis_items = || {
        vec![
            TabDefinition::new("details", "Details"),
            TabDefinition::new("usage", "Usage").with_count(12),
            TabDefinition::new("versions", "Versions").with_count(3),
        ]
    };

    specimen_layout(
        state,
        cx,
        "tabs",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                let spec = TabsSpec::new(axis_items())
                    .with_variant(TabVariant::Card)
                    .with_size(size)
                    .with_value("details")
                    .with_aria_label(format!("{size:?} tabs"));
                div()
                    .max_w(px(360.0))
                    .child(Tabs::from_spec(spec, theme).with_id(format!("specimen-size-{size:?}")))
                    .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                let spec = TabsSpec::new(axis_items())
                    .with_variant(TabVariant::Card)
                    .with_density(density)
                    .with_value("details")
                    .with_aria_label(format!("{density:?} tabs"));
                div()
                    .max_w(px(360.0))
                    .child(
                        Tabs::from_spec(spec, theme)
                            .with_id(format!("specimen-density-{density:?}")),
                    )
                    .into_any_element()
            }),
    )
}
