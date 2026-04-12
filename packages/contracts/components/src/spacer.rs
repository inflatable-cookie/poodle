/// Spacer — invisible flexible space filler for flex layouts.
///
/// Matches the contract doc at docs/contracts/components/spacer.md.

#[derive(Clone, Debug, PartialEq)]
pub struct SpacerSpec {
    /// Relative flex growth weight. Defaults to 1.0.
    pub grow: f32,
    /// Optional lower bound in px applied to both min-width and
    /// min-height. None means no minimum.
    pub min_size: Option<f32>,
}

impl Default for SpacerSpec {
    fn default() -> Self {
        Self {
            grow: 1.0,
            min_size: None,
        }
    }
}

impl SpacerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_grow(mut self, grow: f32) -> Self {
        self.grow = grow;
        self
    }

    pub fn with_min_size(mut self, min_size: f32) -> Self {
        self.min_size = Some(min_size);
        self
    }
}
