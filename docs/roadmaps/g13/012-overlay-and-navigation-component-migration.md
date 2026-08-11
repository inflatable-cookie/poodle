# g13.012 Overlay And Navigation Component Migration

Status: gated on g13.008 adopt verdict
Owner: Poodle core
Depends on: `g13.009`, `g13.011`

## Objective

Migrate components that exercise focus scopes, dismissal, positioning,
portals, roving navigation, typeahead, and nested interaction.

## Scope

- Dialog/drawer, tooltip/popover, menu/command, disclosure, tabs, navigation,
  picker, and related families.
- Make every environment dependency a typed adapter capability.
- Preserve DOM portal semantics and native overlay/focus behavior.

## Acceptance

- Focus return, escape/outside dismissal, nesting, keyboard navigation, and
  accessibility match current contracts.
- No generated framework lifecycle or hidden backend overlay fork.
- Capability support and degradations are reported per runtime.

## Next

`g13.013` completes higher-order composition migration.
