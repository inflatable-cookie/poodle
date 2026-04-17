//! RadioGroup — real GPUI component backed by RadioGroupSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ChoiceOption, ControlSize, Orientation, RadioGroupSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI radio group component backed by `RadioGroupSpec`.
pub struct RadioGroup {
    spec: RadioGroupSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for RadioGroup {
    type Target = RadioGroupSpec;
    fn deref(&self) -> &RadioGroupSpec {
        &self.spec
    }
}

impl RadioGroup {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: RadioGroupSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
        }
    }

    pub fn from_spec(spec: RadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-radio".to_string(),
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
    pub fn description_id(mut self, v: impl Into<String>) -> Self {
        self.spec.description_id = Some(v.into());
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
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for RadioGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Indicator: size.icon.{size} + offset
        // Svelte: xs/sm use +0.25rem offset, md/lg/xl use +0.125rem offset
        let icon_size_token = match effective_size {
            ControlSize::Xs => "size.icon.xs",
            ControlSize::Sm => "size.icon.sm",
            ControlSize::Md => "size.icon.md",
            ControlSize::Lg => "size.icon.lg",
            ControlSize::Xl => "size.icon.xl",
        };
        let indicator_offset = match effective_size {
            ControlSize::Xs | ControlSize::Sm => rem_to_px(0.25),
            _ => rem_to_px(0.125),
        };
        let indicator_size = resolve_px(theme, icon_size_token) + px(indicator_offset);

        // Dot: per-size rem values; md uses token formula (size.icon.md × 0.5)
        // Svelte: xs→0.4rem, sm→0.45rem, md→icon-md×0.5, lg→0.55rem, xl→0.6rem
        let icon_md = resolve_px(theme, "size.icon.md");
        let dot_size = match effective_size {
            ControlSize::Xs => px(rem_to_px(0.4)),
            ControlSize::Sm => px(rem_to_px(0.45)),
            ControlSize::Md => icon_md * 0.5,
            ControlSize::Lg => px(rem_to_px(0.55)),
            ControlSize::Xl => px(rem_to_px(0.6)),
        };

        // Contract: gap per orientation
        let group_gap = resolve_px(theme, spec.option_gap_token());
        let option_gap = resolve_px(theme, "space.inline.sm");

        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let accent = if let Some(ref hex) = spec.selected_color {
            crate::theme_ext::parse_hex_color(hex)
                .unwrap_or_else(|| resolve_color(theme, "color.accent.base"))
        } else {
            resolve_color(theme, "color.accent.base")
        };
        let border = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        // Svelte: label uses typography.label.size (not body)
        let body_size = resolve_px(theme, "typography.label.size");

        let current_value = spec.current_value().map(|s| s.to_string());

        // Wrap on_change in Rc so it can be shared across option closures
        let on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(|h| std::rc::Rc::from(h));

        // Contract: grid layout, orientation drives flow
        // Svelte: horizontal gap = space.inline.md (larger than vertical space.stack.sm)
        let horizontal_gap = resolve_px(theme, "space.inline.md");
        let mut group = match spec.orientation {
            Orientation::Horizontal => div().flex().flex_row().items_center().gap(horizontal_gap),
            Orientation::Vertical => div().flex().flex_col().gap(group_gap),
        };

        let option_values: Vec<String> = spec.options.iter().map(|o| o.value.clone()).collect();

        for (idx, option) in spec.options.iter().enumerate() {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_disabled = spec.is_disabled || option.is_disabled;
            let option_id = SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            // Contract: indicator = circle, border-default, surface bg
            // Checked: border-color = accent (NOT fill), dot bg = accent
            let mut indicator = div()
                .w(indicator_size)
                .h(indicator_size)
                .rounded(px(999.0))
                .bg(surface_bg)
                .border_1()
                .border_color(if is_selected { accent } else { border })
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0();

            // Contract: dot always present, transparent when unchecked, accent when checked
            indicator = indicator.child(div().w(dot_size).h(dot_size).rounded(px(999.0)).bg(
                if is_selected {
                    accent
                } else {
                    gpui::transparent_black()
                },
            ));

            let mut row = div()
                .id(option_id)
                .focusable()
                .flex()
                .items_center()
                .gap(option_gap)
                .min_w(px(0.0))
                .focus(move |s| {
                    s.border_color(focus_ring)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                });

            if is_disabled {
                row = row
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                row = row.cursor_pointer();

                // Click + keyboard handlers
                if let Some(ref handler) = on_change {
                    let click_handler = handler.clone();
                    let key_handler = handler.clone();
                    let val = option.value.clone();
                    let val2 = option.value.clone();
                    let arrow_handler = handler.clone();
                    let ovs = option_values.clone();
                    let current_idx = idx;
                    row = row
                        .on_click(move |_event, window, cx| {
                            click_handler(&val, window, cx);
                        })
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                                key_handler(&val2, window, cx);
                            } else if event.keystroke.key == "down"
                                || event.keystroke.key == "right"
                            {
                                let next = (current_idx + 1) % ovs.len();
                                arrow_handler(&ovs[next], window, cx);
                            } else if event.keystroke.key == "up" || event.keystroke.key == "left" {
                                let prev = if current_idx == 0 {
                                    ovs.len() - 1
                                } else {
                                    current_idx - 1
                                };
                                arrow_handler(&ovs[prev], window, cx);
                            }
                        });
                }
            }

            row = row.child(indicator);

            // Label with contract typography
            row = row.child(
                div()
                    .text_size(body_size)
                    .text_color(text_primary)
                    .whitespace_nowrap()
                    .child(option.label.clone()),
            );

            group = group.child(row);
        }

        group.into_any_element()
    }
}
