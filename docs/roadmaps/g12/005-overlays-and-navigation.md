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
- [x] Wave 2: Dialog (modal machine, ThemePortal, focus trap via
  trapFocusKeydown, focus save/restore, body scroll lock, backdrop +
  Escape), Drawer (edges, modal/non-modal), Tooltip (hover machine +
  anchored bubble + aria-describedby management). Plus the unlocked
  stragglers TimeAgo and ListCardCounter. Two React-idiom findings fixed
  and documented: (1) portal-mounted surfaces render one pass after the
  open flip, so initial focus runs from the surface ref callback via a
  pending flag; (2) React's onPointerEnter synthesizes from bubbling
  pointerover, so event.target is the deepest node — Svelte's non-bubbling
  pointerenter always saw the root; tooltip anchors to the root's direct
  child. Verified 8/8: dialog open/focus/trap/Escape+restore/backdrop,
  drawer open+edge+Escape, tooltip open+describedby+leave-close, time-ago
  relative text + tooltip, counter link mode.
- [ ] Wave 2b: HoverCard, AlertDialog
- [ ] Wave 3: ToastStack/ToastHost, Menubar, CommandPalette
- [ ] Wave 4: FormActions (Menu available now)
