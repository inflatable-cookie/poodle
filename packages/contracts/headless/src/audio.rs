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

#[derive(Clone, Debug, PartialEq)]
pub struct AudioValueContext {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub law: AudioValueLaw,
    pub default_value: f64,
    pub keyboard_step: f64,
    pub automation: AutomationState,
    pub disabled: bool,
    pub drag: DragState,
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
            automation: AutomationState::None,
            disabled: false,
            drag: DragState::None,
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
        state.drag = self.drag;
        state.automation = self.automation;
        state
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioValueEvent {
    GestureBegin {
        fine: bool,
    },
    SetNormalized {
        value_norm: f64,
        fine: bool,
    },
    Nudge {
        direction: i8,
        multiplier: f64,
        fine: bool,
    },
    Bound {
        maximum: bool,
    },
    Reset,
    EntryCommit {
        text: String,
        format: AudioValueFormat,
    },
    GestureEnd,
    GestureCancel,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioValueEffect {
    ValueChange(f64),
    ValueCommit(f64),
    GestureBegin,
    GestureEnd,
}

fn commit_value(
    mut context: AudioValueContext,
    value: f64,
) -> (AudioValueContext, Vec<AudioValueEffect>) {
    let value = constrain_value(value, context.min, context.max, context.law);
    context.value = value;
    (
        context,
        vec![
            AudioValueEffect::ValueChange(value),
            AudioValueEffect::ValueCommit(value),
        ],
    )
}

pub fn audio_value_transition(
    mut context: AudioValueContext,
    event: AudioValueEvent,
) -> (AudioValueContext, Vec<AudioValueEffect>) {
    if context.disabled {
        return (context, vec![]);
    }
    match event {
        AudioValueEvent::GestureBegin { fine } if context.drag == DragState::None => {
            context.drag = if fine {
                DragState::Fine
            } else {
                DragState::Coarse
            };
            (context, vec![AudioValueEffect::GestureBegin])
        }
        AudioValueEvent::GestureBegin { .. } => (context, vec![]),
        AudioValueEvent::SetNormalized { value_norm, fine } => {
            let current = normalize_value(context.value, context.min, context.max, context.law);
            let norm = if fine {
                current + (value_norm - current) * 0.1
            } else {
                value_norm
            };
            let value = denormalize_value(norm, context.min, context.max, context.law);
            context.value = value;
            context.drag = if fine {
                DragState::Fine
            } else {
                DragState::Coarse
            };
            (context, vec![AudioValueEffect::ValueChange(value)])
        }
        AudioValueEvent::Nudge {
            direction,
            multiplier,
            fine,
        } => {
            let scale = if fine { 0.1 } else { 1.0 };
            let value = context.value
                + direction.signum() as f64 * context.keyboard_step * multiplier.max(0.0) * scale;
            commit_value(context, value)
        }
        AudioValueEvent::Bound { maximum } => {
            let value = if maximum { context.max } else { context.min };
            commit_value(context, value)
        }
        AudioValueEvent::Reset => {
            let value = context.default_value;
            commit_value(context, value)
        }
        AudioValueEvent::EntryCommit { text, format } => match parse_value(&text, format) {
            Some(value) => commit_value(context, value),
            None => (context, vec![]),
        },
        AudioValueEvent::GestureEnd | AudioValueEvent::GestureCancel
            if context.drag != DragState::None =>
        {
            context.drag = DragState::None;
            let value = context.value;
            (
                context,
                vec![
                    AudioValueEffect::ValueCommit(value),
                    AudioValueEffect::GestureEnd,
                ],
            )
        }
        AudioValueEvent::GestureEnd | AudioValueEvent::GestureCancel => (context, vec![]),
    }
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
    pub step_x: f64,
    pub step_y: f64,
    pub automation: AutomationState,
    pub disabled: bool,
    pub drag: DragState,
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
            step_x: 0.01,
            step_y: 0.01,
            automation: AutomationState::None,
            disabled: false,
            drag: DragState::None,
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
            hover: false,
            focus: false,
            drag: self.drag,
            automation: self.automation,
            enabled: !self.disabled,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XYPadEvent {
    GestureBegin {
        fine: bool,
    },
    SetNormalized {
        x_norm: f64,
        y_norm: f64,
        fine: bool,
    },
    Nudge {
        x_direction: i8,
        y_direction: i8,
        fine: bool,
    },
    Reset,
    GestureEnd,
    GestureCancel,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XYPadEffect {
    ValueChange(f64, f64),
    ValueCommit(f64, f64),
    GestureBegin,
    GestureEnd,
}
pub fn xy_pad_transition(
    mut context: XYPadContext,
    event: XYPadEvent,
) -> (XYPadContext, Vec<XYPadEffect>) {
    if context.disabled {
        return (context, vec![]);
    }
    match event {
        XYPadEvent::GestureBegin { fine } if context.drag == DragState::None => {
            context.drag = if fine {
                DragState::Fine
            } else {
                DragState::Coarse
            };
            (context, vec![XYPadEffect::GestureBegin])
        }
        XYPadEvent::SetNormalized {
            x_norm,
            y_norm,
            fine,
        } => {
            let current_x = normalize_value(context.x, context.min_x, context.max_x, context.law_x);
            let current_y = normalize_value(context.y, context.min_y, context.max_y, context.law_y);
            let scale = if fine { 0.1 } else { 1.0 };
            context.x = denormalize_value(
                current_x + (x_norm - current_x) * scale,
                context.min_x,
                context.max_x,
                context.law_x,
            );
            context.y = denormalize_value(
                current_y + (y_norm - current_y) * scale,
                context.min_y,
                context.max_y,
                context.law_y,
            );
            context.drag = if fine {
                DragState::Fine
            } else {
                DragState::Coarse
            };
            let (x, y) = (context.x, context.y);
            (context, vec![XYPadEffect::ValueChange(x, y)])
        }
        XYPadEvent::Nudge {
            x_direction,
            y_direction,
            fine,
        } => {
            let scale = if fine { 0.1 } else { 1.0 };
            context.x = clamp_value(
                context.x + x_direction as f64 * context.step_x * scale,
                context.min_x,
                context.max_x,
            );
            context.y = clamp_value(
                context.y + y_direction as f64 * context.step_y * scale,
                context.min_y,
                context.max_y,
            );
            let (x, y) = (context.x, context.y);
            (
                context,
                vec![
                    XYPadEffect::ValueChange(x, y),
                    XYPadEffect::ValueCommit(x, y),
                ],
            )
        }
        XYPadEvent::Reset => {
            context.x = constrain_value(
                context.default_x,
                context.min_x,
                context.max_x,
                context.law_x,
            );
            context.y = constrain_value(
                context.default_y,
                context.min_y,
                context.max_y,
                context.law_y,
            );
            let (x, y) = (context.x, context.y);
            (
                context,
                vec![
                    XYPadEffect::ValueChange(x, y),
                    XYPadEffect::ValueCommit(x, y),
                ],
            )
        }
        XYPadEvent::GestureEnd | XYPadEvent::GestureCancel if context.drag != DragState::None => {
            context.drag = DragState::None;
            let (x, y) = (context.x, context.y);
            (
                context,
                vec![XYPadEffect::ValueCommit(x, y), XYPadEffect::GestureEnd],
            )
        }
        _ => (context, vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn scalar_machine_pairs_gestures_and_reuses_entry_parsing() {
        let context = AudioValueContext {
            value: 250.0,
            min: 0.0,
            max: 5000.0,
            default_value: 440.0,
            keyboard_step: 1.0,
            ..Default::default()
        };
        let (context, effects) =
            audio_value_transition(context, AudioValueEvent::GestureBegin { fine: true });
        assert_eq!(context.drag, DragState::Fine);
        assert_eq!(effects, vec![AudioValueEffect::GestureBegin]);
        let (context, effects) = audio_value_transition(context, AudioValueEvent::GestureEnd);
        assert_eq!(
            effects,
            vec![
                AudioValueEffect::ValueCommit(250.0),
                AudioValueEffect::GestureEnd
            ]
        );
        let (context, _) = audio_value_transition(
            context,
            AudioValueEvent::EntryCommit {
                text: "1.5 s".into(),
                format: AudioValueFormat::Milliseconds { decimals: 1 },
            },
        );
        assert_eq!(context.value, 1500.0);
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
                x_direction: 1,
                y_direction: 0,
                fine: false,
            },
        );
        close(xy.x, 0.26);
        close(xy.y, 0.75);
        assert_eq!(effects.last(), Some(&XYPadEffect::ValueCommit(0.26, 0.75)));
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
