//! RenderComponent implementations for action, text-entry, and field primitives.
//!
//! Implements the adapter trait for:
//! - ButtonSpec, IconButtonSpec
//! - FieldSpec, TextInputSpec
//! - FormActionsSpec, TimeInputSpec
//! - EditableLabelSpec, NumberInputSpec, CodeInputSpec, ToolbarSpec
//!
//! NOTE: `gpui_style` is built and mutated in every render fn as proof of token
//! resolution, but is not yet wired into the returned `GpuiElementHandle` (the
//! GPUI integration is incomplete). The unused-variable / unused-assignment lints
//! are suppressed at the module level until the wiring is in place.
#![allow(unused_variables, unused_assignments)]

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    ButtonSpec, CodeInputSpec, EditableLabelSpec, FieldSpec, FormActionsSpec, IconButtonSpec,
    NumberInputSpec, TextInputSpec, TimeInputSpec, ToolbarSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<ButtonSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &ButtonSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let fill = theme.resolve_color(spec.resolved_fill_token());
        let text = theme.resolve_color(spec.resolved_text_token());
        let border = theme.resolve_color(spec.resolved_border_token());
        let radius = theme.resolve_radius(spec.radius_token());

        gpui_style.background = Some(fill.into());
        gpui_style.text_color = Some(text.into());
        gpui_style.border_color = Some(border.into());
        gpui_style.corner_radii.top_left = radius;
        gpui_style.corner_radii.top_right = radius;
        gpui_style.corner_radii.bottom_left = radius;
        gpui_style.corner_radii.bottom_right = radius;

        if spec.is_disabled {
            gpui_style.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        let mut element_id = format!("button-{}", spec.label.as_deref().unwrap_or("anonymous"));
        if let Some(expanded) = spec.aria_expanded {
            element_id.push_str(if expanded {
                "|aria_expanded=true"
            } else {
                "|aria_expanded=false"
            });
        }

        GpuiElementHandle::new(element_id, "ButtonSpec")
    }
}

impl RenderComponent<IconButtonSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &IconButtonSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let gpui_style = map_style(style);

        // Legacy direct-manifest proof: no presentation scope exists on this
        // path, so omission resolves against the root defaults (md/default) —
        // the same values `RenderContext::new` supplies in poodle-render.
        let size = spec.size.unwrap_or_default();
        // Resolve icon size token (used for width/height constraints)
        let _icon_size = theme.resolve_space(spec.icon_size_token(size));

        // Resolve control height token
        let _control_height = theme.resolve_space(spec.control_height_token(size));

        // gpui_style is constructed for proof of style mapping
        let _ = &gpui_style;

        GpuiElementHandle::new(
            format!(
                "icon-button-{}",
                spec.icon.as_deref().unwrap_or("anonymous")
            ),
            "IconButtonSpec",
        )
    }
}

impl RenderComponent<FieldSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &FieldSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Legacy direct-manifest proof: no presentation scope exists on this
        // path, so omission resolves against the root defaults (md/default) —
        // the same values `RenderContext::new` supplies in poodle-render.
        let size = spec.size.unwrap_or_default();
        let density = spec.density.unwrap_or_default();
        // Resolve label typography size (used as space value for proof)
        let _label_size = theme.resolve_space(spec.label_typography_token(size));

        // Resolve supporting text typography size
        let _supporting_size = theme.resolve_space(spec.supporting_text_typography_token(size));

        // Resolve error color
        let _error_color = theme.resolve_color(spec.error_color_token());

        // Resolve description color
        let desc_color = theme.resolve_color(spec.description_color_token());
        gpui_style.text_color = Some(desc_color.into());

        // Resolve row gap (vertical spacing between label, input, description)
        gpui_style.gap = theme.resolve_space(spec.row_gap_token(density));

        // Resolve header gap (spacing between label and optional indicator)
        let _header_gap = theme.resolve_space(spec.header_gap_token(density));

        GpuiElementHandle::new(format!("field-{}", spec.id), "FieldSpec")
    }
}

impl RenderComponent<TextInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &TextInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let fill = theme.resolve_color(spec.fill_token());
        let border = theme.resolve_color(spec.border_token());
        let radius = theme.resolve_radius(spec.radius_token());
        gpui_style.background = Some(fill.into());
        gpui_style.border_color = Some(border.into());
        gpui_style.corner_radii = crate::GpuiCornerRadii {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        };

        // Resolve text color
        let text_color = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(text_color.into());

        // Resolve placeholder color (for proof)
        let _placeholder_color = theme.resolve_color(spec.placeholder_color_token());

        // Resolve focus ring
        let ring_color = theme.resolve_color(spec.focus_ring_color_token());
        gpui_style.focus_ring_color = Some(ring_color.into());
        gpui_style.focus_ring_width = theme.resolve_space(spec.focus_ring_width_token());

        if spec.is_disabled {
            gpui_style.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        GpuiElementHandle::new(
            format!("text-input-{}", spec.id.as_deref().unwrap_or("anonymous")),
            "TextInputSpec",
        )
    }
}

impl RenderComponent<FormActionsSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &FormActionsSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve action gap (space between action buttons)
        gpui_style.gap = theme.resolve_space(spec.action_gap_token());

        // Resolve stack separation (vertical space above the actions bar)
        let _stack_sep = theme.resolve_space(spec.stack_separation_token());

        GpuiElementHandle::new("form-actions", "FormActionsSpec")
    }
}

impl RenderComponent<TimeInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        _spec: &TimeInputSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _gpui_style = map_style(style);
        GpuiElementHandle::new("time-input", "TimeInputSpec")
    }
}

impl RenderComponent<EditableLabelSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &EditableLabelSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve text color
        let text_color = theme.resolve_color(spec.text_color_token());
        gpui_style.text_color = Some(text_color.into());

        // Resolve edit border color (changes based on editing state)
        let border = theme.resolve_color(spec.edit_border_token());
        gpui_style.border_color = Some(border.into());

        let id = if spec.is_editing {
            "editable-label-editing"
        } else {
            "editable-label"
        };
        GpuiElementHandle::new(id, "EditableLabelSpec")
    }
}

impl RenderComponent<NumberInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &NumberInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let border = theme.resolve_color(spec.border_token());
        gpui_style.border_color = Some(border.into());

        if spec.is_disabled {
            gpui_style.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        }

        GpuiElementHandle::new("number-input", "NumberInputSpec")
    }
}

impl RenderComponent<CodeInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &CodeInputSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let border = theme.resolve_color(spec.border_token());
        let focus_ring = theme.resolve_color(spec.focus_ring_color_token());
        gpui_style.border_color = Some(border.into());
        gpui_style.focus_ring_color = Some(focus_ring.into());

        GpuiElementHandle::new(format!("code-input-{}", spec.length), "CodeInputSpec")
    }
}

impl RenderComponent<ToolbarSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &ToolbarSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve gap between toolbar items
        gpui_style.gap = theme.resolve_space(spec.gap_token());

        // Resolve separator border if present
        if spec.has_separator {
            let border = theme.resolve_color(spec.border_token());
            gpui_style.border_color = Some(border.into());
        }

        let id = match &spec.aria_label {
            Some(label) if !label.is_empty() => format!("toolbar-{}", label),
            _ => "toolbar".to_string(),
        };
        GpuiElementHandle::new(id, "ToolbarSpec")
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_specs::{
        ButtonSpec, CodeInputSpec, EditableLabelSpec, FieldSpec, FormActionsSpec, IconButtonSpec,
        NumberInputSpec, TextInputSpec, TimeInputSpec, ToolbarSpec,
    };
    use poodle_style::StyleDescriptor;

    use crate::theme::GpuiThemeProvider;
    use crate::GpuiAdapter;

    fn adapter() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }

    fn style() -> StyleDescriptor {
        StyleDescriptor::new()
    }

    fn theme() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn render_button_resolves_tokens() {
        let a = adapter();
        let spec = ButtonSpec::new().with_label("Save");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "button-Save");
        assert_eq!(handle.spec_type, "ButtonSpec");
    }

    #[test]
    fn render_button_aria_expanded_in_element_id() {
        let a = adapter();
        let spec = ButtonSpec::new()
            .with_label("Menu")
            .with_aria_expanded(true);
        let handle = a.render(&spec, &style(), &theme());
        assert!(handle.element_id.contains("aria_expanded=true"));
    }

    #[test]
    fn render_icon_button_resolves_tokens() {
        let a = adapter();
        let spec = IconButtonSpec::new().with_icon("close");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "icon-button-close");
        assert_eq!(handle.spec_type, "IconButtonSpec");
    }

    #[test]
    fn render_field_resolves_tokens() {
        let a = adapter();
        let spec = FieldSpec::new("name", "Name");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "field-name");
        assert_eq!(handle.spec_type, "FieldSpec");
    }

    #[test]
    fn render_field_with_different_id() {
        let a = adapter();
        let spec = FieldSpec::new("email", "Email Address");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "field-email");
    }

    #[test]
    fn render_text_input_resolves_tokens() {
        let a = adapter();
        let spec = TextInputSpec::new().with_id("email");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "text-input-email");
        assert_eq!(handle.spec_type, "TextInputSpec");
    }

    #[test]
    fn render_form_actions_resolves_gap() {
        let a = adapter();
        let handle = a.render(&FormActionsSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "form-actions");
        assert_eq!(handle.spec_type, "FormActionsSpec");
    }

    #[test]
    fn render_time_input_produces_handle() {
        let a = adapter();
        let handle = a.render(&TimeInputSpec::new(), &style(), &theme());
        assert_eq!(handle.spec_type, "TimeInputSpec");
    }

    #[test]
    fn render_editable_label_not_editing() {
        let a = adapter();
        let handle = a.render(&EditableLabelSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "editable-label");
        assert_eq!(handle.spec_type, "EditableLabelSpec");
    }

    #[test]
    fn render_editable_label_editing_state() {
        let a = adapter();
        let spec = EditableLabelSpec::new().with_editing(true);
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "editable-label-editing");
    }

    #[test]
    fn render_number_input_produces_handle() {
        let a = adapter();
        let handle = a.render(&NumberInputSpec::new(42.0), &style(), &theme());
        assert_eq!(handle.spec_type, "NumberInputSpec");
    }

    #[test]
    fn render_code_input_produces_handle() {
        let a = adapter();
        let spec = CodeInputSpec::new().with_length(6);
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "code-input-6");
    }

    #[test]
    fn render_toolbar_with_aria_label() {
        let a = adapter();
        let spec = ToolbarSpec::new().with_aria_label("Main Tools");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "toolbar-Main Tools");
        assert_eq!(handle.spec_type, "ToolbarSpec");
    }

    #[test]
    fn render_toolbar_without_label_defaults_id() {
        let a = adapter();
        let handle = a.render(&ToolbarSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "toolbar");
    }

    #[test]
    fn render_toolbar_with_separator() {
        let a = adapter();
        let spec = ToolbarSpec::new().with_separator(true);
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "toolbar");
        assert_eq!(handle.spec_type, "ToolbarSpec");
    }
}
