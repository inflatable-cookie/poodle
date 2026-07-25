//! ResizeHandle — real GPUI component backed by ResizeHandleSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Orientation, ResizeHandleSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI resize handle component backed by `ResizeHandleSpec`.
///
/// Renders a hit target area containing a thin visual affordance line.
/// - Horizontal orientation: vertical line, col-resize cursor, drag left/right.
/// - Vertical orientation: horizontal line, row-resize cursor, drag up/down.
pub struct ResizeHandle {
    spec: ResizeHandleSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
}

impl std::ops::Deref for ResizeHandle {
    type Target = ResizeHandleSpec;
    fn deref(&self) -> &ResizeHandleSpec {
        &self.spec
    }
}

impl ResizeHandle {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ResizeHandleSpec::new(),
            theme: theme.clone(),
            id_prefix: "poodle-resize".to_string(),
        }
    }

    pub fn from_spec(spec: ResizeHandleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-resize".to_string(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn aria_value_now(mut self, v: f32) -> Self {
        self.spec.aria_value_now = Some(v);
        self
    }
    pub fn aria_value_min(mut self, v: f32) -> Self {
        self.spec.aria_value_min = v;
        self
    }
    pub fn aria_value_max(mut self, v: f32) -> Self {
        self.spec.aria_value_max = v;
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }
}

impl IntoElement for ResizeHandle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Resolve tokens
        let border_color = resolve_color(theme, spec.border_color_token());
        let hover_color = resolve_color(theme, spec.hover_color_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let is_horizontal = spec.orientation == Orientation::Horizontal;
        let handle_id = SharedString::from(format!("{}-handle", self.id_prefix));
        // Group name lets the hover state on the hit target recolor the line
        // (contract §8: hover changes the *line* background to accent-base).
        let group_name = SharedString::from(format!("{}-group", self.id_prefix));

        // Root: only as thick as the line, so the divider costs no layout space
        // beyond the hairline itself. The grab area is an absolutely positioned
        // overlay centred on it (contract §7), overlapping the neighbours.
        let mut container = div()
            .id(handle_id)
            .group(group_name.clone())
            .focusable()
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            // Focus ring (contract §8 focus-visible: accent focus-ring outline).
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

        // Visual affordance: the line fills the root.
        // Contract §8: hover/dragging recolors this line to accent-base — wired
        // via group_hover so the hit target's hover drives the line color, not
        // a translucent fill on the container.
        let mut line = if is_horizontal {
            div()
                .w(px(rem_to_px(spec.thickness_rem())))
                .h_full()
                .rounded(px(999.0))
                .bg(border_color)
        } else {
            div()
                .w_full()
                .h(px(rem_to_px(spec.thickness_rem())))
                .rounded(px(999.0))
                .bg(border_color)
        };
        if !spec.is_disabled {
            line = line.group_hover(group_name, move |s| s.bg(hover_color));
        }

        // Grab overlay: wider than the line, centred on it, absolutely
        // positioned so it never contributes to layout.
        let hit_size = px(rem_to_px(spec.hit_size_rem()));
        let hit_offset = px(rem_to_px(spec.hit_offset_rem()));
        let hit = if is_horizontal {
            div().absolute().top_0().left(hit_offset).w(hit_size).h_full()
        } else {
            div().absolute().left_0().top(hit_offset).h(hit_size).w_full()
        };

        if is_horizontal {
            container = container
                .relative()
                .w(px(rem_to_px(spec.thickness_rem())))
                .h_full()
                .cursor_col_resize();
        } else {
            container = container
                .relative()
                .w_full()
                .h(px(rem_to_px(spec.thickness_rem())))
                .cursor_row_resize();
        }

        // Disabled state: reduced opacity, default cursor, no hover.
        if spec.is_disabled {
            container = container
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        container = container.child(hit).child(line);

        container.into_any_element()
    }
}
