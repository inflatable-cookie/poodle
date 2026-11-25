use crate::color::{
    linear_to_srgb, oklab_to_linear_srgb, rgb_to_hex, srgb_to_linear, srgb_to_oklab, Rgb,
};

use super::ThemeColor;

pub(super) const BLACK: ThemeColor = [0, 0, 0, 255];
pub(super) const WHITE: ThemeColor = [255; 4];

fn byte(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

pub(super) fn rgb(r: f64, g: f64, b: f64) -> ThemeColor {
    [byte(r), byte(g), byte(b), 255]
}

pub(super) fn alpha([r, g, b, _]: ThemeColor, a: f64) -> ThemeColor {
    [r, g, b, byte(a)]
}

pub(super) fn hex([r, g, b, a]: ThemeColor) -> String {
    rgb_to_hex(Rgb { r, g, b }, Some(unit(a)))
}

fn unit(channel: u8) -> f64 {
    f64::from(channel) / 255.0
}

pub(super) fn linear_luminance([r, g, b]: [f64; 3]) -> f64 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn luminance([r, g, b, _]: ThemeColor) -> f64 {
    linear_luminance([r, g, b].map(|c| srgb_to_linear(unit(c))))
}

pub(super) fn flare(y: f64) -> f64 {
    (y + 0.01).cbrt()
}

pub(super) fn flare_to_luminance(value: f64) -> f64 {
    value * value * value - 0.01
}

fn contrast(fg: ThemeColor, bg: ThemeColor) -> f64 {
    let a = luminance(fg);
    let b = luminance(bg);
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

pub(super) fn over(fg: ThemeColor, bg: ThemeColor) -> ThemeColor {
    let a = unit(fg[3]);
    let channel = |i: usize| unit(fg[i]) * a + unit(bg[i]) * (1.0 - a);
    rgb(channel(0), channel(1), channel(2))
}

pub(super) fn to_lab([r, g, b, _]: ThemeColor) -> (f64, f64, f64) {
    srgb_to_oklab(unit(r), unit(g), unit(b))
}

pub(super) fn vector(chroma: f64, hue: f64) -> (f64, f64) {
    let radians = hue * std::f64::consts::PI / 180.0;
    (chroma * radians.cos(), chroma * radians.sin())
}

pub(super) fn polar(a: f64, b: f64) -> (f64, f64) {
    let hue = b.atan2(a) * 180.0 / std::f64::consts::PI;
    (a.hypot(b), if hue < 0.0 { hue + 360.0 } else { hue })
}

pub(super) fn to_lch(color: ThemeColor) -> (f64, f64, f64) {
    let (lightness, a, b) = to_lab(color);
    let (chroma, hue) = polar(a, b);
    // Snap greys before fitting to avoid off-grey at a rounding boundary.
    if chroma < 1e-6 {
        (lightness, 0.0, 0.0)
    } else {
        (lightness, chroma, hue)
    }
}

fn in_gamut(channels: [f64; 3]) -> bool {
    channels.iter().all(|c| (-1e-7..=1.0 + 1e-7).contains(c))
}

pub(super) fn lch_to_linear(lightness: f64, chroma: f64, hue: f64) -> [f64; 3] {
    let lightness = lightness.clamp(0.0, 1.0);
    let (a, b) = vector(1.0, hue);
    let at = |c| oklab_to_linear_srgb(lightness, c * a, c * b);
    let chroma = highest_in(|c| in_gamut(at(c)), 0.0, chroma).unwrap_or(0.0);
    at(chroma).map(|c| c.clamp(0.0, 1.0))
}

pub(super) fn lch(lightness: f64, chroma: f64, hue: f64) -> ThemeColor {
    let [r, g, b] = lch_to_linear(lightness, chroma, hue).map(linear_to_srgb);
    rgb(r, g, b)
}

fn lab_distance(x: ThemeColor, y: ThemeColor) -> f64 {
    let (l1, a1, b1) = to_lab(x);
    let (l2, a2, b2) = to_lab(y);
    (l1 - l2).hypot(a1 - a2).hypot(b1 - b2)
}

pub(super) fn flare_distance(x: ThemeColor, y: ThemeColor) -> f64 {
    let (_, a1, b1) = to_lab(x);
    let (_, a2, b2) = to_lab(y);
    (flare(luminance(x)) - flare(luminance(y)))
        .hypot(a1 - a2)
        .hypot(b1 - b2)
}

// Both searches use the prototype's fixed 24 steps.
pub(super) fn lowest_in(test: impl Fn(f64) -> bool, mut low: f64, mut high: f64) -> Option<f64> {
    if !test(high) {
        return None;
    }
    if test(low) {
        return Some(low);
    }
    for _ in 0..24 {
        let mid = (low + high) / 2.0;
        if test(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    Some(high)
}

fn highest_in(test: impl Fn(f64) -> bool, mut low: f64, mut high: f64) -> Option<f64> {
    if !test(low) {
        return None;
    }
    if test(high) {
        return Some(high);
    }
    for _ in 0..24 {
        let mid = (low + high) / 2.0;
        if test(mid) {
            low = mid;
        } else {
            high = mid;
        }
    }
    Some(low)
}

fn min_contrast(color: ThemeColor, backgrounds: &[ThemeColor]) -> f64 {
    backgrounds
        .iter()
        .map(|&bg| contrast(color, bg))
        .fold(f64::INFINITY, f64::min)
}

// Yellows darken toward amber. The correction peaks at 105°.
fn hue_at(hue: f64, offset: f64) -> f64 {
    if offset >= 0.0 {
        return hue;
    }
    let weight = (1.0 - (hue - 105.0).abs() / 35.0).max(0.0);
    hue - 28.0 * weight * (-offset / 0.35).min(1.0)
}

fn fit_lightness(test: impl Fn(f64) -> bool, lightness: f64, direction: f64) -> Option<f64> {
    if direction > 0.0 {
        lowest_in(test, lightness, 1.0)
    } else {
        highest_in(test, 0.0, lightness)
    }
}

const FIT_MARGIN: f64 = 0.05;
const ON_STRONG: f64 = 7.0;
const ON_MIN: f64 = 4.5;
pub(super) const FILL_REACH: f64 = 0.12;

pub(super) struct Fit {
    pub color: ThemeColor,
    pub moved: bool,
}

pub(super) fn fit_away(
    start: ThemeColor,
    backgrounds: &[ThemeColor],
    target: f64,
    direction: f64,
) -> Fit {
    let target = target + FIT_MARGIN;
    if min_contrast(start, backgrounds) >= target {
        return Fit {
            color: start,
            moved: false,
        };
    }
    let (lightness, chroma, hue) = to_lch(start);
    let at = |l| lch(l, chroma, hue_at(hue, l - lightness));
    let found = fit_lightness(
        |l| min_contrast(at(l), backgrounds) >= target,
        lightness,
        direction,
    );
    let color = found.map_or(if direction > 0.0 { WHITE } else { BLACK }, at);
    Fit { color, moved: true }
}

pub(super) struct AccentFill {
    pub fill: ThemeColor,
    pub on: ThemeColor,
    pub hover: ThemeColor,
    pub active: ThemeColor,
    pub moved: bool,
    pub on_contrast: f64,
    pub strong: bool,
}

struct Exit {
    ink: ThemeColor,
    direction: f64,
    color: Option<ThemeColor>,
    cost: f64,
    reach: ThemeColor,
}

pub(super) fn fit_accent_fill(start: ThemeColor) -> AccentFill {
    let (lightness, chroma, hue) = to_lch(start);
    let dark_ink = lch(0.18, (chroma * 0.3).min(0.04), hue);
    let at = |l| lch(l, chroma, hue_at(hue, l - lightness));
    let strong = ON_STRONG + FIT_MARGIN;
    let floor = ON_MIN + FIT_MARGIN;
    let mut on = if contrast(start, WHITE) >= contrast(start, dark_ink) {
        WHITE
    } else {
        dark_ink
    };
    let mut fill = start;
    let moved = contrast(start, on) < strong;

    if moved {
        let exit = |ink, direction| {
            let found = fit_lightness(|l| contrast(at(l), ink) >= strong, lightness, direction);
            let color = found.map(at);
            let room = if direction > 0.0 {
                1.0 - lightness
            } else {
                lightness
            };
            let reach = highest_in(
                |x| lab_distance(at(lightness + direction * x), start) <= FILL_REACH,
                0.0,
                room,
            )
            .unwrap_or(0.0);
            Exit {
                ink,
                direction,
                color,
                cost: color.map_or(f64::INFINITY, |c| lab_distance(c, start)),
                reach: at(lightness + direction * reach),
            }
        };
        let lighter = exit(dark_ink, 1.0);
        let darker = exit(WHITE, -1.0);
        let closest = if lighter.cost <= darker.cost {
            &lighter
        } else {
            &darker
        };
        if let Some(color) = closest.color.filter(|_| closest.cost <= FILL_REACH) {
            on = closest.ink;
            fill = color;
        } else {
            let use_lighter = lighter.cost < darker.cost
                || (lighter.cost == darker.cost
                    && contrast(lighter.reach, lighter.ink) >= contrast(darker.reach, darker.ink));
            let chosen = if use_lighter { &lighter } else { &darker };
            on = chosen.ink;
            fill = chosen.reach;
            if contrast(fill, on) < floor {
                let found = fit_lightness(
                    |l| contrast(at(l), on) >= floor,
                    lightness,
                    chosen.direction,
                );
                fill = found.map_or(if chosen.direction > 0.0 { WHITE } else { BLACK }, at);
            }
        }
    }

    let on_contrast = contrast(fill, on);
    let keep = on_contrast.min(strong);
    let (fill_l, fill_chroma, fill_hue) = to_lch(fill);
    let shade = |offset| lch(fill_l + offset, fill_chroma, hue_at(fill_hue, offset));
    let holds = |offset| contrast(shade(offset), on) >= keep;
    let hover_step = 0.04;
    let active_step = 0.08;
    let darken = fill_l - hover_step >= 0.03 && holds(-hover_step);
    let lighter_step =
        |step| highest_in(|x| fill_l + x <= 0.985 && holds(x), 0.0, step).unwrap_or(0.0);
    let (hover, press) = if darken {
        (
            -hover_step,
            -highest_in(|x| fill_l - x >= 0.03 && holds(-x), hover_step, active_step)
                .unwrap_or(0.0),
        )
    } else {
        (lighter_step(hover_step), lighter_step(active_step))
    };

    AccentFill {
        fill,
        on,
        hover: shade(hover),
        active: shade(press),
        moved,
        on_contrast,
        strong: on_contrast >= ON_STRONG,
    }
}
