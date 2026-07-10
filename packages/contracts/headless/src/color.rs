//! Color conversion machinery. Mirror of core `color.ts` — same numeric
//! conventions (h 0–360, s/v/l 0–100, rgb 0–255, all rounded like
//! JavaScript's `Math.round`) so conformance vectors compare exactly.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsv {
    pub h: i32,
    pub s: i32,
    pub v: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsl {
    pub h: i32,
    pub s: i32,
    pub l: i32,
}

/// JS `Math.round`: half-away-from... actually half-toward-+infinity.
fn js_round(value: f64) -> i32 {
    (value + 0.5).floor() as i32
}

pub fn is_valid_hex(hex: &str) -> bool {
    let raw = match hex.strip_prefix('#') {
        Some(rest) => rest,
        None => return false,
    };

    matches!(raw.len(), 3 | 6 | 8) && raw.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn normalize_hex(hex: &str) -> String {
    let raw = hex.trim_start_matches('#').to_lowercase();

    if raw.len() == 3 {
        let chars: Vec<char> = raw.chars().collect();
        return format!("#{0}{0}{1}{1}{2}{2}", chars[0], chars[1], chars[2]);
    }

    format!("#{raw}")
}

/// Returns rgb plus alpha (0–1) when an 8-digit hex is given.
pub fn hex_to_rgb(hex: &str) -> Option<(Rgb, Option<f64>)> {
    let normalized = normalize_hex(hex);
    let raw = &normalized[1..];

    if raw.len() != 6 && raw.len() != 8 {
        return None;
    }

    let r = u8::from_str_radix(&raw[0..2], 16).ok()?;
    let g = u8::from_str_radix(&raw[2..4], 16).ok()?;
    let b = u8::from_str_radix(&raw[4..6], 16).ok()?;
    let a = if raw.len() == 8 {
        Some(f64::from(u8::from_str_radix(&raw[6..8], 16).ok()?) / 255.0)
    } else {
        None
    };

    Some((Rgb { r, g, b }, a))
}

pub fn rgb_to_hex(rgb: Rgb, alpha: Option<f64>) -> String {
    let base = format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b);

    match alpha {
        Some(a) if a < 1.0 => format!("{base}{:02x}", js_round(a.clamp(0.0, 1.0) * 255.0) as u8),
        _ => base,
    }
}

pub fn rgb_to_hsv(rgb: Rgb) -> Hsv {
    let rn = f64::from(rgb.r) / 255.0;
    let gn = f64::from(rgb.g) / 255.0;
    let bn = f64::from(rgb.b) / 255.0;
    let max = rn.max(gn).max(bn);
    let min = rn.min(gn).min(bn);
    let d = max - min;
    let v = max;
    let s = if max == 0.0 { 0.0 } else { d / max };

    if d == 0.0 {
        return Hsv { h: 0, s: 0, v: js_round(v * 100.0) };
    }

    let h = if max == rn {
        ((gn - bn) / d + if gn < bn { 6.0 } else { 0.0 }) / 6.0
    } else if max == gn {
        ((bn - rn) / d + 2.0) / 6.0
    } else {
        ((rn - gn) / d + 4.0) / 6.0
    };

    Hsv {
        h: js_round(h * 360.0),
        s: js_round(s * 100.0),
        v: js_round(v * 100.0),
    }
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Rgb {
    let sn = s / 100.0;
    let vn = v / 100.0;
    let c = vn * sn;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = vn - c;

    let (rn, gn, bn) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgb {
        r: js_round((rn + m) * 255.0) as u8,
        g: js_round((gn + m) * 255.0) as u8,
        b: js_round((bn + m) * 255.0) as u8,
    }
}

pub fn rgb_to_hsl(rgb: Rgb) -> Hsl {
    let rn = f64::from(rgb.r) / 255.0;
    let gn = f64::from(rgb.g) / 255.0;
    let bn = f64::from(rgb.b) / 255.0;
    let max = rn.max(gn).max(bn);
    let min = rn.min(gn).min(bn);
    let d = max - min;
    let l = (max + min) / 2.0;

    if d == 0.0 {
        return Hsl { h: 0, s: 0, l: js_round(l * 100.0) };
    }

    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == rn {
        ((gn - bn) / d + if gn < bn { 6.0 } else { 0.0 }) / 6.0
    } else if max == gn {
        ((bn - rn) / d + 2.0) / 6.0
    } else {
        ((rn - gn) / d + 4.0) / 6.0
    };

    Hsl {
        h: js_round(h * 360.0),
        s: js_round(s * 100.0),
        l: js_round(l * 100.0),
    }
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Rgb {
    let sn = s / 100.0;
    let ln = l / 100.0;

    if sn == 0.0 {
        let v = js_round(ln * 255.0) as u8;
        return Rgb { r: v, g: v, b: v };
    }

    fn hue2rgb(p: f64, q: f64, t: f64) -> f64 {
        let tt = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };

        if tt < 1.0 / 6.0 {
            p + (q - p) * 6.0 * tt
        } else if tt < 1.0 / 2.0 {
            q
        } else if tt < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - tt) * 6.0
        } else {
            p
        }
    }

    let q = if ln < 0.5 { ln * (1.0 + sn) } else { ln + sn - ln * sn };
    let p = 2.0 * ln - q;
    let hn = h / 360.0;

    Rgb {
        r: js_round(hue2rgb(p, q, hn + 1.0 / 3.0) * 255.0) as u8,
        g: js_round(hue2rgb(p, q, hn) * 255.0) as u8,
        b: js_round(hue2rgb(p, q, hn - 1.0 / 3.0) * 255.0) as u8,
    }
}

pub fn hex_to_hsv(hex: &str) -> Option<Hsv> {
    hex_to_rgb(hex).map(|(rgb, _)| rgb_to_hsv(rgb))
}

pub fn hsv_to_hex(h: f64, s: f64, v: f64, alpha: Option<f64>) -> String {
    rgb_to_hex(hsv_to_rgb(h, s, v), alpha)
}

pub fn hex_to_hsl(hex: &str) -> Option<Hsl> {
    hex_to_rgb(hex).map(|(rgb, _)| rgb_to_hsl(rgb))
}

pub fn hsl_to_hex(h: f64, s: f64, l: f64, alpha: Option<f64>) -> String {
    rgb_to_hex(hsl_to_rgb(h, s, l), alpha)
}

pub fn hsv_to_hsl(h: f64, s: f64, v: f64) -> Hsl {
    rgb_to_hsl(hsv_to_rgb(h, s, v))
}

pub fn hsl_to_hsv(h: f64, s: f64, l: f64) -> Hsv {
    rgb_to_hsv(hsl_to_rgb(h, s, l))
}
