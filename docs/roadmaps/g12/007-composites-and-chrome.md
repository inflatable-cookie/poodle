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
- [ ] Wave 5: pickers + inputs — ColorPicker, FileUpload, EmbedInput,
  EmbedPreview, PickerShell, RelationPicker, InlineListSection,
  ListContainer, ResizeHandle
- [ ] Wave 6: media — AudioPlayer, VideoPlayer, MediaPicker,
  MediaPreview, MediaThumbnail, MediaBrowsePanel
- [ ] Wave 7: editors — MarkdownEditor, BlockEditor
- [ ] Wave 8: workstation + detail — DockRegion, SplitView, ScrollShell,
  DetailItem, DetailSection, DetailSectionGroup, DetailShell, PageHeader
- [ ] Sweep: remaining stragglers to 132/132, then g12.008 parity
  verification
