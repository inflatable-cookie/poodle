use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Eyebrow, TabStrip, Tabs};
use poodle_specs::{
    ControlDensity, ControlSize, EyebrowSpec, Orientation, TabDefinition, TabStripItem,
    TabStripSpec, TabVariant, TabsSpec,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");
    let bg_surface = theme.resolve_color("color.background.surface");

    // 1. UNDERLINE VARIANT (DEFAULT, WITH PANEL)
    let underline_tabs = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ];

    let underline_value = state
        .specimens
        .text
        .get("tabs-underline-value")
        .map(|s| s.as_str())
        .unwrap_or("overview")
        .to_string();

    let underline_spec = TabsSpec::new(underline_tabs)
        .with_variant(TabVariant::Underline)
        .with_value(&underline_value)
        .with_aria_label("Section tabs");

    let underline_component = Tabs::from_spec(underline_spec, theme)
        .with_id("specimen-underline")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-underline-value".to_string(), val.to_string());
            cx.notify();
        }))
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
    let card_tabs = vec![
        TabDefinition::new("index.ts", "index.ts"),
        TabDefinition::new("App.svelte", "App.svelte").with_closable(true),
        TabDefinition::new("utils.ts", "utils.ts").with_closable(true),
        TabDefinition::new("types.ts", "types.ts").with_closable(true),
    ];

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
        .with_aria_label("Open files");

    let card_component = Tabs::from_spec(card_spec, theme)
        .with_id("specimen-card")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-card-value".to_string(), val.to_string());
            cx.notify();
        }));

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
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-counts-value".to_string(), val.to_string());
            cx.notify();
        }));

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
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-pill-value".to_string(), val.to_string());
            cx.notify();
        }));

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
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-block-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 4. UNDERLINE WITH ICONS (NO PANEL)
    let underline_icon_tabs = vec![
        TabDefinition::new("home", "Home").with_icon("home"),
        TabDefinition::new("settings", "Settings").with_icon("settings"),
        TabDefinition::new("users", "Users").with_icon("users"),
    ];

    let underline_icon_value = state
        .specimens
        .text
        .get("tabs-underline-icon-value")
        .map(|s| s.as_str())
        .unwrap_or("home")
        .to_string();

    let underline_icon_spec = TabsSpec::new(underline_icon_tabs)
        .with_variant(TabVariant::Underline)
        .with_bordered(false)
        .with_value(&underline_icon_value)
        .with_aria_label("Icon tabs");

    let underline_icon_component = Tabs::from_spec(underline_icon_spec, theme)
        .with_id("specimen-underline-icons")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-underline-icon-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 5. STRIP VARIANT (HORIZONTAL, CLOSABLE, REORDERABLE)
    let strip_items = vec![
        TabStripItem::new("main.rs", "main.rs").with_closable(true),
        TabStripItem::new("lib.rs", "lib.rs").with_closable(true),
        TabStripItem::new("mod.rs", "mod.rs").with_closable(true),
        TabStripItem::new("Cargo.toml", "Cargo.toml"),
    ];

    let strip_value = state
        .specimens
        .text
        .get("tabs-strip-value")
        .map(|s| s.as_str())
        .unwrap_or("main.rs")
        .to_string();

    let last_strip_closed = state
        .specimens
        .text
        .get("tabs-strip-closed")
        .cloned()
        .unwrap_or_default();
    let last_strip_reorder = state
        .specimens
        .text
        .get("tabs-strip-reorder")
        .cloned()
        .unwrap_or_default();

    let strip_spec = TabStripSpec::new(strip_items)
        .with_value(&strip_value)
        .with_reorderable(true)
        .with_aria_label("File tabs");

    let strip_component = TabStrip::from_spec(strip_spec, theme)
        .with_id("specimen-strip")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-strip-value".to_string(), val.to_string());
            cx.notify();
        }))
        .on_close(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-strip-closed".to_string(), val.to_string());
            cx.notify();
        }));

    // 6. STRIP VARIANT (VERTICAL)
    let vertical_items = vec![
        TabStripItem::new("files", "Files"),
        TabStripItem::new("search", "Search"),
        TabStripItem::new("git", "Git"),
        TabStripItem::new("extensions", "Extensions"),
    ];

    let vertical_value = state
        .specimens
        .text
        .get("tabs-vertical-value")
        .map(|s| s.as_str())
        .unwrap_or("files")
        .to_string();

    let vertical_spec = TabStripSpec::new(vertical_items)
        .with_value(&vertical_value)
        .with_orientation(Orientation::Vertical)
        .with_aria_label("Activity bar");

    let vertical_component = TabStrip::from_spec(vertical_spec, theme)
        .with_id("specimen-vertical")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-vertical-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 7. COLLAPSE TOGGLE
    let panel_collapsed = state.specimens.is_on("tabs-panel-collapsed");

    let collapse_items = vec![
        TabStripItem::new("editor", "Editor").with_closable(true),
        TabStripItem::new("terminal", "Terminal").with_closable(true),
        TabStripItem::new("output", "Output"),
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

    let collapse_spec = TabStripSpec::new(collapse_items)
        .with_value(&collapse_value)
        .with_orientation(collapse_orientation)
        .with_reorderable(true)
        .with_aria_label("Panel tabs");

    let collapse_component = TabStrip::from_spec(collapse_spec, theme)
        .with_id("specimen-collapse")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-collapse-value".to_string(), val.to_string());
            cx.notify();
        }));

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
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state
                .specimens
                .text
                .insert("tabs-fullwidth-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 9. SIZE MATRIX (xs → xl, card variant)
    let size_specs = [
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];
    let mut size_row = div().flex().flex_col().gap(px(12.0)).max_w(px(360.0));
    for (label, size) in size_specs {
        let spec = TabsSpec::new(vec![
            TabDefinition::new("details", "Details"),
            TabDefinition::new("usage", "Usage").with_count(12),
            TabDefinition::new("versions", "Versions").with_count(3),
        ])
        .with_variant(TabVariant::Card)
        .with_size(size)
        .with_value("details")
        .with_aria_label(format!("{label} tabs"));
        size_row = size_row.child(
            Tabs::from_spec(spec, theme).with_id(format!("specimen-size-{label}")),
        );
    }

    // 10. DENSITY MATRIX (compact / default / comfortable, card variant)
    let density_specs = [
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];
    let mut density_row = div().flex().flex_col().gap(px(12.0)).max_w(px(360.0));
    for (label, density) in density_specs {
        let spec = TabsSpec::new(vec![
            TabDefinition::new("details", "Details"),
            TabDefinition::new("usage", "Usage").with_count(12),
            TabDefinition::new("versions", "Versions").with_count(3),
        ])
        .with_variant(TabVariant::Card)
        .with_density(density)
        .with_value("details")
        .with_aria_label(format!("{label} tabs"));
        density_row = density_row.child(
            Tabs::from_spec(spec, theme).with_id(format!("specimen-density-{label}")),
        );
    }

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
            .with_variant(TabVariant::Underline)
            .with_value("overview")
            .with_drag_value(Some("features".to_string()))
            .with_drop_target_value(Some("pricing".to_string())),
        theme,
    )
    .with_id("specimen-drag-underline");
    let drag_card = Tabs::from_spec(
        TabsSpec::new(drag_tabs())
            .with_variant(TabVariant::Card)
            .with_value("overview")
            .with_drag_value(Some("features".to_string()))
            .with_drop_target_value(Some("pricing".to_string())),
        theme,
    )
    .with_id("specimen-drag-card");

    // ASSEMBLE
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // 1. Underline variant (default, with panel)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Text variant (default, with indicator line)"),
                    theme,
                ))
                .child(underline_component),
        )
        // 2. Card variant (closable, reorderable)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Card variant (closable, reorderable)"),
                    theme,
                ))
                .child(card_component),
        )
        // 2b. Card variant with counts and icons
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Card variant with counts, separators"),
                    theme,
                ))
                .child(counts_component),
        )
        // 3. Pill variant
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Pill variant (with icons)"),
                    theme,
                ))
                .child(pill_component),
        )
        // 3b. Block variant
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Block variant (full-width, separators)"),
                    theme,
                ))
                .child(block_component),
        )
        // 4. Underline with icons (no panel)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Text variant (no border)"),
                    theme,
                ))
                .child(underline_icon_component),
        )
        // 5. Strip variant (horizontal, closable, reorderable)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content(
                        "Strip variant (full-width bar with icons, closable, reorderable)",
                    ),
                    theme,
                ))
                .child(
                    div()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .overflow_hidden()
                        .child(strip_component)
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
        .when(!last_strip_closed.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last closed: {}", last_strip_closed)),
            )
        })
        .when(!last_strip_reorder.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last reordered: {}", last_strip_reorder)),
            )
        })
        // 6. Strip variant (vertical)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Strip variant — vertical (icon-only, collapsed panel)"),
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
                        "Strip variant — collapse toggle (click to toggle orientation)",
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
        // 9. Sizes
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Sizes (xs → xl)"),
                    theme,
                ))
                .child(size_row),
        )
        // 10. Densities
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Densities (compact / default / comfortable)"),
                    theme,
                ))
                .child(density_row),
        )
        // 11. Reorder drag states (drag-source dimmed, drop-target ringed)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content(
                        "Reorder drag states — 'Features' dragged (dimmed), 'Pricing' drop-target (ring)",
                    ),
                    theme,
                ))
                .child(drag_underline)
                .child(drag_card),
        )
}
