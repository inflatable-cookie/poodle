//! ColorPicker — real GPUI component backed by ColorPickerSpec.
//!
//! Contract: `docs/contracts/components/color-picker.md`. Svelte reference:
//! `packages/svelte/components/src/ColorPicker.svelte`.
//!
//! Anatomy rendered here: trigger swatch (the spec's actual current color) +
//! optional inline hex input, and — when open — the surface popover containing
//! the 2D saturation/value gradient pad with thumb, hue slider, optional alpha
//! slider, mode toggle (SegmentedControl), channel inputs (hex/RGB/HSL), and
//! optional preset swatch grid.
//!
//! # Color source
//! The gradient/swatch/thumb COLORS are computed from the picker's value via
//! `theme_ext` color math — the one legitimate non-token color source (per
//! contract). All chrome (sizes, radii, borders, surface fill, shadow) resolves
//! from tokens.
//!
//! # GPUI deltas (build-verified only; render structure, not interactivity)
//! - GPUI 0.2.2 `linear_gradient` supports exactly two color stops. The hue
//!   strip (7 rainbow stops in CSS) is approximated by six stacked horizontal
//!   segments, each a two-stop gradient. The gradient pad's two CSS overlays
//!   (white→transparent, transparent→black) are layered as two absolutely
//!   positioned children, each a single two-stop gradient. The alpha strip
//!   checkerboard is approximated by a neutral base with a transparent→color
//!   overlay (no native repeating-conic-gradient).
//! - Interaction (gradient drag, slider drag, mode switch, hex/channel edit,
//!   swatch click) is preview-event-loop bound. The controls render at the
//!   current value; the embedded Slider/SegmentedControl/NumberInput components
//!   carry their own preview wiring. No interactivity is faked here.
//! - No ARIA channel on GPUI native elements (role/aria-valuetext not emitted).

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ChoiceOption, ColorInputMode, ColorPickerSpec, ControlDensity, ControlSize, NumberInputSpec,
    SegmentedControlSpec,
};

use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{
    hex_to_rgb255, hsv_to_hsl, pure_hue_hsla, resolve_color, resolve_opacity, resolve_px,
    resolve_radius, rgb255_to_hsla, rgb_to_hsv, Hsv,
};
use crate::{NumberInput, SegmentedControl};

/// A real GPUI color picker component backed by `ColorPickerSpec`.
pub struct ColorPicker {
    spec: ColorPickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ColorPicker {
    type Target = ColorPickerSpec;
    fn deref(&self) -> &ColorPickerSpec {
        &self.spec
    }
}

impl ColorPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ColorPickerSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    pub fn from_spec(spec: ColorPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn show_alpha(mut self, v: bool) -> Self {
        self.spec.show_alpha = v;
        self
    }
    pub fn show_input(mut self, v: bool) -> Self {
        self.spec.show_input = v;
        self
    }
    pub fn swatches(mut self, v: Vec<String>) -> Self {
        self.spec.swatches = v;
        self
    }
    pub fn default_mode(mut self, v: ColorInputMode) -> Self {
        self.spec.default_mode = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    fn id_base(&self) -> String {
        match self.id_suffix {
            Some(ref s) => format!("poodle-color-picker-{}", s),
            None => "poodle-color-picker".to_string(),
        }
    }
}

impl IntoElement for ColorPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolved chrome tokens ────────────────────────────
        let stack_gap = resolve_px(theme, "space.stack.sm");
        let trigger_radius = resolve_radius(theme, spec.trigger_radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        let radius_control = resolve_radius(theme, "radius.control");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let label_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");

        let border = resolve_color(theme, spec.border_token());
        // Svelte: trigger border = color-mix(border-default 62%, transparent)
        let trigger_border = Hsla { a: border.a * 0.62, ..border };
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Current color (the legitimate non-token color source) ──
        let current = spec.current_value().unwrap_or("#6366f1").to_string();
        let rgb = hex_to_rgb255(&current).unwrap_or(crate::theme_ext::Rgb255 {
            r: 99,
            g: 102,
            b: 241,
            a: 1.0,
        });
        let hsv: Hsv = rgb_to_hsv(rgb);
        let alpha = if spec.show_alpha { rgb.a } else { 1.0 };
        let current_color = rgb255_to_hsla(rgb, alpha);

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let size = effective_size;
        let density = spec.density;
        // Contract: trigger square sized to control height (md = 2.25rem)
        let trigger_size = px(rem_to_px(control_height_rem(effective_size)));

        let id_base = self.id_base();

        // ── Trigger swatch ────────────────────────────────────
        let mut trigger = div()
            .id(SharedString::from(format!("{}-trigger", id_base)))
            .focusable()
            .w(trigger_size)
            .h(trigger_size)
            .rounded(trigger_radius)
            .bg(current_color)
            .border_1()
            .border_color(trigger_border)
            .cursor_pointer()
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

        if spec.is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        let is_open = spec.is_open;
        let is_disabled = spec.is_disabled;

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                let next_open = !is_open;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                trigger = trigger
                    .on_click(move |_event, window, cx| {
                        handler(&next_open, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next_open, window, cx);
                        } else if event.keystroke.key == "escape" && is_open {
                            key_handler(&false, window, cx);
                        }
                    });
            }
        }

        // ── Controls row: trigger + optional inline hex input ──
        let mut controls_row = div().flex().items_center().gap(gap_sm).child(trigger);

        if spec.show_input {
            // Contract §8 inline input: 6.5rem wide, control height, code font.
            controls_row = controls_row.child(
                div()
                    .w(px(rem_to_px(6.5)))
                    .h(trigger_size)
                    .px(resolve_px(theme, "space.control.x"))
                    .rounded(radius_control)
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .text_size(label_size)
                    .text_color(text_primary)
                    .font_family("monospace")
                    .child(current.clone()),
            );
        }

        let mut wrapper = div()
            .relative()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(controls_row);

        if spec.is_open && !spec.is_disabled {
            // ── Surface popover ────────────────────────────────
            // Contract: width 24rem, padding 0.75rem, gap 0.625rem,
            // border-subtle, radius-surface, elevated bg, overlay shadow.
            let surface_gap = resolve_px(theme, "space.stack.md");
            let mut surface = div()
                .w(px(rem_to_px(24.0)))
                .rounded(surface_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border_subtle)
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                .p(gap_md)
                .flex()
                .flex_col()
                .gap(surface_gap);

            // ── Picker area: gradient pad (left) + controls (right) ──
            let pad_size = px(rem_to_px(10.0));
            let pad_radius = resolve_radius(theme, "radius.control");
            let thumb_d = px(rem_to_px(0.875));
            let pure_hue = pure_hue_hsla(hsv.h);

            // Gradient pad: base = pure hue. Layer two single-gradient
            // children: white→transparent (left→right) then
            // transparent→black (top→bottom). Mirrors the CSS ::before/::after.
            let white = hsla(0.0, 0.0, 1.0, 1.0);
            let white_t = hsla(0.0, 0.0, 1.0, 0.0);
            let black = hsla(0.0, 0.0, 0.0, 1.0);
            let black_t = hsla(0.0, 0.0, 0.0, 0.0);

            // Thumb position within the pad (s → x, v inverted → y).
            let thumb_x = px(rem_to_px(10.0) * (hsv.s / 100.0)) - thumb_d / 2.0;
            let thumb_y = px(rem_to_px(10.0) * (1.0 - hsv.v / 100.0)) - thumb_d / 2.0;

            let gradient_pad = div()
                .id(SharedString::from(format!("{}-gradient", id_base)))
                .relative()
                .flex_shrink_0()
                .w(pad_size)
                .h(pad_size)
                .rounded(pad_radius)
                .overflow_hidden()
                .bg(pure_hue)
                // white → transparent, left to right (CSS 90deg == gpui 90)
                .child(
                    div().absolute().inset_0().bg(gpui::linear_gradient(
                        90.0,
                        gpui::linear_color_stop(white, 0.0),
                        gpui::linear_color_stop(white_t, 1.0),
                    )),
                )
                // transparent → black, top to bottom (CSS to bottom == gpui 180)
                .child(
                    div().absolute().inset_0().bg(gpui::linear_gradient(
                        180.0,
                        gpui::linear_color_stop(black_t, 0.0),
                        gpui::linear_color_stop(black, 1.0),
                    )),
                )
                // Thumb ring at current S/V
                .child(
                    div()
                        .absolute()
                        .left(thumb_x)
                        .top(thumb_y)
                        .w(thumb_d)
                        .h(thumb_d)
                        .rounded_full()
                        .border_2()
                        .border_color(white)
                        .bg(current_color)
                        .shadow(vec![gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.3),
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(0.0),
                            spread_radius: px(1.0),
                        }]),
                );

            // ── Controls panel (right of gradient) ─────────────
            let mut controls_panel = div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(resolve_px(theme, "space.stack.sm"))
                .min_w_0();

            // Hue slider — rainbow track, thumb at current hue.
            controls_panel = controls_panel.child(
                hue_strip(theme, &id_base, hsv.h, size, density),
            );

            // Alpha slider (opt-in) — checkerboard→color track + thumb.
            if spec.show_alpha {
                controls_panel = controls_panel.child(alpha_strip(
                    theme,
                    &id_base,
                    alpha,
                    current_color,
                    surface_bg,
                    size,
                    density,
                ));
            }

            // Mode toggle (SegmentedControl: Hex / RGB / HSL).
            let mode_value = match spec.default_mode {
                ColorInputMode::Hex => "hex",
                ColorInputMode::Rgb => "rgb",
                ColorInputMode::Hsl => "hsl",
            };
            controls_panel = controls_panel.child(
                SegmentedControl::from_spec(
                    SegmentedControlSpec::new(vec![
                        ChoiceOption::new("hex", "Hex"),
                        ChoiceOption::new("rgb", "RGB"),
                        ChoiceOption::new("hsl", "HSL"),
                    ])
                    .with_default_value(mode_value),
                    theme,
                )
                .with_id(format!("{}-mode", id_base))
                .value(mode_value)
                .aria_label("Color input mode")
                .size(size)
                .density(density),
            );

            // Channel inputs for the current mode.
            controls_panel = controls_panel.child(channel_inputs(
                theme,
                &id_base,
                spec.default_mode,
                &current,
                rgb,
                hsv,
                alpha,
                spec.show_alpha,
                size,
                density,
                surface_bg,
                border,
                text_primary,
                text_secondary,
                radius_control,
                label_size,
            ));

            let picker_area = div()
                .flex()
                .gap(gap_md)
                .items_start()
                .child(gradient_pad)
                .child(controls_panel);

            surface = surface.child(picker_area);

            // ── Swatch grid (opt-in) ───────────────────────────
            if !spec.swatches.is_empty() {
                // Contract: swatch 1.25rem square, radius 0.1875rem, gap 0.25rem,
                // top divider border-subtle@42%.
                let swatch_size = px(rem_to_px(1.25));
                let swatch_radius = px(rem_to_px(0.1875));
                let divider = Hsla { a: border_subtle.a * 0.42, ..border_subtle };

                let mut grid = div()
                    .flex()
                    .flex_wrap()
                    .gap(resolve_px(theme, "space.inline.xs"))
                    .pt(resolve_px(theme, "space.inline.xs"))
                    .border_t_1()
                    .border_color(divider);

                for (idx, color_hex) in spec.swatches.iter().enumerate() {
                    let swatch_color = rgb255_to_hsla(
                        hex_to_rgb255(color_hex).unwrap_or(crate::theme_ext::Rgb255 {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 1.0,
                        }),
                        1.0,
                    );
                    // Active swatch: text-primary border + surface ring.
                    let is_active = color_hex.eq_ignore_ascii_case(&current);
                    let swatch_id =
                        SharedString::from(format!("{}-swatch-{}", id_base, idx));
                    let mut swatch = div()
                        .id(swatch_id)
                        .w(swatch_size)
                        .h(swatch_size)
                        .rounded(swatch_radius)
                        .border_2()
                        .bg(swatch_color)
                        .cursor_pointer();

                    if is_active {
                        swatch = swatch.border_color(text_primary).shadow(vec![gpui::BoxShadow {
                            color: surface_bg,
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(0.0),
                            spread_radius: px(rem_to_px(0.0625)),
                        }]);
                    } else {
                        swatch = swatch.border_color(hsla(0.0, 0.0, 0.0, 0.0));
                    }

                    if let Some(ref on_change) = self.on_change {
                        let handler = on_change.clone();
                        let hex_value = color_hex.clone();
                        swatch = swatch.on_click(move |_event, window, cx| {
                            handler(&hex_value, window, cx);
                        });
                    }

                    grid = grid.child(swatch);
                }
                surface = surface.child(grid);
            }

            let _ = (caption_size, text_secondary, gap_sm);
            wrapper = wrapper.child(surface);
        }

        wrapper.into_any_element()
    }
}

/// Hue slider with a rainbow track. GPUI 0.2.2 gradients support two stops, so
/// the seven-stop CSS rainbow is approximated by six stacked horizontal
/// segments (red→yellow→green→cyan→blue→magenta→red), each a two-stop gradient.
/// A thumb sits at the current hue.
fn hue_strip(
    theme: &GpuiThemeProvider,
    id_base: &str,
    hue: f32,
    size: ControlSize,
    density: ControlDensity,
) -> AnyElement {
    let track_h = px(rem_to_px(0.375));
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    // Six rainbow stops at the segment boundaries.
    let stops = [
        hsla_from_rgb(255, 0, 0),   // red
        hsla_from_rgb(255, 255, 0), // yellow
        hsla_from_rgb(0, 255, 0),   // green
        hsla_from_rgb(0, 255, 255), // cyan
        hsla_from_rgb(0, 0, 255),   // blue
        hsla_from_rgb(255, 0, 255), // magenta
        hsla_from_rgb(255, 0, 0),   // red (wrap)
    ];

    let mut track = div()
        .id(SharedString::from(format!("{}-hue", id_base)))
        .relative()
        .w_full()
        .h(track_h)
        .rounded_full()
        .overflow_hidden()
        .flex();

    for i in 0..6 {
        track = track.child(
            div().flex_1().h_full().bg(gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(stops[i], 0.0),
                gpui::linear_color_stop(stops[i + 1], 1.0),
            )),
        );
    }

    let progress = (hue / 360.0).clamp(0.0, 1.0);
    let thumb = div()
        .absolute()
        .top(px(-(thumb_d - rem_to_px(0.375)) / 2.0))
        .left(relative(progress))
        .ml(px(-(thumb_d / 2.0)))
        .w(px(thumb_d))
        .h(px(thumb_d))
        .rounded_full()
        .bg(elevated)
        .border_1()
        .border_color(border)
        .shadow(vec![gpui::BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(8.0),
            spread_radius: px(0.0),
        }]);

    let _ = (size, density);
    div()
        .relative()
        .w_full()
        .child(track)
        .child(thumb)
        .into_any_element()
}

/// Alpha slider. CSS layers a transparent→color gradient over a checkerboard;
/// GPUI 0.2.2 has no repeating-conic-gradient, so the checkerboard is
/// approximated by a neutral surface base with a transparent→color overlay.
/// A thumb sits at the current alpha.
#[allow(clippy::too_many_arguments)]
fn alpha_strip(
    theme: &GpuiThemeProvider,
    id_base: &str,
    alpha: f32,
    color: Hsla,
    surface_bg: Hsla,
    size: ControlSize,
    density: ControlDensity,
) -> AnyElement {
    let track_h = px(rem_to_px(0.375));
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    let opaque = Hsla { a: 1.0, ..color };
    let transparent = Hsla { a: 0.0, ..color };

    let track = div()
        .id(SharedString::from(format!("{}-alpha", id_base)))
        .relative()
        .w_full()
        .h(track_h)
        .rounded_full()
        .overflow_hidden()
        // Neutral checkerboard stand-in.
        .bg(surface_bg)
        .child(
            div().absolute().inset_0().bg(gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(transparent, 0.0),
                gpui::linear_color_stop(opaque, 1.0),
            )),
        );

    let progress = alpha.clamp(0.0, 1.0);
    let thumb = div()
        .absolute()
        .top(px(-(thumb_d - rem_to_px(0.375)) / 2.0))
        .left(relative(progress))
        .ml(px(-(thumb_d / 2.0)))
        .w(px(thumb_d))
        .h(px(thumb_d))
        .rounded_full()
        .bg(elevated)
        .border_1()
        .border_color(border)
        .shadow(vec![gpui::BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(8.0),
            spread_radius: px(0.0),
        }]);

    let _ = (size, density);
    div()
        .relative()
        .w_full()
        .child(track)
        .child(thumb)
        .into_any_element()
}

/// Channel inputs row for the current mode (hex / RGB / HSL), each a labelled
/// NumberInput (or hex text field) showing the current value. The optional
/// alpha channel appends when `show_alpha`.
#[allow(clippy::too_many_arguments)]
fn channel_inputs(
    theme: &GpuiThemeProvider,
    id_base: &str,
    mode: ColorInputMode,
    current_hex: &str,
    rgb: crate::theme_ext::Rgb255,
    hsv: Hsv,
    alpha: f32,
    show_alpha: bool,
    size: ControlSize,
    density: ControlDensity,
    surface_bg: Hsla,
    border: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    radius_control: Pixels,
    label_size: Pixels,
) -> AnyElement {
    let gap = resolve_px(theme, "space.inline.xs");
    let mut row = div().flex().gap(gap).items_start();

    // Stacked field: control over an uppercase label, matching Svelte.
    let labelled = |child: AnyElement, label: &str| -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.125)))
            .min_w_0()
            .child(child)
            .child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(label.to_string().to_uppercase()),
            )
    };

    let number = |id: &str, value: f64, min: f64, max: f64, aria: &str| -> AnyElement {
        NumberInput::from_spec(
            NumberInputSpec::new(value)
                .with_min(min)
                .with_max(max)
                .with_step(1.0)
                .with_aria_label(aria),
            theme,
        )
        .with_id(id.to_string())
        .size(size)
        .density(density)
        .into_any_element()
    };

    match mode {
        ColorInputMode::Hex => {
            // Hex: a code-font text field showing the current value.
            let hex_field = div()
                .id(SharedString::from(format!("{}-hex-input", id_base)))
                .w_full()
                .h(px(rem_to_px(2.0)))
                .px(px(rem_to_px(0.375)))
                .rounded(radius_control)
                .bg(surface_bg)
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .text_size(label_size)
                .text_color(text_primary)
                .font_family("monospace")
                .child(current_hex.to_string())
                .into_any_element();
            row = row.child(labelled(hex_field, "Hex"));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
        ColorInputMode::Rgb => {
            row = row.child(labelled(
                number(&format!("{}-r", id_base), rgb.r as f64, 0.0, 255.0, "Red"),
                "R",
            ));
            row = row.child(labelled(
                number(&format!("{}-g", id_base), rgb.g as f64, 0.0, 255.0, "Green"),
                "G",
            ));
            row = row.child(labelled(
                number(&format!("{}-b", id_base), rgb.b as f64, 0.0, 255.0, "Blue"),
                "B",
            ));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
        ColorInputMode::Hsl => {
            let hsl = hsv_to_hsl(hsv.h, hsv.s, hsv.v);
            row = row.child(labelled(
                number(&format!("{}-h", id_base), hsl.h as f64, 0.0, 360.0, "Hue"),
                "H",
            ));
            row = row.child(labelled(
                number(
                    &format!("{}-s", id_base),
                    hsl.s as f64,
                    0.0,
                    100.0,
                    "Saturation",
                ),
                "S",
            ));
            row = row.child(labelled(
                number(
                    &format!("{}-l", id_base),
                    hsl.l as f64,
                    0.0,
                    100.0,
                    "Lightness",
                ),
                "L",
            ));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
    }

    row.into_any_element()
}

/// Opaque `gpui::Hsla` from 8-bit RGB. Local convenience for rainbow stops.
fn hsla_from_rgb(r: u8, g: u8, b: u8) -> Hsla {
    rgb255_to_hsla(
        crate::theme_ext::Rgb255 {
            r,
            g,
            b,
            a: 1.0,
        },
        1.0,
    )
}
