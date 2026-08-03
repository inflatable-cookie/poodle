//! RenderComponent implementations for input primitives.
//!
//! g08.005: TextInputSpec, FieldSpec,
//! NumberInputSpec, CodeInputSpec, EditableLabelSpec, TimeFieldSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    CodeInputSpec, EditableLabelSpec, FieldSpec, NumberInputSpec, TextInputSpec, TimeFieldSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<TextInputSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &TextInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve fill (background) color
        let fill_color = theme.resolve_color(spec.fill_token());
        mapped.visuals.background = Some(JetstreamColor::from(fill_color));

        // Resolve border color (varies by validation state)
        let border_color = theme.resolve_color(spec.border_token());
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        // Resolve corner radius
        let r = theme.resolve_radius(spec.radius_token());
        mapped.visuals.corner_radii = [r; 4];

        // Resolve text color
        let text_color = theme.resolve_color(spec.text_color_token());
        mapped.visuals.text_color = Some(JetstreamColor::from(text_color));

        // Resolve placeholder color (for proof — Jetstream text_color covers primary)
        let _placeholder_color = theme.resolve_color(spec.placeholder_color_token());

        // Resolve focus ring
        let ring_color = theme.resolve_color(spec.focus_ring_color_token());
        mapped.visuals.focus_ring_color = Some(JetstreamColor::from(ring_color));
        mapped.visuals.focus_ring_width = theme.resolve_space(spec.focus_ring_width_token());

        // Handle disabled state
        if spec.is_disabled {
            mapped.visuals.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        let node_id = match &spec.id {
            Some(id) if !id.is_empty() => format!("text-input-{}", id),
            _ => "text-input".to_string(),
        };
        JetstreamNodeHandle::new(node_id, "TextInputSpec", WidgetKind::TextInput, mapped)
    }
}

impl RenderComponent<FieldSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &FieldSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve label typography size (used as space value for proof)
        let _label_size = theme.resolve_space(spec.label_typography_token());

        // Resolve supporting text typography size
        let _supporting_size = theme.resolve_space(spec.supporting_text_typography_token());

        // Resolve error color
        let _error_color = theme.resolve_color(spec.error_color_token());

        // Resolve description color
        let desc_color = theme.resolve_color(spec.description_color_token());
        mapped.visuals.text_color = Some(JetstreamColor::from(desc_color));

        // Resolve row gap (vertical spacing between label, input, description)
        let gap = theme.resolve_space(spec.row_gap_token());
        mapped.layout.gap = taffy::Size {
            width: taffy::LengthPercentage::length(gap),
            height: taffy::LengthPercentage::length(gap),
        };

        // Resolve header gap (spacing between label and optional indicator)
        let _header_gap = theme.resolve_space(spec.header_gap_token());

        let node_id = format!("field-{}", spec.id);
        JetstreamNodeHandle::new(node_id, "FieldSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<NumberInputSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &NumberInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color (varies by validation state)
        let border_color = theme.resolve_color(spec.border_token());
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        // Handle disabled state
        if spec.is_disabled {
            mapped.visuals.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        JetstreamNodeHandle::new(
            "number-input",
            "NumberInputSpec",
            WidgetKind::TextInput,
            mapped,
        )
    }
}

impl RenderComponent<CodeInputSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &CodeInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve border color
        let border_color = theme.resolve_color(spec.border_token());
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        // Resolve focus ring color
        let ring_color = theme.resolve_color(spec.focus_ring_color_token());
        mapped.visuals.focus_ring_color = Some(JetstreamColor::from(ring_color));

        let node_id = format!("code-input-{}", spec.length);
        JetstreamNodeHandle::new(node_id, "CodeInputSpec", WidgetKind::TextInput, mapped)
    }
}

impl RenderComponent<EditableLabelSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        spec: &EditableLabelSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve text color
        let text_color = theme.resolve_color(spec.text_color_token());
        mapped.visuals.text_color = Some(JetstreamColor::from(text_color));

        // Resolve edit border color (changes based on editing state)
        let border_color = theme.resolve_color(spec.edit_border_token());
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        let node_id = if spec.is_editing {
            "editable-label-editing"
        } else {
            "editable-label"
        };
        JetstreamNodeHandle::new(node_id, "EditableLabelSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<TimeFieldSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &TimeFieldSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        // TimeFieldSpec has no specific token methods — apply base style mapping
        let mapped = map_style(style);

        JetstreamNodeHandle::new("time-field", "TimeFieldSpec", WidgetKind::TextInput, mapped)
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::JetstreamThemeProvider, JetstreamAdapter, WidgetKind};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;

    fn a() -> JetstreamAdapter {
        JetstreamAdapter::new(JetstreamThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> JetstreamThemeProvider {
        JetstreamThemeProvider::default()
    }

    #[test]
    fn text_input_with_id_uses_id_in_node_id() {
        let spec = TextInputSpec::new().with_id("email");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "text-input-email");
        assert_eq!(h.spec_type, "TextInputSpec");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }

    #[test]
    fn text_input_without_id_defaults() {
        let h = a().render(&TextInputSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "text-input");
    }

    #[test]
    fn text_input_disabled_renders() {
        let spec = TextInputSpec::new().with_disabled(true).with_id("name");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "text-input-name");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }

    #[test]
    fn field_uses_id_in_node_id() {
        let spec = FieldSpec::new("name", "Name");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "field-name");
        assert_eq!(h.spec_type, "FieldSpec");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }

    #[test]
    fn field_with_different_id() {
        let spec = FieldSpec::new("email", "Email Address");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "field-email");
    }

    #[test]
    fn number_input_renders() {
        let h = a().render(&NumberInputSpec::new(0.0), &s(), &t());
        assert_eq!(h.node_id, "number-input");
        assert_eq!(h.spec_type, "NumberInputSpec");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }

    #[test]
    fn number_input_disabled_renders() {
        let spec = NumberInputSpec::new(5.0).with_disabled(true);
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "number-input");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }

    #[test]
    fn code_input_uses_length_in_id() {
        let spec = CodeInputSpec::new().with_length(6);
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "code-input-6");
        assert_eq!(h.spec_type, "CodeInputSpec");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }

    #[test]
    fn code_input_custom_length() {
        let spec = CodeInputSpec::new().with_length(4);
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "code-input-4");
    }

    #[test]
    fn editable_label_not_editing() {
        let h = a().render(&EditableLabelSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "editable-label");
        assert_eq!(h.spec_type, "EditableLabelSpec");
        assert_eq!(h.widget_kind, WidgetKind::Label);
    }

    #[test]
    fn editable_label_editing_state() {
        let spec = EditableLabelSpec::new().with_editing(true);
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "editable-label-editing");
    }

    #[test]
    fn time_field_renders() {
        let h = a().render(&TimeFieldSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "time-field");
        assert_eq!(h.spec_type, "TimeFieldSpec");
        assert_eq!(h.widget_kind, WidgetKind::TextInput);
    }
}
