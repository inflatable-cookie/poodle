# Poodle Core

`@inflatable-cookie/poodle-core` is Poodle's framework-free web package. It
contains shared component behavior, generated design tokens, component CSS,
and icon infrastructure used by the Svelte and React packages.

`0.2.3` is a preview-channel npm release. Breaking changes may still ship in
`0.x` minor releases; no `stable` channel exists yet, so pin an exact version
and read the [release notes](https://github.com/inflatable-cookie/poodle/blob/main/docs/release-notes/0.2.3.md) before
upgrading.

```sh
bun add @inflatable-cookie/poodle-core@0.2.3
# or: npm install @inflatable-cookie/poodle-core@0.2.3
```

`0.1.0` was the first registry version. The `0.2.0` workflow stopped before
publication and `0.2.1` replaced it. `0.2.2` corrected Poodle's public Rust
GPUI dependency identity. `0.2.3` is a lockstep patch; this package carries
no product API change in it.

## Public Surfaces

| Import | Purpose |
| --- | --- |
| package root | State machines, prop getters, and shared interaction helpers |
| `/styles/*` | Shared component CSS |
| `/tokens` | Token values, paths, theme metadata, and helpers |
| `/tokens/runtime` | `applyThemeAttributes()` and mode types |
| `/tokens/styles.css` | Base token custom properties |
| `/tokens/themes.css` | All generated theme selectors |
| `/tokens/theme-<name>.css` | One generated theme |
| `/tokens/density-<mode>.css` | One density mode |
| `/tokens/control-size-<size>.css` | One control-size mode |
| `/icons` | Icon types and Poodle's scoped default Lucide set |

## Custom-surface drag and drop

Same-document pointer, touch, and keyboard drag is a core controller plus
idiomatic Svelte/React bindings. It is not a portable component and does not
add a ledger row.

```ts
import { createDragDropController } from "@inflatable-cookie/poodle-core";

const controller = createDragDropController();
const disconnect = controller.connect(root);
controller.registerSource(sourceEl, {
  sourceId: "clip-1",
  subject: { kind: "clip", id: "clip-1" },
  allowedOperations: ["move"],
  label: "Intro clip",
});
controller.registerTarget(listEl, {
  targetId: "timeline",
  acceptedKinds: ["clip"],
  label: "Timeline",
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (intent) => ({ status: "committed" }),
});
controller.registerKeyboardTarget({
  targetId: "clip-2",
  acceptedKinds: ["clip"],
  label: "Verse clip",
  order: 1,
  resolvePosition: (input) => (input.direction === "previous" ? "before" : "after"),
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: () => ({ status: "committed" }),
});
```

Framework packages export `DragDropProvider`. Svelte uses `useDragDrop()`
actions plus a logical keyboard-target helper; React uses `useDragSource` /
`useDropTarget` / `useKeyboardDropTarget`. See spec 069 and architecture 011.

### External files

Files never enter Poodle. A source that can be dragged out to the operating
system declares a host bridge, and the host answers with an *opaque receipt*:

```ts
const controller = createDragDropController();
controller.registerSource(clipEl, {
  sourceId: "clip-1",
  subject: { kind: "clip", id: "clip-1" },
  allowedOperations: ["copy"],
  label: "Intro clip",
  fileExportBridge: {
    capabilities: { files: true, multipleFiles: false, promisedFiles: false, customDataTypes: [] },
    // Runs on the pre-drag gesture, before activation, and is abortable.
    prepare: async (request, signal) => ({ receiptId: await render(request.subject, signal), displayName: "take-01.wav" , form: "materialized-file" }),
    // The host starts the operating system's drag here — Poodle refuses the
    // browser's own so there is only ever one.
    start: (prepared, onTerminal) => host.startNativeDrag(prepared.receiptId, onTerminal),
    cancel: (prepared, reason) => host.abandon(prepared.receiptId, reason),
  },
});
```

`prepare` returns a `receiptId` the host chooses and Poodle never parses, plus
an optional `displayName` for presentation. Paths, descriptors, temporary
directories, and `File` objects stay in the host, and a display name that
looks like a location is refused rather than presented. Read
`getSnapshot().fileExport` or the source's `data-poodle-drag-export` attribute
for `unavailable | idle | preparing | armed | dragging | ended | cancelled |
failed`.

A native drag ending is not a commit and never authorizes deletion: `start`'s
terminal reports `ended`, `cancelled`, or `failed`, and whether a destination
consumed the file is not something any browser or shell reports back.
Retention and cleanup stay with the host that made the artifact.

Inbound files arrive through one window-owned bridge and reach the ordinary
target path under `INBOUND_FILE_SUBJECT_KIND`:

```ts
const inboundFileBridge = createInboundFileDataTransferBridge({
  // Optional: without it a batch resolves to receipts and no `File` leaves
  // the adapter.
  project: (file) => file,
});
const controller = createDragDropController({ inboundFileBridge });
controller.registerTarget(zoneEl, {
  targetId: "library",
  acceptedKinds: [INBOUND_FILE_SUBJECT_KIND],
  label: "Sample library",
  inboundFiles: { accept: "audio/*", maxFiles: 4, maxSize: 10_000_000 },
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (intent, context) => commit(inboundFileBridge.resolve(context.inboundFiles!.batchId)),
});
```

`inboundFiles` is validated *before* eligibility, on hover and again at drop.
A browser discloses only item kinds and declared types during `dragover`, so
hover-time receipts carry `null` names and sizes and the rules that need them
are answered at the drop.

`capabilities.transport` is an exclusive claim. A shell whose platform
delivers both a native file-drop capture and webview drag events must supply
one bridge that says which owns the window, rather than enabling both and
taking one gesture as two drops.

Electron and Tauri are integration points, not dependencies: Poodle imports
neither. An Electron adapter maps `start` onto `webContents.startDrag` from
its own preload/main channel; a Tauri application supplies a plugin adapter
whose `subscribe` publishes the window's file-drop events as
`InboundFileEvent`s. Both are ordinary implementations of the interfaces
above.

## Web Setup

Import base tokens and the modes your application can select:

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/themes.css";
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

```ts
import { applyThemeAttributes } from "@inflatable-cookie/poodle-core/tokens/runtime";

applyThemeAttributes(document.documentElement, {
  theme: "eclipse",
  density: "default",
  controlSize: "md",
});
```

Framework components import their own shared component styles. Application
code normally does not import `/styles/*` directly.

## Icons

Poodle includes only the Lucide icons required by its components. Generate an
application-owned set for additional names:

```sh
bun x poodle-icons --names icons.json --out src/icons.generated.ts
```

The generated module contains only the requested icon nodes. Merge it into the
Svelte or React `IconProvider`; do not bundle Lucide's full catalogue at
runtime.

## Development

Token artifacts originate in `packages/tokens/schema/`. Default icon names
originate in `src/icons/default-icons.json`; `effigy icons:build` generates the
web modules and shared Rust SVG assets from pinned Lucide data. Regenerate
tokens with `effigy tokens:build` and validate documentation with
`effigy docs:check`.

See the [token architecture](https://github.com/inflatable-cookie/poodle/blob/main/docs/architecture/002-token-system-and-package-layout.md)
and [repository README](https://github.com/inflatable-cookie/poodle/blob/main/README.md) for the full system.
