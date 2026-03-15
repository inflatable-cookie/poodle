use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{StatusIndicatorSpec, StatusTone};
use pug_gpui_components::PugStatusIndicator;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let mut online = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    online.label = Some("Online".to_string());

    let mut idle = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    idle.label = Some("Idle".to_string());

    let mut error = StatusIndicatorSpec::new().with_status(StatusTone::Danger);
    error.label = Some("Error".to_string());

    div().flex().gap(px(12.0))
        .child(PugStatusIndicator::new(online, theme))
        .child(PugStatusIndicator::new(idle, theme))
        .child(PugStatusIndicator::new(error, theme))
}
