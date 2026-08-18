use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, RefSelect};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    EyebrowSpec, RefKind, RefOption, RefSelectEmphasis, RefSelectSpec, RefSelectVariant,
};

fn demo_refs() -> Vec<RefOption> {
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

fn demo_spec() -> RefSelectSpec {
    RefSelectSpec::new()
        .with_refs(demo_refs())
        .with_value("tree-component")
        .with_current_ref("main")
}

fn section(title: &str, theme: &GpuiThemeProvider, content: AnyElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(section(
            "Refs with the checked-out branch marked (open)",
            theme,
            RefSelect::from_spec(demo_spec().with_open(true), theme).into_any_element(),
        ))
        .child(section(
            "Trigger only (collapsed)",
            theme,
            RefSelect::from_spec(demo_spec(), theme).into_any_element(),
        ))
        .child(section(
            "Host-driven search (searchValue supplied)",
            theme,
            RefSelect::from_spec(
                RefSelectSpec::new()
                    .with_refs(vec![demo_refs()[1].clone(), demo_refs()[2].clone()])
                    .with_value("tree-component")
                    .with_current_ref("main")
                    .with_search_value("comp")
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Loading more refs",
            theme,
            RefSelect::from_spec(demo_spec().with_loading(true).with_open(true), theme)
                .into_any_element(),
        ))
        .child(section(
            "No matches",
            theme,
            RefSelect::from_spec(
                RefSelectSpec::new()
                    .with_search_value("nothing-matches")
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Outlined trigger",
            theme,
            RefSelect::from_spec(demo_spec().with_variant(RefSelectVariant::Outlined), theme)
                .into_any_element(),
        ))
        .child(section(
            "Subdued (as embedded in the composer footer)",
            theme,
            RefSelect::from_spec(demo_spec().with_emphasis(RefSelectEmphasis::Subdued), theme)
                .into_any_element(),
        ))
        .child(section(
            "Disabled",
            theme,
            RefSelect::from_spec(demo_spec().with_disabled(true), theme).into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "ref-select",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                RefSelect::from_spec(demo_spec(), theme)
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                RefSelect::from_spec(demo_spec(), theme)
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
