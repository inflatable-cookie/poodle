//! AppHeader specimen — application header bar.
//!
//! Mirrors the GPUI specimen groups (`gpui/preview/src/specimens/app_header.rs`)
//! and the contract §10 specimen: a full app-window header (title + ghost-button
//! nav + utility IconButtons), title + actions + utility, title-only, a custom
//! identity slot, centred + centred-at-narrow-width groups (g13-b017), plus
//! density and size ladders. The three-region shell is composed via
//! `js_app_header_with_slots`; action / utility clusters are real
//! `js_button` / `js_icon_button` rows. Zero hand-rolled boxes.

use crate::compat::js_button;
use crate::compat::js_icon_button;
use crate::compat::{js_app_header, js_app_header_with_slots};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    AppHeaderSpec, ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconButtonSpec,
};

/// A ghost text-button cluster — the global-actions region (contract §2 actions).
fn ghost_actions(theme: &JetstreamThemeProvider, labels: &[&str], size: ControlSize) -> El {
    let mut row = div().flex_row().items_center().gap(4.0);
    for lbl in labels {
        row = row.child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label(*lbl)
                .with_size(size),
            theme,
        ));
    }
    row
}

/// A trailing utility cluster of icon buttons (contract §2 utility region).
fn utility_icons(theme: &JetstreamThemeProvider, icons: &[&str], size: ControlSize) -> El {
    let mut row = div().flex_row().items_center().gap(4.0);
    for ic in icons {
        row = row.child(js_icon_button(
            // A specimen is an integration test, so it has to model correct
            // usage: an icon button with no accessible name is a defect
            // wherever it is built. The glyph name is the nearest true
            // description available here.
            &IconButtonSpec::new()
                .with_icon(*ic)
                .with_aria_label(*ic)
                .with_size(size),
            theme,
        ));
    }
    row
}

/// A destination-style centre region: three muted labels standing in for a
/// tabs group (mirrors soundcheck's centred destinations).
fn destination_row(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let mut row = div().flex_row().items_center().gap(16.0);
    for lbl in ["Editor", "Preview", "Terminal"] {
        row = row.child(label(lbl).text_color(secondary).text_size(12.0));
    }
    row
}

/// The shared demo header used by both ladders: a "My Application" title with
/// New/Open ghost actions and a settings utility icon (mirrors GPUI `demo_header`).
fn demo_header(spec: AppHeaderSpec, theme: &JetstreamThemeProvider) -> El {
    let size = spec.effective_size();
    js_app_header_with_slots(
        &spec,
        theme,
        None,
        None,
        Some(ghost_actions(theme, &["New", "Open"], size)),
        Some(utility_icons(theme, &["settings"], size)),
    )
}

/// The centred demo header shared by the centred and narrow groups: a
/// "My Application" title, destination centre, New/Open actions, settings
/// utility.
fn centered_header(theme: &JetstreamThemeProvider) -> El {
    js_app_header_with_slots(
        &AppHeaderSpec::new().with_title("My Application").with_center(true),
        theme,
        None,
        Some(destination_row(theme)),
        Some(ghost_actions(theme, &["New", "Open"], ControlSize::Sm)),
        Some(utility_icons(theme, &["settings"], ControlSize::Sm)),
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let text_primary = resolve_color(theme, "color.text.primary");

    div()
        .flex_col()
        .gap(24.0)
        // Full app-window header: title + ghost-button menu nav + 3 utility icons.
        .child(group(
            "Full app window header (title + menubar + utility)",
            secondary,
            js_app_header_with_slots(
                &AppHeaderSpec::new()
                    .with_title("Poodle Studio")
                    .with_drag_region(true)
                    .with_aria_label("Application header"),
                theme,
                None,
                None,
                Some(ghost_actions(
                    theme,
                    &["File", "Edit", "View", "Help"],
                    ControlSize::Sm,
                )),
                Some(utility_icons(
                    theme,
                    &["search", "bell", "settings"],
                    ControlSize::Sm,
                )),
            ),
        ))
        // Title + actions + utility.
        .child(group(
            "With title, actions, and utility",
            secondary,
            js_app_header_with_slots(
                &AppHeaderSpec::new().with_title("My Application"),
                theme,
                None,
                None,
                Some(ghost_actions(theme, &["New", "Open"], ControlSize::Sm)),
                Some(utility_icons(theme, &["settings"], ControlSize::Sm)),
            ),
        ))
        // Title only — default identity region, no slots.
        .child(group(
            "Title only",
            secondary,
            js_app_header(
                &AppHeaderSpec::new().with_title("Poodle Workstation"),
                theme,
            ),
        ))
        // Custom identity slot: swatch + label replaces the default title group.
        .child(group(
            "Custom identity slot",
            secondary,
            js_app_header_with_slots(
                &AppHeaderSpec::new().with_aria_label("Custom identity header"),
                theme,
                Some(
                    div()
                        .flex_row()
                        .items_center()
                        .gap(8.0)
                        .child(div().w(20.0).h(20.0).rounded(4.0).bg(accent))
                        .child(
                            label("Poodle Studio")
                                .text_color(text_primary)
                                .text_size(13.0)
                                .text_weight(600),
                        ),
                ),
                None,
                None,
                Some(utility_icons(theme, &["bell", "user"], ControlSize::Sm)),
            ),
        ))
        // Centred header: destination centre, actions + utility trailing.
        .child(group(
            "Centred header (destination tabs in the centre)",
            secondary,
            centered_header(theme),
        ))
        // Centred header at narrow width (≤45rem viewport).
        .child(group(
            "Centred header at narrow width (≤45rem viewport)",
            secondary,
            // 40rem frame: the native renderer has no viewport breakpoint,
            // so the centred row holds at narrow width (web reflows via the
            // CSS media query; see the contract §8).
            div().w(640.0).child(centered_header(theme)),
        ))
        // Density ladder — region gaps tighten/loosen, height unchanged.
        .child(group(
            "Density ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    "COMPACT",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_density(ControlDensity::Compact),
                        theme,
                    ),
                ))
                .child(ladder(
                    "DEFAULT",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_density(ControlDensity::Default),
                        theme,
                    ),
                ))
                .child(ladder(
                    "COMFORTABLE",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_density(ControlDensity::Comfortable),
                        theme,
                    ),
                )),
        ))
        // Size ladder — shell height + title font scale xs..xl.
        .child(group(
            "Size ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    "XS",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_size(ControlSize::Xs),
                        theme,
                    ),
                ))
                .child(ladder(
                    "SM",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_size(ControlSize::Sm),
                        theme,
                    ),
                ))
                .child(ladder(
                    "MD",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_size(ControlSize::Md),
                        theme,
                    ),
                ))
                .child(ladder(
                    "LG",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_size(ControlSize::Lg),
                        theme,
                    ),
                ))
                .child(ladder(
                    "XL",
                    secondary,
                    demo_header(
                        AppHeaderSpec::new()
                            .with_title("My Application")
                            .with_size(ControlSize::Xl),
                        theme,
                    ),
                )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}

/// A ladder entry: a bold muted label above a single header instance.
fn ladder(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(
            label(title)
                .text_color(text_secondary)
                .text_size(11.0)
                .text_weight(700),
        )
        .child(content)
}
