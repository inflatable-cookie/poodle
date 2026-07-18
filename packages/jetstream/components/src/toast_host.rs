//! ToastHost — Jetstream positioned toast container backed by ToastHostSpec.
//!
//! Wraps a ToastStack and positions it at the chosen screen corner.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ToastHostPlacement, ToastHostSpec, ToastStackSpec};

use crate::presentation::rem_to_px;
use crate::toast_stack::js_toast_stack;

pub fn js_toast_host(
    spec: &ToastHostSpec,
    theme: &JetstreamThemeProvider,
    stack_spec: &ToastStackSpec,
) -> JsEl {
    // Empty toasts — render nothing
    if stack_spec.toasts.is_empty() {
        return ui_element::div();
    }

    // Contract §7/§8: inset 1rem, width cap 28rem — resolved from spec
    // accessors (no raw literals). The `calc(100vw - 2rem)` viewport clamp,
    // narrow-viewport media override, and `z-index: 80` are web-only / have no
    // JsEl channel; the host is mounted last in the overlay so it stacks above
    // app chrome. (note)
    let inset = rem_to_px(spec.inset_rem());
    let width = rem_to_px(spec.width_rem());

    let mut container = ui_element::div().w(width).max_w(width);

    // Position at chosen corner using absolute positioning
    match spec.placement {
        ToastHostPlacement::BottomEnd => {
            container = container
                .absolute()
                .right(inset).bottom(inset);
        }
        ToastHostPlacement::BottomStart => {
            container = container
                .absolute()
                .left(inset).bottom(inset);
        }
        ToastHostPlacement::TopEnd => {
            container = container
                .absolute()
                .right(inset).top(inset);
        }
        ToastHostPlacement::TopStart => {
            container = container
                .absolute()
                .left(inset).top(inset);
        }
    }

    // Render inner toast stack
    let inner = js_toast_stack(stack_spec, theme);
    container = container.child(inner);

    container
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{Toast, ToastHostSpec, ToastStackSpec, ToastTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn empty_store_renders_nothing() {
        // Contract §4: host <div> is not rendered when there are no toasts.
        let host = ToastHostSpec::new();
        let stack = ToastStackSpec::new();
        let tree = probe(&js_toast_host(&host, &theme(), &stack), 600.0, 400.0);
        // Empty JsEl div → a single bare Panel node with no toast text.
        assert!(!tree.has_text("Saved"), "no toast text expected when empty");
        assert_eq!(tree.count_kind("Label"), 0, "no labels when empty: {}", tree.to_json());
    }

    #[test]
    fn stacks_multiple_toasts_with_gap() {
        // Two toasts stack vertically; both titles render and the second sits
        // below the first separated by the stack gap (contract §7).
        let host = ToastHostSpec::new();
        let stack = ToastStackSpec::new().with_toasts(vec![
            Toast::new("t1", "Saved").with_message("Your changes were saved.").with_tone(ToastTone::Success),
            Toast::new("t2", "Publishing failed.").with_tone(ToastTone::Danger),
        ]);
        let tree = probe(&js_toast_host(&host, &theme(), &stack), 600.0, 400.0);
        assert!(tree.has_text("Saved"), "first toast title missing: {:?}", tree.texts());
        assert!(tree.has_text("Publishing failed."), "second toast title missing");

        // Find the two title labels and confirm vertical stacking (distinct y).
        let titles: Vec<&crate::render_probe::ProbeNode> = tree
            .nodes
            .iter()
            .filter(|n| matches!(n.text.as_deref(), Some("Saved") | Some("Publishing failed.")))
            .collect();
        assert_eq!(titles.len(), 2, "expected 2 title labels");
        let ys: Vec<f32> = titles.iter().map(|n| n.y).collect();
        assert!((ys[0] - ys[1]).abs() > 1.0, "toasts should stack on distinct rows: {:?}", ys);
    }

    #[test]
    fn bottom_end_placement_uses_inset_width() {
        // Host container resolves the 1rem inset + 28rem width cap from the spec.
        let host = ToastHostSpec::new().with_placement(ToastHostPlacement::BottomEnd);
        let stack = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("t1", "Hello").with_tone(ToastTone::Info)]);
        let tree = probe(&js_toast_host(&host, &theme(), &stack), 600.0, 400.0);
        // Root host container is the first node; its width is capped at 28rem (448px).
        let root = &tree.nodes[0];
        assert!(
            (root.w - 448.0).abs() <= 1.0,
            "host width should be 28rem (448px), got {}",
            root.w
        );
        assert!(tree.has_text("Hello"), "toast rendered inside host");
    }

    #[test]
    fn top_start_placement_renders_toasts() {
        let host = ToastHostSpec::new().with_placement(ToastHostPlacement::TopStart);
        let stack = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("t1", "Heads up").with_tone(ToastTone::Info)]);
        let tree = probe(&js_toast_host(&host, &theme(), &stack), 600.0, 400.0);
        assert!(tree.has_text("Heads up"), "top-start host should render its toast");
    }
}
