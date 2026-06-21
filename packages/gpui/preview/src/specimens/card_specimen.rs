//! Card specimen — exercises the real `Card` component across the full
//! contract state set (variants, layouts, densities, media, selected,
//! interactive). Every visual resolves from `CardSpec` + tokens; the specimen
//! adds no hand-rolled surfaces, borders, or hardcoded dimensions.

use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Card, Eyebrow};
use poodle_specs::{
    CardLayout, CardSpec, CardVariant, ControlDensity, EyebrowSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");

    // Body text helpers — typography resolves from the theme, not hardcoded.
    let title = move |t: &str| {
        div()
            .text_base()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(color_to_hsla(text_primary))
            .child(t.to_string())
    };
    let body = move |t: &str| {
        div()
            .text_sm()
            .text_color(color_to_hsla(text_secondary))
            .child(t.to_string())
    };
    let meta = move |t: &str| {
        div()
            .text_xs()
            .text_color(color_to_hsla(text_secondary))
            .child(t.to_string())
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Default variant ---
        .child(
            group(theme, "Default variant",
                div().flex().gap(px(16.0)).flex_wrap()
                    .child(
                        div().w(px(280.0)).child(
                            Card::from_spec(CardSpec::new(), theme)
                                .aria_label("Project card")
                                .with_id("card-default-1")
                                .with_header(title("Project Alpha"))
                                .with_body(body("A design system component library for building consistent interfaces."))
                                .with_footer(meta("Updated 2 days ago")),
                        ),
                    )
                    .child(
                        div().w(px(280.0)).child(
                            Card::from_spec(CardSpec::new(), theme)
                                .aria_label("Stats card")
                                .with_id("card-default-2")
                                .with_header(title("Monthly report"))
                                .with_body(body("48 components shipped across 3 packages this month.")),
                        ),
                    ),
            )
        )
        // --- Outlined variant ---
        .child(
            group(theme, "Outlined variant",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new().with_variant(CardVariant::Outlined), theme)
                        .aria_label("Outlined card")
                        .with_id("card-outlined")
                        .with_header(title("Outlined card"))
                        .with_body(body("This card uses a subtle border instead of elevation.")),
                ),
            )
        )
        // --- Elevated variant ---
        .child(
            group(theme, "Elevated variant",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new().with_variant(CardVariant::Elevated), theme)
                        .aria_label("Elevated card")
                        .with_id("card-elevated")
                        .with_header(title("Elevated card"))
                        .with_body(body("This card uses a drop shadow for visual prominence.")),
                ),
            )
        )
        // --- Selected ---
        .child(
            group(theme, "Selected",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new(), theme)
                        .selected()
                        .aria_label("Selected card")
                        .with_id("card-selected")
                        .with_header(title("Selected card"))
                        .with_body(body("Selected cards carry an accent border and accent ring.")),
                ),
            )
        )
        // --- Interactive ---
        .child(
            group(theme, "Interactive",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new(), theme)
                        .interactive()
                        .aria_label("Clickable card")
                        .with_id("card-interactive")
                        .with_header(title("Interactive card"))
                        .with_body(body("Hover to see the interactive state. Cursor changes to pointer.")),
                ),
            )
        )
        // --- Media slot ---
        .child(
            group(theme, "Media slot",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new().with_media(true), theme)
                        .aria_label("Media card")
                        .with_id("card-media")
                        // Media content is a real, clipped region rendered by the
                        // Card; the inset radius comes from the spec.
                        .with_media(div().h(px(120.0)).bg(color_to_hsla(accent)))
                        .with_header(title("Media card"))
                        .with_body(body("The media region is overflow-clipped with an inset radius.")),
                ),
            )
        )
        // --- Horizontal layout ---
        .child(
            group(theme, "Horizontal layout",
                div().w(px(400.0)).child(
                    Card::from_spec(
                        CardSpec::new()
                            .with_layout(CardLayout::Horizontal)
                            .with_media(true),
                        theme,
                    )
                    .aria_label("Horizontal card")
                    .with_id("card-horizontal")
                    .with_media(div().w(px(96.0)).h(px(96.0)).bg(color_to_hsla(accent)))
                    .with_body(
                        div().flex().flex_col().gap(px(4.0))
                            .child(title("Horizontal card"))
                            .child(body("Media occupies the leading column; content fills the rest.")),
                    ),
                ),
            )
        )
        // --- Density variants ---
        .child(
            group(theme, "Density variants",
                div().flex().gap(px(16.0)).flex_wrap()
                    .child(density_card(theme, ControlDensity::Compact, "Compact", "card-compact", title, body, meta))
                    .child(density_card(theme, ControlDensity::Default, "Default", "card-default-density", title, body, meta))
                    .child(density_card(theme, ControlDensity::Comfortable, "Comfortable", "card-comfortable", title, body, meta)),
            )
        )
        // --- Compact layout ---
        .child(
            group(theme, "Compact layout",
                div().w(px(280.0)).child(
                    Card::from_spec(CardSpec::new().with_layout(CardLayout::Compact), theme)
                        .aria_label("Compact card")
                        .with_id("card-compact-layout")
                        .with_header(title("Compact layout"))
                        .with_body(body("Reduced padding and gap via the compact layout.")),
                ),
            )
        )
}

#[allow(clippy::too_many_arguments)]
fn density_card(
    theme: &GpuiThemeProvider,
    density: ControlDensity,
    label: &str,
    id: &'static str,
    title: impl Fn(&str) -> Div,
    body: impl Fn(&str) -> Div,
    meta: impl Fn(&str) -> Div,
) -> Div {
    div().w(px(240.0)).child(
        Card::from_spec(CardSpec::new().with_density(density), theme)
            .aria_label(format!("{label} density card"))
            .with_id(id)
            .with_header(title(label))
            .with_body(body("A design system component library for building consistent interfaces."))
            .with_footer(meta("Updated 2 days ago")),
    )
}

fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div().flex().flex_col().gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(content)
}
