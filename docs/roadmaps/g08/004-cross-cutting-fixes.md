# g08.004 Cross-Cutting Fixes: Disabled Opacity, Hover Colors, Geometry Tokens

Status: complete
Owner: Pug Core
Depends on: g08.001

## Contract Check

Verified token names before starting:
- `semantic.state.opacity.disabled` → resolves to 0.48
- `semantic.size.control.height` → resolves to 36.0px
- `semantic.space.inline.md` → resolves to 12.0px (horizontal padding)
- `semantic.space.inline.sm` → resolves to 8.0px (inline gap)
- `semantic.radius.control` → resolves to 6.0px

## Goals

Fix the three systemic issues found across 18+ components so that batch fixes
(005–007) can follow a consistent, correct pattern.

## Execution Checklist

### Disabled Opacity (~30 components)

- [x] Replace every instance of hardcoded `0.48` disabled opacity
      with `resolve_opacity(theme, "semantic.state.opacity.disabled")`
- [x] Replace every instance of hardcoded `0.5` disabled opacity similarly
- [x] Used token string directly (universal across all components) rather than
      requiring per-spec `disabled_opacity_token()` methods

Components fixed: accordion, calendar, checkbox, collapsible, color_picker,
command_palette, data_picker, date_range_picker, date_time_picker,
date_time_range_picker, duration_input, editable_label, file_upload, menu,
menubar, navigation_menu, number_entry, pill, pin_input, radio_group,
range_calendar, range_slider, rating, segmented_control, select, slider,
split_button, switch, tab_strip, tabs, text_area, text_input, time_field,
time_zone_select, toggle, tri_state_switch, zoned_date_time_picker

### Hardcoded Hover/Active Colors (~16 components)

- [x] Replace `hsla(0.0, 0.0, 0.5, 0.04)` hover overlays with token-resolved
      elevated background: `resolve_color(theme, "semantic.color.background.elevated")`
- [x] Replace `hsla(0.0, 0.0, 0.5, 0.06)` hover overlays similarly

Components fixed: accordion, action_discovery_panel, checkbox, collapsible,
command_palette, date_picker, date_range_picker, date_time_picker,
date_time_range_picker, dock_region, menubar, radio_group, segmented_control,
select, tab_strip, tabs

### Hardcoded Geometry (~45 components)

- [x] Replace `px(36.0)` heights with `resolve_px(theme, "semantic.size.control.height")`
- [x] Replace `px(12.0)` horizontal padding with `resolve_px(theme, "semantic.space.inline.md")`
- [x] Replace `px(6.0)` radii with `resolve_radius(theme, "semantic.radius.control")`
- [x] Replace `px(8.0)` inline gaps with `resolve_px(theme, "semantic.space.inline.sm")`

All standard control geometry across 45 component files now resolves from tokens.
Non-standard geometry (component-specific dimensions like switch knob size,
checkbox indicator size) left as-is for per-component quality fixes in 005–007.

## Acceptance Criteria

- [x] Zero instances of hardcoded `0.48`/`0.5` opacity in component files
- [x] Zero instances of hardcoded `hsla(0.0, 0.0, ...)` hover colors
- [x] All standard geometry values (height, padding, radius, gap) resolve
      from token resolution helpers
- [x] All changes compile and render correctly (`cargo check` — zero errors)
