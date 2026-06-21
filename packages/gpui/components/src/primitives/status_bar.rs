//! StatusBar — real GPUI component backed by ShellStatusBarSpec (contract: status-bar).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole, ShellStatusBarSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// A real GPUI status bar backed by `ShellStatusBarSpec`.
///
/// Renders a `<footer>`-equivalent status strip with a leading region
/// (slot content or summary fallback) and a trailing region (only when
/// trailing items exist), arranged with space-between layout.
pub struct StatusBar {
    spec: ShellStatusBarSpec,
    theme: GpuiThemeProvider,
    /// Slot for leading status items (left side).
    leading_items: Option<AnyElement>,
    /// Slot for trailing status items (right side).
    trailing_items: Option<AnyElement>,
}

impl std::ops::Deref for StatusBar {
    type Target = ShellStatusBarSpec;
    fn deref(&self) -> &ShellStatusBarSpec {
        &self.spec
    }
}

impl StatusBar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ShellStatusBarSpec::new(),
            theme: theme.clone(),
            leading_items: None,
            trailing_items: None,
        }
    }

    pub fn from_spec(spec: ShellStatusBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading_items: None,
            trailing_items: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn summary(mut self, v: impl Into<String>) -> Self {
        self.spec.summary = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn chrome(mut self, v: bool) -> Self {
        self.spec.chrome = v;
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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

    pub fn with_leading_items(mut self, items: impl IntoElement) -> Self {
        self.leading_items = Some(items.into_any_element());
        self
    }

    pub fn with_trailing_items(mut self, items: impl IntoElement) -> Self {
        self.trailing_items = Some(items.into_any_element());
        self
    }
}

impl IntoElement for StatusBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_secondary = resolve_color(theme, spec.text_color_token());

        // Contract §8: default font-size 0.8125rem, scaled by size; default
        // padding 0.375rem 0.75rem, scaled by size (block) / density (inline);
        // gap = space.inline.md (root) overridden by density.
        let font_size = px(rem_to_px(spec.font_size_rem()));
        let pad_block = px(rem_to_px(spec.padding_block_rem()));
        let pad_inline = px(rem_to_px(spec.padding_inline_rem()));
        let root_gap = match spec.density_gap_rem() {
            Some(rem) => px(rem_to_px(rem)),
            None => resolve_px(theme, spec.root_gap_token()),
        };
        let inner_gap = resolve_px(theme, spec.inner_gap_token());

        let mut bar = div()
            .flex()
            .flex_wrap()
            .items_center()
            .justify_between()
            .w_full()
            .gap(root_gap)
            .px(pad_inline)
            .py(pad_block)
            .text_size(font_size)
            .text_color(text_secondary);

        // Chrome modifier: border-top + 94% panel background. Without chrome
        // the bar is transparent and blends into its container.
        if spec.chrome {
            let panel = resolve_color(theme, spec.chrome_background_token());
            let chrome_bg = color_mix(panel, gpui::transparent_black(), spec.chrome_background_opacity());
            let border = resolve_color(theme, spec.chrome_border_token());
            bar = bar.bg(chrome_bg).border_t_1().border_color(border);
        }

        // Leading region: slot content, or summary fallback when no slot.
        let mut leading = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(inner_gap);

        if let Some(leading_items) = self.leading_items {
            leading = leading.child(leading_items);
        } else if let Some(ref summary) = spec.summary {
            leading = leading.child(div().child(summary.clone()));
        }

        bar = bar.child(leading);

        // Trailing region: only rendered when trailing slot content exists.
        if let Some(trailing_items) = self.trailing_items {
            bar = bar.child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(inner_gap)
                    .child(trailing_items),
            );
        }

        bar.into_any_element()
    }
}
