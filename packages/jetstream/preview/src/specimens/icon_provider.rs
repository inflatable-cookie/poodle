//! IconProvider specimen — non-visual icon-registry boundary.
//!
//! Contract: `docs/contracts/components/icon-provider.md`
//!
//! IconProvider renders no chrome of its own. In Jetstream the icon registry is
//! shared process-wide (names resolve via `ui_element::icon`), so — exactly like
//! GPUI — the provider is a no-visual compatibility boundary: descendant icons
//! resolve from the same registry whether or not a provider wraps them. There is
//! no `js_icon_provider` builder; the boundary's only observable effect is that
//! the icons below render. This specimen shows real `js_icon`s inside a labeled
//! "provider scope" and states the boundary is non-visual.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::icon::js_icon;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::IconSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");
    let border = resolve_color(theme, "color.border.subtle");

    div().flex_col().gap(12.0)
        .child(label("Provider boundary").text_color(secondary).text_size(11.0))
        // The bordered region stands in for the provider's subtree. The icons
        // are real `js_icon`s resolving from the shared registry.
        .child(
            div().flex_row().items_center().gap(12.0)
                .p(12.0).border_1().border_color(border).rounded(6.0)
                .child(div().text_color(primary).child(js_icon(&IconSpec::new("search"), theme)))
                .child(div().text_color(primary).child(js_icon(&IconSpec::new("calendar"), theme)))
                .child(div().text_color(primary).child(js_icon(&IconSpec::new("clock"), theme)))
        )
        .child(
            label("Jetstream uses a shared icon registry, so this provider is a no-visual compatibility boundary.")
                .text_color(secondary).text_size(13.0)
        )
}
