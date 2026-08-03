//! PaginationSummary — Jetstream pagination summary backed by PaginationSummarySpec.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, PaginationSummarySpec, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

pub fn js_pagination_summary(spec: &PaginationSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "color.text.secondary");

    // PaginationSummarySpec has no size/size_role fields — use defaults
    let effective_size =
        resolve_semantic_size(ControlSize::default(), SemanticControlSizeRole::default());
    let font_size = rem_to_px(size_font_rem(effective_size));

    // Built in the spec so all three targets say the same thing (contract §7).
    let text = spec.summary_text();

    ui_element::label(&text)
        .text_color(text_color)
        .text_size(font_size)
}
