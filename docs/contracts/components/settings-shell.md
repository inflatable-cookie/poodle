# SettingsShell

Status: contract
Updated: 2026-08-12

## 1. Purpose

- Component name: `SettingsShell`
- Layer: `composites`
- Summary: the settings frame — a modal dialog with search in its header bar, a grouped
  navigation rail, and a page region that search replaces — hosting host-owned
  settings pages as snippets
- Composes: `Dialog`, `SidebarNav`, `Surface`, `ScrollShell`,
  `TextInput`, `Callout`, `EmptyState`
- In scope: dialog posture, search-over-page replacement, grouped navigation
  with its own scroll and surface, page header + scrolling page body, refused
  close notice, empty nav and empty result states
- Out of scope: settings content, navigation semantics, search execution,
  close-allowance policy, native targets (web-only — see §13)

`SettingsShell` is the frame, not the content. The host owns every settings
page (a snippet), the group and result data, the search execution, and the
decision whether a close attempt succeeds. The shell never imports Longhorn,
never fetches, and never learns what a storage profile or a keymap is.

The shell is a **composition**, like `DetailShell`, `PickerShell`, `FormShell`
and `ScrollShell`: CSS and web components only, no `packages/core` machine and
no Rust spec. State is the host's, through props; `searchQuery` and `open` are
bindable.

## 2. Structural Types (no Longhorn)

Shapes are declared locally, structurally, exactly as `history-center.md`
§1 requires: Poodle never imports a Longhorn type and never learns what a
storage profile or a keymap is. The shapes below are what the current host
needs; a host maps its own domain onto them — the dependency runs host →
Poodle, never the reverse.

```ts
interface SettingsNavGroup {
  id: string;
  label: string;
  items: { value: string; label: string }[];
}

interface SettingsSearchResult {
  pageId: string;
  pageLabel: string;
  anchorId?: string;
  anchorLabel?: string;
}
```

Group labels are **section labels, not compositions**. A host that wants its
module named writes that into the section label; the shell never joins module
and section labels (that produced the `STORAGE · STORAGE & BACKUPS` wrap).

## 3. Anatomy

```text
Dialog (width="xl", title, ariaLabel=title, showCloseButton,
        closeLabel "Close settings")
├── header snippet/node — one bar, left of the dialog's own close (R1.5):
│   └── [.poodle-settings-shell__dialog-header]  <div>  flex
│       ├── [.poodle-settings-shell__dialog-title]  <strong> title
│       └── [.poodle-settings-shell__search]  <div>   fills the span
│           └── TextInput (type="search", placeholder/ariaLabel
│                 "Search settings", showClearButton)
└── [.poodle-settings-shell]  <div>   grid: 14rem / minmax(0, 1fr)
    ├── [.poodle-settings-shell__nav]  <aside>
    │   └── Surface (tone="panel", border="subtle", padding="none")
    │       └── ScrollShell (direction="vertical")
    │           ├── (groups empty) EmptyState (variant="neutral", size="compact",
    │           │     "No settings pages")
    │           └── (groups present) SidebarNav (ariaLabel "Settings pages",
    │                 value=activePageId)
    └── [.poodle-settings-shell__page]  <div>   flex column
        ├── [.poodle-settings-shell__notice]  <div>   (when closeRefusedReason)
        │   └── Callout (tone="warning", announceMode="polite",
        │         message=closeRefusedReason)
        └── content region — search **replaces** the page (R1.6):
            ├── (searchResults !== null)
            │   └── [.poodle-settings-shell__results]  <div data-empty>
            │       ├── (no results) EmptyState (variant="search", size="compact",
            │       │     "No results")
            │       └── (results) ScrollShell (direction="vertical", padding="md")
            │           └── [.poodle-settings-shell__result-list]  <ul aria-label>
            │               └── [.poodle-settings-shell__result]  <li>
            │                     └── <button type="button">  (activates the page)
            │                         ├── [.poodle-settings-shell__result-label]
            │                         │     <span> pageLabel
            │                         └── [.poodle-settings-shell__result-anchor]
            │                               <span> anchorLabel  (optional)
            └── (searchResults === null)
                └── [.poodle-settings-shell__page-stack]
                      <section aria-label=pageTitle>  flex column
                    └── ScrollShell (direction="vertical", padding="md")
                        └── page snippet
```

| Part | Element | Notes |
|------|---------|-------|
| Dialog | `Dialog` | Owns modal semantics, focus trap, body scroll lock, and the **only** close affordance (R1.4). |
| Nav | `<aside>` | Region: Surface + border + own scroll — never text floating on the dialog background (R1.1). |
| Nav ScrollShell | `ScrollShell` | Independent vertical scroll owner for navigation. |
| Dialog header | `<div>` | Title, search and the dialog's own close on one bar; the shell renders only the span left of close (R1.5). |
| Page | `<div>` | Right column: notice, then either results or the page stack. Flex column — the notice is optional, so a fixed row template would strand the content. |
| Search | `TextInput` | In the dialog header bar, filling the span between title and close (R1.5, reversed — see §9). |
| Notice | `Callout` | Refused-close notice, warning tone, polite announcement — not an error treatment (R1.7). |
| Results | `<ul>` | Flat result list replacing the page while a query is active (R1.6); no dropdown, no overlay. |
| Page stack | `<section>` | The scrolling snippet body, named by `pageTitle`. The shell draws no heading or description of its own (R1.3, reversed — see §9). |

## 4. Props And Inputs

### Data Shapes

Shapes per §2. Search results carry the anchor only when the result has one.

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `groups` | `SettingsNavGroup[]` | `[]` | no | Navigation groups; empty renders the nav empty state |
| `activePageId` | `string \| null` | `null` | no | Currently active page in the navigation rail |
| `pageTitle` | `string \| null` | `null` | no | Accessible name of the page region. **Not drawn** — the nav rail already names the page (R1.3, reversed) |
| `searchQuery` | `string` | `""` | no | Search field value; two-way bindable in Svelte, `onSearchQueryChange` in React |
| `searchResults` | `SettingsSearchResult[] \| null` | `null` | no | Search outcome; `null` means **not searching** — the page region renders the page. Non-null (including `[]`) replaces the page region with the flat result list / no-results state. The host decides when a query is active; the shell never derives it from the query text |
| `open` | `boolean \| null` | `null` | no | Controlled open state; `null` = uncontrolled, seeded by `defaultOpen`; Svelte supports binding |
| `defaultOpen` | `boolean` | `false` | no | Uncontrolled initial open state |
| `title` | `string \| null` | `"Settings"` | no | Dialog title; also the dialog's accessible name (via `aria-labelledby`) |
| `closeLabel` | `string` | `"Close settings"` | no | Accessible label for the dialog close button |
| `page` | `Snippet` | — | no | The current page body. **Always a snippet** — content never arrives through a data prop |
| `closeRefusedReason` | `string \| null` | `null` | no | The host's reason for refusing a close, shown as a warning `Callout`. Set means the shell stays open on a close attempt; `null` means a close attempt proceeds. Never invented by the shell — see §6 |

### Callbacks

| Callback | Type | Notes |
|----------|------|-------|
| `onNavigate` | `(pageId: string, anchorId?: string \| null) => void` | Fired when a nav item activates (page id) or a search result activates (page id, plus anchor id when the result carries one). Commands out: the host owns navigation |
| `onRequestClose` | `() => void` | Fired on every close attempt (close button, Escape, backdrop). Commands out: the host decides. The host refuses by supplying `closeRefusedReason` — see §6 |
| `onOpenChange` | `(open: boolean) => void` | Open-state request, the `open` binding's React counterpart |
| `onSearchQueryChange` | `(value: string) => void` | Search value change, the `searchQuery` binding's React counterpart |

### Controlled And Uncontrolled

`open` follows the `HistoryCenter` pattern: `open = null` means uncontrolled
with `defaultOpen` seeding the first render; a non-null `open` is controlled
and updates flow back through `onOpenChange`. `searchQuery` follows the
`TextInput` pattern: bindable in Svelte, controlled via
`onSearchQueryChange` in React. Everything else is caller-owned data — the
shell keeps no second store of groups, results, or pages.

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | `open=false` / uncontrolled close | Entire dialog unmounted |
| open | open with groups | Surface'd nav with own scroll + page column with search, fixed header, scrolling snippet body |
| searching | `searchResults !== null` | Page region shows the flat result list; the page snippet does not render (R1.6) |
| no results | `searchResults === []` | Search empty state in the page region |
| no groups | `groups === []` | Nav empty state in the navigation rail |
| refused | `closeRefusedReason` set | Warning `Callout` under the search field, polite-announced; a close attempt keeps the dialog open |
| empty page body | no `page` snippet | The page scroll region renders empty (the host supplies content) |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only; interaction is delegated to the composed
primitives (Dialog's modal machine, SidebarNav activation, TextInput editing)
and native elements. No component-owned behavioral state beyond the two
bindable props. Classified in the g11.004 long-tail sweep.

## 6. Refused Close

`onRequestClose` is **commands out**: every close attempt fires it, and the
shell closes unless the host refuses. The host refuses by supplying
`closeRefusedReason` — a string rendered as a warning `Callout`
(`announceMode="polite"`). A refused close is **not an error**: it is the
host saying there are unsaved changes here (R1.7).

While `closeRefusedReason` is set, a close attempt keeps the dialog open — the
shell never closes itself against a refusal. The host clears the reason (for
example after the user applies, discards, or navigates) and the next close
attempt proceeds normally.

The shell never invents the reason copy: the string is the host's, verbatim.
The shell never blocks a close on its own — a host that wants a close to
succeed just leaves `closeRefusedReason` null.

## 7. Callbacks

See §4. `onNavigate` never fires with an anchor id for a nav item — anchors
only exist on search results.

## 8. Accessibility

### Semantics

- Dialog: `role="dialog"`, `aria-modal`, `aria-labelledby` the dialog title;
  focus trap and body scroll lock from `Dialog`
- Close button: `IconButton` labelled `closeLabel` ("Close settings") — the
  **only** close affordance in the shell; nothing in the page header closes
  the shell (R1.4)
- Nav: `<aside>` containing `SidebarNav` (`<nav aria-label="Settings pages">`);
  group `<section>`s carry `aria-label` from the group label; group titles
  carry `title={label}` for the sighted pointer user on a truncated heading
  (R4a — `SidebarNav` sets it unconditionally)
- Search: `TextInput` with `aria-label="Search settings"`; placeholder never
  counts as the accessible name
- Results: `<ul aria-label="Settings search results">` of buttons; each
  result's accessible name is its page label plus the anchor label when
  present
- Notice: `Callout` with `announceMode="polite"` → `role="status"`,
  `aria-live="polite"`; announced when it mounts (the moment the host refuses)
- Empty states: `EmptyState` labelled by its title ("No settings pages" /
  "No results")

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` / `Shift+Tab` | Dialog focus trap cycles within the surface |
| `Escape` | Close attempt: `onRequestClose`; refused while `closeRefusedReason` is set |
| `Enter` / `Space` | Activates a nav item or search result (native button) |
| Arrow keys | Scroll the focused scroll region (native) |

### Focus And Announcement

- Focus entry on open: `Dialog` `initialFocus="auto"` → the search field
  (the first focusable in the body) — search is the natural entry point
- Focus restoration: `Dialog` restores focus to the trigger on close
- Refusal announcement: the notice `Callout` is polite-announced on mount

## 9. Layout

### Sizing

- Dialog width: `xl` — `min(64rem, 100%)`. `lg` left too little room once
  search moved onto the header bar
- Shell height: `var(--poodle-recipe-settings-shell-height, min(68vh, 36rem))`
  — bounded so the inner regions own their scroll; the dialog surface's own
  max-height is a safety net, not the scroll owner
- Grid: `grid-template-columns: 14rem minmax(0, 1fr)`. The page column and
  the page stack are **flex columns**, not grids: the notice and (formerly)
  the page header are optional, and a fixed row template strands the scroller
  in an `auto` row whenever the optional child is absent

### Scroll Ownership (R1.3)

- The nav rail scrolls **independently** (its own `ScrollShell`)
- The page body scrolls **independently** (its own `ScrollShell`)
- The search field sits in the dialog header bar, outside both scroll regions
- There is no page header to keep out of the scroll region (R1.3, reversed)
- The results list scrolls in the page region's scroll shell
- Every scroll boundary has `min-height: 0` / `min-width: 0` so the grid can
  shrink instead of pushing past the dialog

### Composition

- Parent expectations: an opened `SettingsShell` is a modal dialog; the host
  decides when to open it
- Child expectations: settings pages as snippets (sections, forms, whatever
  the host needs); never shell chrome
- Resizing rules: the nav rail is fixed at 14rem; the page column takes the
  remainder; the whole shell height is bounded by the recipe hook

### Reversals

Two `039` rulings were reversed in use, both on the author's call after seeing
the built component:

- **R1.5 — search placement.** `039` confined search to the page column. It now
  sits in the dialog header bar between the title and the dialog's own close,
  so the three read as one row. `Dialog` drops `aria-labelledby` when given a
  custom header (`Dialog.svelte:116`), so the shell passes `ariaLabel={title}`
  to keep the dialog named.
- **R1.3 — page header.** `039` put a fixed `PageHeader` above the scrolling
  body. The shell no longer draws a page heading or description at all: the nav
  rail already names the current page, and the page snippet owns its own intro.
  `pageDescription` was removed; `pageTitle` survives only as the page region's
  accessible name.

## 10. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Nav surface | `background-panel`, `border-subtle`, `radius-surface` | The nav's edge — a region, not floating text (R1.1) |
| Group titles (truncation) | `typography-label-*` (inherited), `space-*` | One line, `overflow: hidden`, `text-overflow: ellipsis`; never wrapped (R1.2). Truncation CSS lives in `settings-shell.css` scoped to the shell; `SidebarNav` itself only gains the `title` attribute (R4a) |
| Result rows | `background-elevated` hover mix, `radius-control`, `accent-focusRing` | Flat list rows with hover and focus ring |
| Notice | `status-warning` (via `Callout`) | Refused close, not an error |
| Empty states | `EmptyState` variants | Designed absence states (R1.7) |
| Scroll shells | `ScrollShell` | Independent scroll ownership (R1.3) |

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-empty` | `.poodle-settings-shell__results` | `"true"` / `"false"` — whether the search returned no results |

## 11. Specimen Definitions

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Normal | two groups with long labels, active page, page snippet | Dialog with surface'd nav rail (own scroll), page column with search, fixed header, scrolling page body |
| Searching | `searchResults` with a mix of plain and anchored results | Flat result list replacing the page; no page header, no page snippet |
| No groups | `groups=[]` | Nav empty state inside the surface'd rail |
| No results | `searchResults=[]` | Search empty state in the page region |
| Refused close | `closeRefusedReason` set | Warning callout under the search field, dialog still open |

## 12. Required Tests

Both runtimes:

- Group labels never wrap: a label long enough to overflow truncates
  (`white-space: nowrap`, `overflow: hidden`, `text-overflow: ellipsis`) and
  carries a `title` tooltip
- Exactly one close affordance in the whole shell
- A non-null `searchResults` replaces the page region; the page snippet does
  not render; clearing the query (null again) restores it
- Navigation and page are separately scrollable regions; the search field and
  page header are outside both
- `onNavigate` fires with the page id, and with the anchor id when a result
  carries one
- `onRequestClose` fires on a close attempt and the shell stays open
- `closeRefusedReason` renders as a `Callout`, not an error, and is announced
  (`role="status"`, `aria-live="polite"`)
- Empty groups and empty results each render their designed state

## 13. GPUI Notes

None — **web only**. `HistoryCenter` is the precedent for a web-only
component; native parity is deferred to `g13.014`, and the component is
recorded in the native registration gap inventory so the gap stays counted.

## 14. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] search replace semantics match (non-null `searchResults` replaces the page)
- [ ] open controlled/uncontrolled semantics match
- [ ] close refusal behavior matches (reason set → close attempts keep open)
- [ ] anatomy classes match (nav, page, search, results, page-stack)

### Tier 2: Visual Parity

- [ ] nav surface treatment matches (panel tone, subtle border, own scroll)
- [ ] group-title truncation matches (one line, ellipsis, tooltip)
- [ ] result row hover and focus ring match
- [ ] empty-state variants match

### Tier 3: Implementation Freedom

- [ ] internal id generation for the search field may differ
- [ ] uncontrolled query state implementation may differ
