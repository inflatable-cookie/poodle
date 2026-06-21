use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MetaItemSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub struct MetaItem {
    spec: MetaItemSpec,
    theme: GpuiThemeProvider,
    value: Option<AnyElement>,
}

impl std::ops::Deref for MetaItem {
    type Target = MetaItemSpec;
    fn deref(&self) -> &MetaItemSpec {
        &self.spec
    }
}

impl MetaItem {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetaItemSpec::new(), theme)
    }

    pub fn from_spec(spec: MetaItemSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            value: None,
        }
    }

    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn typography(mut self, v: poodle_specs::InlineTypographyMode) -> Self {
        self.spec.typography = v;
        self
    }

    /// Presentational `data-separator` intent read by a parent MetaBar
    /// (contract §6, default `true`).
    pub fn separator(mut self, v: bool) -> Self {
        self.spec.separator = v;
        self
    }

    /// The item's `separator` flag — MetaBar reads this to decide whether to draw
    /// a leading separator dot before this item.
    pub fn separator_intent(&self) -> bool {
        self.spec.separator
    }

    pub fn with_value(mut self, value: impl IntoElement) -> Self {
        self.value = Some(value.into_any_element());
        self
    }
}

impl IntoElement for MetaItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let label_color = resolve_color(&self.theme, self.spec.label_color_token());
        let value_color = resolve_color(&self.theme, self.spec.value_color_token());

        let mut item = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(rem_to_px(self.spec.gap_rem())))
            .min_w(px(0.0));

        if let Some(ref label) = self.spec.label {
            // Label typography from tokens (contract §7): label-family,
            // 0.6875rem, label-weight (500), line-height 1, uppercase. GPUI has
            // no letter-spacing channel — `0.08em` remains an accepted delta.
            item = item.child(
                div()
                    .text_size(px(rem_to_px(self.spec.label_font_size_rem())))
                    .font_family(self.spec.label_family_token())
                    .font_weight(FontWeight(self.spec.label_font_weight() as f32))
                    .line_height(relative(self.spec.label_line_height()))
                    .text_color(label_color)
                    .child(label.to_uppercase()),
            );
        }

        let value = self.value.unwrap_or_else(|| {
            div()
                .text_size(px(rem_to_px(self.spec.value_font_size_rem())))
                .text_color(value_color)
                .child("Value")
                .into_any_element()
        });

        // Value typography (contract §7): body-family, 0.875rem, line-height 1.4.
        item.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(rem_to_px(self.spec.gap_rem())))
                .min_w(px(0.0))
                .text_size(px(rem_to_px(self.spec.value_font_size_rem())))
                .font_family(self.spec.value_family_token())
                .line_height(relative(self.spec.value_line_height()))
                .text_color(value_color)
                .child(value),
        )
        .into_any_element()
    }
}
