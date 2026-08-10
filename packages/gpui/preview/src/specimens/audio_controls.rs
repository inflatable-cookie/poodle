use gpui::AnyElement;
use poodle_gpui::GpuiThemeProvider;

fn render(node: poodle_node::Node) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&node)
}

pub(crate) fn knob(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::knob(theme))
}

pub(crate) fn fader(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::fader(theme))
}

pub(crate) fn audio_meter(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::audio_meter(theme))
}

pub(crate) fn value_readout(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::value_readout(theme))
}

pub(crate) fn drag_number_field(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::drag_number_field(theme))
}

pub(crate) fn envelope_editor(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::envelope_editor(theme))
}

pub(crate) fn xy_pad(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::xy_pad(theme))
}

pub(crate) fn audio_switch(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::audio_switch(theme))
}

pub(crate) fn gain_reduction_meter(theme: &GpuiThemeProvider) -> AnyElement {
    render(poodle_render::audio_specimens::gain_reduction_meter(theme))
}
