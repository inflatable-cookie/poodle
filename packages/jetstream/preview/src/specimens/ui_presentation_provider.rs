//! UiPresentationProvider specimen — size/density propagation boundary.
//!
//! Contract: `docs/contracts/components/ui-presentation-provider.md`
//!
//! The provider renders no chrome of its own; its effect is the size/density
//! cascade applied to descendant controls. Jetstream has no context-cascade
//! primitive (there is no `js_ui_presentation_provider`), so this specimen
//! demonstrates the boundary by resolving each region's effective control size
//! through the provider's own `UiPresentationProviderSpec::resolve_size` logic
//! and applying the resolved size + density to real `js_button` / `js_text_input`
//! controls. Mirrors the GPUI specimen's three rows: compact/sm, comfortable/lg,
//! and a nested override (outer default/md, inner compact/sm).

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_button;
use crate::compat::js_text_input;

use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, SemanticControlSizeRole, TextInputSpec,
    UiPresentationProviderSpec,
};

/// A row of real controls sized/spaced as the given provider would resolve them.
/// `resolve_size(Control)` is the provider's own identity mapping for control-
/// role descendants — using it (rather than passing `size` directly) keeps the
/// resolution path honest.
fn scoped_controls(
    density: ControlDensity,
    size: ControlSize,
    text: &str,
    theme: &JetstreamThemeProvider,
) -> El {
    let provider = UiPresentationProviderSpec::new()
        .with_density(density)
        .with_size_scale(size);
    let resolved = provider.resolve_size(SemanticControlSizeRole::Control);

    div().flex_row().gap(10.0).items_center()
        .child(js_button(
            &ButtonSpec::new()
                .with_label(text.to_string())
                .with_size(resolved)
                .with_density(density),
            theme,
        ))
        .child(div().w(160.0).child(js_text_input(
            &TextInputSpec::new()
                .with_default_value(text.to_string())
                .with_aria_label(text.to_string())
                .with_size(resolved)
                .with_density(density),
            theme,
        )))
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");

    // A bordered region stands in for the provider subtree.
    let region = move |child: El| -> El {
        div().p(12.0).border_1().border_color(border).rounded(6.0).child(child)
    };

    div().flex_col().gap(24.0)
        // --- Compact / sm region ---
        .child(group("Compact / sm region", secondary,
            region(scoped_controls(ControlDensity::Compact, ControlSize::Sm, "Small scope", theme))
        ))
        // --- Comfortable / lg region ---
        .child(group("Comfortable / lg region", secondary,
            region(scoped_controls(ControlDensity::Comfortable, ControlSize::Lg, "Large scope", theme))
        ))
        // --- Nested override: outer default/md, inner compact/sm subtree ---
        .child(group("Nested override", secondary,
            region(
                div().flex_col().gap(12.0)
                    .child(scoped_controls(ControlDensity::Default, ControlSize::Md, "Outer default/md", theme))
                    .child(
                        // Inner provider shadows the outer scope for its subtree.
                        div().p(12.0).border_1().border_color(border).rounded(6.0)
                            .child(scoped_controls(ControlDensity::Compact, ControlSize::Sm, "Inner compact/sm", theme))
                    )
            )
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
