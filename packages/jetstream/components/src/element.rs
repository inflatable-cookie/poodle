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
use std::sync::Arc;

/// A handler taking the id or value of the thing that was acted on.
///
/// Nearly every Poodle event is "this one, by id" — a row opened, a file
/// chosen, an option selected — so the components share one alias rather than
/// spelling the same `Arc<dyn Fn>` out per field.
pub type Handler = Arc<dyn Fn(&str) + Send + Sync>;

/// A handler for an event with nothing to say beyond "it happened".
///
/// A button press is the whole payload. The engine's `ClickEvent` carries
/// pointer coordinates, which are the runtime's business and not a component's.
pub type ActionHandler = Arc<dyn Fn() + Send + Sync>;

/// A handler taking the state a control is moving **to**, not the one it left.
///
/// Matches the GPUI target's `Fn(&bool, …)`. Hosts are stateless here — the
/// spec they pass in is the current state — so a handler that reported the old
/// value would make every caller re-derive the new one.
pub type ToggleHandler = Arc<dyn Fn(bool) + Send + Sync>;

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

    /// Click the centre of the one node whose text is `text`.
    ///
    /// Preferred over `click_at`: a test that says "click the toggle" keeps
    /// meaning that when a padding token changes, where one that says "click at
    /// y=52" quietly starts hitting the row above.
    ///
    /// **Ambiguity is an error.** A picker shows its selected model's label in
    /// the trigger *and* in the panel; taking the first match clicked the
    /// trigger, which has no handler, and the test read as "the wiring is
    /// broken" when the wiring was fine. Use `click_text_nth` where a repeat is
    /// deliberate.
    pub fn click_text(el: &JsEl, width: f32, height: f32, text: &str) {
        let tree = crate::render_probe::probe(el, width, height);
        let matches = tree
            .nodes
            .iter()
            .filter(|node| node.text.as_deref() == Some(text))
            .count();

        assert!(matches > 0, "no node with text {text:?} to click. present: {:?}", tree.texts());
        assert_eq!(
            matches, 1,
            "{matches} nodes carry the text {text:?}, so this click is ambiguous — \
             use click_text_nth to say which one"
        );

        click_text_nth(el, width, height, text, 0);
    }

    /// Click the centre of the `index`-th node whose text is `text`.
    pub fn click_text_nth(el: &JsEl, width: f32, height: f32, text: &str, index: usize) {
        let tree = crate::render_probe::probe(el, width, height);
        let node = tree
            .nodes
            .iter()
            .filter(|node| node.text.as_deref() == Some(text))
            .nth(index)
            .unwrap_or_else(|| {
                panic!("no node {index} with text {text:?}. present: {:?}", tree.texts())
            });

        click_at(el, width, height, node.x + node.w / 2.0, node.y + node.h / 2.0);
    }

    /// Press at `from`, move to `to`, release — driving a real drag.
    ///
    /// The runtime only starts a drag once the pointer has travelled past its
    /// threshold, and only reports `Move` deltas between successive frames, so
    /// this walks the distance in steps rather than teleporting: one frame at
    /// the far end would exceed the threshold and deliver the whole distance as
    /// a single delta, which is not what a real pointer does.
    ///
    /// A completed drag suppresses the click, so a component with both a drag
    /// and a click handler gets exactly one of them — same as under a real
    /// pointer.
    pub fn drag(el: &JsEl, width: f32, height: f32, from: (f32, f32), to: (f32, f32)) {
        const STEPS: usize = 8;

        let mut ui = GameUi::new(width, height);
        ui.active = true;
        ui.render_immediate(el);

        let mut input = InputSystem::new();
        let button = |input: &mut InputSystem, down: bool| {
            input.process_events(&[PlatformEvent::MouseButton {
                button: MouseButton::Left,
                pressed: down,
            }]);
        };

        // Down on the source, then walk to the target one step at a time.
        button(&mut input, true);
        ui.process_input(&input, from.0, from.1);

        for step in 1..=STEPS {
            let t = step as f32 / STEPS as f32;
            ui.process_input(
                &input,
                from.0 + (to.0 - from.0) * t,
                from.1 + (to.1 - from.1) * t,
            );
        }

        button(&mut input, false);
        ui.process_input(&input, to.0, to.1);
    }

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
