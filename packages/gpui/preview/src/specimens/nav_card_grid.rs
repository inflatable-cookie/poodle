use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{NavCardGridSpec, SurfaceSpec, SurfaceTone, SurfaceBorder, PaddingScale};
use pug_gpui_components::{NavCardGrid, Surface};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let nav_card = |title: &str, description: &str| {
        Surface::from_spec(
            SurfaceSpec::new()
                .with_tone(SurfaceTone::Panel)
                .with_border(SurfaceBorder::Subtle)
                .with_padding(PaddingScale::Md),
            theme,
        )
        .with_content(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    div().text_sm().font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child(title.to_string()),
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child(description.to_string()),
                ),
        )
    };

    div().flex().flex_col().gap(px(24.0))
        // --- 2 columns (default) ---
        .child(section_label("2 COLUMNS (DEFAULT)", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new(), theme)
                .with_child(nav_card("Getting Started", "Learn the basics of the platform"))
                .with_child(nav_card("Components", "Browse the component library"))
                .with_child(nav_card("Tokens", "Design token reference"))
                .with_child(nav_card("API Reference", "Complete API documentation"))
        )
        // --- 3 columns ---
        .child(section_label("3 COLUMNS", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new().with_columns(3), theme)
                .with_child(nav_card("Overview", "System overview"))
                .with_child(nav_card("Installation", "Setup guide"))
                .with_child(nav_card("Configuration", "Config options"))
                .with_child(nav_card("Themes", "Theme customization"))
                .with_child(nav_card("Plugins", "Extend functionality"))
                .with_child(nav_card("FAQ", "Common questions"))
        )
        // --- 1 column ---
        .child(section_label("1 COLUMN", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new().with_columns(1), theme)
                .with_child(nav_card("Dashboard", "View your project dashboard"))
                .with_child(nav_card("Settings", "Configure preferences"))
        )
        // --- 4 columns ---
        .child(section_label("4 COLUMNS", text_secondary))
        .child(
            NavCardGrid::from_spec(NavCardGridSpec::new().with_columns(4), theme)
                .with_child(nav_card("Home", "Return home"))
                .with_child(nav_card("Search", "Find content"))
                .with_child(nav_card("Recent", "Recent items"))
                .with_child(nav_card("Favorites", "Saved items"))
        )
        // --- With aria-label ---
        .child(section_label("WITH ARIA-LABEL", text_secondary))
        .child(
            NavCardGrid::from_spec(
                NavCardGridSpec::new()
                    .with_columns(2)
                    .with_aria_label("Documentation navigation"),
                theme,
            )
            .with_child(nav_card("Guides", "Step-by-step tutorials"))
            .with_child(nav_card("Examples", "Code examples and demos"))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
