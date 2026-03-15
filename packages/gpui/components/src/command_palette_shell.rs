//! PugCommandPaletteShell — real GPUI component backed by CommandPaletteShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::CommandPaletteShellSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI command palette shell backed by `CommandPaletteShellSpec`.
///
/// Wraps a command palette with backdrop overlay and open/close state.
/// When open, renders a semi-transparent backdrop and the palette content.
pub struct PugCommandPaletteShell {
    spec: CommandPaletteShellSpec,
    theme: GpuiThemeProvider,
    /// The palette content to render when open.
    content: Option<AnyElement>,
    on_dismiss: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl PugCommandPaletteShell {
    pub fn new(spec: CommandPaletteShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            on_dismiss: None,
        }
    }

    /// Set the palette content to display when the shell is open.
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugCommandPaletteShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        if !spec.current_open() {
            // When closed, render nothing (empty container).
            return div().into_any_element();
        }

        let backdrop_fill = resolve_color(theme, spec.backdrop_fill_token());

        // Full-screen backdrop overlay
        let mut shell = div()
            .absolute()
            .inset_0()
            .bg(backdrop_fill)
            .flex()
            .justify_center()
            .pt(px(80.0));

        // Centered palette content
        if let Some(content) = self.content {
            shell = shell.child(
                div()
                    .flex()
                    .flex_col()
                    .child(content),
            );
        }

        shell.into_any_element()
    }
}
