//! Domain-math conformance: date and color vectors generated from the
//! TypeScript core (`vectors/domain.json`); the Rust mirror must reproduce
//! every expectation exactly.

use serde_json::{json, Value};

use poodle_headless::color::*;
use poodle_headless::date::*;

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
                let result = format_iso_date(add_months(date, case["amount"].as_i64().unwrap() as i32));
                assert_eq!(result, expect.as_str().unwrap(), "addMonths {case}");
            }
            "parse" => {
                let result = parse_iso_date(s(case, "iso")).map(format_iso_date);
                let expected = expect.as_str().map(str::to_string);
                assert_eq!(result, expected, "parse {case}");
            }
            "compare" => {
                let result = compare_iso_date(s(case, "left"), s(case, "right")).unwrap();
                assert_eq!(i64::from(result), expect.as_i64().unwrap(), "compare {case}");
            }
            "monthAnchor" => {
                assert_eq!(month_anchor_iso(s(case, "iso")).unwrap(), expect.as_str().unwrap(), "monthAnchor {case}");
            }
            "normalizeRange" => {
                let (start, end) = normalize_date_range(Some(s(case, "start")), Some(s(case, "end")));
                assert_eq!(start.as_deref(), expect["start"].as_str(), "normalizeRange start {case}");
                assert_eq!(end.as_deref(), expect["end"].as_str(), "normalizeRange end {case}");
            }
            "withinRange" => {
                let result = is_iso_date_within_range(s(case, "iso"), Some(s(case, "start")), Some(s(case, "end")));
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
                assert_eq!(i64::from(result), expect.as_i64().unwrap(), "weekBoundaryDelta {case}");
            }
            "daysBetween" => {
                let result = days_between(s(case, "start"), s(case, "end")).unwrap();
                assert_eq!(result, expect.as_i64().unwrap(), "daysBetween {case}");
            }
            "calendarWeeks" => {
                // today far outside every vector month so isToday never fires
                let weeks = build_calendar_weeks(s(case, "visibleMonth"), week_start(case), "1900-01-01");
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
            "normalizeHex" => assert_eq!(normalize_hex(s(case, "hex")), expect.as_str().unwrap(), "{case}"),
            "isValidHex" => assert_eq!(is_valid_hex(s(case, "hex")), expect.as_bool().unwrap(), "{case}"),
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
            "rgbToHex" => assert_eq!(rgb_to_hex(Rgb { r: ri(), g: gi(), b: bi() }, None), expect.as_str().unwrap(), "{case}"),
            "rgbToHexAlpha" => {
                let a = case["a"].as_f64().unwrap();
                assert_eq!(rgb_to_hex(Rgb { r: ri(), g: gi(), b: bi() }, Some(a)), expect.as_str().unwrap(), "{case}");
            }
            "rgbToHsv" => {
                let hsv = rgb_to_hsv(Rgb { r: ri(), g: gi(), b: bi() });
                assert_eq!(json!({ "h": hsv.h, "s": hsv.s, "v": hsv.v }), *expect, "rgbToHsv {case}");
            }
            "rgbToHsl" => {
                let hsl = rgb_to_hsl(Rgb { r: ri(), g: gi(), b: bi() });
                assert_eq!(json!({ "h": hsl.h, "s": hsl.s, "l": hsl.l }), *expect, "rgbToHsl {case}");
            }
            "hsvToRgb" => {
                let v = case["v"].as_f64().unwrap();
                assert_eq!(rgb_json(hsv_to_rgb(hf(), sf(), v)), *expect, "hsvToRgb {case}");
            }
            "hsvToHex" => {
                let v = case["v"].as_f64().unwrap();
                assert_eq!(hsv_to_hex(hf(), sf(), v, None), expect.as_str().unwrap(), "hsvToHex {case}");
            }
            "hslToRgb" => {
                let l = case["l"].as_f64().unwrap();
                assert_eq!(rgb_json(hsl_to_rgb(hf(), sf(), l)), *expect, "hslToRgb {case}");
            }
            "hexToHsv" => {
                let hsv = hex_to_hsv(s(case, "hex")).unwrap();
                assert_eq!(json!({ "h": hsv.h, "s": hsv.s, "v": hsv.v }), *expect, "hexToHsv {case}");
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
