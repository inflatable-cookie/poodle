//! The single Rust component implementation.
//!
//! Each component here is a pure function `Spec + Theme → poodle_node::Node`.
//! No backend types, no measurement, no window: what a Button *is*, decided
//! once. Per-backend adapters (GPUI's in this repo, Jetstream's in its own)
//! interpret the node tree; the parity evidence for each backend lives with
//! that backend, against fixtures this crate can generate headlessly.
//!
//! Started 2026-08-04 with `Select` — deliberately the hardest component
//! (overlay anchoring, filtering, groups, a trigger with nested clickables) —
//! so the vocabulary was proven where it was most likely to fail. The rest of
//! the tier migrates component-by-component; each port deletes its
//! `packages/jetstream/components` and `packages/gpui/components`
//! predecessors once its parity fixtures are green on both backends.

pub mod avatar;
pub mod accordion;
pub mod alert_dialog;
pub mod badge;
pub mod banner;
pub mod breadcrumbs;
pub mod button;
pub mod callout;
pub mod card;
pub mod checkbox;
pub mod code;
pub mod collapse_toggle;
pub mod collapsible;
pub mod color;
pub mod context_menu;
pub mod detail_item;
pub mod dialog;
pub mod drawer;
pub mod empty_state;
pub mod eyebrow;
pub mod field;
pub mod field_set;
pub mod icon;
pub mod menu;
pub mod menubar;
pub mod meta_bar;
pub mod meta_item;
pub mod icon_button;
pub mod pill;
pub mod presentation;
pub mod progress;
pub mod radio_group;
pub mod segmented_control;
pub mod select;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod text;
pub mod text_input;
pub mod toolbar;
pub mod text_link;
pub mod tooltip;

pub use avatar::avatar;
pub use accordion::accordion;
pub use alert_dialog::{alert_dialog, AlertDialogHandlers, DEFAULT_WORKING_LABEL};
pub use badge::badge;
pub use banner::banner;
pub use breadcrumbs::breadcrumbs;
pub use button::button;
pub use callout::callout;
pub use card::card;
pub use checkbox::checkbox;
pub use code::code;
pub use collapse_toggle::collapse_toggle;
pub use collapsible::collapsible;
pub use context_menu::context_menu;
pub use detail_item::{detail_item, detail_item_with_slots};
pub use dialog::dialog;
pub use drawer::drawer;
pub use empty_state::empty_state;
pub use eyebrow::eyebrow;
pub use field::field;
pub use field_set::field_set;
pub use icon::icon;
pub use menu::menu;
pub use menubar::menubar;
pub use meta_bar::{meta_bar, meta_bar_sep};
pub use meta_item::meta_item;
pub use icon_button::icon_button;
pub use pill::pill;
pub use progress::progress;
pub use radio_group::radio_group;
pub use segmented_control::segmented_control;
pub use select::{select, SelectHandlers};
pub use separator::separator;
pub use skeleton::skeleton;
pub use slider::{slider, SliderHandlers};
pub use spinner::spinner;
pub use switch::switch;
pub use tabs::{tabs, TabHandler};
pub use text::text;
pub use toolbar::toolbar;
pub use text_input::text_input;
pub use text_link::text_link;
pub use tooltip::tooltip;
