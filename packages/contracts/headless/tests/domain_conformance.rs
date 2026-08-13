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
                assert_eq!(result, expect.as_u64().unwrap() as u32, "totalSeconds {case}");
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
                assert_eq!(
                    duration_value_to_json(result),
                    *expect,
                    "adjust {case}"
                );
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
                assert_eq!(
                    duration_value_to_json(result),
                    *expect,
                    "set {case}"
                );
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
            .map(|entries| entries.iter().map(|entry| entry.as_bool().unwrap_or(false)).collect())
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
