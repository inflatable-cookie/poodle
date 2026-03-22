//! TriStateSwitch — real GPUI component backed by TriStateSwitchSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CheckState, TriStateSwitchSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px};

/// A real GPUI tri-state switch component backed by `TriStateSwitchSpec`.
pub struct TriStateSwitch {
    spec: TriStateSwitchSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TriStateSwitch {
    type Target = TriStateSwitchSpec;
    fn deref(&self) -> &TriStateSwitchSpec { &self.spec }
}

impl TriStateSwitch {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TriStateSwitchSpec::new(), theme: theme.clone(), on_click: None }
    }

    pub fn from_spec(spec: TriStateSwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn state(mut self, v: CheckState) -> Self { self.spec.state = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }


    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TriStateSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let thumb_color = resolve_color(theme, "semantic.color.text.inverse");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        // Contract: same dimensions as regular switch
        // Track = 2.125rem (34px) wide x 1.25rem (20px) tall
        let track_w = px(34.0);
        let track_h = px(20.0);
        let track_radius = px(10.0); // pill
        let track_padding = px(2.0);

        // Thumb = 0.875rem (14px) diameter
        let thumb_size = px(14.0);
        let thumb_radius = px(7.0);

        // Thumb position: unchecked=left(2), mixed=center(10), checked=right(18)
        let thumb_offset = match spec.state {
            CheckState::Unchecked => track_padding,
            CheckState::Mixed => px(10.0),
            CheckState::Checked => px(18.0),
        };

        // Track fill based on state
        let track_bg = match spec.state {
            CheckState::Checked => color_mix(accent, surface_bg, 0.24),
            CheckState::Mixed => color_mix(accent, surface_bg, 0.12),
            CheckState::Unchecked => surface_bg,
        };

        let track_border = match spec.state {
            CheckState::Checked => color_mix(accent, border, 0.58),
            CheckState::Mixed => color_mix(accent, border, 0.30),
            CheckState::Unchecked => border,
        };

        // Knob color: accent when checked/mixed, inverse otherwise
        let knob_color = match spec.state {
            CheckState::Checked | CheckState::Mixed => accent,
            CheckState::Unchecked => thumb_color,
        };

        let switch_id = SharedString::from("pug-tri-state-switch");

        let mut track = div()
            .id(switch_id)
            .w(track_w)
            .h(track_h)
            .rounded(track_radius)
            .bg(track_bg)
            .border_1()
            .border_color(track_border)
            .relative()
            .child(
                div()
                    .absolute()
                    .top(track_padding)
                    .left(thumb_offset)
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(knob_color),
            );

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        track = track.focus(move |s| s.border_color(focus_ring));

        if !spec.is_disabled {
            track = track.cursor_pointer();
            if let Some(handler) = self.on_click {
                track = track.on_click(move |event, window, cx| handler(event, window, cx));
            }
        } else {
            track = track
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        let mut el = div()
            .flex()
            .items_center()
            .gap(inline_gap)
            .child(track);

        if let Some(ref label) = spec.label {
            el = el.child(
                div()
                    .text_sm()
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        el.into_any_element()
    }
}
