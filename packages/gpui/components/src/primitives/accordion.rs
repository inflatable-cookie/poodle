//! Accordion — real GPUI component backed by AccordionSpec.
//!
//! Visual structure matches the Svelte implementation:
//! - Each item is a bordered card with padding, tinted background, and inset shadow
//! - Items are separated by gap (grid layout)
//! - Trigger shows title + optional description on left, chevron on right
//! - Chevron-down icon rotates when item is open

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AccordionItemSpec, AccordionSelectionValue, AccordionSpec, ControlDensity, ControlSize,
    IconSize, IconSpec, SemanticControlSizeRole,
};
use std::rc::Rc;

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI accordion component backed by `AccordionSpec`.
pub struct Accordion {
    spec: AccordionSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_toggle: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    content: Vec<(String, AnyElement)>,
}

impl std::ops::Deref for Accordion {
    type Target = AccordionSpec;
    fn deref(&self) -> &AccordionSpec {
        &self.spec
    }
}

impl Accordion {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: AccordionSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_toggle: None,
            content: Vec::new(),
        }
    }

    pub fn from_spec(spec: AccordionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-accordion".to_string(),
            on_toggle: None,
            content: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<AccordionItemSpec>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn value(mut self, v: AccordionSelectionValue) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: AccordionSelectionValue) -> Self {
        self.spec.default_value = Some(v);
        self
    }
    pub fn allow_multiple(mut self, v: bool) -> Self {
        self.spec.allow_multiple = v;
        self
    }
    pub fn collapsible(mut self, v: bool) -> Self {
        self.spec.is_collapsible = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    /// Add content for a specific item value, shown when that item is expanded.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for Accordion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        // Accordion titles use heading-scale sizes, not control-scale (Svelte: accordion__title)
        let title_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.8125,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 1.0,
            ControlSize::Lg => 1.0625,
            ControlSize::Xl => 1.125,
        }));
        // Description scales with size like control fonts (Svelte: accordion__description)
        let description_font = px(rem_to_px(size_font_rem(effective_size)));
        // Contract §8 Item inline padding (density-overridden): compact 0.5rem, default/comfortable 1rem
        let density_pad_x = px(rem_to_px(self.spec.inline_padding_rem(self.spec.density)));
        // Contract §8 Item block padding: fixed 0.625rem
        let density_pad_y = px(rem_to_px(self.spec.block_padding_rem()));
        // Contract §8 Root gap between items = space.stack.md
        let root_gap = resolve_px(theme, self.spec.root_gap_token());
        // Contract §8 Item internal gap (trigger ↔ panel) = 0.625rem
        let item_gap = px(rem_to_px(self.spec.item_internal_gap_rem()));

        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_inverse = resolve_color(theme, self.spec.inset_highlight_color_token());
        let elevated_bg = resolve_color(theme, self.spec.item_bg_elevated_token());
        let panel_bg = resolve_color(theme, self.spec.item_bg_panel_token());
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let surface_radius = resolve_radius(theme, "radius.surface");
        let panel_pad_x = density_pad_x;
        let panel_pad_y = density_pad_y;
        let heading_size = title_font;
        let gap_inline_sm = resolve_px(theme, self.spec.summary_gap_token());
        let gap_inline_md = resolve_px(theme, self.spec.trigger_grid_gap_token());

        // Contract §8 Item background: color-mix(elevated 40%, panel)
        let item_bg = color_mix(elevated_bg, panel_bg, self.spec.item_bg_elevated_ratio());
        // Contract §8 Item border: border-subtle at 36% opacity
        let item_border = Hsla {
            a: border_subtle.a * self.spec.border_subtle_alpha(),
            ..border_subtle
        };
        // Contract §8 Item box-shadow: inset 0 0.0625rem 0 text-inverse 8% — top highlight
        let inset_highlight = Hsla {
            a: text_inverse.a * self.spec.inset_highlight_alpha(),
            ..text_inverse
        };

        let expanded = self.spec.expanded_values();

        // Build content map from the content vec
        let mut content_map: std::collections::HashMap<String, AnyElement> =
            std::collections::HashMap::new();
        for (key, el) in self.content {
            content_map.insert(key, el);
        }

        // Outer container: grid with gap between items
        let mut col = div().flex().flex_col().gap(root_gap);

        for item in &self.spec.items {
            let is_open = expanded.contains(&item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // ── Trigger button (title + description on left, chevron on right) ──
            let mut trigger = div()
                .id(item_id)
                .focusable()
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .gap(gap_inline_md) // Svelte: space-inline-md
                .cursor_pointer();

            trigger = trigger.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                trigger = trigger
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            }

            // Left side: title + optional description stacked vertically
            let mut summary = div()
                .flex()
                .flex_col()
                .gap(gap_inline_sm) // Svelte: space-inline-sm
                .min_w(px(0.0))
                .flex_1();

            // Title: bold, 16px, 1.2 line-height
            summary = summary.child(
                div()
                    .text_size(heading_size)
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .text_color(text_primary)
                    .child(item.label.clone()),
            );

            // Description shown in trigger (not in expanded panel) per Svelte
            if let Some(ref desc) = item.description {
                summary = summary.child(
                    div()
                        .text_size(description_font) // Svelte: size-responsive, matches control font scale
                        .line_height(relative(1.45))
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }

            trigger = trigger.child(summary);

            // Chevron indicator — always chevron-down, visually rotated when open
            // GPUI doesn't support CSS transform rotation on divs easily,
            // so we use chevron-up when open
            let chevron_name = if is_open {
                "chevron-up"
            } else {
                "chevron-down"
            };
            trigger = trigger.child(
                div().flex().items_center().flex_shrink_0().child(
                    Icon::from_spec(IconSpec::new(chevron_name).with_size(IconSize::Sm), theme)
                        .with_color(text_secondary),
                ),
            );

            // Click + keyboard handler
            if !is_disabled {
                if let Some(ref handler) = self.on_toggle {
                    let click_handler = handler.clone();
                    let key_handler = handler.clone();
                    let value = item.value.clone();
                    let value2 = item.value.clone();
                    trigger = trigger
                        .on_click(move |_event, window, cx| {
                            click_handler(&value, window, cx);
                        })
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                                key_handler(&value2, window, cx);
                            }
                        });
                }
            }

            // ── Item card container (bordered, padded, tinted bg) ──
            let mut item_card = div()
                .flex()
                .flex_col()
                .gap(item_gap)
                .px(panel_pad_x)
                .py(panel_pad_y)
                .border_1()
                .border_color(item_border)
                .rounded(surface_radius)
                // Contract §8 Item box-shadow: inset 0 0.0625rem 0 text-inverse 8%
                .shadow(vec![gpui::BoxShadow {
                    color: inset_highlight,
                    offset: point(px(0.0), px(rem_to_px(self.spec.inset_highlight_offset_rem()))),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                }]);

            // Brand-raised treatment: gradient fill for accordion item card
            item_card = item_card.bg(item_bg);

            item_card = item_card.child(trigger);

            // Panel content (when expanded)
            if is_open {
                if let Some(panel_content) = content_map.remove(&item.value) {
                    item_card = item_card.child(div().min_w(px(0.0)).child(panel_content));
                }
            }

            col = col.child(item_card);
        }

        col.into_any_element()
    }
}
