use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{TabsSpec, TabDefinition, TabVariant, TabStripSpec, TabStripItem, Orientation, EyebrowSpec};
use pug_gpui_components::{Tabs, TabStrip, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");
    let bg_surface = theme.resolve_color("semantic.color.background.surface");

    // 1. UNDERLINE VARIANT (DEFAULT, WITH PANEL)
    let underline_tabs = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ];

    let underline_value = state.specimens.text.get("tabs-underline-value")
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
            this.state.specimens.text.insert("tabs-underline-value".to_string(), val.to_string());
            cx.notify();
        }))
        .with_content(
            "overview".to_string(),
            div().p(px(12.0)).text_size(px(14.0)).text_color(color_to_hsla(text_secondary))
                .child("Overview content — this is the landing page with a summary of all features.".to_string()),
        )
        .with_content(
            "features".to_string(),
            div().p(px(12.0)).text_size(px(14.0)).text_color(color_to_hsla(text_secondary))
                .child("Features content — explore the full feature set and capabilities.".to_string()),
        )
        .with_content(
            "pricing".to_string(),
            div().p(px(12.0)).text_size(px(14.0)).text_color(color_to_hsla(text_secondary))
                .child("Pricing content — compare plans and find the right fit for your team.".to_string()),
        );

    // 2. CARD VARIANT (CLOSABLE, REORDERABLE)
    let card_tabs = vec![
        TabDefinition::new("index.ts", "index.ts"),
        TabDefinition::new("App.svelte", "App.svelte"),
        TabDefinition::new("utils.ts", "utils.ts"),
        TabDefinition::new("types.ts", "types.ts"),
    ];

    let card_value = state.specimens.text.get("tabs-card-value")
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
            this.state.specimens.text.insert("tabs-card-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 3. PILL VARIANT (WITH ICONS)
    let pill_tabs = vec![
        TabDefinition::new("home", "Home"),
        TabDefinition::new("settings", "Settings"),
        TabDefinition::new("users", "Users"),
    ];

    let pill_value = state.specimens.text.get("tabs-pill-value")
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
            this.state.specimens.text.insert("tabs-pill-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 4. UNDERLINE WITH ICONS (NO PANEL)
    let underline_icon_tabs = vec![
        TabDefinition::new("home", "Home"),
        TabDefinition::new("settings", "Settings"),
        TabDefinition::new("users", "Users"),
    ];

    let underline_icon_value = state.specimens.text.get("tabs-underline-icon-value")
        .map(|s| s.as_str())
        .unwrap_or("home")
        .to_string();

    let underline_icon_spec = TabsSpec::new(underline_icon_tabs)
        .with_variant(TabVariant::Underline)
        .with_value(&underline_icon_value)
        .with_aria_label("Icon tabs");

    let underline_icon_component = Tabs::from_spec(underline_icon_spec, theme)
        .with_id("specimen-underline-icons")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state.specimens.text.insert("tabs-underline-icon-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 5. STRIP VARIANT (HORIZONTAL, CLOSABLE, REORDERABLE)
    let strip_items = vec![
        TabStripItem::new("main.rs", "main.rs").with_closable(true),
        TabStripItem::new("lib.rs", "lib.rs").with_closable(true),
        TabStripItem::new("mod.rs", "mod.rs").with_closable(true),
        TabStripItem::new("Cargo.toml", "Cargo.toml"),
    ];

    let strip_value = state.specimens.text.get("tabs-strip-value")
        .map(|s| s.as_str())
        .unwrap_or("main.rs")
        .to_string();

    let last_strip_closed = state.specimens.text.get("tabs-strip-closed")
        .cloned()
        .unwrap_or_default();
    let last_strip_reorder = state.specimens.text.get("tabs-strip-reorder")
        .cloned()
        .unwrap_or_default();

    let strip_spec = TabStripSpec::new(strip_items)
        .with_value(&strip_value)
        .with_reorderable(true)
        .with_aria_label("File tabs");

    let strip_component = TabStrip::from_spec(strip_spec, theme)
        .with_id("specimen-strip")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state.specimens.text.insert("tabs-strip-value".to_string(), val.to_string());
            cx.notify();
        }))
        .on_close(cx.listener(|this, val: &str, _w, cx| {
            this.state.specimens.text.insert("tabs-strip-closed".to_string(), val.to_string());
            cx.notify();
        }));

    // 6. STRIP VARIANT (VERTICAL)
    let vertical_items = vec![
        TabStripItem::new("files", "Files"),
        TabStripItem::new("search", "Search"),
        TabStripItem::new("git", "Git"),
        TabStripItem::new("extensions", "Extensions"),
    ];

    let vertical_value = state.specimens.text.get("tabs-vertical-value")
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
            this.state.specimens.text.insert("tabs-vertical-value".to_string(), val.to_string());
            cx.notify();
        }));

    // 7. COLLAPSE TOGGLE
    let panel_collapsed = state.specimens.is_on("tabs-panel-collapsed");

    let collapse_items = vec![
        TabStripItem::new("editor", "Editor").with_closable(true),
        TabStripItem::new("terminal", "Terminal").with_closable(true),
        TabStripItem::new("output", "Output"),
    ];

    let collapse_value = state.specimens.text.get("tabs-collapse-value")
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
            this.state.specimens.text.insert("tabs-collapse-value".to_string(), val.to_string());
            cx.notify();
        }));

    // ASSEMBLE
    div().flex().flex_col().gap(px(24.0))
        // 1. Underline variant (default, with panel)
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Underline variant (default, with panel)"), theme))
                .child(underline_component)
        )

        // 2. Card variant (closable, reorderable)
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Card variant (closable, reorderable)"), theme))
                .child(card_component)
        )

        // 3. Pill variant
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Pill variant (with icons)"), theme))
                .child(pill_component)
        )

        // 4. Underline with icons (no panel)
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Underline (with icons, no panel)"), theme))
                .child(underline_icon_component)
        )

        // 5. Strip variant (horizontal, closable, reorderable)
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Strip variant (full-width bar with icons, closable, reorderable)"), theme))
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
                                .child("Surface content area")
                        )
                )
        )
        .when(!last_strip_closed.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last closed: {}", last_strip_closed))
            )
        })
        .when(!last_strip_reorder.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last reordered: {}", last_strip_reorder))
            )
        })

        // 6. Strip variant (vertical)
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Strip variant — vertical (icon-only, collapsed panel)"), theme))
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
                                .child(format!("Active: {}", vertical_value))
                        )
                )
        )

        // 7. Collapse toggle
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Strip variant — collapse toggle (click to toggle orientation)"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
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
                                        .child(if panel_collapsed { "Expand" } else { "Collapse" })
                                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                            this.state.specimens.toggle("tabs-panel-collapsed");
                                            cx.notify();
                                        }))
                                )
                                .child(
                                    div().text_xs().text_color(color_to_hsla(text_secondary))
                                        .child(if panel_collapsed { "Vertical (collapsed)" } else { "Horizontal (expanded)" })
                                )
                        )
                        .child(
                            div()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(color_to_hsla(border))
                                .overflow_hidden()
                                .when(panel_collapsed, |d| {
                                    d.flex().h(px(120.0))
                                })
                                .child(collapse_component)
                                .when(!panel_collapsed, |d| {
                                    d.child(
                                        div()
                                            .p(px(16.0))
                                            .bg(color_to_hsla(bg_surface))
                                            .min_h(px(48.0))
                                            .text_sm()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!("Panel: {}", collapse_value))
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
                                            .child(format!("Panel: {}", collapse_value))
                                    )
                                })
                        )
                )
        )
}
