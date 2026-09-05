//! The consumer-compatibility proof for g16.005.
//!
//! Every line below fails to compile if `poodle-gpui-node-backend` exposes a
//! GPUI crate identity other than the crates.io `gpui-unofficial` this crate
//! declares for itself. The rustc lib name is still `gpui`; the package
//! identity Longhorn and Nucleus must declare is `gpui-unofficial`.
//!
//! This is a type-identity proof, not a rendering test. It opens no window,
//! captures nothing, and needs no window server.

use gpui::{
    AnyElement, Bounds, Context, FocusHandle, Hsla, IntoElement, ParentElement, Pixels, Render,
    Styled, Window, div,
};
use poodle_gpui_node_backend::{bounds_for, color, focus_handle_for, to_gpui};
use poodle_node::{ColorValue, Node};

/// Poodle → consumer, the direction Longhorn's prototypes needed: an element
/// Poodle builds is annotated with THIS crate's `gpui::AnyElement`.
fn poodle_element(node: &Node) -> AnyElement {
    to_gpui(node)
}

/// The same for the scalar types a consumer threads through its own theme and
/// layout code.
fn poodle_colour(value: ColorValue) -> Hsla {
    color(value)
}

fn poodle_focus(id: &str) -> Option<FocusHandle> {
    focus_handle_for(id)
}

fn poodle_bounds(id: &str) -> Option<Bounds<Pixels>> {
    bounds_for(id)
}

/// Consumer → Poodle: a Poodle element is composed INTO a tree this crate
/// builds with its own `gpui`. `ParentElement::child` takes this crate's
/// `IntoElement`, so a divergent identity is rejected here even if the
/// annotations above somehow were not.
struct ConsumerRoot {
    node: Node,
}

impl Render for ConsumerRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(poodle_colour(ColorValue(0.1, 0.2, 0.3, 1.0)))
            .child(poodle_element(&self.node))
    }
}

fn main() {
    // Never run in the proof; referencing them is what compiles.
    let _ = ConsumerRoot {
        node: Node::button("ok"),
    };
    let _ = poodle_focus("consumer:probe");
    let _ = poodle_bounds("consumer:probe");
    println!("crates.io gpui-unofficial and poodle-gpui-node-backend share one crate identity");
}
