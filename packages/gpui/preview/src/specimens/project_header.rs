use gpui::*;
use pug_gpui_workstation::ProjectHeaderSpec;
use pug_gpui_components::PugProjectHeader;
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let clean_spec = ProjectHeaderSpec::new("Project Name")
        .with_subtitle("main branch");

    let dirty_spec = ProjectHeaderSpec::new("Unsaved Project")
        .with_dirty(true)
        .with_subtitle("feature-branch");

    div().flex().flex_col().gap(px(8.0))
        .child(PugProjectHeader::new(clean_spec, theme))
        .child(PugProjectHeader::new(dirty_spec, theme))
}
