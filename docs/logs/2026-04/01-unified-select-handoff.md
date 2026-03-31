# Handoff: Unified Select Component

Created: 2026-04-01
Thread: g09 visual review → component consolidation → unified Select
Roadmap: g09 (active), component library polish pass

---

## What This Thread Was Doing

This thread started as a Svelte visual review pass through the preview
app and evolved into a significant component consolidation and polish
effort. Over the course of the session:

- **Density/sizing rollout** across all three runtimes (Svelte, GPUI,
  Jetstream) — 74+ specs, presentation modules, adapter wiring
- **Component consolidation**: PinInput + TotpInput → CodeInput,
  RangeCalendar → Calendar (mode="range"), NavCardGrid removed,
  SearchField → SearchInput, DetailRow → DetailItem
- **Contract audit**: all 35 seed contracts → detailed, deep audit of
  70 components, OrderBy/Pagination rewritten from scratch
- **Visual fixes**: ~40 individual component fixes (spacing, sizing,
  backgrounds, animations, hover states, accessibility)
- **Dialog flexibility**: header/footer slots, width presets, bare mode
- **Field rework**: description moved to info popover via Popover component

The thread ended at the decision to consolidate Select + Combobox into
a single unified Select component. The architecture has been fully
designed but implementation has not started.

## Why It Matters

The component library has three separate selection components (Select,
Combobox, and the proposed Listbox) that represent variants of the same
interaction: "pick one thing from a list." This fragmentation is
confusing for consumers (especially AI agents) and doesn't translate
well to GPUI/Jetstream where there is no native `<select>`.

A unified Select with mode escalation (native → custom → searchable →
freeform) provides one component name for all selection use cases,
simpler contract surface for cross-runtime parity, and a clean
migration path from the current components.

## Current State

### What exists today

- **Select.svelte** — native `<select>`, 23 props including async
  loading, groups, clearable. Works well for plain text options.
- **Combobox.svelte** — custom dropdown with filtering, 9 props.
  Keyboard nav, highlight tracking, above/below placement.
- Both have size/sizeRole/density support.
- Both are documented with contracts, component-docs, and specimens.

### What has been designed (not yet built)

A unified Select with these modes:

1. **Native mode** (default) — renders `<select>`, identical to today
2. **Custom mode** (`native={false}` or option slot provided) — custom
   dropdown with rich option rendering
3. **Searchable mode** (`searchable`) — custom dropdown with filter input
4. **Freeform mode** (`searchable` + `freeform`) — text input IS the
   value, options are suggestions not requirements

### Key design decisions made

- Mode resolution cascade: `native` prop → `searchable` → slot presence → default native
- Unified `SelectOption` type: `{ value, label, description?, icon?, disabled? }`
- Grouped options: `SelectOptionGroup[]` with `label` + `options`
- Slots: `option` (rich rendering), `trigger` (custom trigger), `empty` (no matches)
- `freeform` prop: when true with `searchable`, the query text becomes the value if no option selected — options act as autocomplete suggestions
- Async loading: `loadOptions` replaces `loadItems`/`loadGroups`
- Form submission: hidden `<input>` when `name` is provided in custom mode
- Filtering: client-side substring on `label`, `queryChange` event for server-side
- `clearable`: clear option in native mode, X button in custom mode

### Prop table (designed)

See the full prop table in the thread context. Key props:

| Prop | Purpose |
|------|---------|
| `options` | `SelectOption[] \| SelectOptionGroup[]` |
| `searchable` | Enables filter input + custom dropdown |
| `freeform` | With searchable, allows arbitrary text as value |
| `native` | Explicit mode override (undefined = auto) |
| `loadOptions` | Async option loader |
| `loadKey` | Invalidation key for async |
| `clearable` | Clear/reset support |
| `emptyMessage` | "No matches" text |

### Events: `valueChange`, `queryChange`, `openChange`

## Boundaries

### In scope for the unified Select

- All features of current Select (native, groups, async, clearable)
- All features of current Combobox (custom dropdown, filtering, keyboard)
- Rich option rendering via slot
- Freeform text input mode
- Custom trigger rendering via slot
- Form submission in custom mode via hidden input
- Above/below placement for custom dropdown
- Deprecation of Combobox as re-export alias

### Out of scope

- Multiple selection (future-proof the types but don't implement)
- Virtual scrolling for large option lists (can be added later)
- Drag-to-reorder selected items (not a Select concern)
- Tag/chip input (different component)
- Nested/tree options (too complex for this pass)
- Changes to the Rust spec surface (keep SelectSpec, deprecate ComboboxSpec later)
- GPUI/Jetstream implementation (Svelte first, port after)

## Important Context

### User preferences and judgments observed in this thread

1. **Svelte is the reference implementation** — GPUI/Jetstream follow
2. **Contracts must stay in sync** — every Svelte change updates its contract
3. **Zero hardcoded px values** — all dimensions from tokens or presentation helpers
4. **Size controls dimensions, density controls spacing** — the global rule
5. **Components that embed children must forward size/density** — recurring bug pattern
6. **The user values consolidation** — actively removed redundant components (NavCardGrid, PinInput, RangeCalendar, SearchField rename)
7. **Specimens should show diverse use cases** — not just repeat the same pattern
8. **The user is building for AI agent consumers** — API discoverability matters

### Technical patterns to follow

- Use `getUiPresentation()` + `resolveSemanticControlSize()` for size resolution
- Emit `data-size`, `data-density` on root element
- Use Popover component for dropdown positioning (not custom absolute positioning)
- Use `bind:open` carefully — Popover now correctly writes to `open` in controlled mode (fixed this session)
- sr-only clip pattern for hidden inputs (not opacity: 0)
- Svelte transitions for enter/exit animations (not manual CSS state machines)

### Things that broke during this thread

- `bind:open` on Popover was broken in controlled mode (fixed)
- Hidden inputs with `position: absolute; opacity: 0` caused scroll jumps (fixed with clip pattern)
- Treatment tokens with baked-in transparency override `var()` fallbacks (Popover background issue)
- CSS `calc()` can't divide percentage units by numbers (Slider fill issue)

## Suggested Next Move

**Build the unified Select component in Svelte.**

Implementation sequence:

1. Add the unified `SelectOption` / `SelectOptionGroup` types to `types.ts`
2. Build the custom-mode dropdown as an internal rendering path within Select.svelte (absorb Combobox's keyboard nav, highlight tracking, placement logic)
3. Refactor Select.svelte with the mode cascade: native vs custom based on props/slots
4. Wire up `searchable` (filter input replaces trigger)
5. Wire up `freeform` (query text as value when no option selected)
6. Wire up `option` slot for rich rendering
7. Wire up `trigger` slot for custom trigger
8. Handle groups in custom mode
9. Handle async loading in custom mode
10. Add `clearable` X button in custom mode
11. Update specimen to show all modes
12. Update component-docs with full prop surface
13. Update contract
14. Deprecate Combobox as `Select searchable` re-export

Start with steps 1-4 to get the core working, then iterate.

## Completion Protocol

When the unified Select is implemented:

1. All existing Select specimens still render correctly (native mode)
2. Combobox specimens work via `<Select searchable>` with no visual change
3. Rich option rendering works via the `option` slot
4. Freeform input works via `searchable freeform`
5. Grouped options render correctly in both native and custom modes
6. Keyboard navigation matches current Combobox behavior in custom mode
7. Form submission works in both modes
8. `effigy health` passes
9. Contract updated at `docs/contracts/foundation/select.md`
10. Component-docs entry covers all props and modes
11. Combobox.svelte is a deprecated re-export
12. Log the completion in `docs/logs/2026-04/`
