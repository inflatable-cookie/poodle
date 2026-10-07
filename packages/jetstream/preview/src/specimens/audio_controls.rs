//! Jetstream audio specimen pages.
//!
//! Jetstream shows the curated Examples pane only; its axis coverage waits on
//! the program's admission gate, so nothing here reaches for the size or
//! density parts `AudioSpecimen` also exposes.

use crate::nel::El;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_render::{RenderContext, audio_specimens::AudioSpecimen};

pub fn knob(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::Knob.examples(&RenderContext::new(theme)))
}

pub fn fader(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::Fader.examples(&RenderContext::new(theme)))
}

pub fn audio_meter(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::AudioMeter.examples(&RenderContext::new(theme)))
}

pub fn value_readout(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::ValueReadout.examples(&RenderContext::new(theme)))
}

pub fn drag_number_field(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::DragNumberField.examples(&RenderContext::new(theme)))
}

pub fn envelope_editor(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::EnvelopeEditor.examples(&RenderContext::new(theme)))
}

pub fn xy_pad(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::XyPad.examples(&RenderContext::new(theme)))
}

pub fn audio_switch(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::AudioSwitch.examples(&RenderContext::new(theme)))
}

pub fn gain_reduction_meter(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::GainReductionMeter.examples(&RenderContext::new(theme)))
}

pub fn keyboard(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::Keyboard.examples(&RenderContext::new(theme)))
}

pub fn waveform_display(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::WaveformDisplay.examples(&RenderContext::new(theme)))
}

pub fn mod_matrix_grid(theme: &JetstreamThemeProvider) -> El {
    El(AudioSpecimen::ModMatrixGrid.examples(&RenderContext::new(theme)))
}
