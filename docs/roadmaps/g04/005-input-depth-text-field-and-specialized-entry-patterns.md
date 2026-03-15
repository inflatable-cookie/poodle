# g04.005 Input Depth, Text Field, And Specialized Entry Patterns

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `pug`

## Goals

- [ ] extend TextInput with async validation, prefix/suffix slots, and
  character count
- [ ] implement InlineEditableField as a composite for click-to-edit inline text
- [ ] implement SlugField as a composite for auto-generated URL-safe slugs
- [ ] extend Field with grid-spanning layout modes for multi-column forms

## Execution Checklist

- [ ] amend TextInput contract: add `prefix` and `suffix` slot definitions,
  `asyncValidate` callback prop, `showCharCount` with `maxLength`
- [ ] implement TextInput prefix/suffix slots in `@pug/svelte-primitives`
- [ ] implement TextInput async validation with debounce and loading indicator
- [ ] implement TextInput character count display
- [ ] write contract for InlineEditableField: display mode, edit mode, save/
  cancel, validation, escape-to-cancel
- [ ] implement InlineEditableField composite in `@pug/svelte-composites`
- [ ] write contract for SlugField: source text binding, slug preview,
  manual override, validation
- [ ] implement SlugField composite in `@pug/svelte-composites`
- [ ] amend Field contract: add `span` prop for CSS grid column spanning
- [ ] implement Field grid-spanning in `@pug/svelte-primitives`
- [ ] create specimens for InlineEditableField and SlugField
- [ ] update TextInput and Field specimens with new features
- [ ] register new components in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] TextInput renders prefix/suffix elements inline with the input
- [ ] TextInput async validation shows loading spinner and resolves to
  success/error
- [ ] TextInput character count displays current/max count
- [ ] InlineEditableField toggles between display and edit modes on click or
  Enter
- [ ] InlineEditableField saves on Enter/blur and cancels on Escape
- [ ] SlugField auto-generates a slug from bound source text
- [ ] SlugField allows manual slug override and shows validation
- [ ] Field `span` prop controls grid column spanning in form layouts
- [ ] all components pass build and render in the preview catalogue

## Next Task

Open `g04.006` and implement temporal display and duration input components.
