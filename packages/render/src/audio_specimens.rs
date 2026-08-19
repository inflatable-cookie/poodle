//! Deterministic native specimen content for the audio family.
//!
//! GPUI and Jetstream both consume these exact node trees. Keeping the state
//! matrix here prevents their coverage from drifting while their preview
//! shells remain backend-owned.
//!
//! [`AudioSpecimen`] hands out the three parts a specimen page is made of —
//! the curated Examples pane, one representative at a requested size, one at a
//! requested density — and never a combined page. Whichever of those parts a
//! preview shows, and in what layout, is the shell's decision.

use poodle_adapter::ThemeProvider;
use poodle_headless::audio::{
    format_value, switch_visual_state, AudioControlVisualState, AudioMeterVisualState,
    AudioSwitchMode, AudioValueFormat, AudioValueLaw, AutomationState, DragState,
    EnvelopeVisualPoint, EnvelopeVisualState, GainReductionVisualState, KeyboardContext,
    KeyboardOrientation, ModMatrixCell, ModMatrixCellParameters, ModMatrixContext, ModMatrixHeader,
    WaveformContext, WaveformPeakLevel, WaveformPeakPair, WaveformPeakPyramid, XYPadVisualState,
};
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{
    AudioMeterSpec, AudioMeterStyle, AudioSwitchSpec, ControlDensity, ControlSize,
    DragNumberFieldSpec, EnvelopeEditorSpec, FaderSpec, GainReductionMeterSpec, KeyboardSpec,
    KnobSpec, ModMatrixGridSpec, Orientation, ValueReadoutSpec, WaveformDisplaySpec, XYPadSpec,
};

trait AudioPresentationSpec: Clone {
    fn set_size(&mut self, size: ControlSize);
    fn set_density(&mut self, density: ControlDensity);
}

macro_rules! impl_audio_presentation_spec {
    ($($type:ty),+ $(,)?) => {$(
        impl AudioPresentationSpec for $type {
            fn set_size(&mut self, size: ControlSize) { self.size = size; }
            fn set_density(&mut self, density: ControlDensity) { self.density = density; }
        }
    )+};
}

impl_audio_presentation_spec!(
    KnobSpec,
    FaderSpec,
    AudioMeterSpec,
    ValueReadoutSpec,
    DragNumberFieldSpec,
    EnvelopeEditorSpec,
    XYPadSpec,
    AudioSwitchSpec,
    GainReductionMeterSpec,
    KeyboardSpec,
    WaveformDisplaySpec,
    ModMatrixGridSpec,
);

/// The twelve audio controls with a shared native specimen page.
///
/// GPUI asks for the parts its axis-aware layout needs — the curated Examples
/// pane, one representative at a requested size, one at a requested density —
/// and composes them itself. Nothing here returns a combined page, so a
/// consumer cannot show an axis matrix inside Examples by accident.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioSpecimen {
    AudioMeter,
    AudioSwitch,
    DragNumberField,
    EnvelopeEditor,
    Fader,
    GainReductionMeter,
    Keyboard,
    Knob,
    ModMatrixGrid,
    ValueReadout,
    WaveformDisplay,
    XyPad,
}

impl AudioSpecimen {
    /// The curated Examples pane: the states this control has to teach.
    pub fn examples(self, theme: &dyn ThemeProvider) -> Node {
        match self {
            Self::AudioMeter => audio_meter_examples(theme),
            Self::AudioSwitch => audio_switch_examples(theme),
            Self::DragNumberField => drag_number_field_examples(theme),
            Self::EnvelopeEditor => envelope_editor_examples(theme),
            Self::Fader => fader_examples(theme),
            Self::GainReductionMeter => gain_reduction_meter_examples(theme),
            Self::Keyboard => keyboard_examples(theme),
            Self::Knob => knob_examples(theme),
            Self::ModMatrixGrid => mod_matrix_grid_examples(theme),
            Self::ValueReadout => value_readout_examples(theme),
            Self::WaveformDisplay => waveform_display_examples(theme),
            Self::XyPad => xy_pad_examples(theme),
        }
    }

    /// One ordinary representative at `size`. Nothing else varies.
    pub fn size(self, size: ControlSize, theme: &dyn ThemeProvider) -> Node {
        match self {
            Self::AudioMeter => at_size(audio_meter_base(), size, super::audio_meter, theme),
            Self::AudioSwitch => at_size(audio_switch_base(), size, super::audio_switch, theme),
            Self::DragNumberField => at_size(
                drag_number_field_base(),
                size,
                super::drag_number_field,
                theme,
            ),
            Self::EnvelopeEditor => {
                at_size(envelope_editor_base(), size, super::envelope_editor, theme)
            }
            Self::Fader => at_size(fader_base(), size, super::fader, theme),
            Self::GainReductionMeter => at_size(
                gain_reduction_meter_base(),
                size,
                super::gain_reduction_meter,
                theme,
            ),
            Self::Keyboard => at_size(keyboard_base(), size, super::keyboard, theme),
            Self::Knob => at_size(knob_base(), size, super::knob, theme),
            Self::ModMatrixGrid => {
                at_size(mod_matrix_grid_base(), size, super::mod_matrix_grid, theme)
            }
            Self::ValueReadout => at_size(value_readout_base(), size, super::value_readout, theme),
            Self::WaveformDisplay => at_size(
                waveform_display_base(),
                size,
                super::waveform_display,
                theme,
            ),
            Self::XyPad => at_size(xy_pad_base(), size, super::xy_pad, theme),
        }
    }

    /// One ordinary representative at `density`. Nothing else varies.
    pub fn density(self, density: ControlDensity, theme: &dyn ThemeProvider) -> Node {
        match self {
            Self::AudioMeter => at_density(audio_meter_base(), density, super::audio_meter, theme),
            Self::AudioSwitch => {
                at_density(audio_switch_base(), density, super::audio_switch, theme)
            }
            Self::DragNumberField => at_density(
                drag_number_field_base(),
                density,
                super::drag_number_field,
                theme,
            ),
            Self::EnvelopeEditor => at_density(
                envelope_editor_base(),
                density,
                super::envelope_editor,
                theme,
            ),
            Self::Fader => at_density(fader_base(), density, super::fader, theme),
            Self::GainReductionMeter => at_density(
                gain_reduction_meter_base(),
                density,
                super::gain_reduction_meter,
                theme,
            ),
            Self::Keyboard => at_density(keyboard_base(), density, super::keyboard, theme),
            Self::Knob => at_density(knob_base(), density, super::knob, theme),
            Self::ModMatrixGrid => at_density(
                mod_matrix_grid_base(),
                density,
                super::mod_matrix_grid,
                theme,
            ),
            Self::ValueReadout => {
                at_density(value_readout_base(), density, super::value_readout, theme)
            }
            Self::WaveformDisplay => at_density(
                waveform_display_base(),
                density,
                super::waveform_display,
                theme,
            ),
            Self::XyPad => at_density(xy_pad_base(), density, super::xy_pad, theme),
        }
    }
}

// ── Axis bases ────────────────────────────────────────────────────────────
//
// One ordinary state per control. The axis panes vary a single presentation
// prop against these; every other prop stays put.

fn knob_base() -> KnobSpec {
    KnobSpec::new(0.6, 0.0, 1.0, AudioValueLaw::Linear)
}

fn fader_base() -> FaderSpec {
    FaderSpec::new(0.6, 0.0, 1.0, AudioValueLaw::Linear)
}

fn audio_meter_base() -> AudioMeterSpec {
    AudioMeterSpec::new(meter_visual(0.65, Some(0.72), false, true))
}

fn value_readout_base() -> ValueReadoutSpec {
    ValueReadoutSpec::new(
        control(-12.4, -60.0, 6.0, AudioValueLaw::Linear),
        "-12.4 dB",
    )
}

fn drag_number_field_base() -> DragNumberFieldSpec {
    DragNumberFieldSpec::new(-12.4, -60.0, 12.0, 0.1, "-12.4 dB")
}

fn envelope_editor_base() -> EnvelopeEditorSpec {
    envelope_spec(&ADSR, true)
}

fn xy_pad_base() -> XYPadSpec {
    xy_spec(0.4, 0.6, DragState::None, AutomationState::None, true)
}

fn audio_switch_base() -> AudioSwitchSpec {
    switch_spec(1, 2, false, None, true, AudioSwitchMode::Latch)
}

fn gain_reduction_meter_base() -> GainReductionMeterSpec {
    reduction_spec(12.0, true)
}

fn keyboard_base() -> KeyboardSpec {
    keyboard_spec(KeyboardOrientation::Horizontal, true)
}

fn waveform_display_base() -> WaveformDisplaySpec {
    waveform_spec(true)
}

fn mod_matrix_grid_base() -> ModMatrixGridSpec {
    matrix_spec(true)
}

/// One ordinary representative of `base` at `size`. Only the size varies.
fn at_size<S: AudioPresentationSpec>(
    mut base: S,
    size: ControlSize,
    render: fn(&S, &dyn ThemeProvider) -> Node,
    theme: &dyn ThemeProvider,
) -> Node {
    base.set_size(size);
    render(&base, theme)
}

/// One ordinary representative of `base` at `density`. Only the density varies.
fn at_density<S: AudioPresentationSpec>(
    mut base: S,
    density: ControlDensity,
    render: fn(&S, &dyn ThemeProvider) -> Node,
    theme: &dyn ThemeProvider,
) -> Node {
    base.set_density(density);
    render(&base, theme)
}

fn page(groups: Vec<(&str, Vec<Node>)>, theme: &dyn ThemeProvider) -> Node {
    let mut root = Node::container();
    root.style.descriptor.layout.spacing.gap = 24.0;
    for (title, children) in groups {
        let mut group = Node::container();
        group.style.descriptor.layout.spacing.gap = 8.0;
        let mut heading = Node::text(title);
        heading.style.text_size = Some(11.0);
        heading.style.text_weight = Some(600);
        heading.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.spacing.gap = 16.0;
        row.style.flex_wrap = true;
        row.children = children;
        group.children = vec![heading, row];
        root.children.push(group);
    }
    root
}

fn control(value: f64, min: f64, max: f64, law: AudioValueLaw) -> AudioControlVisualState {
    AudioControlVisualState::from_value(value, min, max, law, true)
}

fn knob_node(
    value: f64,
    min: f64,
    max: f64,
    law: AudioValueLaw,
    label: &str,
    theme: &dyn ThemeProvider,
) -> Node {
    let mut spec = KnobSpec::new(value, min, max, law);
    spec.aria_label = label.into();
    spec.value_text = format_value(value, AudioValueFormat::Number { decimals: 2 });
    super::knob(&spec, theme)
}

fn knob_examples(theme: &dyn ThemeProvider) -> Node {
    let bipolar = AudioValueLaw::BipolarCenter { center: 0.0 };
    let mut fine = KnobSpec::new(0.42, 0.0, 1.0, AudioValueLaw::Linear);
    fine.visual_state.drag = DragState::Fine;
    let mut automated = KnobSpec::new(0.7, 0.0, 1.0, AudioValueLaw::Linear);
    automated.visual_state.automation = AutomationState::Writing;
    let mut disabled = KnobSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear);
    disabled.visual_state.enabled = false;
    page(
        vec![
            (
                "Linear / default reset",
                vec![knob_node(
                    0.5,
                    0.0,
                    1.0,
                    AudioValueLaw::Linear,
                    "Linear knob",
                    theme,
                )],
            ),
            (
                "Logarithmic frequency",
                vec![knob_node(
                    1000.0,
                    20.0,
                    20_000.0,
                    AudioValueLaw::Logarithmic,
                    "Frequency",
                    theme,
                )],
            ),
            (
                "Bipolar center",
                vec![knob_node(0.0, -1.0, 1.0, bipolar, "Pan", theme)],
            ),
            (
                "Stepped values",
                vec![knob_node(
                    4.0,
                    0.0,
                    10.0,
                    AudioValueLaw::Stepped {
                        step: 1.0,
                        law: Default::default(),
                    },
                    "Stepped knob",
                    theme,
                )],
            ),
            ("Fine drag", vec![super::knob(&fine, theme)]),
            (
                "Circular mode",
                vec![knob_node(
                    0.8,
                    0.0,
                    1.0,
                    AudioValueLaw::Linear,
                    "Circular drag",
                    theme,
                )],
            ),
            ("Automation state", vec![super::knob(&automated, theme)]),
            (
                "Type-in and keyboard bounds",
                vec![
                    knob_node(
                        0.25,
                        0.0,
                        1.0,
                        AudioValueLaw::Linear,
                        "Type-in value",
                        theme,
                    ),
                    knob_node(0.0, 0.0, 1.0, AudioValueLaw::Linear, "Minimum", theme),
                    knob_node(1.0, 0.0, 1.0, AudioValueLaw::Linear, "Maximum", theme),
                ],
            ),
            ("Disabled", vec![super::knob(&disabled, theme)]),
        ],
        theme,
    )
}

fn fader_node(
    value: f64,
    orientation: Orientation,
    law: AudioValueLaw,
    theme: &dyn ThemeProvider,
) -> Node {
    let (min, max) = if matches!(law, AudioValueLaw::Logarithmic) {
        (20.0, 20_000.0)
    } else if matches!(law, AudioValueLaw::BipolarCenter { .. }) {
        (-1.0, 1.0)
    } else {
        (0.0, 1.0)
    };
    let mut spec = FaderSpec::new(value, min, max, law);
    spec.orientation = orientation;
    spec.value_text = format_value(value, AudioValueFormat::Number { decimals: 2 });
    super::fader(&spec, theme)
}

fn fader_examples(theme: &dyn ThemeProvider) -> Node {
    let mut detents = FaderSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear);
    detents.detents = vec![0.25, 0.5, 0.75];
    let mut fine = FaderSpec::new(0.4, 0.0, 1.0, AudioValueLaw::Linear);
    fine.visual_state.drag = DragState::Fine;
    let mut automation = FaderSpec::new(0.7, 0.0, 1.0, AudioValueLaw::Linear);
    automation.visual_state.automation = AutomationState::Touched;
    let mut disabled = FaderSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear);
    disabled.visual_state.enabled = false;
    page(
        vec![
            (
                "Vertical and horizontal",
                vec![
                    fader_node(0.65, Orientation::Vertical, AudioValueLaw::Linear, theme),
                    fader_node(0.65, Orientation::Horizontal, AudioValueLaw::Linear, theme),
                ],
            ),
            (
                "Linear / log / bipolar laws",
                vec![
                    fader_node(0.4, Orientation::Vertical, AudioValueLaw::Linear, theme),
                    fader_node(
                        1000.0,
                        Orientation::Vertical,
                        AudioValueLaw::Logarithmic,
                        theme,
                    ),
                    fader_node(
                        0.0,
                        Orientation::Vertical,
                        AudioValueLaw::BipolarCenter { center: 0.0 },
                        theme,
                    ),
                ],
            ),
            ("Detents", vec![super::fader(&detents, theme)]),
            (
                "Coarse / fine drag",
                vec![
                    fader_node(0.4, Orientation::Vertical, AudioValueLaw::Linear, theme),
                    super::fader(&fine, theme),
                ],
            ),
            ("Automation touch", vec![super::fader(&automation, theme)]),
            (
                "Type-in",
                vec![fader_node(
                    0.25,
                    Orientation::Vertical,
                    AudioValueLaw::Linear,
                    theme,
                )],
            ),
            (
                "Keyboard bounds",
                vec![
                    fader_node(0.0, Orientation::Vertical, AudioValueLaw::Linear, theme),
                    fader_node(1.0, Orientation::Vertical, AudioValueLaw::Linear, theme),
                ],
            ),
            ("Disabled", vec![super::fader(&disabled, theme)]),
        ],
        theme,
    )
}

fn meter_visual(value: f64, peak: Option<f64>, clip: bool, enabled: bool) -> AudioMeterVisualState {
    AudioMeterVisualState {
        control: AudioControlVisualState {
            value_norm: value,
            raw_value: -60.0 + value * 60.0,
            bipolar_center: None,
            hover: false,
            focus: false,
            drag: DragState::None,
            automation: AutomationState::None,
            enabled,
        },
        ballistic_value: value,
        peak_hold: peak,
        clip,
    }
}

fn meter_node(
    value: f64,
    style: AudioMeterStyle,
    orientation: Orientation,
    stereo: bool,
    clip: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    let visual = meter_visual(value, Some((value + 0.08).min(1.0)), clip, true);
    let mut spec = AudioMeterSpec::new(visual.clone());
    if stereo {
        spec.channels.push(meter_visual(
            (value - 0.12).max(0.0),
            Some(value),
            false,
            true,
        ));
    }
    spec.style = style;
    spec.orientation = orientation;
    spec.value_text = format_value(-60.0 + value * 60.0, AudioValueFormat::Db { decimals: 1 });
    super::audio_meter(&spec, theme)
}

fn audio_meter_examples(theme: &dyn ThemeProvider) -> Node {
    page(
        vec![
            (
                "VU — 300 ms integration",
                vec![meter_node(
                    0.58,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    false,
                    theme,
                )],
            ),
            (
                "PPM",
                vec![meter_node(
                    0.72,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    false,
                    theme,
                )],
            ),
            (
                "Sample peak",
                vec![meter_node(
                    0.86,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    false,
                    theme,
                )],
            ),
            (
                "RMS window",
                vec![meter_node(
                    0.48,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    false,
                    theme,
                )],
            ),
            (
                "Bar and segment styles",
                vec![
                    meter_node(
                        0.66,
                        AudioMeterStyle::Bar,
                        Orientation::Horizontal,
                        false,
                        false,
                        theme,
                    ),
                    meter_node(
                        0.66,
                        AudioMeterStyle::Segments,
                        Orientation::Horizontal,
                        false,
                        false,
                        theme,
                    ),
                ],
            ),
            (
                "Mono and stereo",
                vec![
                    meter_node(
                        0.7,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        false,
                        false,
                        theme,
                    ),
                    meter_node(
                        0.7,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        true,
                        false,
                        theme,
                    ),
                ],
            ),
            (
                "Vertical and horizontal",
                vec![
                    meter_node(
                        0.62,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        false,
                        false,
                        theme,
                    ),
                    meter_node(
                        0.62,
                        AudioMeterStyle::Segments,
                        Orientation::Horizontal,
                        false,
                        false,
                        theme,
                    ),
                ],
            ),
            (
                "Peak hold",
                vec![meter_node(
                    0.52,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    false,
                    theme,
                )],
            ),
            (
                "Clip latch and manual reset",
                vec![
                    meter_node(
                        1.0,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        false,
                        true,
                        theme,
                    ),
                    meter_node(
                        0.2,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        false,
                        false,
                        theme,
                    ),
                ],
            ),
        ],
        theme,
    )
}

fn value_readout_examples(theme: &dyn ThemeProvider) -> Node {
    let values = [
        ("Number", 12.345, AudioValueFormat::Number { decimals: 2 }),
        ("dB", -12.4, AudioValueFormat::Db { decimals: 1 }),
        ("Hz / kHz", 12_500.0, AudioValueFormat::Hz { decimals: 2 }),
        ("Percent", 0.625, AudioValueFormat::Percent { decimals: 1 }),
        ("Ratio", 4.0, AudioValueFormat::Ratio { decimals: 2 }),
        (
            "Milliseconds",
            1250.0,
            AudioValueFormat::Milliseconds { decimals: 2 },
        ),
        ("Note name", 60.0, AudioValueFormat::Note),
        (
            "Semitones",
            -7.0,
            AudioValueFormat::Semitones { decimals: 1 },
        ),
    ];
    let mut groups = values
        .into_iter()
        .map(|(title, value, format)| {
            let visual = control(value, -20_000.0, 20_000.0, AudioValueLaw::Linear);
            let spec = ValueReadoutSpec::new(visual, format_value(value, format));
            (title, vec![super::value_readout(&spec, theme)])
        })
        .collect::<Vec<_>>();
    let mut disabled =
        ValueReadoutSpec::new(control(0.0, -1.0, 1.0, AudioValueLaw::Linear), "Disabled");
    disabled.visual_state.enabled = false;
    groups.push((
        "Negative / boundary / disabled",
        vec![
            super::value_readout(
                &ValueReadoutSpec::new(control(-1.0, -1.0, 1.0, AudioValueLaw::Linear), "-1"),
                theme,
            ),
            super::value_readout(
                &ValueReadoutSpec::new(control(1.0, -1.0, 1.0, AudioValueLaw::Linear), "+1"),
                theme,
            ),
            super::value_readout(&disabled, theme),
        ],
    ));
    page(groups, theme)
}

fn drag_field(
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    text: &str,
    theme: &dyn ThemeProvider,
) -> Node {
    super::drag_number_field(
        &DragNumberFieldSpec::new(value, min, max, step, text),
        theme,
    )
}

fn drag_number_field_examples(theme: &dyn ThemeProvider) -> Node {
    let mut fine = DragNumberFieldSpec::new(0.4, 0.0, 1.0, 0.01, "0.4");
    fine.visual_state.drag = DragState::Fine;
    let mut direct = DragNumberFieldSpec::new(-12.0, -60.0, 12.0, 0.1, "-12 dB");
    direct.visual_state.focus = true;
    let mut disabled = DragNumberFieldSpec::new(0.5, 0.0, 1.0, 0.01, "0.5");
    disabled.visual_state.enabled = false;
    page(
        vec![
            (
                "Default",
                vec![drag_field(0.5, 0.0, 1.0, 0.01, "0.5", theme)],
            ),
            (
                "Integer step",
                vec![drag_field(4.0, 0.0, 10.0, 1.0, "4", theme)],
            ),
            (
                "Formatted dB",
                vec![drag_field(-12.4, -60.0, 12.0, 0.1, "-12.4 dB", theme)],
            ),
            (
                "Coarse / fine drag",
                vec![
                    drag_field(0.4, 0.0, 1.0, 0.01, "0.4", theme),
                    super::drag_number_field(&fine, theme),
                ],
            ),
            (
                "Direct entry",
                vec![super::drag_number_field(&direct, theme)],
            ),
            (
                "Keyboard bounds",
                vec![
                    drag_field(0.0, 0.0, 1.0, 0.1, "0", theme),
                    drag_field(1.0, 0.0, 1.0, 0.1, "1", theme),
                ],
            ),
            (
                "Negative range",
                vec![drag_field(-7.0, -24.0, 24.0, 1.0, "-7", theme)],
            ),
            ("Disabled", vec![super::drag_number_field(&disabled, theme)]),
        ],
        theme,
    )
}

fn envelope(
    points: &[(f64, f64, f64, bool, bool)],
    enabled: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    super::envelope_editor(&envelope_spec(points, enabled), theme)
}

fn envelope_spec(points: &[(f64, f64, f64, bool, bool)], enabled: bool) -> EnvelopeEditorSpec {
    EnvelopeEditorSpec::new(EnvelopeVisualState {
        points: points
            .iter()
            .enumerate()
            .map(
                |(i, (x, y, curve, selected, dragging))| EnvelopeVisualPoint {
                    id: format!("p{i}"),
                    x_norm: *x,
                    y_norm: *y,
                    curve: *curve,
                    selected: *selected,
                    dragging: *dragging,
                },
            )
            .collect(),
        hover_point_id: None,
        focus: true,
        enabled,
    })
}

/// The ADSR-like envelope both the Examples pane and the axis representatives
/// draw from.
const ADSR: [(f64, f64, f64, bool, bool); 5] = [
    (0.0, 0.0, 0.0, false, false),
    (0.12, 1.0, -0.35, true, false),
    (0.35, 0.62, 0.25, false, false),
    (0.82, 0.62, 0.0, false, false),
    (1.0, 0.0, 0.0, false, false),
];

fn envelope_editor_examples(theme: &dyn ThemeProvider) -> Node {
    let adsr = ADSR;
    page(
        vec![
            ("ADSR-like default", vec![envelope(&adsr, true, theme)]),
            (
                "Positive / negative curves",
                vec![envelope(
                    &[
                        (0.0, 0.0, 0.7, false, false),
                        (0.5, 1.0, -0.7, false, false),
                        (1.0, 0.0, 0.0, false, false),
                    ],
                    true,
                    theme,
                )],
            ),
            (
                "Selected and dragging points",
                vec![envelope(
                    &[
                        (0.0, 0.0, 0.0, false, false),
                        (0.5, 0.8, 0.0, true, true),
                        (1.0, 0.0, 0.0, false, false),
                    ],
                    true,
                    theme,
                )],
            ),
            (
                "Add / remove",
                vec![
                    envelope(&adsr[..3], true, theme),
                    envelope(&adsr[..2], true, theme),
                ],
            ),
            (
                "Snapped movement",
                vec![envelope(
                    &[
                        (0.0, 0.0, 0.0, false, false),
                        (0.5, 0.5, 0.0, true, false),
                        (1.0, 0.0, 0.0, false, false),
                    ],
                    true,
                    theme,
                )],
            ),
            ("Keyboard nudges", vec![envelope(&adsr, true, theme)]),
            (
                "Curve nudges",
                vec![envelope(
                    &[(0.0, 0.0, 0.5, true, false), (1.0, 1.0, 0.0, false, false)],
                    true,
                    theme,
                )],
            ),
            (
                "Flat segment regression",
                vec![envelope(
                    &[(0.0, 0.5, 0.8, false, false), (1.0, 0.5, 0.0, false, false)],
                    true,
                    theme,
                )],
            ),
            ("Disabled", vec![envelope(&adsr, false, theme)]),
        ],
        theme,
    )
}

fn xy(
    x: f64,
    y: f64,
    drag: DragState,
    automation: AutomationState,
    enabled: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    super::xy_pad(&xy_spec(x, y, drag, automation, enabled), theme)
}

fn xy_spec(
    x: f64,
    y: f64,
    drag: DragState,
    automation: AutomationState,
    enabled: bool,
) -> XYPadSpec {
    XYPadSpec::new(XYPadVisualState {
        x_norm: x,
        y_norm: y,
        raw_x: x,
        raw_y: y,
        hover: false,
        focus: true,
        drag,
        automation,
        enabled,
    })
}

fn xy_pad_examples(theme: &dyn ThemeProvider) -> Node {
    page(
        vec![
            (
                "Centered / default",
                vec![xy(
                    0.5,
                    0.5,
                    DragState::None,
                    AutomationState::None,
                    true,
                    theme,
                )],
            ),
            (
                "Corners",
                vec![
                    xy(
                        0.0,
                        0.0,
                        DragState::None,
                        AutomationState::None,
                        true,
                        theme,
                    ),
                    xy(
                        1.0,
                        1.0,
                        DragState::None,
                        AutomationState::None,
                        true,
                        theme,
                    ),
                ],
            ),
            (
                "Independent nonlinear laws",
                vec![xy(
                    0.32,
                    0.76,
                    DragState::None,
                    AutomationState::None,
                    true,
                    theme,
                )],
            ),
            (
                "Coarse / fine drag",
                vec![
                    xy(
                        0.4,
                        0.6,
                        DragState::Coarse,
                        AutomationState::None,
                        true,
                        theme,
                    ),
                    xy(
                        0.4,
                        0.6,
                        DragState::Fine,
                        AutomationState::None,
                        true,
                        theme,
                    ),
                ],
            ),
            (
                "Reset",
                vec![xy(
                    0.5,
                    0.5,
                    DragState::None,
                    AutomationState::None,
                    true,
                    theme,
                )],
            ),
            (
                "Automation state",
                vec![xy(
                    0.7,
                    0.2,
                    DragState::None,
                    AutomationState::Writing,
                    true,
                    theme,
                )],
            ),
            (
                "Keyboard axis bounds",
                vec![xy(
                    0.0,
                    1.0,
                    DragState::None,
                    AutomationState::None,
                    true,
                    theme,
                )],
            ),
            (
                "Disabled",
                vec![xy(
                    0.5,
                    0.5,
                    DragState::None,
                    AutomationState::None,
                    false,
                    theme,
                )],
            ),
        ],
        theme,
    )
}

fn switch(
    state: usize,
    count: usize,
    pressed: bool,
    lamp: Option<bool>,
    enabled: bool,
    mode: AudioSwitchMode,
    theme: &dyn ThemeProvider,
) -> Node {
    super::audio_switch(
        &switch_spec(state, count, pressed, lamp, enabled, mode),
        theme,
    )
}

fn switch_spec(
    state: usize,
    count: usize,
    pressed: bool,
    lamp: Option<bool>,
    enabled: bool,
    mode: AudioSwitchMode,
) -> AudioSwitchSpec {
    AudioSwitchSpec::new(
        switch_visual_state(mode, state, count, pressed, lamp, enabled),
        mode,
    )
}

fn audio_switch_examples(theme: &dyn ThemeProvider) -> Node {
    page(
        vec![
            (
                "Off / on latch",
                vec![
                    switch(0, 2, false, None, true, AudioSwitchMode::Latch, theme),
                    switch(1, 2, false, None, true, AudioSwitchMode::Latch, theme),
                ],
            ),
            (
                "Held / released momentary",
                vec![
                    switch(1, 2, true, None, true, AudioSwitchMode::Momentary, theme),
                    switch(0, 2, false, None, true, AudioSwitchMode::Momentary, theme),
                ],
            ),
            (
                "Three-state cycle",
                vec![
                    switch(0, 3, false, None, true, AudioSwitchMode::Multi, theme),
                    switch(1, 3, false, None, true, AudioSwitchMode::Multi, theme),
                    switch(2, 3, false, None, true, AudioSwitchMode::Multi, theme),
                ],
            ),
            (
                "Lamp override",
                vec![
                    switch(0, 2, false, Some(true), true, AudioSwitchMode::Latch, theme),
                    switch(
                        1,
                        2,
                        false,
                        Some(false),
                        true,
                        AudioSwitchMode::Latch,
                        theme,
                    ),
                ],
            ),
            (
                "Pressed / focused",
                vec![switch(
                    1,
                    2,
                    true,
                    None,
                    true,
                    AudioSwitchMode::Latch,
                    theme,
                )],
            ),
            (
                "Disabled",
                vec![switch(
                    1,
                    2,
                    false,
                    None,
                    false,
                    AudioSwitchMode::Latch,
                    theme,
                )],
            ),
        ],
        theme,
    )
}

fn reduction(
    value: f64,
    style: AudioMeterStyle,
    orientation: Orientation,
    enabled: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    let mut spec = reduction_spec(value, enabled);
    spec.style = style;
    spec.orientation = orientation;
    super::gain_reduction_meter(&spec, theme)
}

fn reduction_spec(value: f64, enabled: bool) -> GainReductionMeterSpec {
    let visual = GainReductionVisualState {
        meter: meter_visual((value / 30.0).clamp(0.0, 1.0), None, false, enabled),
        reduction_db: value,
    };
    GainReductionMeterSpec::new(visual, 30.0)
}

fn gain_reduction_meter_examples(theme: &dyn ThemeProvider) -> Node {
    page(
        vec![
            (
                "No reduction / reset",
                vec![reduction(
                    0.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    true,
                    theme,
                )],
            ),
            (
                "Attack",
                vec![reduction(
                    18.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    true,
                    theme,
                )],
            ),
            (
                "Release",
                vec![reduction(
                    8.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    true,
                    theme,
                )],
            ),
            (
                "Maximum reduction",
                vec![reduction(
                    30.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    true,
                    theme,
                )],
            ),
            (
                "Bar and segment styles",
                vec![
                    reduction(
                        12.0,
                        AudioMeterStyle::Bar,
                        Orientation::Horizontal,
                        true,
                        theme,
                    ),
                    reduction(
                        12.0,
                        AudioMeterStyle::Segments,
                        Orientation::Horizontal,
                        true,
                        theme,
                    ),
                ],
            ),
            (
                "Vertical and horizontal",
                vec![
                    reduction(
                        12.0,
                        AudioMeterStyle::Segments,
                        Orientation::Vertical,
                        true,
                        theme,
                    ),
                    reduction(
                        12.0,
                        AudioMeterStyle::Segments,
                        Orientation::Horizontal,
                        true,
                        theme,
                    ),
                ],
            ),
            (
                "Invalid-frame rejection",
                vec![reduction(
                    12.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    true,
                    theme,
                )],
            ),
            (
                "Disabled",
                vec![reduction(
                    12.0,
                    AudioMeterStyle::Segments,
                    Orientation::Vertical,
                    false,
                    theme,
                )],
            ),
        ],
        theme,
    )
}

fn keyboard_spec(orientation: KeyboardOrientation, enabled: bool) -> KeyboardSpec {
    let mut context = KeyboardContext {
        orientation,
        disabled: !enabled,
        external_held_notes: vec![60, 64, 67],
        ..Default::default()
    };
    if orientation == KeyboardOrientation::Vertical {
        context.first_note = 48;
        context.last_note = 60;
    }
    KeyboardSpec::new(poodle_headless::audio::keyboard_visual_state(&context))
}

fn keyboard_examples(theme: &dyn ThemeProvider) -> Node {
    let base = keyboard_spec(KeyboardOrientation::Horizontal, true);
    page(
        vec![
            (
                "Horizontal input / local chord",
                vec![super::keyboard(&base, theme)],
            ),
            (
                "Vertical piano-roll gutter",
                vec![super::keyboard(
                    &keyboard_spec(KeyboardOrientation::Vertical, true),
                    theme,
                )],
            ),
            ("Velocity depth", vec![super::keyboard(&base, theme)]),
            (
                "Computer keys / octave shift",
                vec![super::keyboard(&base, theme)],
            ),
            (
                "External playback highlight",
                vec![super::keyboard(&base, theme)],
            ),
            (
                "Disabled",
                vec![super::keyboard(
                    &keyboard_spec(KeyboardOrientation::Horizontal, false),
                    theme,
                )],
            ),
        ],
        theme,
    )
}

fn waveform_spec_view(
    enabled: bool,
    visible_start: usize,
    visible_end: usize,
    selection: Option<poodle_headless::audio::WaveformSelection>,
) -> WaveformDisplaySpec {
    let peaks: Vec<WaveformPeakPair> = (0..64)
        .map(|index| {
            let value = (index as f64 * 0.31).sin().abs();
            WaveformPeakPair {
                min: -value * 0.8,
                max: value * 0.9,
            }
        })
        .collect();
    let context = WaveformContext {
        pyramid: WaveformPeakPyramid {
            sample_count: 64,
            levels: vec![WaveformPeakLevel {
                samples_per_peak: 1,
                peaks,
            }],
        },
        visible_start,
        visible_end,
        column_count: 64,
        cursor_sample: Some(24),
        selection,
        selection_anchor: None,
        selecting: false,
        focus: true,
        disabled: !enabled,
    };
    WaveformDisplaySpec::new(context.visual_state())
}

fn waveform_spec(enabled: bool) -> WaveformDisplaySpec {
    waveform_spec_view(
        enabled,
        0,
        64,
        Some(poodle_headless::audio::WaveformSelection { start: 12, end: 42 }),
    )
}

fn waveform_display_examples(theme: &dyn ThemeProvider) -> Node {
    let base = waveform_spec(true);
    page(
        vec![
            (
                "Peak pyramid / cursor",
                vec![super::waveform_display(
                    &waveform_spec_view(true, 0, 64, None),
                    theme,
                )],
            ),
            (
                "Zoomed viewport",
                vec![super::waveform_display(
                    &waveform_spec_view(true, 16, 48, None),
                    theme,
                )],
            ),
            (
                "Forward and ordered selection",
                vec![super::waveform_display(&base, theme)],
            ),
            (
                "Empty",
                vec![super::waveform_display(
                    &WaveformDisplaySpec::new(
                        WaveformContext {
                            pyramid: WaveformPeakPyramid {
                                sample_count: 0,
                                levels: vec![],
                            },
                            visible_start: 0,
                            visible_end: 0,
                            column_count: 64,
                            cursor_sample: None,
                            selection: None,
                            selection_anchor: None,
                            selecting: false,
                            focus: false,
                            disabled: false,
                        }
                        .visual_state(),
                    ),
                    theme,
                )],
            ),
            (
                "Disabled",
                vec![super::waveform_display(&waveform_spec(false), theme)],
            ),
            (
                "Inspector ceiling",
                vec![super::waveform_display(&base, theme)],
            ),
        ],
        theme,
    )
}

fn matrix_spec(enabled: bool) -> ModMatrixGridSpec {
    let sources = vec![
        ModMatrixHeader {
            id: "one".into(),
            label: "Source 1".into(),
        },
        ModMatrixHeader {
            id: "two".into(),
            label: "Source 2".into(),
        },
        ModMatrixHeader {
            id: "three".into(),
            label: "Source 3".into(),
        },
    ];
    let destinations = vec![
        ModMatrixHeader {
            id: "a".into(),
            label: "Dest A".into(),
        },
        ModMatrixHeader {
            id: "b".into(),
            label: "Dest B".into(),
        },
        ModMatrixHeader {
            id: "c".into(),
            label: "Dest C".into(),
        },
    ];
    let mut context = ModMatrixContext::new(
        sources,
        destinations,
        vec![
            ModMatrixCell {
                source_id: "one".into(),
                destination_id: "a".into(),
                amount: 0.75,
                enabled: true,
                parameters: ModMatrixCellParameters::default(),
            },
            ModMatrixCell {
                source_id: "one".into(),
                destination_id: "b".into(),
                amount: -0.5,
                enabled: true,
                parameters: ModMatrixCellParameters::default(),
            },
            ModMatrixCell {
                source_id: "one".into(),
                destination_id: "c".into(),
                amount: 0.35,
                enabled: true,
                parameters: ModMatrixCellParameters {
                    min: 0.0,
                    max: 1.0,
                    step: 0.05,
                    law: AudioValueLaw::Linear,
                },
            },
        ],
    );
    context.focus_row = Some(0);
    context.focus_column = Some(0);
    context.disabled = !enabled;
    ModMatrixGridSpec::new(context.visual_state())
}

fn mod_matrix_grid_examples(theme: &dyn ThemeProvider) -> Node {
    let base = matrix_spec(true);
    page(
        vec![
            (
                "Sparse generic matrix",
                vec![super::mod_matrix_grid(&base, theme)],
            ),
            (
                "Bipolar / negative / unipolar",
                vec![super::mod_matrix_grid(&base, theme)],
            ),
            (
                "Keyboard navigation and toggle",
                vec![super::mod_matrix_grid(&base, theme)],
            ),
            (
                "Empty axes",
                vec![super::mod_matrix_grid(
                    &ModMatrixGridSpec::new(
                        ModMatrixContext::new(vec![], vec![], vec![]).visual_state(),
                    ),
                    theme,
                )],
            ),
            (
                "Disabled",
                vec![super::mod_matrix_grid(&matrix_spec(false), theme)],
            ),
        ],
        theme,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_layout::LayoutSizing;

    const ALL: &[AudioSpecimen] = &[
        AudioSpecimen::AudioMeter,
        AudioSpecimen::AudioSwitch,
        AudioSpecimen::DragNumberField,
        AudioSpecimen::EnvelopeEditor,
        AudioSpecimen::Fader,
        AudioSpecimen::GainReductionMeter,
        AudioSpecimen::Keyboard,
        AudioSpecimen::Knob,
        AudioSpecimen::ModMatrixGrid,
        AudioSpecimen::ValueReadout,
        AudioSpecimen::WaveformDisplay,
        AudioSpecimen::XyPad,
    ];

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn text_of(node: &Node) -> Vec<String> {
        let mut out = Vec::new();
        if let poodle_node::NodeKind::Text { content } = &node.kind {
            out.push(content.clone());
        }
        for child in &node.children {
            out.extend(text_of(child));
        }
        out
    }

    fn node_count(node: &Node) -> usize {
        1 + node.children.iter().map(node_count).sum::<usize>()
    }

    /// The split's whole point: the Examples pane stopped carrying the axis
    /// sweeps, so a consumer cannot show a size matrix inside Examples.
    #[test]
    fn examples_pane_carries_no_axis_matrix() {
        for specimen in ALL {
            let captions = text_of(&specimen.examples(&theme()));
            assert!(
                !captions.iter().any(|line| line.starts_with("Sizes —")),
                "{specimen:?} still lists a size sweep in Examples"
            );
            assert!(
                !captions.iter().any(|line| line.starts_with("Densities —")),
                "{specimen:?} still lists a density sweep in Examples"
            );
        }
    }

    /// One requested step, one representative — not a page, and not a matrix.
    #[test]
    fn each_axis_step_returns_one_representative() {
        for specimen in ALL {
            let examples = node_count(&specimen.examples(&theme()));
            for size in [
                ControlSize::Xs,
                ControlSize::Sm,
                ControlSize::Md,
                ControlSize::Lg,
                ControlSize::Xl,
            ] {
                assert!(
                    node_count(&specimen.size(size, &theme())) < examples,
                    "{specimen:?} at {size:?} returned more than one representative"
                );
            }
            for density in [
                ControlDensity::Compact,
                ControlDensity::Default,
                ControlDensity::Comfortable,
            ] {
                assert!(
                    node_count(&specimen.density(density, &theme())) < examples,
                    "{specimen:?} at {density:?} returned more than one representative"
                );
            }
        }
    }

    /// The requested step actually reaches the control: the knob's own box
    /// grows monotonically across the five control sizes.
    #[test]
    fn the_requested_size_reaches_the_control() {
        let widths: Vec<f32> = [
            ControlSize::Xs,
            ControlSize::Sm,
            ControlSize::Md,
            ControlSize::Lg,
            ControlSize::Xl,
        ]
        .into_iter()
        .map(|size| {
            match AudioSpecimen::Knob
                .size(size, &theme())
                .style
                .descriptor
                .layout
                .width
            {
                LayoutSizing::Fixed(px) => px,
                other => panic!("knob width is {other:?}, not a fixed box"),
            }
        })
        .collect();

        assert!(
            widths.windows(2).all(|pair| pair[0] < pair[1]),
            "knob sizes did not ladder: {widths:?}"
        );
    }

    /// Density moves spacing, not the control box. The fader's rail cross-axis
    /// resolves to 4 / 6 / 8 px across compact / default / comfortable, so this
    /// reads the descriptor the requested density actually decides — a node
    /// count would pass even if the density were dropped on the way through.
    ///
    /// The rail is the first child of `fader-root` (`crate::audio::fader`); if
    /// that ordering changes this fails loudly rather than silently weakening.
    #[test]
    fn the_requested_density_reaches_the_control() {
        for (density, expected) in [
            (ControlDensity::Compact, 4.0_f32),
            (ControlDensity::Default, 6.0),
            (ControlDensity::Comfortable, 8.0),
        ] {
            let fader = AudioSpecimen::Fader.density(density, &theme());
            assert_eq!(fader.id.as_deref(), Some("fader-root"));

            let rail = fader.children.first().expect("fader rail");
            match rail.style.descriptor.layout.width {
                LayoutSizing::Fixed(px) => assert_eq!(
                    px, expected,
                    "fader rail at {density:?} is {px}px, not {expected}px"
                ),
                other => panic!("fader rail width is {other:?}, not a fixed box"),
            }
        }
    }
}
