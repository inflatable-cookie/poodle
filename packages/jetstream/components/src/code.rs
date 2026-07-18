//! Code — Jetstream code display backed by CodeSpec.
//!
//! Contract: `docs/contracts/components/code.md`
//! Reference: `packages/svelte/components/src/Code.svelte`
//! Mirrors the GPUI build-out (`packages/gpui/components/src/primitives/code.rs`).
//!
//! Block mode: transparent root + border (the visible surface lives on the
//! scroll/pre area), optional toolbar (language label + copy button composing
//! the Icon), line-numbers gutter, highlighted-line ±1rem bleed, and a
//! max-height vertical scroll container. Inline mode: a bare/chromed fragment
//! at `1em`/body × adjustmentRatio (the new `inline_variant`/`typography` spec
//! fields). ALL geometry/colour resolves from tokens. Clipboard write + the 2s
//! copied-state swap is a preview-loop interaction (the copy button is rendered
//! but inert here).

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CodeInlineVariant, CodeSpec, CodeTypography};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_code(spec: &CodeSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Shared token resolution ───────────────────────────────
    let text_color: Color = resolve_color(theme, spec.text_color_token()).into();
    let text_secondary: Color = resolve_color(theme, spec.text_secondary_token()).into();
    let panel: Color = resolve_color(theme, spec.panel_token()).into();
    let elevated: Color = resolve_color(theme, spec.elevated_token()).into();
    let canvas: Color = resolve_color(theme, spec.canvas_token()).into();
    let accent: Color = resolve_color(theme, spec.accent_token()).into();

    // ── Inline mode: a span-like fragment ─────────────────────
    if spec.is_inline {
        // Font: per-size em scale × code adjustmentRatio (contract §8).
        // `typography="inline"` uses 1em × ratio instead of the body scale.
        // The ratio is a typed f32 token (no string-resolver channel for plain
        // float tokens — read the typed constant directly, as GPUI does).
        let ratio = poodle_tokens::typed::semantic::TYPOGRAPHY_CODE_ADJUSTMENT_RATIO;
        let base_em = match spec.typography {
            CodeTypography::Inline => 1.0,
            CodeTypography::Body => size_font_rem(effective_size),
        };
        let inline_font = rem_to_px(base_em * ratio);

        // Inline `<code>` uses `--poodle-typography-code-family` (contract §8).
        let mut el = ui_element::label(&spec.content)
            .text_size(inline_font)
            .text_color(text_color)
            .font_family(FontFamily::Mono)
            .whitespace_nowrap();

        // `Default` carries inline chrome; `Plain` drops padding/radius/bg
        // (contract §8 `[data-inline-variant="plain"]`).
        if spec.inline_variant == CodeInlineVariant::Default {
            // bg = color-mix(panel 72%, elevated).
            let inline_bg = panel.mix_srgb(elevated, 0.72);
            el = el
                .pl(rem_to_px(0.375))
                .pr(rem_to_px(0.375))
                .pt(rem_to_px(0.125))
                .pb(rem_to_px(0.125))
                .rounded(rem_to_px(0.25))
                .bg(inline_bg);
        }

        return el;
    }

    // ── Block mode ────────────────────────────────────────────
    // Pre padding (density-adjusted) — contract values match the panel space
    // scale exactly (default 0.75rem × 1rem, compact 0.5 × 0.75, etc).
    let pre_pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pre_pad_y = rem_to_px(panel_space_y_rem(spec.density));
    // Source font: contract 0.8125rem; size variants scale it. Line-height 1.4.
    let source_font = rem_to_px(size_font_rem(effective_size));
    let source_line_height = rem_to_px(size_font_rem(effective_size) * 1.4);

    // Block root: border-subtle (plain token via spec), surface radius, and NO
    // background — the visible surface lives on the pre/scroll area.
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let radius = resolve_radius(theme, spec.surface_radius_token());
    let border_width = rem_to_px(0.0625); // contract: 0.0625rem solid

    let mut root = ui_element::div()
        .flex_col()
        .rounded(radius)
        .border(border_width)
        .border_color(border)
        .text_color(text_color)
        .overflow_hidden();

    // ── Toolbar (language label + copy button) ────────────────
    let has_toolbar = spec.language.is_some() || spec.is_copyable;
    if has_toolbar {
        // bg = color-mix(elevated 60%, panel); border-bottom = border-subtle.
        let toolbar_bg = elevated.mix_srgb(panel, 0.60);
        let mut toolbar = ui_element::div()
            .flex_row()
            .items_center()
            .justify_between()
            // Contract toolbar padding: 0.375rem 0.625rem.
            .pl(rem_to_px(0.625))
            .pr(rem_to_px(0.625))
            .pt(rem_to_px(0.375))
            .pb(rem_to_px(0.375))
            .bg(toolbar_bg)
            .border_b_1()
            .border_color(border);

        // Language label: 0.6875rem, label-weight (500 = medium), uppercase,
        // letter-spacing 0.05em (contract §8). Programmatic uppercase (JsEl has
        // no text-transform).
        if let Some(ref lang) = spec.language {
            toolbar = toolbar.child(
                ui_element::label(lang.to_uppercase())
                    .text_size(rem_to_px(0.6875))
                    .text_color(text_secondary)
                    .text_weight(500)
                    .letter_spacing_em(0.05),
            );
        } else {
            // Spacer keeps the actions group right-aligned (margin-left: auto).
            toolbar = toolbar.child(ui_element::div());
        }

        // Copy button — bespoke 1.5rem square (contract §8 `.code__copy`),
        // transparent, text-secondary 0.875rem icon. Composes the engine SVG
        // icon. Clipboard write + 2s check-icon swap is a preview-loop
        // interaction (render-only here). JsEl has no clipboard channel.
        if spec.is_copyable {
            toolbar = toolbar.child(
                ui_element::div()
                    .id("poodle-code-copy")
                    .w(rem_to_px(1.5))
                    .h(rem_to_px(1.5))
                    .rounded(rem_to_px(0.25))
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .text_color(text_secondary)
                    .cursor_pointer()
                    .child(
                        ui_element::icon("copy")
                            .w(rem_to_px(0.875))
                            .h(rem_to_px(0.875))
                            .text_color(text_secondary),
                    ),
            );
        }

        root = root.child(toolbar);
    }

    // ── Code surface (scroll + pre + source) ──────────────────
    // bg = color-mix(canvas 92%, black) — the pre carries the visible surface.
    let pre_bg = canvas.mix_srgb(Color::BLACK, 0.92);
    // Line-highlight bg = color-mix(accent-base 12%, transparent).
    let highlight_bg = accent.with_alpha(accent.a * 0.12);

    let mut scroll = ui_element::div()
        .id("poodle-code-scroll")
        .flex_col()
        .bg(pre_bg)
        .pl(pre_pad_x)
        .pr(pre_pad_x)
        .pt(pre_pad_y)
        .pb(pre_pad_y)
        .text_size(source_font)
        .line_height(source_line_height)
        // Horizontal overflow scrolls (contract §7 `.code__scroll`).
        .overflow_scroll();

    // max-height → vertical scroll container (contract §7). JsEl's
    // `overflow_scroll` covers both axes; `max_h` constrains the height.
    if let Some(mh) = spec.max_height {
        scroll = scroll.max_h(mh as f32);
    }

    let needs_per_line = spec.show_line_numbers || !spec.highlight_lines.is_empty();

    if needs_per_line {
        // Per-line structure: each `.code__line` is a row with an optional
        // gutter number + line content. Highlighted lines bleed ±1rem.
        for (i, line) in spec.content.lines().enumerate() {
            let line_no = i + 1;
            let is_highlighted = spec.highlight_lines.contains(&line_no);

            let mut row = ui_element::div().flex_row().items_start();

            // Contract: highlighted line bleeds ±1rem so the accent band reaches
            // the pre's content edges (negative margin + matching padding).
            if is_highlighted {
                row = row
                    .bg(highlight_bg)
                    .ml(rem_to_px(-1.0))
                    .mr(rem_to_px(-1.0))
                    .pl(rem_to_px(1.0))
                    .pr(rem_to_px(1.0));
            }

            if spec.show_line_numbers {
                // Gutter: 2.5rem wide, right-aligned, padding-right 1rem,
                // text-secondary (tabular-nums has no JsEl channel — note).
                // Gutter lives inside `.code__source` (code-family); JsEl has no
                // inheritance so set Mono explicitly.
                row = row.child(
                    ui_element::label(line_no.to_string())
                        .w(rem_to_px(2.5))
                        .pr(rem_to_px(1.0))
                        .text_color(text_secondary)
                        .font_family(FontFamily::Mono)
                        .text_right(),
                );
            }

            // `.code__source` font-family is code-family (contract §8).
            row = row.child(
                ui_element::label(line.to_string())
                    .font_family(FontFamily::Mono)
                    .whitespace_nowrap(),
            );
            scroll = scroll.child(row);
        }
    } else {
        // No per-line chrome: render the source as a single block.
        // `.code__source` font-family is code-family (contract §8).
        scroll = scroll.child(ui_element::label(&spec.content).font_family(FontFamily::Mono));
    }

    root = root.child(scroll);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn block_renders_toolbar_line_numbers_and_copy() {
        let spec = CodeSpec::new()
            .with_content("let a = 1;\nlet b = 2;\nlet c = 3;")
            .with_language("rust")
            .with_show_line_numbers(true)
            .with_copyable(true);
        let el = js_code(&spec, &theme());
        let tree = probe(&el, 600.0, 400.0);

        // Language label is rendered uppercase in the toolbar.
        assert!(
            tree.has_text("RUST"),
            "language label missing: {:?}",
            tree.texts()
        );
        // Copy button is present (its id is the hit-test key).
        assert!(
            tree.find_token("poodle-code-copy").is_some(),
            "copy button missing from toolbar"
        );
        // Line-number gutter renders one entry per line.
        assert!(tree.has_text("1"), "line number 1 missing");
        assert!(tree.has_text("2"), "line number 2 missing");
        assert!(tree.has_text("3"), "line number 3 missing");
    }

    #[test]
    fn highlighted_line_gets_accent_background() {
        let accent: Color = resolve_color(&theme(), CodeSpec::new().accent_token()).into();
        let expected = crate::render_probe::ProbeColor {
            r: accent.r,
            g: accent.g,
            b: accent.b,
            a: accent.a * 0.12,
        };
        let spec = CodeSpec::new()
            .with_content("one\ntwo\nthree")
            .with_highlight_lines(vec![2]);
        let el = js_code(&spec, &theme());
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            tree.has_background(expected, 0.02),
            "highlighted line missing accent-12% background"
        );
    }

    #[test]
    fn inline_mode_has_no_block_chrome() {
        let spec = CodeSpec::new().with_content("npm install").with_inline(true);
        let el = js_code(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);

        // Inline renders the content directly with no toolbar/copy chrome.
        assert!(
            tree.has_text("npm install"),
            "inline content missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.find_token("poodle-code-copy").is_none(),
            "inline mode should not render the block copy button"
        );
        assert!(
            tree.find_token("poodle-code-scroll").is_none(),
            "inline mode should not render the block scroll container"
        );
    }

    #[test]
    fn inline_plain_drops_background() {
        let default_inline = js_code(
            &CodeSpec::new().with_content("x").with_inline(true),
            &theme(),
        );
        let plain_inline = js_code(
            &CodeSpec::new()
                .with_content("x")
                .with_inline(true)
                .with_inline_variant(CodeInlineVariant::Plain),
            &theme(),
        );
        // Default inline carries a chrome background; plain drops it.
        assert!(
            default_inline.style.background.is_some(),
            "default inline should have a background"
        );
        assert!(
            plain_inline.style.background.is_none(),
            "plain inline should drop the background"
        );
    }
}
