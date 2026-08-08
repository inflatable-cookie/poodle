# Underlay Bridge

**Package:** `@inflatable-cookie/poodle-bridge-underlay` (internal — not for public release)
**Status:** active · internal adoption artifact
**Updated:** 2026-04-23

Zero-leak adapter layer between Poodle and Underlay-owned public APIs. This
package lets the Underlay team adopt Poodle internally while keeping every
app-facing surface fully Underlay-owned.

## Who This Is For

This package is for the **Underlay team**, not for Underlay app developers.
App code never imports from `@inflatable-cookie/poodle-bridge-underlay` directly.

The dependency shape is:

```
Underlay App Code
    ↓  imports from
@underlay/ui/*  (Underlay-owned wrappers)
    ↓  internally consumes
@inflatable-cookie/poodle-bridge-underlay  (token aliases, wrapper policies, proof artifacts)
    ↓  resolves values from
Poodle Tokens / Components
```

Underlay apps see only `@underlay/ui/*`. The bridge and Poodle are internal
implementation detail.

## Ownership

| Layer | Owner | What it controls |
|---|---|---|
| Token meaning and component contracts | Poodle | Canonical definitions |
| Alias maps and wrapper-preservation policies | This bridge | Translation rules |
| App-facing APIs, rollout, deprecation | Underlay | Public contracts |

The bridge maps — it does not redefine. If a token alias meaning diverges from
the Poodle canonical definition, that is a bug in the bridge, not a valid
customization.

---

## Setup

### 1. Install

Add to the Underlay internal adapter package (not to app code):

```sh
bun add @inflatable-cookie/poodle-bridge-underlay
```

### 2. Import the CSS alias layer

In the Underlay adapter's entry stylesheet (loaded once at app root):

```css
@import "@inflatable-cookie/poodle-bridge-underlay/css/poodle-to-underlay.css";
```

This file:
- Imports all Poodle token artifacts (base tokens, all themes, densities, control sizes)
- Declares a `:root` block with `--underlay-*` aliases pointing to the same
  resolved values as the corresponding `--poodle-*` variables
- Never leaks `--poodle-*` names into your component code

After this import, your wrappers reference `--underlay-canvas-bg`,
`--underlay-text-primary`, etc. — not any Poodle variable directly.

### 3. Activate a theme

Use the theme map to translate an Underlay theme identifier into the Poodle
data attributes that drive CSS custom property resolution:

```typescript
import { underlayThemeMap } from "@inflatable-cookie/poodle-bridge-underlay/theme-map";

function applyUnderlayTheme(el: HTMLElement, underlayThemeId: string) {
  const mapping = underlayThemeMap.find(m => m.underlayThemeId === underlayThemeId);
  if (!mapping) throw new Error(`Unknown theme: ${underlayThemeId}`);

  // Activate the corresponding Poodle theme via data attributes
  el.setAttribute("data-theme", mapping.poodleThemeId);
}

// In your app shell:
applyUnderlayTheme(document.documentElement, "underlay-night");  // → data-theme="eclipse"
```

Theme identifiers:

| Underlay ID | Maps to Poodle theme | CSS artifact |
|---|---|---|
| `underlay-default` | `iceberg` | Poodle light theme |
| `underlay-night` | `eclipse` | Poodle dark theme |
| `underlay-studio` | `graphite` | Workstation-oriented dark |

Density and control-size are applied the same way using `underlayDensityModeMap`
and `underlayControlSizeMap`:

```typescript
import { underlayDensityModeMap } from "@inflatable-cookie/poodle-bridge-underlay/theme-map";

const densityMapping = underlayDensityModeMap.find(m => m.underlayModeId === "underlay-compact");
el.setAttribute(densityMapping.selector, "");  // activates Poodle density mode
```

---

## Token Aliases

The token map records the current set of `--underlay-*` aliases and the Poodle
semantic token they resolve to.

```typescript
import { underlayTokenMap, type UnderlayBridgeToken } from "@inflatable-cookie/poodle-bridge-underlay/token-map";

// underlayTokenMap: UnderlayBridgeToken[]
// Each entry:
type UnderlayBridgeToken = {
  underlayVar: string;       // "--underlay-canvas-bg"
  poodleTokenPath: string;   // "color.background.canvas"
  cssVar: string;            // "--poodle-color-background-canvas"
  note?: string;
};
```

Current token aliases (11 total):

| Underlay variable | Poodle token path | Semantic meaning |
|---|---|---|
| `--underlay-canvas-bg` | `color.background.canvas` | Page/window background |
| `--underlay-surface-bg` | `color.background.surface` | Card/panel surfaces |
| `--underlay-panel-bg` | `color.background.panel` | Nested panel background |
| `--underlay-text-primary` | `color.text.primary` | Primary text |
| `--underlay-text-secondary` | `color.text.secondary` | Secondary/muted text |
| `--underlay-border-default` | `color.border.default` | Default border color |
| `--underlay-accent` | `color.accent.base` | Accent/brand color |
| `--underlay-panel-space-x` | `space.panel.x` | Panel horizontal padding |
| `--underlay-panel-space-y` | `space.panel.y` | Panel vertical padding |
| `--underlay-control-height` | `size.control.height` | Standard control height |
| `--underlay-panel-header` | `size.panel.header` | Panel header height |

**Policy:** The alias list widens only in response to concrete adoption pressure.
Do not pre-alias tokens that no Underlay wrapper yet references — the alias map
is evidence of real usage, not a speculative catalog.

---

## Writing a Component Wrapper

The component wrappers module defines the current set of approved wrapper
policies. Each wrapper must follow the zero-leak rules.

```typescript
import { underlayWrapperPolicies, underlayZeroLeakRules } from "@inflatable-cookie/poodle-bridge-underlay/component-wrappers";
```

### Zero-leak rules (non-negotiable)

```
1. Underlay apps do not import Poodle packages directly.
2. Underlay apps do not depend on Poodle component names.
3. Underlay apps do not depend on Poodle token variable names.
4. Wrapper layers preserve accessibility, focus, and keyboard semantics.
```

### Approved wrapper surfaces

Four surfaces are currently proven and policy-documented:

| Underlay export | May use Poodle internal | Notes |
|---|---|---|
| `@underlay/ui/Button` | Button family | Variant/size translation, focus-ring parity, event-name compatibility |
| `@underlay/ui/SearchInput` | TextInput | Query prop naming, submit/cancel behavior, result-shell composition |
| `@underlay/ui/Panel` | Panel internals | Header slot translation, panel identity naming, workstation-only rollout |
| `@underlay/ui/NightfireBlockEditor` | BlockEditor shell | Single/multi posture mapping, opaque block envelope pass-through, grouped type picker override when subcategory menus matter |

### Wrapper implementation pattern

An Underlay wrapper receives Underlay-named props, translates to Poodle props
internally, renders the Poodle component, and returns an element with an
Underlay-owned import path. App code never sees Poodle:

```svelte
<!-- packages/underlay-ui/src/Button.svelte -->
<!-- App imports: import Button from "@underlay/ui/Button" -->

<script lang="ts">
  // Underlay-owned prop names — NOT Poodle names
  import { Button as PoodleButton } from "@inflatable-cookie/poodle-svelte";  // internal use only
  import type { HTMLButtonAttributes } from "svelte/elements";

  // Underlay prop surface
  export let intent: "primary" | "secondary" | "ghost" = "secondary";
  export let scale: "sm" | "md" | "lg" = "md";
  export let disabled: boolean = false;

  // Translate to Poodle equivalents (bridge-owned mapping)
  const variantMap = { primary: "solid", secondary: "outline", ghost: "ghost" } as const;
  const sizeMap    = { sm: "sm", md: "md", lg: "lg" } as const;

  $: poodleVariant = variantMap[intent];
  $: poodleSize    = sizeMap[scale];
</script>

<!-- Poodle component is internal implementation detail -->
<PoodleButton variant={poodleVariant} size={poodleSize} {disabled} on:click>
  <slot />
</PoodleButton>
```

App code:

```svelte
<!-- ✅ Correct — Underlay import path, Underlay prop names -->
<script>
  import Button from "@underlay/ui/Button";
</script>
<Button intent="primary" scale="md" on:click={save}>Save</Button>
```

```svelte
<!-- ❌ Wrong — leaks Poodle into app code -->
<script>
  import { Button } from "@inflatable-cookie/poodle-svelte";
</script>
<Button variant="solid" size="md" on:click={save}>Save</Button>
```

### Preserving accessibility

Wrappers must not drop any of these from the underlying Poodle component:
- ARIA roles and attributes (`role`, `aria-label`, `aria-expanded`, etc.)
- Focus management and focus ring
- Keyboard event handling
- Announcement / live region behavior

If your wrapper needs to intercept an event or slot, verify the accessibility
semantics still flow through. When in doubt, pass remaining attributes with
`{...$$restProps}` (Svelte) or spread.

---

## Nightfire Block Editor

Nightfire is now an explicit bridge use-case. Poodle owns the shell, but
Underlay still owns the Nightfire block JSON shape, schema definition, grouped
type picker semantics, and app-facing wrapper import path.

Use the bridge helper to map Nightfire records onto the Poodle shell without
teaching app code about Poodle types:

```typescript
import {
  buildNightfireBlockEditorBridge,
  toPoodleEditorBlocks,
} from "@inflatable-cookie/poodle-bridge-underlay/nightfire-block-editor";

const bridge = buildNightfireBlockEditorBridge({
  typeOptions: editorTypeOptions,
  groupedOptions,
});

const blocks = toPoodleEditorBlocks(value.blocks ?? [], {
  fallbackType: definition.defaultType,
});
```

Then the Underlay-owned wrapper can decide whether the built-in picker is
enough or whether it should keep rendering `NightfireTypeSelect` through
Poodle's `type-picker` slot:

```svelte
<BlockEditor
  {blocks}
  blockTypes={bridge.blockTypes}
  blockTypeItems={bridge.blockTypeItems}
  mode={definition.mode}
>
  {#if bridge.pickerMode === "slot-override"}
    <svelte:fragment slot="type-picker" let:block let:changeType>
      <NightfireTypeSelect
        value={block.type}
        {groupedOptions}
        typeOptions={editorTypeOptions}
        onChange={changeType}
      />
    </svelte:fragment>
  {/if}
</BlockEditor>
```

Rule of thumb:
- use Poodle's built-in picker when flat or one-level grouped menus are enough
- keep an Underlay-owned slot override when Nightfire needs nested
  category/subcategory menus
- keep block payloads opaque; the bridge should only ensure `id` and `type`
  exist for the shell

---

## Zero-Leak Proof

The proof artifact makes the zero-leak posture machine-readable and
validatable:

```typescript
import {
  underlayZeroLeakProof,
  validateUnderlayZeroLeakProof,
  type UnderlayZeroLeakProof,
} from "@inflatable-cookie/poodle-bridge-underlay/zero-leak-proof";

// Validate at build time or in tests
validateUnderlayZeroLeakProof();  // throws if invariants are violated
```

`validateUnderlayZeroLeakProof()` checks:
- At least one bridge token alias is defined
- At least one theme mapping exists
- At least one adoption surface is recorded
- No recorded surface allows direct Poodle imports in app code

Run this in your CI or test suite to confirm the bridge invariants hold as the
alias map grows.

The proof object (`underlayZeroLeakProof`) is also useful as a reference
artifact — it records the current canonical dependency shape
(`"bridge-owned"`) and the list of remaining friction points.

---

## Remaining Adoption Friction

These items are explicitly tracked in the proof artifact and must be resolved
before the bridge posture is considered fully hardened:

1. **Theme IDs are placeholders** — `underlay-default`, `underlay-night`,
   `underlay-studio` are bridge-local names. Once Underlay's canonical theme
   identifier names are finalized, update `underlayThemeMap` to use them.

2. **Wrapper prop translation is policy-first** — The three current wrapper
   policies describe *how* props should be translated but do not yet have
   downstream wrapper package evidence. Completing a real Underlay wrapper
   package that consumes these policies is the next adoption step.

3. **Accessibility parity needs downstream evidence** — The zero-leak rules
   require wrappers to preserve accessibility semantics, but this has not yet
   been validated with real assistive technology testing on Underlay-wrapped
   components.

4. **Token alias map is intentionally narrow** — Only 11 aliases exist,
   corresponding to current proof surfaces. As more Underlay wrappers are
   built, the alias map should grow to match — but only in response to real
   wrapper code, not speculatively.

---

## Package Exports

| Import path | What it provides |
|---|---|
| `@inflatable-cookie/poodle-bridge-underlay` | Re-exports everything from all subpaths |
| `@inflatable-cookie/poodle-bridge-underlay/token-map` | `underlayTokenMap`, `canonicalTokenFamilies`, `UnderlayBridgeToken` |
| `@inflatable-cookie/poodle-bridge-underlay/theme-map` | `underlayThemeMap`, `underlayDensityModeMap`, `underlayControlSizeMap`, `canonicalPoodleThemes` |
| `@inflatable-cookie/poodle-bridge-underlay/component-wrappers` | `underlayWrapperPolicies`, `underlayZeroLeakRules`, `UnderlayWrapperPolicy` |
| `@inflatable-cookie/poodle-bridge-underlay/nightfire-block-editor` | Nightfire block envelope and type-picker mapping helpers for Poodle `BlockEditor` |
| `@inflatable-cookie/poodle-bridge-underlay/zero-leak-proof` | `underlayZeroLeakProof`, `validateUnderlayZeroLeakProof`, proof types |

---

## Reference Documentation

- Architecture and ownership rules: `docs/architecture/004-underlay-bridge-and-adapter-ownership.md`
- Bridge specification: `docs/specs/007-underlay-bridge-and-wrapper-preservation-rules.md`
- Zero-leak adoption proof spec: `docs/specs/040-underlay-bridge-zero-leak-adoption-proof-baseline.md`
- Ecosystem acceptance: `packages/ecosystem-acceptance.json` → `underlay-bridge-adoption` suite
