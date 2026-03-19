use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{TabsSpec, TabDefinition, TabVariant};
use pug_gpui_components::PugTabs;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // ── Underline variant (default, with panel) ──────────────────────
    // Contract: Overview (active), Features, Pricing, FAQ (disabled)
    let underline_tabs = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("features", "Features"),
        TabDefinition::new("pricing", "Pricing"),
        TabDefinition::new("faq", "FAQ").with_disabled(true),
    ];

    let selected_underline = state.specimens.selected("tabs-underline");
    let underline_value = match selected_underline {
        0 => "overview",
        1 => "features",
        2 => "pricing",
        _ => "overview",
    };

    let underline_spec = TabsSpec::new(underline_tabs)
        .with_variant(TabVariant::Underline)
        .with_value(underline_value)
        .with_aria_label("Section tabs");

    let mut underline_component = PugTabs::new(underline_spec, theme)
        .with_id("specimen-underline");

    underline_component = underline_component
        .with_content(
            "overview".to_string(),
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child("Overview content goes here.".to_string()),
        )
        .with_content(
            "features".to_string(),
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child("Features content goes here.".to_string()),
        )
        .with_content(
            "pricing".to_string(),
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child("Pricing content goes here.".to_string()),
        );

    // ── Card variant (closable, reorderable) ─────────────────────────
    // Contract: index.ts (active), App.svelte (closable), utils.ts (closable), types.ts (closable)
    // Note: TabDefinition does not yet support is_closable or is_reorderable.
    // Rendering with available props only.
    let card_tabs = vec![
        TabDefinition::new("index.ts", "index.ts"),
        TabDefinition::new("App.svelte", "App.svelte"),
        TabDefinition::new("utils.ts", "utils.ts"),
        TabDefinition::new("types.ts", "types.ts"),
    ];

    let card_spec = TabsSpec::new(card_tabs)
        .with_variant(TabVariant::Card)
        .with_value("index.ts")
        .with_aria_label("Open files");

    let card_component = PugTabs::new(card_spec, theme)
        .with_id("specimen-card");

    // ── Pill variant (with icons) ────────────────────────────────────
    // Contract: Home (house icon, active), Settings (settings icon), Users (users icon)
    // Note: TabDefinition does not yet support icon.
    // Rendering with labels only.
    let pill_tabs = vec![
        TabDefinition::new("home", "Home"),
        TabDefinition::new("settings", "Settings"),
        TabDefinition::new("users", "Users"),
    ];

    let pill_spec = TabsSpec::new(pill_tabs)
        .with_variant(TabVariant::Pill)
        .with_value("home")
        .with_aria_label("Navigation");

    let pill_component = PugTabs::new(pill_spec, theme)
        .with_id("specimen-pill");

    // ── Underline (with icons, no panel) ─────────────────────────────
    // Contract: Home (house icon, active), Settings (settings icon), Users (users icon)
    // Note: TabDefinition does not yet support icon.
    // Rendering with labels only, no panel content.
    let underline_icon_tabs = vec![
        TabDefinition::new("home", "Home"),
        TabDefinition::new("settings", "Settings"),
        TabDefinition::new("users", "Users"),
    ];

    let underline_icon_spec = TabsSpec::new(underline_icon_tabs)
        .with_variant(TabVariant::Underline)
        .with_value("home")
        .with_aria_label("Icon tabs");

    let underline_icon_component = PugTabs::new(underline_icon_spec, theme)
        .with_id("specimen-underline-icons");

    // ── Strip variant specimens omitted ──────────────────────────────
    // Contract defines strip variant and vertical strip specimens, but
    // TabVariant does not yet include Strip. These specimens will be
    // added when the Strip variant is implemented.

    div().flex().flex_col().gap(px(16.0))
        // Underline variant (default, with panel)
        .child(section_label("UNDERLINE VARIANT (DEFAULT, WITH PANEL)", text_secondary))
        .child(underline_component)

        // Card variant (closable, reorderable)
        .child(section_label("CARD VARIANT (CLOSABLE, REORDERABLE)", text_secondary))
        .child(card_component)

        // Pill variant (with icons)
        .child(section_label("PILL VARIANT (WITH ICONS)", text_secondary))
        .child(pill_component)

        // Underline (with icons, no panel)
        .child(section_label("UNDERLINE (WITH ICONS, NO PANEL)", text_secondary))
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
