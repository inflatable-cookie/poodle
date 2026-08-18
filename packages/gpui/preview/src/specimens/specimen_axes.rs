//! Axis admission for the preview-local specimen layout.
//!
//! The merged web census (`g15.017`) decides which axis panes a catalogue page
//! may show: `Sizes` exactly when the component takes `size`, `Densities`
//! exactly when it takes `density`. This module holds that decision on its own,
//! with no GPUI or preview-crate dependency, so the focused native regressions
//! can exercise it headlessly.
//!
//! Two rules live here and nowhere else:
//!
//! - A pane the caller did not admit produces no tab, so no pane can be empty.
//! - A retained tab that is no longer available normalises back to `Examples`,
//!   which is why the preview cannot strand a page on a vanished pane.

use poodle_specs::{ControlDensity, ControlSize};

/// Tab identifier for the curated Examples pane. Always available.
pub const EXAMPLES_TAB: &str = "examples";
/// Tab identifier for the size sweep.
pub const SIZES_TAB: &str = "sizes";
/// Tab identifier for the density sweep.
pub const DENSITIES_TAB: &str = "densities";

/// Every `ControlSize` step a Sizes pane walks, in catalogue order.
pub const ALL_SIZES: &[(ControlSize, &str)] = &[
    (ControlSize::Xs, "xs"),
    (ControlSize::Sm, "sm"),
    (ControlSize::Md, "md"),
    (ControlSize::Lg, "lg"),
    (ControlSize::Xl, "xl"),
];

/// Every `ControlDensity` step a Densities pane walks, in catalogue order.
pub const ALL_DENSITIES: &[(ControlDensity, &str)] = &[
    (ControlDensity::Compact, "compact"),
    (ControlDensity::Default, "default"),
    (ControlDensity::Comfortable, "comfortable"),
];

/// Which axis panes a specimen page admits.
///
/// There is no `Default`: a page states its axes or gets Examples only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AxisAdmission {
    pub sizes: bool,
    pub densities: bool,
}

impl AxisAdmission {
    /// The tab strip for this admission: `(value, label)` pairs.
    pub fn tabs(self) -> Vec<(&'static str, &'static str)> {
        let mut tabs = vec![(EXAMPLES_TAB, "Examples")];
        if self.sizes {
            tabs.push((SIZES_TAB, "Sizes"));
        }
        if self.densities {
            tabs.push((DENSITIES_TAB, "Densities"));
        }
        tabs
    }

    /// The tab to render given whatever the page last stored.
    ///
    /// A retained selection that this admission no longer allows falls back to
    /// Examples rather than rendering a blank pane.
    pub fn resolve_tab(self, stored: Option<&str>) -> &'static str {
        match stored {
            Some(SIZES_TAB) if self.sizes => SIZES_TAB,
            Some(DENSITIES_TAB) if self.densities => DENSITIES_TAB,
            _ => EXAMPLES_TAB,
        }
    }
}

/// Stable label for a size step. Specimen pages use it to key per-row element
/// ids so two rows in the same pane cannot collide.
pub fn size_key(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Xs => "xs",
        ControlSize::Sm => "sm",
        ControlSize::Md => "md",
        ControlSize::Lg => "lg",
        ControlSize::Xl => "xl",
    }
}

/// Stable label for a density step. See [`size_key`].
pub fn density_key(density: ControlDensity) -> &'static str {
    match density {
        ControlDensity::Compact => "compact",
        ControlDensity::Default => "default",
        ControlDensity::Comfortable => "comfortable",
    }
}
