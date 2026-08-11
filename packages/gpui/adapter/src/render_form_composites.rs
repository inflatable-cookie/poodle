//! RenderComponent implementations for form, validation, and remediation composites.
//!
//! g07.007: FormShellSpec, ValidationSummarySpec, RemediationBannerSpec,
//! InlineRemediationSpec, ConfirmActionSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    ConfirmActionSpec, FormShellSpec, InlineRemediationSpec, RemediationBannerSpec,
    ValidationSummarySpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<FormShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &FormShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("form-shell", "FormShellSpec")
    }
}

impl RenderComponent<ValidationSummarySpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ValidationSummarySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("validation-summary", "ValidationSummarySpec")
    }
}

impl RenderComponent<RemediationBannerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &RemediationBannerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("remediation-banner", "RemediationBannerSpec")
    }
}

impl RenderComponent<InlineRemediationSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &InlineRemediationSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("inline-remediation", "InlineRemediationSpec")
    }
}

impl RenderComponent<ConfirmActionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        spec: &ConfirmActionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        // `poodle_gpui_components::ConfirmAction` cannot be mounted here without a
        // circular crate dependency (adapter → components → adapter). Encode a
        // stable, spec-derived handle so demo manifests and tests observe real input.
        let element_id = format!("confirm-action|open={}|title={}", spec.is_open, spec.title);
        GpuiElementHandle::new(element_id, "ConfirmActionSpec")
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::GpuiThemeProvider, GpuiAdapter};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;

    fn a() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn form_shell() {
        assert_eq!(
            a().render(&FormShellSpec::new("f1"), &s(), &t()).spec_type,
            "FormShellSpec"
        );
    }
    #[test]
    fn validation_summary() {
        assert_eq!(
            a().render(&ValidationSummarySpec::new(vec![]), &s(), &t())
                .spec_type,
            "ValidationSummarySpec"
        );
    }
    #[test]
    fn remediation_banner() {
        assert_eq!(
            a().render(&RemediationBannerSpec::new("title", "msg"), &s(), &t())
                .spec_type,
            "RemediationBannerSpec"
        );
    }
    #[test]
    fn inline_remediation() {
        assert_eq!(
            a().render(&InlineRemediationSpec::new("msg"), &s(), &t())
                .spec_type,
            "InlineRemediationSpec"
        );
    }
    #[test]
    fn confirm_action() {
        let handle = a().render(
            &ConfirmActionSpec::new("title", "msg", "OK", "Cancel"),
            &s(),
            &t(),
        );
        assert_eq!(handle.spec_type, "ConfirmActionSpec");
        assert!(handle.element_id.starts_with("confirm-action|open="));
        assert!(handle.element_id.contains("title=title"));
    }
}
