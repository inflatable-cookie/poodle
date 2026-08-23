# g15.053 Breadcrumb Item Icons

Date: 2026-08-23
Card: `../../roadmaps/g15/053-breadcrumb-item-icons.md`
Handoff: `../../handoffs/20260823-114443-g15-053-breadcrumb-item-icons.md`
Contract: `../../contracts/components/breadcrumbs.md`
Worker branch: `t3code/fix-breadcrumb-item-icons`
Worker worktree: `/Users/tom/.t3/worktrees/poodle/t3code-83af4086`
(launcher-supplied, registered, clean at start, non-`main`; the generated
branch name differs from the handoff's suggested name and was reused per the
handoff's own instruction)
Planning base: `030d04c263f7a0b61d01337c67c282da01a298e3`, an ancestor of
`HEAD` = `origin/main` = `d1a26fd3ff28001ccef0f2f43dd909253ca71a42`, the merge
of PR #70

## Dependency Check

PR #70 is merged into `origin/main` and `poodle_render::breadcrumbs` takes
`&RenderContext`, so the lane is unblocked. The `g15.043` card and the g15
README still read "in review on PR #70" — planning closeout the orchestrator
owns, not a worker edit. Recorded here rather than fixed.

## API Shape

`label` stays required and stays the item's semantic identity. Icon
presentation is per item; there is no root-only prop and no item slot.

Web (`packages/{svelte,react}/components/src/types.ts`), one discriminated
union so `iconOnly` cannot be authored without an icon:

```ts
type BreadcrumbItem = BreadcrumbItemBase & (
  | { icon?: IconProp; iconOnly?: false }
  | { icon: IconProp; iconOnly: true }
);
```

Rust (`packages/contracts/components/src/breadcrumbs.rs`) mirrors named icons:
`icon: Option<String>`, `icon_only: bool`, with `with_icon(icon)` and
`with_icon_only(icon)`. `with_icon_only` sets both fields in one call, so
normal construction cannot reach the invalid state. `shows_label()` states the
fallback: an item with `icon_only` and no icon renders its label. The
`visible_items()` ellipsis is constructed with `icon: None, icon_only: false`,
so truncation never copies icon presentation onto the sentinel.

## Accessibility Behavior

The icon and the label live inside the same anchor, button, or current-page
span — one navigation target, one accessible name.

Web: an icon-bearing item wraps both in `.poodle-breadcrumbs__content`. An
icon-only item does not drop the label element; it renders it with
`.poodle-breadcrumbs__label--hidden`, which removes it from the visual box and
keeps it in the accessibility tree. That was chosen over `aria-label` because
the current crumb is a bare `<span aria-current="page">` with no role, where
`aria-label` is not reliably exposed; the hidden-text form works identically
for the anchor, the button, and the span. Item icons keep the `Icon`
component's decorative default (`role="presentation"`, `aria-hidden="true"`).

Native: an icon-bearing crumb becomes one row node that carries the crumb's
`a11y.label` and its activation handler, with a decorative icon child and an
optional text child. Separator and ellipsis semantics are untouched.

## Layout And Tokens

Item icons take the Breadcrumbs resolved control size directly. On the web the
`Icon` gets an explicit `size={resolvedSize}`, which bypasses `sizeRole`; in
Rust the `IconSpec` uses the default `Control` role, which is the identity
mapping — no second semantic shift either side. Icon-to-label spacing is
`space.inline.xs`, tighter than the crumb/separator gap and independent of size
and density. Existing separator size/opacity/gap, list and item gaps, font
ladders, truncation, wrap, focus, and navigation are unchanged; a text-only
crumb still emits exactly the markup and the bare text node it did before.

## Generated Icon Change

Lucide 1.31.0 has no `home`; the glyph is `house`. The manifest already models
exactly this with its `aliases` map (`alert-circle → circle-alert`,
`edit → pencil`, ...), so `packages/core/src/icons/default-icons.json` gained
canonical `house` plus alias `home → house`. `effigy icons:build` generated
`packages/core/src/icons/icons/{home,house}.ts` and
`packages/render/assets/icons/{home,house}.svg` and refreshed `generated.ts`
and `aliases.generated.ts`. That is one component-owned icon, reachable under
the name the card and the contract specify; nothing was hand-edited. `folder`
and `package` already existed.

## Specimens

Svelte, React, and GPUI each gained one `Icons` group: icon-only `Home`,
labelled `Folder`/Projects, labelled current `Package`/Poodle. Basic, Deep
path, Collapsed, Sizes, and Densities are unchanged.

## Validation

Run in this worktree, headless only:

- `packages/svelte/components/test/Breadcrumbs.test.ts` — 10 passed
- `packages/react/components/test/Breadcrumbs.test.tsx` — 10 passed
- `cargo test -p poodle-specs breadcrumbs` — 4 passed
- `cargo test -p poodle-render breadcrumbs` — 8 passed
- `effigy icons:build` then `effigy audit:icons` — 108 names verified
- `effigy test:components` — 357 files, 3065 tests passed
- `effigy check:svelte` — 0 errors
- `effigy react:build` — clean
- `effigy probe:gpui-specimens` — 8 passed (the canonical sweep constructs and
  paints the breadcrumbs route with the new group)
- `effigy check:gpui` — clean
- `effigy ci:rust` — clean
- `effigy docs:check` — clean, including `gate:clean` (no writer drift)
- `git diff --check origin/main...HEAD` — clean

## Limitations

- `effigy drift:roles` cannot run here: it builds the Jetstream preview, whose
  `jetstream-input` sibling checkout is absent from this worktree. Environment,
  not diff — it is not in the card's validation list, and the card forbids
  adding a paired checkout.
- Explicit `size` resolves differently across runtimes: the web takes it
  verbatim (`size ?? resolveSemanticControlSize(...)`) while Rust applies the
  component's size role on top of it (`RenderContext::resolve_size`). So
  `size="lg"` yields an `lg` item icon on the web and an `md` one natively.
  This is pre-existing `g15.043` `RenderContext` behavior affecting font size
  and gap equally, not something item icons introduced, and `context.rs` is
  outside this card's writable scope. Flagged for the orchestrator.
- Jetstream is program-deferred as the card directs. No Jetstream source
  needed compile adaptation: every caller builds items through
  `BreadcrumbItem::new`, so the two added fields are source-compatible.
- No live browser or windowed native review was run; operator review of the
  three teaching specimens is still required before merge.
