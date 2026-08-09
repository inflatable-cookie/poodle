//! Toolbar specimen — semantic grouping container for compact action controls.
//!
//! Contract: `docs/contracts/components/toolbar.md` (§13 specimen definitions).
//! Reference: `packages/svelte/preview/src/specimens/ToolbarSpecimen.svelte`.
//!
//! Full contract-state coverage with REAL composed primitives (no hand-rolled
//! boxes): formatting toolbar (ghost icon buttons + a vertical separator + align
//! icon buttons), an actions toolbar (secondary text buttons + separator +
//! primary action), the size scale (xs–xl), the density scale, and a vertical
//! orientation example. Every item is a real `js_icon_button` / `js_button` and
//! every divider a real `js_separator`; the container is a real `js_toolbar`
//! resolving its chrome (panel-mix fill, border-subtle, radius.surface) and its
//! size/density padding + gap from tokens.

use crate::compat::js_button;
use crate::compat::js_icon_button;
use crate::compat::js_separator;
use crate::nel::*;

use crate::compat::js_toolbar;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconButtonSpec, Orientation, RuleTone,
    SeparatorOrientation, SeparatorSpec, ToolbarSpec,
};

/// Ghost icon button used as a toolbar chrome action.
fn ib(theme: &JetstreamThemeProvider, icon: &str, aria: &str) -> El {
    js_icon_button(
        &IconButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_icon(icon)
            .with_aria_label(aria),
        theme,
    )
}

/// Vertical divider between toolbar item groups.
fn divider(theme: &JetstreamThemeProvider) -> El {
    js_separator(
        &SeparatorSpec::new()
            .with_orientation(SeparatorOrientation::Vertical)
            .with_tone(RuleTone::Subtle),
        theme,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // ── Formatting toolbar (contract §13): ghost icon buttons grouped by a
    //    vertical separator into text-style and alignment clusters. ──
    let formatting = js_toolbar(
        &ToolbarSpec::new().with_aria_label("Formatting toolbar"),
        theme,
        vec![
            ib(theme, "bold", "Bold"),
            ib(theme, "italic", "Italic"),
            ib(theme, "underline", "Underline"),
            divider(theme),
            ib(theme, "align-left", "Align left"),
            ib(theme, "align-center", "Align center"),
            ib(theme, "align-right", "Align right"),
        ],
    );

    // ── Actions toolbar (contract §13): secondary text buttons, a separator,
    //    then the primary action. ──
    let actions = js_toolbar(
        &ToolbarSpec::new().with_aria_label("Actions toolbar"),
        theme,
        vec![
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Discard"),
                theme,
            ),
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Save draft"),
                theme,
            ),
            divider(theme),
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Publish"),
                theme,
            ),
        ],
    );

    // ── Size scale: size drives block/inline padding + gap. ──
    let sizes = div().flex_col().gap(12.0);
    let sizes = [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ]
    .into_iter()
    .fold(sizes, |col, (size, name)| {
        col.child(
            div()
                .flex_row()
                .gap(12.0)
                .items_center()
                .child(
                    label(name)
                        .text_color(secondary)
                        .text_size(11.0)
                        .min_w(40.0),
                )
                .child(js_toolbar(
                    &ToolbarSpec::new()
                        .with_size(size)
                        .with_aria_label(format!("Toolbar {name}")),
                    theme,
                    vec![
                        js_icon_button(
                            &IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(size)
                                .with_icon("bold")
                                .with_aria_label("Bold"),
                            theme,
                        ),
                        js_icon_button(
                            &IconButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(size)
                                .with_icon("italic")
                                .with_aria_label("Italic"),
                            theme,
                        ),
                        divider(theme),
                        js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(size)
                                .with_label("Save"),
                            theme,
                        ),
                    ],
                )),
        )
    });

    // ── Density scale: density overrides only inline padding + gap (never the
    //    toolbar's block padding / height). ──
    let densities = div().flex_col().gap(12.0);
    let densities = [
        (ControlDensity::Compact, "compact"),
        (ControlDensity::Default, "default"),
        (ControlDensity::Comfortable, "comfortable"),
    ]
    .into_iter()
    .fold(densities, |col, (density, name)| {
        col.child(
            div()
                .flex_row()
                .gap(12.0)
                .items_center()
                .child(
                    label(name)
                        .text_color(secondary)
                        .text_size(11.0)
                        .min_w(80.0),
                )
                .child(js_toolbar(
                    &ToolbarSpec::new()
                        .with_density(density)
                        .with_aria_label(format!("Toolbar {name}")),
                    theme,
                    vec![
                        ib(theme, "bold", "Bold"),
                        ib(theme, "italic", "Italic"),
                        divider(theme),
                        js_button(
                            &ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Save"),
                            theme,
                        ),
                    ],
                )),
        )
    });

    // ── Vertical orientation: items stack in a column, stretch alignment. ──
    let vertical = js_toolbar(
        &ToolbarSpec::new()
            .with_orientation(Orientation::Vertical)
            .with_aria_label("Vertical toolbar"),
        theme,
        vec![
            ib(theme, "align-left", "Align left"),
            ib(theme, "align-center", "Align center"),
            ib(theme, "align-right", "Align right"),
        ],
    );

    div()
        .flex_col()
        .gap(24.0)
        .child(group("Formatting toolbar", secondary, formatting))
        .child(group("Actions toolbar", secondary, actions))
        .child(group("Sizes", secondary, sizes))
        .child(group("Densities", secondary, densities))
        .child(group("Vertical orientation", secondary, vertical))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
