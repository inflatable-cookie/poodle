# 007 Appearance Recipe Contract

Status: active
Updated: 2026-11-26
Depends on: `006-headless-core-and-machine-model.md`
Supersedes: `005-treatment-system-and-recipe-variables.md` (legacy override layer retired 2026-07-13; Recipes are the single override surface)
Promoted from: `docs/knowledge/specs/026-appearance-recipes-and-downstream-override-strategy.md` (g11.005)

## What A Recipe Is

A recipe is the set of **component-scoped CSS custom properties** a component
resolves its appearance from. Recipes are the third override depth in the
sanctioned ladder (tokens → style → recipe), and the
mechanism apps use to restyle individual components
without forking them or redefining shared meaning.

Decision (g11.005): recipes are **CSS custom-property contracts, not a
JavaScript API**. They are plain data by nature — a variable name and a
value — which keeps them framework-free, SSR-safe, zero-runtime, and
mappable to the Rust runtimes (each public recipe variable corresponds to a
spec field / token override on the GPUI/Jetstream side).

## Naming And Resolution

- Public variable form: `--poodle-recipe-<component>[-<variant>]-<slot>[-<state>]`
  (e.g. `--poodle-recipe-button-fill-hover`,
  `--poodle-recipe-button-primary-fill`, `--poodle-recipe-pill-border`).
- The `--poodle-recipe-*` namespace is **read-only for components**: they
  consume it but never define values in it, so an app-scoped value always
  wins through the cascade. (Component-local `--poodle-<component>-*`
  variables are internal resolution variables — components define them, so
  ancestor overrides cannot reach them. This distinction was verified live
  in g11.005.)
- Components read the recipe hook first, then the style-adjusted semantic
  token and its base fallback. A style applies across a subtree independently
  of its colour theme. The `fill`, `surface` and `selection` switches expand
  to token overrides during the build. Shared CSS consumes the resulting
  tokens.
- Variant-specific rules get variant-scoped hooks (e.g.
  `-button-primary-*`); semantic tone rules (danger/success) keep token
  semantics unless a variant hook is added on demand.
- Slots correspond to the component's contract anatomy parts; states to its
  documented visual states.

Typography follows the same recipe → style role → component fallback order.
For example, a button reads `--poodle-recipe-button-label-font-size` before
the control-label role for the button's resolved size. An `initial` style
value falls through to the component's existing value, including in a
default-styled subtree nested inside quiet. A case recipe or role sets CSS
`text-transform` and leaves the author's text and accessible name intact.
Type recipes affect text metrics. Control height continues to come from the
size contract.

The selection switch resolves selected fills and borders through
`color.select.*` and reuses existing text and indicator roles across Select,
Tabs, ToggleGroup, SegmentedControl, Card (including CardRadioGroup and
CardToggleGroup), ThemeSelect, AgentQuestion, and Rating. Existing recipe
hooks for Select options, Tabs edges and fills, ToggleGroup items,
SegmentedControl labels, Card selected edges, and Rating filled glyphs still
take precedence in all three selection styles (ThemeSelect and AgentQuestion
expose no selected-state recipe hooks). Selected edges on Card, ThemeSelect, and
AgentQuestion read `--poodle-style-selection-itemBorder` (and selected fills
read `--poodle-style-selection-itemFill`), keeping `accent-outline` identical
while suppressing borders under `accent-tint` and `neutral`. A recipe can
supply a visible Tabs outline even when `accent-tint` supplies a transparent
fallback border. See
[002 Token System and Package Layout](002-token-system-and-package-layout.md#modes).

The selected state of sidebar nav, navigation menu, pagination, stepper,
menubar, menu surface, dock region and breadcrumbs follows the same chain.
Where a component's hand-mixed default has no matching role, the fallback
reads `--poodle-style-selection-itemFill` (unset in `accent-outline`) and
keeps its own mix, so the default style stays pixel-identical while
`accent-tint` and `neutral` reach it. The sidebar's inset ring stays
accent-coloured under every selection style; it is a doubled edge, listed for
the redesign.

The surface switch resolves a container's boundary through
`--poodle-style-surface-border`, `-fill`, `-floatingFill` and `-shadow`:
recipe hook, then style role, then the component's own mix. Card and Dialog
took the chain earlier; this cohort is Popover, Tooltip, Drawer, HoverCard,
CommandPalette, ToastStack, PickerShell, Surface, Callout and EmptyState.
Floating containers read `-floatingFill`, then `-fill`, because they sit
above the page: Popover, Tooltip, Drawer, HoverCard, CommandPalette, the
toast body, PickerShell's popover and modal variants and Surface's elevated
tone. Inline ones read `-fill` alone: Surface, Callout, EmptyState and the
inline PickerShell. Under `tonal` each one drops border and shadow and
separates by fill alone, so a toast keeps only its status bar and an empty
state only its tone. Status and instance tones keep their colour in every
style: Callout's tones and solid fills, EmptyState's search and first-run
variants and Surface's canvas tone read the role for their border only, so
the switch moves the boundary and not the meaning. A few values stay literal
and are listed for the redesign: PickerShell's state block (a nested tone
below the shell, which no role supplies) and the tinted icon circles in
Callout and EmptyState, which are markup noise.

The list, detail and form-layout stylesheets follow the same chains. Selection
summary chips and their overflow pill read the control fill and shadow roles,
because they are filled, removable controls. Clickable nav cards read the
surface boundary at rest and the control hover fill on hover. Table shells,
inline list count pills and rows, detail item surface presentations and the
detail shell's state panel read the surface fill and border. The form-layout
error block keeps its danger tone and reads only the surface border. Text links
stay transparent: they are inline text, not filled controls. Left literal and
listed for the redesign: table headers and inline-list and table dividers
(nested tones and hairlines no role supplies), nav-card and count-badge accent
tints (tone, not treatment), and the tinted info-icon circles (markup noise).

The shell, overlay and status cohort follows the same chain. Bars read
`-fill` alone: AppHeader, StatusBar's chrome, Toolbar and the SplitView
toggles. The floating BulkActionBar and PageLoading's card read
`-floatingFill`, then `-fill`, and take their border and shadow from the
surface role. Pill's neutral tag and SkeletonCard read the surface role for
their boundary, and UpdateCenter's notice reads the surface fill and border.
Controls read the fill roles instead: PageLoading's cancel button, the
interactive MessageCenter rows and RemediationBanner's solid-fill buttons.
Status tones keep their colour in every style (Pill tones, the banner's tint,
the update rejection notice and status indicators). Left literal for the
redesign: MessageCenter's unread tint, which marks state and does not select;
the Progress track; the Skeleton shimmer; Avatar's identity tints; Region's
dashed outline, whose boundary is its content; and ResizeHandle's lines,
which are dividers. No selected, current or checked state sits in this
cohort. The SettingsShell group-title accent is in `sidebar-nav.css`, outside
this cohort, and needs a text role there.

## Public Surface vs Internal Variables

Two classes, machine-classified by
`packages/svelte/preview/scripts/build-recipe-inventory.ts` into
`packages/svelte/preview/artifacts/recipe-inventory.json`:

- **Recipe hooks** (`--poodle-recipe-*`, public): the supported override
  surface. Stable: renaming or removing one is a breaking change handled
  via the g11.001 wave process. Three pre-existing internal tone variables
  remain classified as candidates; new audio controls add no candidates. Two
  mechanical shapes: components with local appearance variables wrap the
  variable definition; components without them hook inline at the property.
  Slot names come from the component's anatomy class parts; qualifiers from
  data-attributes and pseudo-state in the defining selector.

- **Metric variables** (internal): sizing, padding, gaps, font sizes. These
  local fallback variables belong to the size/density system and are NOT
  part of the recipe contract. Overriding them is unsupported (density must never change
  component height — see the size/density contract).

The 2026-11-25 rendering-style ruling removed
`--poodle-recipe-card-theme-iceberg-elevated-shadow` and
`--poodle-recipe-metric-tile-theme-iceberg-fill`. Apps can set
`--poodle-recipe-card-elevated-shadow` or `--poodle-recipe-metric-tile-fill`
on their own subtree.

Regenerate the inventory after styling changes:
`bun packages/svelte/preview/scripts/build-recipe-inventory.ts`.

## Block Slider Shared Role Map

Slider and RangeSlider `appearance="block"` share one role map. Each component
exposes the roles through its own `--poodle-recipe-<component>-block-*` hooks.
The roles are selected fill, selected text, remainder fill, remainder text,
handle fill, handle border, focus ring, and fallback text. Fit thresholds,
insets, and hit-wrapper diameters stay internal metrics, not recipe surface.
Forced-color mappings use the system colors named in the Slider and
RangeSlider contracts rather than author token mixes.

## How Apps Override

Scope overrides to a subtree; the CSS cascade delivers them. No imports, no
registration.

```css
/* app-owned stylesheet */
.checkout-flow {
  /* component-level: recipe hooks. For family-level branding, define an
     app variable once and point several recipe hooks at it. */
  --poodle-recipe-button-shadow: 0 2px 0 rgb(0 0 0 / 0.35);
  --poodle-recipe-pill-fill: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
}
```

Rules (unchanged from spec 026): never redefine semantic token meaning in
place; web-only effects (gradients, backdrop-filter) are valid recipe values
but do not imply cross-runtime parity; structural brand expression belongs
in app-owned wrappers, not recipes.

## Part Selectors

Machine-backed components additionally emit
`data-scope` / `data-part` / `data-state` attributes (architecture 006).
These are a **read-only styling hook** for app CSS where a recipe variable
does not exist yet — stable in name, but prefer recipe variables when
available; new recipe variables are added on demonstrated need rather than
exposing every internal property.

## Growing The Surface

When an app needs an appearance property that has no recipe variable, the
correct move is to add the variable to the component (with the standard
fallback chain) and update its contract — not to target internal selectors.
Additions are non-breaking.
