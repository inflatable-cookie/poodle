# g04.013 Rich Text And Markdown Editing

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `pug`

## Goals

- [ ] implement MarkdownEditor as a composite for markdown authoring with
  toolbar and preview

## Execution Checklist

- [ ] evaluate editor engine options (EasyMDE, CodeMirror, Milkdown, custom)
- [ ] write contract for MarkdownEditor: source text binding, toolbar actions
  (bold, italic, heading, link, image, list, code, quote), preview toggle,
  split-view mode, image upload integration
- [ ] implement MarkdownEditor composite in `@pug/svelte-composites`
- [ ] implement toolbar with Pug IconButton and ToggleGroup components
- [ ] implement markdown preview rendering with sanitization
- [ ] implement split-view mode (edit + preview side by side)
- [ ] create MarkdownEditor specimen
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] MarkdownEditor renders a text area with a formatting toolbar
- [ ] toolbar actions insert correct markdown syntax at cursor position
- [ ] preview mode renders markdown as formatted HTML
- [ ] split-view shows editor and preview side by side
- [ ] markdown source is accessible as a bindable value
- [ ] the component passes build and renders in the preview catalogue

## Next Task

Open `g04.014` and implement media playback components.
