//! Typed probes for the renderer-neutral node vocabulary.

mod fixtures;
mod inventory;
mod receipt;

use poodle_node::Node;
use serde::Serialize;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub use fixtures::{cases, IMAGE_ASSET, IMAGE_SVG, NAMED_ICON, NAMED_ICON_SVG};
pub use inventory::vocabulary_fingerprint;
pub use receipt::{digest, Identity, Receipt, ReceiptError, ResultEntry, Status};

/// The backend surface actually observed by a case.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Proof {
    Layout,
    Accessibility,
    Paint,
    Dispatch,
}

/// A read from the last mounted frame. Coordinates are in logical pixels
/// relative to the named target. Bounds are relative to the fixture root.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(tag = "channel", content = "query", rename_all = "snake_case")]
pub enum Probe {
    Bounds,
    A11y(&'static str),
    WiderThan(&'static str),
    ShapedText,
    Focused,
    TabStops,
    /// Test whether finished paint covers a point in the given opaque RGB color.
    Paint {
        point: [f32; 2],
        rgb: [u8; 3],
    },
    /// Visible image or icon output in the target bounds.
    Sprite(Sprite),
}

/// Asset primitive to look for in backend paint output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Sprite {
    Mask,
    Image,
}

/// A future action in a typed fixture. The pure pilot leaves dispatch pending.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Action {
    Activate(&'static str),
}

/// One exact assertion or a per-coordinate bound with tolerance in pixels.
#[derive(Debug, Serialize)]
pub struct Check {
    pub target: &'static str,
    pub probe: Probe,
    pub expected: Value,
    pub tolerance: Option<f64>,
}

impl Check {
    fn matches(&self, actual: &Value) -> bool {
        let Some(tolerance) = self.tolerance else {
            return actual == &self.expected;
        };
        let (Some(expected), Some(actual)) = (self.expected.as_array(), actual.as_array()) else {
            return false;
        };
        expected.len() == actual.len()
            && expected.iter().zip(actual).all(|(a, b)| {
                matches!((a.as_f64(), b.as_f64()), (Some(a), Some(b)) if (a - b).abs() <= tolerance)
            })
    }
}

/// An authored tree, its viewport, and the observations a backend must satisfy.
pub struct Case {
    pub id: String,
    pub proof: Proof,
    pub viewport: [f32; 2],
    pub root: Node,
    pub checks: Vec<Check>,
    pub actions: Vec<Action>,
    pub expected_events: &'static [&'static str],
    /// Event names pushed by this fixture's recording closures.
    pub recorded: Arc<Mutex<Vec<&'static str>>>,
}

/// A mounted backend's answer to a probe. Missing output is an observed null.
#[derive(Debug)]
pub enum Reading {
    Observed(Value),
    Unsupported(String),
}

/// A backend mounts the supplied tree through its production node converter
/// and answers probes from its own output.
pub trait Backend {
    fn mount(&mut self, root: &Node, viewport: [f32; 2]);
    fn read(&mut self, target: &str, probe: Probe) -> Reading;
}
