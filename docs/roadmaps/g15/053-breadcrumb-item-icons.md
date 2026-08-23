# g15.053 — Breadcrumb Item Icons

Status: **ready, serial behind `g15.043`** — do not dispatch until PR #70 is
accepted and merged
Requested by: active downstream adopter, 2026-08-23
Depends on: `g15.043` (`RenderContext` migration touches the same Rust spec and
renderer surfaces)
Unblocks: `g15.050`, then `g15.013`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/breadcrumbs.md`,
`../../architecture/001-poodle-system-shape.md`,
`release-gap-register.md`

## Problem

`Breadcrumbs` can render text only. A common hierarchy starts with a compact
home icon, and richer trails benefit from an icon beside any node. Consumers
cannot express either shape without replacing the component or faking content
outside its item model.

The root case must remain named for assistive technology even when no text is
visible. The upgrade therefore separates semantic label from visible label
instead of allowing an unnamed icon link.

## Fixed Public Shape

Add icon presentation to `BreadcrumbItem`, not a root-only Breadcrumbs prop and
not an item slot:

```ts
type BreadcrumbItemBase = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
};

type BreadcrumbItem = BreadcrumbItemBase & (
  | { icon?: IconProp; iconOnly?: false }
  | { icon: IconProp; iconOnly: true }
);
```

- `icon` renders before the visible label on any authored item.
- `iconOnly` hides the label visually but keeps `label` as the item's
  accessible name. It requires `icon` in the web type.
- Existing text-only items remain valid and unchanged.
- The synthetic ellipsis item never receives an icon or icon-only treatment.
- An invalid native item with `icon_only=true` and no icon renders its label;
  it must never become a blank crumb. Normal Rust construction prevents this
  shape with one atomic `with_icon_only(icon)` builder.

The Rust mirror adds `icon: Option<String>` and `icon_only: bool`, plus
`with_icon(icon)` and `with_icon_only(icon)` builders. Named icons are the
portable contract. Web `IconProp` retains the existing generated-icon-node
capability of the web Icon substrate.

## Rendering And Semantics

- Render the item icon inside the same anchor, button, or current-page span as
  its label. The entire visual remains one navigation target.
- Item icons are decorative. The containing interactive/current element owns
  the accessible name and current-page semantics.
- An icon-only anchor, button, or current-page span exposes `label` as its
  accessible name while omitting visible label text.
- Icon-plus-label content uses one inline row, centre alignment, and
  `space.inline.xs` between glyph and text. Do not reuse the larger
  crumb/separator gap inside the item.
- Item icons use the Breadcrumbs component's resolved control size without a
  second semantic-role shift. Separators keep their existing chevron size,
  opacity, and accessibility behavior.
- Link versus callback navigation, current-item suppression, truncation, wrap,
  focus, size, and density behavior remain unchanged.

Add `home` to Poodle's scoped default Lucide manifest because the component's
canonical specimen now requires it. Regenerate the TypeScript and native SVG
artifacts with the existing icon build; do not hand-edit generated files or
widen the default catalogue beyond this one component-owned icon.

## Exact Scope

### Contract and data model

- Update the Breadcrumbs contract anatomy, item type, state, accessibility,
  layout/token notes, parity checklist, and specimen definition.
- Update paired web `BreadcrumbItem` types with the discriminated icon-only
  shape.
- Extend the renderer-neutral Rust item mirror and keep truncation helpers from
  copying icon presentation onto the ellipsis sentinel.

### Active-cohort implementation

- Svelte remains the reference: render text-only, icon-plus-label, and
  icon-only items through the existing `Icon` and presentation substrates.
- Mirror the exact structure and shared CSS in React.
- Build matching item content in `poodle-render` through the post-`g15.043`
  `RenderContext`; do not reintroduce bare `ThemeProvider` access.
- GPUI consumes the shared Node output. Add backend work only if a reusable
  existing Node accessibility or inline-layout channel is not being projected
  correctly; stop before inventing a Breadcrumbs-specific backend path.
- Jetstream remains program-deferred. Shared spec/render changes may receive
  compile-only caller adaptation, but do not add a paired checkout or claim
  Jetstream parity.

### Teaching and evidence

Add one concise `Icons` example to the Svelte, React, and GPUI Breadcrumbs
specimens:

- root: `Home`, `icon="home"`, `iconOnly=true`;
- intermediate: `Projects`, `icon="folder"`, visible label;
- current: `Poodle`, `icon="package"`, visible label.

Keep the existing Basic, Deep path, Collapsed, Sizes, and Densities teaching
content. Do not turn Examples into an icon matrix.

## Acceptance

- [ ] Every authored breadcrumb item can render without an icon, with an icon
      and visible label, or as an icon-only item.
- [ ] The canonical root home crumb has no visible text but is exposed as
      `Home` to assistive technology and remains one link/button target.
- [ ] Icon-plus-label anchors, buttons, and current spans keep the icon and
      label inside the same semantic element.
- [ ] Icons are decorative; separators and the synthetic ellipsis remain
      hidden from assistive technology as before.
- [ ] `iconOnly` requires an icon in paired web types; Rust's
      `with_icon_only(icon)` constructs the valid state atomically; invalid
      direct native input cannot render a blank crumb.
- [ ] Icon size follows Breadcrumbs' resolved size exactly, icon-label spacing
      uses `space.inline.xs`, and existing crumb/separator spacing is unchanged.
- [ ] Truncation preserves icons on retained authored items and synthesizes a
      plain ellipsis without copied icon state.
- [ ] Navigation, current-item behavior, focus, wrapping, size, and density
      retain focused regressions.
- [ ] Svelte, React, shared Rust output, and the GPUI specimen implement the
      same contract; no runtime-specific Breadcrumbs API or second renderer is
      introduced.
- [ ] The curated Icons example is human-readable in the paired web previews
      and GPUI specimen probe. Operator live review is required before merge.

## Writable Scope

- `docs/contracts/components/breadcrumbs.md`
- `packages/{svelte,react}/components/src/{Breadcrumbs,types}.*`
- `packages/core/src/styles/breadcrumbs.css`
- focused paired Breadcrumbs tests
- `packages/contracts/components/src/breadcrumbs.rs`
- `packages/render/src/breadcrumbs.rs`
- focused shared-Rust and mounted/headless GPUI evidence where needed
- Svelte, React, and GPUI Breadcrumbs specimen files
- `packages/core/src/icons/default-icons.json` and artifacts produced only by
  `effigy icons:build`
- one August `g15.053` execution log and `PAPERCUTS.md` for new execution
  friction

Do not edit unrelated components, global icon APIs, tokens, themes, node
vocabulary, backend painting, catalogue navigation, visual comparator policy,
package versions, workflows, release notes, Longhorn, tags, publication, or
the open `g15.043` branch.

## Validation

- focused paired Breadcrumbs component tests
- focused `poodle-specs` and `poodle-render` Breadcrumbs tests
- `effigy icons:build` followed by `effigy audit:icons`
- `effigy test:components`
- `effigy check:svelte`
- `effigy react:build`
- `effigy probe:gpui-specimens`
- `effigy check:gpui`
- `effigy ci:rust`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Run the broad checks once after the coherent implementation batch. Never run a
`*-windowed`, native-visual, Jetstream preview/QA, release mutation, tag,
publication, or workflow command.

## Stop Conditions

- PR #70 / `g15.043` is not merged into the worker's `origin/main`.
- The item icon requires a new cross-runtime icon authority rather than the
  existing Icon / named-icon substrates.
- Correct icon-only naming needs a general accessibility-vocabulary change
  rather than the existing node label channel.
- The change requires arbitrary per-item slots, overflow menus, tooltips,
  editable breadcrumbs, or a root-specific API.
- A renderer cannot keep icon and label inside one navigation target without a
  component-specific backend implementation.

## Continuation

After `g15.043` merges, dispatch this card as one isolated worker lane. The
orchestrator reviews the three active-runtime specimens with the operator,
then merges and closes the new adopter requirement. `g15.050` remains blocked
until both `g15.043` and `g15.053` are accepted.
