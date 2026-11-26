//! Shared theme and event contracts for native Poodle rendering.
//!
//! The legacy GPUI adapter also uses the direct-rendering traits and manifest.

use poodle_events::SemanticEvent;
use poodle_style::StyleDescriptor;
use poodle_tokens::typed::ColorValue;

/// Provides resolved token values for the current theme context.
///
/// The shared renderer uses this to resolve semantic tokens into concrete
/// values in a node tree. Each backend can supply its own theme provider.
pub trait ThemeProvider {
    /// Resolve a color token path to an RGBA color value, falling back to opaque black.
    fn resolve_color(&self, token: &str) -> ColorValue {
        self.try_resolve_color(token)
            .unwrap_or(ColorValue(0.0, 0.0, 0.0, 1.0))
    }

    /// Resolve a color token when the theme defines it.
    ///
    /// `None` means the token is missing. A defined black is
    /// `Some(ColorValue(0.0, 0.0, 0.0, 1.0))`, distinct from a missing token.
    fn try_resolve_color(&self, token: &str) -> Option<ColorValue>;

    /// Resolve a space/size token path to a pixel value.
    fn resolve_space(&self, token: &str) -> f32;

    /// Resolve a border-width token path to a pixel value.
    fn resolve_border_width(&self, token: &str) -> f32;

    /// Resolve a radius token path to a pixel value.
    fn resolve_radius(&self, token: &str) -> f32;

    /// Resolve an opacity token path to a 0.0–1.0 value, falling back to 1.0.
    fn resolve_opacity(&self, token: &str) -> f32 {
        self.try_resolve_opacity(token).unwrap_or(1.0)
    }

    /// Resolve an opacity token when the theme defines it.
    ///
    /// `None` means the token is missing. A defined fully-opaque value is
    /// `Some(1.0)`, distinct from a missing token.
    fn try_resolve_opacity(&self, token: &str) -> Option<f32>;
}

/// Receives semantic events emitted by rendered components.
///
/// The adapter calls methods on this sink when the user interacts with
/// rendered components. The application layer implements this to handle
/// events in a renderer-agnostic way.
pub trait EventSink {
    /// Handle a semantic event from a component.
    ///
    /// `component_id` identifies which component instance emitted the event.
    fn handle_event(&mut self, component_id: &str, event: SemanticEvent);
}

/// The output type that a legacy direct renderer produces.
pub trait RenderTarget {
    /// An opaque handle to a rendered element in the target system.
    type Handle;
}

/// Maps a Poodle component spec and resolved style to the renderer's native output.
///
/// The legacy GPUI adapter implements this trait for each direct-rendered
/// component. `Spec` is the component's input and `Target` is the output type.
///
/// ## Example (conceptual)
///
/// ```rust,ignore
/// impl RenderComponent<ButtonSpec> for GpuiAdapter {
///     fn render(
///         &self,
///         spec: &ButtonSpec,
///         style: &StyleDescriptor,
///         theme: &dyn ThemeProvider,
///     ) -> <GpuiTarget as RenderTarget>::Handle {
///         // Map spec + style to GPUI Element
///     }
/// }
/// ```
pub trait RenderComponent<Spec> {
    /// The rendering target system.
    type Target: RenderTarget;

    /// Render a component spec with its resolved style.
    ///
    /// The `style` descriptor contains all visual properties already resolved
    /// from tokens. The `theme` provider is available for any additional
    /// dynamic resolution needed at render time.
    fn render(
        &self,
        spec: &Spec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> <Self::Target as RenderTarget>::Handle;
}

/// Describes the direct `RenderComponent` surface implemented by an adapter.
///
/// This is runtime introspection for the legacy direct-adapter path. Shared
/// `poodle-render` coverage is validated separately through its backends and
/// previews; this manifest does not claim whole-runtime parity.
pub trait AdapterManifest {
    /// The adapter's display name.
    fn name(&self) -> &str;

    /// List of component spec type names this adapter supports.
    fn supported_components(&self) -> &[&str];

    /// List of component spec type names intentionally unsupported (with reason).
    fn unsupported_components(&self) -> &[(&str, &str)];
}

#[cfg(test)]
mod tests {
    use poodle_events::EventValue;
    use poodle_tokens::typed::ColorValue;

    use super::*;

    // -- Mock implementations for trait testing --

    struct MockTheme;

    impl ThemeProvider for MockTheme {
        fn try_resolve_color(&self, token: &str) -> Option<ColorValue> {
            match token {
                "color.accent.base" => Some(ColorValue(0.5, 0.5, 0.5, 1.0)),
                "color.black" => Some(ColorValue(0.0, 0.0, 0.0, 1.0)),
                _ => None,
            }
        }
        fn resolve_space(&self, _token: &str) -> f32 {
            8.0
        }
        fn resolve_border_width(&self, _token: &str) -> f32 {
            1.0
        }
        fn resolve_radius(&self, _token: &str) -> f32 {
            4.0
        }
        fn try_resolve_opacity(&self, token: &str) -> Option<f32> {
            match token {
                "opacity.full" => Some(1.0),
                "opacity.half" => Some(0.5),
                _ => None,
            }
        }
    }

    struct MockEventSink {
        events: Vec<(String, SemanticEvent)>,
    }

    impl EventSink for MockEventSink {
        fn handle_event(&mut self, component_id: &str, event: SemanticEvent) {
            self.events.push((component_id.to_string(), event));
        }
    }

    #[test]
    fn theme_provider_resolves_values() {
        let theme = MockTheme;
        let color = theme.resolve_color("color.accent.base");
        assert_eq!(color, ColorValue(0.5, 0.5, 0.5, 1.0));
        assert_eq!(theme.resolve_space("space.stack.md"), 8.0);
    }

    #[test]
    fn theme_provider_distinguishes_defined_fallback_values_from_missing_tokens() {
        let theme = MockTheme;
        let black = ColorValue(0.0, 0.0, 0.0, 1.0);
        assert_eq!(theme.try_resolve_color("color.black"), Some(black));
        assert_eq!(theme.try_resolve_color("color.missing"), None);
        assert_eq!(theme.resolve_color("color.black"), black);
        assert_eq!(theme.resolve_color("color.missing"), black);

        assert_eq!(theme.try_resolve_opacity("opacity.full"), Some(1.0));
        assert_eq!(theme.try_resolve_opacity("opacity.missing"), None);
        assert_eq!(theme.resolve_opacity("opacity.full"), 1.0);
        assert_eq!(theme.resolve_opacity("opacity.missing"), 1.0);
        assert_eq!(theme.resolve_opacity("opacity.half"), 0.5);
    }

    #[test]
    fn event_sink_collects_events() {
        let mut sink = MockEventSink { events: vec![] };
        sink.handle_event("btn-1", SemanticEvent::Activated);
        sink.handle_event(
            "input-1",
            SemanticEvent::ValueChanged {
                value: EventValue::Text(String::from("hello")),
            },
        );
        assert_eq!(sink.events.len(), 2);
        assert_eq!(sink.events[0].0, "btn-1");
        assert_eq!(sink.events[0].1, SemanticEvent::Activated);
    }

    struct MockManifest;

    impl AdapterManifest for MockManifest {
        fn name(&self) -> &str {
            "MockRenderer"
        }
        fn supported_components(&self) -> &[&str] {
            &["ButtonSpec", "DialogSpec"]
        }
        fn unsupported_components(&self) -> &[(&str, &str)] {
            &[("BlockEditorSpec", "No rich text support")]
        }
    }

    #[test]
    fn adapter_manifest_reports_support() {
        let manifest = MockManifest;
        assert_eq!(manifest.name(), "MockRenderer");
        assert_eq!(manifest.supported_components().len(), 2);
        assert_eq!(manifest.unsupported_components().len(), 1);
    }
}
