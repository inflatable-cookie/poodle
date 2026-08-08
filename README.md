# Poodle

Poodle is a multi-renderer design system with one shared contract surface and multiple runtime implementations.
The repo currently ships and validates:

- shared Rust contract crates for tokens, layout, style, primitives, composites, and workstation surfaces
- a Svelte package set and browser preview surface
- a GPUI native adapter, component library, and preview app
- a Jetstream adapter, component library, and preview app
- an Underlay bridge for host adoption without leaking Poodle-specific APIs into app code

## Repo Shape

Key package groups:

- `packages/contracts/*`
  shared renderer-agnostic crates such as `poodle-specs`, `poodle-workstation`, and `poodle-tokens`
- `packages/svelte/*`
  published web packages such as `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-core/tokens`, `@inflatable-cookie/poodle-core/icons`, and the docs or preview app
- `packages/gpui/*`
  native GPUI adapter, renderable components, and preview app
- `packages/jetstream/*`
  Jetstream adapter, renderable components, and preview app
- `packages/bridges/underlay`
  token and wrapper bridge for Underlay-hosted apps
- `packages/tokens`
  canonical token schema and artifact generation pipeline

## Consuming Poodle

Start with the developer guide for your target runtime:

- **Svelte** — [docs/guides/svelte-developer-guide.md](docs/guides/svelte-developer-guide.md)
- **GPUI** — [docs/guides/gpui-developer-guide.md](docs/guides/gpui-developer-guide.md)
- **Jetstream** — [docs/guides/jetstream-developer-guide.md](docs/guides/jetstream-developer-guide.md)

All components are defined by contracts in [docs/contracts/components/](docs/contracts/components/).
The contract is the source of truth for every implementation.

## Canonical Docs

Internal planning and architecture hierarchy:

1. [docs/vision/001-poodle-vision.md](docs/vision/001-poodle-vision.md)
2. [docs/architecture/001-poodle-system-shape.md](docs/architecture/001-poodle-system-shape.md)
3. [docs/roadmaps/README.md](docs/roadmaps/README.md)
4. [docs/specs/README.md](docs/specs/README.md)

## Local Workflow

Install dependencies and generate token artifacts:

```sh
bun install
bun packages/tokens/scripts/build-tokens.ts
```

`bun install` at the repo root is the canonical JS hydration step. Mounted
consumer repos should hydrate the root workspace, not run separate
package-local installs under `packages/svelte/*` unless a package explicitly
documents that requirement.

Common repo tasks:

```sh
effigy health
bun run --cwd packages/svelte/preview dev
cargo run -p poodle-gpui-preview --manifest-path packages/gpui/preview/Cargo.toml
cargo run -p poodle-jetstream-preview --manifest-path packages/jetstream/preview/Cargo.toml
```

The default validation pass is `effigy health`.
That runs docs lint, parity and accessibility artifact generation, and the Svelte production build.

## Naming

Current package and crate namespaces use `poodle` and `@inflatable-cookie/poodle-*`.
Historical `pug` and `flint` references should be treated as migration leftovers unless they appear in explicit rename handoff docs.

## Next Task

Continue the next `g12.019` old-tier constructor wave. Wave 40 moved
EditableLabel onto the node backend with queued text-change, submit, and cancel
intents; its 0.5334% focused text-raster residual remains deferred. The
DurationInput
native-visual residual is parked at a deterministic 0.0033% until backend text
parity is addressed. Meter, Rating, Table, PaginationSummary, ValidationSummary,
Progress, EmptyState, ResizeHandle, MetaBar, MetaItem, NavCard, Callout,
StatusBar, TextLink, Breadcrumbs, Tabs, TabStrip, CodeInput, TokenInput,
FileUpload, SelectionSummary, PasswordRequirements, ErrorBoundary,
InlineListSection, CollapseToggle, Toolbar, OrderBy, RefSelect, and PageHeader are now node-backed with zero old-tier
constructor sites; NavCard, Callout, ErrorBoundary, InlineListSection, and
CollapseToggle, and Toolbar are exact, while metadata/status,
PasswordRequirements, OrderBy, RefSelect, and PageHeader text/icon differences
remain deferred. PickerShell is now node-backed with aligned ready-body
geometry; its 0.5576% focused text/control residual is deferred.
The standalone FormActions specimen is now node-backed with an exact focused
capture. FormDialog is now node-backed too; its remaining 0.1980% focused
residual is deferred modal/text raster parity.
AppHeader is now node-backed too, with an exact focused capture across its
identity, action, and utility slots.
FilterBuilder and MarkdownEditor are now node-backed through the shared
renderers; MarkdownEditor preserves text/mode events through the node queue.
Their focused residuals are 1.0752% and 0.2304% respectively and remain in
the deferred text/layout-raster bucket.
EditableList and RelationPicker are now node-backed too; their focused
residuals are 0.0063% and 1.3903%, with RelationPicker's geometry delta
deferred separately.
FilterToolbar is now node-backed too, including its Select/TextInput child
slots and action/secondary slots; its focused capture is exact. ModelPicker is
now node-backed too; its model/axis panel geometry is aligned and the remaining
0.2638% focused residual is deferred text/control raster parity. Its embedded
AgentChatInput slots leave the same deferred 0.1210% trigger/control residual.
FormLayout is now node-backed; its field/control geometry is aligned and the
remaining 0.7501% focused residual is deferred text/button raster parity.
FieldSet is now node-backed with an exact focused capture. ThemeSelect is also
node-backed with an exact focused capture.
FormDialog is now node-backed; its Dialog/FormLayout modal-stack geometry is
aligned and the remaining 0.1980% focused diff is deferred modal/text raster
parity.
FormShell is now node-backed with section slots and action rows; its focused
capture has only a deferred 0.0054% text/icon raster residual.
MediaThumbnail, EmbedPreview, and MediaPreview are now node-backed as well;
their focused geometry is aligned, with only 0.1492%, 0.2103%, and 0.1154%
text/icon raster residuals deferred.
CardRadioGroup, EmbedInput, and PageLoading are node-backed too; EmbedInput is
focused exact, CardRadioGroup retains a 0.9761% selected-state/text residual,
and PageLoading remains skipped by the native gate.
MediaPicker is node-backed with aligned browse/upload geometry and a deferred
0.5516% icon/text residual. DataTable, AgentQuestion, and AgentTranscript are
node-backed too; AgentQuestion is exact, while DataTable retains a 0.7217%
text/layout-raster residual and AgentTranscript a 0.0131% text-raster residual.
SidebarNav and MediaBrowsePanel are node-backed too, with 0.2954% and 0.1829%
text/icon residuals. ToastStack preserves its corner overlay and retains a
deferred 1.1702% text/icon/animation-raster residual.
ToastHost is node-backed too, preserving placement with a deferred 0.5661%
text/icon/animation-raster residual.
Dialog and Drawer are node-backed too; Dialog keeps custom header/footer slots,
and both focused captures are exact.
DebugDialog, ActionDiscoveryPanel, and BulkActionBar are node-backed too, with
only the documented text/icon-raster residuals.
AgentChatInput is node-backed with ModelPicker, toolbar, and footer slots; its
0.1377% text/control-raster residual is deferred.
