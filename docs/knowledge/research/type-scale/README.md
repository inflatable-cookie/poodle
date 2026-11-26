# Type scale study

Status: research, 2026-11-25. Adopted by the operator as the type for the
redesigned look. The first six web components use the scale in the quiet
rendering style. See the [style architecture](../../architecture/002-token-system-and-package-layout.md#modes).

## What is here

- [inventory.md](inventory.md): every distinct font size, weight, line
  height, tracking value, case and family in `packages/core/src/styles`
  at commit `3a97944f7`, with the files that use each.
- `migration.tsv` and `migration-counts.tsv`: one proposed role for each
  authored type declaration, and the totals per role.
- `audit.cjs` and `migration.py`: the scripts that produced them. They
  read a clean archive of the repository; the README of the audit run is
  in the planner's lane `design-sweep`.
- In the `*.tsv` files an empty last field is written as `-`, so the
  files carry no trailing whitespace.
- `index.html`: a self-contained prototype. Open it in a browser. It
  shows the same console with today's type and with the proposal, and a
  control for each choice below.

## Findings

- 698 direct `font-size` declarations and 373 local size assignments in
  171 stylesheets; 133 distinct size expressions from 7.5px to 36px.
- 598 size declarations sit under `data-size`; none under density.
  Control-size modes define height, minimum width and icon size only.
- Worst faults: SidebarNav group labels fall to 7.5-10px; the DataTable
  secondary line (14px) is larger than its row name (11.5px) at `sm`; the
  page title has three separate scales; dialog titles range from 13px to
  22px across five components.

## Operator rulings (2026-11-25)

- A role scale of 12, 15, 19, 24 and 30px, about 1.25 per step, with 13px
  for utility copy, as semantic tokens by role.
- Page title 30px. Dialog title 22px. Section title 19px.
- Group labels are 13px, sentence case, neutral colour; no all-caps
  tracked labels by default. An explicit overline stays only above a real
  title.
- Table rows use 15px for the primary line over 13px for the secondary.
- Control labels and input text follow control size: 13, 14, 15, 16 and
  17px for `xs` to `xl`, with line heights 16 to 24px.
- Mono only for code, identifiers, paths and key caps. Aligned numbers
  use tabular figures, not a mono face.
- Fonts are a consumer's choice. Poodle picks a default stack and lets
  each consumer set its own. Font families are tokens (sans and mono, and
  a heading family if a style needs one) with system stacks as the
  default; a consumer overrides them on any subtree, and a rendering style
  may set them. Poodle ships no font files in core. On native runtimes
  the application registers its font bytes and names the families
  through the same tokens.

## Open

- Whether Poodle also offers an opt-in self-hosted Inter entry as a
  convenience. The prototype could not show Inter; judge it by eye first.
- Which components that promise size-dependent content type today
  (PageHeader, Table, editor prose) keep it.
