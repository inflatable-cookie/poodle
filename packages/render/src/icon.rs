//! Icon — a named SVG glyph at a token-resolved size.
//!
//! Contract: `docs/contracts/components/icon.md`
//! Ported from: `packages/jetstream/components/src/icon.rs`. Rasterisation and
//! tinting are backend concerns; the node names the glyph and its box.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::IconSpec;

pub fn icon(spec: &IconSpec, theme: &dyn ThemeProvider) -> Node {
    let size = theme.resolve_space(spec.size_token());
    let color = theme.resolve_color("color.icon.primary");

    let mut el = Node::icon(&spec.name, size);
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        // GPUI SVGs do not inherit a usable tint without an explicit text
        // colour. This is the old tier's default icon colour.
        s.descriptor.text_color = Some(color);
    }
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_the_explicit_gpui_svg_tint() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let node = icon(&IconSpec::new("plus"), &theme);
        assert_eq!(
            node.style.descriptor.text_color,
            Some(theme.resolve_color("color.icon.primary"))
        );
    }

    fn paint_asset_exists(name: &str) -> bool {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/icons")
            .join(format!("{name}.svg"))
            .is_file()
    }

    #[test]
    fn default_specimen_icons_have_paint_assets() {
        for name in ["audio-waveform", "piano"] {
            assert!(paint_asset_exists(name), "{name} must have an SVG asset");
        }
    }

    #[test]
    fn custom_icon_names_pass_through() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let node = icon(&IconSpec::new("company-logo"), &theme);
        assert!(matches!(
            &node.kind,
            poodle_node::NodeKind::Icon { name, .. } if name == "company-logo"
        ));
    }
}
