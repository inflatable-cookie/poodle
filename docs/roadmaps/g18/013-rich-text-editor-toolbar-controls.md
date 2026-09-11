# 013 — RichTextEditor toolbar controls

Status: ready — operator-confirmed UX repair; parallel with g18.010
Owner: Poodle web components
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/rich-text-editor.md`,
`../../contracts/components/markdown-editor.md`,
`../../../packages/svelte/components/src/RichTextEditor.svelte`,
`../../../packages/react/components/src/RichTextEditor.tsx`
Depends on: `g18.003`, `g18.008`

## Outcome

Replace RichTextEditor’s undifferentiated text-command row with a compact,
recognizable Poodle toolbar modelled on MarkdownEditor. Commands use real
Poodle control chrome, icons or concise conventional glyphs, logical groups,
tooltips, accessible names, and truthful pressed/disabled states in both Svelte
and React.

Keep the existing rich-text command, feature, controlled-document, and
ProseMirror boundaries. This is a toolbar presentation and interaction repair,
not a new toolbar plugin API.

## Ready-State Rubric

- [x] The paired RichTextEditor specimens expose all commands as a wrapping row
  of labels that reads visually like text links.
- [x] The operator rejected that presentation and selected MarkdownEditor as
  the intended Poodle reference.
- [x] Existing command availability, active state, roving focus, and execution
  semantics are already centralized and can be preserved.
- [x] The task is path-separate from g18.010’s CodeEditor focus work and may run
  in parallel.
- [x] g18.011 is Queue-held until this known repair and g18.010 merge.

## Decisions

- Reuse Poodle’s `IconButton` semantics and MarkdownEditor’s compact toolbar
  spacing, surface, hover, focus, and grouping language. Do not create raw
  TipTap buttons or a second generic button implementation.
- Use one shared command-presentation map for label, icon/glyph, group, toggle
  posture, and destructive tone. Svelte and React must not maintain independent
  visual vocabularies.
- Use clear conventional icons where Poodle owns them. Short typographic glyphs
  such as `H1`, `H2`, and `H3` are acceptable inside IconButton when they are
  more legible than a generic icon. Add only missing reusable Poodle icons that
  are needed to distinguish commands such as ordered lists or table actions.
- Group history, inline formatting, headings, blocks/lists, tables, and optional
  media. Groups may wrap as intact clusters at constrained widths; the toolbar
  must not become one unstructured sentence or force page-width overflow.
- Toggle commands expose `pressed`; unavailable commands expose `disabled`;
  every control has a tooltip and accessible name. Preserve toolbar roving
  focus and editor focus restoration after command execution.
- The link editor’s Apply and Remove actions use ordinary Poodle Button chrome,
  not the toolbar’s icon-button class. Destructive table removal is visually
  distinguishable without changing its command semantics.
- Keep the public `toolbar` command-selection prop unchanged. Consumers choose
  commands, not their internal icons or arbitrary components.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch in parallel with g18.010;
  g18.011 remains held until both close; serial before retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** RichTextEditor contract; shared rich-text command
  presentation metadata; Svelte/React RichTextEditor toolbar composition;
  rich-text styles; narrowly required reusable icon registry/generated assets;
  paired RichTextEditor specimens and focused unit/browser/visual tests; one
  g18.013 execution log
- **Reserved closeout surfaces:** RichText engine/schema/renderer semantics;
  public feature/command domains; MarkdownEditor behavior; g18 README/index/
  dispatch; g18.006 and g18.009 tasks; g18.010–g18.012 task/workspace/PR state;
  versions, release/tag/publication, Desktop, native/GPUI/Jetstream
- **Worker:** web component UX/accessibility worker comfortable with Svelte,
  React, Poodle control primitives, toolbars, responsive layout and visual tests
- **Excluded:** arbitrary consumer toolbar slots; new rich-text commands;
  document/schema changes; engine changes except focus restoration proven
  necessary by the existing command path; MarkdownEditor redesign; release
- **Escalation:** Chatterbox for a required public toolbar API change, command
  semantic change, uncontrolled icon expansion, or conflict with Poodle’s
  IconButton/Toolbar accessibility contracts

## Work

1. Bind the screenshot failure in both real specimens: command labels form an
   undifferentiated text row with weak hit-area/group/state communication.
2. Define the shared command-presentation map and smallest required icon/glyph
   set. Reuse IconButton and existing Poodle tokens; do not hand-roll duplicate
   button state styles.
3. Render grouped command clusters in both wrappers. Preserve feature-derived
   ordering, explicit toolbar subsets, roving keyboard focus, active/available
   snapshots, command execution, and editor focus/selection behavior.
4. Restyle the toolbar after MarkdownEditor across size/density/theme and
   constrained widths. Convert link Apply/Remove actions to ordinary Poodle
   buttons and keep their keyboard flow coherent.
5. Update paired specimens to make default, active, disabled, table-context,
   explicit-subset, and images-enabled postures inspectable.
6. Run focused component/browser/accessibility/visual checks, both preview
   builds, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Controls read as controls | toolbar still appears as one line of link-like labels | paired screenshots show compact hit areas, icons/glyphs, groups and states |
| Poodle language is reused | raw bespoke buttons duplicate IconButton behavior | component tree/style proof delegates control chrome and tooltips to Poodle primitives |
| Commands stay recognizable | icon-only controls have ambiguous or missing names | command map plus tooltip and accessible-name assertions for every admitted command |
| State is truthful | bold looks active when it is not, or disabled table actions appear available | live selection/context cases assert pressed and disabled DOM/state |
| Groups survive layout | controls wrap individually into a scrambled sentence or overflow the page | constrained-width visual and geometry assertions keep clusters coherent |
| Keyboard behavior survives | IconButton wrappers break roving arrows or lose editor selection | Tab/arrow/Enter/Space journey and post-command focus/selection proof |
| Explicit subsets remain exact | presentation map re-adds commands omitted by `toolbar` | custom subset renders and operates exactly once in both wrappers |
| Optional features stay optional | image/table controls appear without their feature/authority | auto-toolbar cases for standard, tables and images-off/on |
| Link actions are proper controls | Apply/Remove retain toolbar-link styling or inaccessible focus | Button composition and keyboard assertions for open link editor |
| Frameworks match | Svelte and React choose different icons, order, grouping or states | shared metadata and paired specimen/browser evidence |
| Release remains gated | g18.011 or g18.006 proceeds before this known defect closes | Queue hold and merged repair before sweep release |

## Stop conditions

- Stop if the repair requires changing the public command or feature domains.
- Stop if IconButton cannot preserve the existing roving-toolbar contract;
  return the exact primitive gap instead of bypassing it with bespoke controls.
- Stop before release, Desktop, native, or retained-task mutations.

## Evidence

Operator screenshot on 2026-09-11 shows Undo through Delete table rendered as
one wrapping row of plain text labels above the editor. The operator required
proper buttons and selected MarkdownEditor as the visual/interaction reference.

## Next task

After this repair merges, g18.014 rebases onto it and proves the image-policy
specimen. Chatterbox releases held g18.011 only after g18.010 and g18.014 also
merge. Keep g18.006 blocked and g18.009 held.
