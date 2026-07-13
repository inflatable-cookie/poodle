//! Neutral-contrast conformance: expectations computed independently with
//! the reference OKLab math (matching browser relative-color rendering,
//! cross-checked against live Playwright computed values).

use poodle_headless::color::*;

fn assert_close(actual: (f64, f64, f64, f64), expected: (f64, f64, f64, f64), label: &str) {
    let pairs = [
        (actual.0, expected.0),
        (actual.1, expected.1),
        (actual.2, expected.2),
        (actual.3, expected.3),
    ];
    for (i, (a, e)) in pairs.iter().enumerate() {
        assert!((a - e).abs() < 0.002, "{label} channel {i}: {a} vs {e}");
    }
}

fn c(v: u8) -> f64 {
    v as f64 / 255.0
}

#[test]
fn dark_anchor_lightness_matches_browser() {
    // Browser computed oklch L of #0e1012 (canvas): 0.17169
    let l = oklab_lightness(c(0x0e), c(0x10), c(0x12));
    assert!((l - 0.171691).abs() < 1e-5, "anchor L {l}");
}

#[test]
fn identity_at_full_contrast() {
    let anchor = oklab_lightness(c(0x0e), c(0x10), c(0x12));
    let out = apply_neutral_contrast(c(0x15), c(0x18), c(0x1b), 1.0, anchor, 1.0);
    assert_close(out, (c(21), c(24), c(27), 1.0), "surface k=1");
}

#[test]
fn dark_backgrounds_at_default_contrast() {
    let anchor = oklab_lightness(c(0x0e), c(0x10), c(0x12));
    // Reference values from the independent Python computation
    let surface = apply_neutral_contrast(c(0x15), c(0x18), c(0x1b), 1.0, anchor, 0.5);
    assert_close(
        surface,
        (17.073 / 255.0, 20.009 / 255.0, 22.943 / 255.0, 1.0),
        "surface k=0.5",
    );
    let elevated = apply_neutral_contrast(c(0x20), c(0x25), c(0x2a), 1.0, anchor, 0.5);
    assert_close(
        elevated,
        (21.598 / 255.0, 26.383 / 255.0, 31.152 / 255.0, 1.0),
        "elevated k=0.5",
    );
}

#[test]
fn light_theme_values() {
    let anchor = oklab_lightness(c(0xe7), c(0xee), c(0xf5));
    let surface = apply_neutral_contrast(c(0xdb), c(0xe5), c(0xef), 1.0, anchor, 0.5);
    assert_close(
        surface,
        (223.678 / 255.0, 233.716 / 255.0, 243.755 / 255.0, 1.0),
        "light surface k=0.5",
    );
    let border = apply_neutral_contrast(c(0x75), c(0x86), c(0x9b), 1.0, anchor, 0.5);
    assert_close(
        border,
        (167.186 / 255.0, 185.203 / 255.0, 207.513 / 255.0, 1.0),
        "light border k=0.5",
    );
}

#[test]
fn translucent_borders_scale_alpha_with_floor() {
    let out = apply_neutral_contrast(227.0 / 255.0, 232.0 / 255.0, 238.0 / 255.0, 0.22, 0.17, 0.5);
    assert_close(out, (227.0 / 255.0, 232.0 / 255.0, 238.0 / 255.0, 0.11), "alpha x0.5");
    // floor at 0.4
    let floored = apply_neutral_contrast(227.0 / 255.0, 232.0 / 255.0, 238.0 / 255.0, 0.22, 0.17, 0.1);
    assert!((floored.3 - 0.22 * 0.4).abs() < 1e-9, "alpha floor");
}

#[test]
fn token_classification() {
    assert!(is_contrast_scaled_token("color.background.surface"));
    assert!(is_contrast_scaled_token("color.border.default"));
    assert!(!is_contrast_scaled_token("color.background.overlay"));
    assert!(!is_contrast_scaled_token("color.accent.base"));
    assert!(!is_contrast_scaled_token("color.text.primary"));
}
