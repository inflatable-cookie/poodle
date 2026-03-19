//! Renderer-agnostic semantic event types for Pug.
//!
//! These events describe component interactions in terms of user intent rather
//! than low-level input. Rendering adapters map from their native event systems:
//!
//! - **GPUI**: Mouse/keyboard event subscriptions → semantic events
//! - **Jetstream**: `UiEvent` enum (Clicked, ValueChanged, etc.) → semantic events

/// A semantic event emitted by a Pug component.
///
/// Each variant describes a meaningful interaction that both rendering targets
/// can produce from their native event systems.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticEvent {
    /// A component was activated (button click, enter key, tap).
    ///
    /// Emitted by: Button, IconButton, MenuItem, Link
    /// - GPUI: `on_click` handler
    /// - Jetstream: `UiEvent::Clicked`
    Activated,

    /// A component's value changed.
    ///
    /// Emitted by: TextInput, TextArea, Slider, Select, Checkbox, Switch,
    /// DatePicker, TimePicker, ColorPicker, etc.
    /// - GPUI: `on_change` handler
    /// - Jetstream: `UiEvent::ValueChanged` / `UiEvent::TextChanged`
    ValueChanged {
        value: EventValue,
    },

    /// Focus moved to or from a component.
    ///
    /// Emitted by: Any focusable component
    /// - GPUI: `on_focus` / `on_blur` handlers
    /// - Jetstream: `UiEvent::FocusGained` / `UiEvent::FocusLost`
    FocusChanged {
        gained: bool,
    },

    /// An overlay or disclosure component opened or closed.
    ///
    /// Emitted by: Dialog, Drawer, Popover, Accordion, Collapsible, Menu, Select
    /// - GPUI: State change handlers
    /// - Jetstream: `ScreenStack` push/pop for modals, widget state for others
    OpenChanged {
        open: bool,
    },

    /// A selection changed in a single-or-multi-select context.
    ///
    /// Emitted by: Select, RadioGroup, SegmentedControl, Tabs, TabStrip
    /// - GPUI: Selection change handlers
    /// - Jetstream: `UiEvent::ValueChanged` with selection payload
    SelectionChanged {
        value: EventValue,
    },

    /// A form was submitted.
    ///
    /// Emitted by: FormShell, TextInput (with submit action)
    /// - GPUI: Form submit handler / enter key
    /// - Jetstream: Button activation mapped to form context
    Submitted,

    /// A pending action was cancelled.
    ///
    /// Emitted by: Dialog (dismiss), FormShell (cancel), TextInput (escape)
    /// - GPUI: Escape key / cancel handler
    /// - Jetstream: Back navigation or cancel button
    Cancelled,

    /// Pointer entered or left a component's bounds.
    ///
    /// Emitted by: Any hoverable component (Button, Tooltip trigger, etc.)
    /// - GPUI: `on_mouse_enter` / `on_mouse_leave`
    /// - Jetstream: Not supported (gamepad/touch primary) — adapters may skip
    Hovered {
        entered: bool,
    },

    /// A drag operation started, moved, or ended.
    ///
    /// Emitted by: Slider (thumb), ReorderableList, SplitView (divider)
    /// - GPUI: Drag event handlers
    /// - Jetstream: `UiEvent::DragStarted` / `DragMoved` / `DragEnded`
    DragChanged {
        phase: DragPhase,
        position: Option<(f32, f32)>,
    },

    /// A keyboard key was pressed in a component context.
    ///
    /// Emitted by: Any component with keyboard shortcuts
    /// - GPUI: `on_key_down` handlers
    /// - Jetstream: `UiEvent::KeyPressed` / `InputSystem` action bindings
    KeyPressed {
        key: String,
        modifiers: EventModifiers,
    },

    /// Directional navigation (Jetstream gamepad / keyboard arrows).
    ///
    /// Emitted by: Focus system, list navigation, grid navigation
    /// - GPUI: Arrow key handlers
    /// - Jetstream: Gamepad d-pad / `InputSystem` directional actions
    Navigate {
        direction: NavigateDirection,
    },

    /// Scroll position changed.
    ///
    /// Emitted by: ScrollShell, DataTable
    /// - GPUI: Scroll event handlers
    /// - Jetstream: `UiEvent::ScrollChanged`
    ScrollChanged {
        offset_x: f32,
        offset_y: f32,
    },
}

/// The payload of a value-change or selection-change event.
#[derive(Debug, Clone, PartialEq)]
pub enum EventValue {
    /// A string value (text input, select option ID, date string).
    Text(String),
    /// A numeric value (slider position, rating).
    Number(f64),
    /// A boolean value (checkbox, switch).
    Bool(bool),
    /// Multiple selected values (multi-select, checkbox group).
    Multiple(Vec<String>),
}

/// Phase of a drag interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragPhase {
    Started,
    Moved,
    Ended,
}

/// Modifier key state for keyboard events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EventModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Direction for spatial navigation (keyboard arrows, gamepad d-pad).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigateDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Describes which semantic events a component category emits.
///
/// Used by the adapter trait to know which events to wire up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentEventProfile {
    /// Buttons, links, menu items: Activated, Hovered, FocusChanged
    Action,
    /// Text inputs, selects, sliders: ValueChanged, FocusChanged, Submitted
    ValueEntry,
    /// Checkboxes, switches, radios: ValueChanged, FocusChanged
    Toggle,
    /// Dialogs, drawers, popovers: OpenChanged, Cancelled
    Overlay,
    /// Tabs, segmented controls: SelectionChanged, FocusChanged
    Selection,
    /// Accordions, collapsibles: OpenChanged, FocusChanged
    Disclosure,
    /// Drag handles, split views: DragChanged
    Draggable,
    /// Scroll containers: ScrollChanged
    Scrollable,
    /// Forms: Submitted, Cancelled
    Form,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_events_carry_expected_payloads() {
        let activated = SemanticEvent::Activated;
        assert_eq!(activated, SemanticEvent::Activated);

        let value_changed = SemanticEvent::ValueChanged {
            value: EventValue::Text(String::from("hello")),
        };
        match &value_changed {
            SemanticEvent::ValueChanged { value } => {
                assert_eq!(value, &EventValue::Text(String::from("hello")));
            }
            _ => panic!("expected ValueChanged"),
        }

        let focus = SemanticEvent::FocusChanged { gained: true };
        match focus {
            SemanticEvent::FocusChanged { gained } => assert!(gained),
            _ => panic!("expected FocusChanged"),
        }
    }

    #[test]
    fn event_value_variants_are_distinct() {
        let text = EventValue::Text(String::from("option-1"));
        let number = EventValue::Number(42.0);
        let boolean = EventValue::Bool(true);
        let multi = EventValue::Multiple(vec![String::from("a"), String::from("b")]);

        assert_ne!(text, number);
        assert_ne!(boolean, multi);
    }

    #[test]
    fn modifiers_default_to_false() {
        let mods = EventModifiers::default();
        assert!(!mods.shift);
        assert!(!mods.ctrl);
        assert!(!mods.alt);
        assert!(!mods.meta);
    }

    #[test]
    fn drag_phases_are_ordered() {
        assert_ne!(DragPhase::Started, DragPhase::Moved);
        assert_ne!(DragPhase::Moved, DragPhase::Ended);
    }

    #[test]
    fn component_profiles_cover_all_interaction_patterns() {
        let profiles = [
            ComponentEventProfile::Action,
            ComponentEventProfile::ValueEntry,
            ComponentEventProfile::Toggle,
            ComponentEventProfile::Overlay,
            ComponentEventProfile::Selection,
            ComponentEventProfile::Disclosure,
            ComponentEventProfile::Draggable,
            ComponentEventProfile::Scrollable,
            ComponentEventProfile::Form,
        ];
        assert_eq!(profiles.len(), 9);
    }
}
