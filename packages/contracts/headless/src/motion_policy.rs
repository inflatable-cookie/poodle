//! Shared host motion policy and framework-free lifecycle laws.
//!
//! Architecture: `docs/architecture/012-semantic-motion-policy.md`
//! Contract: `docs/contracts/components/motion-policy-provider.md`
//! TypeScript mirror: `packages/core/src/motion-policy.ts`
//!
//! Hosts resolve system preference at their edge. Components never look up
//! media queries, OS settings, or backend capture clocks.

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MotionPolicy {
    #[default]
    Full,
    Reduced,
    Frozen,
}

impl MotionPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Reduced => "reduced",
            Self::Frozen => "frozen",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionProperty {
    Opacity,
    Rotate,
    TranslateX,
    TranslateY,
    ScaleX,
    ScaleY,
    Height,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionInterruption {
    None,
    Inert,
    Reverse,
    Retarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionRemnant {
    Endpoint,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuiApproximation {
    None,
    OpacityStandIn,
    StaticEndpoint,
}

pub const MOTION_DURATION_FAST_MS: u32 = 120;
pub const MOTION_DURATION_STANDARD_MS: u32 = 180;
pub const MOTION_DURATION_SLOW_MS: u32 = 260;
pub const MOTION_DURATION_SKELETON_PULSE_MS: u32 = 1600;

pub const MOTION_ROLE_DISCLOSURE_HEIGHT: &str = "disclosure-height";
pub const MOTION_ROLE_DISCLOSURE_INDICATOR: &str = "disclosure-indicator";
pub const MOTION_ROLE_TOAST_ENTER: &str = "toast-enter";
pub const MOTION_ROLE_TOAST_EXIT: &str = "toast-exit";
pub const MOTION_ROLE_TABS_UNDERLINE: &str = "tabs-underline";
pub const MOTION_ROLE_DISCRETE_STATE: &str = "discrete-state";
pub const MOTION_ROLE_LOADING_LOOP: &str = "loading-loop";

pub const MOTION_POLICY_DATA_ATTR: &str = "data-poodle-motion-policy";

pub fn resolve_motion_preference(preference: Option<MotionPolicy>) -> MotionPolicy {
    preference.unwrap_or(MotionPolicy::Full)
}

/// Restriction-only nesting: a descendant may freeze or reduce, never re-enable.
pub fn restrict_motion_policy(
    ancestor: Option<MotionPolicy>,
    requested: Option<MotionPolicy>,
) -> MotionPolicy {
    resolve_motion_preference(ancestor).max(resolve_motion_preference(requested))
}

pub fn motion_key(owner: &str, role: &str, channel: &str) -> String {
    format!("{owner}\u{1f}{role}\u{1f}{channel}")
}

pub fn role_allows_reduced_opacity(role: &str) -> bool {
    matches!(
        role,
        MOTION_ROLE_TOAST_ENTER | MOTION_ROLE_TOAST_EXIT | MOTION_ROLE_DISCRETE_STATE
    )
}

pub fn is_layout_motion_property(property: MotionProperty) -> bool {
    property == MotionProperty::Height
}

pub fn should_run_motion_loop(
    policy: MotionPolicy,
    animated: bool,
    first_frame_committed: bool,
) -> bool {
    animated && policy == MotionPolicy::Full && first_frame_committed
}

pub fn filter_motion_properties(
    policy: MotionPolicy,
    requested: &[MotionProperty],
    looped: bool,
    reduced_opacity: bool,
) -> Vec<MotionProperty> {
    match policy {
        MotionPolicy::Frozen => Vec::new(),
        MotionPolicy::Full => requested.to_vec(),
        MotionPolicy::Reduced => {
            if looped || !reduced_opacity {
                Vec::new()
            } else {
                requested
                    .iter()
                    .copied()
                    .filter(|property| *property == MotionProperty::Opacity)
                    .collect()
            }
        }
    }
}

pub fn gpui_motion_plan(properties: &[MotionProperty]) -> GpuiMotionPlan {
    let mut applied = Vec::new();
    let mut dropped = Vec::new();
    for property in properties {
        match property {
            MotionProperty::Opacity | MotionProperty::Rotate => applied.push(*property),
            other => dropped.push(*other),
        }
    }
    let approximation = if dropped.is_empty() {
        GpuiApproximation::None
    } else if dropped.iter().copied().any(is_layout_motion_property) {
        GpuiApproximation::StaticEndpoint
    } else if applied.contains(&MotionProperty::Opacity) {
        GpuiApproximation::OpacityStandIn
    } else {
        GpuiApproximation::StaticEndpoint
    };
    GpuiMotionPlan {
        applied,
        dropped,
        approximation,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GpuiMotionPlan {
    pub applied: Vec<MotionProperty>,
    pub dropped: Vec<MotionProperty>,
    pub approximation: GpuiApproximation,
}

#[derive(Clone, Debug)]
pub struct MotionIntent {
    pub owner: String,
    pub role: String,
    pub channel: String,
    pub target: String,
    pub properties: Vec<MotionProperty>,
    pub duration_ms: u32,
    pub looped: bool,
    pub initial: bool,
    pub first_frame_committed: bool,
    pub forced_static: bool,
    pub reversible: bool,
    pub reduced_opacity: Option<bool>,
}

impl MotionIntent {
    pub fn new(
        owner: impl Into<String>,
        role: impl Into<String>,
        channel: impl Into<String>,
        target: impl Into<String>,
        properties: Vec<MotionProperty>,
        duration_ms: u32,
    ) -> Self {
        Self {
            owner: owner.into(),
            role: role.into(),
            channel: channel.into(),
            target: target.into(),
            properties,
            duration_ms,
            looped: false,
            initial: false,
            first_frame_committed: false,
            forced_static: false,
            reversible: false,
            reduced_opacity: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotionDecision {
    pub key: String,
    pub schedule: bool,
    pub properties: Vec<MotionProperty>,
    pub duration_ms: u32,
    pub interruption: MotionInterruption,
    pub remnant: MotionRemnant,
    pub live_clock: bool,
    pub paint_endpoint: bool,
}

#[derive(Clone, Debug)]
pub struct MotionClock {
    pub key: String,
    pub target: String,
    pub progress: f32,
    pub properties: Vec<MotionProperty>,
    pub duration_ms: u32,
    pub original_duration_ms: u32,
    pub axis_from: f32,
    pub axis_to: f32,
    pub looped: bool,
    pub reversible: bool,
    pub reduced_opacity: bool,
}

#[derive(Clone, Debug)]
pub struct MotionTrace {
    pub policy: MotionPolicy,
    pub clocks: Vec<MotionClock>,
}

impl MotionTrace {
    pub fn new(policy: MotionPolicy) -> Self {
        Self {
            policy,
            clocks: Vec::new(),
        }
    }

    pub fn live_clock_count(&self) -> usize {
        self.clocks.len()
    }
}

pub fn create_motion_trace(policy: MotionPolicy) -> MotionTrace {
    MotionTrace::new(policy)
}

fn reduced_opacity_for(intent: &MotionIntent) -> bool {
    intent
        .reduced_opacity
        .unwrap_or_else(|| role_allows_reduced_opacity(&intent.role))
}

fn should_schedule(
    policy: MotionPolicy,
    intent: &MotionIntent,
    properties: &[MotionProperty],
) -> bool {
    if intent.forced_static || properties.is_empty() || policy == MotionPolicy::Frozen {
        return false;
    }
    if intent.looped {
        return policy == MotionPolicy::Full && intent.first_frame_committed;
    }
    if intent.initial {
        return false;
    }
    true
}

fn remove_clock(trace: &mut MotionTrace, key: &str) {
    trace.clocks.retain(|clock| clock.key != key);
}

fn axis_for_target(target: &str) -> Option<f32> {
    match target {
        "open" => Some(1.0),
        "closed" => Some(0.0),
        _ => None,
    }
}

pub fn activate_motion(trace: &mut MotionTrace, intent: MotionIntent) -> MotionDecision {
    let key = motion_key(&intent.owner, &intent.role, &intent.channel);
    let reduced_opacity = reduced_opacity_for(&intent);
    let properties = filter_motion_properties(
        trace.policy,
        &intent.properties,
        intent.looped,
        reduced_opacity,
    );
    let existing_index = trace.clocks.iter().position(|clock| clock.key == key);

    if let Some(index) = existing_index {
        if trace.clocks[index].target == intent.target {
            let existing = &trace.clocks[index];
            return MotionDecision {
                key,
                schedule: false,
                properties: existing.properties.clone(),
                duration_ms: existing.duration_ms,
                interruption: MotionInterruption::Inert,
                remnant: MotionRemnant::Endpoint,
                live_clock: true,
                paint_endpoint: false,
            };
        }

        if trace.clocks[index].reversible && intent.reversible {
            let existing = &trace.clocks[index];
            let current =
                existing.axis_from + (existing.axis_to - existing.axis_from) * existing.progress;
            let axis_to = axis_for_target(&intent.target)
                .unwrap_or(if existing.axis_to == 1.0 { 0.0 } else { 1.0 });
            let duration_ms =
                ((axis_to - current).abs() * existing.original_duration_ms as f32).round() as u32;
            let mut continue_intent = intent.clone();
            continue_intent.initial = false;
            let schedule =
                duration_ms > 0 && should_schedule(trace.policy, &continue_intent, &properties);
            if schedule {
                let existing = &mut trace.clocks[index];
                existing.target = intent.target;
                existing.progress = 0.0;
                existing.duration_ms = duration_ms;
                existing.axis_from = current;
                existing.axis_to = axis_to;
                existing.properties = properties.clone();
            } else {
                remove_clock(trace, &key);
            }
            return MotionDecision {
                key,
                schedule,
                properties,
                duration_ms,
                interruption: MotionInterruption::Reverse,
                remnant: MotionRemnant::Endpoint,
                live_clock: schedule,
                paint_endpoint: !schedule,
            };
        }

        if !trace.clocks[index].reversible {
            let duration_ms = trace.clocks[index].duration_ms;
            let mut continue_intent = intent.clone();
            continue_intent.initial = false;
            let schedule = should_schedule(trace.policy, &continue_intent, &properties);
            if schedule {
                let existing = &mut trace.clocks[index];
                existing.target = intent.target;
                existing.progress = 0.0;
                existing.properties = properties.clone();
            } else {
                remove_clock(trace, &key);
            }
            return MotionDecision {
                key,
                schedule,
                properties,
                duration_ms,
                interruption: MotionInterruption::Retarget,
                remnant: MotionRemnant::Endpoint,
                live_clock: schedule,
                paint_endpoint: !schedule,
            };
        }
    }

    let schedule = should_schedule(trace.policy, &intent, &properties);
    if schedule {
        let axis_to = axis_for_target(&intent.target).unwrap_or(1.0);
        trace.clocks.push(MotionClock {
            key: key.clone(),
            target: intent.target,
            progress: 0.0,
            properties: properties.clone(),
            duration_ms: intent.duration_ms,
            original_duration_ms: intent.duration_ms,
            axis_from: 1.0 - axis_to,
            axis_to,
            looped: intent.looped,
            reversible: intent.reversible,
            reduced_opacity,
        });
    }
    MotionDecision {
        key,
        schedule,
        properties,
        duration_ms: intent.duration_ms,
        interruption: MotionInterruption::None,
        remnant: MotionRemnant::Endpoint,
        live_clock: schedule,
        paint_endpoint: !schedule,
    }
}

pub fn sample_motion(trace: &mut MotionTrace, key: &str, progress: f32) {
    if let Some(clock) = trace.clocks.iter_mut().find(|clock| clock.key == key) {
        clock.progress = progress.clamp(0.0, 1.0);
    }
}

pub fn complete_motion(trace: &mut MotionTrace, key: &str) -> MotionDecision {
    let Some(index) = trace.clocks.iter().position(|clock| clock.key == key) else {
        return MotionDecision {
            key: key.to_string(),
            schedule: false,
            properties: Vec::new(),
            duration_ms: 0,
            interruption: MotionInterruption::None,
            remnant: MotionRemnant::Endpoint,
            live_clock: false,
            paint_endpoint: true,
        };
    };
    let looped = trace.clocks[index].looped;
    let properties = trace.clocks[index].properties.clone();
    let duration_ms = trace.clocks[index].duration_ms;
    if looped {
        trace.clocks[index].progress = 0.0;
    } else {
        trace.clocks.remove(index);
    }
    MotionDecision {
        key: key.to_string(),
        schedule: false,
        properties,
        duration_ms,
        interruption: MotionInterruption::None,
        remnant: MotionRemnant::Endpoint,
        live_clock: looped,
        paint_endpoint: true,
    }
}

pub fn set_motion_trace_policy(
    trace: &mut MotionTrace,
    policy: MotionPolicy,
) -> Vec<MotionDecision> {
    trace.policy = policy;
    let keys: Vec<String> = trace.clocks.iter().map(|clock| clock.key.clone()).collect();
    let mut decisions = Vec::new();
    for key in keys {
        let Some(index) = trace.clocks.iter().position(|clock| clock.key == key) else {
            continue;
        };
        let looped = trace.clocks[index].looped;
        let reduced_opacity = trace.clocks[index].reduced_opacity;
        let duration_ms = trace.clocks[index].duration_ms;
        let properties = filter_motion_properties(
            trace.policy,
            &trace.clocks[index].properties,
            looped,
            reduced_opacity,
        );
        if trace.policy == MotionPolicy::Frozen
            || properties.is_empty()
            || (trace.policy == MotionPolicy::Reduced && looped)
        {
            remove_clock(trace, &key);
            decisions.push(MotionDecision {
                key,
                schedule: false,
                properties,
                duration_ms,
                interruption: MotionInterruption::None,
                remnant: MotionRemnant::Endpoint,
                live_clock: false,
                paint_endpoint: true,
            });
            continue;
        }
        let continue_clock = trace.policy != MotionPolicy::Reduced
            || (properties.len() == 1 && properties[0] == MotionProperty::Opacity);
        if continue_clock {
            trace.clocks[index].properties = properties.clone();
        } else {
            remove_clock(trace, &key);
        }
        decisions.push(MotionDecision {
            key,
            schedule: continue_clock,
            properties,
            duration_ms,
            interruption: MotionInterruption::None,
            remnant: MotionRemnant::Endpoint,
            live_clock: continue_clock,
            paint_endpoint: !continue_clock,
        });
    }
    decisions
}

pub fn abort_motion(trace: &mut MotionTrace, key: Option<&str>) -> Vec<MotionDecision> {
    cancel_clocks(trace, key, MotionRemnant::Endpoint)
}

pub fn unmount_motion(trace: &mut MotionTrace, key: Option<&str>) -> Vec<MotionDecision> {
    cancel_clocks(trace, key, MotionRemnant::None)
}

fn cancel_clocks(
    trace: &mut MotionTrace,
    key: Option<&str>,
    remnant: MotionRemnant,
) -> Vec<MotionDecision> {
    let selected: Vec<MotionClock> = match key {
        Some(key) => trace
            .clocks
            .iter()
            .filter(|clock| clock.key == key)
            .cloned()
            .collect(),
        None => trace.clocks.clone(),
    };
    for clock in &selected {
        remove_clock(trace, &clock.key);
    }
    selected
        .into_iter()
        .map(|clock| MotionDecision {
            key: clock.key,
            schedule: false,
            properties: clock.properties,
            duration_ms: clock.duration_ms,
            interruption: MotionInterruption::None,
            remnant,
            live_clock: false,
            paint_endpoint: remnant == MotionRemnant::Endpoint,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_shot(target: &str, properties: Vec<MotionProperty>) -> MotionIntent {
        let mut intent = MotionIntent::new(
            "owner-a",
            MOTION_ROLE_DISCLOSURE_HEIGHT,
            "panel",
            target,
            properties,
            MOTION_DURATION_STANDARD_MS,
        );
        intent.reversible = true;
        intent
    }

    #[test]
    fn missing_preference_resolves_to_full() {
        assert_eq!(resolve_motion_preference(None), MotionPolicy::Full);
        assert_eq!(restrict_motion_policy(None, None), MotionPolicy::Full);
    }

    #[test]
    fn nesting_is_restriction_only() {
        assert_eq!(
            restrict_motion_policy(Some(MotionPolicy::Reduced), Some(MotionPolicy::Full)),
            MotionPolicy::Reduced
        );
        assert_eq!(
            restrict_motion_policy(Some(MotionPolicy::Frozen), Some(MotionPolicy::Reduced)),
            MotionPolicy::Frozen
        );
        assert_eq!(
            restrict_motion_policy(Some(MotionPolicy::Full), Some(MotionPolicy::Reduced)),
            MotionPolicy::Reduced
        );
        assert!(MotionPolicy::Full < MotionPolicy::Reduced);
        assert!(MotionPolicy::Reduced < MotionPolicy::Frozen);
    }

    #[test]
    fn keys_are_semantic_and_stable() {
        let first = motion_key("item-1", MOTION_ROLE_DISCLOSURE_HEIGHT, "panel");
        let rebuilt = motion_key("item-1", MOTION_ROLE_DISCLOSURE_HEIGHT, "panel");
        let sibling = motion_key("item-2", MOTION_ROLE_DISCLOSURE_HEIGHT, "panel");
        let channel = motion_key("item-1", MOTION_ROLE_DISCLOSURE_INDICATOR, "glyph");
        assert_eq!(first, rebuilt);
        assert_ne!(first, sibling);
        assert_ne!(first, channel);
    }

    #[test]
    fn authored_initial_state_paints_the_endpoint() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let mut intent = one_shot("open", vec![MotionProperty::Height, MotionProperty::Rotate]);
        intent.initial = true;
        let decision = activate_motion(&mut trace, intent);
        assert!(!decision.schedule);
        assert!(decision.paint_endpoint);
        assert_eq!(trace.live_clock_count(), 0);
    }

    #[test]
    fn loading_loop_waits_for_the_first_committed_frame() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let mut intent = MotionIntent::new(
            "skeleton",
            MOTION_ROLE_LOADING_LOOP,
            "pulse",
            "loading",
            vec![MotionProperty::Opacity],
            MOTION_DURATION_SKELETON_PULSE_MS,
        );
        intent.looped = true;
        let before = activate_motion(&mut trace, intent.clone());
        assert!(!before.schedule);
        assert_eq!(trace.live_clock_count(), 0);
        intent.first_frame_committed = true;
        let after = activate_motion(&mut trace, intent);
        assert!(after.schedule);
        assert_eq!(trace.live_clock_count(), 1);
    }

    #[test]
    fn repeated_target_is_inert_and_latest_state_wins() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let open = activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        assert!(open.schedule);
        sample_motion(&mut trace, &open.key, 0.4);
        let repeat = activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        assert_eq!(repeat.interruption, MotionInterruption::Inert);
        assert_eq!(trace.live_clock_count(), 1);
        assert!((trace.clocks[0].progress - 0.4).abs() < f32::EPSILON);

        let close = activate_motion(&mut trace, one_shot("closed", vec![MotionProperty::Height]));
        assert_eq!(close.interruption, MotionInterruption::Reverse);
        assert_eq!(close.duration_ms, 72);
        assert_eq!(trace.live_clock_count(), 1);
        assert_eq!(trace.clocks[0].target, "closed");
        assert_eq!(trace.clocks[0].progress, 0.0);

        sample_motion(&mut trace, &close.key, 0.5);
        let reopen = activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        assert_eq!(reopen.interruption, MotionInterruption::Reverse);
        assert_eq!(reopen.duration_ms, 144);
        assert_eq!(trace.clocks[0].target, "open");
    }

    #[test]
    fn controlled_close_starts_at_closed_direction_and_reverses_proportionally() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let close = activate_motion(&mut trace, one_shot("closed", vec![MotionProperty::Height]));
        assert!(close.schedule);
        assert_eq!(trace.clocks[0].axis_from, 1.0);
        assert_eq!(trace.clocks[0].axis_to, 0.0);

        sample_motion(&mut trace, &close.key, 0.25);
        let reopen = activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        assert_eq!(reopen.interruption, MotionInterruption::Reverse);
        assert_eq!(reopen.duration_ms, 45);
        assert_eq!(trace.clocks[0].axis_from, 0.75);
        assert_eq!(trace.clocks[0].axis_to, 1.0);
    }

    #[test]
    fn multi_target_retarget_does_not_queue() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let mut intent = MotionIntent::new(
            "tabs",
            MOTION_ROLE_TABS_UNDERLINE,
            "indicator",
            "a",
            vec![MotionProperty::TranslateX],
            MOTION_DURATION_STANDARD_MS,
        );
        activate_motion(&mut trace, intent.clone());
        intent.target = "b".into();
        activate_motion(&mut trace, intent.clone());
        intent.target = "c".into();
        let third = activate_motion(&mut trace, intent);
        assert_eq!(third.interruption, MotionInterruption::Retarget);
        assert_eq!(trace.live_clock_count(), 1);
        assert_eq!(trace.clocks[0].target, "c");
    }

    #[test]
    fn reduced_keeps_only_allowed_opacity_and_drops_layout() {
        assert_eq!(
            filter_motion_properties(
                MotionPolicy::Reduced,
                &[MotionProperty::Height, MotionProperty::Rotate],
                false,
                false,
            ),
            Vec::<MotionProperty>::new()
        );
        assert_eq!(
            filter_motion_properties(
                MotionPolicy::Reduced,
                &[MotionProperty::Opacity, MotionProperty::TranslateY],
                false,
                true,
            ),
            vec![MotionProperty::Opacity]
        );
        assert!(filter_motion_properties(
            MotionPolicy::Reduced,
            &[MotionProperty::Opacity],
            true,
            true,
        )
        .is_empty());
    }

    #[test]
    fn tightening_full_to_reduced_to_frozen_is_honest() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let mut toast = MotionIntent::new(
            "toast-1",
            MOTION_ROLE_TOAST_ENTER,
            "item",
            "enter",
            vec![MotionProperty::Opacity, MotionProperty::TranslateY],
            MOTION_DURATION_STANDARD_MS,
        );
        toast.reduced_opacity = Some(true);
        activate_motion(&mut trace, toast);
        let mut loop_intent = MotionIntent::new(
            "spinner",
            MOTION_ROLE_LOADING_LOOP,
            "ring",
            "spin",
            vec![MotionProperty::Rotate],
            800,
        );
        loop_intent.looped = true;
        loop_intent.first_frame_committed = true;
        activate_motion(&mut trace, loop_intent);
        assert_eq!(trace.live_clock_count(), 2);

        let reduced = set_motion_trace_policy(&mut trace, MotionPolicy::Reduced);
        assert_eq!(trace.policy, MotionPolicy::Reduced);
        assert_eq!(trace.live_clock_count(), 1);
        assert_eq!(trace.clocks[0].properties, vec![MotionProperty::Opacity]);
        assert!(reduced.iter().any(|decision| !decision.live_clock));

        let frozen = set_motion_trace_policy(&mut trace, MotionPolicy::Frozen);
        assert_eq!(trace.policy, MotionPolicy::Frozen);
        assert_eq!(trace.live_clock_count(), 0);
        assert!(frozen
            .iter()
            .all(|decision| !decision.live_clock && decision.paint_endpoint));

        set_motion_trace_policy(&mut trace, MotionPolicy::Full);
        assert_eq!(trace.policy, MotionPolicy::Full);
    }

    #[test]
    fn abort_keeps_the_endpoint_and_unmount_drops_the_remnant() {
        let mut trace = create_motion_trace(MotionPolicy::Full);
        let decision = activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        let aborted = abort_motion(&mut trace, Some(&decision.key));
        assert_eq!(aborted[0].remnant, MotionRemnant::Endpoint);
        assert_eq!(trace.live_clock_count(), 0);

        activate_motion(&mut trace, one_shot("open", vec![MotionProperty::Height]));
        let unmounted = unmount_motion(&mut trace, None);
        assert_eq!(unmounted[0].remnant, MotionRemnant::None);
        assert_eq!(trace.live_clock_count(), 0);
    }

    #[test]
    fn height_is_the_only_layout_exception_and_gpui_names_gaps() {
        assert!(is_layout_motion_property(MotionProperty::Height));
        assert!(!is_layout_motion_property(MotionProperty::TranslateY));
        let height = gpui_motion_plan(&[MotionProperty::Height]);
        assert_eq!(height.approximation, GpuiApproximation::StaticEndpoint);
        assert!(height.applied.is_empty());
        let toast = gpui_motion_plan(&[MotionProperty::Opacity, MotionProperty::TranslateY]);
        assert_eq!(toast.approximation, GpuiApproximation::OpacityStandIn);
        assert_eq!(toast.applied, vec![MotionProperty::Opacity]);
        let spin = gpui_motion_plan(&[MotionProperty::Rotate]);
        assert_eq!(spin.approximation, GpuiApproximation::None);
    }

    #[test]
    fn child_full_cannot_reenable_reduced() {
        let mut trace = create_motion_trace(MotionPolicy::Reduced);
        let mut intent = one_shot("open", vec![MotionProperty::Height]);
        intent.reduced_opacity = Some(false);
        let decision = activate_motion(&mut trace, intent);
        assert!(!decision.schedule);
        assert!(decision.paint_endpoint);
        assert_eq!(trace.live_clock_count(), 0);
    }
}
