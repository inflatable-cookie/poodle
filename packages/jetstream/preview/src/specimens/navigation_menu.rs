//! NavigationMenu specimen — horizontal nav menus with active and disabled states.
//!
//! Mirrors the GPUI specimen (`gpui/preview/src/specimens/navigation_menu.rs`)
//! and contract §13 + §4: top-level triggers with leading icons (contract §3
//! `icon`), an active trigger (Components) with its disclosed viewport panel
//! rendering the active item's `description` (Known Delta §12 slot-prop
//! equivalent), a disabled item (Changelog), plus size and density ladders.
//! The active fill, leading icon and viewport panel are all rendered by
//! `js_navigation_menu` from the spec.
//!
//! No chevron group: neither the Jetstream nor the GPUI component renders a
//! disclosure chevron (the trigger has no expand affordance), so a chevron
//! group would be a fake — omitted per CLAUDE.md "no fakes".

use crate::compat::js_navigation_menu;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, NavigationMenuEntry, NavigationMenuSpec};

/// The full item set with leading icons + viewport descriptions (mirrors GPUI),
/// ending in a disabled Changelog entry.
fn full_items() -> Vec<NavigationMenuEntry> {
    vec![
        NavigationMenuEntry::new("home", "Home")
            .with_icon("arrow-right")
            .with_description("Overview, highlights, and what's new this release."),
        NavigationMenuEntry::new("components", "Components")
            .with_icon("filter")
            .with_description("Buttons, inputs, overlays, and the full primitive catalog."),
        NavigationMenuEntry::new("tokens", "Tokens")
            .with_icon("check")
            .with_description("Color, spacing, typography, and radius semantic tokens."),
        NavigationMenuEntry::new("guides", "Guides")
            .with_icon("chevron-right")
            .with_description("Adoption guides, theming, and migration walkthroughs."),
        NavigationMenuEntry::new("changelog", "Changelog")
            .with_icon("clock")
            .with_disabled(true),
    ]
}

/// A label-only item set for the ladders (no viewport content).
fn ladder_items() -> Vec<NavigationMenuEntry> {
    vec![
        NavigationMenuEntry::new("home", "Home"),
        NavigationMenuEntry::new("components", "Components"),
        NavigationMenuEntry::new("tokens", "Tokens"),
        NavigationMenuEntry::new("guides", "Guides"),
        NavigationMenuEntry::new("changelog", "Changelog").with_disabled(true),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Horizontal navigation — leading icons per trigger, Components active
        // (accent fill), disabled Changelog, plus the disclosed viewport panel
        // rendering the active item's description.
        .child(group(
            "Horizontal navigation (active = Components)",
            secondary,
            js_navigation_menu(
                &NavigationMenuSpec::new(full_items())
                    .with_value("components")
                    .with_aria_label("Main navigation"),
                theme,
            ),
        ))
        // Active = Tokens — shows the viewport content swap to the active item.
        .child(group(
            "Active = Tokens",
            secondary,
            js_navigation_menu(
                &NavigationMenuSpec::new(full_items())
                    .with_value("tokens")
                    .with_aria_label("Main navigation"),
                theme,
            ),
        ))
        // Size ladder — trigger height + font scale xs..xl.
        .child(group(
            "Size ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    theme,
                    "XS",
                    NavigationMenuSpec::new(ladder_items()).with_size(ControlSize::Xs),
                ))
                .child(ladder(
                    theme,
                    "SM",
                    NavigationMenuSpec::new(ladder_items()).with_size(ControlSize::Sm),
                ))
                .child(ladder(
                    theme,
                    "MD",
                    NavigationMenuSpec::new(ladder_items()).with_size(ControlSize::Md),
                ))
                .child(ladder(
                    theme,
                    "LG",
                    NavigationMenuSpec::new(ladder_items()).with_size(ControlSize::Lg),
                ))
                .child(ladder(
                    theme,
                    "XL",
                    NavigationMenuSpec::new(ladder_items()).with_size(ControlSize::Xl),
                )),
        ))
        // Density ladder — trigger padding tightens/loosens, height unchanged.
        .child(group(
            "Density ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    theme,
                    "COMPACT",
                    NavigationMenuSpec::new(ladder_items()).with_density(ControlDensity::Compact),
                ))
                .child(ladder(
                    theme,
                    "DEFAULT",
                    NavigationMenuSpec::new(ladder_items()).with_density(ControlDensity::Default),
                ))
                .child(ladder(
                    theme,
                    "COMFORTABLE",
                    NavigationMenuSpec::new(ladder_items())
                        .with_density(ControlDensity::Comfortable),
                )),
        ))
}

fn ladder(theme: &JetstreamThemeProvider, lbl: &str, spec: NavigationMenuSpec) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    div()
        .flex_col()
        .gap(8.0)
        .child(
            label(lbl)
                .text_color(secondary)
                .text_size(11.0)
                .text_weight(700),
        )
        .child(js_navigation_menu(
            &spec.with_value("components").with_aria_label("Navigation"),
            theme,
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
