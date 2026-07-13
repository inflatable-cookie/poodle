# g12.005 React Batch: Overlays And Navigation

Status: in progress (2026-07-13)
Owner: Poodle core
Depends on: `g12.004` (Popover proved machine + dismiss-layer + reactifyPart)

## Progress

- [x] Wave 1: Collapsible + Accordion (disclosure/toggle-group machines;
  Svelte's slide transition intentionally not replicated — appear/disappear
  without animation, revisit with CSS if wanted), Menu + MenuSurface
  (menu machine, menuListNavigate roving focus with disabled skipping,
  imperative surface handle, anchor positioning). Verified 7/7: collapsible
  toggle, accordion single-mode swap + collapse, menu open-focuses first
  item, arrow nav skips disabled, Enter fires action + closes, Escape.
- [ ] Wave 2: Tooltip, HoverCard, Dialog, AlertDialog, Drawer
- [ ] Wave 3: ToastStack/ToastHost, Menubar, CommandPalette
- [ ] Wave 4: unlocked stragglers — TimeAgo, ListCardCounter, FormActions
