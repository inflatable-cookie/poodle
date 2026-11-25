//! Preview theme resolution. Mirrors poodle-core's `theme-resolver.ts`.
mod color;

use std::collections::BTreeMap;

use self::color::{
    alpha, fit_accent_fill, fit_away, flare, flare_distance, flare_to_luminance, hex, lch,
    lch_to_linear, linear_luminance, lowest_in, over, polar, rgb, to_lab, to_lch, vector, BLACK,
    FILL_REACH, WHITE,
};
use crate::color::{hex_to_rgb, is_valid_hex};

/// sRGB channels followed by alpha, each stored as an 8-bit value.
pub type ThemeColor = [u8; 4];

/// The 44 renderer-neutral role paths shared with poodle-core.
pub type ThemeRoles = BTreeMap<&'static str, ThemeColor>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeScheme {
    Light,
    Dark,
}

impl ThemeScheme {
    fn name(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn spec(self) -> &'static SchemeSpec {
        match self {
            Self::Light => &LIGHT,
            Self::Dark => &DARK,
        }
    }
}

/// A tint or accent pick. Hex accepts an optional # and three or six digits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeColorInput<'a> {
    Mono,
    Hex(&'a str),
    /// Normalized sRGB channels. Finite values outside 0–1 are clipped.
    Srgb([f64; 3]),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeSettings<'a> {
    pub scheme: ThemeScheme,
    pub base: ThemeColorInput<'a>,
    pub base_blend: f64,
    pub accent: ThemeColorInput<'a>,
    pub accent_blend: f64,
    pub contrast: f64,
    pub even_tint: bool,
}

impl Default for ThemeSettings<'_> {
    fn default() -> Self {
        Self {
            scheme: ThemeScheme::Dark,
            base: ThemeColorInput::Mono,
            base_blend: DEFAULT_BASE_BLEND,
            accent: ThemeColorInput::Mono,
            accent_blend: 0.0,
            contrast: 1.0,
            even_tint: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeClamp {
    pub input: &'static str,
    pub from: String,
    pub to: String,
    pub why: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeAdjustment {
    pub role: &'static str,
    pub from: String,
    pub to: String,
    pub why: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThemeReport {
    pub clamped: Vec<ThemeClamp>,
    pub adjusted: Vec<ThemeAdjustment>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTheme {
    pub roles: ThemeRoles,
    pub report: ThemeReport,
}

struct SchemeSpec {
    direction: f64,
    limit: f64,
    ink_l: f64,
    text_l: [f64; 3],
    border_strong_l: f64,
    derived_l: f64,
    fill: [f64; 3],
    select: [f64; 2],
    border_subtle: f64,
    status_bg: f64,
    scrim: f64,
    shadow: f64,
}

const LIGHT: SchemeSpec = SchemeSpec {
    direction: -1.0,
    limit: 0.9,
    ink_l: 0.2,
    text_l: [0.24, 0.47, 0.72],
    border_strong_l: 0.64,
    derived_l: 0.52,
    fill: [0.045, 0.07, 0.1],
    select: [0.1, 0.14],
    border_subtle: 0.09,
    status_bg: 0.1,
    scrim: 0.32,
    shadow: 0.1,
};

const DARK: SchemeSpec = SchemeSpec {
    direction: 1.0,
    limit: 0.42,
    ink_l: 0.97,
    text_l: [0.95, 0.75, 0.5],
    border_strong_l: 0.56,
    derived_l: 0.79,
    fill: [0.055, 0.085, 0.12],
    select: [0.13, 0.18],
    border_subtle: 0.1,
    status_bg: 0.14,
    scrim: 0.55,
    shadow: 0.45,
};

const STEP: f64 = 0.024;
const TOP_CHROMA: f64 = 0.105;
const DEFAULT_BASE_BLEND: f64 = 0.25;
const HIERARCHY: f64 = 0.06;
const TEXT_CHROMA: [f64; 3] = [0.4, 0.6, 0.6];
const TINT_PULL: f64 = 0.5;
const EDGE_NEAR: f64 = 0.1;
const DANGER_HUE: f64 = 27.0;

// Hue in degrees, chroma scale, lightness offset toward the layers.
const SYNTAX: [(&str, [f64; 3]); 10] = [
    ("syntax.comment", [250.0, 0.3, 0.1]),
    ("syntax.keyword", [300.0, 1.0, 0.0]),
    ("syntax.string", [145.0, 0.95, 0.0]),
    ("syntax.literal", [65.0, 1.0, 0.0]),
    ("syntax.type", [195.0, 0.85, 0.0]),
    ("syntax.callable", [255.0, 1.0, 0.0]),
    ("syntax.property", [345.0, 0.9, 0.0]),
    ("syntax.operator", [0.0, 0.0, -0.06]),
    ("syntax.punctuation", [0.0, 0.0, 0.06]),
    ("syntax.invalid", [25.0, 1.1, 0.0]),
];

fn number_label(value: f64) -> String {
    if value.is_nan() {
        return "NaN".into();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .into();
    }
    if value == 0.0 {
        return "0".into();
    }
    // The report uses JavaScript's decimal/scientific notation thresholds.
    let magnitude = value.abs();
    if magnitude >= 1e21 {
        return format!("{value:e}").replace('e', "e+");
    }
    if magnitude < 1e-6 {
        return format!("{value:e}");
    }
    value.to_string()
}

fn number_input(
    value: f64,
    fallback: f64,
    min: f64,
    max: f64,
    input: &'static str,
    report: &mut ThemeReport,
) -> f64 {
    if !value.is_finite() {
        report.clamped.push(ThemeClamp {
            input,
            from: number_label(value),
            to: number_label(fallback),
            why: "not a number".into(),
        });
        return fallback;
    }
    let clamped = value.clamp(min, max);
    if clamped != value {
        report.clamped.push(ThemeClamp {
            input,
            from: number_label(value),
            to: number_label(clamped),
            why: format!("outside {min} to {max}"),
        });
    }
    clamped
}

fn color_input(
    value: ThemeColorInput<'_>,
    input: &'static str,
    report: &mut ThemeReport,
) -> Option<ThemeColor> {
    let from = match value {
        ThemeColorInput::Mono | ThemeColorInput::Hex("mono") => return None,
        ThemeColorInput::Hex(hex) => {
            let digits = hex.trim().strip_prefix('#').unwrap_or_else(|| hex.trim());
            if matches!(digits.len(), 3 | 6) {
                let prefixed = format!("#{digits}");
                if is_valid_hex(&prefixed) {
                    return hex_to_rgb(&prefixed).map(|(r, _)| [r.r, r.g, r.b, 255]);
                }
            }
            hex.into()
        }
        ThemeColorInput::Srgb([r, g, b]) if [r, g, b].iter().all(|c| c.is_finite()) => {
            return Some(rgb(r, g, b));
        }
        ThemeColorInput::Srgb(channels) => channels.map(number_label).join(","),
    };
    report.clamped.push(ThemeClamp {
        input,
        from,
        to: "mono".into(),
        why: "not a colour".into(),
    });
    None
}

fn slope_max(scheme: ThemeScheme) -> f64 {
    let lightness = flare_to_luminance(scheme.spec().limit).cbrt();
    TOP_CHROMA
        / if scheme == ThemeScheme::Dark {
            lightness
        } else {
            1.0 - lightness
        }
}

type Slope = (f64, f64);

fn tint_slope(color: Option<ThemeColor>, scheme: ThemeScheme, even: bool) -> Slope {
    let Some(color) = color else {
        return (0.0, 0.0);
    };
    let (lightness, a, b) = to_lab(color);
    if a.hypot(b) < 1e-4 {
        return (0.0, 0.0);
    }
    let from = if even { ThemeScheme::Dark } else { scheme };
    let depth = if from == ThemeScheme::Dark {
        lightness
    } else {
        1.0 - lightness
    }
    .max(1e-3);
    let ratio = slope_max(scheme) / slope_max(from);
    (a / depth * ratio, b / depth * ratio)
}

fn cap_slope((a, b): Slope, cap: f64) -> Slope {
    let length = a.hypot(b);
    if length > cap {
        (a * cap / length, b * cap / length)
    } else {
        (a, b)
    }
}

fn weighted_slope(slope: Slope, cap: f64, blend: f64) -> Slope {
    let (a, b) = cap_slope(slope, cap);
    let weight = blend * (2.0 - blend);
    (a * weight, b * weight)
}

fn layer_ramp(
    scheme: ThemeScheme,
    depth: f64,
    step: f64,
    slope: f64,
    hue: f64,
) -> ([ThemeColor; 4], f64, bool) {
    let dark = scheme == ThemeScheme::Dark;
    let (deep, dim) = if dark {
        (flare(0.0) + step, DARK.limit - 2.0 * step)
    } else {
        (flare(1.0) - step, LIGHT.limit + step)
    };
    let surface_flare = deep + (dim - deep) * depth;
    let chroma_at = |l| slope * if dark { l } else { 1.0 - l };
    let y_at = |l| linear_luminance(lch_to_linear(l, chroma_at(l), hue));
    let mut clipped = false;

    let ramp = [-1.0, 0.0, 1.0, 2.0].map(|offset| {
        let y = flare_to_luminance(surface_flare + offset * step);
        if y <= 1e-9 {
            return (BLACK, 0.0);
        }
        if y >= 1.0 {
            return (WHITE, 1.0);
        }
        // Search with unquantised luminance, then round the emitted colour.
        let lightness = lowest_in(|l| y_at(l) >= y, 0.0, 1.0).unwrap_or(1.0);
        let chroma = chroma_at(lightness);
        let color = lch(lightness, chroma, hue);
        clipped |= chroma - to_lch(color).1 > 0.01;
        (color, lightness)
    });
    (ramp.map(|(color, _)| color), chroma_at(ramp[1].1), clipped)
}

fn adjustment(
    role: &'static str,
    from: ThemeColor,
    to: ThemeColor,
    why: Option<String>,
) -> ThemeAdjustment {
    ThemeAdjustment {
        role,
        from: hex(from),
        to: hex(to),
        why,
    }
}

fn has_edge(fill: ThemeColor, layers: &[ThemeColor]) -> bool {
    layers
        .iter()
        .any(|&layer| flare_distance(fill, layer) < EDGE_NEAR)
}

/// Resolve an RGBA role set and report with no rendering or application state.
pub fn resolve_theme(input: ThemeSettings<'_>) -> ResolvedTheme {
    let mut report = ThemeReport::default();
    let scheme = input.scheme;
    let sc = scheme.spec();
    let dir = sc.direction;
    let contrast = number_input(input.contrast, 1.0, 0.5, 2.0, "contrast", &mut report);
    let base_blend = number_input(
        input.base_blend,
        DEFAULT_BASE_BLEND,
        0.0,
        1.0,
        "baseBlend",
        &mut report,
    );
    let accent_blend = number_input(
        input.accent_blend,
        0.0,
        0.0,
        1.0,
        "accentBlend",
        &mut report,
    );
    let base = color_input(input.base, "base", &mut report);
    let accent = color_input(input.accent, "accent", &mut report);
    let even = input.even_tint;

    let cap = slope_max(scheme);
    let base_slope = tint_slope(base, scheme, even);
    let base_length = base_slope.0.hypot(base_slope.1);
    if base_length > cap * 1.01 {
        report.clamped.push(ThemeClamp {
            input: "base",
            from: format!("slope {base_length:.3}"),
            to: format!("{cap:.3}"),
            why: format!("too saturated for a {} ground, muted", scheme.name()),
        });
    }
    let (ba, bb) = weighted_slope(base_slope, cap, base_blend);
    let (aa, ab) = weighted_slope(tint_slope(accent, scheme, even), cap, accent_blend);
    let sum = (ba + aa, bb + ab);
    let (slope_a, slope_b) = cap_slope(sum, cap);
    let (slope, hue) = polar(slope_a, slope_b);
    let base_tint = ba.hypot(bb);
    let accent_tint = aa.hypot(ab);
    if base_tint > 0.1 * cap
        && accent_tint > 0.1 * cap
        && sum.0.hypot(sum.1) < 0.5 * base_tint.max(accent_tint)
    {
        report
            .notes
            .push("Base and accent hues cancel, so the neutrals lose most of their tint.".into());
    }

    let (layers, chroma, clipped) = layer_ramp(
        scheme,
        base_blend.max(accent_blend),
        STEP * contrast,
        slope,
        hue,
    );
    let [canvas, surface, raised, overlay] = layers;
    let (tint_a, tint_b) = vector(chroma, hue);
    if clipped {
        report.notes.push(
            "Some layers reach the edge of sRGB, so they carry less tint than the line asks for."
                .into(),
        );
    }
    let danger_hue_distance = (((hue - DANGER_HUE) % 360.0 + 540.0) % 360.0 - 180.0).abs();
    if chroma > 0.04 && danger_hue_distance < 30.0 {
        report.notes.push(
            "The ground is close to the danger hue, so danger colours stand out less from it."
                .into(),
        );
    }

    let ink = lch(sc.ink_l, chroma * 0.6, hue);
    let [fill_rest, fill_hover, fill_active] = sc.fill.map(|opacity| alpha(ink, opacity));
    let select_neutral = alpha(ink, sc.select[0]);
    let picked = accent.unwrap_or(ink);
    let accent_line = fit_away(picked, &layers, 3.0, dir);
    let accent_text = fit_away(picked, &layers, 4.5, dir);
    if accent.is_some() && accent_line.moved {
        report
            .adjusted
            .push(adjustment("accent.line", picked, accent_line.color, None));
    }
    if accent.is_some() && accent_text.moved {
        report
            .adjusted
            .push(adjustment("accent.text", picked, accent_text.color, None));
    }

    let fill = fit_accent_fill(picked);
    if accent.is_some() && fill.moved {
        let ink_direction = if fill.on[0] < 128 {
            "lighter, so dark text"
        } else {
            "darker, so white text"
        };
        let aim = if fill.strong {
            " reaches 7:1"
        } else {
            " gets as close to 7:1 as allowed"
        };
        report.adjusted.push(adjustment(
            "accent.fill",
            picked,
            fill.fill,
            Some(format!("{ink_direction}{aim}")),
        ));
    }
    if !fill.strong {
        report.notes.push(format!(
            "Text on the accent fill reaches {:.2}:1, short of 7:1: reaching it would move the fill more than ΔE {:.0} from the picked colour.",
            fill.on_contrast, FILL_REACH * 100.0,
        ));
    }
    let edge = has_edge(fill.fill, &layers);
    if edge {
        report.notes.push("The accent fill is close to a layer, so filled controls get a hairline edge in the accent line colour.".into());
    }
    let select_accent = alpha(accent_text.color, sc.select[1]);

    let vivid = accent
        .map_or(0.0, |color| to_lch(color).1)
        .clamp(0.06, 0.15);
    let derived = |[anchor, scale, offset]: [f64; 3]| {
        let (a, b) = vector(vivid * scale, anchor);
        let (c, h) = polar(a + TINT_PULL * tint_a, b + TINT_PULL * tint_b);
        lch(sc.derived_l - dir * offset, c, h)
    };
    let status = |anchor| {
        let color = derived(anchor);
        (color, alpha(color, sc.status_bg))
    };
    let success = status([150.0, 1.0, 0.0]);
    let warning = status([80.0, 1.05, 0.0]);
    let danger = status([DANGER_HUE, 1.0, 0.0]);
    let info = status([250.0, 1.0, 0.0]);

    let tints = [
        fill_rest,
        fill_hover,
        fill_active,
        select_neutral,
        select_accent,
        success.1,
        warning.1,
        danger.1,
        info.1,
    ];
    let all_backgrounds: Vec<_> = layers
        .into_iter()
        .chain(
            layers
                .into_iter()
                .flat_map(|layer| tints.map(|tint| over(tint, layer))),
        )
        .collect();
    let lower_layers = &layers[..3];
    let text_start = |i: usize| lch(sc.text_l[i], chroma * TEXT_CHROMA[i], hue);
    let primary_on_layers = fit_away(text_start(0), lower_layers, 7.0, dir).color;
    let mut primary = fit_away(primary_on_layers, &all_backgrounds, 4.5, dir).color;
    let secondary = fit_away(text_start(1), &all_backgrounds, 4.5, dir).color;
    let secondary_l = to_lch(secondary).0;
    if dir * (to_lch(primary).0 - secondary_l) < HIERARCHY {
        let lifted_l = (secondary_l + dir * (HIERARCHY + 0.004)).clamp(0.0, 1.0);
        if dir * (lifted_l - secondary_l) < HIERARCHY {
            let end = if dir > 0.0 { "white" } else { "black" };
            report.warnings.push(format!(
                "No room for a text hierarchy: secondary text is too close to {end}."
            ));
        }
        let lifted = lch(lifted_l, chroma * TEXT_CHROMA[0], hue);
        report.adjusted.push(adjustment(
            "text.primary",
            primary,
            lifted,
            Some("hierarchy".into()),
        ));
        primary = lifted;
    }

    let status_text = |(color, background)| {
        let [a, b, c, d] = layers.map(|layer| over(background, layer));
        fit_away(
            color,
            &[canvas, surface, raised, overlay, a, b, c, d],
            4.5,
            dir,
        )
        .color
    };
    let danger_text = status_text(danger);
    // The danger fill keeps the anchor hue. Its tint and text use the half pull.
    let danger_fill = fit_accent_fill(lch(0.58, 0.19, DANGER_HUE));
    let danger_edge = has_edge(danger_fill.fill, &layers);
    if danger_edge {
        report.notes.push("The danger fill is close to a layer, so the danger button gets a hairline edge in the danger text colour.".into());
    }

    let mut roles = ThemeRoles::from([
        ("layer.canvas", canvas),
        ("layer.surface", surface),
        ("layer.raised", raised),
        ("layer.overlay", overlay),
        ("text.primary", primary),
        ("text.secondary", secondary),
        (
            "text.disabled",
            fit_away(text_start(2), lower_layers, 3.0, dir).color,
        ),
        ("fill.rest", fill_rest),
        ("fill.hover", fill_hover),
        ("fill.active", fill_active),
        ("select.neutral", select_neutral),
        ("select.accent", select_accent),
        ("border.subtle", alpha(ink, sc.border_subtle)),
        (
            "border.strong",
            fit_away(
                lch(sc.border_strong_l, chroma * 0.8, hue),
                &layers,
                3.0,
                dir,
            )
            .color,
        ),
        ("accent.fill", fill.fill),
        ("accent.fill-hover", fill.hover),
        ("accent.fill-active", fill.active),
        ("accent.on", fill.on),
        ("accent.text", accent_text.color),
        ("accent.line", accent_line.color),
        (
            "accent.edge",
            alpha(accent_line.color, u8::from(edge).into()),
        ),
        ("status.success.fg", status_text(success)),
        ("status.success.bg", success.1),
        ("status.warning.fg", status_text(warning)),
        ("status.warning.bg", warning.1),
        ("status.danger.fg", danger_text),
        ("status.danger.bg", danger.1),
        ("status.info.fg", status_text(info)),
        ("status.info.bg", info.1),
        ("status.danger.solid", danger_fill.fill),
        ("status.danger.on", danger_fill.on),
        (
            "status.danger.edge",
            alpha(danger_text, u8::from(danger_edge).into()),
        ),
        (
            "scrim",
            alpha(
                if dir > 0.0 {
                    BLACK
                } else {
                    lch(0.18, chroma, hue)
                },
                sc.scrim,
            ),
        ),
        (
            "shadow",
            alpha(
                if dir > 0.0 {
                    BLACK
                } else {
                    lch(0.22, chroma, hue)
                },
                sc.shadow,
            ),
        ),
    ]);
    roles.extend(
        SYNTAX
            .into_iter()
            .map(|(role, anchor)| (role, fit_away(derived(anchor), &layers, 4.5, dir).color)),
    );
    ResolvedTheme { roles, report }
}
