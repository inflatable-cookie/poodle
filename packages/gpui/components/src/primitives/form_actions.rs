//! FormActions — real GPUI component backed by FormActionsSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonVariant, ControlDensity, FormActionAlign, FormActionsSpec, SemanticControlSizeRole,
};

use crate::presentation::rem_to_px;
use crate::primitives::IconButton;
use crate::theme_ext::resolve_color;

/// A real GPUI form actions bar backed by `FormActionsSpec`.
///
/// Lays out action buttons (submit, cancel, etc.) with configurable alignment.
/// Optional inline danger content + a collapsed overflow danger menu (contract
/// §2/§8) are rendered when supplied via `with_danger_action` / `dangerItems`.
pub struct FormActions {
    spec: FormActionsSpec,
    theme: GpuiThemeProvider,
    actions: Vec<AnyElement>,
    /// Inline destructive/cancel content (contract `danger` snippet). Rendered
    /// left of the primary actions in a `__danger` inline-flex group.
    danger_actions: Vec<AnyElement>,
}

impl std::ops::Deref for FormActions {
    type Target = FormActionsSpec;
    fn deref(&self) -> &FormActionsSpec {
        &self.spec
    }
}

impl FormActions {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: FormActionsSpec::new(),
            theme: theme.clone(),
            actions: Vec::new(),
            danger_actions: Vec::new(),
        }
    }

    pub fn from_spec(spec: FormActionsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: Vec::new(),
            danger_actions: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn align(mut self, v: FormActionAlign) -> Self {
        self.spec.align = v;
        self
    }

    /// Toggle the top padding that separates the action row from the
    /// field stack above (contract `showTopSeparation`). When false the
    /// row sits flush on a footer rail.
    pub fn show_top_separation(mut self, v: bool) -> Self {
        self.spec.show_top_separation = v;
        self
    }

    /// Toggle the subtle top divider + offset before the row
    /// (contract `showTopBorder`).
    pub fn show_top_border(mut self, v: bool) -> Self {
        self.spec.show_top_border = v;
        self
    }

    /// Spacing density (contract §4 + §8). Controls inline gap, top
    /// separation, and border gap; does not change child button size.
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    /// Add an action element (typically a Button).
    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// Add inline destructive/cancel content (contract `danger` snippet),
    /// rendered left-aligned in a `__danger` inline-flex group before the
    /// primary actions.
    pub fn with_danger_action(mut self, action: impl IntoElement) -> Self {
        self.danger_actions.push(action.into_any_element());
        self
    }
}

impl IntoElement for FormActions {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        // Density-keyed gap: compact/comfortable use contract-exact rems,
        // default inherits the inline-md token (contract §8).
        let gap = match self.spec.gap_rem() {
            Some(rem) => rem_to_px(rem),
            None => theme.resolve_space(self.spec.action_gap_token()),
        };
        // Density-keyed top separation: compact/comfortable use contract
        // rems, default inherits stack-sm token (contract §8).
        let separation = if self.spec.shows_top_separation() {
            match self.spec.top_separation_rem() {
                Some(rem) => rem_to_px(rem),
                None => theme.resolve_space(self.spec.stack_separation_token()),
            }
        } else {
            0.0
        };
        // Density-keyed divider offset (contract §8 Divider Offset Variants).
        let border_gap = if self.spec.shows_top_border() {
            rem_to_px(self.spec.border_gap_rem())
        } else {
            0.0
        };
        let border = resolve_color(theme, self.spec.border_token());

        // Accessibility semantics (contract section 6):
        // Neutral structural container — no implicit ARIA role.
        // Action order must remain logical for keyboard and screen reader users.
        // Collapsed danger trigger must expose an aria-label.

        // Contract says no border, only padding-top for separation
        let mut row = div()
            .flex()
            .flex_wrap() // contract: wraps on narrow widths
            .items_center()
            .gap(px(gap))
            .pt(px(separation))
            .mt(px(border_gap));

        if self.spec.shows_top_border() {
            row = row.border_t_1().border_color(border);
        }

        // Alignment
        match self.spec.align {
            FormActionAlign::Start => {
                row = row.justify_start();
            }
            FormActionAlign::End => {
                row = row.justify_end();
            }
            FormActionAlign::Between => {
                row = row.justify_between();
            }
        }

        // Inline danger group (contract §2 danger snippet, §8 Danger Inline:
        // inline-flex, gap == form-actions gap). Rendered before the primary
        // actions so the destructive/cancel content stays visually separated.
        if !self.danger_actions.is_empty() {
            let mut danger = div().flex().items_center().gap(px(gap));
            for action in self.danger_actions {
                danger = danger.child(action);
            }
            row = row.child(danger);
        }

        // Overflow danger menu trigger (contract §2 danger menu / §8 Responsive
        // Swap). Shown when `dangerItems` is present. The real container-query
        // collapse — hiding the inline group and showing only this trigger below
        // 31.25rem — is a runtime concern GPUI's render-immediate model can't
        // express (no container-query channel), so both the inline group and the
        // trigger render here; the trigger itself is a real ghost IconButton with
        // an aria-label per contract §6.
        if self.spec.has_danger_menu() {
            let trigger = IconButton::new(theme)
                .icon("ellipsis")
                .variant(ButtonVariant::Ghost)
                .aria_label("More actions")
                .size_role(SemanticControlSizeRole::Chrome);
            row = row.child(div().flex().items_center().child(trigger));
        }

        for action in self.actions {
            row = row.child(action);
        }

        row.into_any_element()
    }
}
