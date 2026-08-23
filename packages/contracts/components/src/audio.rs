//! Renderer-neutral native specs for the audio component family.

use poodle_headless::audio::{
    AudioControlVisualState, AudioMeterVisualState, AudioSwitchMode, AudioSwitchVisualState,
    AudioValueLaw, EnvelopeVisualState, GainReductionVisualState, KeyboardVisualState,
    ModMatrixVisualState, WaveformVisualState, XYPadVisualState,
};

use crate::types::{ControlDensity, ControlSize, Orientation, SemanticControlSizeRole};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AudioMeterStyle {
    Bar,
    #[default]
    Segments,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KnobSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: AudioControlVisualState,
    pub min: f64,
    pub max: f64,
    pub law: AudioValueLaw,
    pub value_text: String,
    pub aria_label: String,
}
impl KnobSpec {
    pub fn new(value: f64, min: f64, max: f64, law: AudioValueLaw) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state: AudioControlVisualState::from_value(value, min, max, law, true),
            min,
            max,
            law,
            value_text: value.to_string(),
            aria_label: "Knob value".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaderSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: AudioControlVisualState,
    pub min: f64,
    pub max: f64,
    pub law: AudioValueLaw,
    pub orientation: Orientation,
    /// Renderer geometry in normalized track coordinates. Raw detent values
    /// are converted by the core/adapter before constructing the spec.
    pub detents: Vec<f64>,
    pub value_text: String,
    pub aria_label: String,
}
impl FaderSpec {
    pub fn new(value: f64, min: f64, max: f64, law: AudioValueLaw) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state: AudioControlVisualState::from_value(value, min, max, law, true),
            min,
            max,
            law,
            orientation: Orientation::Vertical,
            detents: vec![],
            value_text: value.to_string(),
            aria_label: "Fader value".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioMeterSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub channels: Vec<AudioMeterVisualState>,
    pub style: AudioMeterStyle,
    pub orientation: Orientation,
    pub segments: usize,
    pub min_db: f64,
    pub max_db: f64,
    pub value_text: String,
    pub aria_label: String,
}
impl AudioMeterSpec {
    pub fn new(channel: AudioMeterVisualState) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            channels: vec![channel],
            style: AudioMeterStyle::Segments,
            orientation: Orientation::Vertical,
            segments: 20,
            min_db: -60.0,
            max_db: 0.0,
            value_text: "-60 dB".into(),
            aria_label: "Audio level".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueReadoutSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: AudioControlVisualState,
    pub text: String,
    pub aria_label: Option<String>,
}
impl ValueReadoutSpec {
    pub fn new(visual_state: AudioControlVisualState, text: impl Into<String>) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            text: text.into(),
            aria_label: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DragNumberFieldSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: AudioControlVisualState,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub text: String,
    pub aria_label: String,
}
impl DragNumberFieldSpec {
    pub fn new(value: f64, min: f64, max: f64, step: f64, text: impl Into<String>) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state: AudioControlVisualState::from_value(
                value,
                min,
                max,
                AudioValueLaw::Stepped {
                    step,
                    law: poodle_headless::audio::ContinuousAudioValueLaw::Linear,
                },
                true,
            ),
            min,
            max,
            step,
            text: text.into(),
            aria_label: "Value".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvelopeEditorSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: EnvelopeVisualState,
    pub aria_label: String,
}
impl EnvelopeEditorSpec {
    pub fn new(visual_state: EnvelopeVisualState) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            aria_label: "Envelope".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct XYPadSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: XYPadVisualState,
    pub aria_label: String,
    pub x_value_text: String,
    pub y_value_text: String,
}
impl XYPadSpec {
    pub fn new(visual_state: XYPadVisualState) -> Self {
        let x = visual_state.raw_x.to_string();
        let y = visual_state.raw_y.to_string();
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            aria_label: "XY position".into(),
            x_value_text: x,
            y_value_text: y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioSwitchSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: AudioSwitchVisualState,
    pub mode: AudioSwitchMode,
    pub aria_label: String,
}
impl AudioSwitchSpec {
    pub fn new(visual_state: AudioSwitchVisualState, mode: AudioSwitchMode) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            mode,
            aria_label: "Audio switch".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GainReductionMeterSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: GainReductionVisualState,
    pub style: AudioMeterStyle,
    pub orientation: Orientation,
    pub segments: usize,
    pub max_reduction_db: f64,
    pub value_text: String,
    pub aria_label: String,
}
impl GainReductionMeterSpec {
    pub fn new(visual_state: GainReductionVisualState, max_reduction_db: f64) -> Self {
        let value_text = format!("{} dB reduction", visual_state.reduction_db);
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            style: AudioMeterStyle::Segments,
            orientation: Orientation::Vertical,
            segments: 20,
            max_reduction_db,
            value_text,
            aria_label: "Gain reduction".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: KeyboardVisualState,
    pub aria_label: String,
}
impl KeyboardSpec {
    pub fn new(visual_state: KeyboardVisualState) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            aria_label: "Keyboard".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaveformDisplaySpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: WaveformVisualState,
    pub aria_label: String,
}
impl WaveformDisplaySpec {
    pub fn new(visual_state: WaveformVisualState) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            aria_label: "Waveform".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixGridSpec {
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub visual_state: ModMatrixVisualState,
    pub aria_label: String,
}
impl ModMatrixGridSpec {
    pub fn new(visual_state: ModMatrixVisualState) -> Self {
        Self {
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            visual_state,
            aria_label: "Modulation matrix".into(),
        }
    }
}
