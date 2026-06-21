use crate::{ButtonVariant, ControlSize};

#[derive(Clone, Debug)]
pub struct DebugDialogSpec {
    pub value: Option<String>,
    pub title: String,
    pub trigger_label: String,
    pub max_height: String,
    pub trigger_variant: ButtonVariant,
    pub trigger_size: Option<ControlSize>,
    pub show_close_button: bool,
    pub close_label: String,
}

impl Default for DebugDialogSpec {
    fn default() -> Self {
        Self {
            value: None,
            title: String::from("Debug data"),
            trigger_label: String::from("View debug data"),
            max_height: String::from("min(60vh, 32rem)"),
            trigger_variant: ButtonVariant::Ghost,
            trigger_size: Some(ControlSize::Sm),
            show_close_button: true,
            close_label: String::from("Close debug dialog"),
        }
    }
}

impl DebugDialogSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_trigger_label(mut self, trigger_label: impl Into<String>) -> Self {
        self.trigger_label = trigger_label.into();
        self
    }

    pub fn with_trigger_variant(mut self, trigger_variant: ButtonVariant) -> Self {
        self.trigger_variant = trigger_variant;
        self
    }

    pub fn with_trigger_size(mut self, trigger_size: Option<ControlSize>) -> Self {
        self.trigger_size = trigger_size;
        self
    }

    pub fn with_max_height(mut self, max_height: impl Into<String>) -> Self {
        self.max_height = max_height.into();
        self
    }

    pub fn with_show_close_button(mut self, show_close_button: bool) -> Self {
        self.show_close_button = show_close_button;
        self
    }

    pub fn with_close_label(mut self, close_label: impl Into<String>) -> Self {
        self.close_label = close_label.into();
        self
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Pixel value for the code block's max-height, parsed from `max_height`.
    ///
    /// Svelte's `maxHeight` is a CSS string (default `"min(60vh, 32rem)"`).
    /// The Rust Code components take a numeric px max-height, so the rem term is
    /// extracted (×16) and used as the clamp. The `vh` term is viewport-relative
    /// and owned by the centering parent, so only the rem term carries here.
    /// Returns `None` when no rem term is present (e.g. a pure `vh` value).
    pub fn max_height_px(&self) -> Option<f64> {
        let s = &self.max_height;
        let idx = s.find("rem")?;
        // Walk back from "rem" over the numeric literal (digits + dot).
        let bytes = s.as_bytes();
        let mut start = idx;
        while start > 0 {
            let c = bytes[start - 1];
            if c.is_ascii_digit() || c == b'.' {
                start -= 1;
            } else {
                break;
            }
        }
        s[start..idx].parse::<f64>().ok().map(|rem| rem * 16.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_max_height_extracts_rem_term() {
        // "min(60vh, 32rem)" → 32rem × 16 = 512px.
        assert_eq!(DebugDialogSpec::new().max_height_px(), Some(512.0));
    }

    #[test]
    fn plain_rem_max_height_parses() {
        let spec = DebugDialogSpec::new().with_max_height("24rem");
        assert_eq!(spec.max_height_px(), Some(384.0));
    }

    #[test]
    fn vh_only_max_height_yields_none() {
        let spec = DebugDialogSpec::new().with_max_height("60vh");
        assert_eq!(spec.max_height_px(), None);
    }
}
