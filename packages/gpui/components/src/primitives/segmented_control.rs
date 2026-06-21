//! SegmentedControl — real GPUI component backed by SegmentedControlSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ChoiceOption, ControlSize, SegmentedControlSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI segmented control component backed by `SegmentedControlSpec`.
pub struct SegmentedControl {
    spec: SegmentedControlSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SegmentedControl {
    type Target = SegmentedControlSpec;
    fn deref(&self) -> &SegmentedControlSpec {
        &self.spec
    }
}

impl SegmentedControl {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SegmentedControlSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
        }
    }

    pub fn from_spec(spec: SegmentedControlSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-seg".to_string(),
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
    pub fn options(mut self, v: Vec<ChoiceOption>) -> Self {
        self.spec.options = v;
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
    pub fn equal_width(mut self, v: bool) -> Self {
        self.spec.equal_width = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for SegmentedControl {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // Contract: root bg = color-mix(surface 93%, text-primary)
        // Contract: root border = color-mix(border-subtle 84%, transparent)
        // Contract §8 Label: inner radius = calc(radius-control - 0.125rem)
        let control_radius = resolve_radius(theme, "radius.control");
        let inner_radius = (control_radius - px(rem_to_px(0.125))).max(px(0.0));

        let accent = resolve_color(theme, self.spec.selected_fill_token());
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated = resolve_color(theme, "color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");

        // Contract §8 selected Label: box-shadow inset 0 0.0625rem 0
        // color-mix(white 12%, transparent). No `white` token exists; `text.inverse`
        // is the closest semantic (white in the dark theme). Mixed to 12% alpha.
        // GPUI BoxShadow has no inset flag, so the highlight is approximated as a
        // 1px top edge line (offset y = 0.0625rem, blur 0) — same pattern as accordion.
        let selected_inset_highlight = Hsla {
            a: text_inverse.a * 0.12,
            ..text_inverse
        };

        // Contract: root bg = surface 93% mix with text-primary
        let root_bg = color_mix(surface_bg, text_primary, 0.93);
        // Contract: root border = border-subtle 84%
        let root_border = Hsla {
            a: border_subtle.a * 0.84,
            ..border_subtle
        };

        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_disabled = self.spec.is_disabled;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        // Container has 0.125rem padding on all sides (Svelte: padding: 0.125rem)
        // Segment height = control height minus top+bottom container padding (2 × 0.125rem)
        let segment_height = control_height - px(rem_to_px(0.25));
        // Svelte: font-size is fixed at 0.75rem for all sizes (not size-responsive)
        let segment_font_size = px(rem_to_px(0.75));
        // Contract §8 Label padding-x = `--poodle-segmented-control-x`, which Svelte
        // derives from density (0.5/0.75/1rem), not size. Match the authoritative
        // density-driven source (was incorrectly size-offset before).
        let segment_pad_x = px(rem_to_px(control_space_x_rem(self.spec.density)));

        let mut row = div()
            .flex()
            .items_center()
            .rounded(control_radius)
            .border_1()
            .border_color(root_border)
            .bg(root_bg)
            .h(control_height)
            .gap(px(rem_to_px(0.125))) // Svelte: gap: 0.125rem between segments
            .p(px(rem_to_px(0.125))); // Svelte: padding: 0.125rem

        if is_disabled {
            row = row
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        let option_values: Vec<String> =
            self.spec.options.iter().map(|o| o.value.clone()).collect();

        for (i, option) in self.spec.options.iter().enumerate() {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_opt_disabled = option.is_disabled;
            let seg_id = SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            // Contract §8 Label. No per-segment border in the authoritative Svelte;
            // focus is rendered as a ring shadow only (contract focus uses `outline`,
            // which does not affect layout), so segments never inflate or shift.
            let mut seg = div()
                .id(seg_id)
                .focusable()
                .px(segment_pad_x)
                .h(segment_height)
                .rounded(inner_radius)
                .text_size(segment_font_size)
                .font_weight(FontWeight::SEMIBOLD)
                .flex()
                .items_center()
                .justify_center()
                .whitespace_nowrap()
                .overflow_x_hidden()
                .text_ellipsis()
                .focus(move |s| s.shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

            // equal_width: every segment takes an equal share of the row.
            // When false, segments size to their content (contract §7).
            if self.spec.equal_width {
                seg = seg.flex_1();
            }

            // Per-option accessible-name override (contract §6). GPUI has no
            // accessibility channel, so `aria_label` cannot be emitted; reading it
            // here documents the intent without affecting render. `title` tooltip
            // is blocked on the shared ChoiceOption spec lacking the field.
            let _aria_label = option.aria_label.as_deref();

            if is_selected {
                // Contract §8 selected Label: accent bg, inverse text, inset highlight.
                // Brand-raised treatment: gradient fill for selected segment.
                if theme.brand_raised && !is_disabled && !is_opt_disabled {
                    use crate::theme_ext::{
                        brand_raised_primary_fill, brand_raised_primary_shadow,
                    };
                    seg = seg
                        .bg(brand_raised_primary_fill(accent))
                        .text_color(text_inverse)
                        .shadow(brand_raised_primary_shadow());
                } else {
                    // Contract: box-shadow inset 0 0.0625rem 0 color-mix(white 12%).
                    seg = seg
                        .bg(accent)
                        .text_color(text_inverse)
                        .shadow(vec![gpui::BoxShadow {
                            color: selected_inset_highlight,
                            offset: point(px(0.0), px(rem_to_px(0.0625))),
                            blur_radius: px(0.0),
                            spread_radius: px(0.0),
                        }]);
                }
            } else {
                // Contract: unselected text = text-secondary
                seg = seg.text_color(text_secondary);
            }

            if !is_disabled && !is_opt_disabled {
                seg = seg.cursor_pointer().hover(move |s| s.bg(hover_bg));

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = option.value.clone();
                    seg = seg.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key roving focus (left/right wraps around)
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = option_values.clone();
                    let current_idx = i;
                    seg = seg.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx = if event.keystroke.key == "right" {
                            Some((current_idx + 1) % nav_values.len())
                        } else if event.keystroke.key == "left" {
                            Some(if current_idx == 0 {
                                nav_values.len() - 1
                            } else {
                                current_idx - 1
                            })
                        } else {
                            None
                        };
                        if let Some(idx) = next_idx {
                            handler(&nav_values[idx], window, cx);
                        }
                    });
                }
            }

            seg = seg.child(option.label.clone());
            row = row.child(seg);
        }

        row.into_any_element()
    }
}
