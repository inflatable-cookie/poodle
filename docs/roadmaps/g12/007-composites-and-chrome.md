# g12.007 React Batch: Composites And Chrome

Status: in progress (2026-07-14)
Owner: Poodle core
Depends on: `g12.006`

Final conversion tier: everything left after primitives, forms, overlays,
and data/date — app chrome, navigation, media, editors, workstation
pieces, and detail shells. Same playbook as prior batches (roadmap 002).

## Progress

- [x] Wave 1: command chrome — Menubar, CommandPalette,
  ActionDiscoveryPanel, EmptyState (landed against the 005 wave-4 entry;
  see that doc for the deferred-focus-restore React trap). 20/20 probes.
- [x] Wave 2: nav chrome — AppHeader (identity/actions/utility lanes,
  presentation re-provide), SidebarNav (grouped nav, anchor-or-button
  items, controlled/uncontrolled value), Breadcrumbs (maxVisibleItems
  ellipsis collapse, forceLastItemCurrent), NavigationMenu (roving
  tabindex triggers, arrow/Home/End nav skipping disabled, panel render
  prop `(value, item) => ReactNode`, dismiss layer). Verified 22/22.
  84/132.
- [x] Wave 3: cards + tiles — Card (media/header/body/footer parts,
  variant/layout/selected data attrs), Surface (tone/border/padding,
  optional region role), NavCard (anchor-or-button dual root, badge,
  arrow), MetricTile (sparkline path built from data, trend arrow +
  label), CardRadioGroup + CardToggleGroup (both drive
  toggleGroupTransition from @poodle/headless; roving tabindex, arrow
  nav skipping disabled, toggle group supports allowDeactivation),
  StatusBar. Verified 28/28. 91/132.
- [x] Wave 4: toolbars + bars — Toolbar (getFocusableElements arrow
  cycling, orientation-aware), FilterToolbar (collapsible header button
  with CollapseToggle, controlled/uncontrolled collapsed pair,
  columns/minItemWidth CSS vars, presentation re-provide), MetaBar (pill
  context provider), BulkActionBar (select-all, tone'd icon actions,
  zero-selection disable; custom icon = isValidElement branch replacing
  Svelte's Component check), SelectionSummary (chip remove, overflow
  count, clear), CollapseToggle, PageLoading (overlay/inline, progress
  when determinate, cancel). Verified 35/35. 98/132. controlHeightRem
  added to React presentation.ts. Specimen note: BulkActionBar is
  position:fixed bottom — one instance per page or they stack.
- [x] Wave 5a: shells + embeds — Callout, PageHeader (entity-detail
  posture, back links, count pill, banner), EmbedInput (debounced parse
  through the ported framework-free embed-input module, provider pill,
  error lane), EmbedPreview (provider iframe URLs, trusted-HTML branch
  via dangerouslySetInnerHTML), PickerShell (browse-state switch, SR
  status region), InlineListSection (generic render-prop list, framed
  Card wrap), ListContainer (PageHeader + state switch + built-in
  pagination), ResizeHandle (resizeAxisPosition/resizeDragDelta/
  resizeKeydownStep from @poodle/headless; window listener pair attached
  per drag). Verified 30/30. 106/132.
- [x] Wave 5b: heavy pickers — ColorPicker (HSV state + pinned-hex model
  over the headless color converters; gradient pad with pointer capture
  and shift-stepped arrows, hue/alpha Sliders, hex/RGB/HSL mode inputs,
  swatch listbox, outside/Escape dismiss, above/below placement),
  FileUpload (drag-drop + browse, validateUploadFile/compressImage from
  the ported file-upload module, preview object-URLs with revoke,
  imperative updateProgress/setError/clear handle), RelationPicker
  (flat + drill-down modes over PickerShell/SelectionSummary; drill
  levels with async item loaders, breadcrumbs, final-items handoff,
  candidate roving arrows, filter Selects, footer FormActions).
  Verified 28/28. 109/132.
- [x] Wave 6: media — AudioPlayer (rAF time loop, seek/volume ranges,
  mute, speed select), VideoPlayer (wrapper-as-button play toggle,
  auto-hiding controls with 3s timer, fullscreen API, progress fill),
  MediaThumbnail (kind/state/aspect data attrs, frame sizing props,
  play overlay, caption), MediaPreview (Card media composition),
  MediaBrowsePanel (grid over compact thumbnails, load-more, error
  Callout), MediaPicker (Dialog + Tabs browse/upload with search filter
  and FileUpload). Verified 30/30. 115/132.
- [x] Wave 7: editors — MarkdownEditor (marked-rendered preview in
  edit/split/preview modes, toolbar wrap/line insertions through
  execCommand with setRangeText fallback, post-render selection restore
  via pending ref), BlockEditor (block CRUD, move up/down, HTML5 drag
  reorder, type-change + add-after Selects with custom trigger,
  typePicker/addPicker/block render props). marked@^18 added to
  @poodle/react deps. Verified 18/18. 117/132.
- [ ] Wave 8: workstation + detail — DockRegion, SplitView, ScrollShell,
  DetailItem, DetailSection, DetailSectionGroup, DetailShell, PageHeader
- [ ] Sweep: remaining stragglers to 132/132, then g12.008 parity
  verification
