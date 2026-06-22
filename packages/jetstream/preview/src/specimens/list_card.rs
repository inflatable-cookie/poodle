//! ListCard specimen — card items for list views.
//!
//! Mirrors `packages/gpui/preview/src/specimens/list_card.rs` to the extent the
//! Jetstream `js_list_card` API allows. Every card is a real
//! `js_list_card(&ListCardSpec, theme)` resolving all visuals from tokens — no
//! hand-rolled boxes.
//!
//! API gaps vs GPUI: the Jetstream `js_list_card` has no leading-snippet,
//! trailing-snippet, or footer-snippet parameters (the leading box is derived
//! from the title's first letter). So the GPUI groups that compose a custom
//! leading icon, a trailing `Pill`, or footer `ListCardCounter`s — "With badges",
//! "Footer counters", "With context menu" — cannot be rendered through the real
//! component and are intentionally omitted rather than faked. Every other
//! contract group is covered through the spec builders.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::list_card::js_list_card;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{LeadingFill, LeadingShape, ListCardLayout, ListCardSpec, SelectionIndicator};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // -- Interactive list cards (interactive + disabled) --
        .child(group("Interactive list cards", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("design-system-v2.figma")
                        .with_subtitle("Updated by Clay \u{00b7} 2h ago")
                        .with_meta("14.2 MB")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("component-specs.pdf")
                        .with_subtitle("Shared with team \u{00b7} Yesterday")
                        .with_meta("2.8 MB")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("brand-assets.zip")
                        .with_subtitle("Archived")
                        .with_meta("48 MB")
                        .with_disabled(true),
                    theme,
                )),
        ))
        // -- Rounded-square leading (thumbnails) --
        .child(group("Rounded-square leading (thumbnails)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("hero-banner.png")
                        .with_subtitle("Uploaded by Jamie \u{00b7} 4h ago")
                        .with_meta("3.1 MB")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("onboarding-flow.mp4")
                        .with_subtitle("Screen recording \u{00b7} Today")
                        .with_meta("128 MB")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Solid fill with accent colors --
        .child(group("Solid fill with accent colors", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Design Tokens")
                        .with_subtitle("Color system")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Typography")
                        .with_subtitle("Font scales & families")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#ec4899"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Spacing")
                        .with_subtitle("Layout grid & spacing tokens")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#10b981"),
                    theme,
                )),
        ))
        // -- Not live (dashed border, interactive) --
        .child(group("Not live (reduced opacity, interactive)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Unpublished Draft")
                        .with_subtitle("Last edited 3 days ago")
                        .with_not_live(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Scheduled Post")
                        .with_subtitle("Publishes tomorrow at 9 AM")
                        .with_not_live(true)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Corner sash badges --
        .child(group("Corner sash badges", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Free tier plan")
                        .with_subtitle("No credit card required")
                        .with_sash("Free")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Premium integration")
                        .with_subtitle("Unlocks advanced features")
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1")
                        .with_sash("New")
                        .with_sash_color("#6366f1")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Legacy connector")
                        .with_subtitle("Deprecated \u{2014} migrate by Q2")
                        .with_sash("EOL")
                        .with_sash_color("#ef4444")
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Selectable (multi-select) --
        .child(group("Selectable (multi-select)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Alice Chen")
                        .with_subtitle("alice@example.com")
                        .with_selectable(true)
                        .with_selected(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Bob Martinez")
                        .with_subtitle("bob@example.com")
                        .with_selectable(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Carol Patel")
                        .with_subtitle("carol@example.com")
                        .with_selectable(true)
                        .with_selected(true),
                    theme,
                )),
        ))
        // -- Reorder handle --
        .child(group("Reorder handle", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("design-system")
                        .with_subtitle("Primary rubric")
                        .with_reorder_handle(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("accessibility")
                        .with_subtitle("WCAG AA baseline")
                        .with_reorder_handle(true),
                    theme,
                )),
        ))
        // -- Link roots (href) --
        .child(group("Link roots (href)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Billing settings")
                        .with_subtitle("Manage invoices and payment methods")
                        .with_href("#billing"),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Documentation portal")
                        .with_subtitle("Opens the external guide")
                        .with_meta("docs.example.com")
                        .with_href("https://example.com/docs"),
                    theme,
                )),
        ))
        // -- Highlighted (accent emphasis) --
        .child(group("Highlighted (accent emphasis)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Active selection")
                        .with_subtitle("Accent-tinted border and inset ring")
                        .with_meta("Now")
                        .with_interactive(true)
                        .with_highlighted(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Highlighted with custom accent")
                        .with_subtitle("Accent gradient over the fill")
                        .with_interactive(true)
                        .with_highlighted(true)
                        .with_accent_color("#6366f1"),
                    theme,
                )),
        ))
        // -- Selection indicator (checkbox) --
        .child(group("Selection indicator (checkbox)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Selected row")
                        .with_subtitle("Checkbox indicator, checked")
                        .with_selectable(true)
                        .with_selected(true)
                        .with_selection_indicator(SelectionIndicator::Checkbox),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Unselected row")
                        .with_subtitle("Checkbox indicator, unchecked")
                        .with_selectable(true)
                        .with_selection_indicator(SelectionIndicator::Checkbox),
                    theme,
                )),
        ))
        // -- Layout (default / compact / stacked) --
        .child(group("Layout (default / compact / stacked)", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Default layout")
                        .with_subtitle("Standard leading + body row")
                        .with_meta("2.0rem")
                        .with_layout(ListCardLayout::Default)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Compact layout")
                        .with_subtitle("Denser \u{2014} smaller leading box")
                        .with_meta("1.75rem")
                        .with_layout(ListCardLayout::Compact)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Stacked layout")
                        .with_subtitle("Leading on top, body below, bottom rail")
                        .with_layout(ListCardLayout::Stacked)
                        .with_leading_shape(LeadingShape::RoundedSquare)
                        .with_leading_fill(LeadingFill::Solid)
                        .with_accent_color("#6366f1")
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Leading size offset --
        .child(group("Leading size offset", secondary,
            div().flex_col().gap(6.0)
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Default leading")
                        .with_subtitle("Matches the card size ladder")
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Offset leading (+1 step)")
                        .with_subtitle("Leading block steps up by 0.25rem")
                        .with_leading_size_offset(1)
                        .with_interactive(true),
                    theme,
                ))
                .child(js_list_card(
                    &ListCardSpec::new()
                        .with_title("Offset leading (+2 steps)")
                        .with_subtitle("Larger leading box, same typography")
                        .with_leading_size_offset(2)
                        .with_interactive(true),
                    theme,
                )),
        ))
        // -- Static list card --
        .child(group("Static list card", secondary,
            js_list_card(
                &ListCardSpec::new()
                    .with_title("System Configuration")
                    .with_subtitle("Read-only \u{2014} managed by admin")
                    .with_meta("v2.1.0"),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
