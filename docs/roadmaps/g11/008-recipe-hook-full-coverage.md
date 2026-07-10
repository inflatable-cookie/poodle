# g11.008 Recipe Hook Full Coverage

Status: complete (2026-07-10)
Owner: Poodle core
Depends on: `g11.005` (recipe contract promoted, architecture 007), god-files
batch 5 (component styles extracted to co-located `.css` files, which makes
this sweep a plain-CSS transform rather than 65 `<style>`-block edits)
Updated: 2026-07-10

## Purpose

Take the recipe contract from "proven on 6 components" to full library
coverage, so a consuming app can restyle any component's appearance from its
own scope — recipe variables only, zero CSS overriding. Decision recorded
here: full sweep now (rather than hooks-on-demonstrated-need), because all
consumers are first-party and a complete surface is cheap to walk back
while adoption is local.

## Baseline (2026-07-10)

`packages/svelte/preview/artifacts/recipe-inventory.json`:

- 6 components with hooks (34 hooks): Button, Card, Pill, PageHeader,
  ListCard, BulkActionBar
- 15 components with appearance variables but no hooks (46 candidates):
  Avatar, Button (partial), Callout, Checkbox, Collapsible, FormActions,
  IconButton, Pill (partial), Radio, Separator, Spinner, Surface, Switch,
  TextInput, TokenInput
- 44 components with no appearance variables at all: their CSS reads
  semantic tokens directly in property positions — no override point.

## Method

Per architecture 007, unchanged:

- Public form `--poodle-recipe-<component>[-<variant>]-<slot>[-<state>]`;
  slots from contract anatomy; states from documented visual states.
- Components consume the namespace, never define in it.
- Resolution chain: recipe hook → (treatment role) → semantic token. No
  override active ⇒ pixel-identical to token default (strictly additive).

Two mechanical shapes:

1. **Candidate components** (local appearance var exists): wrap the var's
   definition — `--poodle-checkbox-fill: var(--poodle-recipe-checkbox-fill,
   <existing value>);`
2. **Bare components** (token read directly in a property): hook inline at
   the property — `background: var(--poodle-recipe-x-panel-fill,
   var(--poodle-color-...));` — introducing a local var only when the same
   value is used in several places.

Hook granularity: fill / border / text / shadow / radius per meaningful
slot, plus -hover/-active/-selected states where the CSS already
distinguishes them. Metric variables (sizing, spacing, typography scale)
stay internal — the size/density system is not part of the recipe surface.

## Batches

- [x] 0. Infra: inventory script scans extracted `.css` files (was
  `.svelte`-only — broken silently by the god-files extraction)
- [x] 1. Candidate components (15, listed above) — wrap existing vars
- [x] 2. Bare: form/input family (DurationInput, Field, FileUpload,
  NumberInput*, SearchInput*, Slider, TextArea*, DatePicker family,
  CodeInput*, RelationPicker, Select, OrderBy, TokenInput remainder)
- [x] 3. Bare: overlay/disclosure/navigation (Accordion, Collapsible
  remainder, CommandPalette, Dialog family, Drawer, Menu, Popover, Tooltip,
  Tabs, SegmentedControl, Toast, SplitButton, DockRegion)
- [x] 4. Bare: data/display (DataTable, Tree, ListCard remainder, LogList,
  EditableList, DetailItem/Section, Skeleton, EmptyState, StatCard,
  Calendar, Pagination, Rating, MediaCard, VideoPlayer, AudioPlayer,
  BlockEditor)
- [x] 5. Bare: chrome/shell (AppHeader, PageHeader remainder, Sidebar,
  StatusBar, Workspace pieces, FormLayout/Dialog, remaining stragglers)
- [x] 6. Regenerate inventory; update architecture 007 adopter list;
  Playwright verification (see below); consumer typechecks

Components marked * may live inside another component's css file — the
inventory attributes hooks by file name, which is acceptable.

## Validation

Per batch:

- Playwright against the preview: for a sample of hooks in the batch, set
  the recipe var on a component ancestor → computed style changes to the
  override; remove → computed style returns to the token default.
- Preview builds; docs:lint green.

Final: underlay `effigy check:types`, acme-admin/dairy `bun x tsc` (CSS-only
change, so typechecks are a smoke signal, not the real gate — the Playwright
additive check is the gate).

## Non-Goals

- Rust mirror of every hook (correspondence stays contract-level: a public
  hook maps to a spec field / token override on the GPUI/Jetstream side,
  filled in on demand per component).
- Per-contract "Recipe Hooks" prose sections — the machine-generated
  inventory is the source of truth (g11.005 precedent).
- Metric/size/density overrides — explicitly unsupported.

## Completion Notes (2026-07-10)

Final inventory: **116 components, 973 recipe hooks, 0 candidates** (from
6 components / 34 hooks / 46 candidates). The component count grew because
the fixed scanner now sees every style-bearing file, including components
with no custom properties at all.

What the sweep surfaced:

- **Inventory scanner** was silently broken by the god-files CSS extraction
  (`.svelte`-only glob) and classified by variable presence rather than
  unhooked definitions; both fixed. Prop-channel definitions (template
  literals, `style:--x` directives) are excluded from candidates.
- **Switch defect**: the default primary tone emitted an inline
  `--poodle-switch-on-color` unconditionally, shadowing any app-scope
  recipe override. Fixed — the stylesheet default already resolves accent
  through the recipe chain; explicit onColor/offColor props still win.
- Naming came out per architecture 007: slots from anatomy class parts
  (`surface`, `trigger`, `item`, `th`…), qualifiers from data-attributes
  (`primary`, `danger`, `not-current-month`, `theme-light`) and pseudo
  states (`hover`, `focus`, `checked`). `:not(...)` is stripped before
  qualifier extraction.
- A handful of properties stayed unhooked by design: values already routed
  through local resolution vars/treatment roles (hooked at the var), data-URI
  composites, and `currentColor` derivations.

Verification: Playwright additive checks across every family — override
applies from the document root, removal restores the pixel-identical token
default (button, icon-button, avatar, switch, text-input, callout, surface,
separator, spinner, accordion, tabs, select, file-upload, dialog surface,
calendar day, data-table header, tree row, pagination, skeleton,
sidebar-nav). docs:lint green; underlay/acme-admin/dairy typechecks green.

## Next Task

g11 runway complete (002–008). Rust-side hook correspondence stays
on-demand per architecture 007; hook renames go through the g11.001 wave
process.
