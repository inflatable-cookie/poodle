# g15.043 UiPresentationProvider Native Cascade

Date: 2026-08-23
Card: `../../roadmaps/g15/043-ui-presentation-provider-native-cascade.md`
Handoff: `../../handoffs/20260823-004625-g15-043-ui-presentation-provider-native-cascade.md`
Architecture: `../../architecture/010-native-presentation-construction-context.md`
Worker branch: `t3code/g15-043-ui-presentation-provider-native-cascade`
Worker worktree:
`/Users/tom/.t3/worktrees/poodle/g15-043-ui-presentation-provider-native-cascade`
(manual fallback per contract 005: the launcher supplied no `g15.043`
worktree, so it was created under the operator-selected
`AGENTS_WORKTREE_CONTAINER_DIR` from `.agents.local.env`; registered, clean
at start, non-`main`)
Planning base: `288746f357334b828ff30e949e066f45998f9b5f`, an ancestor of the
branch point `030d04c263f7a0b61d01337c67c282da01a298e3` (= `origin/main` at
dispatch, containing the handoff itself)

## Outcome

The native presentation cascade is real. One explicit borrowed
`poodle_render::RenderContext` carries the token-only `ThemeProvider` plus
the effective size-scale/density defaults through shared construction; every
public component renderer receives it; every semantic component
`size`/`density` input preserves omission in the type system; and
`UiPresentationProvider` is a construction boundary that builds its child
inside a nested context and returns it unchanged. The GPUI preview's no-op
provider facade is deleted, and its specimen now demonstrates the real
cascade instead of labelling explicit host equivalents.

No compatibility twin, alias, default-value heuristic, ambient state, Node
metadata, backend provider, or universal scene abstraction was added.

## Final Measured Counts

Recomputed at completion (starting counts in parentheses are the planning
audit's):

- Component spec files with semantic `ControlSize`/`ControlDensity`: **125**
  (125) — 124 now preserve omission via `Option` fields; the 125th is
  `UiPresentationProviderSpec`, whose own two values stay concrete by design.
  That is the only denominator explanation: the audit's 125 counted the
  provider spec among files mentioning the semantic types.
- Concrete semantic size surfaces: **0** outside the provider spec (107 at
  start, including `pill.rs`'s qualified-path density field the first sweep
  missed and the standing guard then caught).
- Concrete semantic density surfaces: **0** outside the provider spec (117 at
  start).
- Files already retaining optional presentation inputs at start: 7 (button,
  history_center, text_input, number_input, app_header, media_preview, meter,
  status_indicator cohort — of which app_header and media_preview still
  resolved omission internally against root defaults; both now take resolved
  values like the rest).
- `poodle-render` modules referencing `RenderContext`: **169** (168 accepted
  `ThemeProvider` directly; +1 is `context.rs` itself).
- Bare `&dyn ThemeProvider` in `packages/render/src/` outside `context.rs`:
  **0** (enforced by the standing guard).
- Render modules constructing descendant specs: 113 at start; all now
  propagate ctx-resolved base/effective values into those specs
  (compiler-driven; no residual `spec.size`/`spec.density` read outside a
  `ctx.*` resolver).
- Paired-web internal-provider owners: **14 audited, 6 real native
  boundaries** (see below).

## Public API Changes (atomic pre-v1 break)

- `poodle_render::context::RenderContext<'a>`: `new(&'a dyn ThemeProvider)`
  (root defaults `md`/`default`), `scoped(size_scale, density)`, `theme()`,
  `size_scale()`, `density()`, `base_size(Option<ControlSize>)`,
  `resolve_size(Option<ControlSize>, SemanticControlSizeRole)`,
  `resolve_density(Option<ControlDensity>)`.
- `poodle_render::context::ui_presentation_provider(&spec, &ctx, FnOnce(&RenderContext) -> R) -> R`:
  the provider construction boundary; returns the built child unchanged.
- `poodle_render::context::SlotBuilder<'a> = Box<dyn FnOnce(&RenderContext<'_>) -> Node + 'a>`:
  the bounded immediate child builder for scoped host slots.
- Every public component renderer: `theme: &dyn ThemeProvider` →
  `ctx: &RenderContext<'_>`. Theme-only internal helpers reach the theme via
  `ctx.theme()`.
- Every semantic spec input: `size: ControlSize` → `Option<ControlSize>`,
  `density: ControlDensity` → `Option<ControlDensity>`; defaults `None`;
  `with_size`/`with_density` keep names and parameter types, storing `Some`.
  Component-specific domains (`AvatarSize`, `IconSize`, `SpinnerSize`,
  `PillSize`, `EmptyStateSize`, meter numerics) unchanged.
- Spec helper methods that consumed concrete fields now take resolved values
  (e.g. `CardSpec::padding_x_rem(density)`, `StepperSpec::resolved_size(size)`);
  helpers that apply `size_role` internally receive the base size.
- Scoped host slots (see audit): `Option<Node>`/`Vec<Node>` →
  `Option<SlotBuilder>`/`Vec<SlotBuilder>` on `app_header`, `field`,
  `filter_toolbar`, `media_preview_with_content`, `page_header`,
  `block_editor_with_children`; `ListContainerSlots` gained a lifetime with
  `breadcrumbs`/`actions` as builders (they forward into PageHeader's scope,
  matching the web's lazy snippets).

## Fourteen Internal-Provider Owner Audit

Web evidence: each Svelte component wraps content in
`<UiPresentationProvider sizeScale={…} density={…}>` with props-derived
values. Native verdicts:

| Component | Verdict | Evidence / action |
| --- | --- | --- |
| ActionDiscoveryPanel | no host child crosses | no snippet props; native renderer takes zero `Node`s and builds all descendants from spec data through `ctx` |
| AppHeader | **scope established** | `identity`/`center`/`actions`/`utility` web snippets sit inside the provider (`AppHeader.svelte:49,61,74,82-100`); native slots are now `SlotBuilder`s invoked under `ctx.scoped(resolve_size, resolve_density)` |
| BlockEditor | **scope established** | web `block` snippet inside provider (`BlockEditor.svelte:289,410`); `block_editor_with_children` children are now builders |
| CommandPalette | no host child crosses | no snippet props; native takes handlers only |
| EditableList | no host child crosses | web `item` snippet (`EditableList.svelte:394`) has no native slot; rows render from spec data |
| Field | **scope established** | web `control`/`children` inside provider (`Field.svelte:120-132`); `control` is now a builder scoped at the role-resolved size (matching the web's `resolveSemanticControlSize` scope) |
| FilterToolbar | **scope established** | web provider wraps all four host snippets and publishes the RAW base size (`FilterToolbar.svelte:69-73`); `children`/`actions`/`secondary` are now builders scoped at `base_size` + resolved density |
| LogList | no host child crosses | web `actionIcon`/`entryDetails` (`LogList.svelte:331,375-379`) have no native counterpart; native renders no audit rows |
| MarkdownEditor | no host child crosses | no snippet props (`MarkdownEditor.svelte:16-30`) |
| MediaBrowsePanel | no host child crosses | no snippet props (`MediaBrowsePanel.svelte:16-28`) |
| MediaPicker | no host child crosses | no snippet props (`MediaPicker.svelte:19-32`) |
| MediaPreview | **scope established** | web `mediaContent` inside provider (`MediaPreview.svelte:67,81`); `media_preview_with_content` slot is now a builder. The web `children` body snippet (`:114`) has no native surface at all — recorded, not added here |
| PageHeader | **scope established** | web provider wraps the whole header incl. `actions`/`breadcrumbs`/`meta` (`PageHeader.svelte:118,185,201,211`); all three native slots are now builders |
| RelationPicker | no host child crosses | web `renderItem`/`stateContent` (`RelationPicker.svelte:614,555`) have no native surface; `state_content` is hardcoded `None` natively |

Recorded forward caveats (not new work): a future native `item` slot for
EditableList or `renderItem`/`stateContent` for RelationPicker must enter as
`SlotBuilder`s, and MediaPreview's missing body `children` slot is a separate
parity gap.

## Proof Cases

- Resolver laws (`packages/render/src/context.rs` tests): root defaults,
  outer scope, nested scope, sibling restoration, explicit `md`, explicit
  `default`, role-after-base ordering, provider build/return semantics.
- Component cascade proofs (same module):
  `button_and_text_input_inherit_the_provider_scope` (inherited output equals
  the explicit xl/comfortable reference and differs from root: 52px vs 36px),
  `explicit_md_and_default_reset_wins_inside_a_non_default_scope`,
  `nested_provider_replaces_the_outer_scope_for_its_closure_only`,
  `a_scoped_host_slot_builds_inside_the_composites_scope` (FilterToolbar host
  button inherits xl/comfortable through the slot builder),
  `the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`.
- Mounted headless GPUI evidence
  (`packages/gpui/preview/tests/headless_regressions.rs`:
  `a_provider_scope_cascades_to_mounted_geometry_without_a_wrapper_node`):
  an omitted-input button inside an xl/comfortable provider mounts at the
  inherited painted bounds (50px vs the root-default sibling's 34px; recorded
  bounds exclude the 1px border per side), the mounted node IS the button,
  and the backend's real focus machinery reaches it directly.
- GPUI specimen (`specimens/ui_presentation_provider.rs`) demonstrates root
  defaults, two inherited scopes, a nested override, and an explicit reset —
  every scoped control omits size/density.
- Standing guard (`scripts/check-presentation-cascade.ts`, effigy selector
  `drift:presentation-cascade`, wired into `ci:native`). Planted-regression
  proofs, each caught and restored:
  - concrete `pub size: crate::types::ControlSize` in `button.rs` → FAIL;
  - `pub fn badge(..., _theme: &dyn poodle_adapter::ThemeProvider)` → FAIL;
  - manual-equivalent specimen copy (`.with_size(size).with_density(density)`
    from scope variables) → FAIL;
  - preview passthrough facade (`struct UiPresentationProvider` in
    `providers.rs`) → FAIL (this plant exposed and fixed a doc-comment
    suppression bug in the guard's first draft).

## Jetstream Adaptation (compile-only, no parity claim)

- `packages/jetstream/adapter/src/render_action.rs` / `render_input.rs`:
  legacy direct-manifest proofs pass root-resolved values to the re-signed
  spec helpers (documented in code as root-default resolution).
- `packages/jetstream/preview/src/compat.rs`: all `js_*` facades pass a root
  `RenderContext`; the five slot-carrying facades wrap the preview's eagerly
  built `El` nodes in slot builders — documented in code as a compile-only
  wrap. The eager preview cannot inherit internal scopes until it defers
  construction; that is deferred-program work, not claimed here.
  `specimens/shell_status_bar.rs` passes the root-resolved size to the
  re-signed `font_size_rem`.
- `packages/jetstream/preview` cannot compile in this worktree (it
  path-depends on the sibling Jetstream engine checkout); its edits are
  parse-checked with rustfmt and kept mechanically minimal.

## Validation (exact)

- `cargo test --manifest-path packages/contracts/components/Cargo.toml` —
  267 passed.
- `cargo test --manifest-path packages/render/Cargo.toml -p poodle-render` —
  387 passed (9 resolver laws + 5 provider/composition proofs included).
- `cargo test -p poodle-gpui --manifest-path packages/gpui/adapter/Cargo.toml`
  — 133 passed; `cargo test -p poodle-jetstream --manifest-path
  packages/jetstream/adapter/Cargo.toml` — 162 passed.
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test
  headless_regressions` — 64 passed (incl. the new mounted provider proof).
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test
  catalogue` — 7 passed; `--bin poodle-preview specimen_probe` — 8 passed.
- `bun scripts/check-presentation-cascade.ts` — 349 files, 0 violations;
  four planted regressions caught and restored (above).
- `effigy check:gpui`, `effigy ci:rust`, `effigy ci:native`, `effigy
  docs:check`, `git diff --check origin/main...HEAD` — final board results
  recorded in the PR body (run at completion).
- No windowed, native-visual, workflow, release, tag, or publication path was
  run.

## Scope Notes

- One straggler class found during integration: `app_header.rs` and
  `media_preview.rs` specs still resolved omission internally against root
  defaults (`unwrap_or(Md)`/`unwrap_or(Default)` helpers); both were
  re-signed to take resolved values. `pill.rs`'s qualified-path
  `density: crate::types::ControlDensity` field escaped the first sweep and
  was caught by the new guard — migrated like the rest.
- No Svelte/React component, CSS, token, theme, node-vocabulary, workflow,
  or release surface changed.
