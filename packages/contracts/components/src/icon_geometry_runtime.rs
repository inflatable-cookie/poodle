pub const ICON_GEOMETRY_ROLE: &str = "icon-geometry";
pub const ICON_GEOMETRY_CHANNEL: &str = "glyph";
pub const ICON_GEOMETRY_DURATION_MS: u32 =
    poodle_headless::motion_policy::MOTION_DURATION_STANDARD_MS;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryEndpoint {
    From,
    To,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompactGeometryContour {
    pub closed: bool,
    pub points: Vec<(i32, i32)>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompactGeometryFrame {
    pub contours: Vec<CompactGeometryContour>,
}

#[derive(Clone, Copy, Debug)]
pub struct IconGeometryPlan {
    pair_id: &'static str,
    left: &'static GeneratedIconGeometry,
    right: &'static GeneratedIconGeometry,
    plan: &'static GeneratedIconGeometryPlan,
}

impl IconGeometryPlan {
    pub fn pair_id(self) -> &'static str {
        self.pair_id
    }
}

#[derive(Clone, Debug)]
struct GeometryClock {
    key: String,
    pair_id: &'static str,
    target: GeometryEndpoint,
    progress: f32,
    duration_ms: u32,
    original_duration_ms: u32,
    axis_from: f32,
    axis_to: f32,
}

#[derive(Clone, Debug)]
pub struct IconGeometryRuntime {
    pub policy: MotionPolicy,
    clocks: Vec<GeometryClock>,
    pair_id: Option<&'static str>,
    plan: Option<IconGeometryPlan>,
    frame: CompactGeometryFrame,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryRuntimeIntent {
    pub owner: String,
    pub pair_id: String,
    pub target: GeometryEndpoint,
    pub initial: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryRuntimeDecision {
    pub key: String,
    pub schedule: bool,
    pub interruption: MotionInterruption,
    pub remnant: MotionRemnant,
    pub live_clock: bool,
    pub paint_endpoint: bool,
    pub accepted: bool,
    pub pair_id: Option<&'static str>,
}

pub fn create_icon_geometry_runtime(policy: MotionPolicy) -> IconGeometryRuntime {
    IconGeometryRuntime {
        policy,
        clocks: Vec::new(),
        pair_id: None,
        plan: None,
        frame: CompactGeometryFrame::default(),
    }
}

pub fn live_geometry_clock_count(runtime: &IconGeometryRuntime) -> usize {
    runtime.clocks.len()
}

pub fn candidate_fixture_ids() -> Vec<&'static str> {
    ICON_GEOMETRY_REGISTRY
        .iter()
        .filter(|pair| pair.status == GeneratedPairStatus::Candidate)
        .map(|pair| pair.id)
        .collect()
}

pub fn planned_candidate_fixture(pair_id: &str) -> Option<IconGeometryPlan> {
    ICON_GEOMETRY_REGISTRY.iter().find_map(|pair| {
        if pair.id != pair_id || pair.status != GeneratedPairStatus::Candidate {
            return None;
        }
        Some(IconGeometryPlan {
            pair_id: pair.id,
            left: pair.geometry_left?,
            right: pair.geometry_right?,
            plan: pair.plan?,
        })
    })
}

pub fn activate_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    intent: GeometryRuntimeIntent,
) -> GeometryRuntimeDecision {
    let key = motion_key(&intent.owner, ICON_GEOMETRY_ROLE, ICON_GEOMETRY_CHANNEL);
    let existing_index = runtime.clocks.iter().position(|clock| clock.key == key);
    let plan = planned_candidate_fixture(&intent.pair_id);
    let Some(plan) = plan else {
        if existing_index.is_some() {
            remove_clock(runtime, &key);
        }
        runtime.pair_id = None;
        runtime.plan = None;
        runtime.frame.contours.clear();
        return GeometryRuntimeDecision {
            key,
            schedule: false,
            interruption: if existing_index.is_some() {
                MotionInterruption::Retarget
            } else {
                MotionInterruption::None
            },
            remnant: MotionRemnant::Endpoint,
            live_clock: false,
            paint_endpoint: true,
            accepted: false,
            pair_id: None,
        };
    };

    if let Some(index) = existing_index {
        if runtime.clocks[index].pair_id == plan.pair_id
            && runtime.clocks[index].target == intent.target
        {
            return GeometryRuntimeDecision {
                key,
                schedule: false,
                interruption: MotionInterruption::Inert,
                remnant: MotionRemnant::Endpoint,
                live_clock: true,
                paint_endpoint: false,
                accepted: true,
                pair_id: Some(plan.pair_id),
            };
        }
        if runtime.clocks[index].pair_id == plan.pair_id {
            let current = runtime.clocks[index].axis_from
                + (runtime.clocks[index].axis_to - runtime.clocks[index].axis_from)
                    * runtime.clocks[index].progress;
            let axis_to = axis_for_target(intent.target);
            let duration_ms = ((axis_to - current).abs()
                * runtime.clocks[index].original_duration_ms as f32)
                .round() as u32;
            runtime.clocks[index].target = intent.target;
            runtime.clocks[index].progress = 0.0;
            runtime.clocks[index].duration_ms = duration_ms;
            runtime.clocks[index].axis_from = current;
            runtime.clocks[index].axis_to = axis_to;
            let schedule = duration_ms > 0 && should_schedule(runtime.policy, &intent);
            runtime.pair_id = Some(plan.pair_id);
            runtime.plan = Some(plan);
            if !schedule {
                write_current_frame(&mut runtime.frame, plan, axis_to);
                remove_clock(runtime, &key);
            }
            return GeometryRuntimeDecision {
                key,
                schedule,
                interruption: MotionInterruption::Reverse,
                remnant: MotionRemnant::Endpoint,
                live_clock: schedule,
                paint_endpoint: !schedule,
                accepted: true,
                pair_id: Some(plan.pair_id),
            };
        }
        remove_clock(runtime, &key);
    }

    let had_existing = existing_index.is_some();
    runtime.pair_id = Some(plan.pair_id);
    runtime.plan = Some(plan);
    let axis_to = axis_for_target(intent.target);
    let schedule = should_schedule(runtime.policy, &intent);
    if schedule {
        runtime.clocks.push(GeometryClock {
            key: key.clone(),
            pair_id: plan.pair_id,
            target: intent.target,
            progress: 0.0,
            duration_ms: ICON_GEOMETRY_DURATION_MS,
            original_duration_ms: ICON_GEOMETRY_DURATION_MS,
            axis_from: 1.0 - axis_to,
            axis_to,
        });
        write_current_frame(&mut runtime.frame, plan, 1.0 - axis_to);
    } else {
        write_current_frame(&mut runtime.frame, plan, axis_to);
    }
    GeometryRuntimeDecision {
        key,
        schedule,
        interruption: if had_existing {
            MotionInterruption::Retarget
        } else {
            MotionInterruption::None
        },
        remnant: MotionRemnant::Endpoint,
        live_clock: schedule,
        paint_endpoint: !schedule,
        accepted: true,
        pair_id: Some(plan.pair_id),
    }
}

pub fn sample_icon_geometry<'a>(
    runtime: &'a mut IconGeometryRuntime,
    key: &str,
    progress: f32,
) -> Option<&'a CompactGeometryFrame> {
    let plan = runtime.plan?;
    let axis = {
        let clock = runtime.clocks.iter_mut().find(|entry| entry.key == key)?;
        clock.progress = progress.clamp(0.0, 1.0);
        clock.axis_from + (clock.axis_to - clock.axis_from) * clock.progress
    };
    write_current_frame(&mut runtime.frame, plan, axis);
    Some(&runtime.frame)
}

pub fn current_icon_geometry_frame(runtime: &IconGeometryRuntime) -> Option<&CompactGeometryFrame> {
    if runtime.plan.is_none() || runtime.frame.contours.is_empty() {
        None
    } else {
        Some(&runtime.frame)
    }
}

pub fn complete_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    key: &str,
) -> GeometryRuntimeDecision {
    let Some(index) = runtime.clocks.iter().position(|clock| clock.key == key) else {
        return GeometryRuntimeDecision {
            key: key.to_string(),
            schedule: false,
            interruption: MotionInterruption::None,
            remnant: MotionRemnant::Endpoint,
            live_clock: false,
            paint_endpoint: true,
            accepted: runtime.plan.is_some(),
            pair_id: runtime.pair_id,
        };
    };
    if let Some(plan) = runtime.plan {
        write_current_frame(&mut runtime.frame, plan, runtime.clocks[index].axis_to);
    }
    remove_clock(runtime, key);
    GeometryRuntimeDecision {
        key: key.to_string(),
        schedule: false,
        interruption: MotionInterruption::None,
        remnant: MotionRemnant::Endpoint,
        live_clock: false,
        paint_endpoint: true,
        accepted: true,
        pair_id: runtime.pair_id,
    }
}

pub fn set_icon_geometry_policy(
    runtime: &mut IconGeometryRuntime,
    policy: MotionPolicy,
) -> Vec<GeometryRuntimeDecision> {
    runtime.policy = policy;
    let snap = policy != MotionPolicy::Full;
    let mut decisions = Vec::new();
    for clock in runtime.clocks.clone() {
        if snap {
            if let Some(plan) = runtime.plan {
                write_current_frame(&mut runtime.frame, plan, clock.axis_to);
            }
            remove_clock(runtime, &clock.key);
        }
        decisions.push(GeometryRuntimeDecision {
            key: clock.key,
            schedule: !snap,
            interruption: MotionInterruption::None,
            remnant: MotionRemnant::Endpoint,
            live_clock: !snap,
            paint_endpoint: snap,
            accepted: true,
            pair_id: Some(clock.pair_id),
        });
    }
    decisions
}

pub fn abort_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
) -> Vec<GeometryRuntimeDecision> {
    cancel_clocks(runtime, key, MotionRemnant::Endpoint)
}

pub fn teardown_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
) -> Vec<GeometryRuntimeDecision> {
    let decisions = cancel_clocks(runtime, key, MotionRemnant::None);
    if key.is_none() || runtime.clocks.is_empty() {
        runtime.pair_id = None;
        runtime.plan = None;
        runtime.frame.contours.clear();
    }
    decisions
}

fn should_schedule(policy: MotionPolicy, intent: &GeometryRuntimeIntent) -> bool {
    policy == MotionPolicy::Full && !intent.initial
}

fn axis_for_target(target: GeometryEndpoint) -> f32 {
    match target {
        GeometryEndpoint::To => 1.0,
        GeometryEndpoint::From => 0.0,
    }
}

fn write_current_frame(frame: &mut CompactGeometryFrame, plan: IconGeometryPlan, axis: f32) {
    write_generated_frame(plan, axis as f64, frame);
}

fn cancel_clocks(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
    remnant: MotionRemnant,
) -> Vec<GeometryRuntimeDecision> {
    let selected: Vec<GeometryClock> = match key {
        Some(key) => runtime
            .clocks
            .iter()
            .filter(|clock| clock.key == key)
            .cloned()
            .collect(),
        None => runtime.clocks.clone(),
    };
    for clock in &selected {
        if remnant == MotionRemnant::Endpoint {
            if let Some(plan) = runtime.plan {
                if clock.pair_id == plan.pair_id {
                    write_current_frame(&mut runtime.frame, plan, clock.axis_to);
                }
            }
        }
        remove_clock(runtime, &clock.key);
    }
    selected
        .into_iter()
        .map(|clock| GeometryRuntimeDecision {
            key: clock.key,
            schedule: false,
            interruption: MotionInterruption::None,
            remnant,
            live_clock: false,
            paint_endpoint: remnant == MotionRemnant::Endpoint,
            accepted: true,
            pair_id: Some(clock.pair_id),
        })
        .collect()
}

fn remove_clock(runtime: &mut IconGeometryRuntime, key: &str) {
    runtime.clocks.retain(|clock| clock.key != key);
}

fn write_generated_frame(plan: IconGeometryPlan, progress: f64, out: &mut CompactGeometryFrame) {
    if !progress.is_finite() {
        return;
    }
    if progress <= 0.0 {
        write_canonical(plan.left, out);
        return;
    }
    if progress >= 1.0 {
        write_canonical(plan.right, out);
        return;
    }
    ensure_contours(out, plan.left.contours.len());
    for (left_index, left_contour) in plan.left.contours.iter().enumerate() {
        let mapping = plan
            .plan
            .contour_mappings
            .iter()
            .find(|mapping| mapping.left_index == left_index)
            .expect("candidate fixtures emit a mapping for every left contour");
        let right_contour = &plan.right.contours[mapping.right_index];
        let dest = &mut out.contours[left_index];
        dest.closed = left_contour.closed;
        ensure_points(dest, left_contour.samples.len());
        for (index, left_point) in left_contour.samples.iter().enumerate() {
            let source = if mapping.reversed {
                modulo(
                    mapping.offset as isize - index as isize,
                    right_contour.samples.len(),
                )
            } else {
                modulo(
                    mapping.offset as isize + index as isize,
                    right_contour.samples.len(),
                )
            };
            let right_point = right_contour.samples[source];
            dest.points[index] = (
                ((left_point.x as f64) + (right_point.x - left_point.x) as f64 * progress).round()
                    as i32,
                ((left_point.y as f64) + (right_point.y - left_point.y) as f64 * progress).round()
                    as i32,
            );
        }
    }
}

fn write_canonical(geometry: &GeneratedIconGeometry, out: &mut CompactGeometryFrame) {
    ensure_contours(out, geometry.contours.len());
    for (index, contour) in geometry.contours.iter().enumerate() {
        let dest = &mut out.contours[index];
        dest.closed = contour.closed;
        let count = 1 + contour
            .segments
            .iter()
            .filter(|segment| !segment.closing)
            .count();
        ensure_points(dest, count);
        dest.points[0] = (
            contour.segments[0].start.x as i32,
            contour.segments[0].start.y as i32,
        );
        let mut written = 1;
        for segment in contour.segments.iter().filter(|segment| !segment.closing) {
            dest.points[written] = (segment.end.x as i32, segment.end.y as i32);
            written += 1;
        }
    }
}

fn ensure_contours(out: &mut CompactGeometryFrame, count: usize) {
    while out.contours.len() < count {
        out.contours.push(CompactGeometryContour {
            closed: false,
            points: Vec::new(),
        });
    }
    out.contours.truncate(count);
}

fn ensure_points(contour: &mut CompactGeometryContour, count: usize) {
    while contour.points.len() < count {
        contour.points.push((0, 0));
    }
    contour.points.truncate(count);
}
