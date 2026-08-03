//! ToolCall — one row of agent work, backed by `ToolCallSpec`.
//!
//! Contract: `docs/contracts/components/tool-call.md`. Every dimension resolves
//! from the spec's ladder; the only literal is the hairline the contract states
//! as an absolute.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ToolCallSpec;

use std::sync::Arc;

use crate::element::{Handler, IntoJsEl};
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

/// ToolCall — one row of agent work.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_x(handler)`. See
/// `element.rs` for why components are builders rather than free functions.
pub struct ToolCall {
    spec: ToolCallSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<Handler>,
}

impl ToolCall {
    pub fn from_spec(spec: ToolCallSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    /// Fires when the row is opened or closed. A row with no output is not
    /// interactive at all, so nothing is attached to it.
    pub fn on_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }
}

impl IntoJsEl for ToolCall {
    fn into_js_el(self) -> JsEl {
        let row = js_tool_call(&self.spec, &self.theme);

        // Only a row with output can be opened, so only that row is clickable.
        match (self.spec.has_output(), self.on_toggle) {
            (true, Some(handler)) => {
                let id = self.spec.id.clone();
                row.cursor_pointer().on_click(move |_event| handler(&id))
            }
            _ => row,
        }
    }
}

pub fn js_tool_call(spec: &ToolCallSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let label_color: Color = resolve_color(theme, spec.label_token()).into();
    let detail_color: Color = resolve_color(theme, spec.detail_token()).into();
    let icon_color: Color = resolve_color(theme, spec.icon_token()).into();
    let success: Color = resolve_color(theme, spec.success_token()).into();
    let danger: Color = resolve_color(theme, spec.danger_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let row_height = rem_to_px(spec.row_height_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());

    // Only the label takes the danger colour, never the detail. The detail is
    // already the dimmest thing in the row, and colouring it red as well makes a
    // failed row read as a block of alarm rather than a line you can scan.
    let label_color = match spec.status {
        ToolCallStatus::Error => danger,
        _ => label_color,
    };
    let status_color = match spec.status {
        ToolCallStatus::Error => danger,
        ToolCallStatus::Success => success,
        ToolCallStatus::Running => icon_color,
    };

    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .min_h(row_height)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .rounded(radius)
        .child(
            ui_element::icon(spec.resolved_icon())
                .w(icon_size)
                .h(icon_size)
                .text_color(icon_color),
        )
        .child(
            ui_element::label(spec.label.clone())
                .text_size(font_size)
                .text_color(label_color)
                .flex_shrink_0(),
        );

    if let Some(detail) = &spec.detail {
        // `min_w_0` is load-bearing: without it the detail refuses to shrink
        // below its content width and a long command pushes the status
        // indicator out of the row.
        row = row.child(
            ui_element::label(detail.clone())
                .text_size(font_size)
                .text_color(detail_color)
                .opacity(crate::theme_ext::resolve_opacity(
                    theme,
                    spec.detail_opacity_token(),
                ))
                .grow()
                .min_w_0(),
        );
    } else {
        row = row.child(ui_element::div().grow());
    }

    if spec.has_output() {
        row = row.child(
            ui_element::icon("chevron-down")
                .w(icon_size)
                .h(icon_size)
                .text_color(detail_color),
        );
    }

    row = row.child(
        ui_element::icon(spec.status_icon())
            .w(icon_size)
            .h(icon_size)
            .text_color(status_color)
            .flex_shrink_0(),
    );

    // Status reaches assistive technology through the name; colour and glyph do
    // not.
    let mut root = ui_element::div()
        .id(spec.id.clone())
        .flex_col()
        .w_full()
        .aria_role(jetstream_ui::accesskit::Role::ListItem)
        .aria_label(spec.accessible_name())
        .child(row);

    if spec.has_output() && spec.is_expanded {
        if let Some(output) = &spec.output {
            root = root.child(
                ui_element::label(output.clone())
                    .text_size(font_size)
                    .text_color(detail_color)
                    .pl(pad_x + icon_size + gap),
            );
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// The reason the builder shape exists: a click can be driven and asserted.
    ///
    /// GPUI cannot do this — a real click there needs a live window — which is
    /// how `Stepper` carried two handlers attached to nothing for weeks.
    #[test]
    fn a_click_reaches_the_handler() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = ToolCallSpec::new("call-1", "Ran command")
            .with_detail("bun test")
            .with_output("272 pass");

        let el = crate::element::IntoJsEl::into_js_el(
            ToolCall::from_spec(spec, &theme()).on_toggle(move |id| {
                assert_eq!(id, "call-1", "the handler is told which row was opened");
                counter.fetch_add(1, Ordering::SeqCst);
            }),
        );

        crate::element::click_probe::click_at(&el, 720.0, 200.0, 40.0, 12.0);

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_toggle fired exactly once"
        );
    }

    /// A row with nothing to open must not be clickable, so the absence has to
    /// be provable too.
    #[test]
    fn a_row_without_output_ignores_clicks() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = ToolCallSpec::new("call-2", "Ran command").with_detail("no output");
        let el = crate::element::IntoJsEl::into_js_el(
            ToolCall::from_spec(spec, &theme()).on_toggle(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
            }),
        );

        crate::element::click_probe::click_at(&el, 720.0, 200.0, 40.0, 12.0);

        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "a row with no output is not interactive"
        );
    }

    #[test]
    fn the_detail_reaches_the_render() {
        let spec = ToolCallSpec::new("a", "Ran command").with_detail("bun test");
        let tree = crate::render_probe::probe(&js_tool_call(&spec, &theme()), 720.0, 64.0);

        assert!(tree.has_text("Ran command"), "{:?}", tree.texts());
        assert!(tree.has_text("bun test"), "{:?}", tree.texts());
    }

    #[test]
    fn output_is_only_built_when_open() {
        let base = ToolCallSpec::new("a", "Ran command").with_output("272 pass");
        let closed = crate::render_probe::probe(&js_tool_call(&base, &theme()), 720.0, 64.0);
        let open = crate::render_probe::probe(
            &js_tool_call(&base.clone().with_expanded(true), &theme()),
            720.0,
            128.0,
        );

        assert!(!closed.has_text("272 pass"), "{:?}", closed.texts());
        assert!(open.has_text("272 pass"), "{:?}", open.texts());
    }
}
