//! ColorPicker — swatch trigger + picker surface.
//!
//! Contract: `docs/contracts/components/color-picker.md`
//! Ported from: `packages/jetstream/components/src/color_picker/` (mod +
//! parts + parts2, merged).
//!
//! Anatomy: trigger swatch (the spec's ACTUAL current color) + optional
//! inline hex input, and — when open — the surface popover containing the 2D
//! saturation/value gradient pad with thumb, hue slider, optional alpha
//! slider, mode toggle (SegmentedControl), channel inputs (hex/RGB/HSL), and
//! optional preset swatch grid.
//!
//! The gradient/swatch/thumb COLORS are computed from the picker's value via
//! `crate::color` math — the one legitimate non-token color source (per
//! contract). Values stay sRGB; backends convert at their edge (the old
//! Jetstream tier fed raw bytes into its linear pipeline — the established
//! custom-hex divergence, fixed here; the parity suite pins fixed-point hex
//! values where both agree).
//!
//! Only the preset swatches raise `on_change`: the gradient area and the
//! channel sliders need drag-with-position, which the drag events do not
//! carry usefully yet, so a press there stays inert rather than reporting a
//! colour it cannot compute.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow,
    LayoutSizing, Node, NodePosition, NodeRole, ShadowLayer,
};
use poodle_specs::{
    ColorInputMode, ColorPickerSpec, NumberInputSpec, SegmentedControlOption, SegmentedControlSpec,
};

use crate::color::{
    hex_to_rgb255, hsv_to_hsl, pure_hue_color, rgb255_to_color, rgb_to_hsv, with_alpha, Hsv,
    Rgb255, BLACK, TRANSPARENT, WHITE,
};
use crate::number_input::number_input;
use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::segmented_control::segmented_control;

/// Default fallback color when the spec value is missing/malformed (#6366f1).
const FALLBACK_RGB: Rgb255 = Rgb255 {
    r: 99,
    g: 102,
    b: 241,
    a: 1.0,
};

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct ColorPickerHandlers {
    /// Fires when the trigger swatch is pressed.
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires with the chosen preset's hex.
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

fn all_radius(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn inset_overlay() -> Node {
    let mut o = Node::container();
    // Explicit Row (see switch.rs).
    o.style.descriptor.layout.direction = LayoutDirection::Row;
    o.position = NodePosition::Absolute {
        top: Some(0.0),
        left: Some(0.0),
        right: Some(0.0),
        bottom: Some(0.0),
    };
    o.style.fill_width = true;
    o.style.fill_height = true;
    o
}

pub fn color_picker(
    spec: &ColorPickerSpec,
    theme: &dyn ThemeProvider,
    handlers: ColorPickerHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let trigger_size = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    // The preview theme's density is the active visual axis; the old GPUI
    // tier resolves the control padding from that theme rather than the
    // standalone spec density.
    let pad_x = theme.resolve_space("space.control.x");

    // ── Resolved chrome tokens ────────────────────────────────────
    let border = theme.resolve_color(spec.border_token());
    let trigger_radius = theme.resolve_radius(spec.trigger_radius_token());
    let surface_radius = theme.resolve_radius(spec.surface_radius_token());
    let radius_control = theme.resolve_radius("radius.control");
    let surface_bg = theme.resolve_color("color.background.surface");
    let elevated_bg = theme.resolve_color(spec.overlay_fill_token());
    let border_subtle = theme.resolve_color("color.border.subtle");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    // Stack gap (controls row → surface) and surface internal gaps.
    let stack_gap = theme.resolve_space("space.stack.sm");
    let surface_gap = theme.resolve_space("space.stack.md");

    // Trigger border is 62% opacity of border-default per contract.
    let trigger_border = with_alpha(border, border.3 * 0.62);

    // ── Current color (the legitimate non-token color source) ─────
    let current = spec.current_value().unwrap_or("#6366f1").to_string();
    let rgb = hex_to_rgb255(&current).unwrap_or(FALLBACK_RGB);
    let hsv: Hsv = rgb_to_hsv(rgb);
    let alpha = if spec.show_alpha { rgb.a } else { 1.0 };
    let current_color = rgb255_to_color(rgb, alpha);

    // ── Trigger swatch — fills with the ACTUAL current color ──────
    let mut trigger = Node::container();
    trigger.id = Some("color-picker-trigger".to_string());
    {
        let s = &mut trigger.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(trigger_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(trigger_size);
        s.descriptor.background = Some(current_color);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = trigger_border;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    all_radius(&mut trigger, trigger_radius);
    trigger.interaction.focusable = true;
    if let (false, Some(handler)) = (spec.is_disabled, &handlers.on_toggle) {
        let handler = Arc::clone(handler);
        trigger.interaction.on_activate = Some(Arc::new(move || handler()));
    }
    // ── Controls row: trigger + optional inline hex input ─────────
    let mut controls_row = Node::container();
    controls_row.style.descriptor.layout.direction = LayoutDirection::Row;
    controls_row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    controls_row.style.descriptor.layout.spacing.gap = stack_gap;
    let mut controls_row = controls_row.child(trigger);

    if spec.show_input {
        let input_display = spec.current_value().unwrap_or("#6366f1");
        let input_color = if spec.current_value().is_some() {
            text_primary
        } else {
            text_secondary
        };
        // Contract §8 inline input: 6.5rem wide, control height, code font.
        let mut field = Node::container();
        {
            let s = &mut field.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(6.5));
            s.descriptor.layout.height = LayoutSizing::Fixed(trigger_size);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
            s.descriptor.background = Some(surface_bg);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        all_radius(&mut field, trigger_radius);
        // Contract §8 `.color-picker__input`: code-family hex value.
        let mut value = Node::text(input_display);
        value.style.descriptor.text_color = Some(input_color);
        value.style.text_size = Some(font_size);
        value.style.font_family = Some(FontFamily::Mono);
        controls_row = controls_row.child(field.child(value));
    }

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = stack_gap;
    let mut root = root.child(controls_row);

    // ── Surface popover ───────────────────────────────────────────
    if spec.current_open() && !spec.is_disabled {
        // Contract: width 24rem, padding 0.75rem, gap 0.625rem (stack.md ≈),
        // border-subtle, radius-surface, elevated bg.
        let surface_pad = rem_to_px(0.75);
        let mut surface = Node::container();
        // Contract: the open surface popover is a `dialog`.
        surface.a11y.role = Some(NodeRole::Dialog);
        surface.id = Some("color-picker-surface".to_string());
        {
            let s = &mut surface.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(24.0));
            s.descriptor.background = Some(elevated_bg);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border_subtle;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = surface_pad;
            pad.right = surface_pad;
            pad.top = surface_pad;
            pad.bottom = surface_pad;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = surface_gap;
        }
        all_radius(&mut surface, surface_radius);

        // ── Picker area: gradient pad (left) + controls (right) ───
        let gradient_pad = build_gradient_pad(hsv, current_color, radius_control);
        let controls_panel =
            build_controls_panel(spec, theme, &current, rgb, hsv, alpha, current_color);

        let mut picker_area = Node::container();
        picker_area.style.descriptor.layout.direction = LayoutDirection::Row;
        // GPUI uses the inline-md token for the pad↔controls gap (12px on
        // this axis), not the surface stack approximation.
        picker_area.style.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.md");
        picker_area.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        let mut surface = surface.child(picker_area.child(gradient_pad).child(controls_panel));

        // ── Swatch grid (opt-in) ──────────────────────────────────
        if !spec.swatches.is_empty() {
            surface = surface.child(build_swatch_grid(
                &spec.swatches,
                &current,
                text_primary,
                border_subtle,
                handlers.on_change.as_ref(),
            ));
        }

        root = root.child(surface);
    }

    // ── Disabled state ────────────────────────────────────────────
    if spec.is_disabled {
        root.style.descriptor.opacity = disabled_opacity;
        root.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

/// 2D saturation/value gradient pad. Base = pure hue `hsl(h,100%,50%)`. Two
/// absolutely positioned overlay children carry the CSS `::before`/`::after`
/// gradients (white→transparent left→right, transparent→black top→bottom). A
/// thumb ring sits at the current S/V.
fn build_gradient_pad(hsv: Hsv, current_color: ColorValue, radius_control: f32) -> Node {
    let pad_size = rem_to_px(10.0);
    let thumb_d = rem_to_px(0.875);
    let pure_hue = pure_hue_color(hsv.h);

    let white_t = with_alpha(WHITE, 0.0);
    let black_t = with_alpha(BLACK, 0.0);

    // Thumb position within the pad (s → x, v inverted → y), centered.
    let thumb_x = pad_size * (hsv.s / 100.0) - thumb_d / 2.0;
    let thumb_y = pad_size * (1.0 - hsv.v / 100.0) - thumb_d / 2.0;

    // White → transparent, left to right (CSS `to right` == 90deg).
    let mut before = inset_overlay();
    before.style.gradient = Some((90.0, vec![(WHITE, 0.0), (white_t, 1.0)]));

    // Transparent → black, top to bottom (CSS `to bottom` == 0deg here).
    let mut after = inset_overlay();
    // GPUI's 180° axis is top-to-bottom; 0° would invert the value ramp.
    after.style.gradient = Some((180.0, vec![(black_t, 0.0), (BLACK, 1.0)]));

    // Thumb ring at current S/V.
    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_d);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_d);
        s.descriptor.border.width = 2.0;
        s.descriptor.border.color = WHITE;
        s.descriptor.background = Some(current_color);
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: 1.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.3),
            inset: false,
        }];
    }
    all_radius(&mut thumb, thumb_d / 2.0);
    thumb.position = NodePosition::Absolute {
        top: Some(thumb_y),
        left: Some(thumb_x),
        right: None,
        bottom: None,
    };

    let mut pad = Node::container();
    pad.id = Some("color-picker-gradient".to_string());
    {
        let s = &mut pad.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_shrink_zero = true;
        s.descriptor.layout.width = LayoutSizing::Fixed(pad_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(pad_size);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.background = Some(pure_hue);
    }
    all_radius(&mut pad, radius_control);
    pad.position = NodePosition::Relative;
    pad.child(before).child(after).child(thumb)
}

/// Controls panel (right of the gradient): hue slider, optional alpha slider,
/// mode toggle (SegmentedControl), channel inputs.
fn build_controls_panel(
    spec: &ColorPickerSpec,
    theme: &dyn ThemeProvider,
    current: &str,
    rgb: Rgb255,
    hsv: Hsv,
    alpha: f32,
    current_color: ColorValue,
) -> Node {
    let surface_bg = theme.resolve_color("color.background.surface");

    let mut panel = Node::container();
    {
        let s = &mut panel.style;
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.min_width = Some(0.0);
    }
    let mut panel = panel;

    // Hue slider — full rainbow track + thumb at current hue.
    panel = panel.child(build_hue_strip(theme, hsv.h));

    // Alpha slider (opt-in) — checkerboard stand-in + color overlay + thumb.
    if spec.show_alpha {
        panel = panel.child(build_alpha_strip(theme, alpha, current_color, surface_bg));
    }

    // Mode toggle (SegmentedControl: Hex / RGB / HSL).
    let mode_value = match spec.default_mode {
        ColorInputMode::Hex => "hex",
        ColorInputMode::Rgb => "rgb",
        ColorInputMode::Hsl => "hsl",
    };
    let mode_spec = SegmentedControlSpec::new(vec![
        SegmentedControlOption::new("hex", "Hex"),
        SegmentedControlOption::new("rgb", "RGB"),
        SegmentedControlOption::new("hsl", "HSL"),
    ])
    .with_default_value(mode_value)
    .with_size(spec.size)
    .with_density(spec.density);
    // The old GPUI SegmentedControl uses `.flex_1()` for equal-width mode
    // buttons. The shared renderer's generic Grow sizing preserves its
    // content-sized behavior for the standalone specimen, so apply the
    // zero-basis form at this fractional ColorPicker call site.
    let mut mode = segmented_control(&mode_spec, theme, None);
    for segment in &mut mode.children {
        segment.style.flex_grow = Some(1.0);
        segment.style.flex_basis = Some(0.0);
        segment.style.min_width = Some(0.0);
    }
    panel = panel.child(mode);

    // Channel inputs for the current mode.
    panel.child(build_channel_inputs(spec, theme, current, rgb, hsv, alpha))
}

/// Hue slider with a full rainbow track. A gradient packs only its first +
/// last stop downstream, so the multi-stop rainbow renders as one 2-stop
/// segment per adjacent pair, each flex-grown to its stop spacing — visually
/// identical to a single 7-stop gradient.
fn build_hue_strip(theme: &dyn ThemeProvider, hue: f32) -> Node {
    let track_h = rem_to_px(0.375);
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.default");

    // Seven CSS rainbow stops (#f00 0%, #ff0 17%, #0f0 33%, #0ff 50%,
    // #00f 67%, #f0f 83%, #f00 100%).
    let stops: Vec<(ColorValue, f32)> = vec![
        (ColorValue(1.0, 0.0, 0.0, 1.0), 0.0),
        (ColorValue(1.0, 1.0, 0.0, 1.0), 0.17),
        (ColorValue(0.0, 1.0, 0.0, 1.0), 0.33),
        (ColorValue(0.0, 1.0, 1.0, 1.0), 0.50),
        (ColorValue(0.0, 0.0, 1.0, 1.0), 0.67),
        (ColorValue(1.0, 0.0, 1.0, 1.0), 0.83),
        (ColorValue(1.0, 0.0, 0.0, 1.0), 1.0),
    ];

    let mut track = Node::container();
    track.id = Some("color-picker-hue".to_string());
    {
        let s = &mut track.style;
        s.fill_width = true;
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Row;
    }
    all_radius(&mut track, track_h / 2.0);
    let mut track = track;
    for pair in stops.windows(2) {
        let (c0, p0) = pair[0];
        let (c1, p1) = pair[1];
        let mut seg = Node::container();
        {
            let s = &mut seg.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
            s.gradient = Some((90.0, vec![(c0, 0.0), (c1, 1.0)]));
            s.flex_grow = Some((p1 - p0).max(0.0001));
        }
        track = track.child(seg);
    }

    slider_wrap(
        "Hue",
        track,
        thumb_d,
        track_h,
        (hue / 360.0).clamp(0.0, 1.0),
        elevated,
        border,
    )
}

/// Alpha slider. CSS layers a transparent→color gradient over a checkerboard;
/// no repeating-conic-gradient channel exists, so the checkerboard is a
/// neutral surface base with a transparent→color overlay.
fn build_alpha_strip(
    theme: &dyn ThemeProvider,
    alpha: f32,
    color: ColorValue,
    surface_bg: ColorValue,
) -> Node {
    let track_h = rem_to_px(0.375);
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.default");

    let opaque = with_alpha(color, 1.0);
    let transparent = with_alpha(color, 0.0);

    let mut overlay = inset_overlay();
    overlay.style.gradient = Some((90.0, vec![(transparent, 0.0), (opaque, 1.0)]));

    let mut track = Node::container();
    track.id = Some("color-picker-alpha".to_string());
    {
        let s = &mut track.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
        s.descriptor.layout.height = LayoutSizing::Fixed(track_h);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        // Neutral checkerboard stand-in.
        s.descriptor.background = Some(surface_bg);
    }
    all_radius(&mut track, track_h / 2.0);
    track.position = NodePosition::Relative;

    slider_wrap(
        "Opacity",
        track.child(overlay),
        thumb_d,
        track_h,
        alpha.clamp(0.0, 1.0),
        elevated,
        border,
    )
}

/// Wrap a slider track in a relative container with a thumb at `progress`.
fn slider_wrap(
    // What this channel controls. A slider announced as "slider, 40%" says
    // nothing about which quantity moved, and this picker has several.
    channel: &str,
    track: Node,
    thumb_d: f32,
    track_h: f32,
    progress: f32,
    thumb_fill: ColorValue,
    thumb_border: ColorValue,
) -> Node {
    // Center the thumb vertically on the track; horizontal position is a
    // fraction of (track width − thumb diameter). Track is full-width; the
    // wrap reserves its measured width via layout.
    let thumb_top = -(thumb_d - track_h) / 2.0;
    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_d);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_d);
        s.descriptor.background = Some(thumb_fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = thumb_border;
    }
    all_radius(&mut thumb, thumb_d / 2.0);
    thumb.position = NodePosition::Absolute {
        top: Some(thumb_top),
        // The fixed 24rem surface leaves 11.625rem for the controls track
        // after padding, the 10rem gradient pad, and the inline-md gap.
        // GPUI's `left(relative(progress))` uses that actual track width;
        // keeping the same width here avoids pinning the thumb to the pad
        // size.
        left: Some(progress * rem_to_px(11.625) - thumb_d / 2.0),
        right: None,
        bottom: None,
    };

    // Contract: each channel track is a `slider` reporting its own value, not
    // a decorative bar with a dot on it.
    let mut wrap = Node::container();
    wrap.a11y.role = Some(NodeRole::Slider);
    wrap.a11y.label = Some(channel.to_string());
    {
        let s = &mut wrap.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
    }
    wrap.position = NodePosition::Relative;
    wrap.child(track).child(thumb)
}

/// Channel inputs row for the current mode (hex / RGB / HSL). Each is a
/// stacked field: control over an uppercase label, matching Svelte. The
/// optional alpha channel appends when `show_alpha`.
fn build_channel_inputs(
    spec: &ColorPickerSpec,
    theme: &dyn ThemeProvider,
    current: &str,
    rgb: Rgb255,
    hsv: Hsv,
    alpha: f32,
) -> Node {
    let surface_bg = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color(spec.border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let radius_control = theme.resolve_radius("radius.control");
    // The old GPUI channel captions use the active typography label token
    // (13px on the eclipse axis), not the compact 0.625rem CSS annotation.
    let label_size = theme.resolve_space("typography.label.size");

    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
    row.style.fill_width = true;
    let mut row = row;

    let labelled = |child: Node, label: &str| -> Node {
        let mut field = Node::container();
        {
            let s = &mut field.style;
            s.flex_grow = Some(1.0);
            s.flex_basis = Some(0.0);
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.125);
            s.min_width = Some(0.0);
        }
        let mut caption = Node::text(label.to_uppercase());
        caption.style.text_size = Some(label_size);
        caption.style.descriptor.text_color = Some(text_secondary);
        field.child(child).child(caption)
    };

    let number = |id: &str, value: f64, min: f64, max: f64, aria: &str| -> Node {
        let n = NumberInputSpec::new(value)
            .with_min(min)
            .with_max(max)
            .with_step(1.0)
            .with_aria_label(aria)
            .with_size(spec.size)
            .with_density(spec.density);
        let mut input = number_input(
            &n,
            theme,
            crate::number_input::NumberInputHandlers::default(),
        );
        input.id = Some(id.to_string());
        input
    };
    let alpha_field = |row: Node| -> Node {
        row.child(labelled(
            number(
                "color-picker-a",
                (alpha * 100.0).round() as f64,
                0.0,
                100.0,
                "Alpha",
            ),
            "A",
        ))
    };

    match spec.default_mode {
        ColorInputMode::Hex => {
            // Hex: a code-font text field (height 2rem) showing the value.
            let mut hex_field = Node::container();
            hex_field.id = Some("color-picker-hex-input".to_string());
            {
                let s = &mut hex_field.style;
                s.fill_width = true;
                s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(2.0));
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.375);
                pad.right = rem_to_px(0.375);
                s.descriptor.background = Some(surface_bg);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = border;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            }
            all_radius(&mut hex_field, radius_control);
            // Contract §8 `.color-picker__text-input`: code-family hex value.
            let mut value = Node::text(current);
            value.style.text_size = Some(rem_to_px(0.75));
            value.style.descriptor.text_color = Some(text_primary);
            value.style.font_family = Some(FontFamily::Mono);
            row = row.child(labelled(hex_field.child(value), "Hex"));
            if spec.show_alpha {
                row = alpha_field(row);
            }
        }
        ColorInputMode::Rgb => {
            row = row.child(labelled(
                number("color-picker-r", rgb.r as f64, 0.0, 255.0, "Red"),
                "R",
            ));
            row = row.child(labelled(
                number("color-picker-g", rgb.g as f64, 0.0, 255.0, "Green"),
                "G",
            ));
            row = row.child(labelled(
                number("color-picker-b", rgb.b as f64, 0.0, 255.0, "Blue"),
                "B",
            ));
            if spec.show_alpha {
                row = alpha_field(row);
            }
        }
        ColorInputMode::Hsl => {
            let hsl = hsv_to_hsl(hsv.h, hsv.s, hsv.v);
            row = row.child(labelled(
                number("color-picker-h", hsl.h as f64, 0.0, 360.0, "Hue"),
                "H",
            ));
            row = row.child(labelled(
                number("color-picker-s", hsl.s as f64, 0.0, 100.0, "Saturation"),
                "S",
            ));
            row = row.child(labelled(
                number("color-picker-l", hsl.l as f64, 0.0, 100.0, "Lightness"),
                "L",
            ));
            if spec.show_alpha {
                row = alpha_field(row);
            }
        }
    }

    row
}

/// Preset swatch grid. Each swatch is a 1.25rem square at its hex color; the
/// active swatch (matching the current value) gets a text-primary border, the
/// rest a transparent border. Top divider = border-subtle@42%.
fn build_swatch_grid(
    swatches: &[String],
    current: &str,
    text_primary: ColorValue,
    border_subtle: ColorValue,
    on_change: Option<&Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let swatch_size = rem_to_px(1.25);
    let swatch_radius = rem_to_px(0.1875);
    let gap = rem_to_px(0.25);
    let divider = with_alpha(border_subtle, border_subtle.3 * 0.42);

    // Contract: the preset swatches are a `listbox` of `option`s.
    let mut grid = Node::container();
    grid.a11y.role = Some(NodeRole::ListBox);
    grid.id = Some("color-picker-swatches".to_string());
    {
        let s = &mut grid.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.spacing.padding.top = gap;
        s.border_top_width = Some(1.0);
        s.border_color_top = Some(divider);
    }
    let mut grid = grid;

    for (idx, hex) in swatches.iter().enumerate() {
        let rgb = hex_to_rgb255(hex).unwrap_or(Rgb255 {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        });
        let swatch_color = rgb255_to_color(rgb, 1.0);
        let is_active = hex.eq_ignore_ascii_case(current);

        let mut swatch = Node::container();
        swatch.a11y.role = Some(NodeRole::ListBoxOption);
        swatch.a11y.label = Some(hex.clone());
        swatch.id = Some(format!("color-picker-swatch-{idx}"));
        {
            let s = &mut swatch.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(swatch_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(swatch_size);
            s.descriptor.border.width = 2.0;
            s.descriptor.border.color = if is_active { text_primary } else { TRANSPARENT };
            s.descriptor.background = Some(swatch_color);
            s.descriptor.cursor = CursorHint::Pointer;
        }
        all_radius(&mut swatch, swatch_radius);
        swatch.interaction.focusable = true;

        if let Some(handler) = on_change {
            let handler = Arc::clone(handler);
            let hex = hex.clone();
            swatch.interaction.on_activate = Some(Arc::new(move || handler(&hex)));
        }

        grid = grid.child(swatch);
    }

    grid
}
