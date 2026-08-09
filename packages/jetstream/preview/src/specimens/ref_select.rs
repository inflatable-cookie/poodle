//! RefSelect specimen — version-control ref chooser.

use crate::compat::js_ref_select;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlDensity, ControlSize, RefKind, RefOption, RefSelectEmphasis, RefSelectSpec,
    RefSelectVariant,
};

fn refs() -> Vec<RefOption> {
    vec![
        RefOption::new("main", "main")
            .with_description("a1b2c3d")
            .with_group("Branches"),
        RefOption::new("tree-component", "tree-component")
            .with_description("9f0e1d2")
            .with_group("Branches"),
        RefOption::new("agent-composer", "agent-composer")
            .with_description("4c5b6a7")
            .with_group("Branches"),
        RefOption::new("v1.4.0", "v1.4.0")
            .with_kind(RefKind::Tag)
            .with_group("Tags"),
        RefOption::new("e3f4a5b", "e3f4a5b")
            .with_kind(RefKind::Commit)
            .with_description("Fix the failing parity gate")
            .with_group("Recent commits"),
    ]
}

fn base() -> RefSelectSpec {
    RefSelectSpec::new()
        .with_refs(refs())
        .with_value("tree-component")
        .with_current_ref("main")
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Refs with the checked-out branch marked (open)",
            secondary,
            js_ref_select(&base().with_open(true), theme),
        ))
        .child(group(
            "Trigger only (collapsed)",
            secondary,
            js_ref_select(&base(), theme),
        ))
        .child(group(
            "Host-driven search (searchValue supplied)",
            secondary,
            js_ref_select(
                &RefSelectSpec::new()
                    .with_refs(vec![refs()[1].clone(), refs()[2].clone()])
                    .with_value("tree-component")
                    .with_current_ref("main")
                    .with_search_value("comp")
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Loading more refs",
            secondary,
            js_ref_select(&base().with_loading(true).with_open(true), theme),
        ))
        .child(group(
            "No matches",
            secondary,
            js_ref_select(
                &RefSelectSpec::new()
                    .with_search_value("nothing-matches")
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Search hidden",
            secondary,
            js_ref_select(&base().with_searchable(false).with_open(true), theme),
        ))
        .child(group(
            "Outlined trigger",
            secondary,
            js_ref_select(&base().with_variant(RefSelectVariant::Outlined), theme),
        ))
        .child(group(
            "Subdued (as embedded in the composer footer)",
            secondary,
            js_ref_select(&base().with_emphasis(RefSelectEmphasis::Subdued), theme),
        ))
        .child(group(
            "No selection",
            secondary,
            js_ref_select(
                &RefSelectSpec::new()
                    .with_refs(refs())
                    .with_current_ref("main"),
                theme,
            ),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_ref_select(&base().with_disabled(true), theme),
        ))
        .child(group(
            "Sizes",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| js_ref_select(&base().with_size(size), theme)),
            ),
        ))
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| js_ref_select(&base().with_density(density), theme)),
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
