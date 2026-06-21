//! MetaBar — wrapping inline metadata row backed by MetaBarSpec.
//!
//! Contract: `docs/contracts/components/meta-bar.md`
//! Reference: `packages/gpui/components/src/primitives/meta_bar.rs`
//!
//! Inserts dot separators between children when show_separators is true.
//! Uses color.text.secondary (at 72% opacity) for dot color.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MetaBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius, resolve_px};

/// Separator-dot color mix: `color-mix(in srgb, text-secondary 72%, transparent)`
/// (contract §7 separator-dot `background`). No token carries the 72% factor —
/// it is a contract literal, named here rather than inlined as a bare magic number.
const SEPARATOR_DOT_MIX: f32 = 0.72;

/// Build a MetaBar element from a MetaBarSpec.
///
/// All children draw a leading separator dot when `show_separators` is on (the
/// common case). For per-child separator opt-out (contract §4 `data-separator`),
/// use [`js_meta_bar_sep`].
///
/// Contract anatomy:
/// ```text
/// [Root]   flex-row, wrap, gap space.inline.sm
///   ├── [Child]  each caller-provided item
///   └── [Dot]   0.25rem pill-radius dot (when show_separators, idx > 0, opt-in)
/// ```
pub fn js_meta_bar(spec: &MetaBarSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    js_meta_bar_sep(
        spec,
        theme,
        children.into_iter().map(|c| (c, true)).collect(),
    )
}

/// Build a MetaBar from children paired with their `data-separator` intent.
///
/// A child draws a leading dot only when it is not the first child, separators
/// are on, **and** its flag is `true` (contract §4 per-child opt-in). Callers
/// pass `false` for items that opt out (`MetaItemSpec::separator == false`) or
/// for pill-bearing children (Svelte's `:has(.poodle-pill)` suppression).
pub fn js_meta_bar_sep(
    spec: &MetaBarSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<(JsEl, bool)>,
) -> JsEl {
    let gap = resolve_px(theme, "space.inline.sm");
    let separator_color: Color = resolve_color(theme, "color.text.secondary").into();
    // Dot at 72% of text-secondary alpha (contract §7), named not inlined.
    let dot_color = Color::new(
        separator_color.r,
        separator_color.g,
        separator_color.b,
        separator_color.a * SEPARATOR_DOT_MIX,
    );
    let dot_size = rem_to_px(0.25); // 0.25rem dot
    let dot_radius = resolve_radius(theme, "radius.pill");

    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(gap)
        .min_w(0.0);

    for (idx, (child, separator)) in children.into_iter().enumerate() {
        if idx > 0 && spec.show_separators && separator {
            row = row.child(
                ui_element::div()
                    .w(dot_size)
                    .h(dot_size)
                    .rounded(dot_radius)
                    .bg(dot_color)
            );
        }
        row = row.child(
            ui_element::div()
                .min_w(0.0)
                .child(child)
        );
    }

    row
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_item::js_meta_item;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::MetaItemSpec;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn item(theme: &JetstreamThemeProvider, label: &str, value: &str) -> JsEl {
        js_meta_item(
            &MetaItemSpec::new().with_label(label),
            theme,
            Some(ui_element::label(value)),
        )
    }

    /// The expected separator-dot color: text-secondary at 72% alpha.
    fn dot_probe_color(theme: &JetstreamThemeProvider) -> ProbeColor {
        let c: Color = resolve_color(theme, "color.text.secondary").into();
        ProbeColor {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a * SEPARATOR_DOT_MIX,
        }
    }

    #[test]
    fn renders_items_with_separator_dots_between_them() {
        let theme = theme();
        let el = js_meta_bar(
            &MetaBarSpec::new(),
            &theme,
            vec![
                item(&theme, "Status", "Published"),
                item(&theme, "Format", "WAV"),
                item(&theme, "BPM", "128"),
            ],
        );
        let tree = probe(&el, 600.0, 200.0);

        // All item labels (uppercased) + values render.
        for t in ["STATUS", "Published", "FORMAT", "WAV", "BPM", "128"] {
            assert!(tree.has_text(t), "missing text {t:?}: {:?}", tree.texts());
        }
        // Two dots between three items, each text-secondary @ 72% alpha.
        assert!(
            tree.has_background(dot_probe_color(&theme), 0.02),
            "separator dot color (text-secondary 72%) missing"
        );
    }

    #[test]
    fn show_separators_false_suppresses_dots() {
        let theme = theme();
        let el = js_meta_bar(
            &MetaBarSpec::new().with_show_separators(false),
            &theme,
            vec![item(&theme, "Owner", "Tom"), item(&theme, "Size", "48 MB")],
        );
        let tree = probe(&el, 600.0, 200.0);

        assert!(tree.has_text("OWNER"), "items still render without separators");
        assert!(
            !tree.has_background(dot_probe_color(&theme), 0.02),
            "no separator dots when show_separators is false"
        );
    }

    #[test]
    fn per_child_opt_out_suppresses_its_dot() {
        let theme = theme();
        // Two items, both opting out → no dots even with separators on.
        let el = js_meta_bar_sep(
            &MetaBarSpec::new(),
            &theme,
            vec![
                (item(&theme, "A", "1"), false),
                (item(&theme, "B", "2"), false),
            ],
        );
        let tree = probe(&el, 600.0, 200.0);
        assert!(
            !tree.has_background(dot_probe_color(&theme), 0.02),
            "per-child opt-out should suppress the leading dot"
        );
    }

    #[test]
    fn first_child_never_gets_a_leading_dot() {
        let theme = theme();
        // A single item: idx 0, so no dot regardless of separators.
        let el = js_meta_bar(&MetaBarSpec::new(), &theme, vec![item(&theme, "V", "1.0")]);
        let tree = probe(&el, 600.0, 200.0);
        assert!(
            !tree.has_background(dot_probe_color(&theme), 0.02),
            "the first/only child must not draw a leading dot"
        );
    }
}
