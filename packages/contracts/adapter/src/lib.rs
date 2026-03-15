//! Renderer adapter traits for Pug.
//!
//! These traits define the contract that rendering adapters implement to map
//! Pug component specs and resolved styles to a target renderer's native
//! element/node types.
//!
//! ## Adapter Implementations
//!
//! - **GPUI adapter** (`pug-gpui`, g07): Maps specs to GPUI `Element` types
//!   using GPUI's styling API and event subscription model.
//! - **Jetstream adapter** (`pug-jetstream`, g08): Maps specs to Jetstream
//!   `UiTree` widget variants using `UiStyle` and `UiEvent` pipeline.

use pug_events::SemanticEvent;
use pug_style::StyleDescriptor;
use pug_tokens::typed::ColorValue;

/// Provides resolved token values for the current theme context.
///
/// Adapters use this to resolve token references in spec structs to concrete
/// typed values. Each rendering target implements this based on its theme system:
///
/// - **GPUI**: Reads from GPUI's theme system (CSS-like token → typed value)
/// - **Jetstream**: Reads from Jetstream's `Theme` struct (Vec4 colors, f32 sizes)
pub trait ThemeProvider {
    /// Resolve a color token path to an RGBA color value.
    fn resolve_color(&self, token: &str) -> ColorValue;

    /// Resolve a space/size token path to a pixel value.
    fn resolve_space(&self, token: &str) -> f32;

    /// Resolve a border-width token path to a pixel value.
    fn resolve_border_width(&self, token: &str) -> f32;

    /// Resolve a radius token path to a pixel value.
    fn resolve_radius(&self, token: &str) -> f32;

    /// Resolve an opacity token path to a 0.0–1.0 value.
    fn resolve_opacity(&self, token: &str) -> f32;
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

/// The output type that a rendering adapter produces.
///
/// Each adapter defines its own `Target` that represents a native rendered
/// element or node:
///
/// - **GPUI**: A GPUI `Element` or `AnyElement`
/// - **Jetstream**: A `UiTree` widget node ID
pub trait RenderTarget {
    /// An opaque handle to a rendered element in the target system.
    type Handle;
}

/// Maps a Pug component spec and resolved style to the renderer's native output.
///
/// This is the core trait that adapters implement for each component. The
/// `Spec` type parameter is a Pug spec struct (e.g., `ButtonSpec`, `DialogSpec`),
/// and `Target` is the rendering system.
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

/// Describes which components an adapter supports.
///
/// Used by the parity validation tooling (g06.014) to verify that adapters
/// implement all required components.
pub trait AdapterManifest {
    /// The adapter's display name (e.g., "GPUI", "Jetstream").
    fn name(&self) -> &str;

    /// List of component spec type names this adapter supports.
    fn supported_components(&self) -> &[&str];

    /// List of component spec type names intentionally unsupported (with reason).
    fn unsupported_components(&self) -> &[(&str, &str)];
}

#[cfg(test)]
mod tests {
    use pug_events::EventValue;
    use pug_tokens::typed::ColorValue;

    use super::*;

    // -- Mock implementations for trait testing --

    struct MockTheme;

    impl ThemeProvider for MockTheme {
        fn resolve_color(&self, _token: &str) -> ColorValue {
            ColorValue(0.5, 0.5, 0.5, 1.0)
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
        fn resolve_opacity(&self, _token: &str) -> f32 {
            1.0
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
        let color = theme.resolve_color("semantic.color.accent.base");
        assert_eq!(color, ColorValue(0.5, 0.5, 0.5, 1.0));
        assert_eq!(theme.resolve_space("semantic.space.stack.md"), 8.0);
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
