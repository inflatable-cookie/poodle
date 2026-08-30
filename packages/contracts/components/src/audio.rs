//! Renderer-neutral native specs for the audio component family.

use poodle_headless::audio::{
    AudioControlVisualState, AudioMeterVisualState, AudioSwitchMode, AudioSwitchVisualState,
    AudioValueFormat, AudioValueLaw, EnvelopeVisualState, GainReductionVisualState,
    KeyboardVisualState, KnobDragMode, ModMatrixVisualState, WaveformVisualState, XYPadVisualState,
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
    pub default_value: f64,
    pub keyboard_step: f64,
    pub format: AudioValueFormat,
    pub drag_mode: KnobDragMode,
    pub drag_sensitivity: f64,
    pub entry_open: bool,
    pub entry_draft: String,
    pub drag_start_value: f64,
    pub drag_start_position: f64,
    pub pointer_position: f64,
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
            default_value: 0.0,
            keyboard_step: 0.01,
            format: AudioValueFormat::Number { decimals: 2 },
            drag_mode: KnobDragMode::Vertical,
            drag_sensitivity: 160.0,
            entry_open: false,
            entry_draft: String::new(),
            drag_start_value: value,
            drag_start_position: 0.0,
            pointer_position: 0.0,
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
    /// Plain detent values. The renderer normalizes them for drawing.
    pub detents: Vec<f64>,
    pub detent_snap: f64,
    pub default_value: f64,
    pub keyboard_step: f64,
    pub format: AudioValueFormat,
    pub entry_open: bool,
    pub entry_draft: String,
    pub drag_start_value: f64,
    pub drag_start_position: f64,
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
            detent_snap: 0.015,
            default_value: 0.0,
            keyboard_step: 0.01,
            format: AudioValueFormat::Number { decimals: 2 },
            entry_open: false,
            entry_draft: String::new(),
            drag_start_value: value,
            drag_start_position: 0.0,
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
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub law_x: AudioValueLaw,
    pub law_y: AudioValueLaw,
    pub default_x: f64,
    pub default_y: f64,
    pub keyboard_step_x: f64,
    pub keyboard_step_y: f64,
    pub format_x: AudioValueFormat,
    pub format_y: AudioValueFormat,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    pub drag_start_norm_x: f64,
    pub drag_start_norm_y: f64,
    pub aria_label: String,
    pub x_label: String,
    pub y_label: String,
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
            min_x: 0.0,
            max_x: 1.0,
            min_y: 0.0,
            max_y: 1.0,
            law_x: AudioValueLaw::Linear,
            law_y: AudioValueLaw::Linear,
            default_x: 0.0,
            default_y: 0.0,
            keyboard_step_x: 0.01,
            keyboard_step_y: 0.01,
            format_x: AudioValueFormat::Number { decimals: 2 },
            format_y: AudioValueFormat::Number { decimals: 2 },
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            drag_start_norm_x: 0.0,
            drag_start_norm_y: 0.0,
            aria_label: "XY position".into(),
            x_label: "X".into(),
            y_label: "Y".into(),
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
