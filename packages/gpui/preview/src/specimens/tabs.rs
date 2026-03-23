use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{TabsSpec, TabDefinition, TabVariant};
use pug_gpui_components::Tabs;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // ── Underline variant (default, with panel) ──────────────────────
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

    let mut underline_component = Tabs::from_spec(underline_spec, theme)
        .with_id("specimen-underline")
        .on_change(cx.listener(|this, val: &str, _w, cx| {
            this.state.specimens.text.insert("tabs-underline-value".to_string(), val.to_string());
            cx.notify();
        }));

    underline_component = underline_component
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

    // ── Card variant ─────────────────────────────────────────────────
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

    // ── Pill variant ─────────────────────────────────────────────────
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

    // ── Underline (no panel) ─────────────────────────────────────────
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

    div().flex().flex_col().gap(px(16.0))
        // Underline variant (default, with panel)
        .child(section_label("UNDERLINE VARIANT (DEFAULT, WITH PANEL)", text_secondary))
        .child(underline_component)

        // Card variant
        .child(section_label("CARD VARIANT", text_secondary))
        .child(card_component)

        // Pill variant
        .child(section_label("PILL VARIANT", text_secondary))
        .child(pill_component)

        // Underline (no panel)
        .child(section_label("UNDERLINE (NO PANEL)", text_secondary))
        .child(underline_icon_component)
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
