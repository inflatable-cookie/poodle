//! CardRadioGroup specimen — radio selection across styled cards.

use crate::compat::js_card_radio_group;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::CardRadioGroupSpec;
use poodle_specs::{ChoiceOption, ControlDensity, ControlSize};

fn plan_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("free", "Free").with_description("Basic access with limits."),
        ChoiceOption::new("pro", "Professional").with_description("Full access for individuals."),
        ChoiceOption::new("team", "Team").with_description("Collaboration for groups."),
    ]
}

/// Size options including a disabled tier, for the per-item disabled group.
fn size_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("sm", "Small").with_description("1 CPU, 512 MB RAM"),
        ChoiceOption::new("md", "Medium").with_description("2 CPU, 2 GB RAM"),
        ChoiceOption::new("lg", "Large")
            .with_description("4 CPU, 8 GB RAM")
            .with_disabled(true),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let mut root = div().flex_col().gap(24.0);

    // --- Selection states ---
    root = root.child(group(
        "With selection",
        secondary,
        js_card_radio_group(
            &CardRadioGroupSpec::new(plan_options()).with_value("pro"),
            theme,
        ),
    ));
    root = root.child(group(
        "No selection",
        secondary,
        js_card_radio_group(&CardRadioGroupSpec::new(plan_options()), theme),
    ));

    // --- Disabled option (per-item) ---
    root = root.child(group(
        "Disabled option",
        secondary,
        js_card_radio_group(
            &CardRadioGroupSpec::new(size_options()).with_value("md"),
            theme,
        ),
    ));

    // --- Disabled group ---
    root = root.child(group(
        "Disabled group",
        secondary,
        js_card_radio_group(
            &CardRadioGroupSpec::new(plan_options())
                .with_value("pro")
                .with_disabled(true),
            theme,
        ),
    ));

    // --- Sizes (xs → xl) ---
    let mut sizes_row = div().flex_col().gap(12.0);
    for size in [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ] {
        sizes_row = sizes_row.child(js_card_radio_group(
            &CardRadioGroupSpec::new(plan_options())
                .with_value("pro")
                .with_size(size),
            theme,
        ));
    }
    root = root.child(group(
        "Sizes (xs / sm / md / lg / xl)",
        secondary,
        sizes_row,
    ));

    // --- Densities (compact / default / comfortable) ---
    let mut densities_row = div().flex_col().gap(12.0);
    for density in [
        ControlDensity::Compact,
        ControlDensity::Default,
        ControlDensity::Comfortable,
    ] {
        densities_row = densities_row.child(js_card_radio_group(
            &CardRadioGroupSpec::new(plan_options())
                .with_value("pro")
                .with_density(density),
            theme,
        ));
    }
    root = root.child(group(
        "Densities (compact / default / comfortable)",
        secondary,
        densities_row,
    ));

    root
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
