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
    owner: Option<String>,
    clock: Option<GeometryClock>,
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
        owner: None,
        clock: None,
        pair_id: None,
        plan: None,
        frame: CompactGeometryFrame::default(),
    }
}

pub fn live_geometry_clock_count(runtime: &IconGeometryRuntime) -> usize {
    usize::from(runtime.clock.is_some())
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
    let same_owner = runtime.owner.as_deref() == Some(intent.owner.as_str());
    let plan = planned_candidate_fixture(&intent.pair_id);
    let Some(plan) = plan else {
        let had_clock = runtime.clock.is_some();
        clear_runtime(runtime);
        return GeometryRuntimeDecision {
            key,
            schedule: false,
            interruption: if had_clock {
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

    if same_owner {
        if let Some(clock) = runtime.clock.as_ref() {
            if clock.pair_id == plan.pair_id && clock.target == intent.target {
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
            if clock.pair_id == plan.pair_id {
                let current = clock.axis_from + (clock.axis_to - clock.axis_from) * clock.progress;
                let axis_to = axis_for_target(intent.target);
                let duration_ms =
                    ((axis_to - current).abs() * clock.original_duration_ms as f32).round() as u32;
                let schedule = duration_ms > 0 && should_schedule(runtime.policy, &intent);
                runtime.owner = Some(intent.owner);
                runtime.pair_id = Some(plan.pair_id);
                bind_plan(runtime, plan);
                if let Some(clock) = runtime.clock.as_mut() {
                    clock.key = key.clone();
                    clock.target = intent.target;
                    clock.progress = 0.0;
                    clock.duration_ms = duration_ms;
                    clock.axis_from = current;
                    clock.axis_to = axis_to;
                }
                if !schedule {
                    write_current_frame(&mut runtime.frame, plan, axis_to);
                    runtime.clock = None;
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
        }
    }

    let interruption = if runtime.clock.is_some() {
        MotionInterruption::Retarget
    } else {
        MotionInterruption::None
    };
    let schedule = should_schedule(runtime.policy, &intent);
    runtime.clock = None;
    runtime.owner = Some(intent.owner);
    runtime.pair_id = Some(plan.pair_id);
    bind_plan(runtime, plan);
    let axis_to = axis_for_target(intent.target);
    if schedule {
        runtime.clock = Some(GeometryClock {
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
        interruption,
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
        let clock = runtime.clock.as_mut()?;
        if clock.key != key {
            return None;
        }
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
    let Some(clock) = runtime.clock.as_ref() else {
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
    if clock.key != key {
        return GeometryRuntimeDecision {
            key: key.to_string(),
            schedule: false,
            interruption: MotionInterruption::None,
            remnant: MotionRemnant::Endpoint,
            live_clock: true,
            paint_endpoint: false,
            accepted: runtime.plan.is_some(),
            pair_id: runtime.pair_id,
        };
    }
    let axis_to = clock.axis_to;
    if let Some(plan) = runtime.plan {
        write_current_frame(&mut runtime.frame, plan, axis_to);
    }
    runtime.clock = None;
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
    let Some(clock) = runtime.clock.clone() else {
        return Vec::new();
    };
    let snap = policy != MotionPolicy::Full;
    if snap {
        if let Some(plan) = runtime.plan {
            write_current_frame(&mut runtime.frame, plan, clock.axis_to);
        }
        runtime.clock = None;
    }
    vec![GeometryRuntimeDecision {
        key: clock.key,
        schedule: !snap,
        interruption: MotionInterruption::None,
        remnant: MotionRemnant::Endpoint,
        live_clock: !snap,
        paint_endpoint: snap,
        accepted: true,
        pair_id: Some(clock.pair_id),
    }]
}

pub fn abort_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
) -> Vec<GeometryRuntimeDecision> {
    cancel_clock(runtime, key, MotionRemnant::Endpoint)
}

pub fn teardown_icon_geometry(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
) -> Vec<GeometryRuntimeDecision> {
    let decisions = cancel_clock(runtime, key, MotionRemnant::None);
    if key.is_none() || runtime.clock.is_none() {
        clear_runtime(runtime);
    }
    decisions
}

pub(crate) fn compact_frame_point_caps(runtime: &IconGeometryRuntime) -> Vec<usize> {
    runtime
        .frame
        .contours
        .iter()
        .map(|contour| contour.points.capacity())
        .collect()
}

pub(crate) fn compact_frame_point_ptrs(runtime: &IconGeometryRuntime) -> Vec<*const (i32, i32)> {
    runtime
        .frame
        .contours
        .iter()
        .map(|contour| contour.points.as_ptr())
        .collect()
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

fn bind_plan(runtime: &mut IconGeometryRuntime, plan: IconGeometryPlan) {
    runtime.plan = Some(plan);
    reserve_for_plan(&mut runtime.frame, plan);
}

fn write_current_frame(frame: &mut CompactGeometryFrame, plan: IconGeometryPlan, axis: f32) {
    write_generated_frame(plan, axis as f64, frame);
}

fn cancel_clock(
    runtime: &mut IconGeometryRuntime,
    key: Option<&str>,
    remnant: MotionRemnant,
) -> Vec<GeometryRuntimeDecision> {
    let Some(clock) = runtime.clock.clone() else {
        return Vec::new();
    };
    if let Some(key) = key {
        if clock.key != key {
            return Vec::new();
        }
    }
    if remnant == MotionRemnant::Endpoint {
        if let Some(plan) = runtime.plan {
            if clock.pair_id == plan.pair_id {
                write_current_frame(&mut runtime.frame, plan, clock.axis_to);
            }
        }
    }
    runtime.clock = None;
    vec![GeometryRuntimeDecision {
        key: clock.key,
        schedule: false,
        interruption: MotionInterruption::None,
        remnant,
        live_clock: false,
        paint_endpoint: remnant == MotionRemnant::Endpoint,
        accepted: true,
        pair_id: Some(clock.pair_id),
    }]
}

fn clear_runtime(runtime: &mut IconGeometryRuntime) {
    runtime.owner = None;
    runtime.clock = None;
    runtime.pair_id = None;
    runtime.plan = None;
    runtime.frame.contours.clear();
}

fn reserve_for_plan(frame: &mut CompactGeometryFrame, plan: IconGeometryPlan) {
    ensure_contours(frame, plan.left.contours.len());
    for (index, left) in plan.left.contours.iter().enumerate() {
        let sampled = left.samples.len();
        let left_canonical = canonical_vertex_count(left);
        let right_canonical = canonical_vertex_count(&plan.right.contours[index]);
        ensure_points(
            &mut frame.contours[index],
            sampled.max(left_canonical).max(right_canonical),
        );
    }
}

fn mapping_for_left(
    plan: IconGeometryPlan,
    left_index: usize,
) -> &'static GeneratedContourCorrespondence {
    let mappings = plan.plan.contour_mappings;
    if let Some(direct) = mappings.get(left_index) {
        if direct.left_index == left_index {
            return direct;
        }
    }
    mappings
        .iter()
        .find(|mapping| mapping.left_index == left_index)
        .expect("candidate fixtures emit a mapping for every left contour")
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
        let mapping = mapping_for_left(plan, left_index);
        let right_contour = &plan.right.contours[mapping.right_index];
        let dest = &mut out.contours[left_index];
        dest.closed = left_contour.closed;
        ensure_points(dest, left_contour.samples.len());
        dest.points.truncate(left_contour.samples.len());
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

fn canonical_vertex_count(contour: &GeneratedGeometryContour) -> usize {
    1 + contour
        .segments
        .iter()
        .filter(|segment| !segment.closing)
        .count()
}

fn write_canonical(geometry: &GeneratedIconGeometry, out: &mut CompactGeometryFrame) {
    ensure_contours(out, geometry.contours.len());
    for (index, contour) in geometry.contours.iter().enumerate() {
        let dest = &mut out.contours[index];
        dest.closed = contour.closed;
        let count = canonical_vertex_count(contour);
        ensure_points(dest, count);
        dest.points.truncate(count);
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
    if contour.points.capacity() < count {
        contour.points.reserve(count - contour.points.len());
    }
    while contour.points.len() < count {
        contour.points.push((0, 0));
    }
}
