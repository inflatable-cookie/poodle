//! Domain-math conformance: date and color vectors generated from the
//! TypeScript core (`vectors/domain.json`); the Rust mirror must reproduce
//! every expectation exactly.

use serde_json::{json, Value};

use poodle_headless::color::*;
use poodle_headless::date::*;

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Number(number) => json!(number.as_f64().unwrap_or(0.0)),
        Value::Array(entries) => Value::Array(entries.iter().map(canonicalize).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, entry)| (key.clone(), canonicalize(entry)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn vectors() -> Value {
    serde_json::from_str(include_str!("../vectors/domain.json")).expect("vectors parse")
}

fn s<'a>(case: &'a Value, key: &str) -> &'a str {
    case[key].as_str().unwrap_or("")
}

fn week_start(case: &Value) -> WeekStart {
    if s(case, "weekStartsOn") == "sunday" {
        WeekStart::Sunday
    } else {
        WeekStart::Monday
    }
}

#[test]
fn date_conformance() {
    for case in vectors()["date"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "addDays" => {
                let date = parse_iso_date(s(case, "iso")).unwrap();
                let result = format_iso_date(add_days(date, case["amount"].as_i64().unwrap()));
                assert_eq!(result, expect.as_str().unwrap(), "addDays {case}");
            }
            "addMonths" => {
                let date = parse_iso_date(s(case, "iso")).unwrap();
                let result =
                    format_iso_date(add_months(date, case["amount"].as_i64().unwrap() as i32));
                assert_eq!(result, expect.as_str().unwrap(), "addMonths {case}");
            }
            "parse" => {
                let result = parse_iso_date(s(case, "iso")).map(format_iso_date);
                let expected = expect.as_str().map(str::to_string);
                assert_eq!(result, expected, "parse {case}");
            }
            "compare" => {
                let result = compare_iso_date(s(case, "left"), s(case, "right")).unwrap();
                assert_eq!(
                    i64::from(result),
                    expect.as_i64().unwrap(),
                    "compare {case}"
                );
            }
            "monthAnchor" => {
                assert_eq!(
                    month_anchor_iso(s(case, "iso")).unwrap(),
                    expect.as_str().unwrap(),
                    "monthAnchor {case}"
                );
            }
            "normalizeRange" => {
                let (start, end) =
                    normalize_date_range(Some(s(case, "start")), Some(s(case, "end")));
                assert_eq!(
                    start.as_deref(),
                    expect["start"].as_str(),
                    "normalizeRange start {case}"
                );
                assert_eq!(
                    end.as_deref(),
                    expect["end"].as_str(),
                    "normalizeRange end {case}"
                );
            }
            "withinRange" => {
                let result = is_iso_date_within_range(
                    s(case, "iso"),
                    Some(s(case, "start")),
                    Some(s(case, "end")),
                );
                assert_eq!(result, expect.as_bool().unwrap(), "withinRange {case}");
            }
            "startOfWeek" => {
                let date = parse_iso_date(s(case, "iso")).unwrap();
                let result = format_iso_date(start_of_week(date, week_start(case)));
                assert_eq!(result, expect.as_str().unwrap(), "startOfWeek {case}");
            }
            "weekBoundaryDelta" => {
                let to_end = s(case, "edge") == "end";
                let result = day_delta_for_week_boundary(s(case, "iso"), week_start(case), to_end);
                assert_eq!(
                    i64::from(result),
                    expect.as_i64().unwrap(),
                    "weekBoundaryDelta {case}"
                );
            }
            "daysBetween" => {
                let result = days_between(s(case, "start"), s(case, "end")).unwrap();
                assert_eq!(result, expect.as_i64().unwrap(), "daysBetween {case}");
            }
            "calendarWeeks" => {
                // today far outside every vector month so isToday never fires
                let weeks =
                    build_calendar_weeks(s(case, "visibleMonth"), week_start(case), "1900-01-01");
                let actual: Value = json!(weeks
                    .iter()
                    .map(|week| week
                        .iter()
                        .map(|day| json!({ "iso": day.iso, "label": day.label, "inMonth": day.in_month }))
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>());
                assert_eq!(&actual, expect, "calendarWeeks {}", s(case, "visibleMonth"));
            }
            other => panic!("unknown date op {other}"),
        }
    }
}

fn rgb_json(rgb: Rgb) -> Value {
    json!({ "r": rgb.r, "g": rgb.g, "b": rgb.b })
}

#[test]
fn color_conformance() {
    for case in vectors()["color"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];
        let ri = || case["r"].as_u64().unwrap() as u8;
        let gi = || case["g"].as_u64().unwrap() as u8;
        let bi = || case["b"].as_u64().unwrap() as u8;
        let hf = || case["h"].as_f64().unwrap();
        let sf = || case["s"].as_f64().unwrap();

        match op {
            "normalizeHex" => assert_eq!(
                normalize_hex(s(case, "hex")),
                expect.as_str().unwrap(),
                "{case}"
            ),
            "isValidHex" => assert_eq!(
                is_valid_hex(s(case, "hex")),
                expect.as_bool().unwrap(),
                "{case}"
            ),
            "hexToRgb" => {
                let (rgb, alpha) = hex_to_rgb(s(case, "hex")).unwrap();
                let mut actual = rgb_json(rgb);
                if let Some(a) = alpha {
                    actual["a"] = json!(a);
                }
                // expected alpha is a JS float; compare numerically
                let mut expected = expect.clone();
                if let Some(a) = expected.get("a").and_then(Value::as_f64) {
                    expected["a"] = json!(a);
                    let actual_a = actual["a"].as_f64().unwrap();
                    assert!((actual_a - a).abs() < 1e-9, "hexToRgb alpha {case}");
                    actual.as_object_mut().unwrap().remove("a");
                    expected.as_object_mut().unwrap().remove("a");
                }
                assert_eq!(actual, expected, "hexToRgb {case}");
            }
            "rgbToHex" => assert_eq!(
                rgb_to_hex(
                    Rgb {
                        r: ri(),
                        g: gi(),
                        b: bi()
                    },
                    None
                ),
                expect.as_str().unwrap(),
                "{case}"
            ),
            "rgbToHexAlpha" => {
                let a = case["a"].as_f64().unwrap();
                assert_eq!(
                    rgb_to_hex(
                        Rgb {
                            r: ri(),
                            g: gi(),
                            b: bi()
                        },
                        Some(a)
                    ),
                    expect.as_str().unwrap(),
                    "{case}"
                );
            }
            "rgbToHsv" => {
                let hsv = rgb_to_hsv(Rgb {
                    r: ri(),
                    g: gi(),
                    b: bi(),
                });
                assert_eq!(
                    json!({ "h": hsv.h, "s": hsv.s, "v": hsv.v }),
                    *expect,
                    "rgbToHsv {case}"
                );
            }
            "rgbToHsl" => {
                let hsl = rgb_to_hsl(Rgb {
                    r: ri(),
                    g: gi(),
                    b: bi(),
                });
                assert_eq!(
                    json!({ "h": hsl.h, "s": hsl.s, "l": hsl.l }),
                    *expect,
                    "rgbToHsl {case}"
                );
            }
            "hsvToRgb" => {
                let v = case["v"].as_f64().unwrap();
                assert_eq!(
                    rgb_json(hsv_to_rgb(hf(), sf(), v)),
                    *expect,
                    "hsvToRgb {case}"
                );
            }
            "hsvToHex" => {
                let v = case["v"].as_f64().unwrap();
                assert_eq!(
                    hsv_to_hex(hf(), sf(), v, None),
                    expect.as_str().unwrap(),
                    "hsvToHex {case}"
                );
            }
            "hslToRgb" => {
                let l = case["l"].as_f64().unwrap();
                assert_eq!(
                    rgb_json(hsl_to_rgb(hf(), sf(), l)),
                    *expect,
                    "hslToRgb {case}"
                );
            }
            "hexToHsv" => {
                let hsv = hex_to_hsv(s(case, "hex")).unwrap();
                assert_eq!(
                    json!({ "h": hsv.h, "s": hsv.s, "v": hsv.v }),
                    *expect,
                    "hexToHsv {case}"
                );
            }
            other => panic!("unknown color op {other}"),
        }
    }
}

#[test]
fn pagination_conformance() {
    use poodle_headless::pagination::*;

    for case in vectors()["pagination"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "visiblePages" => {
                let pages = build_visible_pages(
                    case["page"].as_u64().unwrap() as usize,
                    case["count"].as_u64().unwrap() as usize,
                    case["siblings"].as_u64().unwrap() as usize,
                );
                let actual: Vec<Value> = pages
                    .iter()
                    .map(|item| match item {
                        VisiblePage::Page(page) => json!(page),
                        VisiblePage::Ellipsis => json!("ellipsis"),
                    })
                    .collect();
                assert_eq!(json!(actual), *expect, "visiblePages {case}");
            }
            "canRequestPage" => {
                let result = can_request_page(
                    case["next"].as_i64().unwrap(),
                    case["current"].as_i64().unwrap(),
                    case["total"].as_i64().unwrap(),
                );
                assert_eq!(result, expect.as_bool().unwrap(), "canRequestPage {case}");
            }
            other => panic!("unknown pagination op {other}"),
        }
    }
}

// ── Tree ──

struct VecNode {
    value: String,
    children: Vec<VecNode>,
    is_branch: bool,
    is_disabled: bool,
}

impl poodle_headless::tree::TreeNodeLike for VecNode {
    fn value(&self) -> &str {
        &self.value
    }
    fn children(&self) -> &[Self] {
        &self.children
    }
    fn is_branch_flag(&self) -> bool {
        self.is_branch
    }
    fn is_disabled(&self) -> bool {
        self.is_disabled
    }
}

fn nodes_from(value: &Value) -> Vec<VecNode> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .map(|entry| VecNode {
                    value: s(entry, "value").to_string(),
                    children: nodes_from(&entry["children"]),
                    is_branch: entry["isBranch"].as_bool().unwrap_or(false),
                    is_disabled: entry["isDisabled"].as_bool().unwrap_or(false),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn strings_from(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn tree_conformance() {
    use poodle_headless::tree::*;

    let doc = vectors();
    let nodes = nodes_from(&doc["treeNodes"]);

    for case in doc["tree"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "flatten" => {
                let rows = flatten_visible_tree_rows(&nodes, &strings_from(&case["expanded"]));
                let actual: Vec<Value> = rows
                    .iter()
                    .map(|row| json!({ "value": row.value, "depth": row.depth, "parent": row.parent, "disabled": row.disabled }))
                    .collect();
                assert_eq!(json!(actual), *expect, "flatten {case}");
            }
            "checkState" => {
                let node = find_tree_node(&nodes, s(case, "value")).unwrap();
                let state = tree_check_state(node, &strings_from(&case["checked"]));
                let name = match state {
                    TreeCheckState::Checked => "checked",
                    TreeCheckState::Unchecked => "unchecked",
                    TreeCheckState::Mixed => "mixed",
                };
                assert_eq!(name, expect.as_str().unwrap(), "checkState {case}");
            }
            "toggleCheck" => {
                let node = find_tree_node(&nodes, s(case, "value")).unwrap();
                let mut next = tree_toggle_check(node, &strings_from(&case["checked"]));
                next.sort();
                assert_eq!(json!(next), *expect, "toggleCheck {case}");
            }
            "range" => {
                let rows = flatten_visible_tree_rows(&nodes, &strings_from(&case["expanded"]));
                let range = tree_range_selection(&rows, Some(s(case, "anchor")), s(case, "to"));
                assert_eq!(json!(range), *expect, "range {case}");
            }
            "siblingTarget" => {
                let siblings = strings_from(&case["siblings"]);
                let up = case["up"].as_bool().unwrap();
                let target = tree_sibling_reorder_target(&siblings, s(case, "value"), up);
                let actual = target.map(|step| {
                    json!({ "target": step.target, "position": if step.before { "before" } else { "after" } })
                });
                assert_eq!(json!(actual), *expect, "siblingTarget {case}");
            }
            "keydown" => {
                let expanded = strings_from(&case["expanded"]);
                let rows = flatten_visible_tree_rows(&nodes, &expanded);
                let shift = case["shift"].as_bool().unwrap_or(false);
                let intent = tree_keydown_intent(
                    &rows,
                    s(case, "value"),
                    s(case, "key"),
                    TreeKeyModifiers { alt: false, shift },
                    false,
                    &expanded,
                );
                let actual = intent.map(|intent| match intent {
                    TreeKeyIntent::Focus { value, extend_selection } => {
                        json!({ "type": "focus", "value": value, "extendSelection": extend_selection })
                    }
                    TreeKeyIntent::Expand { value } => json!({ "type": "expand", "value": value }),
                    TreeKeyIntent::Collapse { value } => json!({ "type": "collapse", "value": value }),
                    TreeKeyIntent::FocusParent { parent } => json!({ "type": "focusParent", "parent": parent }),
                    TreeKeyIntent::MoveSibling { up } => json!({ "type": "moveSibling", "direction": if up { -1 } else { 1 } }),
                    TreeKeyIntent::Activate => json!({ "type": "activate" }),
                    TreeKeyIntent::ToggleSelection => json!({ "type": "toggleSelection" }),
                    TreeKeyIntent::StartRename => json!({ "type": "startRename" }),
                });
                assert_eq!(json!(actual), *expect, "keydown {case}");
            }
            "virtualWindow" => {
                let window = tree_virtual_window(
                    case["rowCount"].as_u64().unwrap() as usize,
                    case["rowHeight"].as_f64().unwrap(),
                    case["scrollTop"].as_f64().unwrap(),
                    case["viewport"].as_f64().unwrap(),
                    case["overscan"].as_u64().unwrap() as usize,
                );
                let actual = json!({
                    "startIndex": window.start_index,
                    "endIndex": window.end_index,
                    "offsetY": window.offset_y,
                    "totalHeight": window.total_height,
                });
                assert_eq!(
                    canonicalize(&actual),
                    canonicalize(expect),
                    "virtualWindow {case}"
                );
            }
            other => panic!("unknown tree op {other}"),
        }
    }
}

// ── Duration ──

#[test]
fn duration_conformance() {
    use poodle_headless::duration::*;

    fn duration_value_from(value: &Value) -> DurationValue {
        DurationValue {
            hours: value["hours"].as_u64().unwrap_or(0) as u32,
            minutes: value["minutes"].as_u64().unwrap_or(0) as u32,
            seconds: value["seconds"].as_u64().unwrap_or(0) as u32,
        }
    }

    fn duration_value_to_json(value: DurationValue) -> Value {
        json!({
            "hours": value.hours,
            "minutes": value.minutes,
            "seconds": value.seconds,
        })
    }

    for case in vectors()["duration"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "totalSeconds" => {
                let result = duration_total_seconds(duration_value_from(&case["value"]));
                assert_eq!(result, expect.as_u64().unwrap(), "totalSeconds {case}");
            }
            "adjust" => {
                let segment = match s(case, "segment") {
                    "hours" => DurationSegment::Hours,
                    "minutes" => DurationSegment::Minutes,
                    _ => DurationSegment::Seconds,
                };
                let result = adjust_duration_segment(
                    duration_value_from(&case["value"]),
                    segment,
                    case["delta"].as_i64().unwrap(),
                    case["maxHours"].as_u64().unwrap() as u32,
                );
                assert_eq!(duration_value_to_json(result), *expect, "adjust {case}");
            }
            "set" => {
                let segment = match s(case, "segment") {
                    "hours" => DurationSegment::Hours,
                    "minutes" => DurationSegment::Minutes,
                    _ => DurationSegment::Seconds,
                };
                let result = set_duration_segment(
                    duration_value_from(&case["value"]),
                    segment,
                    case["raw"].as_i64().unwrap(),
                    case["maxHours"].as_u64().unwrap() as u32,
                );
                assert_eq!(duration_value_to_json(result), *expect, "set {case}");
            }
            "pad" => {
                let result = pad_duration_segment(case["value"].as_u64().unwrap() as u32);
                assert_eq!(result, expect.as_str().unwrap(), "pad {case}");
            }
            other => panic!("unknown duration op {other}"),
        }
    }
}

// ── Nav ──

#[test]
fn nav_conformance() {
    use poodle_headless::nav::*;

    for case in vectors()["nav"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];
        let disabled: Vec<bool> = case["disabled"]
            .as_array()
            .map(|entries| {
                entries
                    .iter()
                    .map(|entry| entry.as_bool().unwrap_or(false))
                    .collect()
            })
            .unwrap_or_default();

        match op {
            "findNext" => {
                let result = find_next_enabled_index(
                    &disabled,
                    case["startIndex"].as_u64().unwrap_or(0) as usize,
                    case["direction"].as_i64().unwrap_or(1) as i32,
                );
                let actual = result.map(|index| json!(index)).unwrap_or(Value::Null);
                assert_eq!(actual, *expect, "findNext {case}");
            }
            "firstEnabled" => {
                let result = first_enabled_index(&disabled);
                let actual = result.map(|index| json!(index)).unwrap_or(Value::Null);
                assert_eq!(actual, *expect, "firstEnabled {case}");
            }
            other => panic!("unknown nav op {other}"),
        }
    }
}

// ── TimeInput ──

#[test]
fn time_input_conformance() {
    use poodle_headless::time_input::*;

    fn opt_string(value: &Value) -> Option<String> {
        value.as_str().map(str::to_string)
    }

    fn parts_from(value: &Value) -> TimeParts {
        TimeParts {
            hour: value["hour"].as_u64().unwrap_or(0) as u32,
            minute: value["minute"].as_u64().unwrap_or(0) as u32,
            second: value["second"].as_u64().unwrap_or(0) as u32,
        }
    }

    fn parts_json(parts: TimeParts) -> Value {
        json!({
            "hour": parts.hour,
            "minute": parts.minute,
            "second": parts.second,
        })
    }

    fn draft_from(value: &Value) -> Option<TimeInputDraft> {
        if value.is_null() {
            return None;
        }

        Some(TimeInputDraft {
            hour: value["hour"].as_str().unwrap_or("").to_string(),
            minute: value["minute"].as_str().unwrap_or("").to_string(),
            second: value["second"].as_str().unwrap_or("").to_string(),
        })
    }

    fn context_from(value: &Value) -> TimeInputContext {
        TimeInputContext {
            committed: opt_string(&value["committed"]),
            default_value: opt_string(&value["defaultValue"]),
            draft: draft_from(&value["draft"]),
            min: opt_string(&value["min"]),
            max: opt_string(&value["max"]),
            step: value["step"].as_f64().unwrap_or(60.0),
            disabled: value["disabled"].as_bool().unwrap_or(false),
        }
    }

    fn context_json(context: &TimeInputContext) -> Value {
        json!({
            "committed": context.committed,
            "defaultValue": context.default_value,
            "draft": context.draft.as_ref().map(|draft| json!({
                "hour": draft.hour,
                "minute": draft.minute,
                "second": draft.second,
            })),
            "min": context.min,
            "max": context.max,
            "step": context.step,
            "disabled": context.disabled,
        })
    }

    fn segment_from(value: &str) -> TimeSegment {
        match value {
            "hour" => TimeSegment::Hour,
            "minute" => TimeSegment::Minute,
            _ => TimeSegment::Second,
        }
    }

    fn event_from(value: &Value) -> TimeInputEvent {
        match s(value, "type") {
            "DIGIT" => TimeInputEvent::Digit {
                segment: segment_from(s(value, "segment")),
                digit: value["digit"].as_u64().unwrap_or(0) as u32,
            },
            "CLEAR_SEGMENT" => TimeInputEvent::ClearSegment {
                segment: segment_from(s(value, "segment")),
            },
            "CLEAR_ALL" => TimeInputEvent::ClearAll,
            "STEP" => TimeInputEvent::Step {
                direction: value["direction"].as_i64().unwrap_or(1) as i32,
            },
            "BLUR" => TimeInputEvent::Blur,
            "ESCAPE" => TimeInputEvent::Escape,
            "REPLACE" => TimeInputEvent::Replace {
                value: opt_string(&value["value"]),
            },
            "COMMIT_TEXT" => TimeInputEvent::CommitText {
                text: s(value, "text").to_string(),
            },
            "SET_DISABLED" => TimeInputEvent::SetDisabled {
                disabled: value["disabled"].as_bool().unwrap_or(false),
            },
            "SET_CONSTRAINTS" => TimeInputEvent::SetConstraints {
                min: opt_string(&value["min"]),
                max: opt_string(&value["max"]),
                step: value["step"].as_f64().unwrap_or(60.0),
                default_value: opt_string(&value["defaultValue"]),
            },
            other => panic!("unknown timeInput event {other}"),
        }
    }

    fn effects_json(effects: &[TimeInputEffect]) -> Value {
        Value::Array(
            effects
                .iter()
                .map(|effect| match effect {
                    TimeInputEffect::EmitValueChange { value } => json!({
                        "type": "emitValueChange",
                        "value": value,
                    }),
                })
                .collect(),
        )
    }

    for case in vectors()["timeInput"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "parse" => {
                let result = parse_time(case["value"].as_str()).map(parts_json);
                let actual = result.unwrap_or(Value::Null);
                assert_eq!(actual, *expect, "parse {case}");
            }
            "format" => {
                let result = format_time(
                    parts_from(&case["parts"]),
                    case["seconds"].as_bool().unwrap_or(false),
                );
                assert_eq!(result, expect.as_str().unwrap(), "format {case}");
            }
            "secondsVisible" => {
                let result = time_seconds_visible(
                    case["committed"].as_str(),
                    case["defaultValue"].as_str(),
                    case["min"].as_str(),
                    case["max"].as_str(),
                    case["step"].as_f64().unwrap_or(60.0),
                );
                assert_eq!(result, expect.as_bool().unwrap(), "secondsVisible {case}");
            }
            "inBounds" => {
                let result = time_in_bounds(
                    parts_from(&case["parts"]),
                    case["min"].as_str(),
                    case["max"].as_str(),
                );
                assert_eq!(result, expect.as_bool().unwrap(), "inBounds {case}");
            }
            "stepAligned" => {
                let result = time_step_aligned(
                    parts_from(&case["parts"]),
                    case["min"].as_str(),
                    case["step"].as_f64().unwrap_or(60.0),
                );
                assert_eq!(result, expect.as_bool().unwrap(), "stepAligned {case}");
            }
            "step" => {
                let current = parse_time(case["current"].as_str()).map(time_to_seconds);
                let min = case["min"].as_str();
                let max = case["max"].as_str();
                let step = case["step"].as_f64().unwrap_or(60.0);
                let next = step_time_seconds(
                    current,
                    case["direction"].as_i64().unwrap_or(1) as i32,
                    min,
                    max,
                    step,
                );
                let with_seconds =
                    time_seconds_visible(case["current"].as_str(), None, min, max, step);
                let actual = next
                    .map(|seconds| json!(format_time(seconds_to_time(seconds), with_seconds)))
                    .unwrap_or(Value::Null);
                assert_eq!(actual, *expect, "step {case}");
            }
            "transition" => {
                let (context, effects) = time_input_transition(
                    context_from(&case["context"]),
                    event_from(&case["event"]),
                );
                let actual = json!({
                    "context": context_json(&context),
                    "effects": effects_json(&effects),
                    "invalid": time_input_invalid(&context),
                });
                assert_eq!(
                    canonicalize(&actual),
                    canonicalize(expect),
                    "transition {case}"
                );
            }
            other => panic!("unknown timeInput op {other}"),
        }
    }
}

// ── NumberInput ──

#[test]
fn number_input_conformance() {
    use poodle_headless::number_input::*;

    fn opt_f64(value: &Value) -> Option<f64> {
        if value.is_null() {
            None
        } else {
            value.as_f64()
        }
    }

    fn opt_string(value: &Value) -> Option<String> {
        value.as_str().map(str::to_string)
    }

    fn context_from(value: &Value) -> NumberInputContext {
        NumberInputContext {
            committed: opt_f64(&value["committed"]),
            default_value: opt_f64(&value["defaultValue"]),
            draft: opt_string(&value["draft"]),
            min: opt_f64(&value["min"]),
            max: opt_f64(&value["max"]),
            step: opt_f64(&value["step"]),
            precision: opt_f64(&value["precision"]),
            disabled: value["disabled"].as_bool().unwrap_or(false),
            read_only: value["readOnly"].as_bool().unwrap_or(false),
        }
    }

    fn context_json(context: &NumberInputContext) -> Value {
        json!({
            "committed": context.committed,
            "defaultValue": context.default_value,
            "draft": context.draft,
            "min": context.min,
            "max": context.max,
            "step": context.step,
            "precision": context.precision,
            "disabled": context.disabled,
            "readOnly": context.read_only,
        })
    }

    fn event_from(value: &Value) -> NumberInputEvent {
        match s(value, "type") {
            "RAW_EDIT" => NumberInputEvent::RawEdit {
                text: s(value, "text").to_string(),
            },
            "CLEAR" => NumberInputEvent::Clear,
            "ENTER" => NumberInputEvent::Enter,
            "BLUR" => NumberInputEvent::Blur,
            "ESCAPE" => NumberInputEvent::Escape,
            "STEP" => NumberInputEvent::Step {
                direction: value["direction"].as_i64().unwrap_or(1) as i32,
            },
            "HOME" => NumberInputEvent::Home,
            "END" => NumberInputEvent::End,
            "REPLACE" => NumberInputEvent::Replace {
                value: opt_f64(&value["value"]),
            },
            "SET_DISABLED" => NumberInputEvent::SetDisabled {
                disabled: value["disabled"].as_bool().unwrap_or(false),
            },
            "SET_READ_ONLY" => NumberInputEvent::SetReadOnly {
                read_only: value["readOnly"].as_bool().unwrap_or(false),
            },
            "SET_CONSTRAINTS" => NumberInputEvent::SetConstraints {
                min: opt_f64(&value["min"]),
                max: opt_f64(&value["max"]),
                step: opt_f64(&value["step"]),
                precision: opt_f64(&value["precision"]),
                default_value: opt_f64(&value["defaultValue"]),
            },
            other => panic!("unknown numberInput event {other}"),
        }
    }

    fn effects_json(effects: &[NumberInputEffect]) -> Value {
        Value::Array(
            effects
                .iter()
                .map(|effect| match effect {
                    NumberInputEffect::EmitDraftValueChange { draft } => json!({
                        "type": "emitDraftValueChange",
                        "draft": draft,
                    }),
                    NumberInputEffect::EmitValueChange { value } => json!({
                        "type": "emitValueChange",
                        "value": value,
                    }),
                    NumberInputEffect::EmitCommit { value } => json!({
                        "type": "emitCommit",
                        "value": value,
                    }),
                })
                .collect(),
        )
    }

    fn kind_str(kind: NumberDraftKind) -> &'static str {
        match kind {
            NumberDraftKind::Empty => "empty",
            NumberDraftKind::Incomplete => "incomplete",
            NumberDraftKind::Malformed => "malformed",
            NumberDraftKind::Complete => "complete",
        }
    }

    for case in vectors()["numberInput"].as_array().unwrap() {
        let op = s(case, "op");
        let expect = &case["expect"];

        match op {
            "classify" => {
                let classified = classify_number_draft(s(case, "value"));
                let actual = json!({
                    "kind": kind_str(classified.kind),
                    "fractionalDigits": classified.fractional_digits,
                    "value": classified.decimal.map(number_decimal_to_number),
                });
                assert_eq!(canonicalize(&actual), canonicalize(expect), "classify {case}");
            }
            "configValid" => {
                let context = NumberInputContext {
                    committed: None,
                    default_value: None,
                    draft: None,
                    min: opt_f64(&case["min"]),
                    max: opt_f64(&case["max"]),
                    step: opt_f64(&case["step"]),
                    precision: opt_f64(&case["precision"]),
                    disabled: false,
                    read_only: false,
                };
                let actual = number_input_config_valid(&context);
                assert_eq!(actual, expect.as_bool().unwrap(), "configValid {case}");
            }
            "inBounds" => {
                let actual = number_in_bounds(
                    case["value"].as_f64().unwrap(),
                    opt_f64(&case["min"]),
                    opt_f64(&case["max"]),
                );
                assert_eq!(actual, expect.as_bool().unwrap(), "inBounds {case}");
            }
            "stepAligned" => {
                let actual = number_step_aligned(
                    case["value"].as_f64().unwrap(),
                    opt_f64(&case["min"]),
                    opt_f64(&case["step"]),
                );
                assert_eq!(actual, expect.as_bool().unwrap(), "stepAligned {case}");
            }
            "draftValid" => {
                let actual = number_draft_constraint_valid(
                    s(case, "value"),
                    opt_f64(&case["min"]),
                    opt_f64(&case["max"]),
                    opt_f64(&case["step"]),
                    opt_f64(&case["precision"]),
                );
                assert_eq!(actual, expect.as_bool().unwrap(), "draftValid {case}");
            }
            "format" => {
                let actual = format_number_committed(opt_f64(&case["value"]), opt_f64(&case["precision"]));
                assert_eq!(actual, expect.as_str().unwrap(), "format {case}");
            }
            "step" => {
                let actual = step_number_value(
                    opt_f64(&case["current"]),
                    case["direction"].as_i64().unwrap_or(1) as i32,
                    opt_f64(&case["min"]),
                    opt_f64(&case["max"]),
                    opt_f64(&case["step"]),
                    opt_f64(&case["precision"]),
                );
                let actual = actual.map(|value| json!(value)).unwrap_or(Value::Null);
                assert_eq!(canonicalize(&actual), canonicalize(expect), "step {case}");
            }
            "transition" => {
                let (context, effects) = number_input_transition(
                    context_from(&case["context"]),
                    event_from(&case["event"]),
                );
                let actual = json!({
                    "context": context_json(&context),
                    "effects": effects_json(&effects),
                    "invalid": number_input_invalid(&context),
                });
                assert_eq!(
                    canonicalize(&actual),
                    canonicalize(expect),
                    "transition {case}"
                );
            }
            other => panic!("unknown numberInput op {other}"),
        }
    }
}
