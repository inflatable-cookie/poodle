---
name: poodle
description: Build Svelte application UI with Poodle, the inflatable-cookie design system. Use when a Svelte app depends on @inflatable-cookie/poodle-core and @inflatable-cookie/poodle-svelte and you need to choose components for a view, follow their contracts, inherit theme/density/control size, and finish keyboard, focus, empty, loading, error and retry states.
---

# Poodle implementer (Svelte)

Poodle is a design system: tokens, primitives and reusable composites. Use it
for the visible UI language and keep product vocabulary, data loading, routing,
persistence and domain behavior in the application.

This skill routes a view requirement to a component contract and a composition
recipe. The normative source is the contract, read at the version the consumer
pins. `references/reference.md` is a generated, searchable inventory of every
contract, public export and preview specimen in this checkout.

## Install

From the consumer repository:

```sh
# project-local, copied into the detected agent directory
npx skills add inflatable-cookie/poodle --skill poodle -y --copy

# or user-level, shared by every checkout
npx skills add inflatable-cookie/poodle --skill poodle -g -y
```

The skill adds context; it does not install Poodle. Keep the app's exact
`@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte` pins and
install those normally.

## 1. Read the consumer's pinned version first

Do not assume the skill's checkout and the app's installed package match. From
the consumer repository:

```sh
VERSION=$(node -p "JSON.parse(require('node:fs').readFileSync('node_modules/@inflatable-cookie/poodle-svelte/package.json', 'utf8')).version")
echo "$VERSION"
```

Then build every contract link from that release tag:

```sh
CONTRACT="https://github.com/inflatable-cookie/poodle/blob/v${VERSION}/docs/contracts/components/<slug>.md"
```

Read the contract at that tag before composing. If the pinned version has no
tag, the app is on an unpublished build: use this skill's reference for shapes
and say which contract could not be resolved at a release tag.

The installed package decides what exists. If the reference lists a component
that the installed package does not export, do not import it; read the contract
at the pinned tag or choose the nearest exported composite.

## 2. Route the view

| View requirement | Recipe | Contracts to read | Svelte specimen |
| --- | --- | --- | --- |
| Menu-bar panel, app frame, header, navigation | `014-admin-app-shell-recipes.md`, `011-page-shell-and-admin-recipes.md` | `app-header`, `menubar`, `sidebar-nav`, `navigation-menu`, `scroll-shell`, `separator` | all previewed |
| Browse page, list, filters, bulk selection | `003-list-and-filter-recipes.md`, `013-admin-feature-delivery-recipes.md` | `list-container`, `filter-toolbar`, `data-table`, `pagination`, `pagination-summary`, `order-by`, `bulk-action-bar` | all previewed |
| List plus detail (master-detail) | `003-list-and-filter-recipes.md`, `004-dialog-and-detail-recipes.md` | `split-view`, `list-container`, `listbox`, `detail-shell`, `detail-section`, `detail-item` | all previewed |
| Record detail, read-only summary, metadata | `004-dialog-and-detail-recipes.md`, `013-admin-feature-delivery-recipes.md` | `detail-shell`, `detail-section`, `detail-section-group`, `detail-item`, `meta-bar`, `meta-item`, `page-header` | all previewed |
| Settings and preferences | `001-form-layout-and-field-recipes.md`, `006-form-validity-recipes.md` | `settings-shell`, `form-layout`, `field-set`, `field`, `card-toggle-group`, `segmented-control`, `switch`, `select`, `text-input` | all previewed |
| Create/edit form, validation, save intent | `001-form-layout-and-field-recipes.md`, `005-async-validation-recipes.md`, `006-form-validity-recipes.md` | `form-layout`, `field`, `text-input`, `checkbox`, `radio-group`, `select`, `form-actions`, `form-dialog`, `validation-summary` | all previewed |
| Loading, empty, progress and status | `003-list-and-filter-recipes.md` | `empty-state`, `page-loading`, `spinner`, `skeleton`, `progress`, `status-indicator`, `state-tile`, `metric-tile` | all previewed |
| Confirmation and destructive actions | `002-action-pattern-recipes.md`, `004-dialog-and-detail-recipes.md` | `confirm-action`, `alert-dialog`, `dialog`, `button` | all previewed |
| Failure, remediation and retry | `013-admin-feature-delivery-recipes.md` | `remediation-banner`, `inline-remediation`, `callout`, `validation-summary`, `error-boundary`, `empty-state`, `toast-host` | previewed except `inline-remediation` (compose it with `Callout`) |
| Media browse, picker and upload | `008-file-upload-recipes.md`, `009-media-picker-workflow-recipes.md`, `012-media-library-and-upload-recipes.md` | `file-upload`, `media-picker`, `media-browse-panel`, `media-preview`, `media-thumbnail`, `picker-shell` | all previewed |

`FormShell` is a shared Rust/native contract with no Svelte component. In
Svelte, compose `FormLayout`, `FieldSet`, `Field`, `ValidationSummary` and
`FormActions` directly; do not expect a `FormShell` import.

Two routing cautions the contracts make explicit:

- `SettingsShell` is a modal with search and a navigation rail. For an
  in-window or popover settings area, use `FormLayout` with `FieldSet` and
  `Field`; reach for `SettingsShell` only when the settings surface really is a
  modal with grouped navigation.
- `ListContainer`, `DetailShell` and similar shells expose `state` plus
  `error`/`loading`/`empty`/`stateContent` snippets. Put retry in the shell's
  error slot, not in a second page-level banner. `Listbox` rows are host-rendered
  content with no interactive descendants; when a row needs its own actions or
  sortable columns, use `DataTable` or a `ListCard` composition instead.

### Build from behaviour

When Poodle lacks the control you need, use the installed core package's
`llms.txt` alongside its `package.json`. It points to the supported root
exports, the async tag combobox recipe and the appearance hooks at the
installed core version. For older packages without the index, look for
`docs/contracts/core-behaviour.md` and
`docs/guides/015-async-tag-combobox.md` at the core package's release tag.
Use the exports documented for that installation. If the tag or reference
is missing, confirm each import against the installed package and flag the
missing versioned reference.

Build the control in the application. Use supported core behaviours for
interaction, public Svelte helpers for DOM wiring, and Poodle tokens and
appearance hooks for styling. Keep the application's data loading and state
with the app. Record missing reusable behaviour and where you supplied it
when reporting a gap.

## 3. Look up components, helpers and rules

Search `references/reference.md` for a component name, a helper export, a
contract slug or a shared rule. It distinguishes:

- **Components** — a contract plus its Svelte export and preview specimen.
  `Svelte none` marks a native/shared contract or a shape the caller composes.
- **Helpers** — public function exports grouped by their preview coverage.
- **Package subpaths** — every import specifier the published manifests expose.
- **Shared rules** — cross-cutting contracts such as anchored overlays and
  shared control types.

Prefer a contract's own vocabulary (`variant`, `tone`, `sizeRole`, `density`,
callbacks) over inventing props. Every component accepts semantic size and
density inheritance; set them at the nearest `UiPresentationProvider` or shell
and override only where the view needs it.

## 4. Compose with inherited presentation

Theme is a document attribute; density and control size are a Svelte context
used by component size resolution. Set both once at the application root:

```svelte
<script>
  import "@inflatable-cookie/poodle-core/tokens/styles.css";
  import "@inflatable-cookie/poodle-core/tokens/themes.css";
  import "@inflatable-cookie/poodle-core/tokens/density-default.css";
  import "@inflatable-cookie/poodle-core/tokens/control-size-sm.css";
  import { applyThemeAttributes } from "@inflatable-cookie/poodle-core/tokens/runtime";
  import { UiPresentationProvider } from "@inflatable-cookie/poodle-svelte";
  import { onMount } from "svelte";

  let shell;

  onMount(() => {
    applyThemeAttributes(shell, {
      theme: "eclipse",
      density: "default",
      controlSize: "sm",
    });
  });
</script>

<div bind:this={shell}>
  <UiPresentationProvider density="default" sizeScale="sm">
    <!-- app -->
  </UiPresentationProvider>
</div>
```

`applyThemeAttributes` writes the `data-theme`, `data-density` and
`data-control-size` attributes that select the token stylesheets. Component
`size`/`density` resolution reads the `UiPresentationProvider` context, so pass
`sizeScale` and `density` there as well; otherwise controls keep the package
defaults (`md`, `default`). Use explicit `size`/`density` props only for a
deliberate local override.

The generated theme set is larger than the three common defaults; read the
"Themes" table in the Svelte developer guide or `@inflatable-cookie/poodle-core/tokens/themes`
for the current list instead of copying a stale inline list.

Do not hardcode hex colors, spacing, radii or font sizes in application CSS.
Use the semantic custom properties (`--poodle-color-text-primary`,
`--poodle-color-background-surface`, `--poodle-color-border-default`, status
tokens, spacing and radius tokens). Import `lucide-static` data for
application-owned icons; do not add an icon library the contract does not name.

## 5. Finish check

Before calling the view done:

- **Component choice** — use a documented component when it fits. For a
  missing control, follow the build-from-behaviour route above.
- **Contract** — the props, states, events and slots used exist in the contract
  at the pinned version. No invented variants or aliases.
- **Semantic tokens** — no raw visual values in app CSS; theme switching does
  not need app-specific overrides.
- **Inheritance** — theme, density and control size are set at one root and
  reach nested controls; explicit `size`/`density` overrides are intentional.
- **Keyboard and focus** — every interactive control is reachable and operable
  by keyboard, focus is visible, overlays restore focus, and Escape/outside
  dismissal works where the contract requires it.
- **States** — empty, loading, error and retry are visible and distinct; a
  failed action has a recovery path (`RemediationBanner`, `Callout`,
  `ValidationSummary`, `ErrorBoundary` retry, or a first-error jump).
- **Copy and accessibility** — labels, `ariaLabel` and descriptions are real
  application copy, not placeholders.

## 6. Report gaps, keep domain behavior

When a composition exposes a reusable gap, report it to Poodle's planner or
issue tracker. Include:

- the consumer package version,
- the composition you attempted (component names and contract links),
- the missing observable behavior (state, event, slot, token or accessibility
  behavior),
- why the behavior is general rather than product-specific.

Do not add the missing behavior to Poodle from a consumer task, and do not
replace a contract with a local fork. Product vocabulary, data loading,
routing, persistence and service orchestration stay in the app.

For native GPUI apps, use the adapter-lifecycle handoff in
Longhorn's `docs/knowledge/contracts/` tree
(`013-svelte-and-poodle-adapter-lifecycle.md`); Longhorn's `agent-control`
skill drives a running app and is not a Poodle composition guide. This skill
covers the Svelte web package.
