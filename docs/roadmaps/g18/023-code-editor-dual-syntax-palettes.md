# 023 — CodeEditor dual syntax palettes

Status: complete — merged as `155dbc7d82fe04479a986c1f5f5698770e366c17` (PR #255) on 2026-09-12
Owner: Poodle tokens and web components
Created: 2026-09-11
Governing refs: `../../contracts/components/code-editor.md`,
`../../architecture/002-token-system-and-package-layout.md`,
`../../../packages/tokens/schema/`,
`../../../packages/svelte/components/src/code-editor-engine.ts`,
`../../../packages/react/components/src/code-editor-engine.ts`
Depends on: g18.021 complete and merged

## Outcome

Replace CodeEditor’s UI-status-based highlighting with a real syntax colour
system. Ship one deliberate dark base palette and one deliberate light base
palette, map stable Lezer tags to named syntax roles, and let each Poodle theme
override individual roles only where its surfaces need different contrast or
character.

Keep the CodeMirror engine private and the language registry open. Consumers
choose language loaders; Poodle chooses a readable default syntax presentation
through ordinary generated CSS variables. This task may run beside g18.024.
It must merge before g18.006 resumes.

## Ready-State Rubric

- [x] g18.021 made token spans visible but mapped keywords to accent, strings
  to success, literals to info, and types/definitions to warning.
- [x] The active tests prove exact equality with those UI semantic tokens, not
  a useful multi-hue syntax hierarchy.
- [x] The operator observed the specimens as effectively accent and white and
  rejected a one-colour syntax experience.
- [x] The operator selected one dark and one light base syntax palette with
  selective Poodle-theme overrides.
- [x] Language grammars remain consumer-selected and must not carry colours.
- [x] No public CodeMirror extension, `HighlightStyle`, or theme object is
  needed.
- [x] The work is path-independent from Slider-family repair g18.024.

## Decisions

- Syntax roles are distinct from UI status, action, and accent meaning. Do not
  use success/warning/info/accent as the primary syntax palette.
- Add two primitive syntax ramps: dark-surface and light-surface. Each ramp
  defines deliberate values for comment, keyword, string, literal, type,
  callable, property/attribute, operator, punctuation, and invalid roles.
- Add semantic `color.syntax.*` tokens whose default references the dark ramp.
  Light Poodle theme modes reference the light ramp. Any named theme may
  override an individual semantic syntax role without copying or replacing the
  whole palette.
- The base palettes must be genuinely multi-hue. At minimum keyword, string,
  literal, type, and callable roles use perceptually distinct hues; comment and
  punctuation may remain restrained neutrals. Ordinary identifiers remain
  primary text.
- Map Lezer tags by meaning, not by a preferred language’s class names. Cover
  comments/meta, keywords, strings/regexp/escape, numeric and atomic literals,
  types/classes, definitions/callables, properties/attributes/tags,
  operators, punctuation, and invalid syntax. More-specific mappings win over
  generic names.
- Invalid syntax keeps a non-colour cue as well as its syntax danger role.
  Diagnostics remain host-owned and are not replaced by parser highlighting.
- Every syntax foreground must meet WCAG AA text contrast against its actual
  editor panel in the base light and dark palettes. Theme overrides must pass
  the same check against that theme’s panel.
- Poodle’s existing theme switch must restyle the mounted editor without
  remount, document mutation, grammar reload, selection loss, or history loss.
- Keep the API private and CSS-native. Consumers may override the documented
  `--poodle-color-syntax-*` variables through normal theming; they do not pass
  CodeMirror extensions or palette objects to `CodeEditor`.
- Plain text and `performanceMode="plain"` remain unhighlighted and load no
  grammar.

## UI Design Brief

- **Mode:** Operate. Syntax colour exists to accelerate scanning and error
  detection in a workbench, not to decorate source.
- **Dark base:** cool high-contrast foregrounds over the dark panel with a
  balanced spread across violet/blue, green, amber, cyan, and restrained
  neutrals. Avoid a field dominated by the product accent.
- **Light base:** darker, chromatic counterparts tuned independently for light
  panels; do not mechanically reuse or invert the dark values.
- **Hierarchy:** comments and punctuation recede; ordinary identifiers remain
  calm; keywords, strings, literals, types, callables, and properties are
  distinguishable without making every glyph equally loud.
- **Restraint:** no rainbow-for-rainbow’s-sake. Adjacent roles may share a
  family only when syntax meaning remains scannable in representative TS/JSON.
- **States:** prove TypeScript and JSON, valid/invalid text, selection, active
  line, search, diagnostics, focus, disabled, Eclipse/Iceberg plus every named
  theme, forced colours, and live theme changes.
- **Accessibility:** contrast is measured per role and background; meaning
  cannot depend on hue alone where a state is actionable or erroneous.

## Dispatch manifest

- **State:** complete; merged g18.023 ran beside g18.024 Slider work; serial
  before retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; merged by the plugin, never by the worker
- **Owned mutable paths:** token primitive/semantic/theme schemas and generated
  CSS/TypeScript/Rust artifacts; paired CodeEditor engine twins; CodeEditor
  styles and contract; existing language-registry browser probe and focused
  component/token tests; paired CodeEditor specimens only if needed to expose
  representative roles; one g18.023 execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task state;
  g18.024 Slider-family paths; g18.006/g18.009 state; public CodeEditor props;
  grammar dependencies;
  versions, changelog, workflows, release/tag/publication, Desktop, native
  editor work
- **Worker:** high-reasoning web/tokens worker comfortable with colour systems,
  DTCG token generation, CodeMirror/Lezer tags, WCAG contrast, paired wrappers,
  and real-browser computed-style proof
- **Excluded:** consumer palette prop; public CodeMirror types/extensions;
  bundled grammars; diagnostics redesign; editor layout/focus changes; release
  mutations; windowed selectors
- **Escalation:** Chatterbox if the token generator cannot express shared
  dark/light ramps plus sparse theme overrides, a public API appears necessary,
  or a theme cannot meet contrast without changing its panel/text system

## Work

1. Capture the current failure in both specimens: representative TypeScript
   and JSON resolve mainly to accent/primary treatment despite nominal tag
   coverage. Replace exact status-token assertions with perceptual-role proof.
2. Add dark/light primitive syntax ramps and semantic `color.syntax.*` roles to
   the canonical token schemas. Reference the light ramp from light theme modes
   and retain sparse role-level overrides for themes that need them.
3. Regenerate and audit CSS, TypeScript, Rust, metadata, theme aggregate, and
   package artifacts. Prove every syntax variable exists and theme output is
   deterministic.
4. Expand the paired private `HighlightStyle` mapping across the named syntax
   roles. Keep the engine twins byte-equivalent and ordinary identifiers
   primary.
5. Prove representative TS/JSON samples render at least five distinct
   chromatic syntax roles where their grammar exposes them. Assert computed
   role variables, not hard-coded engine class names.
6. Measure AA contrast for every emitted role against dark/light base panels
   and every named theme override. Prove selection, active-line, search, and
   diagnostic overlays do not erase legibility.
7. Prove live dark/light and named-theme switches restyle the same editor node
   without grammar reload, remount, text/selection/history change, or console
   error. Keep plain modes span-free.
8. Run token build/check, focused paired component/browser/accessibility tests,
   package and preview builds, installed-package audit, docs QA, and
   `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Syntax has its own semantics | highlighting still aliases accent/status tokens | schema and generated CSS show dedicated syntax roles with no primary status dependency |
| Both bases are designed | light palette is a blind inversion or dark values copied unchanged | independent ramp values plus per-role contrast report on both panels |
| Highlighting is meaningfully varied | most representative tokens are accent or primary | TS/JSON computed styles show at least five distinct chromatic roles where exposed |
| Themes remain coherent | one global palette becomes unreadable on a named theme | every theme inherits the correct base and sparse overrides pass contrast |
| Languages stay modular | grammar packages or language-specific colours enter Poodle | installed dependency/chunk audit and generic Lezer-tag mapping |
| Theme changes stay live | switching theme remounts or reloads the grammar | stable editor node, load counters, document, selection, and history |
| Plain stays plain | plain mode inherits syntax spans or loads a grammar | zero spans and zero loads in both plain postures |
| Errors stay legible | invalid text differs only by hue or masks diagnostics | danger-role foreground plus non-colour mark and intact host diagnostics |
| Frameworks agree | Svelte and React map roles differently | engine-source equivalence and paired Chromium/WebKit assertions |
| Release stays closed | `0.4.0` candidate starts with the rejected palette | retained g18.006 remains paused until g18.023 merges and the sweep is accepted |

## Stop conditions

- Stop if the repair requires exposing raw CodeMirror themes, tags, extensions,
  or engine handles.
- Stop if a syntax role cannot meet AA contrast on its actual panel without a
  named theme override; return the role/theme matrix rather than weakening the
  threshold.
- Stop before editing g18.024 Slider work, release state,
  versions, changelog, workflows, Desktop, or native editor surfaces.

## Evidence

Planning inspection on 2026-09-11 found:

- both CodeEditor engines use the same private six-rule `HighlightStyle`;
- keywords map to `color.accent.base`, strings to `color.status.success`,
  numbers/atoms to `color.status.info`, types/definitions to
  `color.status.warning`, comments/meta to secondary text, and invalid syntax
  to danger;
- the contract repeats that UI-semantic mapping;
- the existing browser probe asserts exact equality with those tokens and only
  requires two distinct JSON colours, so it cannot reject an accent-dominated
  specimen; and
- the operator observed the shipped examples as effectively accent and white,
  then selected designed dark/light base palettes with Poodle-theme overrides.

## Next task

g18.023 is merged. After g18.024 merges and the repaired specimens are
accepted, resume the same retained g18.006 release-candidate task. Recompute
final package trees and include both repairs in the `0.4.0` source identity.
