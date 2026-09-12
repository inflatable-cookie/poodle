# g18.023 — CodeEditor dual syntax palettes

Status: ready for review
Date: 2026-09-12
Branch: `ns-ebf1cc36-5a9a-46dc-9649-e42f0883673e`
Card: `docs/roadmaps/g18/023-code-editor-dual-syntax-palettes.md`
Handoff: `docs/handoffs/20260911-g18-023-code-editor-dual-syntax-palettes.md`
Governing refs: `docs/contracts/components/code-editor.md`,
`docs/architecture/002-token-system-and-package-layout.md`,
`packages/tokens/schema/`,
`packages/svelte/components/src/code-editor-engine.ts`,
`packages/react/components/src/code-editor-engine.ts`,
`test/code-editor-language-registry/`
Base: `origin/main` at `4255c62ee16b0f35447189509142fc5b7e6e50ae`

## Outcome

`CodeEditor` syntax highlighting no longer borrows UI status and accent
tokens. The token system ships two deliberately designed primitive syntax
ramps — `color.syntax.dark.*` and `color.syntax.light.*` — each defining
comment, keyword, string, literal, type, callable, property, operator,
punctuation, and invalid values, and ten semantic `color.syntax.*` roles
reference the dark primitives by default. The three light theme modes
(iceberg, clay, meadow) select the light primitives through role-level
overrides; dark themes inherit the dark base unchanged; any named theme can
override one role at a time without replacing the palette. The private
`HighlightStyle` in both engine twins maps stable Lezer tag groups onto those
roles by meaning, ordinary identifiers stay primary text, and invalid parser
syntax keeps its role colour plus a new non-colour wavy-underline cue. Every
role meets WCAG AA (≥ 4.5:1) against the actual rendered editor panel of every
named theme, proven in real Chromium and WebKit for both frameworks, with
live theme switches restyling the same mounted editor and overlays that never
repaint token text.

## Palette design

Values were contrast-driven from the start: the binding panels are solarized
`#0a4250` (brightest dark) and clay `#f9f4ea` (brightest light), and every
role keeps ≥ 4.5:1 against the raw schema panel of every theme, so the
contrast axis (which renders panels between canvas and panel lightness, i.e.
friendlier in both directions) never breaks the floor. All 20 ramp values
land between 4.65:1 (light punctuation on clay) and 6.78:1 (dark literal on
solarized); chromatic roles span six hue families — violet keyword (296°),
green string (149°), amber literal (76°), cyan type (201°), blue callable
(261°), rose property (352°) — with red invalid (26°) plus restrained
blue-grey neutrals for comment, operator, and punctuation. Light values are
independently tuned darker chromatic counterparts, not inversions: hue
shifts (light type goes teal 221°, light literal amber-brown 64°) and
chroma rebalancing (light keyword C 0.23 vs dark 0.10) track light-surface
perception. Dark and light bases differ in every one of the ten roles.

## What changed

- `packages/tokens/schema/primitives/color.json`: `color.syntax.dark.*` and
  `color.syntax.light.*` primitive ramps (10 roles each) with per-role
  intent descriptions.
- `packages/tokens/schema/semantic/color.json`: semantic `color.syntax.*`
  roles referencing `{primitives.color.syntax.dark.*}` by default, per the
  approved decision; descriptions record the light-mode selection rule.
- `packages/tokens/schema/modes/themes/{iceberg,clay,meadow}.json`: each
  light theme overrides all ten roles to the corresponding light primitives —
  role-level references, no palette replacement, no colour literals copied.
  Dark themes need zero overrides: the dark ramp passes AA on every dark
  panel, including solarized. No other theme file changed.
- Regenerated artifacts: `packages/tokens/artifacts/{css,ts,rust}/` and
  `packages/core/src/tokens/generated/`; `audit:tokens` verifies the tree
  reproduces them byte-for-byte.
- Both engine twins (`packages/{svelte,react}/components/src/code-editor-engine.ts`,
  byte-identical apart from the header comment): the private
  `codeEditorHighlightStyle` now maps — comments/meta to `syntax.comment`;
  the whole keyword family (`keyword` covers control/definition/module/
  modifier/operator keywords plus `this`) to `syntax.keyword`;
  strings/templates/regexp/escape to `syntax.string`; numbers/bool/null/atom
  to `syntax.literal` (bool/null/atom beat the keyword rule by Lezer tag
  specificity, not rule order); type/class/namespace names and their
  definitions to `syntax.type`; variable definitions, function calls, method
  calls, and function declarations to `syntax.callable`; properties,
  attributes, member names, and their definitions to `syntax.property`;
  operators to `syntax.operator`; punctuation/separators/brackets to
  `syntax.punctuation`; `tags.invalid` to `syntax.invalid`. Every rule
  resolves a `--poodle-color-syntax-*` variable, so live theme switches
  restyle mounted editors without remount or grammar reload. The parser-error
  mark plugin keeps its class but now carries
  `var(--poodle-color-syntax-invalid)` inline.
- `packages/core/src/styles/code-editor.css`:
  `.poodle-code-editor__syntax-invalid` gains the wavy underline
  (non-colour cue) in the invalid role colour; diagnostics keep their own
  marks and remain host-owned.
- `docs/contracts/components/code-editor.md` §8: the transitional note is
  replaced by the shipped-state statement; the role/palette/contrast text was
  already the approved target and is now the implemented contract.
- Paired fixture (`test/code-editor-language-registry/`): the harness twins
  use a richer representative TypeScript sample — comment, keywords, a type
  and its uses, a function declaration, literals, a string, member access,
  operators, punctuation, and unstyled identifier uses (`entry` use site,
  `string`, `boolean`) — alongside the unchanged JSON sample whose property
  keys now intentionally take the property role. The probe replaces every
  exact status-token equality with the g18.023 proof set below.
- `PAPERCUTS.md`: one observation recorded (undefined
  `--poodle-color-surface-hover` in `code-editor.css`); left unfixed as
  editor state redesign is out of scope.

## Planted regression (before/after)

The probe's new g18.023 checks were run against the pre-repair engine (engine
twins reverted, schema and probe already updated): per framework per engine
the status-token assertions and the old `text-secondary`/accent/success/info/
warning expectations failed across the board — TypeScript resolved keyword to
accent, strings to success, definitions to warning, and JSON keys to plain
text, so the "five distinct chromatic roles", "roles equal the dark
primitives", "property role on JSON keys", "invalid role contrast", and
per-theme AA sweeps all failed — while every g18.012 registry check kept
passing. With the repair applied, all checks pass in Chromium and WebKit for
both frameworks.

## Proof surface (per framework, per engine)

- Representative TypeScript: comment/keyword/type/callable/literal/string/
  property/operator/punctuation spans each equal the live
  `--poodle-color-syntax-*` variable for their role (never accent or status);
  six distinct chromatic roles in one sample; the `entry` use site carries no
  styled span.
- Representative JSON: property keys take the property role; strings, number/
  boolean literals, and brace/separator punctuation take their roles; three
  distinct chromatic roles; TypeScript + JSON combined clear the five-role
  floor.
- Base wiring: eclipse resolves every role to the exact dark primitive;
  iceberg resolves keyword to the exact light primitive; iceberg and eclipse
  differ in all ten roles (designed, not inverted — hue/chroma recorded
  above).
- Theme sweep: all twelve named themes switch live on the marked editor node;
  every role meets ≥ 4.5:1 against that theme's actual rendered panel
  (oklch axis rendering parsed in-page and converted to sRGB; worst roles:
  dark punctuation on solarized ≈ 5.8:1 rendered, light punctuation on clay ≈
  4.7:1); roles stay stable within a base and switch exactly when the
  dark/light class changes; document, editor identity, and an active text
  selection survive every switch.
- Invalid syntax: parser error nodes take the invalid role and a wavy
  underline (non-colour cue) whose role contrast on the panel is ≥ 4.5;
  repairing the document removes the marks.
- Overlays: double-click selection and an active search match leave every
  token at its exact role colour; both overlays are proven translucent
  (resolved `::selection` alpha 0.28 accent tint; 0.18 search tint).
- Plain postures: plain-text language and `performanceMode="plain"` render
  zero styled spans and load no grammar; full mode reinstates presentation on
  the same editor without remount and without new registry loads.
- Forced colours: the mounted editor stays visible and legible under
  `forcedColors: active` and the token presentation recovers afterwards.
- g18.012 board unchanged: lazy single loads, memoized re-selection, live
  switching, typing/undo across switches, fail-closed rejected loads,
  mount-time unknown-id refusal.

## Validation

- `effigy audit:tokens` — clean (generated artifacts in sync).
- `effigy test:code-editor-language-registry-chromium` and `-webkit` — all
  checks pass for both frameworks in both engines.
- `bunx vitest run packages/svelte/components/test/CodeEditor.test.ts
  packages/react/components/test/CodeEditor.test.tsx
  packages/svelte/components/test/CodeEditorPackaging.test.ts` — 101 pass,
  4 skipped.
- `bunx vitest run --project svelte-components --project react-components` —
  3062 pass, 8 skipped. `bunx vitest run --project svelte-preview --project
  react-preview` — 126 pass.
- `cargo test --manifest-path packages/contracts/tokens/Cargo.toml` — clean;
  `cargo check --manifest-path packages/jetstream/adapter/Cargo.toml` — clean
  (new generated Rust constants compile; the adapter maps only the tokens it
  consumes and does not enumerate the semantic registry).
- `effigy core:build`, `effigy svelte:package`, `effigy react:package`,
  `effigy svelte:build`, `effigy react:build` — clean (pre-existing specimen
  warnings and chunk-size notices only).
- `effigy test:web-pack-install` — clean: the isolated packed-install consumer
  still resolves the pinned CodeMirror set, and no grammar package enters the
  install graph.
- `effigy docs:check` — pass. `git diff --check` — clean.

## Explicitly not done

- No public CodeMirror extension, highlight style, tag, theme, palette prop,
  or engine surface; component props and adapter APIs are unchanged.
- No grammar dependency, no language-specific colours, no diagnostics
  redesign; host diagnostics and severity marks are untouched.
- No g18.024 Slider work, no g18.011/g18.006/g18.009 state changes, no
  release/version/changelog/workflow mutation, no Desktop or native editor
  changes, no g18 README/dispatch index edits (reserved closeout surfaces).
- The `.cm-activeLine` background var (`--poodle-color-surface-hover`) is
  undefined in the token system today; active lines therefore render on the
  plain panel. Out of scope here — recorded in `PAPERCUTS.md` instead.

## Continuation

g18.023 is ready for exact-head independent review. After it and g18.024
merge and the repaired specimens are accepted, the retained g18.006 release
task resumes with both repairs in the `0.4.0` source identity.
