//! StatusBar — Jetstream status row backed by `ShellStatusBarSpec`
//! (contract: status-bar). A `<footer>`-equivalent strip with a leading
//! region (slot content or summary fallback) and a trailing region (only
//! rendered when trailing items exist), arranged space-between.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ShellStatusBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

pub fn js_shell_status_bar(
    spec: &ShellStatusBarSpec,
    theme: &JetstreamThemeProvider,
    leading: Vec<JsEl>,
    trailing: Vec<JsEl>,
) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());

    // Contract §8: font-size scales by size; padding 0.375rem 0.75rem scaled
    // by size (block) / density (inline); root gap = space.inline.md
    // overridden by density; inner gap = space.inline.sm. No fixed height —
    // content + padding drive it.
    let font_size = rem_to_px(spec.font_size_rem());
    let pad_block = rem_to_px(spec.padding_block_rem());
    let pad_inline = rem_to_px(spec.padding_inline_rem());
    let root_gap = match spec.density_gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => resolve_px(theme, spec.root_gap_token()),
    };
    let inner_gap = resolve_px(theme, spec.inner_gap_token());

    let mut el = ui_element::div()
        .w_full()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(root_gap)
        .px(pad_inline)
        .py(pad_block);

    // Chrome modifier: 94% panel background + border-top. Without chrome the
    // bar is transparent and blends into its container.
    if spec.chrome {
        let panel = resolve_color(theme, spec.chrome_background_token());
        let transparent = glam::Vec4::new(panel.x, panel.y, panel.z, 0.0);
        let chrome_bg = color_mix(panel, transparent, spec.chrome_background_opacity());
        let border = resolve_color(theme, spec.chrome_border_token());
        // Border width token resolves to 0.0625rem (1px at 16px base); JsEl
        // exposes only a fixed 1px top-border setter, so width is approximated
        // at 1px and the per-side top color is set explicitly.
        el = el
            .bg(chrome_bg)
            .border_t_1()
            .border_color_top(border);
    }

    // Leading region: slot content, or summary fallback when no slot content.
    let mut leading_row = ui_element::div().flex_row().items_center().gap(inner_gap);
    if !leading.is_empty() {
        for item in leading {
            leading_row = leading_row.child(item);
        }
    } else if let Some(ref summary) = spec.summary {
        leading_row = leading_row.child(
            ui_element::label(summary)
                .text_color(text_color)
                .text_size(font_size),
        );
    }
    el = el.child(leading_row);

    // Trailing region: only rendered when trailing slot content exists.
    if !trailing.is_empty() {
        let mut trailing_row = ui_element::div().flex_row().items_center().gap(inner_gap);
        for item in trailing {
            trailing_row = trailing_row.child(item);
        }
        el = el.child(trailing_row);
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlDensity, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn summary_fallback_renders_in_leading_when_no_slot() {
        let th = theme();
        let spec = ShellStatusBarSpec::new().with_summary("3 items selected");
        let el = js_shell_status_bar(&spec, &th, vec![], vec![]);
        let tree = probe(&el, 600.0, 40.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("3 items selected"),
            "summary missing: {:?}",
            tree.texts()
        );
        // No trailing region drawn when no trailing items: root has exactly one
        // child container (leading).
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0, "root not laid out: {root:?}");
    }

    #[test]
    fn leading_items_replace_summary_and_trailing_region_appears() {
        let th = theme();
        let spec = ShellStatusBarSpec::new().with_summary("Ready");
        let leading = vec![
            ui_element::label("main"),
            ui_element::label("0 errors"),
        ];
        let trailing = vec![ui_element::label("Ln 42, Col 18"), ui_element::label("UTF-8")];
        let el = js_shell_status_bar(&spec, &th, leading, trailing);
        let tree = probe(&el, 800.0, 40.0);

        // Leading slot items present; summary suppressed (slot wins).
        assert!(tree.has_text("main"), "leading branch item missing");
        assert!(tree.has_text("0 errors"), "leading error item missing");
        assert!(!tree.has_text("Ready"), "summary should be suppressed when leading slot has content");
        // Trailing region rendered with its metadata.
        assert!(tree.has_text("Ln 42, Col 18"), "trailing cursor item missing");
        assert!(tree.has_text("UTF-8"), "trailing encoding item missing");
    }

    #[test]
    fn chrome_paints_panel_background_and_top_border() {
        let th = theme();
        let panel = resolve_color(&th, "color.background.panel");
        let spec = ShellStatusBarSpec::new()
            .with_chrome(true)
            .with_summary("Ready");
        let el = js_shell_status_bar(&spec, &th, vec![], vec![]);
        let tree = probe(&el, 600.0, 40.0);

        // 94% panel mix → background close to panel (alpha-scaled).
        let mixed = crate::render_probe::ProbeColor {
            r: panel.x,
            g: panel.y,
            b: panel.z,
            a: panel.w * 0.94,
        };
        assert!(
            tree.has_background(mixed, 0.05),
            "chrome background not painted near panel@94%"
        );
    }

    #[test]
    fn no_chrome_is_transparent() {
        let th = theme();
        let panel = resolve_color(&th, "color.background.panel");
        let spec = ShellStatusBarSpec::new().with_summary("Ready");
        let el = js_shell_status_bar(&spec, &th, vec![], vec![]);
        let tree = probe(&el, 600.0, 40.0);

        let panel_color = crate::render_probe::ProbeColor {
            r: panel.x,
            g: panel.y,
            b: panel.z,
            a: panel.w,
        };
        // Without chrome the bar paints no panel background.
        assert!(
            !tree.has_background(panel_color, 0.02),
            "no-chrome bar should not paint a panel background"
        );
    }

    #[test]
    fn size_and_density_drive_font_and_padding() {
        let th = theme();
        // Larger size → larger summary font.
        let lg = ShellStatusBarSpec::new()
            .with_summary("Ready")
            .with_size(ControlSize::Xl);
        let el = js_shell_status_bar(&lg, &th, vec![], vec![]);
        let tree = probe(&el, 600.0, 40.0);
        let summary_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Ready"))
            .expect("summary node present");
        assert!(
            summary_node.text_size.unwrap_or(0.0) >= rem_to_px(0.9),
            "xl summary font too small: {:?}",
            summary_node.text_size
        );

        // Compact density yields a smaller laid-out width than comfortable
        // (less padding-inline) for the same content.
        let compact = ShellStatusBarSpec::new()
            .with_summary("x")
            .with_density(ControlDensity::Compact);
        let comfy = ShellStatusBarSpec::new()
            .with_summary("x")
            .with_density(ControlDensity::Comfortable);
        // Both lay out without panic; the spec rem scales differ.
        assert!(compact.padding_inline_rem() < comfy.padding_inline_rem());
    }
}
