use crate::nel::El;
use poodle_jetstream::JetstreamThemeProvider;

pub fn knob(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::knob(theme))
}

pub fn fader(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::fader(theme))
}

pub fn audio_meter(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::audio_meter(theme))
}

pub fn value_readout(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::value_readout(theme))
}

pub fn drag_number_field(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::drag_number_field(theme))
}

pub fn envelope_editor(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::envelope_editor(theme))
}

pub fn xy_pad(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::xy_pad(theme))
}

pub fn audio_switch(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::audio_switch(theme))
}

pub fn gain_reduction_meter(theme: &JetstreamThemeProvider) -> El {
    El(poodle_render::audio_specimens::gain_reduction_meter(theme))
}
