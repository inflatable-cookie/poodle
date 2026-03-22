//! SegmentedControl — real GPUI component backed by SegmentedControlSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{ChoiceOption, SegmentedControlSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI segmented control component backed by `SegmentedControlSpec`.
pub struct SegmentedControl {
    spec: SegmentedControlSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SegmentedControl {
    type Target = SegmentedControlSpec;
    fn deref(&self) -> &SegmentedControlSpec { &self.spec }
}

impl SegmentedControl {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SegmentedControlSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_change: None }
    }

    pub fn from_spec(spec: SegmentedControlSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-seg".to_string(),
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn options(mut self, v: Vec<ChoiceOption>) -> Self { self.spec.options = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SegmentedControl {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // Contract: root bg = color-mix(surface 93%, text-primary)
        // Contract: root border = color-mix(border-subtle 84%, transparent)
        // Contract: inner radius = calc(radius-control - 0.125rem)
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let inner_radius = (control_radius - px(2.0)).max(px(0.0));

        let accent = resolve_color(theme, self.spec.selected_fill_token());
        let border_subtle = resolve_color(theme, "semantic.color.border.subtle");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        // Contract: root bg = surface 93% mix with text-primary
        let root_bg = color_mix(surface_bg, text_primary, 0.93);
        // Contract: root border = border-subtle 84%
        let root_border = Hsla { a: border_subtle.a * 0.84, ..border_subtle };

        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_disabled = self.spec.is_disabled;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        // Contract: segment min-height = calc(control-height - 0.25rem)
        let segment_height = control_height - px(4.0);

        let mut row = div()
            .flex()
            .rounded(control_radius)
            .border_1()
            .border_color(root_border)
            .bg(root_bg)
            .h(control_height);

        if is_disabled {
            row = row.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        for (i, option) in self.spec.options.iter().enumerate() {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_opt_disabled = option.is_disabled;
            let seg_id = SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            // Contract: font 0.75rem (12px), weight 600, padding 0 0.75rem (12px)
            let mut seg = div()
                .id(seg_id)
                .focusable()
                .px(px(12.0))
                .h(segment_height)
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .flex().items_center().justify_center()
                .whitespace_nowrap()
                .overflow_x_hidden()
                .text_ellipsis()
                .focus(move |s| s.border_color(focus_ring));

            if is_selected {
                seg = seg.bg(accent).text_color(text_inverse).rounded(inner_radius);
            } else {
                // Contract: unselected text = text-secondary
                seg = seg.text_color(text_secondary);
            }

            if !is_disabled && !is_opt_disabled {
                seg = seg
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg));
            }

            // Add separator between non-selected items
            if i > 0 && !is_selected {
                let prev_selected = current_value.as_deref()
                    == self.spec.options.get(i - 1).map(|o| o.value.as_str());
                if !prev_selected {
                    row = row.child(
                        div()
                            .w(px(1.0))
                            .h_full()
                            .bg(root_border),
                    );
                }
            }

            seg = seg.child(option.label.clone());
            row = row.child(seg);
        }

        row.into_any_element()
    }
}
