//! Audio-control laws, formatting, feed integration, and serializable visual
//! state. Mirrors `packages/core/src/audio` without a framework or renderer.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContinuousAudioValueLaw {
    Linear,
    Logarithmic,
    Exponential { exponent: f64 },
    BipolarCenter { center: f64 },
}

impl Default for ContinuousAudioValueLaw {
    fn default() -> Self {
        Self::Linear
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioValueLaw {
    Linear,
    Logarithmic,
    Exponential {
        exponent: f64,
    },
    BipolarCenter {
        center: f64,
    },
    Stepped {
        step: f64,
        law: ContinuousAudioValueLaw,
    },
}

impl Default for AudioValueLaw {
    fn default() -> Self {
        Self::Linear
    }
}

pub fn clamp_value(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        min
    } else {
        value.clamp(min, max)
    }
}

pub fn snap_value(value: f64, min: f64, step: f64) -> f64 {
    if !step.is_finite() || step <= 0.0 {
        value
    } else {
        min + ((value - min) / step).round() * step
    }
}

pub fn constrain_value(value: f64, min: f64, max: f64, law: AudioValueLaw) -> f64 {
    let value = match law {
        AudioValueLaw::Stepped { step, .. } => snap_value(value, min, step),
        _ => value,
    };
    clamp_value(value, min, max)
}

fn continuous_law(law: AudioValueLaw) -> ContinuousAudioValueLaw {
    match law {
        AudioValueLaw::Linear => ContinuousAudioValueLaw::Linear,
        AudioValueLaw::Logarithmic => ContinuousAudioValueLaw::Logarithmic,
        AudioValueLaw::Exponential { exponent } => {
            ContinuousAudioValueLaw::Exponential { exponent }
        }
        AudioValueLaw::BipolarCenter { center } => {
            ContinuousAudioValueLaw::BipolarCenter { center }
        }
        AudioValueLaw::Stepped { law, .. } => law,
    }
}

fn valid_law(law: ContinuousAudioValueLaw, min: f64, max: f64) -> bool {
    match law {
        ContinuousAudioValueLaw::Logarithmic => min > 0.0 && max > min,
        ContinuousAudioValueLaw::Exponential { exponent } => exponent.is_finite() && exponent > 0.0,
        ContinuousAudioValueLaw::BipolarCenter { center } => center > min && center < max,
        _ => true,
    }
}

pub fn normalize_value(value: f64, min: f64, max: f64, law: AudioValueLaw) -> f64 {
    if max <= min {
        return 0.0;
    }
    let base = continuous_law(law);
    assert!(valid_law(base, min, max), "invalid audio value law");
    let plain = constrain_value(value, min, max, law);
    match base {
        ContinuousAudioValueLaw::Linear => (plain - min) / (max - min),
        ContinuousAudioValueLaw::Logarithmic => (plain / min).ln() / (max / min).ln(),
        ContinuousAudioValueLaw::Exponential { exponent } => {
            ((plain - min) / (max - min)).powf(1.0 / exponent)
        }
        ContinuousAudioValueLaw::BipolarCenter { center } if plain <= center => {
            ((plain - min) / (center - min)) * 0.5
        }
        ContinuousAudioValueLaw::BipolarCenter { center } => {
            0.5 + ((plain - center) / (max - center)) * 0.5
        }
    }
}

pub fn denormalize_value(norm: f64, min: f64, max: f64, law: AudioValueLaw) -> f64 {
    if max <= min {
        return min;
    }
    let base = continuous_law(law);
    assert!(valid_law(base, min, max), "invalid audio value law");
    let n = norm.clamp(0.0, 1.0);
    let value = match base {
        ContinuousAudioValueLaw::Linear => min + n * (max - min),
        ContinuousAudioValueLaw::Logarithmic => min * (max / min).powf(n),
        ContinuousAudioValueLaw::Exponential { exponent } => min + n.powf(exponent) * (max - min),
        ContinuousAudioValueLaw::BipolarCenter { center } if n <= 0.5 => {
            min + (n / 0.5) * (center - min)
        }
        ContinuousAudioValueLaw::BipolarCenter { center } => {
            center + ((n - 0.5) / 0.5) * (max - center)
        }
    };
    constrain_value(value, min, max, law)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DragState {
    #[default]
    None,
    Coarse,
    Fine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AutomationState {
    #[default]
    None,
    Touched,
    Latched,
    Writing,
    Read,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioControlVisualState {
    pub value_norm: f64,
    pub raw_value: f64,
    pub bipolar_center: Option<f64>,
    pub hover: bool,
    pub focus: bool,
    pub drag: DragState,
    pub automation: AutomationState,
    pub enabled: bool,
}

impl AudioControlVisualState {
    pub fn from_value(value: f64, min: f64, max: f64, law: AudioValueLaw, enabled: bool) -> Self {
        Self {
            value_norm: normalize_value(value, min, max, law),
            raw_value: value,
            bipolar_center: match continuous_law(law) {
                ContinuousAudioValueLaw::BipolarCenter { center } => Some(center),
                _ => None,
            },
            hover: false,
            focus: false,
            drag: DragState::None,
            automation: AutomationState::None,
            enabled,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KnobDragMode {
    /// Anchored pointer delta over `drag_sensitivity` logical pixels.
    #[default]
    Vertical,
    /// Absolute position on the standard 270 degree sweep.
    Circular,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FaderOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueBound {
    Min,
    Max,
}

/// Shared scalar audio-control context. Mirrors the TypeScript
/// `AudioValueContextBase` plus `AudioValueInteraction`
/// (`packages/core/src/audio/value-controls.ts`).
#[derive(Clone, Debug, PartialEq)]
pub struct AudioValueContext {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub law: AudioValueLaw,
    pub default_value: f64,
    pub keyboard_step: f64,
    pub format: AudioValueFormat,
    pub hover: bool,
    pub focus: bool,
    pub drag: DragState,
    pub automation: AutomationState,
    pub entry_open: bool,
    /// Value anchored when the gesture began or last rebased.
    pub drag_start_value: f64,
    /// Pointer position anchored with `drag_start_value`.
    pub drag_start_position: f64,
    pub disabled: bool,
}

impl Default for AudioValueContext {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            law: AudioValueLaw::Linear,
            default_value: 0.0,
            keyboard_step: 0.01,
            format: AudioValueFormat::Number { decimals: 2 },
            hover: false,
            focus: false,
            drag: DragState::None,
            automation: AutomationState::None,
            entry_open: false,
            drag_start_value: 0.0,
            drag_start_position: 0.0,
            disabled: false,
        }
    }
}

impl AudioValueContext {
    pub fn visual_state(&self) -> AudioControlVisualState {
        let mut state = AudioControlVisualState::from_value(
            self.value,
            self.min,
            self.max,
            self.law,
            !self.disabled,
        );
        state.hover = self.hover;
        state.focus = self.focus;
        state.drag = self.drag;
        state.automation = self.automation;
        state
    }

    pub fn value_text(&self) -> String {
        format_value(self.value, self.format)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KnobContext {
    pub base: AudioValueContext,
    pub drag_mode: KnobDragMode,
    pub drag_sensitivity: f64,
}

impl Default for KnobContext {
    fn default() -> Self {
        Self {
            base: AudioValueContext::default(),
            drag_mode: KnobDragMode::Vertical,
            drag_sensitivity: 160.0,
        }
    }
}

impl KnobContext {
    pub fn visual_state(&self) -> AudioControlVisualState {
        self.base.visual_state()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FaderContext {
    pub base: AudioValueContext,
    pub orientation: FaderOrientation,
    /// Plain snap values.
    pub detents: Vec<f64>,
    /// Normalized snap radius.
    pub detent_snap: f64,
}

impl Default for FaderContext {
    fn default() -> Self {
        Self {
            base: AudioValueContext::default(),
            orientation: FaderOrientation::Vertical,
            detents: Vec::new(),
            detent_snap: 0.015,
        }
    }
}

impl FaderContext {
    pub fn visual_state(&self) -> AudioControlVisualState {
        self.base.visual_state()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioValueEvent {
    Hover {
        value: bool,
    },
    Focus {
        value: bool,
    },
    SetAutomation {
        value: AutomationState,
    },
    /// Host-owned value replacement. Not a user gesture, so it emits nothing.
    SetValue {
        value: f64,
    },
    DragBegin {
        position: f64,
        fine: bool,
    },
    /// Anchored pointer delta. Knob vertical mode only.
    DragMove {
        position: f64,
        fine: bool,
    },
    /// Adapter-resolved normalized position. Knob circular mode and Fader.
    DragSetNorm {
        value_norm: f64,
        fine: bool,
    },
    DragEnd,
    DragCancel,
    Wheel {
        direction: i8,
        fine: bool,
    },
    Reset,
    KeyNudge {
        direction: i8,
        multiplier: f64,
        fine: bool,
    },
    KeyBound {
        bound: ValueBound,
    },
    EntryOpen,
    EntryCancel,
    EntryCommit {
        text: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioValueEffect {
    ValueChange(f64),
    ValueCommit(f64),
    GestureBegin,
    GestureEnd,
    RequestEntryFocus,
}

/// Keys, bounds, reset, wheel, and valid type-in are atomic: one change plus
/// one commit. They are never sustained pointer gestures.
fn atomic_value(context: &mut AudioValueContext, value: f64) -> Vec<AudioValueEffect> {
    let value = constrain_value(value, context.min, context.max, context.law);
    context.value = value;
    vec![
        AudioValueEffect::ValueChange(value),
        AudioValueEffect::ValueCommit(value),
    ]
}

/// Events shared by every scalar audio control. Returns `None` for the pointer
/// events, which each control resolves through its own mapping.
fn common_value_transition(
    context: &mut AudioValueContext,
    event: &AudioValueEvent,
) -> Option<Vec<AudioValueEffect>> {
    match event {
        AudioValueEvent::Hover { value } => {
            context.hover = *value;
            Some(vec![])
        }
        AudioValueEvent::Focus { value } => {
            context.focus = *value;
            Some(vec![])
        }
        AudioValueEvent::SetAutomation { value } => {
            context.automation = *value;
            Some(vec![])
        }
        AudioValueEvent::SetValue { value } => {
            context.value = constrain_value(*value, context.min, context.max, context.law);
            Some(vec![])
        }
        AudioValueEvent::Reset => {
            if context.disabled {
                return Some(vec![]);
            }
            let value = context.default_value;
            Some(atomic_value(context, value))
        }
        AudioValueEvent::Wheel { direction, fine } => {
            if context.disabled {
                return Some(vec![]);
            }
            let scale = if *fine { 0.1 } else { 1.0 };
            let value = context.value + *direction as f64 * context.keyboard_step * scale;
            Some(atomic_value(context, value))
        }
        AudioValueEvent::KeyNudge {
            direction,
            multiplier,
            fine,
        } => {
            if context.disabled {
                return Some(vec![]);
            }
            let scale = if *fine { 0.1 } else { 1.0 };
            let value =
                context.value + *direction as f64 * context.keyboard_step * *multiplier * scale;
            Some(atomic_value(context, value))
        }
        AudioValueEvent::KeyBound { bound } => {
            if context.disabled {
                return Some(vec![]);
            }
            let value = match bound {
                ValueBound::Min => context.min,
                ValueBound::Max => context.max,
            };
            Some(atomic_value(context, value))
        }
        AudioValueEvent::EntryOpen => {
            if context.disabled {
                return Some(vec![]);
            }
            context.entry_open = true;
            Some(vec![AudioValueEffect::RequestEntryFocus])
        }
        AudioValueEvent::EntryCancel => {
            context.entry_open = false;
            Some(vec![])
        }
        AudioValueEvent::EntryCommit { text } => {
            let parsed = parse_value(text, context.format);
            let disabled = context.disabled;
            context.entry_open = false;
            match parsed {
                Some(value) if !disabled => Some(atomic_value(context, value)),
                _ => Some(vec![]),
            }
        }
        _ => None,
    }
}

/// Accepts one pointer gesture. A second begin while one is open is inert, so
/// `GestureBegin` and `GestureEnd` stay paired exactly once per gesture.
fn begin_drag(context: &mut AudioValueContext, position: f64, fine: bool) -> Vec<AudioValueEffect> {
    if context.disabled || context.drag != DragState::None {
        return vec![];
    }
    context.drag = if fine {
        DragState::Fine
    } else {
        DragState::Coarse
    };
    context.drag_start_value = context.value;
    context.drag_start_position = position;
    vec![AudioValueEffect::GestureBegin]
}

/// Terminal for an accepted gesture. Release and cancellation close it the same
/// way and are inert once it is closed, so repeated, stale, lost-capture, and
/// teardown terminals cannot duplicate the pair. A control disabled mid-gesture
/// may still close: stranding it would latch host automation open.
fn end_drag(context: &mut AudioValueContext) -> Vec<AudioValueEffect> {
    if context.drag == DragState::None {
        return vec![];
    }
    context.drag = DragState::None;
    vec![
        AudioValueEffect::ValueCommit(context.value),
        AudioValueEffect::GestureEnd,
    ]
}

/// Coarse/fine switching re-anchors at the current value and current pointer,
/// so holding or releasing the modifier never jumps. The transition that flips
/// the modifier only rebases; travel resumes from the next move.
fn rebase_drag(context: &mut AudioValueContext, position: f64, fine: bool) -> bool {
    let next = if fine {
        DragState::Fine
    } else {
        DragState::Coarse
    };
    if context.drag == next {
        return false;
    }
    context.drag = next;
    context.drag_start_value = context.value;
    context.drag_start_position = position;
    true
}

/// A move is live only inside an accepted gesture on an enabled control.
fn dragging(context: &AudioValueContext) -> bool {
    !context.disabled && context.drag != DragState::None
}

pub fn knob_transition(
    mut context: KnobContext,
    event: AudioValueEvent,
) -> (KnobContext, Vec<AudioValueEffect>) {
    if let Some(effects) = common_value_transition(&mut context.base, &event) {
        return (context, effects);
    }
    let effects = match event {
        AudioValueEvent::DragBegin { position, fine } => {
            begin_drag(&mut context.base, position, fine)
        }
        // Vertical mapping: anchored pointer delta over `drag_sensitivity`.
        AudioValueEvent::DragMove { position, fine } => {
            if context.drag_mode != KnobDragMode::Vertical || !dragging(&context.base) {
                return (context, vec![]);
            }
            if rebase_drag(&mut context.base, position, fine) {
                return (context, vec![]);
            }
            let base = &context.base;
            let scale = if fine { 0.1 } else { 1.0 };
            let start_norm = normalize_value(base.drag_start_value, base.min, base.max, base.law);
            let norm = start_norm
                + ((base.drag_start_position - position) / context.drag_sensitivity.max(1.0))
                    * scale;
            let value = denormalize_value(norm, base.min, base.max, base.law);
            context.base.value = value;
            vec![AudioValueEffect::ValueChange(value)]
        }
        // Circular mapping: the adapter resolves the 270 degree sweep position.
        AudioValueEvent::DragSetNorm { value_norm, fine } => {
            if context.drag_mode != KnobDragMode::Circular || !dragging(&context.base) {
                return (context, vec![]);
            }
            if rebase_drag(&mut context.base, value_norm, fine) {
                return (context, vec![]);
            }
            let base = &context.base;
            let start_norm = normalize_value(base.drag_start_value, base.min, base.max, base.law);
            let target = if fine {
                start_norm + (value_norm - base.drag_start_position) * 0.1
            } else {
                value_norm
            };
            let value = denormalize_value(target, base.min, base.max, base.law);
            context.base.value = value;
            vec![AudioValueEffect::ValueChange(value)]
        }
        AudioValueEvent::DragEnd | AudioValueEvent::DragCancel => end_drag(&mut context.base),
        _ => vec![],
    };
    (context, effects)
}

/// Nearest declared detent inside the normalized snap radius. The radius is
/// inclusive and the first declared detent wins a tie, so two equidistant
/// detents always resolve the same way.
fn snap_fader_detent(context: &FaderContext, norm: f64) -> f64 {
    let base = &context.base;
    let mut best = norm;
    let mut distance = f64::INFINITY;
    for detent in &context.detents {
        let detent_norm = normalize_value(*detent, base.min, base.max, base.law);
        let candidate = (norm - detent_norm).abs();
        if candidate <= context.detent_snap && candidate < distance {
            best = detent_norm;
            distance = candidate;
        }
    }
    best
}

pub fn fader_transition(
    mut context: FaderContext,
    event: AudioValueEvent,
) -> (FaderContext, Vec<AudioValueEffect>) {
    if let Some(effects) = common_value_transition(&mut context.base, &event) {
        return (context, effects);
    }
    let effects = match event {
        AudioValueEvent::DragBegin { position, fine } => {
            begin_drag(&mut context.base, position, fine)
        }
        // The adapter resolves the axis position through `fader_point_to_norm`.
        AudioValueEvent::DragSetNorm { value_norm, fine } => {
            if !dragging(&context.base) {
                return (context, vec![]);
            }
            if rebase_drag(&mut context.base, value_norm, fine) {
                return (context, vec![]);
            }
            let base = &context.base;
            let start_norm = normalize_value(base.drag_start_value, base.min, base.max, base.law);
            let target = if fine {
                start_norm + (value_norm - base.drag_start_position) * 0.1
            } else {
                value_norm
            };
            let snapped = snap_fader_detent(&context, target);
            let base = &context.base;
            let value = denormalize_value(snapped, base.min, base.max, base.law);
            context.base.value = value;
            vec![AudioValueEffect::ValueChange(value)]
        }
        AudioValueEvent::DragEnd | AudioValueEvent::DragCancel => end_drag(&mut context.base),
        _ => vec![],
    };
    (context, effects)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

pub fn hit_test_rect(point: AudioPoint, rect: AudioRect) -> bool {
    point.x >= rect.left
        && point.x <= rect.left + rect.width
        && point.y >= rect.top
        && point.y <= rect.top + rect.height
}

pub fn hit_test_circle(point: AudioPoint, rect: AudioRect) -> bool {
    let radius = rect.width.min(rect.height) / 2.0;
    let center_x = rect.left + rect.width / 2.0;
    let center_y = rect.top + rect.height / 2.0;
    (point.x - center_x).hypot(point.y - center_y) <= radius
}

/// Standard 270 degree knob sweep: -135 degrees is zero and +135 degrees is one.
pub fn knob_point_to_norm(point: AudioPoint, rect: AudioRect) -> f64 {
    let x = point.x - (rect.left + rect.width / 2.0);
    let y = point.y - (rect.top + rect.height / 2.0);
    let mut degrees = y.atan2(x) * 180.0 / std::f64::consts::PI + 90.0;
    if degrees < -180.0 {
        degrees += 360.0;
    }
    if degrees > 180.0 {
        degrees -= 360.0;
    }
    ((degrees + 135.0) / 270.0).clamp(0.0, 1.0)
}

pub fn fader_point_to_norm(
    point: AudioPoint,
    rect: AudioRect,
    orientation: FaderOrientation,
) -> f64 {
    match orientation {
        FaderOrientation::Horizontal => {
            ((point.x - rect.left) / rect.width.max(1.0)).clamp(0.0, 1.0)
        }
        FaderOrientation::Vertical => {
            1.0 - ((point.y - rect.top) / rect.height.max(1.0)).clamp(0.0, 1.0)
        }
    }
}

/// Normalized pad coordinates: x increases right and y increases upward.
pub fn xy_pad_point_to_norm(point: AudioPoint, rect: AudioRect) -> (f64, f64) {
    (
        ((point.x - rect.left) / rect.width.max(1.0)).clamp(0.0, 1.0),
        1.0 - ((point.y - rect.top) / rect.height.max(1.0)).clamp(0.0, 1.0),
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioValueFormat {
    Number { decimals: usize },
    Db { decimals: usize },
    Hz { decimals: usize },
    Khz { decimals: usize },
    Percent { decimals: usize },
    Ratio { decimals: usize },
    Milliseconds { decimals: usize },
    Note,
    Semitones { decimals: usize },
}
impl Default for AudioValueFormat {
    fn default() -> Self {
        Self::Number { decimals: 2 }
    }
}

fn fixed(value: f64, decimals: usize) -> String {
    let mut text = format!("{value:.digits$}", digits = decimals.min(12));
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        "0".into()
    } else {
        text
    }
}

pub fn format_value(value: f64, format: AudioValueFormat) -> String {
    match format {
        AudioValueFormat::Number { decimals } => fixed(value, decimals),
        AudioValueFormat::Db { decimals } => format!("{} dB", fixed(value, decimals)),
        AudioValueFormat::Hz { decimals } if value.abs() >= 1000.0 => {
            format!("{} kHz", fixed(value / 1000.0, decimals))
        }
        AudioValueFormat::Hz { decimals } => format!("{} Hz", fixed(value, decimals)),
        AudioValueFormat::Khz { decimals } => format!("{} kHz", fixed(value, decimals)),
        AudioValueFormat::Percent { decimals } => format!("{}%", fixed(value * 100.0, decimals)),
        AudioValueFormat::Ratio { decimals } => format!("{}:1", fixed(value, decimals)),
        AudioValueFormat::Milliseconds { decimals } if value.abs() >= 1000.0 => {
            format!("{} s", fixed(value / 1000.0, decimals))
        }
        AudioValueFormat::Milliseconds { decimals } => format!("{} ms", fixed(value, decimals)),
        AudioValueFormat::Note => {
            const NAMES: [&str; 12] = [
                "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B",
            ];
            let midi = value.round() as i32;
            format!(
                "{}{}",
                NAMES[midi.rem_euclid(12) as usize],
                midi.div_euclid(12) - 1
            )
        }
        AudioValueFormat::Semitones { decimals } => format!(
            "{}{} st",
            if value > 0.0 { "+" } else { "" },
            fixed(value, decimals)
        ),
    }
}

fn parse_number(text: &str) -> Option<f64> {
    text.trim()
        .replace(',', ".")
        .split_whitespace()
        .next()?
        .trim_end_matches(|c: char| c == '%' || c == ':')
        .parse()
        .ok()
}

pub fn parse_value(text: &str, format: AudioValueFormat) -> Option<f64> {
    if format == AudioValueFormat::Note {
        let value = text.trim();
        let split = value
            .char_indices()
            .find(|(_, c)| c.is_ascii_digit() || *c == '-')?
            .0;
        let (pitch, octave) = value.split_at(split);
        let mut chars = pitch.chars();
        let natural = match chars.next()?.to_ascii_uppercase() {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => return parse_number(text),
        };
        let accidental = match chars.next() {
            Some('#' | '♯') => 1,
            Some('b' | '♭') => -1,
            None => 0,
            _ => return None,
        };
        if chars.next().is_some() {
            return None;
        }
        return Some(((octave.parse::<i32>().ok()? + 1) * 12 + natural + accidental) as f64);
    }
    let value = parse_number(text)?;
    let lower = text.to_lowercase();
    Some(match format {
        AudioValueFormat::Percent { .. } => value / 100.0,
        AudioValueFormat::Hz { .. } if lower.contains("khz") => value * 1000.0,
        AudioValueFormat::Khz { .. } if lower.contains("hz") && !lower.contains("khz") => {
            value / 1000.0
        }
        AudioValueFormat::Milliseconds { .. }
            if lower.trim_end().ends_with('s') && !lower.contains("ms") =>
        {
            value * 1000.0
        }
        _ => value,
    })
}

pub const VU_INTEGRATION_MS: f64 = 300.0;
pub const PPM_ATTACK_MS: f64 = 10.0;
pub const PPM_RELEASE_MS: f64 = 1500.0;
pub const RMS_WINDOW_MS: f64 = 300.0;
pub const PEAK_HOLD_MS: f64 = 1500.0;
pub const PEAK_DECAY_DB_PER_SECOND: f64 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AudioMeterMode {
    Vu,
    Ppm,
    #[default]
    SamplePeak,
    Rms,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeterFeedFrame {
    pub at_ms: f64,
    pub peak: f64,
    pub mean_square: f64,
    pub duration_ms: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RmsSlice {
    pub mean_square: f64,
    pub duration_ms: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AudioMeterContext {
    pub mode: AudioMeterMode,
    pub min_db: f64,
    pub max_db: f64,
    pub enabled: bool,
    pub last_at_ms: Option<f64>,
    pub input_db: f64,
    pub ballistic_db: f64,
    pub peak_hold_db: Option<f64>,
    pub peak_hold_until_ms: Option<f64>,
    pub clip: bool,
    pub rms_window: Vec<RmsSlice>,
}
impl Default for AudioMeterContext {
    fn default() -> Self {
        Self {
            mode: AudioMeterMode::SamplePeak,
            min_db: -60.0,
            max_db: 0.0,
            enabled: true,
            last_at_ms: None,
            input_db: -60.0,
            ballistic_db: -60.0,
            peak_hold_db: None,
            peak_hold_until_ms: None,
            clip: false,
            rms_window: vec![],
        }
    }
}

pub fn amplitude_to_db(amplitude: f64) -> f64 {
    if amplitude > 0.0 {
        20.0 * amplitude.log10()
    } else {
        f64::NEG_INFINITY
    }
}
pub fn db_to_amplitude(db: f64) -> f64 {
    if db.is_finite() {
        10_f64.powf(db / 20.0)
    } else {
        0.0
    }
}
pub fn normalize_meter_db(db: f64, min: f64, max: f64) -> f64 {
    if max <= min || !db.is_finite() {
        0.0
    } else {
        ((db - min) / (max - min)).clamp(0.0, 1.0)
    }
}

fn smooth_amplitude(current_db: f64, target: f64, elapsed: f64, time: f64) -> f64 {
    let alpha = 1.0 - (-elapsed.max(0.0) / time).exp();
    amplitude_to_db(db_to_amplitude(current_db) + (target - db_to_amplitude(current_db)) * alpha)
}

fn push_rms(window: &[RmsSlice], frame: MeterFeedFrame) -> Vec<RmsSlice> {
    let mut next = window.to_vec();
    next.push(RmsSlice {
        mean_square: frame.mean_square.max(0.0),
        duration_ms: frame.duration_ms.max(0.0),
    });
    let mut excess = next.iter().map(|s| s.duration_ms).sum::<f64>() - RMS_WINDOW_MS;
    while excess > 0.0 && !next.is_empty() {
        if next[0].duration_ms <= excess {
            excess -= next.remove(0).duration_ms;
        } else {
            next[0].duration_ms -= excess;
            excess = 0.0;
        }
    }
    next
}

pub fn push_meter_frame(context: &AudioMeterContext, frame: MeterFeedFrame) -> AudioMeterContext {
    if !context.enabled
        || !frame.at_ms.is_finite()
        || !frame.peak.is_finite()
        || !frame.mean_square.is_finite()
        || !frame.duration_ms.is_finite()
        || frame.peak < 0.0
        || frame.mean_square < 0.0
        || frame.duration_ms <= 0.0
        || context.last_at_ms.is_some_and(|last| frame.at_ms < last)
    {
        return context.clone();
    }
    let elapsed = context
        .last_at_ms
        .map_or(frame.duration_ms, |last| (frame.at_ms - last).max(0.0));
    let peak = frame.peak.max(0.0);
    let input_db = amplitude_to_db(peak).max(context.min_db);
    let rms_window = push_rms(&context.rms_window, frame);
    let ballistic = match context.mode {
        AudioMeterMode::Vu => smooth_amplitude(
            context.ballistic_db,
            frame.mean_square.max(0.0).sqrt(),
            elapsed,
            VU_INTEGRATION_MS,
        ),
        AudioMeterMode::Ppm => smooth_amplitude(
            context.ballistic_db,
            peak,
            elapsed,
            if input_db >= context.ballistic_db {
                PPM_ATTACK_MS
            } else {
                PPM_RELEASE_MS
            },
        ),
        AudioMeterMode::SamplePeak if input_db >= context.ballistic_db => input_db,
        AudioMeterMode::SamplePeak => (context.ballistic_db
            - elapsed / 1000.0 * PEAK_DECAY_DB_PER_SECOND)
            .max(input_db)
            .max(context.min_db),
        AudioMeterMode::Rms => {
            let duration = rms_window.iter().map(|s| s.duration_ms).sum::<f64>();
            if duration <= 0.0 {
                context.min_db
            } else {
                amplitude_to_db(
                    (rms_window
                        .iter()
                        .map(|s| s.mean_square * s.duration_ms)
                        .sum::<f64>()
                        / duration)
                        .sqrt(),
                )
                .max(context.min_db)
            }
        }
    };
    let (peak_hold_db, peak_hold_until_ms) = if context.peak_hold_db.is_none()
        || input_db >= context.peak_hold_db.unwrap_or(context.min_db)
    {
        (Some(input_db), Some(frame.at_ms + PEAK_HOLD_MS))
    } else if frame.at_ms <= context.peak_hold_until_ms.unwrap_or(frame.at_ms) {
        (context.peak_hold_db, context.peak_hold_until_ms)
    } else {
        let hold = context.peak_hold_until_ms.unwrap_or(frame.at_ms);
        let start = context.last_at_ms.unwrap_or(hold).max(hold);
        (
            Some(
                (context.peak_hold_db.unwrap()
                    - (frame.at_ms - start) / 1000.0 * PEAK_DECAY_DB_PER_SECOND)
                    .max(context.min_db),
            ),
            Some(hold),
        )
    };
    AudioMeterContext {
        last_at_ms: Some(frame.at_ms),
        input_db,
        ballistic_db: ballistic.clamp(context.min_db, context.max_db),
        peak_hold_db,
        peak_hold_until_ms,
        clip: context.clip || peak >= 1.0,
        rms_window,
        ..context.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioMeterVisualState {
    pub control: AudioControlVisualState,
    pub ballistic_value: f64,
    pub peak_hold: Option<f64>,
    pub clip: bool,
}
impl AudioMeterContext {
    pub fn visual_state(&self) -> AudioMeterVisualState {
        AudioMeterVisualState {
            control: AudioControlVisualState {
                value_norm: normalize_meter_db(self.input_db, self.min_db, self.max_db),
                raw_value: self.input_db,
                bipolar_center: None,
                hover: false,
                focus: false,
                drag: DragState::None,
                automation: AutomationState::None,
                enabled: self.enabled,
            },
            ballistic_value: normalize_meter_db(self.ballistic_db, self.min_db, self.max_db),
            peak_hold: self
                .peak_hold_db
                .map(|db| normalize_meter_db(db, self.min_db, self.max_db)),
            clip: self.clip,
        }
    }
}

pub const GAIN_REDUCTION_ATTACK_MS: f64 = 10.0;
pub const GAIN_REDUCTION_RELEASE_MS: f64 = 300.0;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GainReductionFrame {
    pub at_ms: f64,
    pub reduction_db: f64,
    pub duration_ms: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GainReductionContext {
    pub max_reduction_db: f64,
    pub enabled: bool,
    pub last_at_ms: Option<f64>,
    pub reduction_db: f64,
    pub ballistic_db: f64,
}
impl Default for GainReductionContext {
    fn default() -> Self {
        Self {
            max_reduction_db: 30.0,
            enabled: true,
            last_at_ms: None,
            reduction_db: 0.0,
            ballistic_db: 0.0,
        }
    }
}
pub fn push_gain_reduction_frame(
    context: &GainReductionContext,
    frame: GainReductionFrame,
) -> GainReductionContext {
    if !context.enabled
        || !frame.at_ms.is_finite()
        || !frame.reduction_db.is_finite()
        || !frame.duration_ms.is_finite()
        || frame.reduction_db < 0.0
        || frame.duration_ms <= 0.0
        || context.last_at_ms.is_some_and(|last| frame.at_ms < last)
    {
        return context.clone();
    }
    let elapsed = context
        .last_at_ms
        .map_or(frame.duration_ms, |last| frame.at_ms - last);
    let reduction = frame.reduction_db.clamp(0.0, context.max_reduction_db);
    let time = if reduction >= context.ballistic_db {
        GAIN_REDUCTION_ATTACK_MS
    } else {
        GAIN_REDUCTION_RELEASE_MS
    };
    let alpha = 1.0 - (-elapsed.max(0.0) / time).exp();
    GainReductionContext {
        last_at_ms: Some(frame.at_ms),
        reduction_db: reduction,
        ballistic_db: (context.ballistic_db + (reduction - context.ballistic_db) * alpha)
            .clamp(0.0, context.max_reduction_db),
        ..context.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GainReductionVisualState {
    pub meter: AudioMeterVisualState,
    pub reduction_db: f64,
}
impl GainReductionContext {
    pub fn visual_state(&self) -> GainReductionVisualState {
        let max = self.max_reduction_db.max(f64::EPSILON);
        GainReductionVisualState {
            meter: AudioMeterVisualState {
                control: AudioControlVisualState {
                    value_norm: (self.reduction_db / max).clamp(0.0, 1.0),
                    raw_value: self.reduction_db,
                    bipolar_center: None,
                    hover: false,
                    focus: false,
                    drag: DragState::None,
                    automation: AutomationState::None,
                    enabled: self.enabled,
                },
                ballistic_value: (self.ballistic_db / max).clamp(0.0, 1.0),
                peak_hold: None,
                clip: false,
            },
            reduction_db: self.reduction_db,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AudioSwitchMode {
    #[default]
    Latch,
    Momentary,
    Multi,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AudioSwitchVisualState {
    pub state: usize,
    pub state_count: usize,
    pub pressed: bool,
    pub lamp_on: bool,
    pub hover: bool,
    pub focus: bool,
    pub enabled: bool,
}
pub fn switch_visual_state(
    mode: AudioSwitchMode,
    state: usize,
    count: usize,
    pressed: bool,
    lamp: Option<bool>,
    enabled: bool,
) -> AudioSwitchVisualState {
    let count = count.max(2);
    let state = state.min(count - 1);
    let _ = mode;
    AudioSwitchVisualState {
        state,
        state_count: count,
        pressed,
        lamp_on: lamp.unwrap_or(state > 0),
        hover: false,
        focus: false,
        enabled,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioSwitchContext {
    pub mode: AudioSwitchMode,
    pub state: usize,
    pub state_count: usize,
    pub lamp_on: Option<bool>,
    pub pressed: bool,
    pub disabled: bool,
}
impl Default for AudioSwitchContext {
    fn default() -> Self {
        Self {
            mode: AudioSwitchMode::Latch,
            state: 0,
            state_count: 2,
            lamp_on: None,
            pressed: false,
            disabled: false,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSwitchEvent {
    Press,
    Release,
    Cancel,
    Previous,
    Next,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSwitchEffect {
    StateChange(usize),
    StateCommit(usize),
}

pub fn audio_switch_transition(
    mut context: AudioSwitchContext,
    event: AudioSwitchEvent,
) -> (AudioSwitchContext, Vec<AudioSwitchEffect>) {
    context.state_count = context.state_count.max(2);
    context.state = context.state.min(context.state_count - 1);
    if context.disabled {
        return (context, vec![]);
    }
    match (context.mode, event) {
        (_, AudioSwitchEvent::Press) if context.pressed => (context, vec![]),
        (AudioSwitchMode::Momentary, AudioSwitchEvent::Press) => {
            context.pressed = true;
            context.state = 1;
            (context, vec![AudioSwitchEffect::StateChange(1)])
        }
        (_, AudioSwitchEvent::Press) => {
            context.pressed = true;
            (context, vec![])
        }
        (AudioSwitchMode::Momentary, AudioSwitchEvent::Release | AudioSwitchEvent::Cancel)
            if context.pressed =>
        {
            context.pressed = false;
            context.state = 0;
            (
                context,
                vec![
                    AudioSwitchEffect::StateChange(0),
                    AudioSwitchEffect::StateCommit(0),
                ],
            )
        }
        (AudioSwitchMode::Latch, AudioSwitchEvent::Release) if context.pressed => {
            context.pressed = false;
            context.state = usize::from(context.state == 0);
            let state = context.state;
            (
                context,
                vec![
                    AudioSwitchEffect::StateChange(state),
                    AudioSwitchEffect::StateCommit(state),
                ],
            )
        }
        (AudioSwitchMode::Multi, AudioSwitchEvent::Release) if context.pressed => {
            context.pressed = false;
            context.state = (context.state + 1) % context.state_count;
            let state = context.state;
            (
                context,
                vec![
                    AudioSwitchEffect::StateChange(state),
                    AudioSwitchEffect::StateCommit(state),
                ],
            )
        }
        (_, AudioSwitchEvent::Cancel) => {
            context.pressed = false;
            (context, vec![])
        }
        (_, AudioSwitchEvent::Previous | AudioSwitchEvent::Next) => {
            let direction = if event == AudioSwitchEvent::Next {
                1
            } else {
                context.state_count - 1
            };
            context.state = (context.state + direction) % context.state_count;
            let state = context.state;
            (
                context,
                vec![
                    AudioSwitchEffect::StateChange(state),
                    AudioSwitchEffect::StateCommit(state),
                ],
            )
        }
        _ => (context, vec![]),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvelopePoint {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub curve: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnvelopeVisualPoint {
    pub id: String,
    pub x_norm: f64,
    pub y_norm: f64,
    pub curve: f64,
    pub selected: bool,
    pub dragging: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnvelopeVisualState {
    pub points: Vec<EnvelopeVisualPoint>,
    pub hover_point_id: Option<String>,
    pub focus: bool,
    pub enabled: bool,
}
pub fn envelope_segment_value_at(from: &EnvelopePoint, to: &EnvelopePoint, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let shaped = if from.curve == 0.0 {
        t
    } else if from.curve > 0.0 {
        t.powf(1.0 + from.curve * 4.0)
    } else {
        1.0 - (1.0 - t).powf(1.0 + from.curve.abs() * 4.0)
    };
    from.y + (to.y - from.y) * shaped
}

pub fn normalize_envelope_points(points: &[EnvelopePoint]) -> Vec<EnvelopePoint> {
    let mut next = points
        .iter()
        .filter(|point| point.x.is_finite() && point.y.is_finite() && point.curve.is_finite())
        .cloned()
        .collect::<Vec<_>>();
    for point in &mut next {
        point.x = point.x.clamp(0.0, 1.0);
        point.y = point.y.clamp(0.0, 1.0);
        point.curve = point.curve.clamp(-1.0, 1.0);
    }
    next.sort_by(|a, b| a.x.total_cmp(&b.x));
    let mut seen = std::collections::HashSet::new();
    next.retain(|point| seen.insert(point.id.clone()));
    next
}

pub fn move_envelope_point(
    points: &[EnvelopePoint],
    id: &str,
    x: f64,
    y: f64,
) -> Vec<EnvelopePoint> {
    normalize_envelope_points(
        &points
            .iter()
            .cloned()
            .map(|mut point| {
                if point.id == id {
                    point.x = x;
                    point.y = y;
                }
                point
            })
            .collect::<Vec<_>>(),
    )
}

pub fn remove_envelope_point(points: &[EnvelopePoint], id: &str) -> Vec<EnvelopePoint> {
    normalize_envelope_points(
        &points
            .iter()
            .filter(|point| point.id != id)
            .cloned()
            .collect::<Vec<_>>(),
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct XYPadVisualState {
    pub x_norm: f64,
    pub y_norm: f64,
    pub raw_x: f64,
    pub raw_y: f64,
    pub hover: bool,
    pub focus: bool,
    pub drag: DragState,
    pub automation: AutomationState,
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XYPadAxis {
    X,
    Y,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XYPadContext {
    pub x: f64,
    pub y: f64,
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
    pub hover: bool,
    pub focus: bool,
    pub drag: DragState,
    pub automation: AutomationState,
    /// Pair anchored when the gesture began or last rebased.
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    /// Normalized pointer position anchored with the start pair.
    pub drag_start_norm_x: f64,
    pub drag_start_norm_y: f64,
    pub disabled: bool,
}
impl Default for XYPadContext {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
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
            hover: false,
            focus: false,
            drag: DragState::None,
            automation: AutomationState::None,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            drag_start_norm_x: 0.0,
            drag_start_norm_y: 0.0,
            disabled: false,
        }
    }
}
impl XYPadContext {
    pub fn visual_state(&self) -> XYPadVisualState {
        XYPadVisualState {
            x_norm: normalize_value(self.x, self.min_x, self.max_x, self.law_x),
            y_norm: normalize_value(self.y, self.min_y, self.max_y, self.law_y),
            raw_x: self.x,
            raw_y: self.y,
            hover: self.hover,
            focus: self.focus,
            drag: self.drag,
            automation: self.automation,
            enabled: !self.disabled,
        }
    }

    fn constrained(&self, x: f64, y: f64) -> (f64, f64) {
        (
            constrain_value(x, self.min_x, self.max_x, self.law_x),
            constrain_value(y, self.min_y, self.max_y, self.law_y),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XYPadEvent {
    /// Host-owned pair replacement. Not a user gesture, so it emits nothing.
    SetValues {
        x: f64,
        y: f64,
    },
    Hover {
        value: bool,
    },
    Focus {
        value: bool,
    },
    SetAutomation {
        value: AutomationState,
    },
    DragBegin {
        x_norm: f64,
        y_norm: f64,
        fine: bool,
    },
    DragMove {
        x_norm: f64,
        y_norm: f64,
        fine: bool,
    },
    DragEnd,
    DragCancel,
    Reset,
    Nudge {
        axis: XYPadAxis,
        direction: i8,
        multiplier: f64,
        fine: bool,
    },
    Bound {
        axis: XYPadAxis,
        bound: ValueBound,
    },
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XYPadEffect {
    ValueChange(f64, f64),
    ValueCommit(f64, f64),
    GestureBegin,
    GestureEnd,
}

/// Keys, bounds, and reset move the pair atomically: one change plus one
/// commit carrying both axes. Axis values are never emitted separately.
fn xy_atomic(context: &mut XYPadContext, x: f64, y: f64) -> Vec<XYPadEffect> {
    let (x, y) = context.constrained(x, y);
    context.x = x;
    context.y = y;
    vec![
        XYPadEffect::ValueChange(x, y),
        XYPadEffect::ValueCommit(x, y),
    ]
}

pub fn xy_pad_transition(
    mut context: XYPadContext,
    event: XYPadEvent,
) -> (XYPadContext, Vec<XYPadEffect>) {
    let effects = match event {
        XYPadEvent::SetValues { x, y } => {
            let (x, y) = context.constrained(x, y);
            context.x = x;
            context.y = y;
            vec![]
        }
        XYPadEvent::Hover { value } => {
            context.hover = value;
            vec![]
        }
        XYPadEvent::Focus { value } => {
            context.focus = value;
            vec![]
        }
        XYPadEvent::SetAutomation { value } => {
            context.automation = value;
            vec![]
        }
        XYPadEvent::Reset => {
            if context.disabled {
                vec![]
            } else {
                let (x, y) = (context.default_x, context.default_y);
                xy_atomic(&mut context, x, y)
            }
        }
        // One accepted gesture at a time: a second begin cannot reopen or
        // re-anchor an open one, so begin/end stay paired exactly once.
        XYPadEvent::DragBegin {
            x_norm,
            y_norm,
            fine,
        } => {
            if context.disabled || context.drag != DragState::None {
                vec![]
            } else {
                // A coarse press moves the pair to the accepted press
                // position; a fine press only anchors.
                let (x, y) = if fine {
                    (context.x, context.y)
                } else {
                    context.constrained(
                        denormalize_value(x_norm, context.min_x, context.max_x, context.law_x),
                        denormalize_value(y_norm, context.min_y, context.max_y, context.law_y),
                    )
                };
                context.x = x;
                context.y = y;
                context.drag = if fine {
                    DragState::Fine
                } else {
                    DragState::Coarse
                };
                context.drag_start_x = x;
                context.drag_start_y = y;
                context.drag_start_norm_x = x_norm;
                context.drag_start_norm_y = y_norm;
                if fine {
                    vec![XYPadEffect::GestureBegin]
                } else {
                    vec![XYPadEffect::GestureBegin, XYPadEffect::ValueChange(x, y)]
                }
            }
        }
        XYPadEvent::DragMove {
            x_norm,
            y_norm,
            fine,
        } => {
            let next = if fine {
                DragState::Fine
            } else {
                DragState::Coarse
            };
            if context.disabled || context.drag == DragState::None {
                vec![]
            } else if next != context.drag {
                // Modifier flip rebases both axes at the current pair and
                // pointer; travel resumes from the next move.
                context.drag = next;
                context.drag_start_x = context.x;
                context.drag_start_y = context.y;
                context.drag_start_norm_x = x_norm;
                context.drag_start_norm_y = y_norm;
                vec![]
            } else {
                let start_x_norm = normalize_value(
                    context.drag_start_x,
                    context.min_x,
                    context.max_x,
                    context.law_x,
                );
                let start_y_norm = normalize_value(
                    context.drag_start_y,
                    context.min_y,
                    context.max_y,
                    context.law_y,
                );
                let target_x = if fine {
                    start_x_norm + (x_norm - context.drag_start_norm_x) * 0.1
                } else {
                    x_norm
                };
                let target_y = if fine {
                    start_y_norm + (y_norm - context.drag_start_norm_y) * 0.1
                } else {
                    y_norm
                };
                let (x, y) = context.constrained(
                    denormalize_value(target_x, context.min_x, context.max_x, context.law_x),
                    denormalize_value(target_y, context.min_y, context.max_y, context.law_y),
                );
                context.x = x;
                context.y = y;
                vec![XYPadEffect::ValueChange(x, y)]
            }
        }
        // Release and cancellation close the gesture the same way and are inert
        // once it is closed, so repeated, stale, lost-capture, and teardown
        // terminals cannot duplicate the pair.
        XYPadEvent::DragEnd | XYPadEvent::DragCancel => {
            if context.drag == DragState::None {
                vec![]
            } else {
                context.drag = DragState::None;
                vec![
                    XYPadEffect::ValueCommit(context.x, context.y),
                    XYPadEffect::GestureEnd,
                ]
            }
        }
        XYPadEvent::Nudge {
            axis,
            direction,
            multiplier,
            fine,
        } => {
            if context.disabled {
                vec![]
            } else {
                let scale = if fine { 0.1 } else { 1.0 };
                let (x, y) = match axis {
                    XYPadAxis::X => (
                        context.x + direction as f64 * context.keyboard_step_x * multiplier * scale,
                        context.y,
                    ),
                    XYPadAxis::Y => (
                        context.x,
                        context.y + direction as f64 * context.keyboard_step_y * multiplier * scale,
                    ),
                };
                xy_atomic(&mut context, x, y)
            }
        }
        XYPadEvent::Bound { axis, bound } => {
            if context.disabled {
                vec![]
            } else {
                let (x, y) = match (axis, bound) {
                    (XYPadAxis::X, ValueBound::Min) => (context.min_x, context.y),
                    (XYPadAxis::X, ValueBound::Max) => (context.max_x, context.y),
                    (XYPadAxis::Y, ValueBound::Min) => (context.x, context.min_y),
                    (XYPadAxis::Y, ValueBound::Max) => (context.x, context.max_y),
                };
                xy_atomic(&mut context, x, y)
            }
        }
    };
    (context, effects)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardKeyVisualState {
    pub note: u8,
    pub black: bool,
    pub start_norm: f64,
    pub length_norm: f64,
    pub breadth_norm: f64,
    pub held: bool,
    pub externally_held: bool,
    pub velocity: Option<u8>,
    pub focused: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardVisualState {
    pub orientation: KeyboardOrientation,
    pub first_note: u8,
    pub last_note: u8,
    pub octave_shift: i8,
    pub keys: Vec<KeyboardKeyVisualState>,
    pub held_notes: Vec<u8>,
    pub external_held_notes: Vec<u8>,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardContext {
    pub first_note: u8,
    pub last_note: u8,
    pub orientation: KeyboardOrientation,
    pub octave_shift: i8,
    pub active_inputs: Vec<(String, u8, u8)>,
    pub external_held_notes: Vec<u8>,
    pub focused_note: Option<u8>,
    pub disabled: bool,
}

impl Default for KeyboardContext {
    fn default() -> Self {
        Self {
            first_note: 48,
            last_note: 72,
            orientation: KeyboardOrientation::Horizontal,
            octave_shift: 0,
            active_inputs: vec![],
            external_held_notes: vec![],
            focused_note: None,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardEffect {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
}

pub fn keyboard_press(
    mut context: KeyboardContext,
    input: impl Into<String>,
    note: u8,
    velocity: u8,
) -> (KeyboardContext, Vec<KeyboardEffect>) {
    let input = input.into();
    if context.disabled
        || note < context.first_note
        || note > context.last_note
        || context.active_inputs.iter().any(|active| active.0 == input)
    {
        return (context, vec![]);
    }
    let held = context.active_inputs.iter().any(|active| active.1 == note);
    let velocity = velocity.clamp(1, 127);
    context.active_inputs.push((input, note, velocity));
    context.focused_note = Some(note);
    let effects = if held {
        vec![]
    } else {
        vec![KeyboardEffect::NoteOn { note, velocity }]
    };
    (context, effects)
}

pub fn keyboard_release(
    mut context: KeyboardContext,
    input: &str,
) -> (KeyboardContext, Vec<KeyboardEffect>) {
    let Some(index) = context
        .active_inputs
        .iter()
        .position(|active| active.0 == input)
    else {
        return (context, vec![]);
    };
    let note = context.active_inputs.remove(index).1;
    let effects = if context.active_inputs.iter().any(|active| active.1 == note) {
        vec![]
    } else {
        vec![KeyboardEffect::NoteOff { note }]
    };
    (context, effects)
}

pub fn keyboard_retarget(
    mut context: KeyboardContext,
    input: &str,
    note: Option<u8>,
    velocity: u8,
) -> (KeyboardContext, Vec<KeyboardEffect>) {
    if context.disabled {
        return keyboard_release(context, input);
    }
    if let (Some(active), Some(note)) = (
        context
            .active_inputs
            .iter_mut()
            .find(|active| active.0 == input),
        note,
    ) {
        if active.1 == note {
            active.2 = velocity.clamp(1, 127);
            context.focused_note = Some(note);
            return (context, vec![]);
        }
    }
    let (released, mut effects) = keyboard_release(context, input);
    let Some(note) = note else {
        return (released, effects);
    };
    let (pressed, press_effects) = keyboard_press(released, input, note, velocity);
    effects.extend(press_effects);
    (pressed, effects)
}

fn black_note(note: u8) -> bool {
    matches!(note % 12, 1 | 3 | 6 | 8 | 10)
}

pub fn keyboard_velocity(depth_norm: f64) -> u8 {
    (1.0 + depth_norm.clamp(0.0, 1.0) * 126.0).round() as u8
}

pub fn keyboard_visual_state(context: &KeyboardContext) -> KeyboardVisualState {
    let notes: Vec<u8> = (context.first_note..=context.last_note).collect();
    let white: Vec<u8> = notes
        .iter()
        .copied()
        .filter(|note| !black_note(*note))
        .collect();
    let white_length = 1.0 / white.len().max(1) as f64;
    let keys = notes
        .iter()
        .map(|note| {
            let black = black_note(*note);
            let preceding = white.iter().filter(|candidate| **candidate < *note).count();
            let logical_start = if black {
                ((preceding as f64 - 0.32) * white_length).max(0.0)
            } else {
                white
                    .iter()
                    .position(|candidate| candidate == note)
                    .unwrap_or(0) as f64
                    * white_length
            };
            let length_norm = if black {
                white_length * 0.64
            } else {
                white_length
            };
            let velocity = context
                .active_inputs
                .iter()
                .filter(|active| active.1 == *note)
                .map(|active| active.2)
                .max();
            KeyboardKeyVisualState {
                note: *note,
                black,
                start_norm: if context.orientation == KeyboardOrientation::Vertical {
                    1.0 - logical_start - length_norm
                } else {
                    logical_start
                },
                length_norm,
                breadth_norm: if black { 0.62 } else { 1.0 },
                held: velocity.is_some(),
                externally_held: context.external_held_notes.contains(note),
                velocity,
                focused: context.focused_note == Some(*note),
            }
        })
        .collect();
    let mut held_notes: Vec<u8> = context
        .active_inputs
        .iter()
        .map(|active| active.1)
        .collect();
    held_notes.sort();
    held_notes.dedup();
    let mut external = context.external_held_notes.clone();
    external.sort();
    external.dedup();
    KeyboardVisualState {
        orientation: context.orientation,
        first_note: context.first_note,
        last_note: context.last_note,
        octave_shift: context.octave_shift,
        keys,
        held_notes,
        external_held_notes: external,
        enabled: !context.disabled,
    }
}

pub const WAVEFORM_MAX_COLUMNS: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveformPeakPair {
    pub min: f64,
    pub max: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WaveformPeakLevel {
    pub samples_per_peak: usize,
    pub peaks: Vec<WaveformPeakPair>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WaveformPeakPyramid {
    pub sample_count: usize,
    pub levels: Vec<WaveformPeakLevel>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveformSelection {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WaveformVisualState {
    pub sample_count: usize,
    pub visible_start: usize,
    pub visible_end: usize,
    pub columns: Vec<WaveformPeakPair>,
    pub cursor_sample: Option<usize>,
    pub selection: Option<WaveformSelection>,
    pub focus: bool,
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WaveformContext {
    pub pyramid: WaveformPeakPyramid,
    pub visible_start: usize,
    pub visible_end: usize,
    pub column_count: usize,
    pub cursor_sample: Option<usize>,
    pub selection: Option<WaveformSelection>,
    pub selection_anchor: Option<usize>,
    pub selecting: bool,
    pub focus: bool,
    pub disabled: bool,
}

pub fn validate_peak_pyramid(pyramid: &WaveformPeakPyramid) -> bool {
    let mut previous = 0;
    pyramid.levels.iter().all(|level| {
        let valid = level.samples_per_peak > previous
            && level
                .peaks
                .iter()
                .all(|peak| peak.min.is_finite() && peak.max.is_finite() && peak.min <= peak.max);
        previous = level.samples_per_peak;
        valid
    })
}

pub fn waveform_columns(context: &WaveformContext) -> Vec<WaveformPeakPair> {
    if !validate_peak_pyramid(&context.pyramid) || context.visible_end <= context.visible_start {
        return vec![];
    }
    let span = context.visible_end - context.visible_start;
    let target = context.column_count.clamp(1, WAVEFORM_MAX_COLUMNS);
    let Some(level) = context
        .pyramid
        .levels
        .iter()
        .find(|level| span.div_ceil(level.samples_per_peak) <= target)
        .or_else(|| context.pyramid.levels.last())
    else {
        return vec![];
    };
    let first = context.visible_start / level.samples_per_peak;
    let last = context
        .visible_end
        .div_ceil(level.samples_per_peak)
        .min(level.peaks.len());
    let source = &level.peaks[first.min(last)..last];
    let count = target.min(source.len());
    (0..count)
        .map(|column| {
            let start = column * source.len() / count;
            let end = ((column + 1) * source.len()).div_ceil(count).max(start + 1);
            WaveformPeakPair {
                min: source[start..end]
                    .iter()
                    .map(|peak| peak.min)
                    .fold(f64::INFINITY, f64::min)
                    .clamp(-1.0, 1.0),
                max: source[start..end]
                    .iter()
                    .map(|peak| peak.max)
                    .fold(f64::NEG_INFINITY, f64::max)
                    .clamp(-1.0, 1.0),
            }
        })
        .collect()
}

impl WaveformContext {
    pub fn visual_state(&self) -> WaveformVisualState {
        WaveformVisualState {
            sample_count: self.pyramid.sample_count,
            visible_start: self.visible_start,
            visible_end: self.visible_end,
            columns: waveform_columns(self),
            cursor_sample: self.cursor_sample,
            selection: self.selection,
            focus: self.focus,
            enabled: !self.disabled,
        }
    }
    pub fn select_begin(mut self, sample: usize) -> Self {
        if !self.disabled {
            let sample = sample.clamp(
                self.visible_start,
                self.visible_end.saturating_sub(1).max(self.visible_start),
            );
            self.cursor_sample = Some(sample);
            self.selection = Some(WaveformSelection {
                start: sample,
                end: sample,
            });
            self.selection_anchor = Some(sample);
            self.selecting = true;
        }
        self
    }
    pub fn select_move(mut self, sample: usize) -> Self {
        if self.selecting {
            let sample = sample.clamp(
                self.visible_start,
                self.visible_end.saturating_sub(1).max(self.visible_start),
            );
            let anchor = self.selection_anchor.unwrap_or(sample);
            self.cursor_sample = Some(sample);
            self.selection = Some(WaveformSelection {
                start: anchor.min(sample),
                end: anchor.max(sample),
            });
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModMatrixHeader {
    pub id: String,
    pub label: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixCellParameters {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub law: AudioValueLaw,
}
impl Default for ModMatrixCellParameters {
    fn default() -> Self {
        Self {
            min: -1.0,
            max: 1.0,
            step: 0.01,
            law: AudioValueLaw::BipolarCenter { center: 0.0 },
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixCell {
    pub source_id: String,
    pub destination_id: String,
    pub amount: f64,
    pub enabled: bool,
    pub parameters: ModMatrixCellParameters,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixVisualCell {
    pub cell: ModMatrixCell,
    pub amount_norm: f64,
    pub zero_norm: f64,
    pub fill_start_norm: f64,
    pub fill_span_norm: f64,
    pub focused: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixVisualState {
    pub sources: Vec<ModMatrixHeader>,
    pub destinations: Vec<ModMatrixHeader>,
    pub cells: Vec<ModMatrixVisualCell>,
    pub focus: Option<(String, String)>,
    pub enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModMatrixContext {
    pub sources: Vec<ModMatrixHeader>,
    pub destinations: Vec<ModMatrixHeader>,
    pub cells: Vec<ModMatrixCell>,
    pub focus_row: Option<usize>,
    pub focus_column: Option<usize>,
    pub step: f64,
    pub disabled: bool,
}

impl ModMatrixContext {
    pub fn new(
        sources: Vec<ModMatrixHeader>,
        destinations: Vec<ModMatrixHeader>,
        supplied: Vec<ModMatrixCell>,
    ) -> Self {
        assert!(
            sources.iter().all(|header| !header.id.is_empty())
                && sources
                    .iter()
                    .enumerate()
                    .all(|(index, header)| sources[..index]
                        .iter()
                        .all(|prior| prior.id != header.id)),
            "mod matrix source ids must be non-empty and unique"
        );
        assert!(
            destinations.iter().all(|header| !header.id.is_empty())
                && destinations
                    .iter()
                    .enumerate()
                    .all(|(index, header)| destinations[..index]
                        .iter()
                        .all(|prior| prior.id != header.id)),
            "mod matrix destination ids must be non-empty and unique"
        );
        let cells = sources
            .iter()
            .flat_map(|source| {
                destinations.iter().map(|destination| {
                    supplied
                        .iter()
                        .find(|cell| {
                            cell.source_id == source.id && cell.destination_id == destination.id
                        })
                        .cloned()
                        .unwrap_or(ModMatrixCell {
                            source_id: source.id.clone(),
                            destination_id: destination.id.clone(),
                            amount: 0.0,
                            enabled: false,
                            parameters: ModMatrixCellParameters::default(),
                        })
                })
            })
            .map(|mut cell| {
                assert!(
                    cell.parameters.min.is_finite()
                        && cell.parameters.max.is_finite()
                        && cell.parameters.max > cell.parameters.min,
                    "mod matrix cell parameters require finite min < max"
                );
                assert!(
                    cell.parameters.step.is_finite() && cell.parameters.step >= 0.0,
                    "mod matrix cell step must be finite and non-negative"
                );
                assert!(
                    valid_law(
                        continuous_law(cell.parameters.law),
                        cell.parameters.min,
                        cell.parameters.max,
                    ),
                    "invalid mod matrix cell law"
                );
                let fallback = clamp_value(0.0, cell.parameters.min, cell.parameters.max);
                cell.amount = constrain_value(
                    if cell.amount.is_finite() {
                        cell.amount
                    } else {
                        fallback
                    },
                    cell.parameters.min,
                    cell.parameters.max,
                    cell.parameters.law,
                );
                cell
            })
            .collect();
        Self {
            sources,
            destinations,
            cells,
            focus_row: None,
            focus_column: None,
            step: 0.01,
            disabled: false,
        }
    }
    pub fn move_focus(mut self, rows: isize, columns: isize) -> Self {
        if !self.disabled && !self.sources.is_empty() && !self.destinations.is_empty() {
            self.focus_row = Some(
                ((self.focus_row.unwrap_or(0) as isize + rows)
                    .clamp(0, self.sources.len() as isize - 1)) as usize,
            );
            self.focus_column = Some(
                ((self.focus_column.unwrap_or(0) as isize + columns)
                    .clamp(0, self.destinations.len() as isize - 1)) as usize,
            );
        }
        self
    }
    pub fn toggle(mut self) -> Self {
        if let (Some(row), Some(column)) = (self.focus_row, self.focus_column) {
            if !self.disabled {
                let index = row * self.destinations.len() + column;
                self.cells[index].enabled = !self.cells[index].enabled;
            }
        }
        self
    }
    pub fn nudge(mut self, direction: f64, fine: bool) -> Self {
        if let (Some(row), Some(column)) = (self.focus_row, self.focus_column) {
            if !self.disabled {
                let index = row * self.destinations.len() + column;
                let parameters = self.cells[index].parameters.clone();
                self.cells[index].amount = constrain_value(
                    self.cells[index].amount
                        + direction * parameters.step * if fine { 0.1 } else { 1.0 },
                    parameters.min,
                    parameters.max,
                    parameters.law,
                );
            }
        }
        self
    }
    pub fn set_normalized(mut self, norm: f64) -> Self {
        if let (Some(row), Some(column)) = (self.focus_row, self.focus_column) {
            if !self.disabled {
                let index = row * self.destinations.len() + column;
                let parameters = self.cells[index].parameters.clone();
                let (next, _) = crate::slider::slider_control_transition(
                    crate::slider::SliderControlContext {
                        value: self.cells[index].amount,
                        min: parameters.min,
                        max: parameters.max,
                        step: parameters.step,
                        disabled: false,
                        law: parameters.law,
                        polarity: if parameters.min < 0.0 && parameters.max > 0.0 {
                            crate::slider::SliderPolarity::Bipolar
                        } else {
                            crate::slider::SliderPolarity::Unipolar
                        },
                        center_value: None,
                        pointer_active: false,
                    },
                    crate::slider::SliderControlEvent::PointerBegin { value_norm: norm },
                );
                self.cells[index].amount = next.value;
            }
        }
        self
    }
    pub fn visual_state(&self) -> ModMatrixVisualState {
        let focused = self
            .focus_row
            .zip(self.focus_column)
            .map(|(row, column)| row * self.destinations.len() + column);
        ModMatrixVisualState {
            sources: self.sources.clone(),
            destinations: self.destinations.clone(),
            cells: self
                .cells
                .iter()
                .enumerate()
                .map(|(index, cell)| {
                    let parameters = &cell.parameters;
                    let slider =
                        crate::slider::slider_visual_state(crate::slider::SliderControlContext {
                            value: cell.amount,
                            min: parameters.min,
                            max: parameters.max,
                            step: parameters.step,
                            disabled: self.disabled,
                            law: parameters.law,
                            polarity: if parameters.min < 0.0 && parameters.max > 0.0 {
                                crate::slider::SliderPolarity::Bipolar
                            } else {
                                crate::slider::SliderPolarity::Unipolar
                            },
                            center_value: None,
                            pointer_active: false,
                        });
                    ModMatrixVisualCell {
                        cell: cell.clone(),
                        amount_norm: slider.value_norm,
                        zero_norm: slider.center_norm,
                        fill_start_norm: slider.fill_start_norm,
                        fill_span_norm: slider.fill_span_norm,
                        focused: focused == Some(index),
                    }
                })
                .collect(),
            focus: focused.map(|index| {
                (
                    self.cells[index].source_id.clone(),
                    self.cells[index].destination_id.clone(),
                )
            }),
            enabled: !self.disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_three_keyboard_pairs_notes_and_matches_velocity() {
        assert_eq!(keyboard_velocity(0.0), 1);
        assert_eq!(keyboard_velocity(1.0), 127);
        let (context, effects) = keyboard_press(KeyboardContext::default(), "pointer", 60, 64);
        assert_eq!(
            effects,
            vec![KeyboardEffect::NoteOn {
                note: 60,
                velocity: 64
            }]
        );
        assert_eq!(keyboard_visual_state(&context).held_notes, vec![60]);
        let (_, effects) = keyboard_release(context, "pointer");
        assert_eq!(effects, vec![KeyboardEffect::NoteOff { note: 60 }]);
    }

    #[test]
    fn phase_three_keyboard_retargets_captured_pointer_notes() {
        let (context, _) = keyboard_press(KeyboardContext::default(), "pointer", 60, 64);
        let (context, effects) = keyboard_retarget(context, "pointer", Some(62), 96);
        assert_eq!(
            effects,
            vec![
                KeyboardEffect::NoteOff { note: 60 },
                KeyboardEffect::NoteOn {
                    note: 62,
                    velocity: 96
                },
            ]
        );
        assert_eq!(keyboard_visual_state(&context).held_notes, vec![62]);
        let (context, effects) = keyboard_retarget(context, "pointer", Some(62), 110);
        assert!(effects.is_empty());
        assert_eq!(context.active_inputs[0].2, 110);
        let (context, effects) = keyboard_retarget(context, "pointer", None, 1);
        assert_eq!(effects, vec![KeyboardEffect::NoteOff { note: 62 }]);
        assert!(keyboard_visual_state(&context).held_notes.is_empty());
    }

    #[test]
    fn phase_three_waveform_reduces_extrema_and_caps_columns() {
        let mut fine = vec![
            WaveformPeakPair {
                min: -0.2,
                max: 0.4
            };
            WAVEFORM_MAX_COLUMNS + 8
        ];
        fine[0] = WaveformPeakPair {
            min: -2.0,
            max: 1.5,
        };
        let context = WaveformContext {
            pyramid: WaveformPeakPyramid {
                sample_count: fine.len(),
                levels: vec![WaveformPeakLevel {
                    samples_per_peak: 1,
                    peaks: fine,
                }],
            },
            visible_start: 0,
            visible_end: WAVEFORM_MAX_COLUMNS + 8,
            column_count: WAVEFORM_MAX_COLUMNS + 10,
            cursor_sample: None,
            selection: None,
            selection_anchor: None,
            selecting: false,
            focus: false,
            disabled: false,
        };
        let columns = waveform_columns(&context);
        assert_eq!(columns.len(), WAVEFORM_MAX_COLUMNS);
        assert_eq!(
            columns[0],
            WaveformPeakPair {
                min: -1.0,
                max: 1.0
            }
        );
        let selected = context.select_begin(7).select_move(2);
        assert_eq!(
            selected.selection,
            Some(WaveformSelection { start: 2, end: 7 })
        );
    }

    #[test]
    fn phase_three_matrix_is_row_major_and_bipolar() {
        let headers = vec![
            ModMatrixHeader {
                id: "a".into(),
                label: "A".into(),
            },
            ModMatrixHeader {
                id: "b".into(),
                label: "B".into(),
            },
        ];
        let context = ModMatrixContext::new(headers.clone(), headers, vec![])
            .move_focus(1, 1)
            .toggle();
        assert_eq!(context.cells.len(), 4);
        assert!(context.cells[3].enabled);
        assert_eq!(context.visual_state().cells[0].amount_norm, 0.5);
    }

    #[test]
    fn phase_three_matrix_supports_per_cell_unipolar_parameters() {
        let headers = vec![ModMatrixHeader {
            id: "a".into(),
            label: "A".into(),
        }];
        let context = ModMatrixContext::new(
            headers.clone(),
            headers,
            vec![ModMatrixCell {
                source_id: "a".into(),
                destination_id: "a".into(),
                amount: 0.4,
                enabled: true,
                parameters: ModMatrixCellParameters {
                    min: 0.0,
                    max: 1.0,
                    step: 0.1,
                    law: AudioValueLaw::Linear,
                },
            }],
        )
        .move_focus(0, 0)
        .nudge(1.0, false)
        .set_normalized(0.75);
        let visual = context.visual_state();
        close(context.cells[0].amount, 0.8);
        close(visual.cells[0].amount_norm, 0.8);
        close(visual.cells[0].zero_norm, 0.0);
        close(visual.cells[0].fill_start_norm, 0.0);
        close(visual.cells[0].fill_span_norm, 0.8);
    }

    #[test]
    #[should_panic(expected = "mod matrix source ids must be non-empty and unique")]
    fn phase_three_matrix_rejects_duplicate_axis_ids() {
        let duplicates = vec![
            ModMatrixHeader {
                id: "a".into(),
                label: "A".into(),
            },
            ModMatrixHeader {
                id: "a".into(),
                label: "Again".into(),
            },
        ];
        let _ = ModMatrixContext::new(duplicates, vec![], vec![]);
    }
    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }
    #[test]
    fn law_golden_values_match_web() {
        close(
            normalize_value(1000.0, 20.0, 20_000.0, AudioValueLaw::Logarithmic),
            0.5663233347786729,
        );
        close(
            denormalize_value(0.5, 20.0, 20_000.0, AudioValueLaw::Logarithmic),
            632.4555320336758,
        );
        close(
            normalize_value(0.0, -1.0, 1.0, AudioValueLaw::BipolarCenter { center: 0.0 }),
            0.5,
        );
    }
    #[test]
    fn format_golden_values_match_web() {
        assert_eq!(
            format_value(-12.4, AudioValueFormat::Db { decimals: 1 }),
            "-12.4 dB"
        );
        assert_eq!(
            format_value(12_500.0, AudioValueFormat::Hz { decimals: 2 }),
            "12.5 kHz"
        );
        assert_eq!(format_value(60.0, AudioValueFormat::Note), "C4");
        assert_eq!(
            parse_value("2.5 kHz", AudioValueFormat::Hz { decimals: 2 }),
            Some(2500.0)
        );
        assert_eq!(parse_value("D♭4", AudioValueFormat::Note), Some(61.0));
    }
    #[test]
    fn meter_ballistics_match_constants() {
        let mut vu = AudioMeterContext {
            mode: AudioMeterMode::Vu,
            ballistic_db: f64::NEG_INFINITY,
            ..Default::default()
        };
        vu = push_meter_frame(
            &vu,
            MeterFeedFrame {
                at_ms: 300.0,
                peak: 1.0,
                mean_square: 1.0,
                duration_ms: 300.0,
            },
        );
        close(db_to_amplitude(vu.ballistic_db), 1.0 - (-1.0_f64).exp());
        let peak = push_meter_frame(
            &AudioMeterContext::default(),
            MeterFeedFrame {
                at_ms: 16.0,
                peak: 1.0,
                mean_square: 1.0,
                duration_ms: 16.0,
            },
        );
        assert!(peak.clip);
        assert_eq!(peak.peak_hold_until_ms, Some(1516.0));
    }
    #[test]
    fn envelope_curve_golden_values_match_web() {
        let from = EnvelopePoint {
            id: "a".into(),
            x: 0.0,
            y: 0.0,
            curve: 0.5,
        };
        let to = EnvelopePoint {
            id: "b".into(),
            x: 1.0,
            y: 1.0,
            curve: 0.0,
        };
        close(envelope_segment_value_at(&from, &to, 0.5), 0.125);
    }

    fn knob(base: AudioValueContext) -> KnobContext {
        KnobContext {
            base,
            ..Default::default()
        }
    }

    #[test]
    fn knob_pairs_gestures_and_reuses_entry_parsing() {
        let context = knob(AudioValueContext {
            value: 250.0,
            min: 0.0,
            max: 5000.0,
            default_value: 440.0,
            keyboard_step: 1.0,
            format: AudioValueFormat::Milliseconds { decimals: 1 },
            ..Default::default()
        });
        let (context, effects) = knob_transition(
            context,
            AudioValueEvent::DragBegin {
                position: 100.0,
                fine: true,
            },
        );
        assert_eq!(context.base.drag, DragState::Fine);
        assert_eq!(effects, vec![AudioValueEffect::GestureBegin]);

        // A second begin cannot reopen or re-anchor the accepted gesture.
        let (context, effects) = knob_transition(
            context,
            AudioValueEvent::DragBegin {
                position: 40.0,
                fine: false,
            },
        );
        assert!(effects.is_empty());
        assert_eq!(context.base.drag, DragState::Fine);
        assert_eq!(context.base.drag_start_position, 100.0);

        let (context, effects) = knob_transition(context, AudioValueEvent::DragEnd);
        assert_eq!(
            effects,
            vec![
                AudioValueEffect::ValueCommit(250.0),
                AudioValueEffect::GestureEnd
            ]
        );
        // Repeated and stale terminals are inert.
        let (context, effects) = knob_transition(context, AudioValueEvent::DragEnd);
        assert!(effects.is_empty());
        let (context, effects) = knob_transition(context, AudioValueEvent::DragCancel);
        assert!(effects.is_empty());

        let (context, _) = knob_transition(
            context,
            AudioValueEvent::EntryCommit {
                text: "1.5 s".into(),
            },
        );
        assert_eq!(context.base.value, 1500.0);
    }

    #[test]
    fn knob_modes_anchor_movement_and_rebase_without_jumps() {
        let vertical = knob(AudioValueContext {
            value: 0.5,
            ..Default::default()
        });
        let (vertical, _) = knob_transition(
            vertical,
            AudioValueEvent::DragBegin {
                position: 100.0,
                fine: false,
            },
        );
        let (vertical, effects) = knob_transition(
            vertical,
            AudioValueEvent::DragMove {
                position: 84.0,
                fine: false,
            },
        );
        close(vertical.base.value, 0.6);
        assert_eq!(effects, vec![AudioValueEffect::ValueChange(0.6)]);

        // The transition that flips the modifier rebases only.
        let (vertical, effects) = knob_transition(
            vertical,
            AudioValueEvent::DragMove {
                position: 84.0,
                fine: true,
            },
        );
        assert!(effects.is_empty());
        close(vertical.base.value, 0.6);
        let (vertical, _) = knob_transition(
            vertical,
            AudioValueEvent::DragMove {
                position: 68.0,
                fine: true,
            },
        );
        close(vertical.base.value, 0.61);

        // Circular positions are inert in vertical mode and vice versa.
        let (vertical, effects) = knob_transition(
            vertical,
            AudioValueEvent::DragSetNorm {
                value_norm: 1.0,
                fine: true,
            },
        );
        assert!(effects.is_empty());
        close(vertical.base.value, 0.61);

        let circular = KnobContext {
            base: AudioValueContext {
                value: 0.5,
                ..Default::default()
            },
            drag_mode: KnobDragMode::Circular,
            ..Default::default()
        };
        let (circular, _) = knob_transition(
            circular,
            AudioValueEvent::DragBegin {
                position: 0.5,
                fine: true,
            },
        );
        let (circular, _) = knob_transition(
            circular,
            AudioValueEvent::DragSetNorm {
                value_norm: 1.0,
                fine: true,
            },
        );
        close(circular.base.value, 0.55);
        let (circular, effects) = knob_transition(
            circular,
            AudioValueEvent::DragMove {
                position: 0.0,
                fine: true,
            },
        );
        assert!(effects.is_empty());
        close(circular.base.value, 0.55);
    }

    #[test]
    fn fader_snaps_to_the_first_nearest_detent_and_maps_both_axes() {
        let context = FaderContext {
            base: AudioValueContext {
                value: 0.4,
                ..Default::default()
            },
            detents: vec![0.45, 0.55],
            detent_snap: 0.05,
            ..Default::default()
        };
        let (context, _) = fader_transition(
            context,
            AudioValueEvent::DragBegin {
                position: 0.4,
                fine: false,
            },
        );
        // 0.5 is equidistant from both detents; the first declared wins.
        let (context, effects) = fader_transition(
            context,
            AudioValueEvent::DragSetNorm {
                value_norm: 0.5,
                fine: false,
            },
        );
        close(context.base.value, 0.45);
        assert_eq!(effects, vec![AudioValueEffect::ValueChange(0.45)]);
        // Outside the radius the raw position wins.
        let (context, _) = fader_transition(
            context,
            AudioValueEvent::DragSetNorm {
                value_norm: 0.2,
                fine: false,
            },
        );
        close(context.base.value, 0.2);

        let rect = AudioRect {
            left: 10.0,
            top: 5.0,
            width: 100.0,
            height: 100.0,
        };
        let point = AudioPoint { x: 20.0, y: 25.0 };
        close(
            fader_point_to_norm(point, rect, FaderOrientation::Horizontal),
            0.1,
        );
        close(
            fader_point_to_norm(point, rect, FaderOrientation::Vertical),
            0.8,
        );
        close(
            knob_point_to_norm(
                AudioPoint { x: 50.0, y: 0.0 },
                AudioRect {
                    left: 0.0,
                    top: 0.0,
                    width: 100.0,
                    height: 100.0,
                },
            ),
            0.5,
        );
    }

    #[test]
    fn disabled_scalar_controls_stay_inert_but_still_close_a_gesture() {
        let context = knob(AudioValueContext {
            value: 0.5,
            disabled: true,
            ..Default::default()
        });
        for event in [
            AudioValueEvent::DragBegin {
                position: 0.0,
                fine: false,
            },
            AudioValueEvent::Wheel {
                direction: 1,
                fine: false,
            },
            AudioValueEvent::Reset,
            AudioValueEvent::KeyBound {
                bound: ValueBound::Max,
            },
            AudioValueEvent::EntryOpen,
        ] {
            let (next, effects) = knob_transition(context.clone(), event);
            assert!(effects.is_empty());
            close(next.base.value, 0.5);
            assert_eq!(next.base.drag, DragState::None);
        }

        // A control disabled mid-gesture still terminates exactly once.
        let open = knob(AudioValueContext {
            value: 0.5,
            drag: DragState::Coarse,
            disabled: true,
            ..Default::default()
        });
        let (closed, effects) = knob_transition(open, AudioValueEvent::DragCancel);
        assert_eq!(
            effects,
            vec![
                AudioValueEffect::ValueCommit(0.5),
                AudioValueEffect::GestureEnd
            ]
        );
        assert_eq!(closed.base.drag, DragState::None);
    }

    #[test]
    fn switch_and_xy_machines_commit_atomic_values() {
        let (pressed, _) =
            audio_switch_transition(AudioSwitchContext::default(), AudioSwitchEvent::Press);
        let (released, effects) = audio_switch_transition(pressed, AudioSwitchEvent::Release);
        assert_eq!(released.state, 1);
        assert_eq!(effects.last(), Some(&AudioSwitchEffect::StateCommit(1)));
        let xy = XYPadContext {
            x: 0.25,
            y: 0.75,
            ..Default::default()
        };
        let (xy, effects) = xy_pad_transition(
            xy,
            XYPadEvent::Nudge {
                axis: XYPadAxis::X,
                direction: 1,
                multiplier: 1.0,
                fine: false,
            },
        );
        close(xy.x, 0.26);
        close(xy.y, 0.75);
        assert_eq!(effects.last(), Some(&XYPadEffect::ValueCommit(0.26, 0.75)));
    }

    #[test]
    fn xy_pad_presses_at_position_then_anchors_fine_travel() {
        let context = XYPadContext::default();
        let (context, effects) = xy_pad_transition(
            context,
            XYPadEvent::DragBegin {
                x_norm: 0.25,
                y_norm: 0.75,
                fine: false,
            },
        );
        assert_eq!(
            effects,
            vec![
                XYPadEffect::GestureBegin,
                XYPadEffect::ValueChange(0.25, 0.75)
            ]
        );
        assert_eq!(context.drag, DragState::Coarse);

        // A second begin cannot reopen or re-anchor the accepted gesture.
        let (context, effects) = xy_pad_transition(
            context,
            XYPadEvent::DragBegin {
                x_norm: 0.9,
                y_norm: 0.1,
                fine: false,
            },
        );
        assert!(effects.is_empty());
        close(context.x, 0.25);

        // The modifier flip rebases both axes; travel resumes from the next move.
        let (context, effects) = xy_pad_transition(
            context,
            XYPadEvent::DragMove {
                x_norm: 0.25,
                y_norm: 0.75,
                fine: true,
            },
        );
        assert!(effects.is_empty());
        let (context, _) = xy_pad_transition(
            context,
            XYPadEvent::DragMove {
                x_norm: 0.75,
                y_norm: 0.25,
                fine: true,
            },
        );
        close(context.x, 0.3);
        close(context.y, 0.7);

        let (context, effects) = xy_pad_transition(context, XYPadEvent::DragCancel);
        assert_eq!(
            effects,
            vec![
                XYPadEffect::ValueCommit(context.x, context.y),
                XYPadEffect::GestureEnd
            ]
        );
        let (_, effects) = xy_pad_transition(context, XYPadEvent::DragEnd);
        assert!(effects.is_empty());

        let (x_norm, y_norm) = xy_pad_point_to_norm(
            AudioPoint { x: 60.0, y: 45.0 },
            AudioRect {
                left: 10.0,
                top: 20.0,
                width: 100.0,
                height: 50.0,
            },
        );
        close(x_norm, 0.5);
        close(y_norm, 0.5);
    }

    #[test]
    fn envelope_mutations_clamp_sort_and_preserve_flat_segments() {
        let points = vec![
            EnvelopePoint {
                id: "b".into(),
                x: 1.2,
                y: 0.5,
                curve: 2.0,
            },
            EnvelopePoint {
                id: "a".into(),
                x: 0.0,
                y: 0.5,
                curve: 0.0,
            },
        ];
        let normalized = normalize_envelope_points(&points);
        assert_eq!(normalized[0].id, "a");
        assert_eq!(normalized[1].x, 1.0);
        assert_eq!(normalized[1].curve, 1.0);
        assert_eq!(remove_envelope_point(&normalized, "a").len(), 1);
    }
}
