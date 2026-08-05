//! Card specimen — exercises the real `js_card` component across the full
//! contract state set (variants, layouts, densities, media, selected,
//! interactive). Every dimension/color resolves from `CardSpec` + tokens; the
//! specimen adds no hand-rolled surfaces or raw borders.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_card;
use crate::compat::{rem_to_px, size_font_rem};

use poodle_specs::{CardLayout, CardSpec, CardVariant, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    // Token-derived typography: title/body at the Md control font, footer meta
    // at the Xs control font. No hardcoded text sizes.
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let meta_font = rem_to_px(size_font_rem(ControlSize::Xs));

    let title = move |t: &str| {
        label(t)
            .text_color(primary)
            .text_size(body_font)
            .text_weight(600)
    };
    let body = move |t: &str| label(t).text_color(secondary).text_size(body_font);
    let meta = move |t: &str| label(t).text_color(secondary).text_size(meta_font);

    div().flex_col().gap(24.0)
        // Default variant — two cards with header/body/footer.
        .child(group("Default variant", secondary,
            div().flex_row().gap(16.0)
                .child(div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new(), theme, vec![
                    title("Project Alpha"),
                    body("A design system component library for building consistent interfaces."),
                    meta("Updated 2 days ago"),
                ])))
                .child(div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new(), theme, vec![
                    title("Monthly report"),
                    body("48 components shipped across 3 packages this month."),
                ])))
        ))
        // Outlined variant.
        .child(group("Outlined variant", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().with_variant(CardVariant::Outlined), theme, vec![
                title("Outlined card"),
                body("This card uses a subtle border instead of elevation."),
            ]))
        ))
        // Elevated variant.
        .child(group("Elevated variant", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().with_variant(CardVariant::Elevated), theme, vec![
                title("Elevated card"),
                body("This card uses a drop shadow for visual prominence."),
            ]))
        ))
        // Selected — accent border ring.
        .child(group("Selected", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().selected(), theme, vec![
                title("Selected card"),
                body("Selected cards carry an accent border and accent ring."),
            ]))
        ))
        // Interactive — hover fill + pointer cursor.
        .child(group("Interactive", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().interactive(), theme, vec![
                title("Interactive card"),
                body("Hover to see the interactive state. Cursor changes to pointer."),
            ]))
        ))
        // Media slot — first child is the clipped media region.
        .child(group("Media slot", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().with_media(true), theme, vec![
                div().h(rem_to_px(7.5)).bg(accent),
                title("Media card"),
                body("The media region is overflow-clipped with an inset radius."),
            ]))
        ))
        // Horizontal layout — media leads, content fills the rest.
        .child(group("Horizontal layout", secondary,
            div().w(rem_to_px(25.0)).child(js_card(
                &CardSpec::new().with_layout(CardLayout::Horizontal).with_media(true),
                theme,
                vec![
                    div().w(rem_to_px(6.0)).h(rem_to_px(6.0)).bg(accent),
                    div().flex_col().gap(4.0)
                        .child(title("Horizontal card"))
                        .child(body("Media occupies the leading column; content fills the rest.")),
                ],
            ))
        ))
        // Density variants — compact / default / comfortable.
        .child(group("Density variants", secondary,
            div().flex_row().gap(16.0)
                .child(density_card(theme, ControlDensity::Compact, "Compact", title, body, meta))
                .child(density_card(theme, ControlDensity::Default, "Default", title, body, meta))
                .child(density_card(theme, ControlDensity::Comfortable, "Comfortable", title, body, meta))
        ))
        // Compact layout — reduced padding and gap.
        .child(group("Compact layout", secondary,
            div().w(rem_to_px(17.5)).child(js_card(&CardSpec::new().with_layout(CardLayout::Compact), theme, vec![
                title("Compact layout"),
                body("Reduced padding and gap via the compact layout."),
            ]))
        ))
}

fn density_card(
    theme: &JetstreamThemeProvider,
    density: ControlDensity,
    name: &str,
    title: impl Fn(&str) -> El,
    body: impl Fn(&str) -> El,
    meta: impl Fn(&str) -> El,
) -> El {
    div().w(rem_to_px(15.0)).child(js_card(
        &CardSpec::new().with_density(density),
        theme,
        vec![
            title(name),
            body("A design system component library for building consistent interfaces."),
            meta("Updated 2 days ago"),
        ],
    ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
