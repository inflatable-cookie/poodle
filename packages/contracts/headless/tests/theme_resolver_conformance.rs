//! The same TypeScript-generated theme vectors run in both runtimes.

use poodle_headless::color::{rgb_to_hex, Rgb};
use poodle_headless::theme_resolver::{
    resolve_theme, ThemeColorInput, ThemeReport, ThemeScheme, ThemeSettings,
};
use serde_json::{json, Value};

fn color_input(value: &Value) -> ThemeColorInput<'_> {
    match value {
        Value::Null => ThemeColorInput::Mono,
        Value::String(hex) => ThemeColorInput::Hex(hex),
        Value::Array(channels) => {
            assert_eq!(channels.len(), 3);
            let channel = |i: usize| channels[i].as_f64().expect("numeric sRGB channel");
            ThemeColorInput::Srgb([channel(0), channel(1), channel(2)])
        }
        _ => panic!("invalid vector colour {value}"),
    }
}

fn theme_settings(settings: &Value) -> ThemeSettings<'_> {
    let defaults = ThemeSettings::default();
    ThemeSettings {
        scheme: match settings.get("scheme") {
            None | Some(Value::Null) => defaults.scheme,
            Some(Value::String(s)) if s == "light" => ThemeScheme::Light,
            Some(Value::String(s)) if s == "dark" => ThemeScheme::Dark,
            scheme => panic!("invalid vector scheme {scheme:?}"),
        },
        base: color_input(&settings["base"]),
        base_blend: settings["baseBlend"]
            .as_f64()
            .unwrap_or(defaults.base_blend),
        accent: color_input(&settings["accent"]),
        accent_blend: settings["accentBlend"]
            .as_f64()
            .unwrap_or(defaults.accent_blend),
        contrast: settings["contrast"].as_f64().unwrap_or(defaults.contrast),
        even_tint: match settings.get("evenTint") {
            None | Some(Value::Null) => defaults.even_tint,
            Some(Value::Bool(even)) => *even,
            even => panic!("invalid vector tint mode {even:?}"),
        },
    }
}

fn report_json(report: ThemeReport) -> Value {
    let clamped: Vec<_> = report
        .clamped
        .into_iter()
        .map(|c| json!({ "input": c.input, "from": c.from, "to": c.to, "why": c.why }))
        .collect();
    let adjusted: Vec<_> = report
        .adjusted
        .into_iter()
        .map(|a| {
            let mut value = json!({ "role": a.role, "from": a.from, "to": a.to });
            if let Some(why) = a.why {
                value["why"] = json!(why);
            }
            value
        })
        .collect();
    json!({
        "clamped": clamped, "adjusted": adjusted, "warnings": report.warnings, "notes": report.notes,
    })
}

#[test]
fn theme_roles_and_reports_match_typescript() {
    let fixture: Value = serde_json::from_str(include_str!("../vectors/theme-resolver.json"))
        .expect("theme vectors");
    let keys = fixture["roles"].as_array().expect("vector role names");
    let vectors = fixture["vectors"].as_array().expect("theme vectors");
    assert_eq!(keys.len(), 44);
    assert!(
        vectors
            .iter()
            .filter(|v| v["id"].as_str().is_some_and(|id| id.starts_with("seeded/")))
            .count()
            >= 500
    );

    for vector in vectors {
        let id = vector["id"].as_str().expect("vector id");
        let theme = resolve_theme(theme_settings(&vector["settings"]));
        let colors = vector["colors"].as_array().expect("role colors");
        assert_eq!(theme.roles.len(), keys.len(), "{id}");
        assert_eq!(colors.len(), keys.len(), "{id}");
        for (key, expected) in keys.iter().zip(colors) {
            let key = key.as_str().expect("role name");
            let [r, g, b, a] = theme.roles[key];
            let actual = rgb_to_hex(Rgb { r, g, b }, Some(f64::from(a) / 255.0));
            assert_eq!(Some(actual.as_str()), expected.as_str(), "{id} {key}");
        }
        assert_eq!(report_json(theme.report), vector["report"], "{id}");
    }
}
