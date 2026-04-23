//! PageHeader — Jetstream page header backed by PageHeaderSpec.
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{PageHeaderAlign, PageHeaderSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px};

/// Map a status tone icon name (mirrors the banner component convention).
fn tone_icon(tone: poodle_specs::StatusTone) -> &'static str {
    match tone {
        poodle_specs::StatusTone::Neutral | poodle_specs::StatusTone::Info => "info",
        poodle_specs::StatusTone::Success => "check-circle",
        poodle_specs::StatusTone::Warning => "alert-triangle",
        poodle_specs::StatusTone::Danger => "x-circle",
        poodle_specs::StatusTone::Pending => "loader",
    }
}

pub fn js_page_header(spec: &PageHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    // Typography scale
    let title_size = rem_to_px(size_font_rem(effective_size) + 0.4375);
    let subtitle_size = rem_to_px(size_font_rem(effective_size));
    let eyebrow_size = rem_to_px(size_font_rem(effective_size) * 0.75);
    let back_size = rem_to_px(size_font_rem(effective_size) * 0.875);
    let count_size = rem_to_px(size_font_rem(effective_size) * 0.875);
    let body_font = rem_to_px(size_font_rem(effective_size));

    // Spacing
    let gap = resolve_px(theme, spec.gap_token());
    let header_gap = resolve_px(theme, spec.header_gap_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());

    // Colors
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());
    let eyebrow_color = resolve_color(theme, spec.eyebrow_color_token());
    let back_color = resolve_color(theme, spec.back_color_token());
    let count_color = resolve_color(theme, spec.count_color_token());
    let banner_color = resolve_color(theme, spec.banner_color_token());
    let panel: Color = resolve_color(theme, "color.background.panel").into();

    let mut outer = ui_element::div().flex_col().gap(gap).pb(pad_y);

    // ── Back link ─────────────────────────────────────────────────────────────
    if spec.has_back_link() {
        if let (Some(ref label), _) = (&spec.back_label, &spec.back_href) {
            let back_row = ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.25))
                .cursor_pointer()
                .child(
                    ui_element::icon("chevron-left")
                        .w(icon_size)
                        .h(icon_size)
                        .text_color(back_color),
                )
                .child(
                    ui_element::label(label)
                        .text_color(back_color)
                        .text_size(back_size),
                );
            outer = outer.child(back_row);
        }
    }

    // ── Header row ────────────────────────────────────────────────────────────
    let mut header_row = ui_element::div().flex_row().items_center();
    header_row = match spec.align {
        PageHeaderAlign::Start => header_row.justify_start(),
        PageHeaderAlign::Between => header_row.justify_between(),
    };

    // Left column: eyebrow/section, title row, subtitle
    let mut left_col = ui_element::div().flex_col().gap(rem_to_px(0.25));

    // Eyebrow / section label (section takes precedence if both present)
    let eyebrow_text = spec.section.as_ref().or(spec.eyebrow.as_ref());
    if let Some(label) = eyebrow_text {
        left_col = left_col.child(
            ui_element::label(label)
                .text_color(eyebrow_color)
                .text_size(eyebrow_size)
                .text_weight(600),
        );
    }

    // Title row: title + optional count badge
    let mut title_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(header_gap);

    title_row = title_row.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_size)
            .text_weight(700),
    );

    if let Some(count) = spec.count {
        let count_str = format!("{}", count);
        let count_badge = ui_element::div()
            .flex_row()
            .items_center()
            .border_1()
            .border_color(resolve_color(theme, "color.border.subtle"))
            .rounded(rem_to_px(0.75))
            .pl(rem_to_px(0.375))
            .pr(rem_to_px(0.375))
            .child(
                ui_element::label(&count_str)
                    .text_color(count_color)
                    .text_size(count_size),
            );
        title_row = title_row.child(count_badge);
    }

    left_col = left_col.child(title_row);

    // Subtitle
    if let Some(ref desc) = spec.subtitle {
        left_col = left_col.child(
            ui_element::label(desc)
                .text_color(text_secondary)
                .text_size(subtitle_size),
        );
    }

    header_row = header_row.child(left_col);
    outer = outer.child(header_row);

    // ── Banner ────────────────────────────────────────────────────────────────
    if spec.has_banner() {
        if let Some(ref message) = spec.banner_message {
            let banner_fill: Color = banner_color.into();
            let tinted_fill = banner_fill.mix(panel, 0.10);

            let banner = ui_element::div()
                .bg(tinted_fill)
                .border_l_1()
                .border_color(banner_color)
                .pl(rem_to_px(0.75))
                .pr(rem_to_px(0.75))
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5))
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.5))
                .child(
                    ui_element::icon(tone_icon(spec.banner_tone))
                        .w(rem_to_px(1.0))
                        .h(rem_to_px(1.0))
                        .text_color(banner_color),
                )
                .child(
                    ui_element::label(message)
                        .text_color(text_primary)
                        .text_size(body_font)
                        .grow(),
                );

            outer = outer.child(banner);
        }
    }

    outer
}
