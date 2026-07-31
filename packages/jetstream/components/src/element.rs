//! The shape every Jetstream component takes.
//!
//! Components are builder structs, not free functions, and they mirror the GPUI
//! target's naming and verbs exactly — `Component::from_spec(spec, theme)` then
//! `.on_x(handler)`. A developer moving between the two native targets should
//! not have to learn a second vocabulary for the same component.
//!
//! ## Why not free functions
//!
//! `js_button(spec, theme) -> JsEl` had nowhere to put a handler. Every option
//! was bad: closures on the spec break its `Clone`/`Debug`/`PartialEq` derives,
//! extra parameters break every call site whenever one component gains an
//! event, and parallel `*_interactive` functions double a 151-component
//! surface. The result was that no Jetstream component was ever interactive —
//! not because the runtime could not dispatch a click, but because there was
//! nowhere to hang one.
//!
//! A builder makes handlers additive: a component gains an event without any
//! existing caller changing.
//!
//! ## Why handlers are `Send + Sync`
//!
//! `JsEl::on_click` requires it, so a Poodle handler cannot borrow host state
//! the way GPUI's `cx.listener` closures do. Hosts capture an `Arc` — of a
//! channel sender, a `Mutex`ed model, an atomic — which is the ordinary shape
//! for an immediate-mode UI, where the tree is rebuilt every frame and nothing
//! outlives it.
//!
//! ## Why this is testable
//!
//! Jetstream dispatches clicks through `GameUi` with no window, so a component's
//! interaction is provable in an ordinary unit test — see `click_probe`. That is
//! not true of the GPUI target, where a real click needs a live window, and it
//! is the reason this design puts handlers somewhere a test can reach them.

use jetstream_ui::ui_element::JsEl;

/// Anything that can become an element tree.
///
/// The mirror of GPUI's `IntoElement`. Parents take `impl IntoJsEl` so a
/// builder can be passed wherever an element is wanted, without the caller
/// having to remember which one they are holding.
pub trait IntoJsEl {
    fn into_js_el(self) -> JsEl;
}

impl IntoJsEl for JsEl {
    fn into_js_el(self) -> JsEl {
        self
    }
}

#[cfg(test)]
pub(crate) mod click_probe {
    //! Drive a real click at a point and return what the handlers recorded.
    //!
    //! This is the whole argument for the builder shape: an interaction that can
    //! be asserted rather than eyeballed. `Stepper` on GPUI carried two handlers
    //! that were attached to nothing for weeks precisely because nothing could
    //! test them.

    use jetstream_ui::ui_element::JsEl;
    use jetstream_ui::GameUi;
    use jetstream_input::InputSystem;
    use jetstream_platform::{MouseButton, PlatformEvent};

    /// Render `el`, click at `(x, y)`, and let the handlers run.
    pub fn click_at(el: &JsEl, width: f32, height: f32, x: f32, y: f32) {
        let mut ui = GameUi::new(width, height);
        ui.active = true;
        ui.render_immediate(el);

        let mut input = InputSystem::new();
        let press = |input: &mut InputSystem, down: bool| {
            input.process_events(&[PlatformEvent::MouseButton {
                button: MouseButton::Left,
                pressed: down,
            }]);
        };

        // Down then up over the same point: a click is the pair, and dispatch
        // only fires on release.
        press(&mut input, true);
        ui.process_input(&input, x, y);
        press(&mut input, false);
        ui.process_input(&input, x, y);
    }
}
