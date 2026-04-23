# poodle-jetstream-components

Status: active
Updated: 2026-04-23

Jetstream render functions for the full Poodle component surface. Every
component in this crate is a pure function: it takes a spec and a theme
provider, and returns a `JsEl` element tree ready for the Jetstream runtime.

## Component Pattern

Every component follows the same signature:

```rust
pub fn js_<component>(spec: &<ComponentSpec>, theme: &JetstreamThemeProvider) -> JsEl
```

Example — rendering a Button:

```rust
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_specs::{ButtonSpec, ButtonVariant, ButtonTone, ControlSize};
use poodle_tokens::ThemeDefinition;

let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark);

let el = js_button(
    &ButtonSpec::new()
        .with_label("Save changes")
        .with_variant(ButtonVariant::Solid)
        .with_tone(ButtonTone::Accent)
        .with_size(ControlSize::Md),
    &theme,
);

// Compose into a parent element
let root = div()
    .flex_row()
    .gap(8.0)
    .child(el);

game_ui.render_immediate(&root);
```

## Instantiation in Practice

Components are typically instantiated directly from spec builders.
Full usage pattern from the preview app:

```rust
use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tri_state_switch::js_tri_state_switch;
use poodle_jetstream_components::theme_ext::resolve_color;
use poodle_specs::{CheckState, ControlSize, TriStateSwitchSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    div().flex_col().gap(24.0)
        .child(
            js_tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_state(CheckState::Unchecked)
                    .with_label("Excluded"),
                theme,
            )
        )
}
```

## Implemented Components

### Primitives (61)

| Module | Render Function | Spec |
|---|---|---|
| `accordion` | `js_accordion` | `AccordionSpec` |
| `alert_dialog` | `js_alert_dialog` | `AlertDialogSpec` |
| `breadcrumbs` | `js_breadcrumbs` | `BreadcrumbsSpec` |
| `bulk_action_bar` | `js_bulk_action_bar` | `BulkActionBarSpec` |
| `button` | `js_button` | `ButtonSpec` |
| `calendar` | `js_calendar` | `CalendarSpec` |
| `callout` | `js_callout` | `CalloutSpec` |
| `card` | `js_card` | `CardSpec` |
| `checkbox` | `js_checkbox` | `CheckboxSpec` |
| `code` | `js_code` | `CodeSpec` |
| `code_input` | `js_code_input` | `CodeInputSpec` |
| `collapse_toggle` | `js_collapse_toggle` | `CollapseToggleSpec` |
| `collapsible` | `js_collapsible` | `CollapsibleSpec` |
| `color_picker` | `js_color_picker` | `ColorPickerSpec` |
| `context_menu` | `js_context_menu` | `ContextMenuSpec` |
| `date_picker` | `js_date_picker` | `DatePickerSpec` |
| `date_range_picker` | `js_date_range_picker` | `DateRangePickerSpec` |
| `date_time_picker` | `js_date_time_picker` | `DateTimePickerSpec` |
| `date_time_range_picker` | `js_date_time_range_picker` | `DateTimeRangePickerSpec` |
| `date_time_zone_picker` | `js_date_time_zone_picker` | `DateTimeZonePickerSpec` |
| `detail_item` | `js_detail_item` | `DetailItemSpec` |
| `dialog` | `js_dialog` | `DialogSpec` |
| `drawer` | `js_drawer` | `DrawerSpec` |
| `duration_input` | `js_duration_input` | `DurationInputSpec` |
| `editable_label` | `js_editable_label` | `EditableLabelSpec` |
| `eyebrow` | `js_eyebrow` | `EyebrowSpec` |
| `field` | `js_field` | `FieldSpec` |
| `field_set` | `js_field_set` | `FieldSetSpec` |
| `file_upload` | `js_file_upload` | `FileUploadSpec` |
| `floating_overlay` | `js_floating_overlay` | `FloatingOverlaySpec` |
| `form_actions` | `js_form_actions` | `FormActionsSpec` |
| `grid` | `js_grid` | `GridSpec` |
| `hover_card` | `js_hover_card` | `HoverCardSpec` |
| `icon_button` | `js_icon_button` | `IconButtonSpec` |
| `list_card` | `js_list_card` | `ListCardSpec` |
| `list_card_counter` | `js_list_card_counter` | `ListCardCounterSpec` |
| `list_grid` | `js_list_grid` | `ListGridSpec` |
| `menu` | `js_menu` | `MenuSpec` |
| `menubar` | `js_menubar` | `MenubarSpec` |
| `meta_bar` | `js_meta_bar` | `MetaBarSpec` |
| `meta_item` | `js_meta_item` | `MetaItemSpec` |
| `meter` | `js_meter` | `MeterSpec` |
| `nav_card` | `js_nav_card` | `NavCardSpec` |
| `navigation_menu` | `js_navigation_menu` | `NavigationMenuSpec` |
| `number_input` | `js_number_input` | `NumberInputSpec` |
| `order_by` | `js_order_by` | `OrderBySpec` |
| `pagination` | `js_pagination` | `PaginationSpec` |
| `pagination_summary` | `js_pagination_summary` | `PaginationSummarySpec` |
| `password_requirements` | `js_password_requirements` | `PasswordRequirementsSpec` |
| `pill` | `js_pill` | `PillSpec` |
| `popover` | `js_popover` | `PopoverSpec` |
| `progress` | `js_progress` | `ProgressSpec` |
| `radio_group` | `js_radio_group` | `RadioGroupSpec` |
| `range_slider` | `js_range_slider` | `RangeSliderSpec` |
| `rating` | `js_rating` | `RatingSpec` |
| `segmented_control` | `js_segmented_control` | `SegmentedControlSpec` |
| `select` | `js_select` | `SelectSpec` |
| `separator` | `js_separator` | `SeparatorSpec` |
| `skeleton` | `js_skeleton` | `SkeletonSpec` |
| `slider` | `js_slider` | `SliderSpec` |
| `switch` | `js_switch` | `SwitchSpec` |
| `tab_strip` | `js_tab_strip` | `TabStripSpec` |
| `tabs` | `js_tabs` | `TabsSpec` |
| `text_input` | `js_text_input` | `TextInputSpec` |
| `toggle_group` | `js_toggle_group` | `ToggleGroupSpec` |
| `toolbar` | `js_toolbar` | `ToolbarSpec` |
| `tooltip` | `js_tooltip` | `TooltipSpec` |
| `tri_state_switch` | `js_tri_state_switch` | `TriStateSwitchSpec` |

### Composites (47)

| Module | Render Function | Spec |
|---|---|---|
| `action_discovery_panel` | `js_action_discovery_panel` | `ActionDiscoveryPanelSpec` |
| `app_header` | `js_app_header` | `AppHeaderSpec` |
| `audio_player` | `js_audio_player` | `AudioPlayerSpec` |
| `block_editor` | `js_block_editor` | `BlockEditorSpec` |
| `card_radio_group` | `js_card_radio_group` | `CardRadioGroupSpec` |
| `command_palette` | `js_command_palette` | `CommandPaletteSpec` |
| `confirm_action` | `js_confirm_action` | `ConfirmActionSpec` |
| `data_table` | `js_data_table` | `DataTableSpec` |
| `detail_section` | `js_detail_section` | `DetailSectionSpec` |
| `empty_state` | `js_empty_state` | `EmptyStateSpec` |
| `filter_toolbar` | `js_filter_toolbar` | `FilterToolbarSpec` |
| `form_dialog` | `js_form_dialog` | `FormDialogSpec` |
| `form_layout` | `js_form_layout` | `FormLayoutSpec` |
| `list_container` | `js_list_container` | `ListContainerSpec` |
| `media_browse_panel` | `js_media_browse_panel` | `MediaBrowsePanelSpec` |
| `media_picker` | `js_media_picker` | `MediaPickerSpec` |
| `media_preview` | `js_media_preview` | `MediaPreviewSpec` |
| `media_thumbnail` | `js_media_thumbnail` | `MediaThumbnailSpec` |
| `metric_tile` | `js_metric_tile` | `MetricTileSpec` |
| `page_header` | `js_page_header` | `PageHeaderSpec` |
| `picker_shell` | `js_picker_shell` | `PickerShellSpec` |
| `relation_picker` | `js_relation_picker` | `RelationPickerSpec` |
| `selection_summary` | `js_selection_summary` | `SelectionSummarySpec` |
| `sidebar_nav` | `js_sidebar_nav` | `SidebarNavSpec` |
| `toast_host` | `js_toast_host` | `ToastHostSpec` |
| `toast_stack` | `js_toast_stack` | `ToastStackSpec` |
| `video_player` | `js_video_player` | `VideoPlayerSpec` |

### Workstation (13)

| Module | Render Function | Spec |
|---|---|---|
| `detail_shell` | `js_detail_shell` | `DetailShellSpec` |
| `dock_region` | `js_dock_region` | `DockRegionSpec` |
| `split_view` | `js_split_view` | `SplitViewSpec` |

(Full workstation surface in `poodle-workstation` crate.)

## Parity

Current parity status is tracked in:

```
packages/jetstream/cross-runtime-parity-report.json
```

Format summary:

```json
{
  "generation": "g10.014",
  "runtime": "jetstream",
  "summary": {
    "componentExports": 117,
    "parityTiers": {
      "full": 109,
      "partial": 8,
      "skip": 0
    },
    "adapterTests": 165,
    "verification": { "cargoCheck": "pass", "cargoTest": "165 tests passing" }
  }
}
```

Parity tiers:
- **Tier 1 (full)** — Strict visual and behavioral parity with the Svelte reference
- **Tier 2 (partial)** — Visual parity with documented, approved native adaptations
- **Tier 3 (skip)** — Out of scope for this runtime (none currently)

## Dependencies

```toml
[dependencies]
poodle-jetstream          = { path = "../adapter" }
poodle-adapter            = { path = "../../contracts/adapter" }
poodle-specs              = { path = "../../contracts/components" }
poodle-tokens             = { path = "../../contracts/tokens" }
jetstream-runtime         = { ... }
taffy                     = "0.9"
glam                      = "0.29"
```

## Related Crates

- `poodle-jetstream` — adapter, theme provider, layout mapping
- `poodle-specs` — spec structs passed into render functions
- `poodle-tokens` — `ThemeDefinition`, `DensityDefinition`, `ControlSizeDefinition`
- `jetstream-runtime::ui_element` — `JsEl` and all fluent builder functions
- Developer guide: `docs/guides/jetstream-developer-guide.md`
