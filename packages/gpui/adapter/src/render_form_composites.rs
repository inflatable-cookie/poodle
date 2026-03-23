//! RenderComponent implementations for form, validation, and remediation composites.
//!
//! g07.007: FormShellSpec, ValidationSummarySpec, RemediationBannerSpec,
//! InlineRemediationSpec, ConfirmActionSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_composites::{
    ConfirmActionSpec, FormShellSpec, InlineRemediationSpec, RemediationBannerSpec,
    ValidationSummarySpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<FormShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &FormShellSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("form-shell", "FormShellSpec")
    }
}

impl RenderComponent<ValidationSummarySpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &ValidationSummarySpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("validation-summary", "ValidationSummarySpec")
    }
}

impl RenderComponent<RemediationBannerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &RemediationBannerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("remediation-banner", "RemediationBannerSpec")
    }
}

impl RenderComponent<InlineRemediationSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &InlineRemediationSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("inline-remediation", "InlineRemediationSpec")
    }
}

impl RenderComponent<ConfirmActionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &ConfirmActionSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("confirm-action", "ConfirmActionSpec")
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_composites::*;
    use poodle_style::StyleDescriptor;
    use crate::{GpuiAdapter, theme::GpuiThemeProvider};

    fn a() -> GpuiAdapter { GpuiAdapter::new(GpuiThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> GpuiThemeProvider { GpuiThemeProvider::default() }

    #[test] fn form_shell() { assert_eq!(a().render(&FormShellSpec::new("f1"), &s(), &t()).spec_type, "FormShellSpec"); }
    #[test] fn validation_summary() { assert_eq!(a().render(&ValidationSummarySpec::new(vec![]), &s(), &t()).spec_type, "ValidationSummarySpec"); }
    #[test] fn remediation_banner() { assert_eq!(a().render(&RemediationBannerSpec::new("title", "msg"), &s(), &t()).spec_type, "RemediationBannerSpec"); }
    #[test] fn inline_remediation() { assert_eq!(a().render(&InlineRemediationSpec::new("msg"), &s(), &t()).spec_type, "InlineRemediationSpec"); }
    #[test] fn confirm_action() { assert_eq!(a().render(&ConfirmActionSpec::new("title", "msg", "OK", "Cancel"), &s(), &t()).spec_type, "ConfirmActionSpec"); }
}
