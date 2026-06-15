use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::CardToggleGroup;
use poodle_specs::{CardToggleGroupSpec, CardToggleOption};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().child(CardToggleGroup::from_spec(
        CardToggleGroupSpec::new(vec![
            CardToggleOption::new("grid", "Grid view").with_description("Show records as cards."),
            CardToggleOption::new("list", "List view").with_description("Use dense rows."),
            CardToggleOption::new("audit", "Audit view")
                .with_description("Show operational metadata."),
        ])
        .with_values(vec!["grid".into(), "audit".into()]),
        theme,
    ))
}
